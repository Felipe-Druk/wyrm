#pragma once
#include "parser.h"

/*
Archivo de cabecera para la declaración de variables.
Wyrm variable_declaration = "TIPO" T_IDENTIFIER (T_ASSIGN | T_LARROW) expression T_SEMICOLON
*/

// cumple con var_decl -> "TIPO" T_IDENTIFIER (T_ASSIGN | T_LARROW) expression T_SEMICOLON
ast_node_t *variable_declaration(parser_t *parser);