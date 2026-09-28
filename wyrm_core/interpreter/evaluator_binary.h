#pragma once

#include "../ast_node.h"
#include "../wyrm_value.h"

/*
Archivo que resuelve las expresiones binarias
son muchas para listar y cada una tiene una interaction distinca segun los tipos, asi que no se listaran.
*/

// evalua el nodo siempre y cuando sea del tipo expresión binaria, luego devuelve el resultado
wyrm_value_t eval_binary(ast_node_t *node, wyrm_value_t left, wyrm_value_t right);