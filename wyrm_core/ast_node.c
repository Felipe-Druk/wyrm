#include "ast_node.h"
#include <stdlib.h>
#include <string.h>
#include "utils/error.h"

void check_node(const ast_node_t *node, const char *name) {
    if (node == NULL) {
        panic(ERR_OUT_MEMORY, "Fatal error: out of memory of %s", name);
    }
}

ast_node_t *create_number_node(char *value, TokenType numeric_type) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "Number");

    new_node->type = AST_NUMBER_LITERAL;
    new_node->ast_node_value.number_expr.value = strdup(value);
    new_node->ast_node_value.number_expr.numeric_type = numeric_type;

    return new_node;
}

ast_node_t *create_bool_node(bool value) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "Bool");

    new_node->type = AST_BOOL_LITERAL;
    new_node->ast_node_value.bool_expr.value = value;

    return new_node;
}

ast_node_t *create_identifier_node(char *value) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "Identifier");

    new_node->type = AST_IDENTIFIER;
    new_node->ast_node_value.identifier_expr.name = strdup(value);
    return new_node;
}

ast_node_t *create_binary_node(ast_node_t *left, TokenType operator, ast_node_t *right) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "Binary");

    new_node->type = AST_BINARY_EXPR;
    new_node->ast_node_value.binary_expr.left = left;
    new_node->ast_node_value.binary_expr.operator = operator;
    new_node->ast_node_value.binary_expr.right = right;

    return new_node;
}

ast_node_t *create_unary_node(TokenType operator, ast_node_t *right) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "Unary");

    new_node->type = AST_UNARY_EXPR;
    new_node->ast_node_value.binary_expr.operator = operator;
    new_node->ast_node_value.binary_expr.right = right;
    return new_node;
}

ast_node_t *create_var_decl_node(TokenType var_type, char *identifier, ast_node_t *expression, TokenType assign_op) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "var_decl");

    new_node->type = AST_VAR_DECLARATION;
    new_node->ast_node_value.var_decl_expr.expression = expression;
    new_node->ast_node_value.var_decl_expr.identifier = identifier;
    new_node->ast_node_value.var_decl_expr.var_type = var_type;
    new_node->ast_node_value.var_decl_expr.assign_op = assign_op;

    return new_node;
}

ast_node_t *create_block_node(size_t capacity) {
    if (capacity == 0) {
        return NULL;
    }

    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "Block");

    new_node->type = AST_BLOCK;
    new_node->ast_node_value.block_expr.capacity = capacity;
    new_node->ast_node_value.block_expr.size = 0;
    new_node->ast_node_value.block_expr.nodes = malloc(sizeof(ast_node_t *) * capacity);

    return new_node;
}

int block_is_full(const ast_block_t *block_node) { return (block_node->size >= block_node->capacity); }

void resize_block(ast_block_t *block_node) {
    block_node->capacity *= 2;
    block_node->nodes = realloc(block_node->nodes, sizeof(ast_node_t *) * block_node->capacity);

    if (block_node->nodes == NULL) {
        panic(ERR_OUT_MEMORY, "Fatal error: out of memory of block resize");
    }
}

ast_node_t *create_if_node(ast_node_t *condition, ast_node_t *then_branch, ast_node_t *else_branch) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "IfElse");
    new_node->type = AST_IF_EXPR;
    new_node->ast_node_value.if_expr.condition = condition;
    new_node->ast_node_value.if_expr.then_branch = then_branch;
    new_node->ast_node_value.if_expr.else_branch = else_branch;
    return new_node;
}

ast_node_t *create_assignment_node(const char *name, TokenType operator, ast_node_t *value) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "Assignment");

    new_node->type = AST_ASSIGNMENT;
    new_node->ast_node_value.assignment_expr.name = strdup(name);
    new_node->ast_node_value.assignment_expr.operator = operator;
    new_node->ast_node_value.assignment_expr.value = value;
    return new_node;
}