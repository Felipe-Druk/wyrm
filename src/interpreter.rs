#![allow(dead_code)]

use crate::ast_node::AstNode;
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
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WyrmValue {
    pub value_type: WyrmValueType,
    pub value: WyrmValueData,
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "C" {
        pub fn interpreter_interpret(interpreter: *mut Interpreter) -> WyrmValue;
    }
}
