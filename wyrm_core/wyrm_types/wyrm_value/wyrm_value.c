#include "wyrm_value.h"
#include "../../utils/error.h"
#include <stdlib.h>

static wyrm_vec_t *allocated_vectors_head = NULL; // singlenton de vectors, para elminar memoria

uint64_t get_cast_nat64(wyrm_value_t value) {
    switch (value.type) {
    case VAL_NAT8:
        return (uint64_t)value.value.nat8_val;
    case VAL_NAT16:
        return (uint64_t)value.value.nat16_val;
    case VAL_NAT32:
        return (uint64_t)value.value.nat32_val;
    case VAL_NAT64:
        return value.value.nat64_val;

    case VAL_INT8:
        return (uint64_t)value.value.int8_val;
    case VAL_INT16:
        return (uint64_t)value.value.int16_val;
    case VAL_INT32:
        return (uint64_t)value.value.int32_val;
    case VAL_INT64:
        return (uint64_t)value.value.int64_val;

    case VAL_FLOAT32:
        return (uint64_t)value.value.float32_val;
    case VAL_FLOAT64:
        return (uint64_t)value.value.float64_val;

    default:
        return 0.0;
    }
}

int64_t get_cast_int64(wyrm_value_t value) {
    switch (value.type) {
    case VAL_NAT8:
        return (int64_t)value.value.nat8_val;
    case VAL_NAT16:
        return (int64_t)value.value.nat16_val;
    case VAL_NAT32:
        return (int64_t)value.value.nat32_val;
    case VAL_NAT64:
        return (int64_t)value.value.nat64_val;

    case VAL_INT8:
        return (int64_t)value.value.int8_val;
    case VAL_INT16:
        return (int64_t)value.value.int16_val;
    case VAL_INT32:
        return (int64_t)value.value.int32_val;
    case VAL_INT64:
        return value.value.int64_val;

    case VAL_FLOAT32:
        return (int64_t)value.value.float32_val;
    case VAL_FLOAT64:
        return (int64_t)value.value.float64_val;

    default:
        return 0;
    }
}

_Float64 get_cast_float64(wyrm_value_t value) {
    switch (value.type) {
    case VAL_NAT8:
        return (_Float64)value.value.nat8_val;
    case VAL_NAT16:
        return (_Float64)value.value.nat16_val;
    case VAL_NAT32:
        return (_Float64)value.value.nat32_val;
    case VAL_NAT64:
        return (_Float64)value.value.nat64_val;

    case VAL_INT8:
        return (_Float64)value.value.int8_val;
    case VAL_INT16:
        return (_Float64)value.value.int16_val;
    case VAL_INT32:
        return (_Float64)value.value.int32_val;
    case VAL_INT64:
        return value.value.int64_val;

    case VAL_FLOAT32:
        return (_Float64)value.value.float32_val;
    case VAL_FLOAT64:
        return value.value.float64_val;

    default:
        return 0;
    }
}

wyrm_value_type_t token_to_value_type(TokenType token_type) {
    switch (token_type) {
    case T_INT8:
        return VAL_INT8;
    case T_INT16:
        return VAL_INT16;
    case T_INT32:
        return VAL_INT32;
    case T_INT64:
        return VAL_INT64;
    case T_NAT8:
        return VAL_NAT8;
    case T_NAT16:
        return VAL_NAT16;
    case T_NAT32:
        return VAL_NAT32;
    case T_NAT64:
        return VAL_NAT64;
    case T_FLOAT32:
        return VAL_FLOAT32;
    case T_FLOAT64:
        return VAL_FLOAT64;
    case T_BOOL:
        return VAL_BOOL;
    case T_VEC:
        return VAL_VEC;
    default:
        return VAL_VOID;
    }
}

#define CAST_FROM_INT64(enum_type, field_name, c_type)                                                                 \
    result.type = enum_type;                                                                                           \
    result.value.field_name = (c_type)original.value.int64_val;                                                        \
    break;

#define CAST_FROM_FLOAT64(enum_type, field_name, c_type)                                                               \
    result.type = enum_type;                                                                                           \
    result.value.field_name = (c_type)original.value.float64_val;                                                      \
    break;

wyrm_value_t cast_to_value_type(wyrm_value_t original, wyrm_value_type_t token_type) {
    if (original.type == token_type || original.type == VAL_VOID) {
        return original;
    }

    wyrm_value_t result = {0};
    result.type = token_type;

    switch (token_type) {
    case VAL_INT8:
        result.value.int8_val = (int8_t)get_cast_int64(original);
        break;
    case VAL_INT16:
        result.value.int16_val = (int16_t)get_cast_int64(original);
        break;
    case VAL_INT32:
        result.value.int32_val = (int32_t)get_cast_int64(original);
        break;
    case VAL_INT64:
        result.value.int64_val = (int64_t)get_cast_int64(original);
        break;
    case VAL_NAT8:
        result.value.nat8_val = (uint8_t)get_cast_nat64(original);
        break;
    case VAL_NAT16:
        result.value.nat16_val = (uint16_t)get_cast_nat64(original);
        break;
    case VAL_NAT32:
        result.value.nat32_val = (uint32_t)get_cast_nat64(original);
        break;
    case VAL_NAT64:
        result.value.nat64_val = (uint64_t)get_cast_nat64(original);
        break;
    case VAL_FLOAT32:
        result.value.float32_val = (_Float32)get_cast_float64(original);
        break;
    case VAL_FLOAT64:
        result.value.float64_val = (_Float64)get_cast_float64(original);
        break;
    case VAL_BOOL:
        result.value.bool_val = original.value.bool_val;
        break;
    default:
        return original;
    }

    return result;
}

wyrm_value_t cast_value(wyrm_value_t original, token_type_t target_type) {
    return cast_to_value_type(original, token_to_value_type(target_type));
}

int is_float(wyrm_value_type_t type) { return (type == VAL_FLOAT32) || (type == VAL_FLOAT64); }

int is_int(wyrm_value_type_t type) { return (type >= VAL_INT8) && (type <= VAL_INT64); }

int is_zero(wyrm_value_t value) { return (get_cast_float64(value) == 0.0) || (get_cast_nat64(value) == 0); }

wyrm_vec_t *create_wyrm_vec(wyrm_value_type_t element_type, size_t initial_capacity) {
    wyrm_vec_t *vec = calloc(1, sizeof(wyrm_vec_t));
    if (!vec) {
        panic(ERR_OUT_MEMORY, "Could not allocate wyrm_vec_t");
    }
    vec->element_type = element_type;
    vec->items = create_vector(sizeof(wyrm_value_t), initial_capacity > 0 ? initial_capacity : 4);

    // Igual que la vm de java este metodo previene que se quede memoria sin liberar
    vec->next_allocated = allocated_vectors_head;
    allocated_vectors_head = vec;
    return vec;
}

void free_all_wyrm_vecs() {
    wyrm_vec_t *current = allocated_vectors_head;
    while (current != NULL) {
        wyrm_vec_t *next = current->next_allocated;
        destroy_vector(&current->items);
        free(current);
        current = next;
    }
    allocated_vectors_head = NULL;
}