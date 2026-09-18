#pragma once

#include "../utils/token_vector.h"

/*
Este archivo se encargar de resolver los limitadores como ";"
No tenemso ningun otro pero en caso de que exista, solo debemos agregarlo en este archivo.
*/


// Resive el input string, el vetor de tokens y el indicie atual
// devuleve el nuevo indice o -1 para error
int resolver_limiter(char* input, int index, token_vector_t* tokens);