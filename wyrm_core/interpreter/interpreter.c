#include "interpreter.h"
#include <stdlib.h>

#include "evaluator_number.h"

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