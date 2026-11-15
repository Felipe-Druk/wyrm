#include "evaluator_function.h"
#include "interpreter_internal.h"
#include "../../utils/error.h"
#include <string.h>
#include <stdlib.h>
#include <unistd.h>

wyrm_value_t eval_function_decl(ast_node_t *node, environment_t *scope) {
    wyrm_function_t function = node->ast_node_value.function_decl.function;
    char *name = node->ast_node_value.function_decl.name;
    wyrm_value_t func_val;
    func_val.type = VAL_FUNCTION;
    func_val.value.wyrm_function_val = function;

    environment_set(scope, name, func_val, MAX_TYPE);

    return (wyrm_value_t){.type = VAL_VOID};
}

static wyrm_value_t eval_native_write(size_t arg_count, ast_node_t **args, environment_t *scope) {
    if (arg_count != 2) {
        panic(ERR_TYPE, "Runtime error: 'write' expects 2 arguments (fd, buffer), got %zu", arg_count);
    }

    wyrm_value_t fd_val = eval_node(args[0], scope);
    wyrm_value_t buf_val = eval_node(args[1], scope);

    if (buf_val.type != VAL_VEC || buf_val.value.vec_val == NULL) {
        panic(ERR_TYPE, "Runtime error: Second argument to 'write' must be a vector");
    }

    int fd = (int)get_cast_int64(fd_val);
    wyrm_vec_t *vec = buf_val.value.vec_val;
    size_t len = vec->items.size;

    uint8_t *raw_buf = NULL;
    if (len > 0) {
        raw_buf = malloc(len);
        for (size_t i = 0; i < len; i++) {
            wyrm_value_t item = *(wyrm_value_t *)vector_get(&vec->items, i);
            raw_buf[i] = (uint8_t)get_cast_int64(item);
        }
        if (raw_buf[len - 1] == 0) {
            len--;
        }
    }

    ssize_t written = (len > 0) ? write(fd, raw_buf, len) : 0;
    free(raw_buf);

    return (wyrm_value_t){
        .type = VAL_INT64,
        .value.int64_val = (int64_t)written,
    };
}

wyrm_value_t eval_call(ast_node_t *node, environment_t *scope) {
    char *func_name = node->ast_node_value.call_expr.name;
    size_t arg_count = node->ast_node_value.call_expr.arg_count;
    ast_node_t **args = node->ast_node_value.call_expr.args;

    if (strcmp(func_name, "write") == 0) {
        return eval_native_write(arg_count, args, scope);
    }

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
        wyrm_value_type_t expected_type = func_def.args[i].type;

        environment_set(local_env, expected_name, arg_evaluated, (token_type_t)expected_type);
    }
    wyrm_value_t result = cast_to_value_type(eval_node(func_def.body, local_env), func_def.returned);
    destroy_environment(local_env);

    if (func_def.returned == VAL_VOID) {
        return (wyrm_value_t){.type = VAL_VOID};
    }

    result = cast_to_value_type(result, func_def.returned);
    if (result.type != func_def.returned) {
        panic(ERR_TYPE, "Runtime error: Function '%s' returned wrong type", func_name);
    }
    return result;
}