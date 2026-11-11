#include "compiler_functions.h"

#define INITIAL_FUNC_LOCALS 8

static size_t emit_func_jump(wyrm_chunk_t *chunk, wyrm_opcode_t opcode) {
    chunk_write_byte(chunk, (uint8_t)opcode);
    size_t operand_offset = chunk->code.size;
    chunk_write_uint16(chunk, 0xFFFF);
    return operand_offset;
}

static void patch_func_jump(wyrm_chunk_t *chunk, size_t operand_offset) {
    uint16_t jump_target = (uint16_t)chunk->code.size;
    uint8_t *code_bytes = (uint8_t *)chunk->code.data;
    code_bytes[operand_offset] = (uint8_t)(jump_target & 0xFF);
    code_bytes[operand_offset + 1] = (uint8_t)((jump_target >> 8) & 0xFF);
}

void compile_function_decl(ast_node_t *node, compiler_t *compiler) {
    ast_function_decl_t func_decl = node->ast_node_value.function_decl;
    size_t jump_over_func = emit_func_jump(compiler->chunk, WYRM_OP_JUMP);
    uint32_t body_start = (uint32_t)compiler->chunk->code.size;
    uint16_t func_id = (uint16_t)compiler->chunk->functions.size;
    wyrm_func_entry_t entry = {
        .arg_count = (uint8_t)func_decl.function.arg_count,
        .return_type = func_decl.function.returned,
        .code_offset = body_start,
        .code_length = 0 // Se actualiza al terminar de compilar el cuerpo
    };
    vector_push(&compiler->chunk->functions, &entry);

    wyrm_func_symbol_t sym = {.name = func_decl.name, .func_id = func_id};
    vector_push(&compiler->func_symbols, &sym);
    wyrm_vector_t outer_locals = compiler->locals;
    int outer_scope = compiler->scope_level;

    compiler->locals = create_vector(sizeof(wyrm_local_t), INITIAL_FUNC_LOCALS);
    compiler->scope_level = 1;

    for (size_t i = 0; i < func_decl.function.arg_count; i++) {
        wyrm_local_t param = {.name = func_decl.function.args[i].name,
                              .var_type = func_decl.function.args[i].type,
                              .scope_level = compiler->scope_level};
        vector_push(&compiler->locals, &param);
    }

    compile_node(func_decl.function.body, compiler);
    chunk_write_byte(compiler->chunk, WYRM_OP_RETURN);

    uint32_t body_end = (uint32_t)compiler->chunk->code.size;
    wyrm_func_entry_t *funcs_arr = (wyrm_func_entry_t *)compiler->chunk->functions.data;
    funcs_arr[func_id].code_length = body_end - body_start;
    patch_func_jump(compiler->chunk, jump_over_func);

    destroy_vector(&compiler->locals);
    compiler->locals = outer_locals;
    compiler->scope_level = outer_scope;
}

void compile_call_expr(ast_node_t *node, compiler_t *compiler) {
    ast_call_expr_t call = node->ast_node_value.call_expr;
    for (size_t i = 0; i < call.arg_count; i++) {
        compile_node(call.args[i], compiler);
    }
    int func_id = resolve_function(compiler, call.name);
    if (func_id == ERRRESOLVER) {
        panic(ERR_UNDEFINED_VAR, "Undefined function in call: %s", call.name);
    }
    chunk_write_byte(compiler->chunk, WYRM_OP_CALL);
    chunk_write_uint16(compiler->chunk, (uint16_t)func_id);
}