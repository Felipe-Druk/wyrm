#include "unary.h"
#include "parser_internal.h"
#include "power.h"

ast_node_t *unary(parser_t *parser) {
    if (match(parser, T_SUB) || match(parser, T_NOT)) {
        TokenType operator = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = unary(parser); // para resolver cadena de negadores
        return create_unary_node(operator, right_expr);
    }
    return power(parser);
}