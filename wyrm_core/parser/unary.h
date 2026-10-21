#pragma once
#include "parser.h"

/*
Archivo de cabecera para la expresión unary, es un operador que solo actua sobre una expresión a su derecha
Wyrm unary = ( T_SUB | T_NOT) power
*/

//  cumple con unary -> ( T_SUB | T_NOT) power
ast_node_t *unary(parser_t *parser);