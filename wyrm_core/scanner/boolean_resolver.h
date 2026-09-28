#pragma once

#include "../utils/token_vector.h"

/*
Este archivo se encargar de resolver las palabras booleanas:
    - true
    - false
    - and
    - or
    - not
    - xor


*/

// Resive el input string, el vector de tokens y el indicie actual
// devuleve el nuevo indice o -1 para error
int resolver_boolean(char *input, int index, token_vector_t *tokens);