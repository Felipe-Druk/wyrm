#include "evaluator_loop.h"
#include "interpreter.h"
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