#pragma once
#include "parser.h"

/*
Archivo donde se declara la función factor, son las opreaciones binarias con gerarquia (* / %)
Wyrm factor =  unary ( ( T_MUL | T_DIV | T_MOD ) unary
*/

// cumple con factor -> unary ( ( T_MUL | T_DIV | T_MOD ) unary )
ast_node_t *factor(parser_t *parser);