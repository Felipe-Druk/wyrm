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

// no funciona la macro para el operador %
wyrm_value_t eval_mod(wyrm_value_t a, wyrm_value_t b, int has_float, int has_int) {
    if (has_float) {
        return MAKE_WYRM_VAL((_Float64)fmod(get_cast_float64(a), get_cast_float64(b)));
    } else if (has_int) {
        return MAKE_WYRM_VAL(get_cast_int64(a) % get_cast_int64(b));
    } else {
        return MAKE_WYRM_VAL(get_cast_nat64(a) % get_cast_nat64(b));
    }
}

// no funciona la macro para el operador ^,
wyrm_value_t eval_pow(wyrm_value_t a, wyrm_value_t b, int has_float, int has_int) {
    double result = pow(get_cast_float64(a), get_cast_float64(b)); // pow usa por defecto double
    if (has_float) {
        return MAKE_WYRM_VAL((_Float64)result);
    } else if (has_int) {
        return MAKE_WYRM_VAL((int64_t)result);
    } else {
        return MAKE_WYRM_VAL((uint64_t)result);
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
        return eval_mod(left, right, has_float, has_int);
    case T_POW:
        return eval_pow(left, right, has_float, has_int);
    default:
        panic(ERR_UNKNOWN_OP, "operator %s unknown", token_type_to_string(node->ast_node_value.binary_expr.operator));
        break;
    }
}
