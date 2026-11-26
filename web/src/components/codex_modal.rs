//! Modal técnico "Códice de Wyrm" con referencia de tipos, operadores, funciones y E/S.

use crate::components::WyrmAsciiArt;
use crate::wyrm_art;
use leptos::prelude::*;
use web_sys::MouseEvent;

/// Modal de referencia concisa del lenguaje Wyrm.
#[component]
pub fn CodexModal(show_codex_modal: RwSignal<bool>) -> impl IntoView {
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
