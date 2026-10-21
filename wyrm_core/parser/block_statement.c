#include "block_statement.h"
#include "../utils/error.h"
#include "parser.h"
#include "parser_internal.h"

static const size_t INITIAL_CAPACITY = 5;

ast_node_t *expression_statement(parser_t *parser); // solo la firma

ast_node_t *block_statement(parser_t *parser) {
    ast_node_t *block = create_block_node(INITIAL_CAPACITY);

    while (peek(parser)->type != T_RBRACE && peek(parser)->type != T_EOF) {
        ast_node_t *stmt = expression_statement(parser);

        if (block_is_full(&block->ast_node_value.block_expr)) {
            resize_block(&block->ast_node_value.block_expr);
        }
        block->ast_node_value.block_expr.nodes[block->ast_node_value.block_expr.size++] = stmt;

        if (stmt->type != AST_BLOCK && stmt->type != AST_IF_EXPR && stmt->type != AST_WHILE_EXPR &&
            stmt->type != AST_FUNCTION_DECL) {

            if (!match(parser, T_SEMICOLON) && peek(parser)->type != T_RBRACE && peek(parser)->type != T_EOF) {
                panic(ERR_SYNTAX, "Syntax error: ';' expected at the end of the statement");
            }
        }
    }

    if (!match(parser, T_RBRACE)) {
        panic(ERR_SYNTAX, "Syntax error: Expected '}' to close the block");
    }

    return block;
}