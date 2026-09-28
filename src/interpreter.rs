#![allow(dead_code)]

use crate::ast_node::AstNode;
use std::fmt;
use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug)]
pub struct Interpreter {
    pub root: *mut AstNode,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WyrmValueType {
    ValVoid = 0,
    ValNat8,
    ValNat16,
    ValNat32,
    ValNat64,
    ValInt8,
    ValInt16,
    ValInt32,
    ValInt64,
    ValFloat32,
    ValFloat64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union WyrmValueData {
    pub nat8_val: u8,
    pub nat16_val: u16,
    pub nat32_val: u32,
    pub nat64_val: u64,
    pub int8_val: i8,
    pub int16_val: i16,
    pub int32_val: i32,
    pub int64_val: i64,
    pub float32_val: f32,
    pub float64_val: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WyrmValue {
    pub value_type: WyrmValueType,
    pub value: WyrmValueData,
}

// para mayor comodidad dentro de consola leemos el valor sin el tipo
impl fmt::Display for WyrmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            match self.value_type {
                WyrmValueType::ValInt64 => write!(f, "{}", self.value.int64_val),
                WyrmValueType::ValNat64 => write!(f, "{}", self.value.nat64_val),
                WyrmValueType::ValFloat64 => write!(f, "{}", self.value.float64_val),

                WyrmValueType::ValVoid => write!(f, "void"),
                _ => write!(f, "<tipo desconocido>"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_node::*;
    use crate::token_vector::*;

    unsafe extern "C" {
        pub fn interpreter_interpret(interpreter: *mut Interpreter) -> WyrmValue;
        pub fn create_interpreter(root: *mut AstNode) -> *mut Interpreter;
    }

    #[test]
    fn test_interpreter_evalua_ast_number() {
        let lex_10 = std::ffi::CString::new("10").unwrap();
        let mut int_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_10.as_ptr() as *mut _,
                },
            },
        };
        unsafe {
            let interpreter_ptr = create_interpreter(&mut int_node as *mut _);
            let resultado = interpreter_interpret(interpreter_ptr);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo no es int64"
            );
            assert_eq!(resultado.value.int64_val, 10, "El valor no fue conservado");
        }
    }

    #[test]
    fn test_interpreter_evalua_ast_number_float() {
        let lex_10 = std::ffi::CString::new("10.0").unwrap();
        let mut int_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TFloatNumber,
                    value: lex_10.as_ptr() as *mut _,
                },
            },
        };
        unsafe {
            let interpreter_ptr = create_interpreter(&mut int_node as *mut _);
            let resultado = interpreter_interpret(interpreter_ptr);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValFloat64,
                "El tipo no es float64"
            );
            assert_eq!(
                resultado.value.float64_val, 10.0,
                "El valor no fue conservado"
            );
        }
    }

    #[test]
    fn test_interpreter_evalua_suma_basica() {
        let lex_10 = std::ffi::CString::new("10").unwrap();
        let lex_20 = std::ffi::CString::new("20").unwrap();

        let mut left_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_10.as_ptr() as *mut _,
                },
            },
        };
        let mut right_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_20.as_ptr() as *mut _,
                },
            },
        };

        let mut root_node = AstNode {
            node_type: AstNodeType::AstBinaryExpr,
            ast_node_value: AstNodeValue {
                binary_expr: BinaryExprStruct {
                    left: &mut left_node as *mut _,
                    operator: TokenType::TAdd,
                    right: &mut right_node as *mut _,
                },
            },
        };

        unsafe {
            let interpreter_ptr = create_interpreter(&mut root_node as *mut _);
            let resultado = interpreter_interpret(interpreter_ptr);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante de la suma no es int64"
            );
            assert_eq!(
                resultado.value.int64_val, 30,
                "El cálculo matemático de 10 + 20 falló"
            );
        }
    }

    #[test]
    fn test_interpreter_evalua_resta_basica() {
        let lex_10 = std::ffi::CString::new("10").unwrap();
        let lex_20 = std::ffi::CString::new("20").unwrap();

        let mut left_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_10.as_ptr() as *mut _,
                },
            },
        };
        let mut right_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_20.as_ptr() as *mut _,
                },
            },
        };

        let mut root_node = AstNode {
            node_type: AstNodeType::AstBinaryExpr,
            ast_node_value: AstNodeValue {
                binary_expr: BinaryExprStruct {
                    left: &mut left_node as *mut _,
                    operator: TokenType::TSub,
                    right: &mut right_node as *mut _,
                },
            },
        };

        unsafe {
            let interpreter_ptr = create_interpreter(&mut root_node as *mut _);
            let resultado = interpreter_interpret(interpreter_ptr);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante de la resta no es int64"
            );
            assert_eq!(
                resultado.value.int64_val, -10,
                "El cálculo matemático de 10 - 20 falló"
            );
        }
    }

    #[test]
    fn test_interpreter_evalua_multiplicación_basica() {
        let lex_10 = std::ffi::CString::new("10").unwrap();
        let lex_20 = std::ffi::CString::new("20").unwrap();

        let mut left_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_10.as_ptr() as *mut _,
                },
            },
        };
        let mut right_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_20.as_ptr() as *mut _,
                },
            },
        };

        let mut root_node = AstNode {
            node_type: AstNodeType::AstBinaryExpr,
            ast_node_value: AstNodeValue {
                binary_expr: BinaryExprStruct {
                    left: &mut left_node as *mut _,
                    operator: TokenType::TMul,
                    right: &mut right_node as *mut _,
                },
            },
        };

        unsafe {
            let interpreter_ptr = create_interpreter(&mut root_node as *mut _);
            let resultado = interpreter_interpret(interpreter_ptr);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante de la multiplicación  no es int64"
            );
            assert_eq!(
                resultado.value.int64_val, 200,
                "El cálculo matemático de 10 * 20 falló"
            );
        }
    }

    #[test]
    fn test_interpreter_evalua_division_basica() {
        let lex_10 = std::ffi::CString::new("10").unwrap();
        let lex_20 = std::ffi::CString::new("20").unwrap();

        let mut left_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_10.as_ptr() as *mut _,
                },
            },
        };
        let mut right_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_20.as_ptr() as *mut _,
                },
            },
        };

        let mut root_node = AstNode {
            node_type: AstNodeType::AstBinaryExpr,
            ast_node_value: AstNodeValue {
                binary_expr: BinaryExprStruct {
                    left: &mut left_node as *mut _,
                    operator: TokenType::TDiv,
                    right: &mut right_node as *mut _,
                },
            },
        };

        unsafe {
            let interpreter_ptr = create_interpreter(&mut root_node as *mut _);
            let resultado = interpreter_interpret(interpreter_ptr);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante de la division no es int64"
            );
            assert_eq!(
                resultado.value.int64_val, 0,
                "El cálculo matemático de 10 / 20 falló"
            );
        }
    }

    #[test]
    fn test_interpreter_evalua_modulo_basico() {
        let lex_10 = std::ffi::CString::new("10").unwrap();
        let lex_20 = std::ffi::CString::new("20").unwrap();

        let mut left_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_10.as_ptr() as *mut _,
                },
            },
        };
        let mut right_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_20.as_ptr() as *mut _,
                },
            },
        };

        let mut root_node = AstNode {
            node_type: AstNodeType::AstBinaryExpr,
            ast_node_value: AstNodeValue {
                binary_expr: BinaryExprStruct {
                    left: &mut left_node as *mut _,
                    operator: TokenType::TMod,
                    right: &mut right_node as *mut _,
                },
            },
        };

        unsafe {
            let interpreter_ptr = create_interpreter(&mut root_node as *mut _);
            let resultado = interpreter_interpret(interpreter_ptr);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante del modulo no es int64"
            );
            assert_eq!(
                resultado.value.int64_val, 10,
                "El cálculo matemático de 10 % 20 falló"
            );
        }
    }

    #[test]
    fn test_interpreter_evalua_potencia_basica() {
        let lex_2 = std::ffi::CString::new("2").unwrap();
        let lex_3 = std::ffi::CString::new("3").unwrap();

        let mut left_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_2.as_ptr() as *mut _,
                },
            },
        };
        let mut right_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_3.as_ptr() as *mut _,
                },
            },
        };

        let mut root_node = AstNode {
            node_type: AstNodeType::AstBinaryExpr,
            ast_node_value: AstNodeValue {
                binary_expr: BinaryExprStruct {
                    left: &mut left_node as *mut _,
                    operator: TokenType::TPow,
                    right: &mut right_node as *mut _,
                },
            },
        };

        unsafe {
            let interpreter_ptr = create_interpreter(&mut root_node as *mut _);
            let resultado = interpreter_interpret(interpreter_ptr);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante de la potencia  no es int64"
            );
            assert_eq!(
                resultado.value.int64_val, 8,
                "El cálculo matemático de 2 ^ 3 falló"
            );
        }
    }

    #[test]
    fn test_interpreter_evalua_negacion_basica() {
        let lex_20 = std::ffi::CString::new("20").unwrap();
        let mut right_node = AstNode {
            node_type: AstNodeType::AstNumberLiteral,
            ast_node_value: AstNodeValue {
                number_expr: NumberExprStruct {
                    numeric_type: TokenType::TNumber,
                    value: lex_20.as_ptr() as *mut _,
                },
            },
        };

        let mut root_node = AstNode {
            node_type: AstNodeType::AstUnaryExpr,
            ast_node_value: AstNodeValue {
                unary_expr: UnaryExprStruct {
                    operator: TokenType::TSub,
                    right: &mut right_node as *mut _,
                },
            },
        };

        unsafe {
            let interpreter_ptr = create_interpreter(&mut root_node as *mut _);
            let resultado = interpreter_interpret(interpreter_ptr);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante de la negacion no es int64"
            );
            assert_eq!(
                resultado.value.int64_val, -20,
                "El cálculo matemático de -20 falló"
            );
        }
    }
}
