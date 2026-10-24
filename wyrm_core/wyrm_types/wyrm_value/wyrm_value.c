#include "wyrm_value.h"

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
    wyrm_value_t result = original;

    if (original.type == VAL_INT64) {
        switch (token_type) {
        case VAL_INT8:
            CAST_FROM_INT64(VAL_INT8, int8_val, int8_t)
        case VAL_INT16:
            CAST_FROM_INT64(VAL_INT16, int16_val, int16_t)
        case VAL_INT32:
            CAST_FROM_INT64(VAL_INT32, int32_val, int32_t)
        case VAL_NAT8:
            CAST_FROM_INT64(VAL_NAT8, nat8_val, uint8_t)
        case VAL_NAT16:
            CAST_FROM_INT64(VAL_NAT16, nat16_val, uint16_t)
        case VAL_NAT32:
            CAST_FROM_INT64(VAL_NAT32, nat32_val, uint32_t)
        case VAL_NAT64:
            CAST_FROM_INT64(VAL_NAT64, nat64_val, uint64_t)
        case VAL_FLOAT32:
            CAST_FROM_INT64(VAL_FLOAT32, float32_val, _Float32)
        case VAL_FLOAT64:
            CAST_FROM_INT64(VAL_FLOAT64, float64_val, _Float64)
        default:
            break;
        }
    } else if (original.type == VAL_FLOAT64) {
        switch (token_type) {
        case VAL_INT8:
            CAST_FROM_FLOAT64(VAL_INT8, int8_val, int8_t)
        case VAL_INT16:
            CAST_FROM_FLOAT64(VAL_INT16, int16_val, int16_t)
        case VAL_INT32:
            CAST_FROM_FLOAT64(VAL_INT32, int32_val, int32_t)
        case VAL_NAT8:
            CAST_FROM_FLOAT64(VAL_NAT8, nat8_val, uint8_t)
        case VAL_NAT16:
            CAST_FROM_FLOAT64(VAL_NAT16, nat16_val, uint16_t)
        case VAL_NAT32:
            CAST_FROM_FLOAT64(VAL_NAT32, nat32_val, uint32_t)
        case VAL_NAT64:
            CAST_FROM_FLOAT64(VAL_NAT64, nat64_val, uint64_t)
        case VAL_FLOAT32:
            CAST_FROM_FLOAT64(VAL_FLOAT32, float32_val, _Float32)
        default:
            break;
        }
    }
    return result;
}

wyrm_value_t cast_value(wyrm_value_t original, token_type_t target_type) {
    return cast_to_value_type(original, token_to_value_type(target_type));
}

int is_float(wyrm_value_type_t type) { return (type == VAL_FLOAT32) || (type == VAL_FLOAT64); }

int is_int(wyrm_value_type_t type) { return (type >= VAL_INT8) && (type <= VAL_INT64); }

int is_zero(wyrm_value_t value) { return (get_cast_float64(value) == 0.0) || (get_cast_nat64(value) == 0); }