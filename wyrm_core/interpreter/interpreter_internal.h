#pragma once
#include "interpreter.h"

/*
Queremos "esconder" la función que evalua un nodo puntual, solo es necesario para algunos "evaluadores"
*/

// evalua el nodo puntual
wyrm_value_t eval_node(ast_node_t *node, environment_t *scope);