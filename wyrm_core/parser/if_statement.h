#pragma once
#include "parser.h"

/*
Archivo donde se declara la función if_statement, que es responsable de analizar una declaración if en el lenguaje Wyrm
if_statement = "if" "(" expression ")" statement ( "else" statement )
*/

ast_node_t *if_statement(parser_t *parser);
