#include "while_statement.h"
#include "block_statement.h"
#include "../utils/error.h"
#include "../parser_internal.h"

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

ast_node_t *for_statement(parser_t *parser) {
    if (!match(parser, T_LPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: expected '(' after 'for'");
    }
    ast_node_t *iter_expr = expression_statement(parser);
    if (!match(parser, T_RPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: expected ')' after for header");
    }
    if (!match(parser, T_LBRACE)) {
        panic(ERR_SYNTAX, "Syntax error: expected '{' to start for body");
    }
    ast_node_t *body = block_statement(parser);
    ast_node_t *for_node = create_while_node(iter_expr, body); // while node
    for_node->type = AST_FOR_EXPR;
    return for_node;
}
