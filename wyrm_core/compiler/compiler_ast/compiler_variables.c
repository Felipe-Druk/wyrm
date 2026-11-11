#include "compiler_variables.h"

void compile_var_declaration(ast_node_t *node, compiler_t *compiler) {
    ast_var_decl_t decl = node->ast_node_value.var_decl_expr;
    compile_node(decl.expression, compiler);

    wyrm_value_type_t target_type = token_to_value_type(decl.var_type);
    chunk_write_byte(compiler->chunk, WYRM_OP_CAST);
    chunk_write_byte(compiler->chunk, (uint8_t)target_type);

    wyrm_local_t local = {.name = decl.identifier, .var_type = target_type, .scope_level = compiler->scope_level};
    vector_push(&compiler->locals, &local);
}

void compile_identifier(ast_node_t *node, compiler_t *compiler) {
    const char *name = node->ast_node_value.identifier_expr.name;
    int slot = resolve_local(compiler, name);

    if (slot == ERRRESOLVER) {
        panic(ERR_UNDEFINED_VAR, "Error UNDEFINED_VAR: %s", name);
    }

    chunk_write_byte(compiler->chunk, WYRM_OP_GET_LOCAL);
    chunk_write_uint16(compiler->chunk, (uint16_t)slot);
}

void compile_assignment(ast_node_t *node, compiler_t *compiler) {
    ast_assignment_t assign = node->ast_node_value.assignment_expr;
    compile_node(assign.value, compiler);

    int slot = resolve_local(compiler, assign.name);
    if (slot == ERRRESOLVER) {
        panic(ERR_UNDEFINED_VAR, "Error UNDEFINED_VAR in assignment: %s", assign.name);
    }

    chunk_write_byte(compiler->chunk, WYRM_OP_SET_LOCAL);
    chunk_write_uint16(compiler->chunk, (uint16_t)slot);
}