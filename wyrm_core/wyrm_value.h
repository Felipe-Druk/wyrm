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
    } value;
} WyrmValue;

typedef WyrmValue wyrm_value_t;