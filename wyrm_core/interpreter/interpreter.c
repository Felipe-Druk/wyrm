#include "interpreter.h"
#include <stdlib.h>

#include "evaluator_number.h"
#include "evaluator_bool.h"
#include "evaluator_binary.h"
#include "evaluator_unary.h"
#include "evaluator_identifier.h"
#include "evaluator_block.h"
#include "evaluator_if.h"
#include "evaluator_loop.h"
#include "evaluator_function.h"

interpreter_t *create_interpreter(ast_node_t *root, environment_t *scope) {

    interpreter_t *new_interpreter = malloc(sizeof(interpreter_t));
    if (new_interpreter == NULL) {
        return NULL;
    }

    new_interpreter->flags = 0;
    new_interpreter->root = root;
    new_interpreter->scope = scope;

    return new_interpreter;
}

wyrm_value_t eval_node(ast_node_t *node, environment_t *scope) {
    wyrm_value_t result = {.type = VAL_VOID};
    if (node == NULL)
        return result;

    switch (node->type) {
    case AST_NUMBER_LITERAL:
        result = eval_number(node);
        break;
    case AST_BOOL_LITERAL:
        result = eval_bool(node);
        break;
    case AST_BINARY_EXPR: {
        wyrm_value_t left = eval_node(node->ast_node_value.binary_expr.left, scope);
        wyrm_value_t right = eval_node(node->ast_node_value.binary_expr.right, scope);

        result = eval_binary(node, left, right);
        break;
    }
    case AST_UNARY_EXPR: {
        wyrm_value_t right = eval_node(node->ast_node_value.unary_expr.right, scope);
        result = eval_unary(node, right);
        break;
    }
    case AST_BLOCK: {
        result = eval_block(node, scope);
        break;
    }
    case AST_PROGRAM: {
        wyrm_value_t final_val = {0};
        final_val.type = VAL_VOID;

        for (size_t i = 0; i < node->ast_node_value.block_expr.size; i++) {
            final_val = eval_node(node->ast_node_value.block_expr.nodes[i], scope);
        }
        return final_val;
    }
    case AST_IDENTIFIER: {
        result = eval_identifier(node, scope);
        break;
    }
    case AST_VAR_DECLARATION: {
        token_type_t var_type = node->ast_node_value.var_decl_expr.var_type;
        wyrm_value_t casted_value = {0};
        if (node->ast_node_value.var_decl_expr.expression == NULL) {
            casted_value.type = token_to_value_type(var_type);
        } else {
            wyrm_value_t expr_value = eval_node(node->ast_node_value.var_decl_expr.expression, scope);
            casted_value = cast_value(expr_value, var_type);
        }
        result = eval_val_decl(node, scope, casted_value);
        break;
    }
    case AST_ASSIGNMENT: {
        wyrm_value_t new_val = eval_node(node->ast_node_value.assignment_expr.value, scope);
        result = eval_assignment(node, scope, new_val);
        break;
    }
    case AST_IF_EXPR: {
        result = eval_if(node, scope);
        break;
    }
    case AST_WHILE_EXPR: {
        result = eval_loop(node, scope);
        break;
    }
    case AST_FUNCTION_DECL:
        return eval_function_decl(node, scope);
    case AST_CALL:
        return eval_call(node, scope);
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
    return eval_node(interpreter->root, interpreter->scope);
}

void destroy_interpreter(interpreter_t *interpreter) {
    if (interpreter == NULL) {
        return;
    }
    interpreter->root = NULL;
    interpreter->scope = NULL;
    free(interpreter);
}
