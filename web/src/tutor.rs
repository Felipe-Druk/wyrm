//! Interfaz de visualización paso a paso de la Wyrm VM (Wyrm Tutor).

use crate::trace::{StepDiff, TraceConstant, TraceHeapVec, TraceInstruction, TraceSession, TraceStep};
use leptos::html::Div;
use leptos::leptos_dom::helpers::{set_timeout_with_handle, TimeoutHandle};
use leptos::prelude::*;
use std::time::Duration;

const BYTECODE_ROW_HEIGHT_PX: i32 = 24;

/// Velocidad de reproducción automática del Tutor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlaybackSpeed {
    Half,
    Normal,
    Double,
}

impl PlaybackSpeed {
    fn delay_ms(self) -> u64 {
        match self {
            Self::Half => 1100,
            Self::Normal => 550,
            Self::Double => 220,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Half => "0.5x",
            Self::Normal => "1x",
            Self::Double => "2x",
        }
    }
}

/// Panel interactivo de Wyrm Tutor: controles de reproducción, Stack & CallFrames, Heap y Bytecode.
#[component]
pub fn TutorPanel(
    trace_session: RwSignal<Option<TraceSession>>,
    trace_error: RwSignal<Option<String>>,
    step_idx: RwSignal<usize>,
    is_running: RwSignal<bool>,
    on_trace: Callback<()>,
) -> impl IntoView {
    let is_playing = RwSignal::new(false);
    let playback_speed = RwSignal::new(PlaybackSpeed::Normal);
    let show_constants = RwSignal::new(false);
    let bytecode_list_ref = NodeRef::<Div>::new();

    let play_timer = StoredValue::new(None::<TimeoutHandle>);
    let play_generation = StoredValue::new(0u64);

    let clear_play_timer = move || {
        play_timer.update_value(|slot| {
            if let Some(handle) = slot.take() {
                handle.clear();
            }
        });
    };

    on_cleanup(move || {
        clear_play_timer();
    });

    let max_step = Memo::new(move |_| {
        trace_session.with(|opt| {
            opt.as_ref()
                .map(|session| session.steps.len().saturating_sub(1))
                .unwrap_or(0)
        })
    });

    // Tupla reactiva (step_num, ip_before, ip_after) del paso activo
    let active_ips = Memo::new(move |_| {
        let idx = step_idx.get();
        trace_session.with(|opt| {
            let Some(session) = opt.as_ref() else {
                return (0usize, 0u32, 0u32);
            };
            let clamped = idx.min(session.steps.len().saturating_sub(1));
            match session.steps.get(clamped) {
                Some(step) => (step.step, step.ip_before, step.ip_after),
                None => (0, 0, 0),
            }
        })
    });

    // Detiene la reproducción automática si llega una nueva ejecución o se limpia la sesión
    Effect::new(move |_| {
        let running = is_running.get();
        let has_session = trace_session.with(|opt| opt.is_some());
        if running || !has_session {
            is_playing.set(false);
            clear_play_timer();
        }
    });

    // Bucle de auto-play gobernado por TimeoutHandle cancelable
    Effect::new(move |_| {
        clear_play_timer();
        let playing = is_playing.get();
        let speed = playback_speed.get();
        let current = step_idx.get();
        let limit = max_step.get();

        if !playing {
            return;
        }
        if limit == 0 || current >= limit {
            is_playing.set(false);
            return;
        }

        let next_gen = play_generation.get_value().wrapping_add(1);
        play_generation.set_value(next_gen);

        if let Ok(handle) = set_timeout_with_handle(
            move || {
                if play_generation.get_value() == next_gen && is_playing.get_untracked() {
                    let cur = step_idx.get_untracked();
                    let lim = max_step.get_untracked();
                    if cur < lim {
                        step_idx.set(cur + 1);
                    } else {
                        is_playing.set(false);
                    }
                }
            },
            Duration::from_millis(speed.delay_ms()),
        ) {
            play_timer.set_value(Some(handle));
        }
    });

    // Mantiene visible la instrucción activa dentro de la lista scrolleable de bytecode
    Effect::new(move |_| {
        let (step_num, ip_before, ip_after) = active_ips.get();
        let target_ip = if step_num == 0 { ip_after } else { ip_before };
        let active_row = trace_session.with(|opt| {
            opt.as_ref()
                .and_then(|session| session.instruction_index_for_ip(target_ip))
        });
        if let (Some(row_idx), Some(list_el)) = (active_row, bytecode_list_ref.get()) {
            let target_top = (row_idx as i32) * BYTECODE_ROW_HEIGHT_PX;
            let view_h = list_el.client_height();
            let cur_top = list_el.scroll_top();
            if view_h > 0
                && (target_top < cur_top || target_top + BYTECODE_ROW_HEIGHT_PX > cur_top + view_h)
            {
                list_el
                    .set_scroll_top((target_top - view_h / 2 + BYTECODE_ROW_HEIGHT_PX / 2).max(0));
            }
        }
    });

    let on_start = move |_: leptos::ev::MouseEvent| {
        is_playing.set(false);
        step_idx.set(0);
    };

    let on_prev = move |_: leptos::ev::MouseEvent| {
        is_playing.set(false);
        step_idx.update(|idx| *idx = idx.saturating_sub(1));
    };

    let on_toggle_play = move |_: leptos::ev::MouseEvent| {
        let limit = max_step.get_untracked();
        if limit == 0 {
            return;
        }
        if is_playing.get_untracked() {
            is_playing.set(false);
        } else {
            if step_idx.get_untracked() >= limit {
                step_idx.set(0);
            }
            is_playing.set(true);
        }
    };

    let on_next = move |_: leptos::ev::MouseEvent| {
        is_playing.set(false);
        let limit = max_step.get_untracked();
        step_idx.update(|idx| {
            if *idx < limit {
                *idx += 1;
            }
        });
    };

    let on_end = move |_: leptos::ev::MouseEvent| {
        is_playing.set(false);
        step_idx.set(max_step.get_untracked());
    };

    view! {
        <div class="tutor-root">
            {move || {
                if let Some(err_msg) = trace_error.get() {
                    return view! {
                        <div class="tutor-error-wrap">
                            <div class="console-block error corner-frame">
                                <div class="stream-header">
                                    <span class="stream-glyph error" aria-hidden="true">"✕"</span>
                                    <span class="stream-label">"ERROR AL TRAZAR EN LA WYRM VM"</span>
                                </div>
                                <pre class="terminal-pre error">{err_msg}</pre>
                            </div>
                            <button
                                type="button"
                                class="tutor-cta-btn"
                                disabled=move || is_running.get()
                                on:click=move |_| on_trace.run(())
                            >
                                <span aria-hidden="true">"▷"</span>
                                <span>"REINTENTAR PASO A PASO"</span>
                            </button>
                        </div>
                    }
                    .into_any();
                }

                let Some(session) = trace_session.get() else {
                    return view! {
                        <div class="empty-state tutor-empty-state">
                            <p class="empty-title">":: WYRM TUTOR — INSPECTOR PASO A PASO"</p>
                            <p class="empty-hint">
                                "Trazá la ejecución de la VM instrucción por instrucción para observar "
                                <code>"IP"</code>
                                ", "
                                <code>"CallFrames"</code>
                                ", el Stack en bytes hexadecimales y el Heap de "
                                <code>"vec<T>"</code>
                                "."
                            </p>
                            <button
                                type="button"
                                class="tutor-cta-btn"
                                disabled=move || is_running.get()
                                title="Iniciar traza paso a paso (Ctrl + Shift + Enter)"
                                on:click=move |_| on_trace.run(())
                            >
                                <span aria-hidden="true">"▷"</span>
                                <span>"TRAZAR CÓDIGO ACTUAL"</span>
                                <kbd class="tutor-shortcut">"Ctrl+⇧+↵"</kbd>
                            </button>
                        </div>
                    }
                    .into_any();
                };

                let total_steps = session.steps.len().saturating_sub(1);
                let at_end = move || step_idx.get() >= total_steps;
                let truncated = session.truncated;
                let max_limit = session.max_steps;
                let instructions = session.instructions.clone();
                let constants = session.constants.clone();

                let dynamic_state_view = move || {
                    let clamped_idx = step_idx.get().min(total_steps);
                    let Some(step) = session.steps.get(clamped_idx) else {
                        return view! { <div class="tutor-empty-msg">"∅ sin pasos registrados"</div> }
                            .into_any();
                    };
                    let diff = session.diff_at(clamped_idx);

                    view! {
                        <div class="tutor-dynamic-sections">
                            {render_step_summary(step, &diff)}
                            {render_stack_and_frames(step, &diff)}
                            {render_heap_section(step, &diff)}
                        </div>
                    }
                    .into_any()
                };

                view! {
                    <div class="tutor-workspace">
                        <div class="tutor-controls-bar">
                            <div class="tutor-transport" role="group" aria-label="Controles de paso a paso">
                                <button
                                    type="button"
                                    class="tutor-ctrl-btn"
                                    disabled=move || step_idx.get() == 0
                                    title="Ir al inicio (paso 0)"
                                    on:click=on_start
                                >
                                    "⏮"
                                </button>
                                <button
                                    type="button"
                                    class="tutor-ctrl-btn"
                                    disabled=move || step_idx.get() == 0
                                    title="Paso anterior"
                                    on:click=on_prev
                                >
                                    "◀"
                                </button>
                                <button
                                    type="button"
                                    class="tutor-ctrl-btn tutor-ctrl-btn--play"
                                    class:active=move || is_playing.get()
                                    disabled=move || total_steps == 0
                                    title="Reproducir / pausar avance automático"
                                    on:click=on_toggle_play
                                >
                                    {move || if is_playing.get() { "⏸" } else { "▶" }}
                                </button>
                                <button
                                    type="button"
                                    class="tutor-ctrl-btn"
                                    disabled=at_end
                                    title="Siguiente paso"
                                    on:click=on_next
                                >
                                    "▶|"
                                </button>
                                <button
                                    type="button"
                                    class="tutor-ctrl-btn"
                                    disabled=at_end
                                    title="Ir al paso final"
                                    on:click=on_end
                                >
                                    "⏭"
                                </button>
                            </div>

                            <div class="tutor-speed-group" role="group" aria-label="Velocidad de reproducción">
                                {[PlaybackSpeed::Half, PlaybackSpeed::Normal, PlaybackSpeed::Double]
                                    .into_iter()
                                    .map(|spd| {
                                        view! {
                                            <button
                                                type="button"
                                                class="tutor-speed-btn"
                                                class:active=move || playback_speed.get() == spd
                                                on:click=move |_| playback_speed.set(spd)
                                            >
                                                {spd.label()}
                                            </button>
                                        }
                                    })
                                    .collect_view()}
                            </div>

                            <div class="tutor-counter">
                                {move || {
                                    format!(
                                        "PASO {:03} / {:03}",
                                        step_idx.get().min(total_steps),
                                        total_steps
                                    )
                                }}
                            </div>

                            <Show when=move || truncated>
                                <span
                                    class="tutor-truncated-badge"
                                    title="La ejecución superó el tope de pasos del trazador"
                                >
                                    {format!("TRUNCADO ({max_limit})")}
                                </span>
                            </Show>
                        </div>

                        <div class="tutor-scrubber-row">
                            <input
                                type="range"
                                class="tutor-scrubber"
                                aria-label="Línea de tiempo de pasos de la VM"
                                min="0"
                                max=total_steps.to_string()
                                prop:value=move || step_idx.get().min(total_steps).to_string()
                                on:input=move |ev| {
                                    is_playing.set(false);
                                    if let Ok(parsed) = event_target_value(&ev).parse::<usize>() {
                                        step_idx.set(parsed.min(total_steps));
                                    }
                                }
                            />
                        </div>

                        <div class="tutor-step-content">
                            {dynamic_state_view}
                            {render_bytecode_section(
                                instructions,
                                constants,
                                active_ips,
                                show_constants,
                                bytecode_list_ref,
                            )}
                        </div>
                    </div>
                }
                .into_any()
            }}
        </div>
    }
}

/// Tira compacta de resumen del paso actual: IP, opcode, operando, línea y cambio de SP.
fn render_step_summary(step: &TraceStep, diff: &StepDiff) -> impl IntoView {
    let ip_label = format!("IP 0x{:04X}", step.ip_before);
    let opcode_label = if step.operand.is_empty() {
        step.opcode.clone()
    } else {
        format!("{} {}", step.opcode, step.operand)
    };
    let next_ip_label = format!("→ 0x{:04X}", step.ip_after);
    let sp_changed = diff.prev_stack_len != diff.curr_stack_len;
    let sp_label = if step.step == 0 {
        format!("SP: {}", diff.curr_stack_len)
    } else {
        format!("SP: {} → {}", diff.prev_stack_len, diff.curr_stack_len)
    };
    let line = step.line;
    let stdout_len = step.stdout_len;

    view! {
        <div class="tutor-summary-strip">
            <span class="tutor-summary-ip">{ip_label}</span>
            <span class="tutor-summary-sep" aria-hidden="true">"::"</span>
            <span class="tutor-summary-op">{opcode_label}</span>
            <span class="tutor-summary-next">{next_ip_label}</span>
            <div class="tutor-summary-meta">
                {if line > 0 {
                    Some(view! { <span class="tutor-meta-chip">{format!("LN {line}")}</span> })
                } else {
                    None
                }}
                <span class="tutor-meta-chip" class:changed=sp_changed>
                    {sp_label}
                </span>
                {if stdout_len > 0 {
                    Some(view! {
                        <span class="tutor-meta-chip">{format!("OUT {stdout_len}B")}</span>
                    })
                } else {
                    None
                }}
            </div>
        </div>
    }
}

/// Sección 01: Stack y CallFrames con ancho de bits y bytes en hexadecimal.
fn render_stack_and_frames(step: &TraceStep, diff: &StepDiff) -> impl IntoView {
    let stack_len = step.stack.len();
    let frame_count = step.frames.len();

    let frames_view = step
        .frames
        .iter()
        .map(|frame| {
            let depth = frame.depth;
            let frame_name = if frame.name.is_empty() {
                if depth == 0 {
                    "<script>".to_string()
                } else {
                    format!("fn#{depth}")
                }
            } else {
                frame.name.clone()
            };
            let title = format!("FRAME #{depth} :: {frame_name}");
            let meta = match frame.return_ip {
                Some(ret_ip) => format!(
                    "base={} · ret=0x{:04X} · -> {}",
                    frame.base_slot, ret_ip, frame.return_type
                ),
                None => format!("base={}", frame.base_slot),
            };
            let is_top_frame = depth + 1 == frame_count;

            let slots: Vec<_> = step
                .stack
                .iter()
                .filter(|slot| slot.frame_depth == depth)
                .collect();

            let slots_body = if slots.is_empty() {
                view! { <div class="tutor-frame-empty">"∅ frame sin slots activos"</div> }.into_any()
            } else {
                view! {
                    <div class="tutor-slot-grid">
                        {slots
                            .into_iter()
                            .map(|slot| {
                                let changed = diff.changed_stack_slots.contains(&slot.index);
                                let idx_label = match &slot.name {
                                    Some(name) => format!("[{}] {name}", slot.index),
                                    None => format!("[{}]", slot.index),
                                };
                                let bits_label = format!("{}b", slot.bits);
                                let hex_display = if slot.hex.is_empty() {
                                    "∅".to_string()
                                } else {
                                    slot.hex.clone()
                                };
                                let val_display = slot.value.clone();
                                let val_title = val_display.clone();
                                let type_display = slot.value_type.clone();
                                let is_ptr = slot.vec_ptr.is_some();

                                view! {
                                    <div
                                        class="tutor-slot"
                                        class:tutor-slot--changed=changed
                                        class:tutor-slot--ptr=is_ptr
                                    >
                                        <div class="tutor-slot-top">
                                            <span class="tutor-slot-idx">{idx_label}</span>
                                            <span class="tutor-slot-type">
                                                {type_display}
                                                <span class="tutor-slot-bits">{bits_label}</span>
                                            </span>
                                        </div>
                                        <div class="tutor-slot-val" title=val_title>
                                            {val_display}
                                        </div>
                                        <div
                                            class="tutor-slot-hex"
                                            title="Bytes crudos en little-endian / puntero"
                                        >
                                            {hex_display}
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                }
                .into_any()
            };

            view! {
                <div class="tutor-frame" class:tutor-frame--active=is_top_frame>
                    <div class="tutor-frame-header">
                        <span class="tutor-frame-title">{title}</span>
                        <span class="tutor-frame-meta">{meta}</span>
                    </div>
                    {slots_body}
                </div>
            }
        })
        .collect_view();

    view! {
        <section class="tutor-section">
            <div class="tutor-section-header">
                <span class="tutor-section-title">"01 :: STACK & CALLFRAMES"</span>
                <span class="tutor-section-count">
                    {format!("{frame_count} frame(s) · {stack_len} slot(s)")}
                </span>
            </div>
            <div class="tutor-frames-list">{frames_view}</div>
        </section>
    }
}

/// Sección 02: Heap de vectores `vec<T>` vivos con referencias desde el Stack y vista ASCII.
fn render_heap_section(step: &TraceStep, diff: &StepDiff) -> impl IntoView {
    let heap_count = step.heap.len();

    let body = if step.heap.is_empty() {
        view! { <div class="tutor-empty-msg">"∅ sin asignaciones en heap"</div> }.into_any()
    } else {
        view! {
            <div class="tutor-heap-list">
                {step
                    .heap
                    .iter()
                    .map(|vec| render_heap_vec_block(vec, step, diff))
                    .collect_view()}
            </div>
        }
        .into_any()
    };

    view! {
        <section class="tutor-section">
            <div class="tutor-section-header">
                <span class="tutor-section-title">"02 :: HEAP (VECTORES & PUNTEROS)"</span>
                <span class="tutor-section-count">{format!("{heap_count} vec(s)")}</span>
            </div>
            {body}
        </section>
    }
}

fn render_heap_vec_block(vec: &TraceHeapVec, step: &TraceStep, diff: &StepDiff) -> impl IntoView {
    let vec_changed = diff.changed_heap_ptrs.contains(&vec.ptr);
    let ptr_label = vec.ptr.clone();
    let type_label = format!("vec<{}>[{}]", vec.elem_type, vec.len);

    let ref_tags: Vec<String> = step
        .stack
        .iter()
        .filter(|slot| slot.vec_ptr.as_deref() == Some(vec.ptr.as_str()))
        .map(|slot| match &slot.name {
            Some(name) => format!("STACK[{}] ({name}) → {}", slot.index, vec.ptr),
            None => format!("STACK[{}] → {}", slot.index, vec.ptr),
        })
        .collect();

    let ascii_row = vec.ascii.as_ref().map(|text| {
        let preview = format!("\"{text}\"");
        view! {
            <div class="tutor-heap-ascii">
                <span class="tutor-heap-ascii-key">"ASCII ::"</span>
                <span class="tutor-heap-ascii-val">{preview}</span>
            </div>
        }
    });

    let extra_count = vec.len.saturating_sub(vec.elements.len());
    let ptr_for_cells = vec.ptr.clone();

    view! {
        <div class="tutor-heap-block" class:tutor-heap-block--changed=vec_changed>
            <div class="tutor-heap-header">
                <div class="tutor-heap-title-group">
                    <span class="tutor-heap-ptr">{ptr_label}</span>
                    <span class="tutor-heap-type">{type_label}</span>
                </div>
                <div class="tutor-heap-refs">
                    {ref_tags
                        .into_iter()
                        .map(|tag| view! { <span class="tutor-ref-badge">{tag}</span> })
                        .collect_view()}
                </div>
            </div>
            {ascii_row}
            <div class="tutor-heap-grid">
                {vec.elements
                    .iter()
                    .map(|elem| {
                        let elem_changed = diff
                            .changed_heap_elems
                            .contains(&(ptr_for_cells.clone(), elem.index));
                        let idx_txt = format!("[{}]", elem.index);
                        let val_txt = elem.value.clone();
                        let hex_txt = elem.hex.clone();
                        view! {
                            <div
                                class="tutor-heap-cell"
                                class:tutor-heap-cell--changed=elem_changed
                            >
                                <span class="tutor-heap-idx">{idx_txt}</span>
                                <span class="tutor-heap-val">{val_txt}</span>
                                <span class="tutor-heap-hex">{hex_txt}</span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
            {if extra_count > 0 {
                Some(view! {
                    <div class="tutor-heap-more">
                        {format!("+{extra_count} elemento(s) adicionales omitidos en vista previa")}
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}

/// Sección 03: Bytecode desensamblado (`.wrc`) estable en DOM con foco reactivo en `IP` y Constant Pool colapsable.
fn render_bytecode_section(
    instructions: Vec<TraceInstruction>,
    constants: Vec<TraceConstant>,
    active_ips: Memo<(usize, u32, u32)>,
    show_constants: RwSignal<bool>,
    bytecode_list_ref: NodeRef<Div>,
) -> impl IntoView {
    let instr_count = instructions.len();
    let constants_count = constants.len();

    let rows_view = instructions
        .into_iter()
        .map(|instr| {
            let instr_ip = instr.ip;
            let ip_txt = format!("0x{:04X}", instr_ip);
            let op_txt = instr.opcode;
            let operand_txt = instr.operand;

            let is_active = move || {
                let (step_num, ip_before, ip_after) = active_ips.get();
                if step_num == 0 {
                    instr_ip == ip_after
                } else {
                    instr_ip == ip_before
                }
            };
            let is_next = move || {
                let (step_num, ip_before, ip_after) = active_ips.get();
                step_num > 0 && instr_ip == ip_after && instr_ip != ip_before
            };

            view! {
                <div
                    class="tutor-bc-row"
                    class:tutor-bc-row--active=is_active
                    class:tutor-bc-row--next=is_next
                >
                    <span class="tutor-bc-marker" aria-hidden="true">
                        {move || {
                            if is_active() {
                                "▸"
                            } else if is_next() {
                                "›"
                            } else {
                                " "
                            }
                        }}
                    </span>
                    <span class="tutor-bc-ip">{ip_txt}</span>
                    <span class="tutor-bc-op">{op_txt}</span>
                    <span class="tutor-bc-arg">{operand_txt}</span>
                </div>
            }
        })
        .collect_view();

    view! {
        <section class="tutor-section">
            <div class="tutor-section-header">
                <span class="tutor-section-title">"03 :: BYTECODE (CHUNK .WRC)"</span>
                <div class="tutor-bc-header-right">
                    <span class="tutor-section-count">{format!("{instr_count} instr.")}</span>
                    <button
                        type="button"
                        class="tutor-const-toggle"
                        aria-expanded=move || if show_constants.get() { "true" } else { "false" }
                        on:click=move |_| show_constants.update(|open| *open = !*open)
                    >
                        <span>{move || if show_constants.get() { "▾" } else { "▸" }}</span>
                        <span>{format!("CONST POOL ({constants_count})")}</span>
                    </button>
                </div>
            </div>

            <Show when=move || show_constants.get()>
                <div class="tutor-const-pool">
                    {if constants.is_empty() {
                        view! { <div class="tutor-empty-msg">"∅ sin constantes"</div> }.into_any()
                    } else {
                        view! {
                            <div class="tutor-const-grid">
                                {constants
                                    .iter()
                                    .map(|c| {
                                        view! {
                                            <div class="tutor-const-item">
                                                <span class="tutor-const-idx">{format!("#{}", c.index)}</span>
                                                <span class="tutor-const-type">{c.value_type.clone()}</span>
                                                <span class="tutor-const-val">{c.value.clone()}</span>
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        }
                        .into_any()
                    }}
                </div>
            </Show>

            <div class="tutor-bc-list" node_ref=bytecode_list_ref>
                {rows_view}
            </div>
        </section>
    }
}
