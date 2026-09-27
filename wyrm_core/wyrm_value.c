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