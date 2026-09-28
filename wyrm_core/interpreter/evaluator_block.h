#pragma once

#include "../ast_node.h"
#include "../wyrm_value.h"
#include "../environment.h"

/*
Archivo que resuelve los bloques wyrm, resuelve el resultado de todo el bloque y sub-bloques
*/

// evalua el nodo siempre y cuando sea del tipo expresión binaria, luego devuelve el resultado
wyrm_value_t eval_block(ast_node_t *node, environment_t *parent_scope);