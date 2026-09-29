#include "evaluator_function.h"
#include "interpreter.h"
#include "../utils/error.h"
#include <string.h>

wyrm_value_t eval_function_decl(ast_node_t *node, environment_t *scope) {
    wyrm_function_t function = node->ast_node_value.function_decl.function;
    char *name = node->ast_node_value.function_decl.name;
    wyrm_value_t func_val;
    func_val.type = VAL_FUNCTION;
    func_val.value.wyrm_function_val = function;

    environment_set(scope, name, func_val, MAX_TYPE);

    return (wyrm_value_t){.type = VAL_VOID};
}

wyrm_value_t eval_call(ast_node_t *node, environment_t *scope) {
    char *func_name = node->ast_node_value.call_expr.name;
    size_t arg_count = node->ast_node_value.call_expr.arg_count;
    ast_node_t **args = node->ast_node_value.call_expr.args;

    wyrm_value_t *func_val = environment_get(scope, func_name);
    if (func_val == NULL) {
        panic(ERR_TYPE, "Runtime error: Function '%s' is not defined", func_name);
    }
    if (func_val->type != VAL_FUNCTION) {
        panic(ERR_TYPE, "Runtime error: '%s' is not a function", func_name);
    }

    wyrm_function_t func_def = func_val->value.wyrm_function_val;

    if (func_def.arg_count != arg_count) {
        panic(ERR_TYPE, "Runtime error: Function '%s' expects %zu arguments, but got %zu", func_name,
              func_def.arg_count, arg_count);
    }
    environment_t *local_env = create_environment(scope);

    for (size_t i = 0; i < arg_count; i++) {

        wyrm_value_t arg_evaluated = eval_node(args[i], scope);

        char *expected_name = func_def.args[i].name;
        WyrmValueType expected_type = func_def.args[i].type;

        environment_set(local_env, expected_name, arg_evaluated, (TokenType)expected_type);
    }
    wyrm_value_t result = eval_node(func_def.body, local_env);
    destroy_environment(local_env);
    if (result.type != func_def.returned && func_def.returned != VAL_VOID) {
        panic(ERR_TYPE, "Runtime error: Function '%s' returned wrong type", func_name);
    }

    return result;
}