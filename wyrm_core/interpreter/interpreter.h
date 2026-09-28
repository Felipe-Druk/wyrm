#pragma once

#include "../ast_node.h"
#include "../wyrm_value.h"
#include "../environment.h"

typedef struct {
    ast_node_t *root;
    environment_t *scope;
    int flags;
} Interpreter;

typedef Interpreter interpreter_t;

// Crea el interpreter del un nodo ast raiz, si falla devuelve un NULL pointer
interpreter_t *create_interpreter(ast_node_t *root, environment_t *scope);

// evalua toda el ast y retorna el resultado
wyrm_value_t interpreter_interpret(interpreter_t *interpreter);
