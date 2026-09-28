#include "evaluator_unary.h"
#include "../utils/error.h"

wyrm_value_t eval_sub(wyrm_value_t value) {
    if (is_float(value.type)) {
        return MAKE_WYRM_VAL(-get_cast_float64(value));
    } else if (is_int(value.type)) {
        return MAKE_WYRM_VAL(-get_cast_int64(value));
    } else {
        panic(ERR_TYPE_MISMATCH, "You cannot negate a natural number (nat)");
    }
}

wyrm_value_t eval_not(wyrm_value_t value) {
    if (value.type != VAL_BOOL) {
        panic(ERR_TYPE_MISMATCH, "The 'not' operator can only be applied to booleans");
    }

    wyrm_value_t result;
    result.type = VAL_BOOL;
    result.value.bool_val = !value.value.bool_val;
    return result;
}

wyrm_value_t eval_unary(ast_node_t *node, wyrm_value_t right) {
    if (node->type != AST_UNARY_EXPR) {
        panic(ERR_UNKNOWN_OP, "Missing expresión");
    }

    switch (node->ast_node_value.unary_expr.operator) {
    case T_SUB:
        return eval_sub(right);
    case T_NOT:
        return eval_not(right);
    default:
        panic(ERR_UNKNOWN_OP, "Operator %s unknown", token_type_to_string(node->ast_node_value.unary_expr.operator));
    }
}