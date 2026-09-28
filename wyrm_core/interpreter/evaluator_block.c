#include "evaluator_block.h"
#include "interpreter.h"

wyrm_value_t eval_block(ast_node_t *node, environment_t *parent_scope) {
    environment_t *local_scope = create_environment(parent_scope);

    wyrm_value_t final_val = {0};
    final_val.type = VAL_VOID;

    for (size_t i = 0; i < node->ast_node_value.block_expr.size; i++) {
        final_val = eval_node(node->ast_node_value.block_expr.nodes[i], local_scope);
    }

    destroy_environment(local_scope);

    return final_val;
}