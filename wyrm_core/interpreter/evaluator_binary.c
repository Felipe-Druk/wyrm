#include "evaluator_binary.h"
#include "../wyrm_token.h"
#include "../utils/error.h"
#include <math.h>

int is_float(WyrmValueType type) { return (type == VAL_FLOAT32) || (type == VAL_FLOAT64); }

int is_int(WyrmValueType type) { return (type >= VAL_INT8) && (type <= VAL_INT64); }

int is_zero(wyrm_value_t value) { return (get_cast_float64(value) == 0.0) || (get_cast_nat64(value) == 0); }

#define EXECUTE_MATH_OP(op)                                                                                            \
    do {                                                                                                               \
        if (has_float) {                                                                                               \
            return MAKE_WYRM_VAL(get_cast_float64(left) op get_cast_float64(right));                                   \
        } else if (has_int) {                                                                                          \
            return MAKE_WYRM_VAL(get_cast_int64(left) op get_cast_int64(right));                                       \
        } else {                                                                                                       \
            return MAKE_WYRM_VAL(get_cast_nat64(left) op get_cast_nat64(right));                                       \
        }                                                                                                              \
    } while (0)

wyrm_value_t eval_binary(ast_node_t *node, wyrm_value_t left, wyrm_value_t right) {
    if (node->type != AST_BINARY_EXPR) {
        wyrm_value_t error_val = {.type = VAL_VOID}; // TODO: Error
        return error_val;
    }

    int has_float = is_float(left.type) || is_float(right.type);
    int has_int = is_int(left.type) || is_int(right.type);

    switch (node->ast_node_value.binary_expr.operator) {
    case T_ADD:
        EXECUTE_MATH_OP(+);
        break;
    case T_SUB:
        EXECUTE_MATH_OP(-);
        break;
    case T_MUL:
        EXECUTE_MATH_OP(*);
        break;
    case T_DIV:
        if (is_zero(right)) {
            panic(ERR_DIV_BY_ZERO, "Dont divide by zero");
        }
        EXECUTE_MATH_OP(/);
        break;
    case T_MOD:
        if (is_zero(right)) {
            panic(ERR_DIV_BY_ZERO, "You cannot perform x '%' 0");
        }
        if (has_float) {
            return MAKE_WYRM_VAL((_Float64)fmod(get_cast_float64(left), get_cast_float64(right)));
        } else if (has_int) {
            return MAKE_WYRM_VAL(get_cast_int64(left) % get_cast_int64(right));
        } else {
            return MAKE_WYRM_VAL(get_cast_nat64(left) % get_cast_nat64(right));
        }
        break;
    default:
        panic(ERR_UNKNOWN_OP, "operator %s unknown", token_type_to_string(node->ast_node_value.binary_expr.operator));
        break;
    }
}
