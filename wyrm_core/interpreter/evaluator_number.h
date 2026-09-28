#pragma once
#include "../ast_node.h"
#include "../wyrm_value.h"

/*
Archivo que resuelve un literal number, siguiendo las reglas
Todo numero menor a (2**63) -1 es un int64, de lo contrario se tomara como nat64
Si el numero es flotante sera un float64

Todo atado a futuras optimizaciones
*/

// evalua el nodo siempre y cuando sea de tipo number_literal
wyrm_value_t eval_number(ast_node_t *node);