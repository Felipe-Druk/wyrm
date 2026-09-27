#include "evaluator_binary.h"

#include "../wyrm_token.h"

int is_float(WyrmValueType type) { return (type == VAL_FLOAT32) || (type == VAL_FLOAT64); }

int is_int(WyrmValueType type) { return (type >= VAL_INT8) && (type <= VAL_INT64); }

wyrm_value_t eval_add(wyrm_value_t a, wyrm_value_t b, int has_float, int has_int) {
    if (has_float) {
        _Float64 result = get_cast_float64(a) + get_cast_float64(b);
        return MAKE_WYRM_VAL(result);
    } else if (has_int) {
        int64_t result = get_cast_int64(a) + get_cast_int64(b);
        return MAKE_WYRM_VAL(result);
    } else {
        uint64_t result = get_cast_nat64(a) + get_cast_nat64(b);
        return MAKE_WYRM_VAL(result);
    }
}

wyrm_value_t eval_binary(ast_node_t *node, wyrm_value_t left, wyrm_value_t right) {
    if (node->type != AST_BINARY_EXPR) {
        wyrm_value_t error_val = {.type = VAL_VOID}; // TODO: Error
        return error_val;
    }

    int has_float = is_float(left.type) || is_float(right.type);
    int has_int = is_int(left.type) || is_int(right.type);

    switch (node->ast_node_value.binary_expr.operator) {
    case T_ADD:
        return eval_add(left, right, has_float, has_int);

    default:
        wyrm_value_t error_val = {.type = VAL_VOID};
        return error_val;
    }
}
