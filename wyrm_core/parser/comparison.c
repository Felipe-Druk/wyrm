#include "comparison.h"
#include "parser_internal.h"
#include "term.h"

ast_node_t *comparison(parser_t *parser) {
    ast_node_t *expr = term(parser);

    while (match(parser, T_LESS) || match(parser, T_GREATER) || match(parser, T_LESS_EQUAL) ||
           match(parser, T_GREATER_EQUAL)) {
        TokenType operator = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = term(parser);
        expr = create_binary_node(expr, operator, right_expr);
    }
    return expr;
}