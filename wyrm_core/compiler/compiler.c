#include "compiler.h"
#include "../utils/error.h"
#include <stdlib.h>

compiler_t *create_compiler(ast_node_t *root_ast, int flags) {
    compiler_t *compiler = malloc(sizeof(compiler_t));
    if (compiler == NULL) {
        panic(ERR_OUT_MEMORY, "Could not allocate memory for compiler");
    }
    compiler->root_ast = root_ast;
    compiler->flags = flags;
    return compiler;
}

wyrm_chunk_t *compile_ast(compiler_t *compiler) {
    (void)compiler;
    wyrm_chunk_t *chunk = create_wyrm_chunk();
    // while (compiler->root_ast != NULL) y se compilara cada tipo de ast
    return chunk;
}

void destroy_compiler(compiler_t *compiler) {
    if (compiler == NULL) {
        return;
    }
    destroy_ast_node(compiler->root_ast);
    free(compiler);
    compiler = NULL;
}