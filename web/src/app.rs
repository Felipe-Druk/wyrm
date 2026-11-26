//! Coordinador raíz de estado, llamadas al motor WASM, resizer horizontal y composición UI.

use crate::components::{
    CodexModal, EditorPanel, FooterBar, OutputPanel, OutputTab, TopBar, TutorMode,
    WindowDragListeners,
};
use crate::engine::{
    self, AstTraceRequest, ExecutionMode, ExecutionRequest, ExecutionResult, TraceRequest,
};
use crate::examples;
use crate::storage::{self, DEFAULT_EDITOR_WIDTH_PCT, MAX_EDITOR_WIDTH_PCT, MIN_EDITOR_WIDTH_PCT};
use crate::tutor::{AstTraceSession, TraceSession};
use leptos::html::Main;
use leptos::leptos_dom::helpers::{set_timeout_with_handle, TimeoutHandle};
use leptos::prelude::*;
use std::cell::RefCell;
use std::time::Duration;
use web_sys::{KeyboardEvent, MouseEvent};

const RUNNING_INDICATOR_DELAY_MS: u64 = 150;
const RUN_PULSE_DURATION_MS: u64 = 300;
const TUTOR_MAX_STEPS: u32 = 1500;

thread_local! {
    static SPLIT_DRAG_LISTENERS: RefCell<Option<WindowDragListeners>> = const { RefCell::new(None) };
}

fn clear_split_drag_listeners() {
    let existing = SPLIT_DRAG_LISTENERS.with(|cell| cell.borrow_mut().take());
    if let Some(listeners) = existing {
        listeners.detach();
    }
}

/// Componente raíz del Playground de Wyrm.
#[component]
pub fn App() -> impl IntoView {
    let initial = examples::default_example();

    let palette = RwSignal::new(storage::load_palette());
    let selected_example_id = RwSignal::new(initial.id.to_string());
    let code = RwSignal::new(initial.code.to_string());
    let mode = RwSignal::new(ExecutionMode::Interpreter);
    let last_mode = RwSignal::new(ExecutionMode::Interpreter);
    let verbose = RwSignal::new(false);
    let stdin_text = RwSignal::new(initial.default_stdin.to_string());
    let stdin_open = RwSignal::new(false);
    let active_tab = RwSignal::new(OutputTab::Console);
    let is_running = RwSignal::new(false);
    let show_running_indicator = RwSignal::new(false);
    let run_count = RwSignal::new(0u32);
    let pulse_phase = RwSignal::new(0u8);
    let result = RwSignal::new(None::<ExecutionResult>);
    let tutor_mode = RwSignal::new(TutorMode::Ast);
    let trace_session = RwSignal::new(None::<TraceSession>);
    let trace_error = RwSignal::new(None::<String>);
    let trace_step_idx = RwSignal::new(0usize);
    let ast_trace_session = RwSignal::new(None::<AstTraceSession>);
    let ast_trace_error = RwSignal::new(None::<String>);
    let ast_step_idx = RwSignal::new(0usize);
    let show_codex_modal = RwSignal::new(false);
    let editor_width_pct = RwSignal::new(storage::load_split_pct());
    let right_expanded = RwSignal::new(false);
    let is_resizing_split = RwSignal::new(false);
    let workspace_ref = NodeRef::<Main>::new();

    let tutor_highlight_line = Memo::new(move |_| {
        if active_tab.get() != OutputTab::Tutor || tutor_mode.get() != TutorMode::Vm {
            return None;
        }
        let idx = trace_step_idx.get();
        trace_session.with(|opt| {
            let session = opt.as_ref()?;
            let step = session.steps.get(idx)?;
            (step.line > 0).then_some(step.line)
        })
    });

    let last_code_snapshot = StoredValue::new(initial.code.to_string());
    let run_generation = StoredValue::new(0u64);
    let running_timer = StoredValue::new(None::<TimeoutHandle>);
    let pulse_generation = StoredValue::new(0u64);
    let pulse_timer = StoredValue::new(None::<TimeoutHandle>);

    let clear_traces = move || {
        trace_session.set(None);
        trace_error.set(None);
        trace_step_idx.set(0);
        ast_trace_session.set(None);
        ast_trace_error.set(None);
        ast_step_idx.set(0);
    };

    Effect::new(move |_| {
        let current = palette.get();
        storage::apply_document_palette(current);
        storage::save_palette(current);
    });

    // Conserva ambas trazas (AST y VM) en memoria hasta que cambie el código fuente
    Effect::new(move |_| {
        let current_code = code.get();
        let changed = last_code_snapshot.with_value(|prev| *prev != current_code);
        if changed {
            last_code_snapshot.set_value(current_code);
            clear_traces();
        }
    });

    // Desconecta los listeners globales de arrastre fuera del stack de `mouseup` / `blur`
    Effect::new(move |_| {
        if right_expanded.get() && is_resizing_split.get_untracked() {
            is_resizing_split.set(false);
        }
        if !is_resizing_split.get() {
            clear_split_drag_listeners();
        }
    });

    let clear_running_timer = move || {
        running_timer.update_value(|slot| {
            if let Some(handle) = slot.take() {
                handle.clear();
            }
        });
        show_running_indicator.set(false);
    };

    let clear_pulse = move || {
        pulse_timer.update_value(|slot| {
            if let Some(handle) = slot.take() {
                handle.clear();
            }
        });
        pulse_phase.set(0);
    };

    on_cleanup(move || {
        clear_split_drag_listeners();
        running_timer.update_value(|slot| {
            if let Some(handle) = slot.take() {
                handle.clear();
            }
        });
        pulse_timer.update_value(|slot| {
            if let Some(handle) = slot.take() {
                handle.clear();
            }
        });
    });

    let begin_run = move || -> u64 {
        clear_running_timer();
        let current_gen = run_generation.get_value().wrapping_add(1);
        run_generation.set_value(current_gen);
        is_running.set(true);

        if let Ok(handle) = set_timeout_with_handle(
            move || {
                if run_generation.get_value() == current_gen && is_running.get_untracked() {
                    show_running_indicator.set(true);
                }
            },
            Duration::from_millis(RUNNING_INDICATOR_DELAY_MS),
        ) {
            running_timer.set_value(Some(handle));
        }
        current_gen
    };

    let trigger_run_pulse = move |run_number: u32| {
        clear_pulse();
        let next_gen = pulse_generation.get_value().wrapping_add(1);
        pulse_generation.set_value(next_gen);

        let phase = if run_number % 2 == 1 { 1u8 } else { 2u8 };
        pulse_phase.set(phase);

        if let Ok(handle) = set_timeout_with_handle(
            move || {
                if pulse_generation.get_value() == next_gen {
                    pulse_phase.set(0);
                }
            },
            Duration::from_millis(RUN_PULSE_DURATION_MS),
        ) {
            pulse_timer.set_value(Some(handle));
        }
    };

    let on_toggle_palette = Callback::new(move |()| {
        palette.update(|current| *current = current.toggle());
    });

    let on_run = Callback::new(move |()| {
        if is_running.get_untracked() {
            return;
        }
        let exec_mode = mode.get_untracked();
        let request = ExecutionRequest {
            source_code: code.get_untracked(),
            mode: exec_mode,
            verbose: verbose.get_untracked(),
            stdin: stdin_text.get_untracked(),
        };
        let current_gen = begin_run();

        engine::execute_async(request, move |execution_result| {
            if run_generation.get_value() != current_gen {
                return;
            }
            clear_running_timer();
            is_running.set(false);

            let next_run = run_count.get_untracked().wrapping_add(1);
            run_count.set(next_run);
            last_mode.set(exec_mode);
            result.set(Some(execution_result));
            trigger_run_pulse(next_run);
        });
    });

    let on_vm_trace = Callback::new(move |()| {
        if is_running.get_untracked() {
            return;
        }
        let request = TraceRequest {
            source_code: code.get_untracked(),
            stdin: stdin_text.get_untracked(),
            max_steps: TUTOR_MAX_STEPS,
        };
        let current_gen = begin_run();

        engine::execute_trace_async(request, move |trace_res| {
            if run_generation.get_value() != current_gen {
                return;
            }
            clear_running_timer();
            is_running.set(false);

            let next_run = run_count.get_untracked().wrapping_add(1);
            run_count.set(next_run);
            last_mode.set(ExecutionMode::CompilerVm);

            result.set(Some(ExecutionResult {
                stdout: trace_res.stdout.clone(),
                return_value: trace_res.return_value.clone(),
                return_type: trace_res.return_type.clone(),
                inspector_log: String::new(),
                elapsed_ms: trace_res.elapsed_ms,
                is_error: trace_res.is_error,
            }));

            let parsed = if trace_res.is_error {
                Err(if trace_res.stdout.trim().is_empty() {
                    "Error al ejecutar la traza paso a paso en la Wyrm VM.".to_string()
                } else {
                    trace_res.stdout
                })
            } else {
                match trace_res.trace_json.as_deref() {
                    Some(raw_json) => TraceSession::from_json(raw_json),
                    None => Err("El motor WASM no devolvió datos de traza JSON.".to_string()),
                }
            };

            trace_step_idx.set(0);
            match parsed {
                Ok(session) => {
                    trace_error.set(None);
                    trace_session.set(Some(session));
                }
                Err(err) => {
                    trace_session.set(None);
                    trace_error.set(Some(err));
                }
            }

            tutor_mode.set(TutorMode::Vm);
            active_tab.set(OutputTab::Tutor);
            trigger_run_pulse(next_run);
        });
    });

    let on_ast_trace = Callback::new(move |()| {
        if is_running.get_untracked() {
            return;
        }
        let request = AstTraceRequest {
            source_code: code.get_untracked(),
            stdin: stdin_text.get_untracked(),
            max_steps: TUTOR_MAX_STEPS,
        };
        let current_gen = begin_run();

        engine::execute_ast_trace_async(request, move |ast_res| {
            if run_generation.get_value() != current_gen {
                return;
            }
            clear_running_timer();
            is_running.set(false);

            let next_run = run_count.get_untracked().wrapping_add(1);
            run_count.set(next_run);
            last_mode.set(ExecutionMode::Interpreter);

            result.set(Some(ExecutionResult {
                stdout: ast_res.stdout.clone(),
                return_value: ast_res.return_value.clone(),
                return_type: ast_res.return_type.clone(),
                inspector_log: String::new(),
                elapsed_ms: ast_res.elapsed_ms,
                is_error: ast_res.is_error,
            }));

            let parsed = if ast_res.is_error {
                Err(if ast_res.stdout.trim().is_empty() {
                    "Error al ejecutar la traza paso a paso del AST.".to_string()
                } else {
                    ast_res.stdout
                })
            } else {
                match ast_res.ast_trace_json.as_deref() {
                    Some(raw_json) => AstTraceSession::from_json(raw_json),
                    None => Err("El motor WASM no devolvió datos de traza AST JSON.".to_string()),
                }
            };

            ast_step_idx.set(0);
            match parsed {
                Ok(session) => {
                    ast_trace_error.set(None);
                    ast_trace_session.set(Some(session));
                }
                Err(err) => {
                    ast_trace_session.set(None);
                    ast_trace_error.set(Some(err));
                }
            }

            tutor_mode.set(TutorMode::Ast);
            active_tab.set(OutputTab::Tutor);
            trigger_run_pulse(next_run);
        });
    });

    let on_step_action = Callback::new(move |()| match mode.get_untracked() {
        ExecutionMode::Interpreter => on_ast_trace.run(()),
        ExecutionMode::CompilerVm => on_vm_trace.run(()),
    });

    let on_stop = Callback::new(move |()| {
        clear_running_timer();
        engine::cancel();
        is_running.set(false);
    });

    let on_clear = Callback::new(move |()| {
        run_generation.set_value(run_generation.get_value().wrapping_add(1));
        clear_running_timer();
        clear_pulse();
        engine::cancel_silent();
        is_running.set(false);
        code.set(String::new());
        result.set(None);
        clear_traces();
        run_count.set(0);
        selected_example_id.set(String::new());
    });

    let on_select_example = Callback::new(move |example_id: String| {
        if let Some(example) = examples::find_by_id(&example_id) {
            if is_running.get_untracked() {
                run_generation.set_value(run_generation.get_value().wrapping_add(1));
                clear_running_timer();
                engine::cancel_silent();
                is_running.set(false);
            }
            clear_pulse();
            selected_example_id.set(example.id.to_string());
            code.set(example.code.to_string());
            stdin_text.set(example.default_stdin.to_string());
            result.set(None);
            clear_traces();
            run_count.set(0);
        }
    });

    let on_global_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Escape" && show_codex_modal.get_untracked() {
            show_codex_modal.set(false);
        }
    };

    let on_split_mousedown = move |ev: MouseEvent| {
        if ev.button() != 0 || right_expanded.get_untracked() {
            return;
        }
        ev.prevent_default();
        clear_split_drag_listeners();

        let on_move = move |move_ev: MouseEvent| {
            if let Some(workspace_el) = workspace_ref.get_untracked() {
                let rect = workspace_el.get_bounding_client_rect();
                let width = rect.width();
                if width > 1.0 {
                    let pct = (((move_ev.client_x() as f64) - rect.left()) / width) * 100.0;
                    editor_width_pct.set(storage::clamp_pct(
                        pct,
                        MIN_EDITOR_WIDTH_PCT,
                        MAX_EDITOR_WIDTH_PCT,
                        DEFAULT_EDITOR_WIDTH_PCT,
                    ));
                }
            }
        };

        let on_up = move || {
            if !is_resizing_split.get_untracked() {
                return;
            }
            storage::save_split_pct(editor_width_pct.get_untracked());
            is_resizing_split.set(false);
        };

        if let Some(listeners) = WindowDragListeners::attach(on_move, on_up) {
            SPLIT_DRAG_LISTENERS.with(|cell| {
                *cell.borrow_mut() = Some(listeners);
            });
            is_resizing_split.set(true);
        }
    };

    let on_split_dblclick = move |ev: MouseEvent| {
        ev.prevent_default();
        editor_width_pct.set(DEFAULT_EDITOR_WIDTH_PCT);
        storage::save_split_pct(DEFAULT_EDITOR_WIDTH_PCT);
    };

    view! {
        <div
            class="playground-shell"
            data-theme=move || palette.get().as_str()
            on:keydown=on_global_keydown
        >
            <TopBar
                palette=palette
                selected_example_id=selected_example_id
                mode=mode
                verbose=verbose
                is_running=is_running
                show_running_indicator=show_running_indicator
                on_toggle_palette=on_toggle_palette
                on_select_example=on_select_example
                on_run=on_run
                on_trace=on_step_action
                on_stop=on_stop
            />

            <main
                class="workspace-split"
                class:workspace--resizing=move || is_resizing_split.get()
                class:workspace-split--expanded=move || right_expanded.get()
                node_ref=workspace_ref
                style=move || format!("--editor-pct: {:.2}%;", editor_width_pct.get())
            >
                <EditorPanel
                    code=code
                    selected_example_id=selected_example_id
                    tutor_highlight_line=tutor_highlight_line
                    editor_width_pct=editor_width_pct
                    right_expanded=right_expanded
                    on_run=on_run
                    on_trace=on_step_action
                />

                <div
                    class="split-resizer"
                    class:split-resizer--dragging=move || is_resizing_split.get()
                    role="separator"
                    aria-orientation="vertical"
                    title="Arrastrar para ajustar ancho · Doble clic para restablecer"
                    on:mousedown=on_split_mousedown
                    on:dblclick=on_split_dblclick
                >
                    <span class="split-resizer-grip" aria-hidden="true">"⋮"</span>
                </div>

                <OutputPanel
                    mode=mode
                    last_mode=last_mode
                    verbose=verbose
                    active_tab=active_tab
                    tutor_mode=tutor_mode
                    is_running=is_running
                    show_running_indicator=show_running_indicator
                    run_count=run_count
                    pulse_phase=pulse_phase
                    result=result
                    trace_session=trace_session
                    trace_error=trace_error
                    trace_step_idx=trace_step_idx
                    ast_trace_session=ast_trace_session
                    ast_trace_error=ast_trace_error
                    ast_step_idx=ast_step_idx
                    stdin_text=stdin_text
                    stdin_open=stdin_open
                    editor_width_pct=editor_width_pct
                    right_expanded=right_expanded
                    on_clear=on_clear
                    on_vm_trace=on_vm_trace
                    on_ast_trace=on_ast_trace
                />
            </main>

            <FooterBar
                show_running_indicator=show_running_indicator
                show_codex_modal=show_codex_modal
            />

            <Show when=move || show_codex_modal.get()>
                <CodexModal show_codex_modal=show_codex_modal />
            </Show>
        </div>
    }
}
