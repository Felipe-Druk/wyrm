#include "evaluator_number.h"
#include "../wyrm_token.h"

#include <stdlib.h>
#include <errno.h>

wyrm_value_t eval_number(ast_node_t *node) {
    if (node->type != AST_NUMBER_LITERAL) {
        wyrm_value_t error_val = {.type = VAL_VOID}; // TODO: Error
        return error_val;
    }

    wyrm_value_t result;
    const char *text_value = node->ast_node_value.number_expr.value;

    if (node->ast_node_value.number_expr.numeric_type == T_NUMBER) {
        // El codigo no es complicado, basicamente intentamos traer el numero, pero si es muy grande se lo intenta como
        // un natural
        long long int parsed_val = strtoll(text_value, NULL, 10);
        if (errno == ERANGE) {
            result.type = VAL_NAT64;
            result.value.nat64_val = (uint64_t)strtoull(text_value, NULL, 10);
        } else {
            result.type = VAL_INT64;
            result.value.int64_val = (int64_t)parsed_val;
        }
    } else {
        result.type = VAL_FLOAT64;
        result.value.float64_val = strtod(text_value, NULL);
    }
    return result;
}