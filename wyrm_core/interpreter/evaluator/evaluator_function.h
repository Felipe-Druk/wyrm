#pragma once
#include "../wyrm_types/ast/ast_node.h"
#include "../wyrm_types/environment/environment.h"
#include "../wyrm_types/wyrm_value/wyrm_value.h"

/*
Archivo que resuelve todo lo refecnte a funciones, tanto declaración como el "llamado"
*/

// Guarda la declaración de la función
wyrm_value_t eval_function_decl(ast_node_t *node, environment_t *scope);

// Ejecuta la función aislando su scope
wyrm_value_t eval_call(ast_node_t *node, environment_t *scope);