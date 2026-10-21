#include "parser_internal.h"
#include "primary.h"

ast_node_t *power(parser_t *parser) {
    ast_node_t *primary_expr = primary(parser);

    if (match(parser, T_POW)) {
        ast_node_t *right_expr = power(parser); // para resolver cadena de potencias
        return create_binary_node(primary_expr, T_POW, right_expr);
    }

    return primary_expr;
}