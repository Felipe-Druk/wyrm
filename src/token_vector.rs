use std::os::raw::c_char;

#[allow(dead_code)]
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // enteros
    TInt8 = 0,
    TInt16,
    TInt32,
    TInt64,
    // naturals
    TNat8,
    TNat16,
    TNat32,
    TNat64,
    // flotantes
    TFloat32,
    TFloat64,
    // booleanos
    TBool,
    // palabras para booleanos
    TTrue,
    TFalse,
    TNot,
    TAnd,
    TOr,
    TXor,
    // operadores arigmeticos
    TAdd,
    TSub,
    TMul,
    TDiv,
    TPow,
    TMod,
    // operadores logicos
    TEqual,
    TNotEqual,
    TLess,
    TGreater,
    TLessEqual,
    TGreaterEqual,
    // operadores varios
    TAssign,
    TRassign,
    TNumber,
    TString,
    TFloatNumber,
    // importante
    TIdentifier,
    // If/else
    TIf,
    TElse,
    // loops
    TWhile,
    TFor,
    // Limitadores
    TEof,
    TSemicolon,
    TLparen,
    TRparen,
    TLbrace,
    TRbrace,
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
