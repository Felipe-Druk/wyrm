#include "compiler_operations.h"

void compile_binary_expr(ast_node_t *node, compiler_t *compiler) {
    ast_binary_expr_t bin = node->ast_node_value.binary_expr;

    compile_node(bin.left, compiler);
    compile_node(bin.right, compiler);
    switch (bin.operator) {
    case T_ADD:
        chunk_write_byte(compiler->chunk, WYRM_OP_ADD);
        break;
    case T_SUB:
        chunk_write_byte(compiler->chunk, WYRM_OP_SUB);
        break;
    case T_MUL:
        chunk_write_byte(compiler->chunk, WYRM_OP_MUL);
        break;
    case T_DIV:
        chunk_write_byte(compiler->chunk, WYRM_OP_DIV);
        break;
    case T_MOD:
        chunk_write_byte(compiler->chunk, WYRM_OP_MOD);
        break;
    case T_AND:
        chunk_write_byte(compiler->chunk, WYRM_OP_AND);
        break;
    case T_OR:
        chunk_write_byte(compiler->chunk, WYRM_OP_OR);
        break;

    default:
        panic(ERR_UNKNOWN_OP, "Unknown binary operator in compiler: %d", bin.operator);
    }
}