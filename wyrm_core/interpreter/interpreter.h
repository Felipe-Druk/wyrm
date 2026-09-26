#pragma once

#include "../ast_node.h"
#include "../wyrm_value.h"

typedef struct {
    ast_node_t *root;
    int flags;
} Interpreter;

typedef Interpreter interpreter_t;

// Crea el interpreter del un nodo ast raiz, si falla devuelve un NULL pointer
interpreter_t *create_interpreter(ast_node_t *root);

// evalua toda el ast y retorna el resultado
wyrm_value_t Interpreter_ineterpret(interpreter_t *interpreter);