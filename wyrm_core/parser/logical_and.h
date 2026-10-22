#pragma once
#include "parser.h"

/*
Archivo donde se declara "logica and", and es una compuerta logcia que basicamente es true solo si las dos expresiones
son true Existe otro "and" que es "bit a bit" que no esta implementado Wyrm logical and -> equality ( T_AND equality
*/

// cumple con logical_and -> equality ( T_AND equality )
ast_node_t *logical_and(parser_t *parser);