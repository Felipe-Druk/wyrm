use std::ffi::CString;
use std::os::raw::{c_char, c_int};

const VERSION: &str = "0.0.2";
const EXIT_COMMAND: &str = "exit";
const C_VERBOSE_MODE: c_int = 1 << 0;
const C_DEBUG_MODE: c_int = 1 << 1;

/// Aplicacion de consola que lee entradas del usuario, y llama a la implementación de Wyrm en C
pub struct Wyrm {
    verbose: bool,
}

unsafe extern "C" {
    fn run_wyrm(input: *const c_char, flags: c_int);
}

impl Wyrm {
    /// Crea un nuevo objeto Wyrm
    pub fn new(verbose: bool) -> Self {
        Wyrm { verbose }
    }

    fn input_verbose(&self, input: &str) {
        println!("Entrada recibida: {}", input);
    }

    fn call_scanner(&self, input: &str) {
        let c_input = CString::new(input).expect("Error al convertir a CString");
        unsafe {
            run_wyrm(
                c_input.as_ptr(),
                if self.verbose {
                    C_VERBOSE_MODE | C_DEBUG_MODE
                } else {
                    C_DEBUG_MODE
                },
            );
        }
    }

    /// While infinto que lee entradas del usuario y llama a la implementación de Wyrm en C, en principio se sale con el comando "exit"
    pub fn run(&self) {
        println!("Wyrm Version {}", VERSION);
        println!("Power by DrukDev");
        if self.verbose {
            println!("Modo Verbose activado");
        }

        let mut input = String::new();
        let mut is_running = true;

        while is_running {
            println!("Wyrm >: ");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error al leer la entrada");

            if self.verbose {
                self.input_verbose(&input);
            }

            self.call_scanner(&input);

            if input == EXIT_COMMAND {
                is_running = false;
            }
            input = "".to_string();
        }
        println!("Wyrm se despide :D ...");
    }
}

// Los tests de scanner, parser y intérprete se declaran en este archivo ya que son "unitarios"

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[allow(dead_code)]
    #[repr(C)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum TokenType {
        TInt8 = 0,
        TInt16,
        TInt32,
        TInt64,
        TNat8,
        TNat16,
        TNat32,
        TNat64,
        TAdd,
        TSub,
        TMul,
        TDiv,
        TPow,
        TMod,
        TAssign,
        TRassign,
        TNumber,
        TString,
        TFloatNumber,
        TIdentifier,
        TEof,
        TSemicolon,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct WyrmToken {
        pub t_type: TokenType,
        pub lexeme: *mut c_char,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct TokenVector {
        pub size: usize,
        pub capacity: usize,
        pub tokens: *mut WyrmToken,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct Scanner {
        pub tokens: *mut TokenVector,
        pub flags: c_int,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct Parser {
        pub tokens: *mut TokenVector,
        pub current_index: usize,
    }

    #[allow(dead_code)]
    #[repr(C)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum AstNodeType {
        AstNumberLiteral = 0,
        AstIdentifier,
        AstBinaryExpr,
        AstVarDeclaration,
        AstUnaryExpr,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union AstNodeValue {
        pub number_expr: NumberExprStruct,
        pub identifier_expr: IdentifierExprStruct,
        pub binary_expr: BinaryExprStruct,
        pub unary_expr: UnaryExprStruct,
        pub var_decl_expr: VarDeclExprStruct,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct NumberExprStruct {
        pub value: *mut c_char,
        pub numeric_type: TokenType,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct IdentifierExprStruct {
        pub name: *mut c_char,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct BinaryExprStruct {
        pub left: *mut AstNode,
        pub operator: TokenType,
        pub right: *mut AstNode,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct UnaryExprStruct {
        pub operator: TokenType,
        pub right: *mut AstNode,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct VarDeclExprStruct {
        pub var_type: TokenType,
        pub identifier: *mut c_char,
        pub expression: *mut AstNode,
    }

    #[repr(C)]
    pub struct AstNode {
        pub node_type: AstNodeType,
        pub ast_node_value: AstNodeValue,
    }

    unsafe extern "C" {
        pub fn scanner_scan(input: *mut c_char, scanner: *mut Scanner) -> *mut TokenVector;
        pub fn create_parser(tokens: *mut TokenVector) -> *mut Parser;
        pub fn parser_parse(parser: *mut Parser) -> *mut AstNode;
    }

    // Test de scanner, si ocurre un bug se agrea un tests

    fn mostrar_diferencias_de_vectors(esperados: &[TokenType], recibidos: &TokenVector) {
        println!("\n=== DIFERENCIAS DE TOKENS ===");
        println!("Esperábamos {} tokens:", esperados.len());
        for (i, t) in esperados.iter().enumerate() {
            println!("  [{}] {:?}", i, t);
        }

        println!("\nEl Scanner de C generó {} tokens:", recibidos.size);
        for i in 0..recibidos.size {
            unsafe {
                let token = &*recibidos.tokens.add(i);
                println!("  [{}] {:?}", i, token.t_type);
            }
        }
        println!("==============================\n");
    }

    /// Función que permite comparar salida esperada con la salida real del scanner
    fn validar_toknes_esperados(
        scanner_state: &mut Scanner,
        esperados: &[TokenType],
        codigo: CString,
    ) {
        unsafe {
            let vector_ptr = scanner_scan(codigo.as_ptr() as *mut c_char, &mut *scanner_state);
            assert!(
                !vector_ptr.is_null(),
                "El scanner falló al crear el vector de tokens"
            );
            let vector = &*vector_ptr;

            if vector.size != esperados.len() {
                mostrar_diferencias_de_vectors(esperados, vector);

                panic!(
                    "Fallo por cantidad: Esperábamos {} pero C devolvió {}",
                    esperados.len(),
                    vector.size
                );
            }

            for (i, tipo_esperado) in esperados.iter().enumerate() {
                let token = &*vector.tokens.add(i);
                assert_eq!(
                    &token.t_type, tipo_esperado,
                    "ERROR en el token {}: esperado {:?}  recibido {:?}",
                    i, tipo_esperado, token.t_type
                );
            }
        }
    }

    #[test]
    fn test_scanner_salta_espacios_y_genera_eof() {
        let codigo = CString::new("   \n \t ").unwrap();

        let mut scanner_state = Scanner {
            tokens: std::ptr::null_mut(),
            flags: 0,
        };

        unsafe {
            let vector_ptr = scanner_scan(codigo.as_ptr() as *mut c_char, &mut scanner_state);
            assert!(
                !vector_ptr.is_null(),
                "El scanner falló al crear el vector de tokens"
            );
            let vector = &*vector_ptr;
            assert_eq!(vector.size, 1);
            let primer_token = &*vector.tokens.add(0);
            assert_eq!(primer_token.t_type, TokenType::TEof);
        }
    }

    #[test]
    fn test_scanner_detecta_suma_generica() {
        let codigo = CString::new("32 + 1;").unwrap();

        let mut scanner_state = Scanner {
            tokens: std::ptr::null_mut(),
            flags: 0,
        };

        let esperados = [
            TokenType::TNumber,
            TokenType::TAdd,
            TokenType::TNumber,
            TokenType::TSemicolon,
            TokenType::TEof,
        ];

        validar_toknes_esperados(&mut scanner_state, &esperados, codigo);
    }

    #[test]
    fn test_scanner_ignore_comentarios() {
        let codigo = CString::new("nat64 numero = 312; // comentario a ignorar").unwrap();

        let esperados = [
            TokenType::TNat64,
            TokenType::TIdentifier,
            TokenType::TAssign,
            TokenType::TNumber,
            TokenType::TSemicolon,
            TokenType::TEof,
        ];

        let mut scanner_state = Scanner {
            tokens: std::ptr::null_mut(),
            flags: 0,
        };

        validar_toknes_esperados(&mut scanner_state, &esperados, codigo);
    }

    #[test]
    fn test_sentencia_con_multiples_lineas() {
        let codigo = CString::new(
            "nat64 numero = 312;
     int8 aumento = 2;
     numero = numero + 12;",
        )
        .unwrap();

        let esperados = [
            //nat64 numero = 312;
            TokenType::TNat64,
            TokenType::TIdentifier,
            TokenType::TAssign,
            TokenType::TNumber,
            TokenType::TSemicolon,
            //int8 aumento = 2;
            TokenType::TInt8,
            TokenType::TIdentifier,
            TokenType::TAssign,
            TokenType::TNumber,
            TokenType::TSemicolon,
            //numero = numero +12;
            TokenType::TIdentifier,
            TokenType::TAssign,
            TokenType::TIdentifier,
            TokenType::TAdd,
            TokenType::TNumber,
            TokenType::TSemicolon,
            TokenType::TEof,
        ];

        let mut scanner_state = Scanner {
            tokens: std::ptr::null_mut(),
            flags: 0,
        };

        validar_toknes_esperados(&mut scanner_state, &esperados, codigo);
    }

    // Test de parser, si ocurre un bug se agrea un tests
    #[test]
    fn test_parser_identifica_numero() {
        unsafe {
            let lexeme_cstr = CString::new("42").unwrap();

            let token_number = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_cstr.as_ptr() as *mut c_char,
            };

            let token_eof = WyrmToken {
                t_type: TokenType::TEof,
                lexeme: std::ptr::null_mut(),
            };

            let mut tokens_array = [token_number, token_eof];

            let mut vector = TokenVector {
                size: 2,
                capacity: 2,
                tokens: tokens_array.as_mut_ptr(),
            };

            let parser_ptr = create_parser(&mut vector as *mut _);
            assert!(!parser_ptr.is_null(), "El parser falló al instanciarse");

            let ast_root_ptr = parser_parse(parser_ptr);
            assert!(!ast_root_ptr.is_null(), "El parser devolvió un árbol nulo");

            let ast_root = &*ast_root_ptr;
            assert_eq!(
                ast_root.node_type,
                AstNodeType::AstNumberLiteral,
                "El nodo no es un número"
            );

            let number_data = ast_root.ast_node_value.number_expr;
            assert_eq!(number_data.numeric_type, TokenType::TNumber);

            // Validamos que el string es "42"
            let c_str_recibido = CStr::from_ptr(number_data.value);
            assert_eq!(c_str_recibido.to_str().unwrap(), "42");
        }
    }

    #[test]
    fn test_parser_identifica_power_expresión() {
        unsafe {
            let lexeme_1 = CString::new("1").unwrap();
            let lexeme_pow = CString::new("^").unwrap();
            let lexeme_2 = CString::new("2").unwrap();

            let token_1 = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_1.as_ptr() as *mut c_char,
            };

            let token_pow = WyrmToken {
                t_type: TokenType::TPow,
                lexeme: lexeme_pow.as_ptr() as *mut c_char,
            };

            let token_2 = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_2.as_ptr() as *mut c_char,
            };

            let token_eof = WyrmToken {
                t_type: TokenType::TEof,
                lexeme: std::ptr::null_mut(),
            };

            let mut tokens_array = [token_1, token_pow, token_2, token_eof];

            let mut vector = TokenVector {
                size: 4,
                capacity: 4,
                tokens: tokens_array.as_mut_ptr(),
            };

            let parser_ptr = create_parser(&mut vector as *mut _);
            assert!(!parser_ptr.is_null(), "El parser falló al instanciarse");

            let ast_root_ptr = parser_parse(parser_ptr);
            assert!(!ast_root_ptr.is_null(), "El parser devolvió un árbol nulo");

            let ast_root = &*ast_root_ptr;
            assert_eq!(
                ast_root.node_type,
                AstNodeType::AstBinaryExpr,
                "La raiz no es una operacion bianria"
            );

            let binary_data = ast_root.ast_node_value.binary_expr;
            assert_eq!(
                binary_data.operator,
                TokenType::TPow,
                "El operador no es exponente"
            );

            assert!(!binary_data.left.is_null(), "No hay nodo izquiedo");

            let left_node = &*binary_data.left;
            assert_eq!(
                left_node.node_type,
                AstNodeType::AstNumberLiteral,
                "Operador izquierdo no es un numero"
            );

            let left_val = CStr::from_ptr(left_node.ast_node_value.number_expr.value);
            assert_eq!(
                left_val.to_str().unwrap(),
                "1",
                "Lexema del nodo izquierdo fallido"
            );

            assert!(!binary_data.right.is_null(), "El hay nodo derecho");
            let right_node = &*binary_data.right;
            assert_eq!(
                right_node.node_type,
                AstNodeType::AstNumberLiteral,
                "El nodo derecho no es un numero"
            );

            let right_val = CStr::from_ptr(right_node.ast_node_value.number_expr.value);
            assert_eq!(
                right_val.to_str().unwrap(),
                "2",
                "lexema del nodo derecho fallido"
            );
        }
    }

    #[test]
    fn test_parser_identifica_unary_expresión() {
        unsafe {
            let lexeme_sub = CString::new("-").unwrap();
            let lexeme_23 = CString::new("23").unwrap();

            let token_sub = WyrmToken {
                t_type: TokenType::TSub,
                lexeme: lexeme_sub.as_ptr() as *mut c_char,
            };

            let token_23 = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_23.as_ptr() as *mut c_char,
            };

            let token_eof = WyrmToken {
                t_type: TokenType::TEof,
                lexeme: std::ptr::null_mut(),
            };

            let mut tokens_array = [token_sub, token_23, token_eof];

            let mut vector = TokenVector {
                size: 3,
                capacity: 3,
                tokens: tokens_array.as_mut_ptr(),
            };

            let parser_ptr = create_parser(&mut vector as *mut _);
            assert!(!parser_ptr.is_null(), "El parser falló al instanciarse");

            let ast_root_ptr = parser_parse(parser_ptr);
            assert!(!ast_root_ptr.is_null(), "El parser devolvió un árbol nulo");

            let ast_root = &*ast_root_ptr;
            assert_eq!(
                ast_root.node_type,
                AstNodeType::AstUnaryExpr,
                "La raiz no es una operacion Unaria"
            );

            let binary_data = ast_root.ast_node_value.binary_expr;
            assert_eq!(
                binary_data.operator,
                TokenType::TSub,
                "El operador no una negacion"
            );

            assert!(!binary_data.right.is_null(), "El hay nodo derecho");
            let right_node = &*binary_data.right;

            assert_eq!(
                right_node.node_type,
                AstNodeType::AstNumberLiteral,
                "El nodo derecho no es un numero"
            );

            let right_val = CStr::from_ptr(right_node.ast_node_value.number_expr.value);
            assert_eq!(
                right_val.to_str().unwrap(),
                "23",
                "lexema del nodo derecho fallido"
            );
        }
    }
}
