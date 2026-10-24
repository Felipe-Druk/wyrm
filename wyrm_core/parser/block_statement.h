#pragma once
#include "parser.h"

/*
Archivo donde se declara la función block_statement, que es responsable de analizar un bloque de código en el lenguaje
Wyrm Un bloque de código está delimitado por llaves y puede contener múltiples declaraciones y expresiones
Wyrm block = { statement* }
*/

ast_node_t *block_statement(parser_t *parser);