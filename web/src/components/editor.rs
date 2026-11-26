//! Editor de código fuente de Wyrm con resaltado en overlay, gutter y cursor sincronizado.

use crate::examples;
use crate::highlight;
use leptos::html::{Div, Pre, Textarea};
use leptos::prelude::*;
use web_sys::KeyboardEvent;

const TAB_SPACES: &str = "    ";

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

/// Calcula línea y col (1-indexadas) para un desplazamiento UTF-16 del cursor.
fn cursor_line_col(text: &str, utf16_offset: usize) -> (usize, usize) {
    let byte_idx = utf16_offset_to_byte_index(text, utf16_offset);
    let prefix = &text[..byte_idx];
    let line = prefix.bytes().filter(|&b| b == b'\n').count() + 1;
    let col = prefix.rsplit('\n').next().map_or(0, |s| s.chars().count()) + 1;
    (line, col)
}

/// Panel izquierdo con cabecera `main.wyrm`, numeración sincronizada y editor de código.
#[component]
pub fn EditorPanel(
    code: RwSignal<String>,
    selected_example_id: RwSignal<String>,
    tutor_highlight_line: Memo<Option<usize>>,
    editor_width_pct: RwSignal<f64>,
    right_expanded: RwSignal<bool>,
    on_run: Callback<()>,
    on_trace: Callback<()>,
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

    // Al cambiar de ejemplo, reinicia scroll en todas las capas y el cursor en Ln 1, Col 1
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

    // Mantiene sincronizados overlay, gutter y textarea al cambiar el ancho o restaurar el editor
    Effect::new(move |_| {
        let _ = editor_width_pct.get();
        if !right_expanded.get() {
            request_animation_frame(move || {
                sync_editor_scroll();
            });
        }
    });

    let on_editor_keydown = move |ev: KeyboardEvent| {
        if (ev.ctrl_key() || ev.meta_key()) && ev.key() == "Enter" {
            ev.prevent_default();
            if ev.shift_key() {
                on_trace.run(());
            } else {
                on_run.run(());
            }
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
                                    let is_cursor = cursor_line.get() == line_number;
                                    let is_tutor = tutor_highlight_line.get() == Some(line_number);
                                    match (is_cursor, is_tutor) {
                                        (true, true) => {
                                            "gutter-number line-number--active line-number--tutor"
                                        }
                                        (true, false) => "gutter-number line-number--active",
                                        (false, true) => "gutter-number line-number--tutor",
                                        (false, false) => "gutter-number",
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
