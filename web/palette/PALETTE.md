# Wyrm Design System — Paleta Oficial de Colores

Referencia técnica de todos los tokens reales auditados desde `web/styles.css` y `web/src/app.rs` (sin redondeos ni valores inventados).

## Archivos exportados

| Archivo | Formato | Uso |
| --- | --- | --- |
| `wyrm.tokens.json` | W3C DTCG (`$value`, `$type`, `$extensions`) | Fuente de verdad estructurada con `rgb`, `hsl` y origen CSS |
| `wyrm-palette.css` | CSS Custom Properties (`[data-palette]`) | Hoja standalone lista para importar en sitios o documentación |
| `wyrm-obsidian.gpl` | GIMP / Aseprite Palette | 33 colores sólidos deduplicados de **Wyrm Obsidian** |
| `wyrm-silver.gpl` | GIMP / Aseprite Palette | 31 colores sólidos deduplicados de **Wyrm Silver** |
| `wyrm-obsidian.base16.yaml` | Base16 YAML (`base00`..`base0F`) | Esquema para terminal, Neovim y Omarchy |

> Nota semántica sobre `status`: Wyrm prescinde de verdes o amarillos arbitrarios en la interfaz de estado; usa oro/acero para corridas OK, fase `ENTER` y camino activo del AST, violeta/azul hielo para `EVAL`, ejecución en Worker y cambios `NEW`/`UPD` del Tutor, y carmesí para errores, `DETENER` y truncado.

## Wyrm Obsidian (`obsidian`)

Escamas negras, oro bruñido, luz violeta y sintaxis Monokai Classic.

### Surface (Superficies, fondos y escamas)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `surface.base` | `#09080C` | `rgb(9, 8, 12)` | `--bg-base` | Fondo raíz obsidiana profunda del viewport y la consola. |
| `surface.inset` | `#0B0A10` | `rgb(11, 10, 16)` | `--bg-inset` | Superficie hundida del lienzo SVG del AST. |
| `surface.gutter` | `#0D0B12` | `rgb(13, 11, 18)` | `--gutter-bg` | Fondo de la franja de numeración de líneas del editor. |
| `surface.panel` | `#110F17` | `rgb(17, 15, 23)` | `--bg-panel` | Fondo estructural de paneles, topbar, footer y bloques de salida. |
| `surface.scale-dark` | `#151220` | `rgb(21, 18, 32)` | `--wyrm-scale-dark` | Escama oscura del isologo pixel art SVG de Wyrm. |
| `surface.elevated` | `#181522` | `rgb(24, 21, 34)` | `--bg-surface` | Superficie elevada para cabeceras de panel, tabs y grips de divisor. |
| `surface.scrollbar-thumb` | `#2C2640` | `rgb(44, 38, 64)` | `--scrollbar-thumb` | Indicador deslizable en reposo de las barras de desplazamiento. |
| `surface.scale-mid` | `#2E2742` | `rgb(46, 39, 66)` | `--wyrm-scale-mid` | Escama intermedia del isologo pixel art SVG de Wyrm. |
| `surface.scale-light` | `#5B4F7D` | `rgb(91, 79, 125)` | `--wyrm-scale-light` | Escama iluminada del isologo pixel art SVG de Wyrm. |

### Border (Bordes estructurales y divisores)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `border.structural` | `#272235` | `rgb(39, 34, 53)` | `--border-struct` | Trazo estructural de 1px entre paneles, divisores en reposo y grilla punteada AST. |
| `border.active` | `#453B5E` | `rgb(69, 59, 94)` | `--border-active` | Borde enfatizado, nodos resueltos y hover de scrollbar. |
| `border.scrollbar-hover` | `#453B5E` | `rgb(69, 59, 94)` | `--scrollbar-thumb-hover` | Indicador de scrollbar al pasar el puntero. |

### Ink (Tintas y tipografía)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `ink.primary` | `#EDEAE2` | `rgb(237, 234, 226)` | `--ink-primary` | Tinta pergamino principal de alto contraste sobre obsidiana. |
| `ink.secondary` | `#9A93A8` | `rgb(154, 147, 168)` | `--ink-secondary` | Tinta gris-violácea secundaria para etiquetas y controles inactivos. |
| `ink.muted` | `#625B73` | `rgb(98, 91, 115)` | `--ink-muted` | Tinta tenue para metadatos, contadores y estados vacíos. |
| `ink.on-primary` | `#09080C` | `rgb(9, 8, 12)` | `--accent-primary-ink` | Tinta oscura sobre el botón sólido dorado EJECUTAR. |
| `ink.gutter-active` | `#EBD08B` | `rgb(235, 208, 139)` | `--gutter-active-ink` | Número de línea activo en el gutter del editor. |
| `ink.inspector` | `#E0C482` | `rgb(224, 196, 130)` | `--inspector-ink` | Tinta dorada cálida para el registro verbose del Inspector. |
| `ink.error` | `#F495A1` | `rgb(244, 149, 161)` | `--error-ink` | Tinta rosada legible para mensajes de error en terminal. |

### Accent (Acentos metálicos y energía)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `accent.gold` | `#D8A848` | `rgb(216, 168, 72)` | `--accent-gold` | Oro bruñido principal de identidad Wyrm y acción EJECUTAR. |
| `accent.gold-hover` | `#EAB958` | `rgb(234, 185, 88)` | `--accent-gold-hover` | Oro iluminado en estado hover sobre acción principal. |
| `accent.gold-highlight` | `#F3D27A` | `rgb(243, 210, 122)` | `--ascii-gradient` | Reflejo dorado claro en el extremo inicial del degradado ASCII. |
| `accent.gold-bronze` | `#946C2B` | `rgb(148, 108, 43)` | `--ascii-gradient` | Bronce envejecido en la transición media del degradado ASCII. |
| `accent.primary` | `#D8A848` | `rgb(216, 168, 72)` | `--accent-primary` | Alias semántico del acento principal activo. |
| `accent.primary-hover` | `#EAB958` | `rgb(234, 185, 88)` | `--accent-primary-hover` | Alias semántico hover del acento principal. |
| `accent.violet` | `#A370FF` | `rgb(163, 112, 255)` | `--accent-violet` | Luz violeta arcana para foco, cursor, pestañas activas y energía. |
| `accent.violet-bright` | `#BE97FF` | `rgb(190, 151, 255)` | `--accent-violet-bright` | Violeta brillante para pulso de corrida y valores evaluados del AST. |
| `accent.violet-tail` | `#8D5CF6` | `rgb(141, 92, 246)` | `--ascii-gradient` | Violeta profundo en el cierre del degradado metálico ASCII. |
| `accent.arcane` | `#A370FF` | `rgb(163, 112, 255)` | `--accent-arcane` | Alias semántico del acento arcano. |
| `accent.pulse-start` | `#BE97FF` | `rgb(190, 151, 255)` | `--pulse-start` | Color inicial del destello táctil de ejecución. |
| `accent.pulse-end` | `#D8A848` | `rgb(216, 168, 72)` | `--pulse-end` | Color intermedio del destello táctil de ejecución. |

### Status (Estados, errores y diff del Tutor)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `status.error` | `#E05263` | `rgb(224, 82, 99)` | `--accent-crimson` | Carmesí de error, detención del Worker y traza truncada. |
| `status.error-ink` | `#F495A1` | `rgb(244, 149, 161)` | `--error-ink` | Texto legible de error sobre fondo obsidiana. |
| `status.ok-active` | `#D8A848` | `rgb(216, 168, 72)` | `--accent-gold` | Telemetría OK, fase ENTER y camino activo en el grafo AST. |
| `status.tutor-diff` | `#A370FF` | `rgb(163, 112, 255)` | `--accent-violet` | Cambios NEW/UPD en scopes, slots modificados, fase EVAL y ejecución activa. |
| `status.idle` | `#625B73` | `rgb(98, 91, 115)` | `--ink-muted` | Estado en espera / inactivo en telemetría y nodos sin visitar. |

### Syntax (Resaltado de código `.tok-*`)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `syntax.keyword` | `#F92672` | `rgb(249, 38, 114)` | `.tok-keyword` | Palabras reservadas de control y declaración (Monokai Classic). |
| `syntax.logical` | `#F92672` | `rgb(249, 38, 114)` | `.tok-logical` | Conectivos lógicos and, or, not, xor (Monokai Classic). |
| `syntax.type` | `#66D9EF` | `rgb(102, 217, 239)` | `.tok-type` | Tipos primitivos y compuestos: int32, nat8, bool, vec. |
| `syntax.builtin` | `#A6E22E` | `rgb(166, 226, 46)` | `.tok-builtin` | Primitivas nativas de E/S como read y write. |
| `syntax.fn` | `#A6E22E` | `rgb(166, 226, 46)` | `.tok-fn` | Invocaciones y nombres de función. |
| `syntax.string` | `#E6DB74` | `rgb(230, 219, 116)` | `.tok-string` | Literales de cadena entre comillas dobles. |
| `syntax.char` | `#E6DB74` | `rgb(230, 219, 116)` | `.tok-char` | Literales de carácter ASCII entre comillas simples. |
| `syntax.escape` | `#AE81FF` | `rgb(174, 129, 255)` | `.tok-escape` | Secuencias de escape dentro de cadenas y caracteres. |
| `syntax.number` | `#AE81FF` | `rgb(174, 129, 255)` | `.tok-number` | Literales numéricos enteros y flotantes. |
| `syntax.boolean` | `#AE81FF` | `rgb(174, 129, 255)` | `.tok-boolean` | Literales booleanos true y false. |
| `syntax.operator` | `#FD971F` | `rgb(253, 151, 31)` | `.tok-operator` | Operadores aritméticos, relacionales y flechas <- / ->. |
| `syntax.comment` | `#75715E` | `rgb(117, 113, 94)` | `.tok-comment` | Comentarios de línea única // (en cursiva). |
| `syntax.punct` | `#9A93A8` | `rgb(154, 147, 168)` | `.tok-punct` | Delimitadores y puntuación estructural. |
| `syntax.ident` | `#F8F8F2` | `rgb(248, 248, 242)` | `.tok-ident` | Identificadores de variables y parámetros. |
| `syntax.plain` | `#F8F8F2` | `rgb(248, 248, 242)` | `.tok-plain` | Texto plano y espacios sin clasificación especial. |

## Wyrm Silver (`silver` — Plata y Luz)

Dragón blanco: alabastro, plata cepillada, azul hielo y sintaxis de alto contraste.

### Surface (Superficies, fondos y escamas)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `surface.base` | `#F2F4F8` | `rgb(242, 244, 248)` | `--bg-base` | Fondo raíz alabastro frío del viewport y la consola. |
| `surface.inset` | `#EDF0F6` | `rgb(237, 240, 246)` | `--bg-inset` | Superficie hundida del lienzo SVG del AST. |
| `surface.panel` | `#E8ECF4` | `rgb(232, 236, 244)` | `--bg-panel` | Fondo estructural de paneles, topbar, footer y bloques de salida. |
| `surface.gutter` | `#E1E6F0` | `rgb(225, 230, 240)` | `--gutter-bg` | Fondo de la franja de numeración de líneas del editor. |
| `surface.elevated` | `#DDE3EE` | `rgb(221, 227, 238)` | `--bg-surface` | Superficie elevada plata cepillada para cabeceras y tabs. |
| `surface.scale-mid` | `#C5CFE0` | `rgb(197, 207, 224)` | `--wyrm-scale-mid` | Escama intermedia plateada del isologo pixel art SVG. |
| `surface.scrollbar-thumb` | `#B2BDD2` | `rgb(178, 189, 210)` | `--scrollbar-thumb` | Indicador deslizable en reposo de las barras de desplazamiento. |
| `surface.scale-dark` | `#4F5D75` | `rgb(79, 93, 117)` | `--wyrm-scale-dark` | Escama sombreada acero del isologo pixel art SVG. |
| `surface.scale-light` | `#F8FAFD` | `rgb(248, 250, 253)` | `--wyrm-scale-light` | Escama blanca hielo del isologo pixel art SVG. |

### Border (Bordes estructurales y divisores)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `border.structural` | `#BCC5D8` | `rgb(188, 197, 216)` | `--border-struct` | Trazo estructural de 1px entre paneles, divisores en reposo y grilla AST. |
| `border.active` | `#8F9DB8` | `rgb(143, 157, 184)` | `--border-active` | Borde enfatizado, nodos resueltos y hover de scrollbar. |
| `border.scrollbar-hover` | `#8F9DB8` | `rgb(143, 157, 184)` | `--scrollbar-thumb-hover` | Indicador de scrollbar al pasar el puntero. |

### Ink (Tintas y tipografía)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `ink.primary` | `#141824` | `rgb(20, 24, 36)` | `--ink-primary` | Tinta azul-obsidiana principal de alto contraste sobre alabastro. |
| `ink.secondary` | `#4A556E` | `rgb(74, 85, 110)` | `--ink-secondary` | Tinta pizarra secundaria para etiquetas y controles inactivos. |
| `ink.muted` | `#748099` | `rgb(116, 128, 153)` | `--ink-muted` | Tinta acero tenue para metadatos, contadores y estados vacíos. |
| `ink.on-primary` | `#F8FAFD` | `rgb(248, 250, 253)` | `--accent-primary-ink` | Tinta blanca hielo sobre el botón sólido azul EJECUTAR. |
| `ink.gutter-active` | `#141824` | `rgb(20, 24, 36)` | `--gutter-active-ink` | Número de línea activo en el gutter del editor. |
| `ink.inspector` | `#34435C` | `rgb(52, 67, 92)` | `--inspector-ink` | Tinta acero profundo para el registro verbose del Inspector. |
| `ink.error` | `#A81C2C` | `rgb(168, 28, 44)` | `--error-ink` | Tinta carmesí oscura de alto contraste para errores en terminal. |

### Accent (Acentos metálicos y energía)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `accent.steel` | `#5C6B82` | `rgb(92, 107, 130)` | `--accent-gold` | Acero bruñido secundario que reemplaza al oro en la paleta clara. |
| `accent.steel-hover` | `#465469` | `rgb(70, 84, 105)` | `--accent-gold-hover` | Acero oscuro en estado hover. |
| `accent.steel-deep` | `#273248` | `rgb(39, 50, 72)` | `--ascii-gradient` | Acero profundo en el inicio del degradado metálico ASCII. |
| `accent.steel-mid` | `#4B5D7A` | `rgb(75, 93, 122)` | `--ascii-gradient` | Acero intermedio en el punto medio del degradado ASCII. |
| `accent.ice-blue` | `#1E73E8` | `rgb(30, 115, 232)` | `--accent-violet` | Azul hielo eléctrico para acción EJECUTAR, foco, cursor y energía. |
| `accent.ice-blue-bright` | `#3B88F5` | `rgb(59, 136, 245)` | `--accent-violet-bright` | Azul hielo brillante para pulso de corrida y reflejos. |
| `accent.primary` | `#1E73E8` | `rgb(30, 115, 232)` | `--accent-primary` | Alias semántico del acento principal en Plata y Luz. |
| `accent.primary-hover` | `#155CC0` | `rgb(21, 92, 192)` | `--accent-primary-hover` | Azul hielo profundo en hover del botón primario EJECUTAR. |
| `accent.arcane` | `#5C6B82` | `rgb(92, 107, 130)` | `--accent-arcane` | Alias semántico del acento metálico complementario. |
| `accent.pulse-start` | `#3B88F5` | `rgb(59, 136, 245)` | `--pulse-start` | Color inicial del destello táctil de ejecución. |
| `accent.pulse-end` | `#5C6B82` | `rgb(92, 107, 130)` | `--pulse-end` | Color intermedio del destello táctil de ejecución. |

### Status (Estados, errores y diff del Tutor)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `status.error` | `#C62839` | `rgb(198, 40, 57)` | `--accent-crimson` | Carmesí de error, detención del Worker y traza truncada. |
| `status.error-ink` | `#A81C2C` | `rgb(168, 28, 44)` | `--error-ink` | Texto carmesí legible sobre superficie clara. |
| `status.ok-active` | `#5C6B82` | `rgb(92, 107, 130)` | `--accent-gold` | Telemetría OK, fase ENTER y camino activo en el grafo AST. |
| `status.tutor-diff` | `#1E73E8` | `rgb(30, 115, 232)` | `--accent-violet` | Cambios NEW/UPD en scopes, slots modificados, fase EVAL y ejecución activa. |
| `status.idle` | `#748099` | `rgb(116, 128, 153)` | `--ink-muted` | Estado en espera / inactivo en telemetría y nodos sin visitar. |

### Syntax (Resaltado de código `.tok-*`)

| Token | HEX | RGB | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `syntax.keyword` | `#C41456` | `rgb(196, 20, 86)` | `[data-theme="silver"] .tok-keyword` | Palabras reservadas de control y declaración (alto contraste). |
| `syntax.logical` | `#C41456` | `rgb(196, 20, 86)` | `[data-theme="silver"] .tok-logical` | Conectivos lógicos and, or, not, xor (alto contraste). |
| `syntax.type` | `#0B6B8A` | `rgb(11, 107, 138)` | `[data-theme="silver"] .tok-type` | Tipos primitivos y compuestos: int32, nat8, bool, vec. |
| `syntax.builtin` | `#2C7A1F` | `rgb(44, 122, 31)` | `[data-theme="silver"] .tok-builtin` | Primitivas nativas de E/S como read y write. |
| `syntax.fn` | `#2C7A1F` | `rgb(44, 122, 31)` | `[data-theme="silver"] .tok-fn` | Invocaciones y nombres de función. |
| `syntax.string` | `#8A6100` | `rgb(138, 97, 0)` | `[data-theme="silver"] .tok-string` | Literales de cadena entre comillas dobles. |
| `syntax.char` | `#8A6100` | `rgb(138, 97, 0)` | `[data-theme="silver"] .tok-char` | Literales de carácter ASCII entre comillas simples. |
| `syntax.escape` | `#6B32D6` | `rgb(107, 50, 214)` | `[data-theme="silver"] .tok-escape` | Secuencias de escape dentro de cadenas y caracteres. |
| `syntax.number` | `#6B32D6` | `rgb(107, 50, 214)` | `[data-theme="silver"] .tok-number` | Literales numéricos enteros y flotantes. |
| `syntax.boolean` | `#6B32D6` | `rgb(107, 50, 214)` | `[data-theme="silver"] .tok-boolean` | Literales booleanos true y false. |
| `syntax.operator` | `#B84B00` | `rgb(184, 75, 0)` | `[data-theme="silver"] .tok-operator` | Operadores aritméticos, relacionales y flechas <- / ->. |
| `syntax.comment` | `#68738A` | `rgb(104, 115, 138)` | `[data-theme="silver"] .tok-comment` | Comentarios de línea única // (en cursiva). |
| `syntax.punct` | `#4A556E` | `rgb(74, 85, 110)` | `[data-theme="silver"] .tok-punct` | Delimitadores y puntuación estructural. |
| `syntax.ident` | `#141824` | `rgb(20, 24, 36)` | `[data-theme="silver"] .tok-ident` | Identificadores de variables y parámetros. |
| `syntax.plain` | `#141824` | `rgb(20, 24, 36)` | `[data-theme="silver"] .tok-plain` | Texto plano y espacios sin clasificación especial. |

## Colores con transparencia (`rgba`)

### Wyrm Obsidian — Transparencias

| Token | Valor `rgba` | HSL / Alpha | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `surface.backdrop` | `rgba(5, 4, 8, 0.82)` | `hsla(255, 33%, 2%, 0.82)` | `--backdrop-bg` | Velo oscuro semitransparente detrás del modal Códice de Wyrm. |
| `surface.shortcut-bg` | `rgba(0, 0, 0, 0.14)` | `hsla(0, 0%, 0%, 0.14)` | `.shortcut-hint` | Fondo translúcido de la etiqueta de atajo Ctrl+Enter sobre botón primario. |
| `border.gold` | `rgba(216, 168, 72, 0.45)` | `hsla(40, 65%, 56%, 0.45)` | `--accent-gold-border` | Borde translúcido dorado para badges ENTER y estados destacados. |
| `border.violet` | `rgba(163, 112, 255, 0.45)` | `hsla(261, 100%, 72%, 0.45)` | `--accent-violet-border` | Borde translúcido violeta para badges, enfoque, grips y cajón stdin abierto. |
| `border.crimson` | `rgba(224, 82, 99, 0.48)` | `hsla(353, 70%, 60%, 0.48)` | `--accent-crimson-border` | Borde translúcido carmesí para bloques de error y badge truncado. |
| `border.shortcut` | `rgba(0, 0, 0, 0.25)` | `hsla(0, 0%, 0%, 0.25)` | `.shortcut-hint` | Borde sutil de la etiqueta de atajo sobre el botón dorado. |
| `accent.gold-dim` | `rgba(216, 168, 72, 0.12)` | `hsla(40, 65%, 56%, 0.12)` | `--accent-gold-dim` | Fondo tenue dorado para línea Tutor, fila activa y badge ENTER. |
| `accent.violet-dim` | `rgba(163, 112, 255, 0.12)` | `hsla(261, 100%, 72%, 0.12)` | `--accent-violet-dim` | Relleno tenue violeta para badges, píldoras activas y banners. |
| `accent.violet-hover` | `rgba(163, 112, 255, 0.08)` | `hsla(261, 100%, 72%, 0.08)` | `--accent-violet-hover` | Realce sutil violeta al pasar el puntero sobre controles planos. |
| `accent.glow` | `rgba(163, 112, 255, 0.22)` | `hsla(261, 100%, 72%, 0.22)` | `--accent-glow` | Resplandor ambiental violeta en nodos activos, pulsos y divisores. |
| `accent.return-glow` | `rgba(163, 112, 255, 0.35)` | `hsla(261, 100%, 72%, 0.35)` | `--return-glow` | Halo lateral del bloque de valor de retorno del programa. |
| `accent.selection` | `rgba(163, 112, 255, 0.25)` | `hsla(261, 100%, 72%, 0.25)` | `--selection-bg` | Selección de texto dentro del editor de código. |
| `accent.gutter-active-bg` | `rgba(163, 112, 255, 0.09)` | `hsla(261, 100%, 72%, 0.09)` | `--gutter-active-bg` | Tinte violeta de la línea activa en el gutter. |
| `accent.ascii-drop-shadow` | `rgba(163, 112, 255, 0.16)` | `hsla(261, 100%, 72%, 0.16)` | `--ascii-drop-shadow` | Sombra proyectada violeta del arte ASCII del wyrm. |
| `accent.ascii-eye-halo` | `rgba(163, 112, 255, 0.6)` | `hsla(261, 100%, 72%, 0.6)` | `--ascii-eye-shadow` | Halo luminoso exterior del ojo ◈ en el arte ASCII. |
| `accent.empty-glow-gold` | `rgba(216, 168, 72, 0.03)` | `hsla(40, 65%, 56%, 0.03)` | `--empty-radial-glow` | Aureola radial dorada muy tenue del estado vacío de consola. |
| `status.error-dim` | `rgba(224, 82, 99, 0.14)` | `hsla(353, 70%, 60%, 0.14)` | `--accent-crimson-dim` | Fondo translúcido para badges de error y límite alcanzado. |
| `status.error-border` | `rgba(224, 82, 99, 0.48)` | `hsla(353, 70%, 60%, 0.48)` | `--accent-crimson-border` | Borde de bloques de error y avisos de truncado. |
| `status.ok-active-dim` | `rgba(216, 168, 72, 0.12)` | `hsla(40, 65%, 56%, 0.12)` | `--accent-gold-dim` | Fondo de fila activa de bytecode y badge ENTER. |
| `status.tutor-diff-bg` | `rgba(163, 112, 255, 0.12)` | `hsla(261, 100%, 72%, 0.12)` | `--accent-violet-dim` | Resaltado de filas modificadas en scopes, celdas heap y banner activo. |
| `status.tutor-diff-border` | `rgba(163, 112, 255, 0.45)` | `hsla(261, 100%, 72%, 0.45)` | `--accent-violet-border` | Borde de etiquetas NEW/UPD y bloques modificados en Tutor. |

### Wyrm Silver — Transparencias

| Token | Valor `rgba` | HSL / Alpha | Variable / Selector CSS | Uso recomendado |
| --- | --- | --- | --- | --- |
| `surface.backdrop` | `rgba(20, 24, 36, 0.56)` | `hsla(225, 29%, 11%, 0.56)` | `--backdrop-bg` | Velo oscuro semitransparente detrás del modal Códice de Wyrm. |
| `surface.shortcut-bg` | `rgba(255, 255, 255, 0.16)` | `hsla(0, 0%, 100%, 0.16)` | `[data-theme="silver"] .shortcut-hint` | Fondo translúcido claro de la etiqueta Ctrl+Enter sobre azul hielo. |
| `border.steel` | `rgba(92, 107, 130, 0.48)` | `hsla(216, 17%, 44%, 0.48)` | `--accent-gold-border` | Borde translúcido acero para badges ENTER y estados destacados. |
| `border.ice-blue` | `rgba(30, 115, 232, 0.45)` | `hsla(215, 81%, 51%, 0.45)` | `--accent-violet-border` | Borde translúcido azul hielo para badges, enfoque, grips y cajón stdin. |
| `border.crimson` | `rgba(198, 40, 57, 0.44)` | `hsla(354, 66%, 47%, 0.44)` | `--accent-crimson-border` | Borde translúcido carmesí para bloques de error y badge truncado. |
| `border.shortcut` | `rgba(255, 255, 255, 0.35)` | `hsla(0, 0%, 100%, 0.35)` | `[data-theme="silver"] .shortcut-hint` | Borde claro de la etiqueta de atajo sobre el botón azul hielo. |
| `accent.steel-dim` | `rgba(92, 107, 130, 0.12)` | `hsla(216, 17%, 44%, 0.12)` | `--accent-gold-dim` | Fondo tenue acero para línea Tutor, fila activa y badge ENTER. |
| `accent.ice-blue-dim` | `rgba(30, 115, 232, 0.1)` | `hsla(215, 81%, 51%, 0.1)` | `--accent-violet-dim` | Relleno tenue azul hielo para badges, píldoras activas y banners. |
| `accent.ice-blue-hover` | `rgba(30, 115, 232, 0.07)` | `hsla(215, 81%, 51%, 0.07)` | `--accent-violet-hover` | Realce sutil azul hielo al pasar el puntero sobre controles planos. |
| `accent.glow` | `rgba(30, 115, 232, 0.16)` | `hsla(215, 81%, 51%, 0.16)` | `--accent-glow` | Resplandor azul hielo en nodos activos, pulsos y divisores. |
| `accent.return-glow` | `rgba(30, 115, 232, 0.28)` | `hsla(215, 81%, 51%, 0.28)` | `--return-glow` | Halo lateral del bloque de valor de retorno del programa. |
| `accent.selection` | `rgba(30, 115, 232, 0.2)` | `hsla(215, 81%, 51%, 0.2)` | `--selection-bg` | Selección de texto dentro del editor de código. |
| `accent.gutter-active-bg` | `rgba(30, 115, 232, 0.1)` | `hsla(215, 81%, 51%, 0.1)` | `--gutter-active-bg` | Tinte azul hielo de la línea activa en el gutter. |
| `accent.ascii-drop-shadow` | `rgba(30, 115, 232, 0.14)` | `hsla(215, 81%, 51%, 0.14)` | `--ascii-drop-shadow` | Sombra proyectada azul hielo del arte ASCII. |
| `accent.ascii-eye-halo` | `rgba(30, 115, 232, 0.55)` | `hsla(215, 81%, 51%, 0.55)` | `--ascii-eye-shadow` | Halo luminoso exterior del ojo ◈ en el arte ASCII. |
| `accent.empty-glow-ice` | `rgba(30, 115, 232, 0.08)` | `hsla(215, 81%, 51%, 0.08)` | `--empty-radial-glow` | Centro radial azul hielo del estado vacío de consola. |
| `accent.empty-glow-steel` | `rgba(92, 107, 130, 0.03)` | `hsla(216, 17%, 44%, 0.03)` | `--empty-radial-glow` | Aureola radial acero muy tenue del estado vacío de consola. |
| `status.error-dim` | `rgba(198, 40, 57, 0.1)` | `hsla(354, 66%, 47%, 0.1)` | `--accent-crimson-dim` | Fondo translúcido para badges de error y límite alcanzado. |
| `status.error-border` | `rgba(198, 40, 57, 0.44)` | `hsla(354, 66%, 47%, 0.44)` | `--accent-crimson-border` | Borde de bloques de error y avisos de truncado. |
| `status.ok-active-dim` | `rgba(92, 107, 130, 0.12)` | `hsla(216, 17%, 44%, 0.12)` | `--accent-gold-dim` | Fondo de fila activa de bytecode y badge ENTER. |
| `status.tutor-diff-bg` | `rgba(30, 115, 232, 0.1)` | `hsla(215, 81%, 51%, 0.1)` | `--accent-violet-dim` | Resaltado de filas modificadas en scopes, celdas heap y banner activo. |
| `status.tutor-diff-border` | `rgba(30, 115, 232, 0.45)` | `hsla(215, 81%, 51%, 0.45)` | `--accent-violet-border` | Borde de etiquetas NEW/UPD y bloques modificados en Tutor. |

## Degradados metálicos y estructurales

### Wyrm Obsidian — Degradados

| Token | Variable / Selector CSS | Definición exacta | Uso recomendado |
| --- | --- | --- | --- |
| `gradients.topbar-hairline` | `--topbar-top-gradient` | `linear-gradient(90deg, #a370ff 0%, #d8a848 38%, transparent 85%)` | Línea superior de 1px en la barra principal. |
| `gradients.tab-active` | `--tab-active-gradient` | `linear-gradient(90deg, #a370ff, #d8a848)` | Subrayado de 2px en pestañas y submodos activos. |
| `gradients.resizer-vertical` | `.split-resizer::after` | `linear-gradient(180deg, #a370ff, #d8a848)` | Brillo de 2px del divisor horizontal principal entre Editor y Salida. |
| `gradients.resizer-horizontal` | `.ast-v-resizer::after` | `linear-gradient(90deg, #a370ff, #d8a848)` | Brillo de 2px del divisor vertical entre grafo AST y SCOPES. |
| `gradients.running-scanline` | `.running-banner::after` | `linear-gradient(90deg, transparent, #a370ff, #d8a848, transparent)` | Barrido animado inferior en el banner de ejecución del Web Worker. |
| `gradients.ascii-metallic` | `--ascii-gradient` | `linear-gradient(150deg, #f3d27a 0%, #d8a848 36%, #946c2b 66%, #8d5cf6 100%)` | Degradado metálico oro -> bronce -> violeta del arte ASCII del wyrm. |
| `gradients.empty-radial-glow` | `--empty-radial-glow` | `radial-gradient(circle at 50% 45%, rgba(163, 112, 255, 0.08) 0%, rgba(216, 168, 72, 0.03) 35%, transparent 68%)` | Reflejo radial ambiental detrás del estado vacío de la consola. |
| `gradients.ast-dot-grid` | `.ast-canvas-stage` | `radial-gradient(circle, #272235 1px, transparent 1px)` | Patrón de retícula punteada de 20px en el lienzo interactivo del AST. |

### Wyrm Silver — Degradados

| Token | Variable / Selector CSS | Definición exacta | Uso recomendado |
| --- | --- | --- | --- |
| `gradients.topbar-hairline` | `--topbar-top-gradient` | `linear-gradient(90deg, #1e73e8 0%, #5c6b82 38%, transparent 85%)` | Línea superior de 1px en la barra principal. |
| `gradients.tab-active` | `--tab-active-gradient` | `linear-gradient(90deg, #1e73e8, #5c6b82)` | Subrayado de 2px en pestañas y submodos activos. |
| `gradients.resizer-vertical` | `.split-resizer::after` | `linear-gradient(180deg, #1e73e8, #5c6b82)` | Brillo de 2px del divisor horizontal principal entre Editor y Salida. |
| `gradients.resizer-horizontal` | `.ast-v-resizer::after` | `linear-gradient(90deg, #1e73e8, #5c6b82)` | Brillo de 2px del divisor vertical entre grafo AST y SCOPES. |
| `gradients.running-scanline` | `.running-banner::after` | `linear-gradient(90deg, transparent, #1e73e8, #5c6b82, transparent)` | Barrido animado inferior en el banner de ejecución del Web Worker. |
| `gradients.ascii-metallic` | `--ascii-gradient` | `linear-gradient(150deg, #273248 0%, #4b5d7a 48%, #1e73e8 100%)` | Degradado metálico acero profundo -> plata azulada -> azul hielo del arte ASCII. |
| `gradients.empty-radial-glow` | `--empty-radial-glow` | `radial-gradient(circle at 50% 45%, rgba(30, 115, 232, 0.08) 0%, rgba(92, 107, 130, 0.03) 35%, transparent 68%)` | Reflejo radial ambiental detrás del estado vacío de la consola. |
| `gradients.ast-dot-grid` | `.ast-canvas-stage` | `radial-gradient(circle, #bcc5d8 1px, transparent 1px)` | Patrón de retícula punteada de 20px en el lienzo interactivo del AST. |
