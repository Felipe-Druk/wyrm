#pragma once
#include "../compiler_internal.h"

// Compila nodos del tipo while,
void compile_while_expr(ast_node_t *node, compiler_t *compiler);

// Compila nodos de tipo if, tiene en cuenta si hay else if o else al final
void compile_if_expr(ast_node_t *node, compiler_t *compiler);