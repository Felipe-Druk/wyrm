#![allow(dead_code)]

use crate::token_vector::*;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct Parser {
    pub tokens: *mut TokenVector,
    pub current_index: usize,
}
#[allow(clippy::enum_variant_names)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    unsafe extern "C" {
        pub fn create_parser(tokens: *mut TokenVector) -> *mut Parser;
        pub fn parser_parse(parser: *mut Parser) -> *mut AstNode;
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

    #[test]
    fn test_parser_identifica_factor_expresión() {
        unsafe {
            let lexeme_11 = CString::new("11").unwrap();
            let lexeme_mul = CString::new("*").unwrap();
            let lexeme_3 = CString::new("3").unwrap();

            let token_11 = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_11.as_ptr() as *mut c_char,
            };

            let token_mul = WyrmToken {
                t_type: TokenType::TMul,
                lexeme: lexeme_mul.as_ptr() as *mut c_char,
            };

            let token_3 = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_3.as_ptr() as *mut c_char,
            };

            let token_eof = WyrmToken {
                t_type: TokenType::TEof,
                lexeme: std::ptr::null_mut(),
            };

            let mut tokens_array = [token_11, token_mul, token_3, token_eof];

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
                TokenType::TMul,
                "El operador no es una multiplicasion"
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
                "11",
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
                "3",
                "lexema del nodo derecho fallido"
            );
        }
    }

    #[test]
    fn test_parser_identifica_term_expresión() {
        unsafe {
            let lexeme_1 = CString::new("1").unwrap();
            let lexeme_add = CString::new("+").unwrap();
            let lexeme_2 = CString::new("2").unwrap();

            let token_1 = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_1.as_ptr() as *mut c_char,
            };

            let token_add = WyrmToken {
                t_type: TokenType::TAdd,
                lexeme: lexeme_add.as_ptr() as *mut c_char,
            };

            let token_2 = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_2.as_ptr() as *mut c_char,
            };

            let token_eof = WyrmToken {
                t_type: TokenType::TEof,
                lexeme: std::ptr::null_mut(),
            };

            let mut tokens_array = [token_1, token_add, token_2, token_eof];

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
                TokenType::TAdd,
                "El operador no es una adicion"
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
    fn test_parser_identifica_multiple_expresión() {
        unsafe {
            let lexeme_1 = CString::new("1").unwrap();
            let lexeme_add = CString::new("+").unwrap();
            let lexeme_2 = CString::new("2").unwrap();
            let lexeme_div = CString::new("/").unwrap();
            let lexeme_3 = CString::new("3").unwrap();

            let token_add = WyrmToken {
                t_type: TokenType::TAdd,
                lexeme: lexeme_add.as_ptr() as *mut c_char,
            };
            let token_1 = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_1.as_ptr() as *mut c_char,
            };
            let token_2 = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_2.as_ptr() as *mut c_char,
            };
            let token_3 = WyrmToken {
                t_type: TokenType::TNumber,
                lexeme: lexeme_3.as_ptr() as *mut c_char,
            };
            let token_div = WyrmToken {
                t_type: TokenType::TDiv,
                lexeme: lexeme_div.as_ptr() as *mut c_char,
            };

            let token_eof = WyrmToken {
                t_type: TokenType::TEof,
                lexeme: std::ptr::null_mut(),
            };

            let mut tokens_array = [token_1, token_add, token_2, token_div, token_3, token_eof];

            let mut vector = TokenVector {
                size: 6,
                capacity: 6,
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
                TokenType::TAdd,
                "El operador no es una adicion"
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
                AstNodeType::AstBinaryExpr,
                "El nodo derecho no es una operacion"
            );

            let right_binary_data = right_node.ast_node_value.binary_expr;

            let right_left_node = &*right_binary_data.left;

            assert_eq!(
                right_left_node.node_type,
                AstNodeType::AstNumberLiteral,
                "Operador izquierdo del divisor no es un numero"
            );
            let right_left_val = CStr::from_ptr(right_left_node.ast_node_value.number_expr.value);
            assert_eq!(
                right_left_val.to_str().unwrap(),
                "2",
                "Lexema del nodo izquierdo de la division fallido"
            );

            let right_right_node = &*right_binary_data.right;

            assert_eq!(
                right_right_node.node_type,
                AstNodeType::AstNumberLiteral,
                "Operador derecho del divisor no es un numero"
            );
            let right_right_val = CStr::from_ptr(right_right_node.ast_node_value.number_expr.value);
            assert_eq!(
                right_right_val.to_str().unwrap(),
                "3",
                "Lexema del nodo izquierdo de la division fallido"
            );
        }
    }
}
