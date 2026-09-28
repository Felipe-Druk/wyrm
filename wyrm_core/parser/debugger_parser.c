#include "debugger_parser.h"

void debug_ast(ast_node_t *node, int level) {
    if (!node)
        return;

    print_line_debug("In level %i : ", level);

    switch (node->type) {
    case AST_NUMBER_LITERAL:
        print_line_debug("NumberLiteral(%s)", node->ast_node_value.number_expr.value);
        break;
    case AST_BOOL_LITERAL:
        print_line_debug("BoolLiteral(%i)", node->ast_node_value.bool_expr.value);
        break;
    case AST_IDENTIFIER:
        print_line_debug("Identifier(%s)", node->ast_node_value.identifier_expr.name);
        break;
    case AST_UNARY_EXPR:
        print_line_debug("UnaryExpr(op: %s)", token_type_to_string(node->ast_node_value.unary_expr.operator));
        debug_ast(node->ast_node_value.unary_expr.right, level + 1);
        break;
    case AST_BINARY_EXPR:
        print_line_debug("BinaryExpr(op: %s)", token_type_to_string(node->ast_node_value.binary_expr.operator));
        debug_ast(node->ast_node_value.binary_expr.left, level + 1);
        debug_ast(node->ast_node_value.binary_expr.right, level + 1);
        break;
    case AST_VAR_DECLARATION:
        print_line_debug("VarDeclaration(%s)", node->ast_node_value.var_decl_expr.identifier);
        debug_ast(node->ast_node_value.var_decl_expr.expression, level + 1);
        break;
    case AST_ASSIGNMENT:
        print_line_debug("VarAssignment(%s)", node->ast_node_value.assignment_expr.name);
        debug_ast(node->ast_node_value.assignment_expr.value, level + 1);
        break;
    case AST_PROGRAM:
    case AST_BLOCK:
        print_line_debug("Block(size: %zu)", node->ast_node_value.block_expr.size);
        for (size_t i = 0; i < node->ast_node_value.block_expr.size; i++) {
            debug_ast(node->ast_node_value.block_expr.nodes[i], level + 1);
        }
        break;
    case AST_IF_EXPR:
        print_line_debug("IfExpr: ");
        debug_ast(node->ast_node_value.if_expr.condition, level + 1);
        if (node->ast_node_value.if_expr.then_branch) {
            debug_ast(node->ast_node_value.if_expr.then_branch, level + 1);
        }
        if (node->ast_node_value.if_expr.else_branch) {
            debug_ast(node->ast_node_value.if_expr.else_branch, level + 1);
        }
        break;
    case AST_WHILE_EXPR:
        print_line_debug("WhileExpr");
        debug_ast(node->ast_node_value.while_expr.condition, level + 1);
        debug_ast(node->ast_node_value.while_expr.body, level + 1);
        break;
    }
}