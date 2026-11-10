#pragma once
#include "compiler.h"
#include "../utils/error.h"
#include "../wyrm_types/wyrmc_types/wyrm_opcode.h"
#include <string.h>

/*
Definimos internamente la fimra de "compile_node" para facilitar la recursion y esconder la interfaz de los modulos que
incluyan ""ompiler"
*/

// Compila el nodo segun su tipo,
void compile_node(ast_node_t *node, compiler_t *compiler);

// devuelve el indice de la variable, si esta en el "scope" local
int resolve_local(compiler_t *compiler, const char *name);