use std::os::raw::c_char;

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
    TFloat32,
    TFloat64,
    TBool,
    TTrue,
    TFalse,
    TNot,
    TAnd,
    TOr,
    TXor,
    TAdd,
    TSub,
    TMul,
    TDiv,
    TPow,
    TMod,
    TEqual,
    TNotEqual,
    TLess,
    TGreater,
    TLessEqual,
    TGreaterEqual,
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
