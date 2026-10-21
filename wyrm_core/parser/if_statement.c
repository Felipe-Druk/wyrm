#include "if_statement.h"
#include "../utils/error.h"
#include "parser_internal.h"

ast_node_t *expression(parser_t *parser);
ast_node_t *expression_statement(parser_t *parser); // solo la firma

ast_node_t *if_statement(parser_t *parser) {
    if (!match(parser, T_LPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: Expected '(' after 'if'");
    }

    ast_node_t *condition = expression(parser);

    if (!match(parser, T_RPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: Expected ')' after condition");
    }

    ast_node_t *then_branch = expression_statement(parser);
    ast_node_t *else_branch = NULL;

    if (match(parser, T_ELSE)) {
        if (match(parser, T_IF)) {
            else_branch = if_statement(parser);
        } else {
            else_branch = expression_statement(parser);
        }
    }
    return create_if_node(condition, then_branch, else_branch);
}