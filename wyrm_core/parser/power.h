#pragma once
#include "parser.h"

/*
Archivo donde se define la expresión de potencia, en cualqueier expresión se resuelven primero las potencias
Tambien se adminten cadenas de potencias (primary)
Wyrm pow = primary ( T_POW primary )
*/

// cumple con power -> primary ( T_POW primary )
ast_node_t *power(parser_t *parser);