//! Interfaz interactiva de visualización paso a paso del AST (Intérprete Tree-Walk).

use super::ast_layout::{compute_ast_layout, AstLayout, LayoutNode};
use super::ast_trace::{
    AstNodeInfo, AstResolvedValue, AstScopeDiff, AstTraceSession, AstTraceStep, ScopeVarChange,
};
use crate::components::{handle_tutor_keydown, TutorTransport, WindowDragListeners};
use crate::storage::{
    clamp_pct, load_ast_split_pct, save_ast_split_pct, DEFAULT_AST_GRAPH_PCT, MAX_AST_GRAPH_PCT,
    MIN_AST_GRAPH_PCT,
};
use leptos::ev::PointerEvent;
use leptos::html::Div;
use leptos::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use web_sys::{KeyboardEvent, MouseEvent, WheelEvent};

const DEFAULT_ZOOM: f64 = 0.92;
const MIN_ZOOM: f64 = 0.25;
const MAX_ZOOM: f64 = 2.50;
const ZOOM_STEP_FACTOR: f64 = 1.22;
const WHEEL_ZOOM_FACTOR: f64 = 1.12;
const DEFAULT_STAGE_W: f64 = 640.0;
const DEFAULT_STAGE_H: f64 = 360.0;

thread_local! {
    static AST_V_DRAG_LISTENERS: RefCell<Option<WindowDragListeners>> = const { RefCell::new(None) };
}

fn clear_ast_v_drag_listeners() {
    let existing = AST_V_DRAG_LISTENERS.with(|cell| cell.borrow_mut().take());
    if let Some(listeners) = existing {
        listeners.detach();
    }
}

/// Estado interno de arrastre del lienzo SVG (pan con puntero: mouse o táctil).
#[derive(Debug, Clone, Copy, Default)]
struct DragState {
    active: bool,
    moved: bool,
    pointer_id: Option<i32>,
    start_client_x: i32,
    start_client_y: i32,
    origin_cx: f64,
    origin_cy: f64,
}

/// Panel interactivo de Wyrm Tutor para el recorrido del AST en el intérprete tree-walk.
#[component]
pub fn AstTutorPanel(
    ast_session: RwSignal<Option<AstTraceSession>>,
    ast_error: RwSignal<Option<String>>,
    step_idx: RwSignal<usize>,
    is_running: RwSignal<bool>,
    editor_width_pct: RwSignal<f64>,
    right_expanded: RwSignal<bool>,
    on_ast_trace: Callback<()>,
) -> impl IntoView {
    let is_playing = RwSignal::new(false);
    let auto_focus = RwSignal::new(true);
    let cam_cx = RwSignal::new(200.0f64);
    let cam_cy = RwSignal::new(140.0f64);
    let zoom = RwSignal::new(DEFAULT_ZOOM);
    let is_dragging_ui = RwSignal::new(false);
    let hovered_node = RwSignal::new(None::<usize>);
    let ast_graph_pct = RwSignal::new(load_ast_split_pct());
    let is_resizing_ast = RwSignal::new(false);
    let scopes_collapsed = RwSignal::new(false);
    let stage_resize_tick = RwSignal::new(0u64);

    let canvas_ref = NodeRef::<Div>::new();
    let ast_split_ref = NodeRef::<Div>::new();
    let drag_state = StoredValue::new(DragState::default());
    let suppress_next_click = StoredValue::new(false);

    on_cleanup(move || {
        clear_ast_v_drag_listeners();
    });

    // Layout geométrico top-down memorizado una sola vez por sesión AST
    let cached_layout = Memo::new(move |_| {
        ast_session.with(|opt| {
            opt.as_ref()
                .map(|session| Arc::new(compute_ast_layout(&session.nodes, session.root_id)))
        })
    });

    // Conjunto de nodos visitados al menos una vez memorizado por sesión
    let executed_nodes = Memo::new(move |_| {
        ast_session.with(|opt| {
            opt.as_ref()
                .map(|session| Arc::new(session.executed_nodes()))
        })
    });

    let max_step = Memo::new(move |_| {
        ast_session.with(|opt| {
            opt.as_ref()
                .map(|session| session.steps.len().saturating_sub(1))
                .unwrap_or(0)
        })
    });

    // Detiene auto-play si corre una nueva ejecución o se limpia la traza
    Effect::new(move |_| {
        let running = is_running.get();
        let has_session = ast_session.with(|opt| opt.is_some());
        if running || !has_session {
            is_playing.set(false);
        }
    });

    // Desconecta los listeners globales de ajuste vertical fuera del stack de `mouseup` / `blur`
    Effect::new(move |_| {
        if scopes_collapsed.get() && is_resizing_ast.get_untracked() {
            is_resizing_ast.set(false);
        }
        if !is_resizing_ast.get() {
            clear_ast_v_drag_listeners();
        }
    });

    // Recalcula el viewBox del SVG tras cambios de ancho, expansión, altura o colapso de scopes
    Effect::new(move |_| {
        let _ = editor_width_pct.get();
        let _ = right_expanded.get();
        let _ = ast_graph_pct.get();
        let _ = scopes_collapsed.get();
        let _ = cached_layout.get();
        request_animation_frame(move || {
            let _ = stage_resize_tick.try_update(|n| *n = n.wrapping_add(1));
        });
    });

    // Restablece cámara y activa auto-focus al cargarse una nueva sesión AST
    Effect::new(move |_| {
        hovered_node.set(None);
        if let Some(layout) = cached_layout.get() {
            zoom.set(DEFAULT_ZOOM);
            auto_focus.set(true);
            let initial_target = ast_session.with_untracked(|opt| {
                opt.as_ref()
                    .and_then(|s| s.steps.first().map(|st| st.node_id))
                    .or_else(|| opt.as_ref().map(|s| s.root_id))
            });
            if let Some(pos) = initial_target.and_then(|id| layout.node(id)) {
                cam_cx.set(pos.cx);
                cam_cy.set(pos.cy);
            } else {
                cam_cx.set(layout.view_width / 2.0);
                cam_cy.set(layout.view_height / 2.0);
            }
        }
    });

    // Centra automáticamente la cámara sobre el nodo activo cuando auto_focus está habilitado
    Effect::new(move |_| {
        if !auto_focus.get() {
            return;
        }
        let idx = step_idx.get();
        let Some(layout) = cached_layout.get() else {
            return;
        };
        let active_node_id = ast_session.with(|opt| {
            let session = opt.as_ref()?;
            let clamped = idx.min(session.steps.len().saturating_sub(1));
            session.steps.get(clamped).map(|step| step.node_id)
        });
        if let Some(pos) = active_node_id.and_then(|id| layout.node(id)) {
            cam_cx.set(pos.cx);
            cam_cy.set(pos.cy);
        }
    });

    let center_on_active = Callback::new(move |()| {
        let idx = step_idx.get_untracked();
        let Some(layout) = cached_layout.get_untracked() else {
            return;
        };
        let active_id = ast_session.with_untracked(|opt| {
            let session = opt.as_ref()?;
            let clamped = idx.min(session.steps.len().saturating_sub(1));
            session.steps.get(clamped).map(|step| step.node_id)
        });
        if let Some(pos) = active_id.and_then(|id| layout.node(id)) {
            cam_cx.set(pos.cx);
            cam_cy.set(pos.cy);
        }
    });

    let on_fit_all = Callback::new(move |()| {
        let Some(layout) = cached_layout.get_untracked() else {
            return;
        };
        let (stage_w, stage_h) = read_stage_size_untracked(&canvas_ref);
        let fit_x = stage_w / layout.view_width.max(1.0);
        let fit_y = stage_h / layout.view_height.max(1.0);
        let fit_zoom = fit_x.min(fit_y).clamp(MIN_ZOOM, 1.15);
        auto_focus.set(false);
        zoom.set(fit_zoom);
        cam_cx.set(layout.view_width / 2.0);
        cam_cy.set(layout.view_height / 2.0);
    });

    let on_node_select = Callback::new(move |node_id: usize| {
        if suppress_next_click.get_value() {
            suppress_next_click.set_value(false);
            return;
        }
        let cur = step_idx.get_untracked();
        let target = ast_session.with_untracked(|opt| {
            opt.as_ref()
                .and_then(|session| session.next_step_for_node(node_id, cur))
        });
        if let Some(next_step) = target {
            is_playing.set(false);
            step_idx.set(next_step);
        }
    });

    let on_ast_keydown = move |ev: KeyboardEvent| {
        handle_tutor_keydown(ev, step_idx, is_playing, max_step.get_untracked());
    };

    let on_ast_v_mousedown = move |ev: MouseEvent| {
        if ev.button() != 0 || scopes_collapsed.get_untracked() {
            return;
        }
        ev.prevent_default();
        clear_ast_v_drag_listeners();

        let on_move = move |move_ev: MouseEvent| {
            if let Some(split_el) = ast_split_ref.get_untracked() {
                let rect = split_el.get_bounding_client_rect();
                let height = rect.height();
                if height > 1.0 {
                    let pct = (((move_ev.client_y() as f64) - rect.top()) / height) * 100.0;
                    ast_graph_pct.set(clamp_pct(
                        pct,
                        MIN_AST_GRAPH_PCT,
                        MAX_AST_GRAPH_PCT,
                        DEFAULT_AST_GRAPH_PCT,
                    ));
                }
            }
        };

        let on_up = move || {
            if !is_resizing_ast.get_untracked() {
                return;
            }
            save_ast_split_pct(ast_graph_pct.get_untracked());
            is_resizing_ast.set(false);
        };

        if let Some(listeners) = WindowDragListeners::attach(on_move, on_up) {
            AST_V_DRAG_LISTENERS.with(|cell| {
                *cell.borrow_mut() = Some(listeners);
            });
            is_resizing_ast.set(true);
        }
    };

    let on_ast_v_dblclick = move |ev: MouseEvent| {
        ev.prevent_default();
        ast_graph_pct.set(DEFAULT_AST_GRAPH_PCT);
        save_ast_split_pct(DEFAULT_AST_GRAPH_PCT);
    };

    view! {
        <div class="tutor-root ast-tutor-root" tabindex="0" on:keydown=on_ast_keydown>
            {move || {
                if let Some(err_msg) = ast_error.get() {
                    return view! {
                        <div class="tutor-error-wrap">
                            <div class="console-block error corner-frame">
                                <div class="stream-header">
                                    <span class="stream-glyph error" aria-hidden="true">"✕"</span>
                                    <span class="stream-label">"ERROR AL TRAZAR EL AST (INTÉRPRETE)"</span>
                                </div>
                                <pre class="terminal-pre error">{err_msg}</pre>
                            </div>
                            <button
                                type="button"
                                class="tutor-cta-btn"
                                disabled=move || is_running.get()
                                on:click=move |_| on_ast_trace.run(())
                            >
                                <span aria-hidden="true">"▷"</span>
                                <span>"REINTENTAR TRAZA AST"</span>
                            </button>
                        </div>
                    }
                    .into_any();
                }

                let Some(session) = ast_session.get() else {
                    return view! {
                        <div class="empty-state tutor-empty-state">
                            <p class="empty-title">":: WYRM TUTOR — RECORRIDO PASO A PASO DEL AST"</p>
                            <p class="empty-hint">
                                "Explorá el árbol sintáctico abstracto mientras el intérprete evalúa cada nodo ("
                                <code>"enter"</code>
                                " / "
                                <code>"exit"</code>
                                "), burbujea resultados post-orden y actualiza la cadena de "
                                <code>"environment_t"</code>
                                "."
                            </p>
                            <button
                                type="button"
                                class="tutor-cta-btn"
                                disabled=move || is_running.get()
                                title="Trazar recorrido del AST (Ctrl + Shift + Enter en modo Intérprete)"
                                on:click=move |_| on_ast_trace.run(())
                            >
                                <span aria-hidden="true">"▷"</span>
                                <span>"TRAZAR AST DEL CÓDIGO ACTUAL"</span>
                                <kbd class="tutor-shortcut">"Ctrl+⇧+↵"</kbd>
                            </button>
                        </div>
                    }
                    .into_any();
                };

                let total_steps = session.steps.len().saturating_sub(1);
                let truncated = session.truncated;
                let max_limit = session.max_steps;
                let layout_arc = cached_layout
                    .get_untracked()
                    .unwrap_or_else(|| Arc::new(compute_ast_layout(&session.nodes, session.root_id)));
                let exec_set_arc = executed_nodes
                    .get_untracked()
                    .unwrap_or_else(|| Arc::new(session.executed_nodes()));

                let session_for_summary = session.clone();
                let session_for_graph = session.clone();
                let session_for_scopes = session;

                let summary_view = move || {
                    let clamped_idx = step_idx.get().min(total_steps);
                    let Some(step) = session_for_summary.steps.get(clamped_idx) else {
                        return view! { <div class="tutor-empty-msg">"∅ sin pasos registrados"</div> }
                            .into_any();
                    };
                    let node_info = session_for_summary.node(step.node_id);
                    render_ast_step_summary(step, node_info).into_any()
                };

                view! {
                    <div
                        class="tutor-workspace ast-workspace"
                        class:ast-workspace--resizing=move || is_resizing_ast.get()
                    >
                        <TutorTransport
                            step_idx=step_idx
                            is_playing=is_playing
                            is_running=is_running
                            total_steps=total_steps
                            truncated=truncated
                            max_limit=max_limit
                            transport_aria_label="Controles de paso a paso AST"
                            scrubber_aria_label="Línea de tiempo de pasos del AST"
                            truncated_title="La evaluación superó el límite de pasos del trazador AST"
                        />

                        {summary_view}

                        <div
                            class="ast-split-pane"
                            class:ast-split-pane--scopes-collapsed=move || scopes_collapsed.get()
                            node_ref=ast_split_ref
                            style=move || format!("--ast-graph-pct: {:.2}%;", ast_graph_pct.get())
                        >
                            <AstGraphSection
                                session=session_for_graph
                                layout=layout_arc
                                executed_nodes=exec_set_arc
                                step_idx=step_idx
                                total_steps=total_steps
                                cam_cx=cam_cx
                                cam_cy=cam_cy
                                zoom=zoom
                                auto_focus=auto_focus
                                is_dragging_ui=is_dragging_ui
                                hovered_node=hovered_node
                                stage_resize_tick=stage_resize_tick
                                canvas_ref=canvas_ref
                                drag_state=drag_state
                                suppress_next_click=suppress_next_click
                                center_on_active=center_on_active
                                on_fit_all=on_fit_all
                                on_node_select=on_node_select
                            />

                            <div
                                class="ast-v-resizer"
                                class:ast-v-resizer--dragging=move || is_resizing_ast.get()
                                role="separator"
                                aria-orientation="horizontal"
                                title="Arrastrar para ajustar altura del AST · Doble clic para restablecer"
                                on:mousedown=on_ast_v_mousedown
                                on:dblclick=on_ast_v_dblclick
                            >
                                <span class="ast-v-resizer-grip" aria-hidden="true">"⋯"</span>
                            </div>

                            <AstScopesPanel
                                session=session_for_scopes
                                step_idx=step_idx
                                total_steps=total_steps
                                scopes_collapsed=scopes_collapsed
                            />
                        </div>
                    </div>
                }
                .into_any()
            }}
        </div>
    }
}

/// Sección 01: lienzo SVG interactivo del AST con pan, zoom y auto-centrado.
#[component]
fn AstGraphSection(
    session: AstTraceSession,
    layout: Arc<AstLayout>,
    executed_nodes: Arc<HashSet<usize>>,
    step_idx: RwSignal<usize>,
    total_steps: usize,
    cam_cx: RwSignal<f64>,
    cam_cy: RwSignal<f64>,
    zoom: RwSignal<f64>,
    auto_focus: RwSignal<bool>,
    is_dragging_ui: RwSignal<bool>,
    hovered_node: RwSignal<Option<usize>>,
    stage_resize_tick: RwSignal<u64>,
    canvas_ref: NodeRef<Div>,
    drag_state: StoredValue<DragState>,
    suppress_next_click: StoredValue<bool>,
    center_on_active: Callback<()>,
    on_fit_all: Callback<()>,
    on_node_select: Callback<usize>,
) -> impl IntoView {
    let node_count = session.nodes.len();
    let step_total_count = session.total_steps;
    let session_for_svg = session.clone();
    let session_for_hint = session;

    let on_zoom_out = move |_: MouseEvent| {
        zoom.update(|z| *z = (*z / ZOOM_STEP_FACTOR).clamp(MIN_ZOOM, MAX_ZOOM));
    };

    let on_zoom_in = move |_: MouseEvent| {
        zoom.update(|z| *z = (*z * ZOOM_STEP_FACTOR).clamp(MIN_ZOOM, MAX_ZOOM));
    };

    let on_zoom_reset = move |_: MouseEvent| {
        zoom.set(1.0);
        center_on_active.run(());
    };

    let on_toggle_auto_focus = move |_: MouseEvent| {
        let next = !auto_focus.get_untracked();
        auto_focus.set(next);
        if next {
            center_on_active.run(());
        }
    };

    let on_canvas_pointerdown = move |ev: PointerEvent| {
        if !ev.is_primary() {
            return;
        }
        if ev.pointer_type() == "mouse" && ev.button() != 0 {
            return;
        }
        drag_state.set_value(DragState {
            active: true,
            moved: false,
            pointer_id: Some(ev.pointer_id()),
            start_client_x: ev.client_x(),
            start_client_y: ev.client_y(),
            origin_cx: cam_cx.get_untracked(),
            origin_cy: cam_cy.get_untracked(),
        });
        is_dragging_ui.set(true);
    };

    let on_canvas_pointermove = move |ev: PointerEvent| {
        let st = drag_state.get_value();
        if !st.active || st.pointer_id != Some(ev.pointer_id()) {
            return;
        }
        let dx = ev.client_x() - st.start_client_x;
        let dy = ev.client_y() - st.start_client_y;
        if !st.moved && (dx.abs() + dy.abs()) > 4 {
            let mut updated = st;
            updated.moved = true;
            drag_state.set_value(updated);
            auto_focus.set(false);
            if let Some(el) = canvas_ref.get_untracked() {
                let _ = el.set_pointer_capture(ev.pointer_id());
            }
        }
        let cur_zoom = zoom.get_untracked().max(MIN_ZOOM);
        cam_cx.set(st.origin_cx - (dx as f64) / cur_zoom);
        cam_cy.set(st.origin_cy - (dy as f64) / cur_zoom);
    };

    let finish_drag = move |pointer_id: i32| {
        let st = drag_state.get_value();
        if !st.active || st.pointer_id != Some(pointer_id) {
            return;
        }
        if st.moved {
            if let Some(el) = canvas_ref.get_untracked() {
                let _ = el.release_pointer_capture(pointer_id);
            }
            suppress_next_click.set_value(true);
            request_animation_frame(move || {
                suppress_next_click.set_value(false);
            });
        } else {
            suppress_next_click.set_value(false);
        }
        drag_state.set_value(DragState::default());
        is_dragging_ui.set(false);
    };

    let on_canvas_pointerup = move |ev: PointerEvent| {
        finish_drag(ev.pointer_id());
    };

    let on_canvas_pointercancel = move |ev: PointerEvent| {
        finish_drag(ev.pointer_id());
    };

    let on_canvas_pointerleave = move |ev: PointerEvent| {
        let st = drag_state.get_value();
        if st.active && !st.moved && st.pointer_id == Some(ev.pointer_id()) {
            finish_drag(ev.pointer_id());
        }
    };

    let on_canvas_wheel = move |ev: WheelEvent| {
        ev.prevent_default();
        let delta = ev.delta_y();
        if delta.abs() < 0.01 {
            return;
        }
        let factor = if delta < 0.0 {
            WHEEL_ZOOM_FACTOR
        } else {
            1.0 / WHEEL_ZOOM_FACTOR
        };
        zoom.update(|z| *z = (*z * factor).clamp(MIN_ZOOM, MAX_ZOOM));
    };

    let view_box = move || {
        let _ = stage_resize_tick.get();
        let (stage_w, stage_h) = read_stage_size(&canvas_ref);
        let cur_zoom = zoom.get().max(MIN_ZOOM);
        let vb_w = stage_w / cur_zoom;
        let vb_h = stage_h / cur_zoom;
        let vb_x = cam_cx.get() - vb_w / 2.0;
        let vb_y = cam_cy.get() - vb_h / 2.0;
        format!("{vb_x:.1} {vb_y:.1} {vb_w:.1} {vb_h:.1}")
    };

    let svg_contents_view = move || {
        let clamped_idx = step_idx.get().min(total_steps);
        let Some(step) = session_for_svg.steps.get(clamped_idx) else {
            return view! { <g></g> }.into_any();
        };
        let resolved = session_for_svg.resolved_values_at(clamped_idx);
        render_ast_svg_scene(
            &session_for_svg,
            &layout,
            &executed_nodes,
            step,
            &resolved,
            on_node_select,
            hovered_node,
        )
        .into_any()
    };

    let canvas_hint_view = move || {
        if let Some(nid) = hovered_node.get() {
            if let Some(info) = session_for_hint.node(nid) {
                let detail_part = if info.detail.is_empty() {
                    String::new()
                } else {
                    format!(" [{}]", info.detail)
                };
                return format!(
                    "#{} {} :: {}{} · clic para saltar a su paso",
                    info.id, info.kind, info.label, detail_part
                );
            }
        }
        "Arrastrar: pan · Rueda: zoom · Clic en nodo: saltar a su paso · ←/→: pasos".to_string()
    };

    view! {
        <section class="tutor-section ast-graph-section">
            <div class="tutor-section-header ast-graph-header">
                <div class="ast-graph-title-wrap">
                    <span class="tutor-section-title">"01 :: ÁRBOL SINTÁCTICO ABSTRACTO (AST)"</span>
                    <span class="tutor-section-count">
                        {format!("{node_count} nodos · {step_total_count} eventos")}
                    </span>
                </div>

                <div class="ast-view-controls" role="group" aria-label="Controles de cámara del grafo AST">
                    <button
                        type="button"
                        class="ast-cam-btn"
                        title="Alejar zoom"
                        on:click=on_zoom_out
                    >
                        "−"
                    </button>
                    <button
                        type="button"
                        class="ast-cam-btn ast-cam-btn--pct"
                        title="Restablecer zoom a 100%"
                        on:click=on_zoom_reset
                    >
                        {move || format!("{}%", (zoom.get() * 100.0).round() as i32)}
                    </button>
                    <button
                        type="button"
                        class="ast-cam-btn"
                        title="Acercar zoom"
                        on:click=on_zoom_in
                    >
                        "+"
                    </button>
                    <button
                        type="button"
                        class="ast-cam-btn"
                        title="Encuadrar todo el AST"
                        on:click=move |_| on_fit_all.run(())
                    >
                        "⊞"
                    </button>
                    <button
                        type="button"
                        class="ast-cam-btn ast-cam-btn--focus"
                        class:active=move || auto_focus.get()
                        title="Centrar automáticamente el nodo en evaluación"
                        on:click=on_toggle_auto_focus
                    >
                        "◎ CENTRAR"
                    </button>
                </div>
            </div>

            <div
                class="ast-canvas-stage"
                class:dragging=move || is_dragging_ui.get()
                node_ref=canvas_ref
                on:pointerdown=on_canvas_pointerdown
                on:pointermove=on_canvas_pointermove
                on:pointerup=on_canvas_pointerup
                on:pointercancel=on_canvas_pointercancel
                on:pointerleave=on_canvas_pointerleave
                on:wheel=on_canvas_wheel
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox=view_box
                    class="ast-svg"
                >
                    {svg_contents_view}
                </svg>
                <div class="ast-canvas-hint">
                    {canvas_hint_view}
                </div>
            </div>
        </section>
    }
}

/// Sección 02: panel plegable de ámbitos léxicos (`environment_t`) con diff en violeta.
#[component]
fn AstScopesPanel(
    session: AstTraceSession,
    step_idx: RwSignal<usize>,
    total_steps: usize,
    scopes_collapsed: RwSignal<bool>,
) -> impl IntoView {
    let session_for_count = session.clone();

    let scopes_count_view = move || {
        let clamped_idx = step_idx.get().min(total_steps);
        match session_for_count.steps.get(clamped_idx) {
            Some(step) => {
                let scope_count = step.scopes.len();
                let total_vars: usize = step.scopes.iter().map(|frame| frame.vars.len()).sum();
                format!("{scope_count} ámbito(s) · {total_vars} var(s)")
            }
            None => "0 ámbito(s) · 0 var(s)".to_string(),
        }
    };

    let scopes_body_view = move || {
        if scopes_collapsed.get() {
            return None;
        }
        let clamped_idx = step_idx.get().min(total_steps);
        let body = match session.steps.get(clamped_idx) {
            Some(step) => {
                let diff = session.scope_diff_at(clamped_idx);
                render_scopes_body(step, &diff).into_any()
            }
            None => {
                view! { <div class="tutor-empty-msg">"∅ sin entornos activos"</div> }.into_any()
            }
        };
        Some(view! { <div class="ast-scopes-body">{body}</div> })
    };

    view! {
        <section
            class="tutor-section ast-scopes-section"
            class:ast-scopes-section--collapsed=move || scopes_collapsed.get()
        >
            <button
                type="button"
                class="tutor-section-header ast-scopes-header-btn"
                aria-expanded=move || {
                    if scopes_collapsed.get() { "false" } else { "true" }
                }
                title=move || {
                    if scopes_collapsed.get() {
                        "Expandir bloque de scopes"
                    } else {
                        "Colapsar scopes para ampliar el grafo AST"
                    }
                }
                on:click=move |_| {
                    scopes_collapsed.update(|collapsed| *collapsed = !*collapsed)
                }
            >
                <span class="ast-scopes-title-wrap">
                    <span class="ast-scopes-chevron" aria-hidden="true">
                        {move || if scopes_collapsed.get() { "▸" } else { "▾" }}
                    </span>
                    <span class="tutor-section-title">"02 :: SCOPES (environment_t)"</span>
                </span>
                <span class="tutor-section-count">{scopes_count_view}</span>
            </button>
            {scopes_body_view}
        </section>
    }
}

fn read_stage_size(canvas_ref: &NodeRef<Div>) -> (f64, f64) {
    canvas_ref
        .get()
        .map(|el| {
            let w = el.client_width() as f64;
            let h = el.client_height() as f64;
            (
                if w > 40.0 { w } else { DEFAULT_STAGE_W },
                if h > 40.0 { h } else { DEFAULT_STAGE_H },
            )
        })
        .unwrap_or((DEFAULT_STAGE_W, DEFAULT_STAGE_H))
}

fn read_stage_size_untracked(canvas_ref: &NodeRef<Div>) -> (f64, f64) {
    canvas_ref
        .get_untracked()
        .map(|el| {
            let w = el.client_width() as f64;
            let h = el.client_height() as f64;
            (
                if w > 40.0 { w } else { DEFAULT_STAGE_W },
                if h > 40.0 { h } else { DEFAULT_STAGE_H },
            )
        })
        .unwrap_or((DEFAULT_STAGE_W, DEFAULT_STAGE_H))
}

fn truncate_chars(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let prefix: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        let trimmed: String = prefix.chars().take(max_chars.saturating_sub(1)).collect();
        format!("{trimmed}…")
    } else {
        prefix
    }
}

/// Tira compacta de resumen del evento AST activo (`ENTER` dorado / `EVAL => valor : tipo` violeta).
fn render_ast_step_summary(step: &AstTraceStep, node: Option<&AstNodeInfo>) -> impl IntoView {
    let is_enter = step.is_enter();
    let phase_class = if is_enter {
        "ast-phase-badge ast-phase-badge--enter"
    } else {
        "ast-phase-badge ast-phase-badge--exit"
    };
    let phase_text = if is_enter { "▸ ENTER" } else { "◈ EVAL" };

    let kind_text = node
        .map(|n| n.kind.clone())
        .unwrap_or_else(|| "Node".to_string());
    let label_text = node
        .map(|n| n.label.clone())
        .unwrap_or_else(|| format!("#{}", step.node_id));

    let result_badge = if !is_enter {
        match (&step.result_value, &step.result_type) {
            (Some(val), Some(ty)) => {
                let text = format!("=> {val} : {ty}");
                let title_text = text.clone();
                Some(view! {
                    <span class="ast-summary-result" title=title_text>
                        {text}
                    </span>
                })
            }
            _ => None,
        }
    } else {
        None
    };

    let depth_chip = format!("PROF {}", step.depth);
    let node_chip = format!("NODO #{}", step.node_id);
    let step_chip = format!("EVT #{}", step.step);
    let note_text = step.note.clone();
    let has_note = !note_text.is_empty();

    view! {
        <div
            class="ast-summary-strip"
            class:ast-summary-strip--enter=is_enter
            class:ast-summary-strip--exit=!is_enter
        >
            <div class="ast-summary-top">
                <span class=phase_class>{phase_text}</span>
                <span class="ast-summary-kind">{kind_text}</span>
                <span class="tutor-summary-sep" aria-hidden="true">"::"</span>
                <span class="ast-summary-label">{label_text}</span>
                {result_badge}
                <div class="tutor-summary-meta">
                    <span class="tutor-meta-chip">{node_chip}</span>
                    <span class="tutor-meta-chip">{depth_chip}</span>
                    <span class="tutor-meta-chip">{step_chip}</span>
                </div>
            </div>
            {if has_note {
                Some(view! { <div class="ast-summary-note">{note_text}</div> })
            } else {
                None
            }}
        </div>
    }
}

/// Contexto inmutable de escena para renderizar cada nodo SVG del AST.
struct NodeRenderCtx<'a> {
    step: &'a AstTraceStep,
    active_set: &'a HashSet<usize>,
    executed_nodes: &'a HashSet<usize>,
    on_node_select: Callback<usize>,
    hovered_node: RwSignal<Option<usize>>,
}

/// Escena SVG completa: aristas ortogonales, saltos dinámicos de llamada y nodos AST clicables.
fn render_ast_svg_scene(
    session: &AstTraceSession,
    layout: &AstLayout,
    executed_nodes: &HashSet<usize>,
    step: &AstTraceStep,
    resolved: &HashMap<usize, AstResolvedValue>,
    on_node_select: Callback<usize>,
    hovered_node: RwSignal<Option<usize>>,
) -> impl IntoView {
    let active_set: HashSet<usize> = step.active_path.iter().copied().collect();

    // 1. Aristas estáticas ortogonales padre -> hijo
    let edges_view = layout
        .edges
        .iter()
        .map(|edge| {
            let on_path =
                active_set.contains(&edge.parent_id) && active_set.contains(&edge.child_id);
            let is_done = !on_path && resolved.contains_key(&edge.child_id);
            let edge_class = if on_path {
                "ast-edge ast-edge--path"
            } else if is_done {
                "ast-edge ast-edge--done"
            } else {
                "ast-edge"
            };
            let label_class = if on_path {
                "ast-edge-label ast-edge-label--path"
            } else {
                "ast-edge-label"
            };
            let d = edge.path_d.clone();
            let lx = format!("{:.1}", edge.label_x);
            let ly = format!("{:.1}", edge.label_y);
            let lbl = edge.label.clone();
            let has_label = !lbl.is_empty();

            view! {
                <g class="ast-edge-group">
                    <path class=edge_class d=d />
                    {if has_label {
                        Some(view! {
                            <text class=label_class x=lx y=ly>{lbl}</text>
                        })
                    } else {
                        None
                    }}
                </g>
            }
        })
        .collect_view();

    // 2. Conectores dinámicos de invocación (cuando `active_path` salta de un `Call` al cuerpo de `FnDecl`)
    let call_jumps_view = step
        .active_path
        .windows(2)
        .filter_map(|pair| {
            let from_id = pair[0];
            let to_id = pair[1];
            let to_parent = session.node(to_id).and_then(|n| n.parent_id);
            if to_parent == Some(from_id) {
                return None;
            }
            let from_pos = layout.node(from_id)?;
            let to_pos = layout.node(to_id)?;
            let x1 = from_pos.cx;
            let y1 = from_pos.cy;
            let x2 = to_pos.cx;
            let y2 = to_pos.y;
            let mid_x = (x1 + x2) / 2.0;
            let arch_y = y1.min(y2) - 44.0;
            let jump_d = format!("M {x1:.1} {y1:.1} Q {mid_x:.1} {arch_y:.1} {x2:.1} {y2:.1}");
            Some(view! { <path class="ast-edge-jump" d=jump_d /> })
        })
        .collect_view();

    // 3. Nodos del AST con borde activo en oro/violeta, etiqueta y píldora de valor burbujeado
    let node_ctx = NodeRenderCtx {
        step,
        active_set: &active_set,
        executed_nodes,
        on_node_select,
        hovered_node,
    };
    let nodes_view = layout
        .nodes
        .iter()
        .filter_map(|pos| {
            let info = session.node(pos.id)?;
            Some(render_ast_node_svg(
                pos,
                info,
                resolved.get(&pos.id),
                &node_ctx,
            ))
        })
        .collect_view();

    view! {
        <g class="ast-scene">
            <g class="ast-layer-edges">{edges_view}</g>
            <g class="ast-layer-jumps">{call_jumps_view}</g>
            <g class="ast-layer-nodes">{nodes_view}</g>
        </g>
    }
}

fn render_ast_node_svg(
    pos: &LayoutNode,
    info: &AstNodeInfo,
    resolved: Option<&AstResolvedValue>,
    ctx: &NodeRenderCtx<'_>,
) -> impl IntoView {
    let node_id = pos.id;
    let is_current = ctx.step.node_id == node_id;
    let is_enter = ctx.step.is_enter();
    let on_path = !is_current && ctx.active_set.contains(&node_id);
    let was_executed = ctx.executed_nodes.contains(&node_id);
    let on_node_select = ctx.on_node_select;
    let hovered_node = ctx.hovered_node;

    let group_class = if is_current && is_enter {
        "ast-node ast-node--current ast-node--enter"
    } else if is_current {
        "ast-node ast-node--current ast-node--exit"
    } else if on_path {
        "ast-node ast-node--path"
    } else if resolved.is_some() {
        "ast-node ast-node--resolved"
    } else if !was_executed {
        "ast-node ast-node--unvisited"
    } else {
        "ast-node"
    };

    let x_str = format!("{:.1}", pos.x);
    let y_str = format!("{:.1}", pos.y);
    let w_str = format!("{:.1}", pos.width);
    let h_str = format!("{:.1}", pos.height);

    let accent_x = format!("{:.1}", pos.x);
    let accent_y = format!("{:.1}", pos.y);
    let accent_h = format!("{:.1}", pos.height);

    let kind_x = format!("{:.1}", pos.x + 9.0);
    let kind_y = format!("{:.1}", pos.y + 14.0);
    let kind_txt = truncate_chars(&info.kind.to_uppercase(), 12);

    let label_x = format!("{:.1}", pos.x + 9.0);
    let label_y = format!(
        "{:.1}",
        if info.detail.is_empty() {
            pos.y + 35.0
        } else {
            pos.y + 31.0
        }
    );
    let label_txt = truncate_chars(&info.label, 19);

    let detail_view = if !info.detail.is_empty() {
        let dx = format!("{:.1}", pos.x + 9.0);
        let dy = format!("{:.1}", pos.y + 44.5);
        let dtxt = truncate_chars(&info.detail, 21);
        Some(view! {
            <text class="ast-node-detail" x=dx y=dy>{dtxt}</text>
        })
    } else {
        None
    };

    let phase_view = if is_current {
        let px = format!("{:.1}", pos.x + pos.width - 7.0);
        let py = format!("{:.1}", pos.y + 14.0);
        let pclass = if is_enter {
            "ast-node-phase ast-node-phase--enter"
        } else {
            "ast-node-phase ast-node-phase--exit"
        };
        let ptxt = if is_enter { "▸ ENTER" } else { "◈ EVAL" };
        Some(view! {
            <text class=pclass x=px y=py>{ptxt}</text>
        })
    } else {
        None
    };

    let pill_view = resolved.map(|res| {
        let raw_pill = format!("=> {} : {}", res.value, res.value_type);
        let pill_txt = truncate_chars(&raw_pill, 21);
        let pill_y = pos.y + pos.height + 2.0;
        let pill_x = pos.x + 4.0;
        let pill_w = pos.width - 8.0;
        let pill_cx = pos.cx;
        let pill_ty = pill_y + 12.0;
        let pill_box_class = if is_current && !is_enter {
            "ast-pill-box ast-pill-box--active"
        } else {
            "ast-pill-box"
        };
        let pill_txt_class = if is_current && !is_enter {
            "ast-pill-text ast-pill-text--active"
        } else {
            "ast-pill-text"
        };
        view! {
            <g class="ast-pill">
                <rect
                    class=pill_box_class
                    x=format!("{pill_x:.1}")
                    y=format!("{pill_y:.1}")
                    width=format!("{pill_w:.1}")
                    height="17"
                />
                <text
                    class=pill_txt_class
                    x=format!("{pill_cx:.1}")
                    y=format!("{pill_ty:.1}")
                >
                    {pill_txt}
                </text>
            </g>
        }
    });

    view! {
        <g
            class=group_class
            on:click=move |ev: MouseEvent| {
                ev.stop_propagation();
                on_node_select.run(node_id);
            }
            on:mouseenter=move |_: MouseEvent| {
                hovered_node.set(Some(node_id));
            }
            on:mouseleave=move |_: MouseEvent| {
                if hovered_node.get_untracked() == Some(node_id) {
                    hovered_node.set(None);
                }
            }
        >
            <rect class="ast-node-box" x=x_str y=y_str width=w_str height=h_str />
            <rect class="ast-node-accent" x=accent_x y=accent_y width="3" height=accent_h />
            <text class="ast-node-kind" x=kind_x y=kind_y>{kind_txt}</text>
            {phase_view}
            <text class="ast-node-label" x=label_x y=label_y>{label_txt}</text>
            {detail_view}
            {pill_view}
        </g>
    }
}

/// Contenido interno de la sección 02: cadena activa de ámbitos léxicos (`environment_t`).
fn render_scopes_body(step: &AstTraceStep, diff: &AstScopeDiff) -> impl IntoView {
    let scope_count = step.scopes.len();

    if step.scopes.is_empty() {
        view! { <div class="tutor-empty-msg">"∅ sin ámbitos activos"</div> }.into_any()
    } else {
        view! {
            <div class="ast-scopes-list">
                {step
                    .scopes
                    .iter()
                    .enumerate()
                    .map(|(idx, frame)| {
                        let is_innermost = idx + 1 == scope_count;
                        let frame_depth = frame.depth;
                        let title = format!("SCOPE #{frame_depth} :: {}", frame.label);
                        let count_label = format!("{} var(s)", frame.vars.len());

                        let vars_view = if frame.vars.is_empty() {
                            view! {
                                <div class="tutor-frame-empty">"∅ sin bindings en este ámbito"</div>
                            }
                            .into_any()
                        } else {
                            view! {
                                <div class="ast-scope-table">
                                    <div class="ast-scope-head">
                                        <span>"NOMBRE"</span>
                                        <span>"TIPO"</span>
                                        <span>"VALOR"</span>
                                    </div>
                                    {frame
                                        .vars
                                        .iter()
                                        .map(|var| {
                                            let change = diff.change_for(frame_depth, &var.name);
                                            let is_changed = change.is_some();
                                            let change_tag = match change {
                                                Some(ScopeVarChange::Created) => Some("NEW"),
                                                Some(ScopeVarChange::Updated) => Some("UPD"),
                                                None => None,
                                            };
                                            let var_name = var.name.clone();
                                            let var_type = var.value_type.clone();
                                            let var_val = var.value.clone();
                                            let var_val_title = var_val.clone();

                                            view! {
                                                <div
                                                    class="ast-scope-row"
                                                    class:ast-scope-row--changed=is_changed
                                                >
                                                    <span class="ast-scope-var-name">
                                                        <span>{var_name}</span>
                                                        {change_tag.map(|tag| {
                                                            view! {
                                                                <span class="ast-scope-change-tag">
                                                                    {tag}
                                                                </span>
                                                            }
                                                        })}
                                                    </span>
                                                    <span class="ast-scope-var-type">{var_type}</span>
                                                    <span
                                                        class="ast-scope-var-val"
                                                        title=var_val_title
                                                    >
                                                        {var_val}
                                                    </span>
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                            .into_any()
                        };

                        view! {
                            <div
                                class="ast-scope-frame"
                                class:ast-scope-frame--active=is_innermost
                            >
                                <div class="ast-scope-header">
                                    <span class="ast-scope-title">{title}</span>
                                    <span class="ast-scope-count">{count_label}</span>
                                </div>
                                {vars_view}
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        }
        .into_any()
    }
}
