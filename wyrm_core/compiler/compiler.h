#pragma once
#include "../wyrm_types/ast/ast_node.h"
#include "../wyrm_types/wyrmc_types/wyrm_chunk.h"

typedef struct Compiler {
    ast_node_t *root_ast;
    int flags;
} compiler_t;

// Crea el compilador con el AST raíz y las banderas de compilación
compiler_t *create_compiler(ast_node_t *root_ast, int flags);

// Compila el archivo a partir del nodo raíz y devuelve un chunk de código
wyrm_chunk_t *compile_ast(compiler_t *compiler);

// Libera la memoria del compilador, tambien elimina el AST raíz
void destroy_compiler(compiler_t *compiler);