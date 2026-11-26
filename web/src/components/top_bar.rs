//! Barra superior plana del Playground e isologo pixel art SVG de Wyrm.

use crate::engine::ExecutionMode;
use crate::examples::EXAMPLES;
use crate::storage::Palette;
use leptos::prelude::*;

/// Isologo Pixel Art 24×24 de un wyrm serpentino sin alas en curva geométrica.
#[component]
fn WyrmPixelLogo() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            shape-rendering="crispEdges"
            class="wyrm-pixel-logo"
        >
            <rect x="0" y="0" width="24" height="24" fill="var(--bg-surface)" />
            <g fill="var(--wyrm-scale-dark)">
                <rect x="6" y="19" width="12" height="3" />
                <rect x="12" y="13" width="9" height="3" />
                <rect x="8" y="4" width="2" height="11" />
                <rect x="15" y="12" width="5" height="8" />
                <rect x="14" y="5" width="4" height="4" />
                <rect x="15" y="11" width="4" height="10" />
                <rect x="6" y="6" width="14" height="2" />
                <rect x="8" y="4" width="9" height="4" />
                <rect x="7" y="5" width="12" height="3" />
                <rect x="15" y="13" width="6" height="4" />
                <rect x="6" y="18" width="4" height="4" />
                <rect x="6" y="11" width="5" height="3" />
            </g>
            <g fill="var(--wyrm-scale-light)">
                <rect x="8" y="4" width="8" height="4" />
                <rect x="15" y="12" width="4" height="8" />
                <rect x="5" y="7" width="4" height="6" />
                <rect x="11" y="19" width="5" height="4" />
                <rect x="5" y="18" width="4" height="3" />
                <rect x="7" y="5" width="11" height="3" />
                <rect x="15" y="11" width="3" height="10" />
                <rect x="10" y="12" width="9" height="3" />
                <rect x="15" y="13" width="5" height="4" />
                <rect x="6" y="6" width="3" height="8" />
            </g>
            <g fill="var(--wyrm-scale-light)">
                <rect x="4" y="9" width="5" height="3" />
                <rect x="9" y="3" width="6" height="5" />
                <rect x="14" y="11" width="4" height="5" />
                <rect x="14" y="17" width="4" height="4" />
                <rect x="4" y="19" width="6" height="1" />
                <rect x="6" y="6" width="13" height="2" />
                <rect x="10" y="20" width="6" height="3" />
                <rect x="13" y="18" width="5" height="3" />
                <rect x="6" y="11" width="4" height="3" />
            </g>
            <g fill="var(--wyrm-scale-mid)">
                <rect x="13" y="12" width="4" height="3" />
                <rect x="7" y="20" width="9" height="1" />
                <rect x="12" y="5" width="3" height="3" />
                <rect x="6" y="8" width="2" height="4" />
                <rect x="8" y="13" width="10" height="1" />
                <rect x="17" y="14" width="2" height="3" />
                <rect x="11" y="7" width="8" height="1" />
                <rect x="15" y="18" width="2" height="2" />
                <rect x="7" y="12" width="3" height="1" />
                <rect x="9" y="5" width="6" height="1" />
            </g>
            <g fill="var(--wyrm-scale-mid)">
                <rect x="13" y="19" width="3" height="2" />
                <rect x="8" y="6" width="2" height="1" />
                <rect x="7" y="7" width="2" height="1" />
                <rect x="16" y="17" width="2" height="1" />
                <rect x="6" y="19" width="2" height="1" />
                <rect x="12" y="13" width="6" height="2" />
                <rect x="12" y="6" width="4" height="2" />
                <rect x="16" y="13" width="2" height="3" />
                <rect x="5" y="10" width="3" height="1" />
            </g>
            <g fill="var(--accent-gold)">
                <rect x="10" y="22" width="6" height="1" />
                <rect x="8" y="1" width="2" height="2" />
                <rect x="13" y="2" width="2" height="2" />
                <rect x="9" y="2" width="2" height="2" />
                <rect x="14" y="10" width="3" height="1" />
                <rect x="6" y="17" width="3" height="1" />
                <rect x="7" y="1" width="3" height="1" />
                <rect x="17" y="7" width="1" height="1" />
                <rect x="13" y="11" width="1" height="1" />
                <rect x="5" y="18" width="1" height="1" />
            </g>
            <rect x="11" y="5" width="2" height="1" fill="var(--accent-violet)" />
        </svg>
    }
}

/// Barra superior plana estilo grilla continua: segmentos divididos por líneas verticales de 1px.
#[component]
pub fn TopBar(
    palette: RwSignal<Palette>,
    selected_example_id: RwSignal<String>,
    mode: RwSignal<ExecutionMode>,
    verbose: RwSignal<bool>,
    is_running: RwSignal<bool>,
    show_running_indicator: RwSignal<bool>,
    on_toggle_palette: Callback<()>,
    on_select_example: Callback<String>,
    on_run: Callback<()>,
    on_trace: Callback<()>,
    on_stop: Callback<()>,
) -> impl IntoView {
    view! {
        <header class="topbar">
            <div class="topbar-brand topbar-segment">
                <div class="brand-emblem" aria-hidden="true">
                    <WyrmPixelLogo />
                </div>
                <div class="brand-text">
                    <span class="brand-name">"WYRM"</span>
                    <span class="brand-sep" aria-hidden="true">"::"</span>
                    <span class="brand-subtitle">"PLAYGROUND"</span>
                </div>
            </div>

            <div class="topbar-controls">
                <div class="topbar-segment control-group">
                    <label class="sr-only" for="example-select">"Ejemplo precargado"</label>
                    <select
                        id="example-select"
                        class="select-input"
                        title="Cargar un ejemplo precargado de Wyrm"
                        prop:value=move || selected_example_id.get()
                        on:change=move |ev| {
                            let id = event_target_value(&ev);
                            if !id.is_empty() {
                                on_select_example.run(id);
                            }
                        }
                    >
                        <option
                            value=""
                            disabled=true
                            selected=move || selected_example_id.get().is_empty()
                        >
                            "Código editado"
                        </option>
                        {EXAMPLES
                            .iter()
                            .map(|example| {
                                let id = example.id;
                                view! {
                                    <option
                                        value=id
                                        selected=move || selected_example_id.get() == id
                                    >
                                        {example.title}
                                    </option>
                                }
                            })
                            .collect_view()}
                    </select>
                </div>

                <div class="topbar-segment mode-switch" role="group" aria-label="Modo de ejecución">
                    <button
                        type="button"
                        class="mode-btn"
                        class:active=move || mode.get() == ExecutionMode::Interpreter
                        title="Evaluar directamente el AST con el intérprete"
                        on:click=move |_| mode.set(ExecutionMode::Interpreter)
                    >
                        {ExecutionMode::Interpreter.label()}
                    </button>
                    <button
                        type="button"
                        class="mode-btn"
                        class:active=move || mode.get() == ExecutionMode::CompilerVm
                        title="Compilar a bytecode .wrc y ejecutar en la Wyrm VM"
                        on:click=move |_| mode.set(ExecutionMode::CompilerVm)
                    >
                        {ExecutionMode::CompilerVm.label()}
                    </button>
                </div>

                <label
                    class="topbar-segment verbose-toggle"
                    class:active=move || verbose.get()
                    title="Habilita la traza detallada en la pestaña Inspector"
                >
                    <input
                        type="checkbox"
                        class="verbose-checkbox"
                        prop:checked=move || verbose.get()
                        on:change=move |ev| verbose.set(event_target_checked(&ev))
                    />
                    <span class="verbose-flag" aria-hidden="true">
                        {move || if verbose.get() { "[◈]" } else { "[ ]" }}
                    </span>
                    <span class="toggle-label">"VERBOSE"</span>
                </label>

                <button
                    type="button"
                    class="topbar-segment theme-toggle-btn"
                    title=move || palette.get().toggle_title()
                    on:click=move |_| on_toggle_palette.run(())
                >
                    <span class="theme-glyph" aria-hidden="true">{move || palette.get().glyph()}</span>
                    <span>{move || palette.get().label()}</span>
                </button>
            </div>

            <div class="topbar-actions">
                <button
                    type="button"
                    class="topbar-segment step-trace-btn"
                    disabled=move || is_running.get()
                    title=move || match mode.get() {
                        ExecutionMode::Interpreter => {
                            "Trazar recorrido del AST paso a paso en el Intérprete (Ctrl + Shift + Enter)"
                        }
                        ExecutionMode::CompilerVm => {
                            "Trazar bytecode paso a paso en la Wyrm VM (Ctrl + Shift + Enter)"
                        }
                    }
                    on:click=move |_| on_trace.run(())
                >
                    <span class="step-trace-glyph" aria-hidden="true">"▷"</span>
                    <span>"PASO A PASO"</span>
                </button>
                <Show
                    when=move || show_running_indicator.get()
                    fallback=move || {
                        view! {
                            <button
                                type="button"
                                class="btn btn-primary"
                                disabled=move || is_running.get()
                                title="Ejecutar código (Ctrl + Enter)"
                                on:click=move |_| on_run.run(())
                            >
                                <span class="run-glyph" aria-hidden="true">"▶"</span>
                                <span>"EJECUTAR"</span>
                                <kbd class="shortcut-hint">"Ctrl+↵"</kbd>
                            </button>
                        }
                    }
                >
                    <button
                        type="button"
                        class="btn btn-danger stop-active"
                        title="Detener el Web Worker actual (corta bucles infinitos)"
                        on:click=move |_| on_stop.run(())
                    >
                        <span class="stop-glyph" aria-hidden="true">"■"</span>
                        <span>"DETENER"</span>
                    </button>
                </Show>
            </div>
        </header>
    }
}
