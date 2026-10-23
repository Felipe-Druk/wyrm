#include "equality.h"
#include "parser_internal.h"
#include "comparison.h"

ast_node_t *equality(parser_t *parser) {
    ast_node_t *expr = comparison(parser);

    while (match(parser, T_EQUAL) || match(parser, T_NOT_EQUAL)) {
        token_type_t operator = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = comparison(parser);
        expr = create_binary_node(expr, operator, right_expr);
    }
    return expr;
}