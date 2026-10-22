#pragma once
#include "parser.h"

/*
Archivo donde se define la "igualdad", es de gerarquia menor a los de comparacion, tenmos igual "==" y no igual "~="
Wyem equality = comparison ( ( T_EQUAL | T_NOT_EQUAL ) comparison
*/

// cumple con equality -> comparison ( ( T_EQUAL | T_NOT_EQUAL ) comparison )
ast_node_t *equality(parser_t *parser);