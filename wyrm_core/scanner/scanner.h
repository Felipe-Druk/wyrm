#pragma once

#include "../utils/token_vector.h"s

/*
El sacanner deberi a groso modo agarrar un string y devolver tokens.
Tambien es el responsable de determinar como funciona la sintaxis del lenguaje.
*/
typedef struct {
    token_vector_t* tokens;
    int flags;
} Scanner;

typedef Scanner scanner_t;


// Funcion principal, tomar el texto plano y costruye el arreglo de tokens.
token_vector_t* scanner_scan(char* input);