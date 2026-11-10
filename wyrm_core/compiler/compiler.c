#include "compiler_internal.h"
#include "compiler_ast/compiler_variables.h"
#include "compiler_ast/compiler_operations.h"
#include "../utils/debugger.h"
#include <stdlib.h>

#define INITIALLOCALS 5

compiler_t *create_compiler(ast_node_t *root_ast, int flags) {
    compiler_t *compiler = malloc(sizeof(compiler_t));
    if (compiler == NULL) {
        panic(ERR_OUT_MEMORY, "Could not allocate memory for compiler");
    }
    compiler->root_ast = root_ast;
    compiler->chunk = NULL;
    compiler->locals = create_vector(sizeof(wyrm_local_t), INITIALLOCALS);
    compiler->scope_level = 0;
    compiler->flags = flags;
    return compiler;
}

void compile_node(ast_node_t *node, compiler_t *compiler) {
    if (node == NULL || compiler == NULL || compiler->chunk == NULL) {
        panic(ERR_NULL_POINTER, "Node or chunk is NULL");
    }
    switch (node->type) {
    case AST_PROGRAM:
    case AST_BLOCK: {
        ast_block_t block = node->ast_node_value.block_expr;
        for (size_t i = 0; i < block.size; i++) {
            compile_node(block.nodes[i], compiler);
        }
        break;
    }

    case AST_NUMBER_LITERAL: {
        int64_t val_num = strtoll(node->ast_node_value.number_expr.value, NULL, 10);
        wyrm_value_t constant = {0};
        constant.type = VAL_INT64; // tipo default
        constant.value.int64_val = val_num;
        uint16_t const_idx = chunk_add_constant(compiler->chunk, constant);
        chunk_write_byte(compiler->chunk, WYRM_OP_CONSTANT);
        chunk_write_uint16(compiler->chunk, const_idx);
        break;
    }

    case AST_VAR_DECLARATION: {
        compile_var_declaration(node, compiler);
        break;
    }
    case AST_IDENTIFIER: {
        compile_identifier(node, compiler);
        break;
    }
    case AST_BINARY_EXPR: {
        compile_binary_expr(node, compiler);
        break;
    }
    default:
        panic(ERR_UNKNOWN_OP, "Unknown AST node type: %d", node->type);
    }
}

int resolve_local(compiler_t *compiler, const char *name) {
    wyrm_local_t *locals_arr = (wyrm_local_t *)compiler->locals.data;
    for (int i = (int)compiler->locals.size - 1; i >= 0; i--) {
        if (strcmp(locals_arr[i].name, name) == 0) {
            return i;
        }
    }
    return -1;
}

void *compile_ast(compiler_t *compiler) {
    if (compiler == NULL || compiler->root_ast == NULL) {
        panic(ERR_NULL_POINTER, "Compiler or root AST is NULL");
    }
    if (compiler->flags) {
        print_line_debug("COMPILE AST....");
        print_jump_line();
    }
    compiler->chunk = create_wyrm_chunk();
    compile_node(compiler->root_ast, compiler);
    chunk_write_byte(compiler->chunk, WYRM_OP_RETURN);
    return compiler->chunk;
}

wyrm_chunk_t *get_compiled_chunk(compiler_t *compiler) {
    if (compiler == NULL) {
        panic(ERR_NULL_POINTER, "Compiler is NULL");
    }

    return compiler->chunk;
}

void destroy_compiler(compiler_t *compiler) {
    if (compiler == NULL) {
        return;
    }
    destroy_ast_node(compiler->root_ast);
    if (compiler->chunk) {
        destroy_wyrm_chunk(compiler->chunk);
    }
    destroy_vector(&compiler->locals);
    free(compiler);
    compiler = NULL;
}