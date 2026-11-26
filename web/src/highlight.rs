//! Lexer de resaltado de sintaxis puro en Rust para el lenguaje Wyrm.
//!
//! Regla de integridad: al concatenar todas las partes devueltas por [`tokenize`]
//! se reconstruye la entrada carácter por carácter sin omisiones.

/// Categoría léxica de cada token resaltado en el editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Comment,
    String,
    Char,
    Escape,
    Type,
    Keyword,
    LogicalOp,
    Boolean,
    Number,
    Builtin,
    FunctionCall,
    Operator,
    Punctuation,
    Identifier,
    Plain,
}

impl TokenKind {
    /// Clase CSS asociada a cada tipo de token.
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Comment => "tok-comment",
            Self::String => "tok-string",
            Self::Char => "tok-char",
            Self::Escape => "tok-escape",
            Self::Type => "tok-type",
            Self::Keyword => "tok-keyword",
            Self::LogicalOp => "tok-logical",
            Self::Boolean => "tok-boolean",
            Self::Number => "tok-number",
            Self::Builtin => "tok-builtin",
            Self::FunctionCall => "tok-fn",
            Self::Operator => "tok-operator",
            Self::Punctuation => "tok-punct",
            Self::Identifier => "tok-ident",
            Self::Plain => "tok-plain",
        }
    }
}

fn push_token(tokens: &mut Vec<(TokenKind, String)>, kind: TokenKind, text: &str) {
    if text.is_empty() {
        return;
    }
    if kind == TokenKind::Plain {
        if let Some((last_kind, last_text)) = tokens.last_mut() {
            if *last_kind == TokenKind::Plain {
                last_text.push_str(text);
                return;
            }
        }
    }
    tokens.push((kind, text.to_string()));
}

fn is_escape_char(byte: u8) -> bool {
    matches!(byte, b'n' | b't' | b'r' | b'0' | b'\\' | b'"' | b'\'')
}

fn tokenize_quoted(
    source: &str,
    start: usize,
    quote: u8,
    kind: TokenKind,
    tokens: &mut Vec<(TokenKind, String)>,
) -> usize {
    let bytes = source.as_bytes();
    let len = bytes.len();
    let mut i = start + 1;
    let mut seg_start = start;

    while i < len {
        let b = bytes[i];
        if b == quote {
            i += 1;
            push_token(tokens, kind, &source[seg_start..i]);
            return i;
        }
        if b == b'\n' {
            push_token(tokens, kind, &source[seg_start..i]);
            return i;
        }
        if b == b'\\' && i + 1 < len && is_escape_char(bytes[i + 1]) {
            push_token(tokens, kind, &source[seg_start..i]);
            push_token(tokens, TokenKind::Escape, &source[i..i + 2]);
            i += 2;
            seg_start = i;
            continue;
        }
        let ch_len = source[i..].chars().next().map_or(1, char::len_utf8);
        i += ch_len;
    }

    push_token(tokens, kind, &source[seg_start..i]);
    i
}

fn is_followed_by_call_paren(bytes: &[u8], mut index: usize) -> bool {
    while index < bytes.len() && matches!(bytes[index], b' ' | b'\t') {
        index += 1;
    }
    index < bytes.len() && bytes[index] == b'('
}

fn classify_word(word: &str, is_call: bool) -> TokenKind {
    match word {
        "nat8" | "nat16" | "nat32" | "nat64" | "int8" | "int16" | "int32" | "int64" | "float32"
        | "float64" | "bool" | "vec" => TokenKind::Type,
        "if" | "else" | "while" | "for" | "loop" | "break" | "match" => TokenKind::Keyword,
        "and" | "or" | "not" | "xor" => TokenKind::LogicalOp,
        "true" | "false" => TokenKind::Boolean,
        "read" | "write" => TokenKind::Builtin,
        _ if is_call => TokenKind::FunctionCall,
        _ => TokenKind::Identifier,
    }
}

/// Divide el código fuente Wyrm en tokens coloreables preservando cada byte.
pub fn tokenize(source: &str) -> Vec<(TokenKind, String)> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;

    while i < len {
        let b = bytes[i];

        // Comentarios de línea: solo `//` hasta `\n`
        if b == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
            let start = i;
            i += 2;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            push_token(&mut tokens, TokenKind::Comment, &source[start..i]);
            continue;
        }

        // Cadenas y caracteres entre comillas
        if b == b'"' {
            i = tokenize_quoted(source, i, b'"', TokenKind::String, &mut tokens);
            continue;
        }
        if b == b'\'' {
            i = tokenize_quoted(source, i, b'\'', TokenKind::Char, &mut tokens);
            continue;
        }

        // Espacios y saltos de línea
        if b.is_ascii_whitespace() {
            let start = i;
            i += 1;
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            push_token(&mut tokens, TokenKind::Plain, &source[start..i]);
            continue;
        }

        // Números enteros y flotantes (\d+(\.\d+)?)
        if b.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < len && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i + 1 < len && bytes[i] == b'.' && bytes[i + 1].is_ascii_digit() {
                i += 1;
                while i < len && bytes[i].is_ascii_digit() {
                    i += 1;
                }
            }
            push_token(&mut tokens, TokenKind::Number, &source[start..i]);
            continue;
        }

        // Identificadores, palabras clave, tipos, operadores lógicos, booleanos y llamadas
        if b.is_ascii_alphabetic() || b == b'_' {
            let start = i;
            i += 1;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let word = &source[start..i];
            let is_call = is_followed_by_call_paren(bytes, i);
            let kind = classify_word(word, is_call);
            push_token(&mut tokens, kind, word);
            continue;
        }

        // Operadores de dos caracteres
        if i + 1 < len {
            let pair = &bytes[i..i + 2];
            if matches!(pair, b"->" | b"<-" | b"~=" | b"==" | b"<=" | b">=" | b":=") {
                push_token(&mut tokens, TokenKind::Operator, &source[i..i + 2]);
                i += 2;
                continue;
            }
        }

        // Operadores de un carácter
        if matches!(
            b,
            b'+' | b'-' | b'*' | b'/' | b'%' | b'^' | b'=' | b'<' | b'>'
        ) {
            push_token(&mut tokens, TokenKind::Operator, &source[i..i + 1]);
            i += 1;
            continue;
        }

        // Puntuación
        if matches!(b, b'{' | b'}' | b'(' | b')' | b'[' | b']' | b';' | b',') {
            push_token(&mut tokens, TokenKind::Punctuation, &source[i..i + 1]);
            i += 1;
            continue;
        }

        // Cualquier otro carácter UTF-8
        let ch_len = source[i..].chars().next().map_or(1, char::len_utf8);
        push_token(&mut tokens, TokenKind::Plain, &source[i..i + ch_len]);
        i += ch_len;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::EXAMPLES;

    fn reconstruct(tokens: &[(TokenKind, String)]) -> String {
        tokens.iter().map(|(_, text)| text.as_str()).collect()
    }

    #[test]
    fn preserves_exact_source_across_all_examples() {
        for example in EXAMPLES {
            let tokens = tokenize(example.code);
            assert_eq!(
                reconstruct(&tokens),
                example.code,
                "falló la reconstrucción fiel en el ejemplo '{}'",
                example.id
            );
        }
    }

    #[test]
    fn preserves_exact_source_on_unclosed_literals_and_comments() {
        let cases = [
            "",
            "\n\n",
            "vec<nat8> s = \"sin cerrar",
            "vec<nat8> s = \"fila 1\\n\nint32 x = 1",
            "nat8 c = 'a",
            "nat8 c = '\\",
            "// comentario sin salto final con acentos áéíóú",
            "# esto no es comentario en Wyrm",
        ];

        for input in cases {
            let tokens = tokenize(input);
            assert_eq!(reconstruct(&tokens), input);
        }
    }

    #[test]
    fn classifies_comments_and_ignores_hash_as_comment() {
        let tokens = tokenize("// hola\n# texto");
        assert_eq!(tokens[0], (TokenKind::Comment, "// hola".to_string()));
        assert_eq!(tokens[1], (TokenKind::Plain, "\n# ".to_string()));
        assert_eq!(tokens[2], (TokenKind::Identifier, "texto".to_string()));
    }

    #[test]
    fn classifies_strings_chars_and_escapes() {
        let tokens = tokenize("\"hola\\n\\t\\\"wyrm\\\\\" '\\'' '\\0'");
        assert!(tokens.contains(&(TokenKind::String, "\"hola".to_string())));
        assert!(tokens.contains(&(TokenKind::Escape, "\\n".to_string())));
        assert!(tokens.contains(&(TokenKind::Escape, "\\t".to_string())));
        assert!(tokens.contains(&(TokenKind::Escape, "\\\"".to_string())));
        assert!(tokens.contains(&(TokenKind::Escape, "\\\\".to_string())));
        assert!(tokens.contains(&(TokenKind::Char, "'".to_string())));
        assert!(tokens.contains(&(TokenKind::Escape, "\\'".to_string())));
        assert!(tokens.contains(&(TokenKind::Escape, "\\0".to_string())));
    }

    #[test]
    fn classifies_types_vectors_operators_and_calls() {
        let src = "vec<nat8> s = \"ok\"\n(int32) -> int32 f = (x) -> { if (x ~= 0 and true) { write(1, s); sumar(x) } }\nfor (n <- nums) { }";
        let tokens = tokenize(src);
        assert_eq!(reconstruct(&tokens), src);

        assert!(tokens.contains(&(TokenKind::Type, "vec".to_string())));
        assert!(tokens.contains(&(TokenKind::Type, "nat8".to_string())));
        assert!(tokens.contains(&(TokenKind::Type, "int32".to_string())));
        assert!(tokens.contains(&(TokenKind::Operator, "->".to_string())));
        assert!(tokens.contains(&(TokenKind::Operator, "<-".to_string())));
        assert!(tokens.contains(&(TokenKind::Operator, "~=".to_string())));
        assert!(tokens.contains(&(TokenKind::Keyword, "if".to_string())));
        assert!(tokens.contains(&(TokenKind::Keyword, "for".to_string())));
        assert!(tokens.contains(&(TokenKind::LogicalOp, "and".to_string())));
        assert!(tokens.contains(&(TokenKind::Boolean, "true".to_string())));
        assert!(tokens.contains(&(TokenKind::Builtin, "write".to_string())));
        assert!(tokens.contains(&(TokenKind::FunctionCall, "sumar".to_string())));
        assert!(!tokens.contains(&(TokenKind::FunctionCall, "if".to_string())));
        assert!(!tokens.contains(&(TokenKind::FunctionCall, "write".to_string())));
    }
}
