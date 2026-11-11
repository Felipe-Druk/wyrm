#pragma once
#include "../compiler_internal.h"

/*
Compila tdo ast_node relacionado variables, tanto declaraciones como invocaciones
*/

// Compila nodos de tipo var_declaration, guarda una referencia de la variable local para el "scope"
void compile_var_declaration(ast_node_t *node, compiler_t *compiler);

// Resuelve nodos tipo identifier, emite un opcode que llama a la variable local
void compile_identifier(ast_node_t *node, compiler_t *compiler);

// Resuleve situaciones del tipo variable (= | <-) valor, compila llamando a la variable y asignando el valor del stack
void compile_assignment(ast_node_t *node, compiler_t *compiler);