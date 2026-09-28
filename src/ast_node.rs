use crate::token_vector::TokenType;
use std::os::raw::c_char;

#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AstNodeType {
    AstNumberLiteral = 0,
    AstBoolLiteral,
    AstIdentifier,
    AstBinaryExpr,
    AstVarDeclaration,
    AstUnaryExpr,
    AstBlock,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union AstNodeValue {
    pub number_expr: NumberExprStruct,
    pub bool_expr: BoolExprStruct,
    pub identifier_expr: IdentifierExprStruct,
    pub binary_expr: BinaryExprStruct,
    pub unary_expr: UnaryExprStruct,
    pub var_decl_expr: VarDeclExprStruct,
    pub block_expr: AstBlock,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NumberExprStruct {
    pub value: *mut c_char,
    pub numeric_type: TokenType,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BoolExprStruct {
    pub value: bool,
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AstBlock {
    pub size: usize,
    pub capacity: usize,
    pub nodes: *mut *mut AstNode,
}
