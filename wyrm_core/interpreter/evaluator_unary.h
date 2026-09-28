#pragma once

#include "../ast_node.h"
#include "../wyrm_value.h"

/*
Archivo que resuelve las expresiones unarias
Operadores como "-", "~", etc
*/

// evalua el nodo siempre y cuando sea del tipo expresión binaria, luego devuelve el resultado
wyrm_value_t eval_unary(ast_node_t *node, wyrm_value_t right);