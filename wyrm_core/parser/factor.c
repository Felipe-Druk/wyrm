#include "factor.h"
#include "parser_internal.h"
#include "unary.h"

ast_node_t *factor(parser_t *parser) {
    ast_node_t *expr = unary(parser);

    while (match(parser, T_MUL) || match(parser, T_DIV) || match(parser, T_MOD)) {
        wyrm_token_t *previous = get_token(parser->tokens, parser->current_index - 1);
        ast_node_t *right_expr = unary(parser);
        expr = create_binary_node(expr, previous->type, right_expr);
    }

    return expr;
}