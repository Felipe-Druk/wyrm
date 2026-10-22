#include "logical_and.h"
#include "parser_internal.h"
#include "equality.h"

ast_node_t *logical_and(parser_t *parser) {
    ast_node_t *expr = equality(parser);

    while (match(parser, T_AND)) {
        TokenType operator = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = equality(parser);
        expr = create_binary_node(expr, operator, right_expr);
    }
    return expr;
}