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

void destroy_number_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    free(node->ast_node_value.number_expr.value);
    free(node);
}

ast_node_t *create_bool_node(bool value) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "Bool");

    new_node->type = AST_BOOL_LITERAL;
    new_node->ast_node_value.bool_expr.value = value;

    return new_node;
}

void destroy_bool_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    free(node);
}

ast_node_t *create_identifier_node(char *value) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "Identifier");

    new_node->type = AST_IDENTIFIER;
    new_node->ast_node_value.identifier_expr.name = strdup(value);
    return new_node;
}

void destroy_identifier_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    free(node->ast_node_value.identifier_expr.name);
    free(node);
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

void destroy_binary_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    destroy_ast_node(node->ast_node_value.binary_expr.left);
    destroy_ast_node(node->ast_node_value.binary_expr.right);
    free(node);
}

ast_node_t *create_unary_node(TokenType operator, ast_node_t *right) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "Unary");

    new_node->type = AST_UNARY_EXPR;
    new_node->ast_node_value.unary_expr.operator = operator;
    new_node->ast_node_value.unary_expr.right = right;
    return new_node;
}

void destroy_unary_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    destroy_ast_node(node->ast_node_value.unary_expr.right);
    free(node);
}

ast_node_t *create_var_decl_node(TokenType var_type, char *identifier, ast_node_t *expression, TokenType assign_op) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "var_decl");

    new_node->type = AST_VAR_DECLARATION;
    new_node->ast_node_value.var_decl_expr.expression = expression;
    new_node->ast_node_value.var_decl_expr.identifier = strdup(identifier);
    new_node->ast_node_value.var_decl_expr.var_type = var_type;
    new_node->ast_node_value.var_decl_expr.assign_op = assign_op;

    return new_node;
}

void destroy_var_decl_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    free(node->ast_node_value.var_decl_expr.identifier);
    destroy_ast_node(node->ast_node_value.var_decl_expr.expression);
    free(node);
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

void destroy_block_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    for (size_t i = 0; i < node->ast_node_value.block_expr.size; i++) {
        destroy_ast_node(node->ast_node_value.block_expr.nodes[i]);
    }
    free(node->ast_node_value.block_expr.nodes);
    free(node);
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

void destroy_if_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    destroy_ast_node(node->ast_node_value.if_expr.condition);
    destroy_ast_node(node->ast_node_value.if_expr.then_branch);
    destroy_ast_node(node->ast_node_value.if_expr.else_branch);
    free(node);
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

void destroy_assignment_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    free(node->ast_node_value.assignment_expr.name);
    destroy_ast_node(node->ast_node_value.assignment_expr.value);
    free(node);
}

ast_node_t *create_while_node(ast_node_t *condition, ast_node_t *body) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "While");
    new_node->type = AST_WHILE_EXPR;
    new_node->ast_node_value.while_expr.condition = condition;
    new_node->ast_node_value.while_expr.body = body;
    return new_node;
}

void destroy_while_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    destroy_ast_node(node->ast_node_value.while_expr.condition);
    destroy_ast_node(node->ast_node_value.while_expr.body);
    free(node);
}

ast_node_t *create_function_decl_node(char *name, wyrm_function_t function) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "FunctionDecl");
    new_node->type = AST_FUNCTION_DECL;
    new_node->ast_node_value.function_decl.name = strdup(name);
    new_node->ast_node_value.function_decl.function = function;
    return new_node;
}

void destroy_function_decl_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    free(node->ast_node_value.function_decl.name);
    destroy_ast_node(node->ast_node_value.function_decl.function.body);
    for (size_t i = 0; i < node->ast_node_value.function_decl.function.arg_count; i++) {
        free(node->ast_node_value.function_decl.function.args[i].name);
    }
    free(node->ast_node_value.function_decl.function.args);
    free(node);
}

ast_node_t *create_call_node(char *name, size_t arg_count, ast_node_t **args) {
    ast_node_t *new_node = calloc(1, sizeof(ast_node_t));
    check_node(new_node, "FunctionCall");
    new_node->type = AST_CALL;
    new_node->ast_node_value.call_expr.name = strdup(name);
    new_node->ast_node_value.call_expr.arg_count = arg_count;
    new_node->ast_node_value.call_expr.args = args;
    return new_node;
}

void destroy_call_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    free(node->ast_node_value.call_expr.name);
    for (size_t i = 0; i < node->ast_node_value.call_expr.arg_count; i++) {
        destroy_ast_node(node->ast_node_value.call_expr.args[i]);
    }
    free(node->ast_node_value.call_expr.args);
    free(node);
}

void destroy_ast_node(ast_node_t *node) {
    if (node == NULL) {
        return;
    }
    switch (node->type) {
    case AST_NUMBER_LITERAL:
        destroy_number_node(node);
        break;
    case AST_BOOL_LITERAL:
        destroy_bool_node(node);
        break;
    case AST_IDENTIFIER:
        destroy_identifier_node(node);
        break;
    case AST_BINARY_EXPR:
        destroy_binary_node(node);
        break;
    case AST_UNARY_EXPR:
        destroy_unary_node(node);
        break;
    case AST_VAR_DECLARATION:
        destroy_var_decl_node(node);
        break;
    case AST_PROGRAM:
    case AST_BLOCK:
        destroy_block_node(node);
        break;
    case AST_IF_EXPR:
        destroy_if_node(node);
        break;
    case AST_ASSIGNMENT:
        destroy_assignment_node(node);
        break;
    case AST_WHILE_EXPR:
        destroy_while_node(node);
        break;
    case AST_FUNCTION_DECL:
        destroy_function_decl_node(node);
        break;
    case AST_CALL:
        destroy_call_node(node);
        break;
    default:
        break;
    }
}