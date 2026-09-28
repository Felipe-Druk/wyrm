#include "interpreter.h"
#include <stdlib.h>

#include "evaluator_number.h"
#include "evaluator_bool.h"
#include "evaluator_binary.h"
#include "evaluator_unary.h"
#include "evaluator_identifier.h"
#include "evaluator_block.h"
#include "evaluator_if.h"

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

#define CAST_FROM_INT64(enum_type, field_name, c_type)                                                                 \
    result.type = enum_type;                                                                                           \
    result.value.field_name = (c_type)original.value.int64_val;                                                        \
    break;

#define CAST_FROM_FLOAT64(enum_type, field_name, c_type)                                                               \
    result.type = enum_type;                                                                                           \
    result.value.field_name = (c_type)original.value.float64_val;                                                      \
    break;

wyrm_value_t cast_value(wyrm_value_t original, TokenType target_type) {
    wyrm_value_t result = original;

    if (original.type == VAL_INT64) {
        switch (target_type) {
        case T_INT8:
            CAST_FROM_INT64(VAL_INT8, int8_val, int8_t)
        case T_INT16:
            CAST_FROM_INT64(VAL_INT16, int16_val, int16_t)
        case T_INT32:
            CAST_FROM_INT64(VAL_INT32, int32_val, int32_t)
        case T_NAT8:
            CAST_FROM_INT64(VAL_NAT8, nat8_val, uint8_t)
        case T_NAT16:
            CAST_FROM_INT64(VAL_NAT16, nat16_val, uint16_t)
        case T_NAT32:
            CAST_FROM_INT64(VAL_NAT32, nat32_val, uint32_t)
        case T_NAT64:
            CAST_FROM_INT64(VAL_NAT64, nat64_val, uint64_t)
        case T_FLOAT32:
            CAST_FROM_INT64(VAL_FLOAT32, float32_val, _Float32)
        case T_FLOAT64:
            CAST_FROM_INT64(VAL_FLOAT64, float64_val, _Float64)
        default:
            break;
        }
    } else if (original.type == VAL_FLOAT64) {
        switch (target_type) {
        case T_INT8:
            CAST_FROM_FLOAT64(VAL_INT8, int8_val, int8_t)
        case T_INT16:
            CAST_FROM_FLOAT64(VAL_INT16, int16_val, int16_t)
        case T_INT32:
            CAST_FROM_FLOAT64(VAL_INT32, int32_val, int32_t)
        case T_NAT8:
            CAST_FROM_FLOAT64(VAL_NAT8, nat8_val, uint8_t)
        case T_NAT16:
            CAST_FROM_FLOAT64(VAL_NAT16, nat16_val, uint16_t)
        case T_NAT32:
            CAST_FROM_FLOAT64(VAL_NAT32, nat32_val, uint32_t)
        case T_NAT64:
            CAST_FROM_FLOAT64(VAL_NAT64, nat64_val, uint64_t)
        case T_FLOAT32:
            CAST_FROM_FLOAT64(VAL_FLOAT32, float32_val, _Float32)
        default:
            break;
        }
    }
    return result;
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
        wyrm_value_t expr_value = eval_node(node->ast_node_value.var_decl_expr.expression, scope);
        wyrm_value_t casted_value = cast_value(expr_value, node->ast_node_value.var_decl_expr.var_type);
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