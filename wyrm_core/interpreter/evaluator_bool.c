#include "evaluator_bool.h"

wyrm_value_t eval_bool(ast_node_t *node) {
    wyrm_value_t result = {0};
    result.type = VAL_BOOL;
    result.value.bool_val = node->ast_node_value.bool_expr.value;
    return result;
}