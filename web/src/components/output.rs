//! Panel derecho con pestañas `CONSOLA`, `INSPECTOR` y `TUTOR`, telemetría y cajón `stdin`.

use crate::components::WyrmAsciiArt;
use crate::engine::{ExecutionMode, ExecutionResult};
use crate::tutor::{AstTraceSession, AstTutorPanel, TraceSession, TutorPanel};
use crate::wyrm_art;
use leptos::prelude::*;

/// Pestaña elegida en el panel de resultados.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputTab {
    Console,
    Inspector,
    Tutor,
}

/// Sub-modo activo dentro de la pestaña `:: TUTOR` (AST del Intérprete vs Bytecode de la VM).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TutorMode {
    Ast,
    Vm,
}

/// Etiqueta compacta del modo de motor para la cabecera de telemetría.
fn mode_short_label(mode: ExecutionMode) -> &'static str {
    match mode {
        ExecutionMode::Interpreter => "INTÉRPRETE",
        ExecutionMode::CompilerVm => "VM",
    }
}

fn render_console_view(result: Option<ExecutionResult>, pulse_phase: RwSignal<u8>) -> AnyView {
    let is_pulsing_a = move || pulse_phase.get() == 1;
    let is_pulsing_b = move || pulse_phase.get() == 2;

    let Some(res) = result else {
        return view! {
            <div class="empty-state empty-state--console">
                <WyrmAsciiArt
                    art=wyrm_art::WYRM_CONSOLE_ART
                    class="wyrm-ascii"
                />
                <p class="empty-status-line">
                    "WYRM RUNTIME READY :: CTRL+ENTER PARA EJECUTAR"
                </p>
            </div>
        }
        .into_any();
    };

    let is_error = res.is_error;
    let has_stdout = !res.stdout.is_empty();
    let stdout_content = if has_stdout {
        res.stdout
    } else {
        "∅ (sin salida en stdout)".to_string()
    };
    let status_glyph = if is_error { "✕" } else { "▸" };
    let status_label = if is_error {
        "ERROR DE EJECUCIÓN"
    } else {
        "STDOUT :: FD 1"
    };
    let return_display = res.return_value.unwrap_or_else(|| "∅".to_string());
    let return_type = if res.return_type.is_empty() {
        "void".to_string()
    } else {
        res.return_type
    };
    let show_return_block = !is_error;
    let is_empty_stdout = !has_stdout && !is_error;

    view! {
        <div class="console-stack">
            <div
                class="console-block corner-frame"
                class:error=is_error
                class:run-pulse-a=is_pulsing_a
                class:run-pulse-b=is_pulsing_b
            >
                <div class="stream-header">
                    <span
                        class="stream-glyph"
                        class:error=is_error
                        aria-hidden="true"
                    >
                        {status_glyph}
                    </span>
                    <span class="stream-label">{status_label}</span>
                </div>
                <pre
                    class="terminal-pre"
                    class:error=is_error
                    class:muted=is_empty_stdout
                >
                    {stdout_content}
                </pre>
            </div>

            <Show when=move || show_return_block>
                <div
                    class="return-block corner-frame"
                    class:run-pulse-a=is_pulsing_a
                    class:run-pulse-b=is_pulsing_b
                >
                    <div class="return-header">
                        <div class="return-header-left">
                            <span class="stream-glyph return" aria-hidden="true">"◈"</span>
                            <span class="stream-label">"RETORNO DEL PROGRAMA"</span>
                        </div>
                        <span
                            class="return-type-badge"
                            title="Tipo Wyrm del valor retornado"
                        >
                            {return_type.clone()}
                        </span>
                    </div>
                    <div class="return-body">
                        <span class="return-prefix" aria-hidden="true">"=>"</span>
                        <pre class="return-value-pre">{return_display.clone()}</pre>
                    </div>
                </div>
            </Show>
        </div>
    }
    .into_any()
}

fn render_inspector_view(
    result: Option<ExecutionResult>,
    verbose: RwSignal<bool>,
    pulse_phase: RwSignal<u8>,
) -> AnyView {
    let is_pulsing_a = move || pulse_phase.get() == 1;
    let is_pulsing_b = move || pulse_phase.get() == 2;

    let Some(res) = result else {
        return view! {
            <div class="empty-state">
                <p class="empty-title">":: INSPECTOR EN ESPERA"</p>
                <p class="empty-hint">
                    "Activá "
                    <strong>"[◈] VERBOSE"</strong>
                    " en la barra superior y ejecutá para inspeccionar tokens, AST o bytecode "
                    <code>".wrc"</code>
                    "."
                </p>
            </div>
        }
        .into_any();
    };

    if res.inspector_log.is_empty() {
        let hint = if verbose.get() {
            "Volvé a ejecutar el código para capturar la traza en modo VERBOSE."
        } else {
            "El modo VERBOSE estaba deshabilitado en la última corrida. Activá [◈] VERBOSE y ejecutá nuevamente."
        };
        return view! {
            <div class="empty-state">
                <p class="empty-title">"∅ SIN TRAZA DE INSPECCIÓN"</p>
                <p class="empty-hint">{hint}</p>
            </div>
        }
        .into_any();
    }

    view! {
        <div
            class="inspector-block corner-frame"
            class:run-pulse-a=is_pulsing_a
            class:run-pulse-b=is_pulsing_b
        >
            <div class="stream-header">
                <span class="stream-glyph inspector" aria-hidden="true">"::"</span>
                <span class="stream-label">"TRAZA VERBOSE DEL MOTOR"</span>
            </div>
            <pre class="terminal-pre inspector-pre">{res.inspector_log}</pre>
        </div>
    }
    .into_any()
}

/// Panel derecho con pestañas `Consola`, `Inspector` y `Tutor`, telemetría fija, `Expandir`, `Limpiar` y `stdin / fd 0`.
#[component]
pub fn OutputPanel(
    mode: RwSignal<ExecutionMode>,
    last_mode: RwSignal<ExecutionMode>,
    verbose: RwSignal<bool>,
    active_tab: RwSignal<OutputTab>,
    tutor_mode: RwSignal<TutorMode>,
    is_running: RwSignal<bool>,
    show_running_indicator: RwSignal<bool>,
    run_count: RwSignal<u32>,
    pulse_phase: RwSignal<u8>,
    result: RwSignal<Option<ExecutionResult>>,
    trace_session: RwSignal<Option<TraceSession>>,
    trace_error: RwSignal<Option<String>>,
    trace_step_idx: RwSignal<usize>,
    ast_trace_session: RwSignal<Option<AstTraceSession>>,
    ast_trace_error: RwSignal<Option<String>>,
    ast_step_idx: RwSignal<usize>,
    stdin_text: RwSignal<String>,
    stdin_open: RwSignal<bool>,
    editor_width_pct: RwSignal<f64>,
    right_expanded: RwSignal<bool>,
    on_clear: Callback<()>,
    on_vm_trace: Callback<()>,
    on_ast_trace: Callback<()>,
) -> impl IntoView {
    let has_inspector_output = Memo::new(move |_| {
        result
            .get()
            .map(|res| !res.inspector_log.is_empty())
            .unwrap_or(false)
    });
    let has_vm_session = Memo::new(move |_| trace_session.with(|opt| opt.is_some()));
    let has_ast_session = Memo::new(move |_| ast_trace_session.with(|opt| opt.is_some()));
    let has_trace_session = Memo::new(move |_| has_vm_session.get() || has_ast_session.get());

    let stdin_bytes = Memo::new(move |_| stdin_text.get().len());
    let is_pulsing_a = move || pulse_phase.get() == 1;
    let is_pulsing_b = move || pulse_phase.get() == 2;

    view! {
        <section class="panel output-panel" aria-label="Salida e inspector">
            <div class="panel-header output-header">
                <div class="tab-bar" role="tablist" aria-label="Vistas de salida">
                    <button
                        type="button"
                        role="tab"
                        class="tab-btn"
                        class:active=move || active_tab.get() == OutputTab::Console
                        aria-selected=move || {
                            if active_tab.get() == OutputTab::Console { "true" } else { "false" }
                        }
                        on:click=move |_| active_tab.set(OutputTab::Console)
                    >
                        <span class="tab-glyph" aria-hidden="true">"▸"</span>
                        <span>"CONSOLA"</span>
                    </button>
                    <button
                        type="button"
                        role="tab"
                        class="tab-btn"
                        class:active=move || active_tab.get() == OutputTab::Inspector
                        aria-selected=move || {
                            if active_tab.get() == OutputTab::Inspector { "true" } else { "false" }
                        }
                        on:click=move |_| active_tab.set(OutputTab::Inspector)
                    >
                        <span class="tab-glyph" aria-hidden="true">"::"</span>
                        <span>"INSPECTOR"</span>
                        <Show when=move || has_inspector_output.get()>
                            <span
                                class="tab-ready-glyph"
                                title="Traza verbose lista"
                                aria-hidden="true"
                            >
                                "◈"
                            </span>
                        </Show>
                    </button>
                    <button
                        type="button"
                        role="tab"
                        class="tab-btn"
                        class:active=move || active_tab.get() == OutputTab::Tutor
                        aria-selected=move || {
                            if active_tab.get() == OutputTab::Tutor { "true" } else { "false" }
                        }
                        on:click=move |_| active_tab.set(OutputTab::Tutor)
                    >
                        <span class="tab-glyph" aria-hidden="true">"::"</span>
                        <span>"TUTOR"</span>
                        <Show when=move || has_trace_session.get()>
                            <span
                                class="tab-ready-glyph"
                                title="Traza paso a paso lista"
                                aria-hidden="true"
                            >
                                "◈"
                            </span>
                        </Show>
                    </button>
                </div>

                <div class="output-header-right">
                    <div class="result-telemetry">
                        {move || {
                            if show_running_indicator.get() {
                                return view! {
                                    <span class="telemetry-chip telemetry-chip--running">
                                        <span class="pulse-glyph" aria-hidden="true">"◈"</span>
                                        <span>{format!("EJECUTANDO · {}", mode_short_label(mode.get()))}</span>
                                    </span>
                                }
                                .into_any();
                            }

                            match result.get() {
                                Some(res) => {
                                    let is_error = res.is_error;
                                    let run_num = run_count.get().max(1);
                                    let engine_tag = mode_short_label(last_mode.get());
                                    let glyph = if is_error { "✕" } else { "◈" };
                                    let label = format!(
                                        "RUN #{run_num:02} · {:.2} ms · {engine_tag}",
                                        res.elapsed_ms
                                    );
                                    view! {
                                        <span
                                            class="telemetry-chip telemetry-chip--result"
                                            class:error=is_error
                                            class:run-pulse-a=is_pulsing_a
                                            class:run-pulse-b=is_pulsing_b
                                            title="Contador de corrida, duración y motor utilizado"
                                        >
                                            <span class="telemetry-glyph" aria-hidden="true">{glyph}</span>
                                            <span>{label}</span>
                                        </span>
                                    }
                                    .into_any()
                                }
                                None => view! {
                                    <span class="telemetry-chip telemetry-chip--idle">
                                        <span class="telemetry-glyph" aria-hidden="true">"∅"</span>
                                        <span>{move || format!("LISTO · {}", mode_short_label(mode.get()))}</span>
                                    </span>
                                }
                                .into_any(),
                            }
                        }}
                    </div>

                    <button
                        type="button"
                        class="output-expand-btn"
                        class:active=move || right_expanded.get()
                        aria-pressed=move || if right_expanded.get() { "true" } else { "false" }
                        title=move || {
                            if right_expanded.get() {
                                "Volver a vista dividida"
                            } else {
                                "Expandir panel al 100%"
                            }
                        }
                        on:click=move |_| right_expanded.update(|expanded| *expanded = !*expanded)
                    >
                        <span class="output-expand-glyph" aria-hidden="true">
                            {move || if right_expanded.get() { "⤡" } else { "⤢" }}
                        </span>
                        <span>
                            {move || if right_expanded.get() { "RESTAURAR" } else { "EXPANDIR" }}
                        </span>
                    </button>

                    <button
                        type="button"
                        class="output-clear-btn"
                        title="Limpiar el editor y la salida actual"
                        on:click=move |_| on_clear.run(())
                    >
                        "LIMPIAR"
                    </button>
                </div>
            </div>

            <Show when=move || show_running_indicator.get()>
                <div class="running-banner" role="status" aria-live="polite">
                    <span class="pulse-glyph" aria-hidden="true">"◈"</span>
                    <span class="running-banner-text">
                        {move || format!(
                            "EJECUTANDO EN WEB WORKER ({}) :: PRESIONÁ ■ DETENER PARA INTERRUMPIR",
                            mode_short_label(mode.get())
                        )}
                    </span>
                </div>
            </Show>

            <div class="output-viewport">
                {move || match active_tab.get() {
                    OutputTab::Console => render_console_view(result.get(), pulse_phase),
                    OutputTab::Inspector => {
                        render_inspector_view(result.get(), verbose, pulse_phase)
                    }
                    OutputTab::Tutor => view! {
                        <div
                            class="tutor-shell"
                            class:tutor-shell--ast-active=move || {
                                tutor_mode.get() == TutorMode::Ast && has_ast_session.get()
                            }
                        >
                            <div
                                class="tutor-mode-bar"
                                role="tablist"
                                aria-label="Motor de visualización en Wyrm Tutor"
                            >
                                <button
                                    type="button"
                                    role="tab"
                                    class="tutor-mode-seg"
                                    class:active=move || tutor_mode.get() == TutorMode::Ast
                                    aria-selected=move || {
                                        if tutor_mode.get() == TutorMode::Ast { "true" } else { "false" }
                                    }
                                    title="Inspeccionar el árbol sintáctico abstracto paso a paso (Intérprete)"
                                    on:click=move |_| tutor_mode.set(TutorMode::Ast)
                                >
                                    <span class="tutor-mode-glyph" aria-hidden="true">"▸"</span>
                                    <span>"AST :: INTÉRPRETE"</span>
                                    <Show when=move || has_ast_session.get()>
                                        <span
                                            class="tutor-mode-ready"
                                            title="Traza AST cargada en memoria"
                                            aria-hidden="true"
                                        >
                                            "◈"
                                        </span>
                                    </Show>
                                </button>
                                <button
                                    type="button"
                                    role="tab"
                                    class="tutor-mode-seg"
                                    class:active=move || tutor_mode.get() == TutorMode::Vm
                                    aria-selected=move || {
                                        if tutor_mode.get() == TutorMode::Vm { "true" } else { "false" }
                                    }
                                    title="Inspeccionar Stack, Heap y Bytecode .wrc paso a paso (Compilador + VM)"
                                    on:click=move |_| tutor_mode.set(TutorMode::Vm)
                                >
                                    <span class="tutor-mode-glyph" aria-hidden="true">"▸"</span>
                                    <span>"VM :: BYTECODE"</span>
                                    <Show when=move || has_vm_session.get()>
                                        <span
                                            class="tutor-mode-ready"
                                            title="Traza VM cargada en memoria"
                                            aria-hidden="true"
                                        >
                                            "◈"
                                        </span>
                                    </Show>
                                </button>
                                <button
                                    type="button"
                                    class="tutor-mode-retrace"
                                    disabled=move || is_running.get()
                                    title="Volver a ejecutar y trazar el motor seleccionado sobre el código actual"
                                    on:click=move |_| match tutor_mode.get_untracked() {
                                        TutorMode::Ast => on_ast_trace.run(()),
                                        TutorMode::Vm => on_vm_trace.run(()),
                                    }
                                >
                                    <span aria-hidden="true">"↻"</span>
                                    <span>"RETRAZAR"</span>
                                </button>
                            </div>

                            {move || match tutor_mode.get() {
                                TutorMode::Ast => view! {
                                    <AstTutorPanel
                                        ast_session=ast_trace_session
                                        ast_error=ast_trace_error
                                        step_idx=ast_step_idx
                                        is_running=is_running
                                        editor_width_pct=editor_width_pct
                                        right_expanded=right_expanded
                                        on_ast_trace=on_ast_trace
                                    />
                                }
                                .into_any(),
                                TutorMode::Vm => view! {
                                    <TutorPanel
                                        trace_session=trace_session
                                        trace_error=trace_error
                                        step_idx=trace_step_idx
                                        is_running=is_running
                                        on_trace=on_vm_trace
                                    />
                                }
                                .into_any(),
                            }}
                        </div>
                    }
                    .into_any(),
                }}
            </div>

            <div class="stdin-drawer" class:open=move || stdin_open.get()>
                <div class="stdin-bar">
                    <button
                        type="button"
                        class="stdin-toggle-btn"
                        aria-expanded=move || if stdin_open.get() { "true" } else { "false" }
                        on:click=move |_| stdin_open.update(|open| *open = !*open)
                    >
                        <span class="stdin-chevron" aria-hidden="true">
                            {move || if stdin_open.get() { "▾" } else { "▸" }}
                        </span>
                        <span class="stdin-title">"STDIN :: FD 0"</span>
                        <Show when=move || stdin_bytes.get() != 0>
                            <span class="stdin-badge">
                                {move || format!("{} B", stdin_bytes.get())}
                            </span>
                        </Show>
                    </button>

                    <Show when=move || stdin_bytes.get() != 0>
                        <button
                            type="button"
                            class="stdin-clear-btn"
                            title="Vaciar entrada estándar"
                            on:click=move |_| stdin_text.set(String::new())
                        >
                            "VACIAR"
                        </button>
                    </Show>
                </div>

                <Show when=move || stdin_open.get()>
                    <div class="stdin-body">
                        <textarea
                            class="stdin-textarea"
                            rows="4"
                            aria-label="Entrada estándar para read(0, n)"
                            placeholder="Texto listo para read(0, bytes) durante la ejecución..."
                            spellcheck="false"
                            prop:value=move || stdin_text.get()
                            on:input=move |ev| stdin_text.set(event_target_value(&ev))
                        />
                    </div>
                </Show>
            </div>
        </section>
    }
}
