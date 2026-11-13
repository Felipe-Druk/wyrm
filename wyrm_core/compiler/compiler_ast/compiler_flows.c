#include "compiler_flows.h"

void compile_while_expr(ast_node_t *node, compiler_t *compiler) {
    ast_while_expr_t while_expr = node->ast_node_value.while_expr;
    uint16_t loop_start = (uint16_t)compiler->chunk->code.size;
    compile_node(while_expr.condition, compiler);
    size_t exit_jump = chunk_emit_jump(compiler->chunk, (uint16_t)WYRM_OP_JUMP_IF_NOT);
    compile_node(while_expr.body, compiler);
    chunk_write_byte(compiler->chunk, WYRM_OP_JUMP);
    chunk_write_uint16(compiler->chunk, loop_start);
    chunk_patch_jump(compiler->chunk, exit_jump);
}

void compile_if_expr(ast_node_t *node, compiler_t *compiler) {
    ast_if_expr_t if_expr = node->ast_node_value.if_expr;
    compile_node(if_expr.condition, compiler);
    size_t then_jump = chunk_emit_jump(compiler->chunk, WYRM_OP_JUMP_IF_NOT);
    compile_node(if_expr.then_branch, compiler);

    if (if_expr.else_branch != NULL) {
        size_t else_jump = chunk_emit_jump(compiler->chunk, WYRM_OP_JUMP);
        chunk_patch_jump(compiler->chunk, then_jump);

        compile_node(if_expr.else_branch, compiler);
        chunk_patch_jump(compiler->chunk, else_jump);
    } else {
        chunk_patch_jump(compiler->chunk, then_jump);
    }
}