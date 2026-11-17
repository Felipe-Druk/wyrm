#include "compiler_flows.h"

// Estrutra interna para armar un ciclo for
typedef struct ForSlots {
    uint16_t vec;
    uint16_t len;
    uint16_t idx;
    uint16_t iter;
} for_slots_t;

// Revisa el incido del for
static for_slots_t setup_for_locals(compiler_t *compiler, ast_node_t *header) {
    if (header == NULL || header->type != AST_ASSIGNMENT ||
        header->ast_node_value.assignment_expr.operator != T_LARROW) {
        panic(ERR_SYNTAX, "Compile error: 'for' loop requires extraction syntax 'item <- vector'");
    }

    wyrm_chunk_t *chunk = compiler->chunk;
    char *iter_name = header->ast_node_value.assignment_expr.name;
    for_slots_t slots = {0};

    compile_node(header->ast_node_value.assignment_expr.value, compiler);
    slots.vec = add_local_slot(compiler, (char *)"", VAL_VEC, 0);

    emit_local_op(chunk, WYRM_OP_GET_LOCAL, slots.vec);
    chunk_write_byte(chunk, WYRM_OP_VEC_LEN);
    slots.len = add_local_slot(compiler, (char *)"", VAL_INT64, 0);

    emit_constant_val(chunk, VAL_INT64, 0);
    slots.idx = add_local_slot(compiler, (char *)"", VAL_INT64, 0);

    emit_constant_val(chunk, VAL_VOID, 0);
    slots.iter = add_local_slot(compiler, iter_name, VAL_VOID, 1);

    return slots;
}

// Verifica que sigamos en el for (indice < len)
static size_t emit_for_step_header(wyrm_chunk_t *chunk, for_slots_t slots) {
    emit_local_op(chunk, WYRM_OP_GET_LOCAL, slots.idx);
    emit_local_op(chunk, WYRM_OP_GET_LOCAL, slots.len);
    chunk_write_byte(chunk, WYRM_OP_LESS);

    size_t exit_jump = chunk_emit_jump(chunk, WYRM_OP_JUMP_IF_NOT);

    emit_local_op(chunk, WYRM_OP_GET_LOCAL, slots.vec);
    emit_local_op(chunk, WYRM_OP_GET_LOCAL, slots.idx);
    chunk_write_byte(chunk, WYRM_OP_VEC_GET);
    emit_local_op(chunk, WYRM_OP_SET_LOCAL, slots.iter);

    return exit_jump;
}

// indice ++
static void emit_for_increment(wyrm_chunk_t *chunk, uint16_t idx_slot, uint16_t loop_start) {
    emit_local_op(chunk, WYRM_OP_GET_LOCAL, idx_slot);
    emit_constant_val(chunk, VAL_INT64, 1);
    chunk_write_byte(chunk, WYRM_OP_ADD);
    emit_local_op(chunk, WYRM_OP_SET_LOCAL, idx_slot);

    chunk_write_byte(chunk, WYRM_OP_JUMP);
    chunk_write_uint16(chunk, loop_start);
}

void compile_for_expr(ast_node_t *node, compiler_t *compiler) {
    ast_while_expr_t for_expr = node->ast_node_value.while_expr;
    for_slots_t slots = setup_for_locals(compiler, for_expr.condition);

    uint16_t loop_start = (uint16_t)compiler->chunk->code.size;
    size_t exit_jump = emit_for_step_header(compiler->chunk, slots);

    compile_node(for_expr.body, compiler);
    emit_for_increment(compiler->chunk, slots.idx, loop_start);

    chunk_patch_jump(compiler->chunk, exit_jump);
    ((wyrm_local_t *)compiler->locals.data)[slots.iter].name = (char *)"";
}

void compile_while_expr(ast_node_t *node, compiler_t *compiler) {
    ast_while_expr_t while_expr = node->ast_node_value.while_expr;
    uint16_t loop_start = (uint16_t)compiler->chunk->code.size;

    compile_node(while_expr.condition, compiler);
    size_t exit_jump = chunk_emit_jump(compiler->chunk, WYRM_OP_JUMP_IF_NOT);

    compiler->scope_level += 2;
    compile_node(while_expr.body, compiler);
    compiler->scope_level -= 2;

    chunk_write_byte(compiler->chunk, WYRM_OP_JUMP);
    chunk_write_uint16(compiler->chunk, loop_start);
    chunk_patch_jump(compiler->chunk, exit_jump);
}

void compile_if_expr(ast_node_t *node, compiler_t *compiler) {
    ast_if_expr_t if_expr = node->ast_node_value.if_expr;

    compile_node(if_expr.condition, compiler);
    size_t then_jump = chunk_emit_jump(compiler->chunk, WYRM_OP_JUMP_IF_NOT);

    compiler->scope_level += 2;
    compile_node(if_expr.then_branch, compiler);

    if (if_expr.else_branch != NULL) {
        size_t else_jump = chunk_emit_jump(compiler->chunk, WYRM_OP_JUMP);
        chunk_patch_jump(compiler->chunk, then_jump);
        compile_node(if_expr.else_branch, compiler);
        chunk_patch_jump(compiler->chunk, else_jump);
    } else {
        chunk_patch_jump(compiler->chunk, then_jump);
    }
    compiler->scope_level -= 2;
}