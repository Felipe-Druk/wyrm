#pragma once
#include "../compiler_internal.h"

/*
Compila tdo ast_node relacionado con operaciones, binarias, unarias,etc
*/

// Compila expreciones binarias, cada operacion tinen su corespondiente opcode
void compile_binary_expr(ast_node_t *node, compiler_t *compiler);

// Compila expreciones binarias, cada operacion tinen su corespondiente opcode
void compile_unary_expr(ast_node_t *node, compiler_t *compiler);