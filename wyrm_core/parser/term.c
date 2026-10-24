#include "term.h"
#include "parser_internal.h"
#include "factor.h"

ast_node_t *term(parser_t *parser) {
    ast_node_t *expr = factor(parser);

    while (match(parser, T_ADD) || match(parser, T_SUB)) {
        wyrm_token_t *previous = get_token(parser->tokens, parser->current_index - 1);
        ast_node_t *right_expr = factor(parser);
        expr = create_binary_node(expr, previous->type, right_expr);
    }
    return expr;
}