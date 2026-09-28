#include "evaluator_if.h"
#include "interpreter.h"
#include "../utils/error.h"

wyrm_value_t eval_if(ast_node_t *node, environment_t *env) {
    wyrm_value_t condition_val = eval_node(node->ast_node_value.if_expr.condition, env);

    if (condition_val.type != VAL_BOOL) {
        panic(ERR_TYPE, "Type error: The 'if' condition must evaluate to a bool.");
    }

    if (condition_val.value.bool_val) {
        return eval_node(node->ast_node_value.if_expr.then_branch, env);
    } else if (node->ast_node_value.if_expr.else_branch != NULL) {
        return eval_node(node->ast_node_value.if_expr.else_branch, env);
    }

    wyrm_value_t void_val = {0};
    void_val.type = VAL_VOID;
    return void_val;
}