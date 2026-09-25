#include "ast_node.h"
#include <stdlib.h>
#include <string.h>

ast_node_t *create_number_node(char *value, TokenType numeric_type) {
    ast_node_t *new_node = malloc(sizeof(ast_node_t));

    new_node->type = AST_NUMBER_LITERAL;
    new_node->ast_node_value.number_expr.value = strdup(value);
    new_node->ast_node_value.number_expr.numeric_type = numeric_type;

    return new_node;
}

ast_node_t *create_identifier_node(char *value) {
    ast_node_t *new_node = malloc(sizeof(ast_node_t));

    new_node->type = AST_IDENTIFIER;
    new_node->ast_node_value.identifier_expr.name = strdup(value);
    return new_node;
}