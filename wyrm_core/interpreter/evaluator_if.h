#pragma once
#include "../ast_node.h"
#include "../wyrm_value.h"
#include "../environment.h"

/*
Archivo que resuelve los bloques if
enrruta sengun las condiciones
*/

// evalua el nodo siempre y cuando sea de tipo if
wyrm_value_t eval_if(ast_node_t *node, environment_t *env);