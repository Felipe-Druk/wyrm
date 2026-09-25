#include "parser.h"
#include <stdlib.h>

parset_t *create_parset(token_vector_t *tokens) {
    parser_t *new_parser = malloc(sizeof(parser_t));
    if (new_parser == NULL) {
        return NULL;
    }
    new_parser->current_index = 0;
    new_parser->tokens = tokens;
    return new_parser;
}

ast_node_t *primary(parser) {
    ast_node_t *primary_expr;
    return primary_expr;
}

ast_node_t *power(parser) {
    ast_node_t *primary_expr = primary(parser);
    return primary_expr;
}

ast_node_t *term(parser) {
    ast_node_t *power_expr = power(parser);
    return power_expr;
}

ast_node_t *expression(parser) {
    ast_node_t *term_expr = term(parser);
    return term_expr;
}

ast_node_t *expression_statement(parser_t *parser) {
    ast_node_t *expr = expression(parser);

    return expr;
}

ast_node_t *parser_parse(parser_t *parser) {
    // Arriba los especiales

    // retornamos expresión
    return expression_statement(parser);
}