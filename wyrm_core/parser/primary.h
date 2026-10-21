#include "parser.h"

/*
Archivo de cabecera para la expresión primaria. Primary simpre el el ultimo eslabon de la cadena, todo el parser es un
gran "CHAIN OF RESPONSIBILITY" No puede haber nada mas abajo de primary,
Wyrm primary = T_NUMBER | T_STRING |T_IDENTIFIER | T_LPAREN expression T_RPAREN
*/

// cumple con primary ->  T_NUMBER | T_FLOAT_NUMBER | T_IDENTIFIER | T_LPAREN expression T_RPAREN
ast_node_t *primary(parser_t *parser);