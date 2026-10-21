#include "while_statement.h"
#include "../utils/error.h"
#include "parser_internal.h"

ast_node_t *expression(parser_t *parser);
ast_node_t *expression_statement(parser_t *parser); // solo la firma

ast_node_t *while_statement(parser_t *parser) {
    if (!match(parser, T_LPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: Expected '(' after 'while'");
    }

    ast_node_t *condition = expression(parser);

    if (!match(parser, T_RPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: Expected ')' after condition");
    }

    ast_node_t *body = expression_statement(parser);

    return create_while_node(condition, body);
}