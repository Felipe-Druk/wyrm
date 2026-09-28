#pragma once
#include "../ast_node.h"
#include "../wyrm_value.h"
#include "../environment.h"

/*
Archivo que resuelve los loop como while y for
*/

// evalua el nodo siempre y cuando sea de tipo while
wyrm_value_t eval_loop(ast_node_t *node, environment_t *env);