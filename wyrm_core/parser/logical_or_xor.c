#include "logical_or_xor.h"
#include "parser_internal.h"
#include "logical_and.h"

ast_node_t *logical_or_xor(parser_t *parser) {
    ast_node_t *expr = logical_and(parser);

    while (match(parser, T_OR) || match(parser, T_XOR)) {
        token_type_t operator = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = logical_and(parser);
        expr = create_binary_node(expr, operator, right_expr);
    }
    return expr;
}