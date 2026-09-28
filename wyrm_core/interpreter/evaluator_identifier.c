#include "evaluator_identifier.h"
#include "../utils/error.h"

wyrm_value_t eval_val_decl(ast_node_t *node, environment_t *env, wyrm_value_t expr_value) {
    wyrm_value_t assign_value = {.type = VAL_VOID};
    environment_set(env, node->ast_node_value.var_decl_expr.identifier, expr_value,
                    node->ast_node_value.var_decl_expr.var_type);
    if (node->ast_node_value.var_decl_expr.assign_op == T_RASSIGN) {
        return expr_value;
    }
    return assign_value;
}

wyrm_value_t eval_identifier(ast_node_t *node, environment_t *env) {
    wyrm_value_t *stored_value = environment_get(env, node->ast_node_value.identifier_expr.name);

    if (stored_value == NULL) {
        panic(ERR_UNDEFINED_VAR, "Error UNDEFINED_VAR: %s", node->ast_node_value.identifier_expr.name);
    }

    return *stored_value;
}