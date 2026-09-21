
#pragma once
#include "../utils/token_vector.h"


/*
Este archivo se encargara de resolver como optener los operadores logicos como
- "=="
- "~="
- ">="
- "<="
- "<"
- ">"

tambien tenemos que atajar casos especiales como "<-" y "="
*/

// Resive el input string, el vetor de tokens y el indicie atual
// devuleve el nuevo indice o -1 para error
int resolver_logical_operator(char* input, int index, token_vector_t* tokens);