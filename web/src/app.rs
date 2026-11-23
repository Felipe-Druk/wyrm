//! Componente principal y vistas de interfaz para WYRM Playground ("Códice de Obsidiana").

use crate::engine::{self, ExecutionMode, ExecutionRequest, ExecutionResult};
use crate::examples::{self, EXAMPLES};
use crate::highlight;
use crate::wyrm_art;
use leptos::html::{Div, Pre, Textarea};
use leptos::leptos_dom::helpers::{set_timeout_with_handle, TimeoutHandle};
use leptos::prelude::*;
use std::time::Duration;
use web_sys::{KeyboardEvent, MouseEvent};

const PLAYGROUND_VERSION: &str = "v0.2.0";
const REPO_URL: &str = "https://github.com/Felipe-Druk/wyrm";
const TAB_SPACES: &str = "    ";
const THEME_STORAGE_KEY: &str = "wyrm-theme";
const RUNNING_INDICATOR_DELAY_MS: u64 = 150;
const RUN_PULSE_DURATION_MS: u64 = 300;

/// Pestaña elegida en el panel de resultados.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputTab {
    Console,
    Inspector,
}

/// Tema dracónico conmutable del Playground.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Theme {
    Obsidian,
    Silver,
}

impl Theme {
    fn as_str(self) -> &'static str {
        match self {
            Self::Obsidian => "obsidian",
            Self::Silver => "silver",
        }
    }

    fn from_storage(raw: &str) -> Self {
        match raw {
            "silver" => Self::Silver,
            _ => Self::Obsidian,
        }
    }

    fn toggle(self) -> Self {
        match self {
            Self::Obsidian => Self::Silver,
            Self::Silver => Self::Obsidian,
        }
    }

    fn glyph(self) -> &'static str {
        match self {
            Self::Obsidian => "◈",
            Self::Silver => "◇",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Obsidian => "OBSIDIANA",
            Self::Silver => "PLATA",
        }
    }

    fn toggle_title(self) -> &'static str {
        match self {
            Self::Obsidian => "Cambiar a tema Plata (Dragón Blanco)",
            Self::Silver => "Cambiar a tema Obsidiana",
        }
    }
}

/// Etiqueta compacta del modo de motor para la cabecera de telemetría.
fn mode_short_label(mode: ExecutionMode) -> &'static str {
    match mode {
        ExecutionMode::Interpreter => "INTÉRPRETE",
        ExecutionMode::CompilerVm => "VM",
    }
}

/// Recupera el tema persistido en `window.localStorage`.
fn load_saved_theme() -> Theme {
    web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(THEME_STORAGE_KEY).ok().flatten())
        .map(|raw| Theme::from_storage(&raw))
        .unwrap_or(Theme::Obsidian)
}

/// Persiste el tema activo en `window.localStorage`.
fn save_theme(theme: Theme) {
    if let Some(storage) = web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.set_item(THEME_STORAGE_KEY, theme.as_str());
    }
}

/// Sincroniza `data-theme` en `<html>` para cubrir fondo raíz y barras de desplazamiento.
fn apply_document_theme(theme: Theme) {
    if let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|doc| doc.document_element())
    {
        let _ = root.set_attribute("data-theme", theme.as_str());
    }
}

/// Cuenta la cantidad de líneas de un buffer de código.
fn count_lines(text: &str) -> usize {
    text.split('\n').count().max(1)
}

/// Traduce un desplazamiento UTF-16 (DOM `selectionStart`/`selectionEnd`)
/// al índice de byte UTF-8 correspondiente dentro de un `&str` de Rust.
fn utf16_offset_to_byte_index(text: &str, utf16_offset: usize) -> usize {
    let mut utf16_count = 0usize;
    for (byte_idx, ch) in text.char_indices() {
        if utf16_count >= utf16_offset {
            return byte_idx;
        }
        utf16_count += ch.len_utf16();
    }
    text.len()
}

/// Calcula línea y columna (1-indexadas) para un desplazamiento UTF-16 del cursor.
fn cursor_line_col(text: &str, utf16_offset: usize) -> (usize, usize) {
    let byte_idx = utf16_offset_to_byte_index(text, utf16_offset);
    let prefix = &text[..byte_idx];
    let line = prefix.bytes().filter(|&b| b == b'\n').count() + 1;
    let col = prefix.rsplit('\n').next().map_or(0, |s| s.chars().count()) + 1;
    (line, col)
}

/// Renderiza el arte ASCII sombreado del wyrm envolviendo el ojo `◈` sin usar `inner_html`.
#[component]
fn WyrmAsciiArt(art: &'static str, class: &'static str) -> impl IntoView {
    let (before_eye, after_eye) = art.split_once('◈').unwrap_or((art, ""));
    let has_eye = art.contains('◈');

    view! {
        <pre class=class aria-hidden="true">
            {before_eye}
            {has_eye.then(|| view! { <span class="wyrm-ascii-eye">"◈"</span> })}
            {after_eye}
        </pre>
    }
}

/// Componente raíz del Playground de Wyrm.
#[component]
pub fn App() -> impl IntoView {
    let initial = examples::default_example();
    let initial_theme = load_saved_theme();

    let theme = RwSignal::new(initial_theme);
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
    let show_codex_modal = RwSignal::new(false);

    let run_generation = StoredValue::new(0u64);
    let running_timer = StoredValue::new(None::<TimeoutHandle>);
    let pulse_generation = StoredValue::new(0u64);
    let pulse_timer = StoredValue::new(None::<TimeoutHandle>);

    Effect::new(move |_| {
        let current = theme.get();
        apply_document_theme(current);
        save_theme(current);
    });

    on_cleanup(move || {
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

    let trigger_run_pulse = move |run_number: u32| {
        pulse_timer.update_value(|slot| {
            if let Some(handle) = slot.take() {
                handle.clear();
            }
        });
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

    let on_toggle_theme = Callback::new(move |()| {
        theme.update(|current| *current = current.toggle());
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
            run_count.set(0);
        }
    });

    let on_global_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Escape" && show_codex_modal.get_untracked() {
            show_codex_modal.set(false);
        }
    };

    view! {
        <div
            class="playground-shell"
            data-theme=move || theme.get().as_str()
            on:keydown=on_global_keydown
        >
            <TopBar
                theme=theme
                selected_example_id=selected_example_id
                mode=mode
                verbose=verbose
                is_running=is_running
                show_running_indicator=show_running_indicator
                on_toggle_theme=on_toggle_theme
                on_select_example=on_select_example
                on_run=on_run
                on_stop=on_stop
            />

            <main class="workspace-split">
                <EditorPanel
                    code=code
                    selected_example_id=selected_example_id
                    on_run=on_run
                />

                <OutputPanel
                    mode=mode
                    last_mode=last_mode
                    verbose=verbose
                    active_tab=active_tab
                    show_running_indicator=show_running_indicator
                    run_count=run_count
                    pulse_phase=pulse_phase
                    result=result
                    stdin_text=stdin_text
                    stdin_open=stdin_open
                    on_clear=on_clear
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
fn TopBar(
    theme: RwSignal<Theme>,
    selected_example_id: RwSignal<String>,
    mode: RwSignal<ExecutionMode>,
    verbose: RwSignal<bool>,
    is_running: RwSignal<bool>,
    show_running_indicator: RwSignal<bool>,
    on_toggle_theme: Callback<()>,
    on_select_example: Callback<String>,
    on_run: Callback<()>,
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
                    title=move || theme.get().toggle_title()
                    on:click=move |_| on_toggle_theme.run(())
                >
                    <span class="theme-glyph" aria-hidden="true">{move || theme.get().glyph()}</span>
                    <span>{move || theme.get().label()}</span>
                </button>
            </div>

            <div class="topbar-actions">
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

/// Panel izquierdo con cabecera `main.wyrm`, numeración sincronizada y editor de código.
#[component]
fn EditorPanel(
    code: RwSignal<String>,
    selected_example_id: RwSignal<String>,
    on_run: Callback<()>,
) -> impl IntoView {
    let textarea_ref = NodeRef::<Textarea>::new();
    let highlight_ref = NodeRef::<Pre>::new();
    let gutter_ref = NodeRef::<Div>::new();

    let cursor_line = RwSignal::new(1usize);
    let cursor_col = RwSignal::new(1usize);

    let line_count = Memo::new(move |_| count_lines(&code.get()));
    let char_count = Memo::new(move |_| code.get().chars().count());
    let tokens = Memo::new(move |_| highlight::tokenize(&code.get()));
    let needs_trailing_space = Memo::new(move |_| {
        let current = code.get();
        current.is_empty() || current.ends_with('\n')
    });

    let example_description = move || {
        let current_id = selected_example_id.get();
        examples::find_by_id(&current_id).map(|example| example.description)
    };

    let sync_editor_scroll = move || {
        if let Some(textarea) = textarea_ref.get_untracked() {
            let scroll_top = textarea.scroll_top();
            let scroll_left = textarea.scroll_left();
            if let Some(gutter) = gutter_ref.get_untracked() {
                gutter.set_scroll_top(scroll_top);
            }
            if let Some(pre) = highlight_ref.get_untracked() {
                pre.set_scroll_top(scroll_top);
                pre.set_scroll_left(scroll_left);
            }
        }
    };

    let update_cursor = move || {
        if let Some(textarea) = textarea_ref.get_untracked() {
            let value = textarea.value();
            let pos_utf16 = textarea.selection_start().ok().flatten().unwrap_or(0) as usize;
            let (line, col) = cursor_line_col(&value, pos_utf16);
            cursor_line.set(line);
            cursor_col.set(col);
        }
    };

    // Desactiva autocorrect en Safari/iOS una vez montado el textarea
    Effect::new(move |_| {
        if let Some(el) = textarea_ref.get() {
            let _ = el.set_attribute("autocorrect", "off");
        }
    });

    // Al cambiar de ejemplo, reiniciamos scroll en todas las capas y el cursor en Ln 1, Col 1
    Effect::new(move |_| {
        let current_id = selected_example_id.get();
        if !current_id.is_empty() {
            cursor_line.set(1);
            cursor_col.set(1);
            if let Some(textarea) = textarea_ref.get_untracked() {
                textarea.set_scroll_top(0);
                textarea.set_scroll_left(0);
            }
            if let Some(pre) = highlight_ref.get_untracked() {
                pre.set_scroll_top(0);
                pre.set_scroll_left(0);
            }
            if let Some(gutter) = gutter_ref.get_untracked() {
                gutter.set_scroll_top(0);
            }
        }
    });

    let on_editor_keydown = move |ev: KeyboardEvent| {
        if (ev.ctrl_key() || ev.meta_key()) && ev.key() == "Enter" {
            ev.prevent_default();
            on_run.run(());
            return;
        }

        if ev.key() == "Tab" {
            ev.prevent_default();
            if let Some(textarea) = textarea_ref.get_untracked() {
                let current_value = textarea.value();
                let raw_start = textarea.selection_start().ok().flatten().unwrap_or(0) as usize;
                let raw_end = textarea
                    .selection_end()
                    .ok()
                    .flatten()
                    .unwrap_or(raw_start as u32) as usize;

                let (start_utf16, end_utf16) = if raw_start <= raw_end {
                    (raw_start, raw_end)
                } else {
                    (raw_end, raw_start)
                };

                let start_byte = utf16_offset_to_byte_index(&current_value, start_utf16);
                let end_byte = utf16_offset_to_byte_index(&current_value, end_utf16);

                let mut updated = String::with_capacity(current_value.len() + TAB_SPACES.len());
                updated.push_str(&current_value[..start_byte]);
                updated.push_str(TAB_SPACES);
                updated.push_str(&current_value[end_byte..]);

                let next_cursor_utf16 = (start_utf16 + TAB_SPACES.len()) as u32;
                let (line, col) = cursor_line_col(&updated, next_cursor_utf16 as usize);
                textarea.set_value(&updated);
                let _ = textarea.set_selection_range(next_cursor_utf16, next_cursor_utf16);

                code.set(updated);
                selected_example_id.set(String::new());
                cursor_line.set(line);
                cursor_col.set(col);

                request_animation_frame(move || {
                    let _ = textarea.set_selection_range(next_cursor_utf16, next_cursor_utf16);
                    sync_editor_scroll();
                });
            }
        }
    };

    view! {
        <section class="panel editor-panel" aria-label="Editor de código Wyrm">
            <div class="panel-header editor-header">
                <div
                    class="file-tab"
                    title=move || example_description().unwrap_or("main.wyrm")
                >
                    <span class="file-glyph" aria-hidden="true">"◈"</span>
                    <span class="file-name">"main.wyrm"</span>
                    <Show when=move || example_description().is_some()>
                        <span class="file-meta-sep" aria-hidden="true">"::"</span>
                        <span class="file-example-hint">
                            {move || example_description().unwrap_or_default()}
                        </span>
                    </Show>
                </div>
                <div class="editor-stats" aria-live="polite">
                    <span class="cursor-pos">
                        {move || format!("Ln {}, Col {}", cursor_line.get(), cursor_col.get())}
                    </span>
                    <span class="stats-sep" aria-hidden="true">"::"</span>
                    <span>
                        {move || {
                            let lines = line_count.get();
                            if lines == 1 {
                                "1 L".to_string()
                            } else {
                                format!("{lines} L")
                            }
                        }}
                    </span>
                    <span class="stats-sep" aria-hidden="true">"·"</span>
                    <span>{move || format!("{} ch", char_count.get())}</span>
                </div>
            </div>

            <div class="editor-body">
                <div class="line-gutter" node_ref=gutter_ref aria-hidden="true">
                    {move || {
                        (1..=line_count.get())
                            .map(|line_number| {
                                let line_class = move || {
                                    if cursor_line.get() == line_number {
                                        "gutter-number line-number--active"
                                    } else {
                                        "gutter-number"
                                    }
                                };
                                view! { <span class=line_class>{line_number}</span> }
                            })
                            .collect_view()
                    }}
                </div>

                <div class="editor-stage">
                    <pre
                        node_ref=highlight_ref
                        class="code-highlight"
                        aria-hidden="true"
                    >
                        <code>
                            {move || {
                                tokens
                                    .get()
                                    .into_iter()
                                    .map(|(kind, text)| {
                                        view! { <span class=kind.css_class()>{text}</span> }
                                    })
                                    .collect_view()
                            }}
                            {move || if needs_trailing_space.get() { " " } else { "" }}
                        </code>
                    </pre>

                    <textarea
                        node_ref=textarea_ref
                        class="code-input"
                        aria-label="Código fuente en Wyrm"
                        placeholder="// Escribí tu programa en Wyrm aquí...\n// Atajo: Ctrl + Enter para ejecutar | Tab agrega 4 espacios"
                        spellcheck="false"
                        autocomplete="off"
                        autocapitalize="off"
                        prop:value=move || code.get()
                        on:input=move |ev| {
                            code.set(event_target_value(&ev));
                            selected_example_id.set(String::new());
                            update_cursor();
                            sync_editor_scroll();
                        }
                        on:keydown=on_editor_keydown
                        on:keyup=move |_| update_cursor()
                        on:click=move |_| update_cursor()
                        on:scroll=move |_| sync_editor_scroll()
                    />
                </div>
            </div>
        </section>
    }
}

/// Panel derecho con pestañas `Consola` e `Inspector`, telemetría fija, `Limpiar` y `stdin / fd 0`.
#[component]
fn OutputPanel(
    mode: RwSignal<ExecutionMode>,
    last_mode: RwSignal<ExecutionMode>,
    verbose: RwSignal<bool>,
    active_tab: RwSignal<OutputTab>,
    show_running_indicator: RwSignal<bool>,
    run_count: RwSignal<u32>,
    pulse_phase: RwSignal<u8>,
    result: RwSignal<Option<ExecutionResult>>,
    stdin_text: RwSignal<String>,
    stdin_open: RwSignal<bool>,
    on_clear: Callback<()>,
) -> impl IntoView {
    let has_inspector_output = Memo::new(move |_| {
        result
            .get()
            .map(|res| !res.inspector_log.is_empty())
            .unwrap_or(false)
    });

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
                {move || {
                    match (active_tab.get(), result.get()) {
                        (OutputTab::Console, None) => view! {
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
                        .into_any(),

                        (OutputTab::Console, Some(res)) => {
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
                            let return_display =
                                res.return_value.unwrap_or_else(|| "∅".to_string());
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
                                                    <span class="stream-glyph return" aria-hidden="true">
                                                        "◈"
                                                    </span>
                                                    <span class="stream-label">
                                                        "RETORNO DEL PROGRAMA"
                                                    </span>
                                                </div>
                                                <span
                                                    class="return-type-badge"
                                                    title="Tipo Wyrm del valor retornado"
                                                >
                                                    {return_type.clone()}
                                                </span>
                                            </div>
                                            <div class="return-body">
                                                <span class="return-prefix" aria-hidden="true">
                                                    "=>"
                                                </span>
                                                <pre class="return-value-pre">
                                                    {return_display.clone()}
                                                </pre>
                                            </div>
                                        </div>
                                    </Show>
                                </div>
                            }
                            .into_any()
                        }

                        (OutputTab::Inspector, None) => view! {
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
                        .into_any(),

                        (OutputTab::Inspector, Some(res)) => {
                            if res.inspector_log.is_empty() {
                                let hint = if verbose.get() {
                                    "Volvé a ejecutar el código para capturar la traza en modo VERBOSE."
                                } else {
                                    "El modo VERBOSE estaba deshabilitado en la última corrida. Activá [◈] VERBOSE y ejecutá nuevamente."
                                };
                                view! {
                                    <div class="empty-state">
                                        <p class="empty-title">"∅ SIN TRAZA DE INSPECCIÓN"</p>
                                        <p class="empty-hint">{hint}</p>
                                    </div>
                                }
                                .into_any()
                            } else {
                                view! {
                                    <div
                                        class="inspector-block corner-frame"
                                        class:run-pulse-a=is_pulsing_a
                                        class:run-pulse-b=is_pulsing_b
                                    >
                                        <div class="stream-header">
                                            <span class="stream-glyph inspector" aria-hidden="true">
                                                "::"
                                            </span>
                                            <span class="stream-label">"TRAZA VERBOSE DEL MOTOR"</span>
                                        </div>
                                        <pre class="terminal-pre inspector-pre">{res.inspector_log}</pre>
                                    </div>
                                }
                                .into_any()
                            }
                        }
                    }
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

/// Pie de página con versión, estado del motor, Códice de Wyrm y enlace a GitHub.
#[component]
fn FooterBar(
    show_running_indicator: RwSignal<bool>,
    show_codex_modal: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <footer class="footer-bar">
            <div class="footer-left">
                <span class="footer-version">{format!("WYRM {PLAYGROUND_VERSION}")}</span>
                <span class="footer-sep" aria-hidden="true">"::"</span>
                <span class="footer-status">
                    <span
                        class="status-glyph-engine"
                        class:running=move || show_running_indicator.get()
                        aria-hidden="true"
                    >
                        "◈"
                    </span>
                    {move || {
                        if show_running_indicator.get() {
                            "WASM :: EJECUTANDO EN WORKER..."
                        } else {
                            "WASM :: wyrm_core (Web Worker)"
                        }
                    }}
                </span>
            </div>

            <div class="footer-right">
                <button
                    type="button"
                    class="footer-link-btn"
                    on:click=move |_| show_codex_modal.set(true)
                >
                    <span aria-hidden="true">"◈ "</span>
                    <span>"Códice de Wyrm"</span>
                </button>
                <span class="footer-sep" aria-hidden="true">"::"</span>
                <a
                    class="footer-link"
                    href=REPO_URL
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    "GitHub / Felipe-Druk/wyrm"
                </a>
            </div>
        </footer>
    }
}

/// Modal técnico "Códice de Wyrm" con referencia de tipos, operadores, funciones y E/S.
#[component]
fn CodexModal(show_codex_modal: RwSignal<bool>) -> impl IntoView {
    view! {
        <div
            class="modal-backdrop"
            role="dialog"
            aria-modal="true"
            aria-label="Códice de Wyrm"
            on:click=move |_| show_codex_modal.set(false)
        >
            <div
                class="modal-card corner-frame"
                on:click=move |ev: MouseEvent| ev.stop_propagation()
            >
                <div class="modal-header">
                    <div class="modal-header-main">
                        <WyrmAsciiArt
                            art=wyrm_art::WYRM_CODEX_ART
                            class="wyrm-ascii wyrm-ascii--modal"
                        />
                        <div class="modal-heading-group">
                            <span class="modal-kicker">":: MANUAL DE REFERENCIA TÉCNICA ::"</span>
                            <h2 class="modal-title">"Códice de Wyrm"</h2>
                            <p class="modal-subtitle">
                                "Especificación concisa de tipos por ancho de bit, operadores, bloques, funciones y primitivas nativas de E/S."
                            </p>
                        </div>
                    </div>
                    <button
                        type="button"
                        class="modal-close-btn"
                        aria-label="Cerrar Códice de Wyrm"
                        title="Cerrar (Esc)"
                        on:click=move |_| show_codex_modal.set(false)
                    >
                        "✕"
                    </button>
                </div>

                <div class="modal-body">
                    <section class="codex-section">
                        <h3 class="codex-section-title">"01 :: TIPOS POR ANCHO DE BIT Y COMPUESTOS"</h3>
                        <div class="codex-table-wrap">
                            <table class="codex-table">
                                <thead>
                                    <tr>
                                        <th>"TIPO"</th>
                                        <th>"ANCHO"</th>
                                        <th>"SEMÁNTICA / NOTAS"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td><code>"nat8 / nat16 / nat32 / nat64"</code></td>
                                        <td class="col-width">"8..64 b"</td>
                                        <td>"Naturales sin signo. Caracteres ASCII literales (ej. 'W') son nat8."</td>
                                    </tr>
                                    <tr>
                                        <td><code>"int8 / int16 / int32 / int64"</code></td>
                                        <td class="col-width">"8..64 b"</td>
                                        <td>"Enteros con signo en complemento a dos."</td>
                                    </tr>
                                    <tr>
                                        <td><code>"float32 / float64"</code></td>
                                        <td class="col-width">"32 / 64 b"</td>
                                        <td>"Punto flotante IEEE 754 de precisión simple y doble."</td>
                                    </tr>
                                    <tr>
                                        <td><code>"bool"</code></td>
                                        <td class="col-width">"1 b"</td>
                                        <td>"Booleano estricto: true | false."</td>
                                    </tr>
                                    <tr>
                                        <td><code>"vec<T>"</code></td>
                                        <td class="col-width">"dinámico"</td>
                                        <td>"Vector mutable indexado desde 0. Cadenas = vec<nat8> cerradas con byte 0."</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </section>

                    <section class="codex-section">
                        <h3 class="codex-section-title">"02 :: OPERADORES Y COMENTARIOS"</h3>
                        <div class="codex-table-wrap">
                            <table class="codex-table">
                                <thead>
                                    <tr>
                                        <th>"SÍMBOLO"</th>
                                        <th>"CLASE"</th>
                                        <th>"COMPORTAMIENTO"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td><code>"// ..."</code></td>
                                        <td class="col-width">"comentario"</td>
                                        <td>"Comentario de línea única hasta salto de línea."</td>
                                    </tr>
                                    <tr>
                                        <td><code>"+  -  *  /  %  ^"</code></td>
                                        <td class="col-width">"aritmética"</td>
                                        <td>"Operaciones numéricas y potencia con ^ (ej. 2 ^ 8 = 256)."</td>
                                    </tr>
                                    <tr>
                                        <td><code>"==  ~=  <  <=  >  >="</code></td>
                                        <td class="col-width">"relacional"</td>
                                        <td>"Igualdad (==), desigualdad estricta (~=) y orden."</td>
                                    </tr>
                                    <tr>
                                        <td><code>"and  or  not  xor"</code></td>
                                        <td class="col-width">"lógica"</td>
                                        <td>"Conectivos booleanos por palabra reservada."</td>
                                    </tr>
                                    <tr>
                                        <td><code>"<-   ->"</code></td>
                                        <td class="col-width">"flujo / fn"</td>
                                        <td>"Iteración sobre vectores for (x <- vec) y firma/cuerpo (T) -> R."</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </section>

                    <div class="codex-dual-grid">
                        <section class="codex-section">
                            <h3 class="codex-section-title">"03 :: BLOQUES, CONTROL Y FUNCIONES (T) -> R"</h3>
                            <pre class="codex-code">
{r#"// Todo bloque { ... } retorna su última expresión
int32 r = if (x ~= 0) { x * 2 } else { 0 }

for (n <- nums) { total = total + n }
while (i < 10)  { i = i + 1 }

// Firma explícita (Tipos) -> Retorno
(int32, int32) -> int32 sumar = (a, b) -> {
    a + b
}
() -> () noop = () -> { }"#}
                            </pre>
                        </section>

                        <section class="codex-section">
                            <h3 class="codex-section-title">"04 :: PRIMITIVAS NATIVAS DE E/S"</h3>
                            <div class="codex-table-wrap">
                                <table class="codex-table">
                                    <thead>
                                        <tr>
                                            <th>"LLAMADA"</th>
                                            <th>"FD"</th>
                                            <th>"DESCRIPCIÓN"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"write(1, v)"</code></td>
                                            <td class="col-width">"fd 1"</td>
                                            <td>"Escribe vec<nat8> o cadena literal en stdout."</td>
                                        </tr>
                                        <tr>
                                            <td><code>"read(0, n)"</code></td>
                                            <td class="col-width">"fd 0"</td>
                                            <td>"Lee hasta n bytes desde stdin como vec<nat8>."</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>
                            <pre class="codex-code">
{r#"write(1, "Códice activo\n")
vec<nat8> buf = read(0, 32)
write(1, buf)"#}
                            </pre>
                        </section>
                    </div>
                </div>
            </div>
        </div>
    }
}
