#include "wvm_operations.h"
#include <math.h>

static int is_float_type(wyrm_value_type_t t) { return t == VAL_FLOAT32 || t == VAL_FLOAT64; }

static int is_int_type(wyrm_value_type_t t) {
    return t == VAL_INT8 || t == VAL_INT16 || t == VAL_INT32 || t == VAL_INT64;
}

#define EXECUTE_WVM_MATH_OP(op)                                                                                        \
    do {                                                                                                               \
        if (has_float) {                                                                                               \
            res.type = VAL_FLOAT64;                                                                                    \
            res.value.float64_val = get_cast_float64(left) op get_cast_float64(right);                                 \
        } else if (has_int) {                                                                                          \
            res.type = VAL_INT64;                                                                                      \
            res.value.int64_val = get_cast_int64(left) op get_cast_int64(right);                                       \
        } else {                                                                                                       \
            res.type = VAL_NAT64;                                                                                      \
            res.value.nat64_val = get_cast_nat64(left) op get_cast_nat64(right);                                       \
        }                                                                                                              \
    } while (0)

void wvm_op_binary_arith(wyrm_vm_t *vm, wyrm_opcode_t op) {
    // Importante: en una VM de pila, el primer pop extrae el operando DERECHO
    wyrm_value_t right = vm_pop(vm);
    wyrm_value_t left = vm_pop(vm);

    int has_float = is_float_type(left.type) || is_float_type(right.type);
    int has_int = is_int_type(left.type) || is_int_type(right.type);
    wyrm_value_t res = {0};

    switch (op) {
    case WYRM_OP_ADD:
        EXECUTE_WVM_MATH_OP(+);
        break;
    case WYRM_OP_SUB:
        EXECUTE_WVM_MATH_OP(-);
        break;
    case WYRM_OP_MUL:
        EXECUTE_WVM_MATH_OP(*);
        break;
    case WYRM_OP_DIV:
        if ((has_float && get_cast_float64(right) == 0.0) || (!has_float && get_cast_nat64(right) == 0)) {
            panic(ERR_DIV_BY_ZERO, "Division by zero in VM");
        }
        EXECUTE_WVM_MATH_OP(/);
        break;
    case WYRM_OP_MOD:
        if ((has_float && get_cast_float64(right) == 0.0) || (!has_float && get_cast_nat64(right) == 0)) {
            panic(ERR_DIV_BY_ZERO, "Modulo by zero in VM");
        }
        if (has_float) {
            res.type = VAL_FLOAT64;
            res.value.float64_val = fmod(get_cast_float64(left), get_cast_float64(right));
        } else if (has_int) {
            res.type = VAL_INT64;
            res.value.int64_val = get_cast_int64(left) % get_cast_int64(right);
        } else {
            res.type = VAL_NAT64;
            res.value.nat64_val = get_cast_nat64(left) % get_cast_nat64(right);
        }
        break;
    default:
        panic(ERR_UNKNOWN_OP, "Invalid arithmetic opcode: %d", op);
    }

    vm_push(vm, res);
}