#include "compiler_variables.h"

void compile_var_declaration(ast_node_t *node, compiler_t *compiler) {
    ast_var_decl_t decl = node->ast_node_value.var_decl_expr;
    wyrm_value_type_t target_type = token_to_value_type(decl.var_type);

    if (decl.expression == NULL) {
        if (decl.var_type == T_VEC) {
            chunk_write_byte(compiler->chunk, WYRM_OP_BUILD_VEC);
            chunk_write_uint16(compiler->chunk, 0);
        } else {
            emit_constant_val(compiler->chunk, VAL_INT64, 0);
        }
    } else {
        compile_node(decl.expression, compiler);
    }

    if (decl.var_type == T_VEC) {
        wyrm_value_type_t elem_type = token_to_value_type(decl.element_type);
        chunk_write_byte(compiler->chunk, WYRM_OP_VEC_CAST);
        chunk_write_byte(compiler->chunk, (uint8_t)elem_type);
    } else {
        chunk_write_byte(compiler->chunk, WYRM_OP_CAST);
        chunk_write_byte(compiler->chunk, (uint8_t)target_type);
    }

    int existing_slot = resolve_local(compiler, decl.identifier);
    uint16_t slot = (existing_slot != ERRRESOLVER) ? (uint16_t)existing_slot
                                                   : add_local_slot(compiler, decl.identifier, target_type, 0);

    if (compiler->scope_level >= 2 || existing_slot != ERRRESOLVER) {
        emit_local_op(compiler->chunk, WYRM_OP_SET_LOCAL, slot);
    }
}

void compile_identifier(ast_node_t *node, compiler_t *compiler) {
    const char *name = node->ast_node_value.identifier_expr.name;
    int slot = resolve_local(compiler, name);

    if (slot == ERRRESOLVER) {
        panic(ERR_UNDEFINED_VAR, "Error UNDEFINED_VAR in compiler: %s", name);
    }

    emit_local_op(compiler->chunk, WYRM_OP_GET_LOCAL, (uint16_t)slot);
}

void compile_assignment(ast_node_t *node, compiler_t *compiler) {
    ast_assignment_t assign = node->ast_node_value.assignment_expr;
    int slot = resolve_local(compiler, assign.name);
    bool is_global = false;

    if (slot == ERRRESOLVER) {
        slot = resolve_global(compiler, assign.name);
        is_global = true;
    }

    if (slot == ERRRESOLVER) {
        panic(ERR_UNDEFINED_VAR, "Error UNDEFINED_VAR in assignment: %s", assign.name);
    }

    if (assign.index != NULL) {
        wyrm_opcode_t get_op = is_global ? WYRM_OP_GET_GLOBAL : WYRM_OP_GET_LOCAL;
        emit_local_op(compiler->chunk, get_op, (uint16_t)slot);
        compile_node(assign.index, compiler);
        compile_node(assign.value, compiler);
        chunk_write_byte(compiler->chunk, WYRM_OP_VEC_SET);
        return;
    }

    compile_node(assign.value, compiler);
    wyrm_opcode_t set_op = is_global ? WYRM_OP_SET_GLOBAL : WYRM_OP_SET_LOCAL;
    emit_local_op(compiler->chunk, set_op, (uint16_t)slot);
}

void compile_vec_literal(ast_node_t *node, compiler_t *compiler) {
    ast_block_t block = node->ast_node_value.block_expr;
    for (size_t i = 0; i < block.size; i++) {
        compile_node(block.nodes[i], compiler);
    }
    chunk_write_byte(compiler->chunk, WYRM_OP_BUILD_VEC);
    chunk_write_uint16(compiler->chunk, (uint16_t)block.size);
}

void compile_vec_access(ast_node_t *node, compiler_t *compiler) {
    ast_binary_expr_t bin = node->ast_node_value.binary_expr;
    compile_node(bin.left, compiler);
    compile_node(bin.right, compiler);
    chunk_write_byte(compiler->chunk, WYRM_OP_VEC_GET);
}