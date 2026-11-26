//! Persistencia en `window.localStorage` y definición de paletas visuales del Playground.

const PALETTE_STORAGE_KEY: &str = "wyrm-theme";
const SPLIT_STORAGE_KEY: &str = "wyrm-split-pct";
const AST_SPLIT_STORAGE_KEY: &str = "wyrm-ast-split-pct";
const WORD_WRAP_STORAGE_KEY: &str = "wyrm-word-wrap";

pub const DEFAULT_EDITOR_WIDTH_PCT: f64 = 44.0;
pub const MIN_EDITOR_WIDTH_PCT: f64 = 20.0;
pub const MAX_EDITOR_WIDTH_PCT: f64 = 80.0;

pub const DEFAULT_AST_GRAPH_PCT: f64 = 62.0;
pub const MIN_AST_GRAPH_PCT: f64 = 35.0;
pub const MAX_AST_GRAPH_PCT: f64 = 85.0;

/// Paleta visual conmutable del Playground.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Palette {
    Obsidian,
    Silver,
}

impl Palette {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Obsidian => "obsidian",
            Self::Silver => "silver",
        }
    }

    pub fn from_storage(raw: &str) -> Self {
        match raw {
            "silver" => Self::Silver,
            _ => Self::Obsidian,
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Obsidian => Self::Silver,
            Self::Silver => Self::Obsidian,
        }
    }

    pub fn glyph(self) -> &'static str {
        match self {
            Self::Obsidian => "◈",
            Self::Silver => "◇",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Obsidian => "OBSIDIANA",
            Self::Silver => "PLATA",
        }
    }

    pub fn toggle_title(self) -> &'static str {
        match self {
            Self::Obsidian => "Cambiar a paleta Plata (Dragón Blanco)",
            Self::Silver => "Cambiar a paleta Obsidiana",
        }
    }
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|window| window.local_storage().ok().flatten())
}

fn read_item(key: &str) -> Option<String> {
    local_storage().and_then(|storage| storage.get_item(key).ok().flatten())
}

fn write_item(key: &str, value: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(key, value);
    }
}

/// Acota un porcentaje dentro del rango `[min, max]`, cayendo a `default` si no es finito.
pub fn clamp_pct(pct: f64, min: f64, max: f64, default: f64) -> f64 {
    if pct.is_finite() {
        pct.clamp(min, max)
    } else {
        default
    }
}

/// Recupera la paleta persistida en `window.localStorage`.
pub fn load_palette() -> Palette {
    read_item(PALETTE_STORAGE_KEY)
        .map(|raw| Palette::from_storage(&raw))
        .unwrap_or(Palette::Obsidian)
}

/// Persiste la paleta activa en `window.localStorage`.
pub fn save_palette(palette: Palette) {
    write_item(PALETTE_STORAGE_KEY, palette.as_str());
}

/// Sincroniza `data-theme` en `<html>` para cubrir fondo raíz y barras de desplazamiento.
pub fn apply_document_palette(palette: Palette) {
    if let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|doc| doc.document_element())
    {
        let _ = root.set_attribute("data-theme", palette.as_str());
    }
}

/// Recupera el porcentaje de división principal persistido en `window.localStorage`.
pub fn load_split_pct() -> f64 {
    read_item(SPLIT_STORAGE_KEY)
        .and_then(|raw| raw.parse::<f64>().ok())
        .map(|pct| {
            clamp_pct(
                pct,
                MIN_EDITOR_WIDTH_PCT,
                MAX_EDITOR_WIDTH_PCT,
                DEFAULT_EDITOR_WIDTH_PCT,
            )
        })
        .unwrap_or(DEFAULT_EDITOR_WIDTH_PCT)
}

/// Persiste el porcentaje de división principal en `window.localStorage`.
pub fn save_split_pct(pct: f64) {
    let clamped = clamp_pct(
        pct,
        MIN_EDITOR_WIDTH_PCT,
        MAX_EDITOR_WIDTH_PCT,
        DEFAULT_EDITOR_WIDTH_PCT,
    );
    write_item(SPLIT_STORAGE_KEY, &format!("{clamped:.2}"));
}

/// Recupera la altura porcentual del lienzo AST persistida en `window.localStorage`.
pub fn load_ast_split_pct() -> f64 {
    read_item(AST_SPLIT_STORAGE_KEY)
        .and_then(|raw| raw.parse::<f64>().ok())
        .map(|pct| {
            clamp_pct(
                pct,
                MIN_AST_GRAPH_PCT,
                MAX_AST_GRAPH_PCT,
                DEFAULT_AST_GRAPH_PCT,
            )
        })
        .unwrap_or(DEFAULT_AST_GRAPH_PCT)
}

/// Persiste la altura porcentual del lienzo AST en `window.localStorage`.
pub fn save_ast_split_pct(pct: f64) {
    let clamped = clamp_pct(
        pct,
        MIN_AST_GRAPH_PCT,
        MAX_AST_GRAPH_PCT,
        DEFAULT_AST_GRAPH_PCT,
    );
    write_item(AST_SPLIT_STORAGE_KEY, &format!("{clamped:.2}"));
}

/// Recupera la preferencia de ajuste de línea (soft-wrap) en el editor (`true` por defecto).
pub fn load_word_wrap() -> bool {
    !matches!(
        read_item(WORD_WRAP_STORAGE_KEY).as_deref(),
        Some("false" | "0")
    )
}

/// Persiste la preferencia de ajuste de línea (soft-wrap) en `window.localStorage`.
pub fn save_word_wrap(enabled: bool) {
    write_item(WORD_WRAP_STORAGE_KEY, if enabled { "true" } else { "false" });
}
