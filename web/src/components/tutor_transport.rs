//! Barra compartida de transporte paso a paso, scrubber y autoplay para Tutor VM y Tutor AST.

use leptos::leptos_dom::helpers::{set_timeout_with_handle, TimeoutHandle};
use leptos::prelude::*;
use std::time::Duration;
use web_sys::{KeyboardEvent, MouseEvent};

/// Velocidad de reproducción automática compartida por ambos modos de Wyrm Tutor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackSpeed {
    Half,
    Normal,
    Double,
}

impl PlaybackSpeed {
    pub fn delay_ms(self) -> u64 {
        match self {
            Self::Half => 1050,
            Self::Normal => 520,
            Self::Double => 200,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Half => "0.5x",
            Self::Normal => "1x",
            Self::Double => "2x",
        }
    }
}

/// Atajos de teclado compartidos (`←`, `→`, `Home`, `End`) para navegar pasos del Tutor.
pub fn handle_tutor_keydown(
    ev: KeyboardEvent,
    step_idx: RwSignal<usize>,
    is_playing: RwSignal<bool>,
    max_step: usize,
) {
    match ev.key().as_str() {
        "ArrowLeft" => {
            ev.prevent_default();
            is_playing.set(false);
            step_idx.update(|idx| *idx = idx.saturating_sub(1));
        }
        "ArrowRight" => {
            ev.prevent_default();
            is_playing.set(false);
            step_idx.update(|idx| {
                if *idx < max_step {
                    *idx += 1;
                }
            });
        }
        "Home" => {
            ev.prevent_default();
            is_playing.set(false);
            step_idx.set(0);
        }
        "End" => {
            ev.prevent_default();
            is_playing.set(false);
            step_idx.set(max_step);
        }
        _ => {}
    }
}

/// Barra de controles (`⏮`, `◀`, `▶/⏸`, `▶|`, `⏭`, velocidad, contador, truncado) y scrubber.
#[component]
pub fn TutorTransport(
    step_idx: RwSignal<usize>,
    is_playing: RwSignal<bool>,
    is_running: RwSignal<bool>,
    total_steps: usize,
    truncated: bool,
    max_limit: usize,
    transport_aria_label: &'static str,
    scrubber_aria_label: &'static str,
    truncated_title: &'static str,
) -> impl IntoView {
    let playback_speed = RwSignal::new(PlaybackSpeed::Normal);
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

    // Bucle de auto-play gobernado por TimeoutHandle cancelable
    Effect::new(move |_| {
        clear_play_timer();
        let running = is_running.get();
        let playing = is_playing.get();
        let speed = playback_speed.get();
        let current = step_idx.get();

        if running || !playing {
            if running && playing {
                is_playing.set(false);
            }
            return;
        }
        if total_steps == 0 || current >= total_steps {
            is_playing.set(false);
            return;
        }

        let next_gen = play_generation.get_value().wrapping_add(1);
        play_generation.set_value(next_gen);

        if let Ok(handle) = set_timeout_with_handle(
            move || {
                if play_generation.get_value() == next_gen && is_playing.get_untracked() {
                    let cur = step_idx.get_untracked();
                    if cur < total_steps {
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

    let at_start = move || step_idx.get() == 0;
    let at_end = move || step_idx.get() >= total_steps;

    let on_start = move |_: MouseEvent| {
        is_playing.set(false);
        step_idx.set(0);
    };

    let on_prev = move |_: MouseEvent| {
        is_playing.set(false);
        step_idx.update(|idx| *idx = idx.saturating_sub(1));
    };

    let on_toggle_play = move |_: MouseEvent| {
        if total_steps == 0 {
            return;
        }
        if is_playing.get_untracked() {
            is_playing.set(false);
        } else {
            if step_idx.get_untracked() >= total_steps {
                step_idx.set(0);
            }
            is_playing.set(true);
        }
    };

    let on_next = move |_: MouseEvent| {
        is_playing.set(false);
        step_idx.update(|idx| {
            if *idx < total_steps {
                *idx += 1;
            }
        });
    };

    let on_end = move |_: MouseEvent| {
        is_playing.set(false);
        step_idx.set(total_steps);
    };

    let max_attr = total_steps.to_string();

    view! {
        <div class="tutor-controls-bar">
            <div class="tutor-transport" role="group" aria-label=transport_aria_label>
                <button
                    type="button"
                    class="tutor-ctrl-btn"
                    disabled=at_start
                    title="Ir al inicio (Home)"
                    on:click=on_start
                >
                    "⏮"
                </button>
                <button
                    type="button"
                    class="tutor-ctrl-btn"
                    disabled=at_start
                    title="Paso anterior (←)"
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
                    title="Siguiente paso (→)"
                    on:click=on_next
                >
                    "▶|"
                </button>
                <button
                    type="button"
                    class="tutor-ctrl-btn"
                    disabled=at_end
                    title="Ir al paso final (End)"
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
                <span class="tutor-truncated-badge" title=truncated_title>
                    {format!("TRUNCADO ({max_limit})")}
                </span>
            </Show>
        </div>

        <div class="tutor-scrubber-row">
            <input
                type="range"
                class="tutor-scrubber"
                aria-label=scrubber_aria_label
                min="0"
                max=max_attr
                prop:value=move || step_idx.get().min(total_steps).to_string()
                on:input=move |ev| {
                    is_playing.set(false);
                    if let Ok(parsed) = event_target_value(&ev).parse::<usize>() {
                        step_idx.set(parsed.min(total_steps));
                    }
                }
            />
        </div>
    }
}
