#pragma once
#include "../compiler_internal.h"

// Guarda una nueva función con un nuevo id
void compile_function_decl(ast_node_t *node, compiler_t *compiler);

// Compila nodos de tipo llamada "call", buscara el id de la función (si existe) y se escribira una operacion para
// llamar a ese id
void compile_call_expr(ast_node_t *node, compiler_t *compiler);