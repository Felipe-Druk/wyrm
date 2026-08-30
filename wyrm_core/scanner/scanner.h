#pragma once

#include "../wyrm_token.h"


/*
El sacanner deberi a groso modo agarrar un string y devolver tokens.
Tambien es el responsable de determinar como funciona la sintaxis del lenguaje.
*/



// Funcion principal, tomar el texto plano y costruye el arreglo de tokens.
wyrm_token_t* scanner_scan(char* input);