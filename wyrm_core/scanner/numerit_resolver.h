#pragma once

#include "../utils/token_vector.h"

/*
Este archivo se encargar de resolver todos los numeros y separarlos entre numeros con "." y numeros sin "."
Aceptamos cualquier caracter con "1234567890."
*/



// carga el token de numero, devuelve el nuevo indice o -1 para error
int resolver_numerit(char* input, int index, token_vector_t* tokens);