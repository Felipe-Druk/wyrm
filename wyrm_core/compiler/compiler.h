#pragma once
#include "../wyrm_types/ast/ast_node.h"
#include "../wyrm_types/wyrmc_types/wyrm_chunk.h"

// estructura que represenata una variable local (en tiempo de compilación), se podria aprovechar para detectar
// funciones no usadas
typedef struct WyrmLocal {
    char *name;
    wyrm_value_type_t var_type;
    int scope_level;
} wyrm_local_t;

typedef struct Compiler {
    ast_node_t *root_ast;
    wyrm_chunk_t *chunk;
    wyrm_vector_t locals;
    int scope_level;
    int flags;
} compiler_t;

// Crea el compilador con el AST raíz y las banderas de compilación
compiler_t *create_compiler(ast_node_t *root_ast, int flags);

// Compila el archivo a partir del nodo raíz y devuelve un chunk de código
void *compile_ast(compiler_t *compiler);

wyrm_chunk_t *get_compiled_chunk(compiler_t *compiler);

// Libera la memoria del compilador, tambien elimina el AST raíz
void destroy_compiler(compiler_t *compiler);