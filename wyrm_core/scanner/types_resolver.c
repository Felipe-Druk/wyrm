#include "types_resolver.h"
#include <string.h>
#include <ctype.h>

// este vector contendra todos los tipos de datos que el scanner puede reconocer
const char* types[] = {
    // Enteros
    "int8",
    "int16",
    "int32",
    "int64",

    // Naturales
    "nat8",
    "nat16",
    "nat32",
    "nat64",

    NULL // marca el final del vector
};

// vector para resolver los toknes, debe respetar el orden del vector de arriba
const TokenType t_types[] = {
    // Enteros
    T_INT8, 
    T_INT16,
    T_INT32,
    T_INT64,

    // Naturales
    T_NAT8, 
    T_NAT16, 
    T_NAT32, 
    T_NAT64

};

int charge_types(token_vector_t* tokens, size_t type_index) {
    wyrm_token_t type_token = { .type = t_types[type_index], .lexeme = types[type_index] };
    int result = push_token(tokens, type_token);
    return result;
}


int resolver_types(char* input, int index, token_vector_t* tokens){
    size_t i = 0;
    size_t actual_len = 0;
    char nex_char;
    while(types[i] != NULL){
        actual_len = strlen(types[i]);
        if(!strncmp(input + index, types[i], actual_len)){
            nex_char = input[index + actual_len];
            if(!isalnum(nex_char)){
                if (charge_types(tokens, i) != ERROR_TOKEN_VECTOR){
                    return index + actual_len;
                }
                return -1;

            }
        }
        i++;
    }
    return index;
}