#include "evaluator_identifier.h"
#include "../../utils/error.h"
#include "../interpreter_internal.h"

wyrm_value_t eval_val_decl(ast_node_t *node, environment_t *env, wyrm_value_t expr_value) {
    wyrm_value_t assign_value = {.type = VAL_VOID};
    environment_set(env, node->ast_node_value.var_decl_expr.identifier, expr_value,
                    node->ast_node_value.var_decl_expr.var_type);
    if (node->ast_node_value.var_decl_expr.assign_op == T_LARROW) {
        return expr_value;
    }
    return assign_value;
}

wyrm_value_t eval_identifier(ast_node_t *node, environment_t *env) {
    wyrm_value_t *stored_value = environment_get(env, node->ast_node_value.identifier_expr.name);

    if (stored_value == NULL) {
        panic(ERR_UNDEFINED_VAR, "Error UNDEFINED_VAR: %s", node->ast_node_value.identifier_expr.name);
    }

    return *stored_value;
}

wyrm_value_t eval_assignment(ast_node_t *node, environment_t *env, wyrm_value_t expr_value) {
    wyrm_value_t return_value = {.type = VAL_VOID};

    // asignacion a un vector
    if (node->ast_node_value.assignment_expr.index != NULL) {
        wyrm_value_t *vec_var = environment_get(env, node->ast_node_value.assignment_expr.name);
        if (vec_var == NULL) {
            panic(ERR_UNDEFINED_VAR, "Error UNDEFINED_VAR: '%s'", node->ast_node_value.assignment_expr.name);
        }
        if (vec_var->type != VAL_VEC || vec_var->value.vec_val == NULL) {
            panic(ERR_TYPE, "Runtime error: '%s' is not a vector", node->ast_node_value.assignment_expr.name);
        }

        wyrm_value_t idx_val = eval_node(node->ast_node_value.assignment_expr.index, env);
        int64_t idx = get_cast_int64(idx_val);
        wyrm_vec_t *vec = vec_var->value.vec_val;

        if (idx < 0 || (size_t)idx >= vec->items.size) {
            panic(ERR_OVERFLOW, "Runtime error: Vector index %lld out of bounds (size: %zu)", (long long)idx,
                  vec->items.size);
        }

        wyrm_value_t casted_elem = cast_to_value_type(expr_value, vec->element_type);
        wyrm_value_t *slot = (wyrm_value_t *)vector_get(&vec->items, (size_t)idx);
        *slot = casted_elem;

        if (node->ast_node_value.assignment_expr.operator == T_LARROW) {
            return casted_elem;
        }
        return return_value;
    }

    // default
    if (!environment_assign(env, node->ast_node_value.assignment_expr.name, expr_value)) {
        panic(ERR_UNDEFINED_VAR, "Error UNDEFINED_VAR: Cannot assign to undeclared variable '%s'",
              node->ast_node_value.assignment_expr.name);
    }
    if (node->ast_node_value.assignment_expr.operator == T_LARROW) {
        return expr_value;
    }

    return return_value;
}

wyrm_value_t eval_vec_literal(ast_node_t *node, environment_t *env) {
    size_t count = node->ast_node_value.block_expr.size;
    wyrm_vec_t *vec = create_wyrm_vec(VAL_INT64, count);

    for (size_t i = 0; i < count; i++) {
        wyrm_value_t item = eval_node(node->ast_node_value.block_expr.nodes[i], env);
        if (i == 0) {
            vec->element_type = item.type;
        }
        vector_push(&vec->items, &item);
    }

    wyrm_value_t result = {0};
    result.type = VAL_VEC;
    result.value.vec_val = vec;
    return result;
}

wyrm_value_t eval_vec_access(ast_node_t *node, environment_t *env) {
    wyrm_value_t vec_val = eval_node(node->ast_node_value.binary_expr.left, env);
    if (vec_val.type != VAL_VEC || vec_val.value.vec_val == NULL) {
        panic(ERR_TYPE, "Runtime error: Attempting to index a non-vector value");
    }

    wyrm_value_t idx_val = eval_node(node->ast_node_value.binary_expr.right, env);
    int64_t idx = get_cast_int64(idx_val);
    wyrm_vec_t *vec = vec_val.value.vec_val;

    if (idx < 0 || (size_t)idx >= vec->items.size) {
        panic(ERR_OVERFLOW, "Runtime error: Vector index %lld out of bounds (size: %zu)", (long long)idx,
              vec->items.size);
    }

    return *(wyrm_value_t *)vector_get(&vec->items, (size_t)idx);
}