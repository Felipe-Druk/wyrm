#pragma once

#include "../ast_node.h"
#include "../wyrm_value.h"
#include "../environment.h"

/*
Archivo que resuelve la declaración de variables
*/

// evalua el nodo siempre y cuando sea del tipo val_decl
wyrm_value_t eval_val_decl(ast_node_t *node, environment_t *env, wyrm_value_t expr_value);

// evalua simpere y cuando sea un identifier, si esta definido devuelve el valor
wyrm_value_t eval_identifier(ast_node_t *node, environment_t *env);

// evalua simpere y cuando sea un assignment, reasigna el valor en la variable
wyrm_value_t eval_assignment(ast_node_t *node, environment_t *env, wyrm_value_t expr_value);