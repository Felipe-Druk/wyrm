#include "parser.h"
#include "../options.h"
#include "debugger_parser.h"
#include "../utils/error.h"
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "parser_internal.h"
/// se resuleven antes de la cadena
#include "block_statement.h"
#include "if_statement.h"
#include "while_statement.h"
#include "function_declaration.h"
#include "variable_declaration.h"
// inicio de la "cadena"
#include "logical_or_xor.h"

static const size_t INITIAL_CAPACITY = 5;

parser_t *create_parser(token_vector_t *tokens) {
    parser_t *new_parser = malloc(sizeof(parser_t));
    if (new_parser == NULL) {
        return NULL;
    }
    new_parser->current_index = 0;
    new_parser->tokens = tokens;
    new_parser->flags = 0;
    return new_parser;
}

// punto de control de la cadena
ast_node_t *expression(parser_t *parser) { return logical_or_xor(parser); }

ast_node_t *expression_statement(parser_t *parser) {
    if (match(parser, T_LBRACE)) {
        return block_statement(parser);
    }

    if (match(parser, T_IF)) {
        return if_statement(parser);
    }

    if (match(parser, T_WHILE)) {
        return while_statement(parser);
    }

    // cumple con (TIPO, ...) -> RETORNO nombre = (args) -> { ... }
    if (peek(parser)->type == T_LPAREN) {
        TokenType next_type = get_token(parser->tokens, parser->current_index + 1)->type;
        if ((next_type >= MIN_TYPE && next_type <= MAX_TYPE) || next_type == T_RPAREN) {
            advance(parser);
            return function_declaration(parser);
        }
    }

    // cumple con var_decl -> "TIPO" T_IDENTIFIER (T_ASSIGN | T_LARROW) expression T_SEMICOLON
    if (peek(parser)->type >= MIN_TYPE && peek(parser)->type <= MAX_TYPE) {
        return variable_declaration(parser);
    }

    ast_node_t *expr = expression(parser);

    if (expr == NULL) {
        panic(ERR_SYNTAX, "Syntax error: unexpected term or invalid expression");
    }
    // cumple T_IDENTIFIER (T_ASSIGN | T_LARROW) expression
    if (expr->type == AST_IDENTIFIER && (match(parser, T_ASSIGN) || match(parser, T_LARROW))) {
        TokenType assign_op = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = expression_statement(parser);

        ast_node_t *assignment_node =
            create_assignment_node(expr->ast_node_value.identifier_expr.name, assign_op, right_expr);
        destroy_ast_node(expr); // Liberamos la memoria del nodo de identificador original
        return assignment_node;
    }

    return expr;
}

ast_node_t *parser_parse(parser_t *parser) {

    ast_node_t *root = create_block_node(INITIAL_CAPACITY);
    root->type = AST_PROGRAM;

    while (peek(parser)->type != T_EOF) {
        ast_node_t *actual_node = expression_statement(parser);
        if (block_is_full(&root->ast_node_value.block_expr)) {
            resize_block(&root->ast_node_value.block_expr);
        }
        root->ast_node_value.block_expr.nodes[root->ast_node_value.block_expr.size++] = actual_node;

        if (actual_node->type != AST_BLOCK && actual_node->type != AST_IF_EXPR && actual_node->type != AST_WHILE_EXPR &&
            actual_node->type != AST_FUNCTION_DECL) {

            if (!match(parser, T_SEMICOLON) && peek(parser)->type != T_EOF) {
                panic(ERR_SYNTAX, "Syntax error: ';' expected at the end of the statement");
            }
        }
    }

    if (parser->flags & (DEBUG_MODE | PARSER_MODE)) {
        print_line_debug("== END OF PARSER ==");
        debug_ast(root, 0);
    }
    return root;
}

void destroy_parser(parser_t *parser) {
    if (parser) {
        parser->tokens = NULL;
        parser->current_index = 0;
        parser->flags = 0;
        free(parser);
    }
}