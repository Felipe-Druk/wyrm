#include "evaluator_loop.h"
#include "interpreter_internal.h"
#include "../utils/error.h"
#include <stdbool.h>

wyrm_value_t eval_loop(ast_node_t *node, environment_t *env) {
    wyrm_value_t return_val = {.type = VAL_VOID};
    bool condition = true;
    while (condition) {
        wyrm_value_t condition_val = eval_node(node->ast_node_value.while_expr.condition, env);

        if (condition_val.type != VAL_BOOL) {
            panic(ERR_TYPE, "Type error: La condición del 'while' debe resultar en un booleano");
        }
        if (!condition_val.value.bool_val) {
            condition = false;
            break;
        }
        eval_node(node->ast_node_value.while_expr.body, env);
    }

    return return_val;
}

wyrm_value_t eval_for(ast_node_t *node, environment_t *scope) {
    ast_node_t *header = node->ast_node_value.while_expr.condition;
    ast_node_t *body = node->ast_node_value.while_expr.body;

    if (header == NULL || header->type != AST_ASSIGNMENT ||
        header->ast_node_value.assignment_expr.operator != T_LARROW) {
        panic(ERR_SYNTAX, "Runtime error: 'for' loop requires extraction syntax 'item <- vector'");
    }

    char *iter_name = header->ast_node_value.assignment_expr.name;
    wyrm_value_t vec_val = eval_node(header->ast_node_value.assignment_expr.value, scope);

    if (vec_val.type != VAL_VEC || vec_val.value.vec_val == NULL) {
        panic(ERR_TYPE, "Runtime error: 'for' loop can only iterate over a 'vec'");
    }

    wyrm_vec_t *vec = vec_val.value.vec_val;
    wyrm_value_t last_result = {.type = VAL_VOID};
    environment_t *for_scope = create_environment(scope);

    for (size_t i = 0; i < vec->items.size; i++) {
        wyrm_value_t elem = *(wyrm_value_t *)vector_get(&vec->items, i);
        if (i == 0) {
            environment_set(for_scope, iter_name, elem, (token_type_t)vec->element_type);
        } else {
            wyrm_value_t *slot = environment_get(for_scope, iter_name);
            *slot = elem;
        }
        last_result = eval_node(body, for_scope);
    }

    destroy_environment(for_scope);
    return last_result;
}