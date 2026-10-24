#pragma once
#include "parser.h"

/*
Archivo donde se defienen las comparaciones, menos las de "igual" y "desigual", estas tienen "más gerarquia"
Wyrm comparison = term ( ( T_LESS | T_GREATER | T_LESS_EQUAL | T_GREATER_EQUAL ) term
*/

// cumple con comparison -> term ( ( T_LESS | T_GREATER | T_LESS_EQUAL | T_GREATER_EQUAL ) term )
ast_node_t *comparison(parser_t *parser);