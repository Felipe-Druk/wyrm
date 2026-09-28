#pragma once
#include "../ast_node.h"
#include "../wyrm_value.h"

/*
Archivo que resuelve un literal bool

Todo atado a futuras optimizaciones
*/

// evalua el nodo siempre y cuando sea de tipo number_literal
wyrm_value_t eval_bool(ast_node_t *node);