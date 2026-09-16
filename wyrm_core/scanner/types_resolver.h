#pragma once

#include "../utils/token_vector.h"

/*
Este archivo se encargar de resolver todos tipos de datos:
    - int8
    - int16
    - int32
    - int64
    
    - nat8
    - nat16
    - nat32
    - nat64


*/

// Resive el input string, el vetor de tokens y el indicie atual
// devuleve el nuevo indice o -1 para error
int resolver_types(char* input, int index, token_vector_t* tokens);