#include "interpreter.h"
#include <stdlib.h>

#include "evaluator_number.h"
#include "evaluator_binary.h"
#include "evaluator_unary.h"

interpreter_t *create_interpreter(ast_node_t *root) {

    interpreter_t *new_interpreter = malloc(sizeof(interpreter_t));
    if (new_interpreter == NULL) {
        return NULL;
    }

    new_interpreter->flags = 0;
    new_interpreter->root = root;

    return new_interpreter;
}

wyrm_value_t eval_node(ast_node_t *node) {
    wyrm_value_t result = {.type = VAL_VOID};
    if (node == NULL)
        return result;

    switch (node->type) {
    case AST_NUMBER_LITERAL:
        result = eval_number(node);
        break;
    case AST_BINARY_EXPR: {
        wyrm_value_t left = eval_node(node->ast_node_value.binary_expr.left);
        wyrm_value_t right = eval_node(node->ast_node_value.binary_expr.right);

        result = eval_binary(node, left, right);
        break;
    }
    case AST_UNARY_EXPR: {
        wyrm_value_t right = eval_node(node->ast_node_value.unary_expr.right);
        result = eval_unary(node, right);
        break;
    }
    case AST_BLOCK: {
        wyrm_value_t final_val = {.type = VAL_VOID};
        for (size_t i = 0; i < node->ast_node_value.block_expr.size; i++) {
            final_val = eval_node(node->ast_node_value.block_expr.nodes[i]);
        }
        result = final_val;
        break;
    }
    default:
        break;
    }
    return result;
}

wyrm_value_t interpreter_interpret(interpreter_t *interpreter) {
    if (interpreter == NULL || interpreter->root == NULL) {
        wyrm_value_t empty = {.type = VAL_VOID};
        return empty;
    }
    return eval_node(interpreter->root);
}