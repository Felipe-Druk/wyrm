//! Componente principal y vistas de interfaz para WYRM Playground.

use crate::engine::{self, ExecutionMode, ExecutionRequest, ExecutionResult};
use crate::examples::{self, EXAMPLES};
use leptos::html::{Div, Textarea};
use leptos::prelude::*;
use web_sys::{KeyboardEvent, MouseEvent};

const PLAYGROUND_VERSION: &str = "v0.1.0";
const REPO_URL: &str = "https://github.com/Felipe-Druk/wyrm";
const TAB_SPACES: &str = "    ";

/// Pestaña elegida en el panel de resultados.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputTab {
    Console,
    Inspector,
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

/// Componente raíz del Playground de Wyrm.
#[component]
pub fn App() -> impl IntoView {
    let initial = examples::default_example();

    let selected_example_id = RwSignal::new(initial.id.to_string());
    let code = RwSignal::new(initial.code.to_string());
    let mode = RwSignal::new(ExecutionMode::Interpreter);
    let verbose = RwSignal::new(false);
    let stdin_text = RwSignal::new(initial.default_stdin.to_string());
    let stdin_open = RwSignal::new(false);
    let active_tab = RwSignal::new(OutputTab::Console);
    let result = RwSignal::new(None::<ExecutionResult>);
    let show_syntax_modal = RwSignal::new(false);

    let on_run = Callback::new(move |()| {
        let request = ExecutionRequest {
            source_code: code.get_untracked(),
            mode: mode.get_untracked(),
            verbose: verbose.get_untracked(),
            stdin: stdin_text.get_untracked(),
        };
        let execution_result = engine::execute(&request);
        result.set(Some(execution_result));
    });

    let on_clear = Callback::new(move |()| {
        code.set(String::new());
        result.set(None);
        selected_example_id.set(String::new());
    });

    let on_select_example = Callback::new(move |example_id: String| {
        if let Some(example) = examples::find_by_id(&example_id) {
            selected_example_id.set(example.id.to_string());
            code.set(example.code.to_string());
            stdin_text.set(example.default_stdin.to_string());
            result.set(None);
        }
    });

    let on_global_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Escape" && show_syntax_modal.get_untracked() {
            show_syntax_modal.set(false);
        }
    };

    view! {
        <div class="playground-shell" on:keydown=on_global_keydown>
            <TopBar
                selected_example_id=selected_example_id
                mode=mode
                verbose=verbose
                on_select_example=on_select_example
                on_run=on_run
                on_clear=on_clear
            />

            <main class="workspace-split">
                <EditorPanel
                    code=code
                    selected_example_id=selected_example_id
                    on_run=on_run
                />

                <OutputPanel
                    mode=mode
                    verbose=verbose
                    active_tab=active_tab
                    result=result
                    stdin_text=stdin_text
                    stdin_open=stdin_open
                />
            </main>

            <FooterBar show_syntax_modal=show_syntax_modal />

            <Show when=move || show_syntax_modal.get()>
                <SyntaxGuideModal show_syntax_modal=show_syntax_modal />
            </Show>
        </div>
    }
}

/// Barra superior con marca, selector de ejemplos, modo de motor, verbose y acciones.
#[component]
fn TopBar(
    selected_example_id: RwSignal<String>,
    mode: RwSignal<ExecutionMode>,
    verbose: RwSignal<bool>,
    on_select_example: Callback<String>,
    on_run: Callback<()>,
    on_clear: Callback<()>,
) -> impl IntoView {
    view! {
        <header class="topbar">
            <div class="topbar-brand">
                <div class="brand-emblem" aria-hidden="true">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 32 32"
                        fill="none"
                        class="brand-icon"
                    >
                        <path
                            d="M5.5 9.5L10.5 23.5L16 13.5L21.5 23.5L26.5 9.5"
                            stroke="currentColor"
                            stroke-width="2.6"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                        <path
                            d="M12.5 6.5L16 3.5L19.5 6.5"
                            stroke="#d29922"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                    </svg>
                </div>
                <div class="brand-text">
                    <span class="brand-name">"WYRM"</span>
                    <span class="brand-subtitle">"Playground"</span>
                </div>
            </div>

            <div class="topbar-controls">
                <div class="control-group">
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

                <div class="mode-switch" role="group" aria-label="Modo de ejecución">
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

                <label class="verbose-toggle" title="Habilita la traza detallada en la pestaña Inspector">
                    <input
                        type="checkbox"
                        class="verbose-checkbox"
                        prop:checked=move || verbose.get()
                        on:change=move |ev| verbose.set(event_target_checked(&ev))
                    />
                    <span class="toggle-track" aria-hidden="true">
                        <span class="toggle-thumb"></span>
                    </span>
                    <span class="toggle-label">"Verbose"</span>
                </label>
            </div>

            <div class="topbar-actions">
                <button
                    type="button"
                    class="btn btn-primary"
                    title="Ejecutar código (Ctrl + Enter)"
                    on:click=move |_| on_run.run(())
                >
                    <span class="run-glyph" aria-hidden="true">"▶"</span>
                    <span>"Ejecutar"</span>
                    <kbd class="shortcut-hint">"Ctrl+↵"</kbd>
                </button>
                <button
                    type="button"
                    class="btn btn-secondary"
                    title="Limpiar el editor y la salida actual"
                    on:click=move |_| on_clear.run(())
                >
                    "Limpiar"
                </button>
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
    let gutter_ref = NodeRef::<Div>::new();

    let line_count = Memo::new(move |_| count_lines(&code.get()));
    let char_count = Memo::new(move |_| code.get().chars().count());

    let example_description = move || {
        let current_id = selected_example_id.get();
        examples::find_by_id(&current_id).map(|example| example.description)
    };

    let sync_gutter_scroll = move |_| {
        if let (Some(textarea), Some(gutter)) =
            (textarea_ref.get_untracked(), gutter_ref.get_untracked())
        {
            gutter.set_scroll_top(textarea.scroll_top());
        }
    };

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
                textarea.set_value(&updated);
                let _ = textarea.set_selection_range(next_cursor_utf16, next_cursor_utf16);

                code.set(updated);
                selected_example_id.set(String::new());

                request_animation_frame(move || {
                    let _ = textarea.set_selection_range(next_cursor_utf16, next_cursor_utf16);
                });
            }
        }
    };

    view! {
        <section class="panel editor-panel" aria-label="Editor de código Wyrm">
            <div class="panel-header">
                <div class="file-tab">
                    <span class="file-dot" aria-hidden="true"></span>
                    <span class="file-name">"main.wyrm"</span>
                </div>
                <div class="editor-stats" aria-live="polite">
                    <span>
                        {move || {
                            let lines = line_count.get();
                            if lines == 1 {
                                "1 línea".to_string()
                            } else {
                                format!("{lines} líneas")
                            }
                        }}
                    </span>
                    <span class="stats-sep" aria-hidden="true">"·"</span>
                    <span>
                        {move || {
                            let chars = char_count.get();
                            if chars == 1 {
                                "1 carácter".to_string()
                            } else {
                                format!("{chars} caracteres")
                            }
                        }}
                    </span>
                </div>
            </div>

            <Show when=move || example_description().is_some()>
                <div class="editor-context-bar">
                    <span class="context-badge">"Ejemplo"</span>
                    <span class="context-text">
                        {move || example_description().unwrap_or_default()}
                    </span>
                </div>
            </Show>

            <div class="editor-body">
                <div class="line-gutter" node_ref=gutter_ref aria-hidden="true">
                    {move || {
                        (1..=line_count.get())
                            .map(|line_number| {
                                view! { <span class="gutter-number">{line_number}</span> }
                            })
                            .collect_view()
                    }}
                </div>

                <textarea
                    node_ref=textarea_ref
                    class="code-textarea"
                    aria-label="Código fuente en Wyrm"
                    placeholder="// Escribí tu programa en Wyrm aquí...\n// Atajo: Ctrl + Enter para ejecutar | Tab agrega 4 espacios"
                    spellcheck="false"
                    autocomplete="off"
                    autocapitalize="off"
                    prop:value=move || code.get()
                    on:input=move |ev| {
                        code.set(event_target_value(&ev));
                        selected_example_id.set(String::new());
                    }
                    on:keydown=on_editor_keydown
                    on:scroll=sync_gutter_scroll
                />
            </div>
        </section>
    }
}

/// Panel derecho con pestañas `Consola` e `Inspector (Verbose)`, badges y bloque `stdin / fd 0`.
#[component]
fn OutputPanel(
    mode: RwSignal<ExecutionMode>,
    verbose: RwSignal<bool>,
    active_tab: RwSignal<OutputTab>,
    result: RwSignal<Option<ExecutionResult>>,
    stdin_text: RwSignal<String>,
    stdin_open: RwSignal<bool>,
) -> impl IntoView {
    let has_inspector_output = Memo::new(move |_| {
        result
            .get()
            .map(|res| !res.inspector_log.is_empty())
            .unwrap_or(false)
    });

    let stdin_bytes = Memo::new(move |_| stdin_text.get().len());

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
                        "Consola"
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
                        <span>"Inspector (Verbose)"</span>
                        <Show when=move || has_inspector_output.get()>
                            <span class="tab-indicator" title="Traza verbose lista"></span>
                        </Show>
                    </button>
                </div>

                <div class="result-badges">
                    {move || match result.get() {
                        Some(res) => {
                            let return_label = res
                                .return_value
                                .unwrap_or_else(|| "void".to_string());
                            let time_label = format!("{:.2} ms", res.elapsed_ms);
                            let is_error = res.is_error;
                            view! {
                                <div class="badge-group">
                                    <span
                                        class="badge badge-return"
                                        class:error=is_error
                                        title="Valor de retorno implícito del programa"
                                    >
                                        <span class="badge-key">"Retorno:"</span>
                                        <span class="badge-val">{return_label}</span>
                                    </span>
                                    <span class="badge badge-time" title="Tiempo total de ejecución">
                                        {time_label}
                                    </span>
                                </div>
                            }
                            .into_any()
                        }
                        None => view! {
                            <span class="badge badge-idle">
                                {move || format!("Listo · {}", mode.get().label())}
                            </span>
                        }
                        .into_any(),
                    }}
                </div>
            </div>

            <div class="output-viewport">
                {move || match (active_tab.get(), result.get()) {
                    (OutputTab::Console, None) => view! {
                        <div class="empty-state">
                            <p class="empty-title">"Sin ejecución todavía"</p>
                            <p class="empty-hint">
                                "Presioná "
                                <kbd>"▶ Ejecutar"</kbd>
                                " o "
                                <kbd>"Ctrl + Enter"</kbd>
                                " dentro del editor para evaluar "
                                <code>"main.wyrm"</code>
                                " con "
                                <strong>{move || mode.get().label()}</strong>
                                "."
                            </p>
                        </div>
                    }
                    .into_any(),

                    (OutputTab::Console, Some(res)) => {
                        let is_error = res.is_error;
                        let stdout_content = if res.stdout.is_empty() {
                            "(stdout vacío — el programa no escribió en fd 1)".to_string()
                        } else {
                            res.stdout
                        };
                        let status_label = if is_error {
                            "Error de ejecución"
                        } else {
                            "stdout (fd 1)"
                        };

                        view! {
                            <div class="console-block" class:error=is_error>
                                <div class="stream-header">
                                    <span class="stream-dot" class:error=is_error></span>
                                    <span class="stream-label">{status_label}</span>
                                </div>
                                <pre class="terminal-pre" class:error=is_error>
                                    {stdout_content}
                                </pre>
                            </div>
                        }
                        .into_any()
                    }

                    (OutputTab::Inspector, None) => view! {
                        <div class="empty-state">
                            <p class="empty-title">"Inspector en espera"</p>
                            <p class="empty-hint">
                                "Habilitá el switch "
                                <strong>"Verbose"</strong>
                                " en la barra superior y ejecutá el programa para inspeccionar tokens, AST o bytecode "
                                <code>".wrc"</code>
                                "."
                            </p>
                        </div>
                    }
                    .into_any(),

                    (OutputTab::Inspector, Some(res)) => {
                        if res.inspector_log.is_empty() {
                            let hint = if verbose.get() {
                                "Volvé a ejecutar el código para capturar la traza en modo Verbose."
                            } else {
                                "El modo Verbose estaba deshabilitado en la última ejecución. Habilitá el switch Verbose y ejecutá nuevamente."
                            };
                            view! {
                                <div class="empty-state">
                                    <p class="empty-title">"Sin traza de inspección"</p>
                                    <p class="empty-hint">{hint}</p>
                                </div>
                            }
                            .into_any()
                        } else {
                            view! {
                                <div class="inspector-block">
                                    <div class="stream-header">
                                        <span class="stream-dot inspector"></span>
                                        <span class="stream-label">"Traza Verbose del motor"</span>
                                    </div>
                                    <pre class="terminal-pre inspector-pre">{res.inspector_log}</pre>
                                </div>
                            }
                            .into_any()
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
                        <span class="stdin-title">"Entrada estándar (stdin / fd 0)"</span>
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
                            "Vaciar"
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

/// Pie de página con versión, estado del motor, guía de sintaxis y enlace a GitHub.
#[component]
fn FooterBar(show_syntax_modal: RwSignal<bool>) -> impl IntoView {
    view! {
        <footer class="footer-bar">
            <div class="footer-left">
                <span class="footer-version">{format!("WYRM Playground {PLAYGROUND_VERSION}")}</span>
                <span class="footer-sep" aria-hidden="true">"·"</span>
                <span class="footer-status">
                    <span class="status-dot-amber" aria-hidden="true"></span>
                    "Motor WASM: mock listo para enlace"
                </span>
            </div>

            <div class="footer-right">
                <button
                    type="button"
                    class="footer-link-btn"
                    on:click=move |_| show_syntax_modal.set(true)
                >
                    "Guía rápida de sintaxis Wyrm"
                </button>
                <span class="footer-sep" aria-hidden="true">"·"</span>
                <a
                    class="footer-link"
                    href=REPO_URL
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    "GitHub · Felipe-Druk/wyrm"
                </a>
            </div>
        </footer>
    }
}

/// Modal con referencia rápida y fiel a la gramática de Wyrm.
#[component]
fn SyntaxGuideModal(show_syntax_modal: RwSignal<bool>) -> impl IntoView {
    view! {
        <div
            class="modal-backdrop"
            role="dialog"
            aria-modal="true"
            aria-label="Guía rápida de sintaxis Wyrm"
            on:click=move |_| show_syntax_modal.set(false)
        >
            <div
                class="modal-card"
                on:click=move |ev: MouseEvent| ev.stop_propagation()
            >
                <div class="modal-header">
                    <div>
                        <h2 class="modal-title">"Guía rápida de sintaxis Wyrm"</h2>
                        <p class="modal-subtitle">
                            "Resumen de tipos, operadores, bloques y funciones según la gramática de Wyrm."
                        </p>
                    </div>
                    <button
                        type="button"
                        class="modal-close-btn"
                        aria-label="Cerrar guía"
                        on:click=move |_| show_syntax_modal.set(false)
                    >
                        "×"
                    </button>
                </div>

                <div class="modal-body">
                    <div class="guide-grid">
                        <article class="guide-card">
                            <h3 class="guide-card-title">"1. Tipos explícitos"</h3>
                            <p class="guide-card-note">
                                "Enteros con signo, naturales, flotantes y booleanos. Los caracteres ASCII son "
                                <code>"nat8"</code>
                                "."
                            </p>
                            <pre class="guide-code">
{r#"int8 / int16 / int32 / int64
nat8 / nat16 / nat32 / nat64
float32 / float64 / bool

int32 edad = 24
nat8 letra = 'W'
bool jugando = true"#}
                            </pre>
                        </article>

                        <article class="guide-card">
                            <h3 class="guide-card-title">"2. Vectores y cadenas"</h3>
                            <p class="guide-card-note">
                                "Los vectores dinámicos usan "
                                <code>"vec<T>"</code>
                                " y las cadenas son "
                                <code>"vec<nat8>"</code>
                                " cerradas con "
                                <code>"0"</code>
                                "."
                            </p>
                            <pre class="guide-code">
{r#"vec<int32> nums = [10, 20, 30]
nums[0] = 99

vec<nat8> msg = "Hola Wyrm!\n""#}
                            </pre>
                        </article>

                        <article class="guide-card">
                            <h3 class="guide-card-title">"3. Operadores y comentarios"</h3>
                            <p class="guide-card-note">
                                "Desigualdad con "
                                <code>"~="</code>
                                ", potencia con "
                                <code>"^"</code>
                                " y lógica con palabras reservadas."
                            </p>
                            <pre class="guide-code">
{r#"// Comentario de línea
int64 p = 2 ^ 8
bool ok = (p ~= 0) and (p >= 100)
bool inv = not ok or (true xor false)"#}
                            </pre>
                        </article>

                        <article class="guide-card">
                            <h3 class="guide-card-title">"4. Bloques y control de flujo"</h3>
                            <p class="guide-card-note">
                                "Todo bloque "
                                <code>"{ ... }"</code>
                                " retorna implícitamente su última expresión. "
                                <code>"for (n <- vec)"</code>
                                " itera vectores."
                            </p>
                            <pre class="guide-code">
{r#"if (x > 0) { x } else { 0 }

while (i < 10) {
    i = i + 1
}

for (n <- nums) {
    total = total + n
}"#}
                            </pre>
                        </article>

                        <article class="guide-card">
                            <h3 class="guide-card-title">"5. Funciones y lambdas"</h3>
                            <p class="guide-card-note">
                                "Firma explícita "
                                <code>"(Tipos) -> Retorno"</code>
                                " antes del nombre y parámetros con "
                                <code>"->"</code>
                                "."
                            </p>
                            <pre class="guide-code">
{r#"(int32, int32) -> int32 sumar = (a, b) -> {
    a + b
}

() -> () noop = () -> { }"#}
                            </pre>
                        </article>

                        <article class="guide-card">
                            <h3 class="guide-card-title">"6. Primitivas nativas de E/S"</h3>
                            <p class="guide-card-note">
                                "Flujos estándar (fd): "
                                <code>"0"</code>
                                " (stdin) y "
                                <code>"1"</code>
                                " (stdout)."
                            </p>
                            <pre class="guide-code">
{r#"write(1, "Salida por stdout\n")

vec<nat8> buf = read(0, 32)
write(1, buf)"#}
                            </pre>
                        </article>
                    </div>
                </div>
            </div>
        </div>
    }
}
