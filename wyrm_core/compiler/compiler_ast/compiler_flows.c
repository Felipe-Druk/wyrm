#include "compiler_flows.h"

#define MASK2BYTES 0xFF
#define MASK4BYTES 0xFFFF
#define SHIFT8 8

// Emite un salto con un offset temporal 0xFFFF
static size_t emit_jump(wyrm_chunk_t *chunk, wyrm_opcode_t opcode) {
    chunk_write_byte(chunk, (uint8_t)opcode);
    size_t operand_offset = chunk->code.size;
    chunk_write_uint16(chunk, MASK4BYTES);
    return operand_offset;
}

// Sobrescribe el offset temporal
static void patch_jump(wyrm_chunk_t *chunk, size_t operand_offset) {
    uint16_t jump_target = (uint16_t)chunk->code.size;
    uint8_t *code_bytes = (uint8_t *)chunk->code.data;

    code_bytes[operand_offset] = (uint8_t)(jump_target & MASK2BYTES);
    code_bytes[operand_offset + 1] = (uint8_t)((jump_target >> SHIFT8) & MASK2BYTES);
}

void compile_while_expr(ast_node_t *node, compiler_t *compiler) {
    ast_while_expr_t while_expr = node->ast_node_value.while_expr;
    uint16_t loop_start = (uint16_t)compiler->chunk->code.size;
    compile_node(while_expr.condition, compiler);
    size_t exit_jump = emit_jump(compiler->chunk, WYRM_OP_JUMP_IF_NOT);
    compile_node(while_expr.body, compiler);
    chunk_write_byte(compiler->chunk, WYRM_OP_JUMP);
    chunk_write_uint16(compiler->chunk, loop_start);
    patch_jump(compiler->chunk, exit_jump);
}

void compile_if_expr(ast_node_t *node, compiler_t *compiler) {
    ast_if_expr_t if_expr = node->ast_node_value.if_expr;
    compile_node(if_expr.condition, compiler);
    size_t then_jump = emit_jump(compiler->chunk, WYRM_OP_JUMP_IF_NOT);
    compile_node(if_expr.then_branch, compiler);

    if (if_expr.else_branch != NULL) {
        size_t else_jump = emit_jump(compiler->chunk, WYRM_OP_JUMP);
        patch_jump(compiler->chunk, then_jump);

        compile_node(if_expr.else_branch, compiler);
        patch_jump(compiler->chunk, else_jump);
    } else {
        patch_jump(compiler->chunk, then_jump);
    }
}