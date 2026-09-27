#pragma once

#include <stdint.h>
#include <stdbool.h>

typedef enum {
    VAL_VOID, // cuando no hay retorno

    VAL_NAT8,  // nat8
    VAL_NAT16, // nat16
    VAL_NAT32, // nat32
    VAL_NAT64, // nat64

    VAL_INT8,  // int8
    VAL_INT16, // int16
    VAL_INT32, // int32
    VAL_INT64, // int64

    VAL_FLOAT32, // float32
    VAL_FLOAT64, // float64

} WyrmValueType;

// En general todo lo que pueda retornar un "source" de wyrm, inlcuso si no hay retorno.
typedef struct {
    WyrmValueType type;
    union {
        uint8_t nat8_val;
        uint16_t nat16_val;
        uint32_t nat32_val;
        uint64_t nat64_val;

        int8_t int8_val;
        int16_t int16_val;
        int32_t int32_val;
        int64_t int64_val;

        _Float32 float32_val;
        _Float64 float64_val;
    } value;
} WyrmValue;

typedef WyrmValue wyrm_value_t;

uint64_t get_cast_nat64(wyrm_value_t value);

int64_t get_cast_int64(wyrm_value_t value);

_Float64 get_cast_float64(wyrm_value_t value);

#define MAKE_WYRM_VAL(X)                                                                                               \
    _Generic((X),                                                                                                      \
        uint8_t: (wyrm_value_t){.type = VAL_NAT8, .value.nat8_val = (X)},                                              \
        uint16_t: (wyrm_value_t){.type = VAL_NAT16, .value.nat16_val = (X)},                                           \
        uint32_t: (wyrm_value_t){.type = VAL_NAT32, .value.nat32_val = (X)},                                           \
        int32_t: (wyrm_value_t){.type = VAL_INT32, .value.int32_val = (X)},                                            \
        int64_t: (wyrm_value_t){.type = VAL_INT64, .value.int64_val = (X)},                                            \
        uint64_t: (wyrm_value_t){.type = VAL_NAT64, .value.nat64_val = (X)},                                           \
        _Float64: (wyrm_value_t){.type = VAL_FLOAT64, .value.float64_val = (X)},                                       \
        default: (wyrm_value_t){.type = VAL_VOID})
