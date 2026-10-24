#pragma once
#include "parser.h"

/*
Archivo donde se declara la función function_declaration, que es responsable de analizar una declaración de función en
el lenguaje Wyrm function_declaration = ("TIPO") -> TIPO T_IDENTIFIER = (T_IDENTIFIER) -> { statement* }
*/

// para procersar de emanera correcta la función, el primer `(` debe ser consumido antes de llamar a esta función
ast_node_t *function_declaration(parser_t *parser);