#pragma once
#include "parser.h"

/*
Archivo donde se declara "logica or xor", son las operaciones binarias de maxima gerarquia
Existe sus equivalentes "bit a bit" pero no estan implementados
Wyrm logical and -> equality ( T_OR equality
*/

// cumple con logical_or_xor -> logical_and ( ( T_OR | T_XOR ) logical_and )
ast_node_t *logical_or_xor(parser_t *parser);