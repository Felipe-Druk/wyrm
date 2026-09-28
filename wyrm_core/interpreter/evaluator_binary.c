#include "evaluator_binary.h"
#include "../wyrm_token.h"
#include "../utils/error.h"
#include <math.h>

#define EXECUTE_MATH_OP(op)                                                                                            \
    do {                                                                                                               \
        if (has_float) {                                                                                               \
            _Float64 cast_result = get_cast_float64(left) op get_cast_float64(right);                                  \
            return MAKE_WYRM_VAL(cast_result);                                                                         \
        } else if (has_int) {                                                                                          \
            int64_t cast_result = get_cast_int64(left) op get_cast_int64(right);                                       \
            return MAKE_WYRM_VAL(cast_result);                                                                         \
        } else {                                                                                                       \
            uint64_t cast_result = get_cast_nat64(left) op get_cast_nat64(right);                                      \
            return MAKE_WYRM_VAL(cast_result);                                                                         \
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
// macro para operadores logicos
#define EXECUTE_RELATIONAL_OP(op)                                                                                      \
    do {                                                                                                               \
        wyrm_value_t result;                                                                                           \
        result.type = VAL_BOOL;                                                                                        \
        if (has_float) {                                                                                               \
            result.value.bool_val = (get_cast_float64(left) op get_cast_float64(right));                               \
        } else if (has_int) {                                                                                          \
            result.value.bool_val = (get_cast_int64(left) op get_cast_int64(right));                                   \
        } else {                                                                                                       \
            result.value.bool_val = (get_cast_nat64(left) op get_cast_nat64(right));                                   \
        }                                                                                                              \
        return result;                                                                                                 \
    } while (0)

wyrm_value_t eval_logical(TokenType op, wyrm_value_t left, wyrm_value_t right) {
    if (left.type != VAL_BOOL || right.type != VAL_BOOL) {
        panic(ERR_TYPE_MISMATCH, "Logical operators can only be applied to booleans");
    }

    wyrm_value_t result;
    result.type = VAL_BOOL;
    bool bool_l = left.value.bool_val;
    bool bool_r = right.value.bool_val;

    if (op == T_AND)
        result.value.bool_val = bool_l && bool_r;
    else if (op == T_OR)
        result.value.bool_val = bool_l || bool_r;
    else if (op == T_XOR)
        result.value.bool_val = bool_l != bool_r;

    return result;
}

wyrm_value_t eval_equality(wyrm_value_t left, wyrm_value_t right, bool is_eq) {
    wyrm_value_t result = {0};
    result.type = VAL_BOOL;

    if (left.type == VAL_BOOL && right.type == VAL_BOOL) {
        bool eq = left.value.bool_val == right.value.bool_val;
        result.value.bool_val = is_eq ? eq : !eq;
        return result;
    }

    if (left.type == VAL_BOOL || right.type == VAL_BOOL) { // si alguno no es bool devolvemos false
        result.value.bool_val = is_eq ? false : true;
        return result;
    }

    int has_float = is_float(left.type) || is_float(right.type);
    int has_int = is_int(left.type) || is_int(right.type);
    bool eq;

    if (has_float)
        eq = get_cast_float64(left) == get_cast_float64(right);
    else if (has_int)
        eq = get_cast_int64(left) == get_cast_int64(right);
    else
        eq = get_cast_nat64(left) == get_cast_nat64(right);

    result.value.bool_val = is_eq ? eq : !eq;
    return result;
}

wyrm_value_t eval_binary(ast_node_t *node, wyrm_value_t left, wyrm_value_t right) {
    if (node->type != AST_BINARY_EXPR) {
        panic(ERR_UNKNOWN_OP, "Missing expresión");
    }

    int has_float = is_float(left.type) || is_float(right.type);
    int has_int = is_int(left.type) || is_int(right.type);

    switch (node->ast_node_value.binary_expr.operator) {
    // Arigmeticos
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
        // Logicos

    case T_LESS:
        EXECUTE_RELATIONAL_OP(<);
        break;
    case T_GREATER:
        EXECUTE_RELATIONAL_OP(>);
        break;
    case T_LESS_EQUAL:
        EXECUTE_RELATIONAL_OP(<=);
        break;
    case T_GREATER_EQUAL:
        EXECUTE_RELATIONAL_OP(>=);
        break;
    case T_EQUAL:
        return eval_equality(left, right, true);
    case T_NOT_EQUAL:
        return eval_equality(left, right, false);
    case T_AND:
    case T_OR:
    case T_XOR:
        return eval_logical(node->ast_node_value.binary_expr.operator, left, right);

    default:
        panic(ERR_UNKNOWN_OP, "Operator %s unknown", token_type_to_string(node->ast_node_value.binary_expr.operator));
        break;
    }
}
