#pragma once

#include "../utils/token_vector.h"

/*
Este archivo se encargar de resolver los "identificadores", todo lo que no sea palabra reservada, operador o dato es identificador
*/


// Resive el input string, el vetor de tokens y el indicie atual
// devuleve el nuevo indice o -1 para error
int resolver_identidier(char* input, int index, token_vector_t* tokens);