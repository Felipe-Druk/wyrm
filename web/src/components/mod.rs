//! Componentes de interfaz y utilidades DOM compartidas de WYRM Playground.

pub mod codex_modal;
pub mod editor;
pub mod output;
pub mod top_bar;
pub mod tutor_transport;

pub use codex_modal::CodexModal;
pub use editor::EditorPanel;
pub use output::{OutputPanel, OutputTab, TutorMode};
pub use top_bar::TopBar;
pub use tutor_transport::{handle_tutor_keydown, TutorTransport};

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Event, MouseEvent};

const PLAYGROUND_VERSION: &str = "v0.2.0";
const REPO_URL: &str = "https://github.com/Felipe-Druk/wyrm";

/// Listeners globales de arrastre sobre `window` con desconexión segura.
pub struct WindowDragListeners {
    on_move: Closure<dyn FnMut(MouseEvent)>,
    on_up: Closure<dyn FnMut(MouseEvent)>,
    on_blur: Closure<dyn FnMut(Event)>,
}

impl WindowDragListeners {
    /// Registra `mousemove`, `mouseup` y `blur` sobre `window`.
    pub fn attach<FM, FU>(mut on_move_cb: FM, on_up_cb: FU) -> Option<Self>
    where
        FM: FnMut(MouseEvent) + 'static,
        FU: Fn() + Clone + 'static,
    {
        let window = web_sys::window()?;
        let on_move = Closure::<dyn FnMut(MouseEvent)>::new(move |ev: MouseEvent| {
            on_move_cb(ev);
        });

        let finish_on_up = on_up_cb.clone();
        let on_up = Closure::<dyn FnMut(MouseEvent)>::new(move |_ev: MouseEvent| {
            finish_on_up();
        });

        let on_blur = Closure::<dyn FnMut(Event)>::new(move |_ev: Event| {
            on_up_cb();
        });

        let move_ok = window
            .add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref())
            .is_ok();
        let up_ok = window
            .add_event_listener_with_callback("mouseup", on_up.as_ref().unchecked_ref())
            .is_ok();
        let blur_ok = window
            .add_event_listener_with_callback("blur", on_blur.as_ref().unchecked_ref())
            .is_ok();

        let listeners = Self {
            on_move,
            on_up,
            on_blur,
        };

        if move_ok && up_ok && blur_ok {
            Some(listeners)
        } else {
            listeners.detach();
            None
        }
    }

    /// Quita los listeners registrados de `window` y libera los closures.
    pub fn detach(self) {
        if let Some(window) = web_sys::window() {
            let _ = window.remove_event_listener_with_callback(
                "mousemove",
                self.on_move.as_ref().unchecked_ref(),
            );
            let _ = window.remove_event_listener_with_callback(
                "mouseup",
                self.on_up.as_ref().unchecked_ref(),
            );
            let _ = window
                .remove_event_listener_with_callback("blur", self.on_blur.as_ref().unchecked_ref());
        }
    }
}

/// Renderiza el arte ASCII sombreado del wyrm envolviendo el ojo `◈` sin usar `inner_html`.
#[component]
pub fn WyrmAsciiArt(art: &'static str, class: &'static str) -> impl IntoView {
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

/// Pie de página con versión, estado del motor, Códice de Wyrm y enlace a GitHub.
#[component]
pub fn FooterBar(
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
