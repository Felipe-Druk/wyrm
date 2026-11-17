#pragma once
#include "../parser.h"

/*
Este archivo declara la función while_statement, que es responsable de analizar una declaración while en el lenguaje
Wyrm while_statement = "while" "(" expression ")" { statement* }
*/

ast_node_t *while_statement(parser_t *parser);

// internamente es lo mismo que un while, ata su ejecucion auna "coleccion"
ast_node_t *for_statement(parser_t *parser);
