#pragma once
#include "parser.h"

/*
Archivo donde se declara la función "termino", son las opreaciones binarias de gerarquia media, incluye + y -
Dentro de las operaciones arigmeticas son las de menor gerarquia
Wyrm term =  factor ( ( T_ADD | T_SUB ) factor
*/

// cumple con term -> factor ( ( T_ADD | T_SUB ) factor )
ast_node_t *term(parser_t *parser);