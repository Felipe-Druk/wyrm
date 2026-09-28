#include "parser.h"
#include "../options.h"
#include "debugger_parser.h"
#include "../utils/error.h"
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

const size_t INITIAL_CAPACITY = 5;

// definimos funciones privadas para resolver llamados recursivos
ast_node_t *expression(parser_t *parser);
ast_node_t *expression_statement(parser_t *parser);
ast_node_t *block_statement(parser_t *parser);
ast_node_t *if_statement(parser_t *parser);

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

// puede devolver NULL
wyrm_token_t *peek(parser_t *parser) { return get_token(parser->tokens, parser->current_index); }

wyrm_token_t *advance(parser_t *parser) {
    wyrm_token_t *current = peek(parser);
    if (current->type != T_EOF) {
        parser->current_index++;
    }
    return current;
}

int match(parser_t *parser, TokenType expected_type_token) {
    if (peek(parser)->type == expected_type_token) {
        advance(parser);
        return 1;
    }
    return 0;
}

// cumple con primary -> T_NUMBER | T_FLOAT_NUMBER | T_IDENTIFIER
ast_node_t *primary(parser_t *parser) {

    if (match(parser, T_TRUE)) {
        return create_bool_node(true);
    }
    if (match(parser, T_FALSE)) {
        return create_bool_node(false);
    }

    if (match(parser, T_LPAREN)) {
        ast_node_t *expr = expression(parser);
        if (!match(parser, T_RPAREN)) {
            panic(ERR_SYNTAX, "Syntax error: Expected ')' after expression");
        }
        return expr;
    }

    if (match(parser, T_NUMBER) || match(parser, T_FLOAT_NUMBER)) {
        wyrm_token_t *previous = get_token(parser->tokens, parser->current_index - 1);
        return create_number_node(previous->lexeme, previous->type);
    }

    if (match(parser, T_IDENTIFIER)) {
        wyrm_token_t *previous = get_token(parser->tokens, parser->current_index - 1);
        return create_identifier_node(previous->lexeme);
    }

    // Futuro error
    return NULL;
}

// cumple con power -> primary ( T_POW primary )
ast_node_t *power(parser_t *parser) {
    ast_node_t *primary_expr = primary(parser);

    if (match(parser, T_POW)) {
        ast_node_t *right_expr = power(parser); // para resolver cadena de potencias
        return create_binary_node(primary_expr, T_POW, right_expr);
    }

    return primary_expr;
}

//  cumple con unary -> ( T_SUB ) power
ast_node_t *unary(parser_t *parser) {
    if (match(parser, T_SUB) || match(parser, T_NOT)) {
        TokenType operator = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = unary(parser); // para resolver cadena de negadores
        return create_unary_node(operator, right_expr);
    }
    return power(parser);
}

// cumple con factor -> unary ( ( T_MUL | T_DIV | T_MOD ) unary )
ast_node_t *factor(parser_t *parser) {
    ast_node_t *expr = unary(parser);

    while (match(parser, T_MUL) || match(parser, T_DIV) || match(parser, T_MOD)) {
        wyrm_token_t *previous = get_token(parser->tokens, parser->current_index - 1);
        ast_node_t *right_expr = unary(parser);
        expr = create_binary_node(expr, previous->type, right_expr);
    }

    return expr;
}

// cumple con term -> factor ( ( T_ADD | T_SUB ) factor )
ast_node_t *term(parser_t *parser) {
    ast_node_t *expr = factor(parser);

    while (match(parser, T_ADD) || match(parser, T_SUB)) {
        wyrm_token_t *previous = get_token(parser->tokens, parser->current_index - 1);
        ast_node_t *right_expr = factor(parser);
        expr = create_binary_node(expr, previous->type, right_expr);
    }
    return expr;
}

// cumple con comparison -> term ( ( T_LESS | T_GREATER | T_LESS_EQUAL | T_GREATER_EQUAL ) term )
ast_node_t *comparison(parser_t *parser) {
    ast_node_t *expr = term(parser);

    while (match(parser, T_LESS) || match(parser, T_GREATER) || match(parser, T_LESS_EQUAL) ||
           match(parser, T_GREATER_EQUAL)) {
        TokenType operator = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = term(parser);
        expr = create_binary_node(expr, operator, right_expr);
    }
    return expr;
}

// cumple con equality -> comparison ( ( T_EQUAL | T_NOT_EQUAL ) comparison )
ast_node_t *equality(parser_t *parser) {
    ast_node_t *expr = comparison(parser);

    while (match(parser, T_EQUAL) || match(parser, T_NOT_EQUAL)) {
        TokenType operator = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = comparison(parser);
        expr = create_binary_node(expr, operator, right_expr);
    }
    return expr;
}

// cumple con logical_and -> equality ( T_AND equality )
ast_node_t *logical_and(parser_t *parser) {
    ast_node_t *expr = equality(parser);

    while (match(parser, T_AND)) {
        TokenType operator = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = equality(parser);
        expr = create_binary_node(expr, operator, right_expr);
    }
    return expr;
}

// cumple con logical_or -> logical_and ( ( T_OR | T_XOR ) logical_and )
ast_node_t *logical_or(parser_t *parser) {
    ast_node_t *expr = logical_and(parser);

    while (match(parser, T_OR) || match(parser, T_XOR)) {
        TokenType operator = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = logical_and(parser);
        expr = create_binary_node(expr, operator, right_expr);
    }
    return expr;
}

// punto de control de la cadena
ast_node_t *expression(parser_t *parser) { return logical_or(parser); }

ast_node_t *if_statement(parser_t *parser) {
    if (!match(parser, T_LPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: Expected '(' after 'if'");
    }

    ast_node_t *condition = expression(parser);

    if (!match(parser, T_RPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: Expected ')' after condition");
    }

    ast_node_t *then_branch = expression_statement(parser);
    ast_node_t *else_branch = NULL;

    if (match(parser, T_ELSE)) {
        if (match(parser, T_IF)) {
            else_branch = if_statement(parser);
        } else {
            else_branch = expression_statement(parser);
        }
    }

    return create_if_node(condition, then_branch, else_branch);
}

ast_node_t *block_statement(parser_t *parser) {
    ast_node_t *block = create_block_node(INITIAL_CAPACITY);

    while (peek(parser)->type != T_RBRACE && peek(parser)->type != T_EOF) {
        ast_node_t *stmt = expression_statement(parser);

        if (block_is_full(&block->ast_node_value.block_expr)) {
            resize_block(&block->ast_node_value.block_expr);
        }
        block->ast_node_value.block_expr.nodes[block->ast_node_value.block_expr.size++] = stmt;

        if ((stmt->type != AST_BLOCK && stmt->type != AST_IF_EXPR) && !match(parser, T_SEMICOLON) &&
            peek(parser)->type != T_RBRACE && peek(parser)->type != T_EOF) {
            panic(ERR_SYNTAX, "Syntax error: ';' expected at the end of the statement");
        }
    }

    if (!match(parser, T_RBRACE)) {
        panic(ERR_SYNTAX, "Syntax error: Expected '}' to close the block");
    }

    return block;
}

ast_node_t *expression_statement(parser_t *parser) {
    if (match(parser, T_LBRACE)) {
        return block_statement(parser);
    }

    if (match(parser, T_IF)) {
        return if_statement(parser);
    }

    // cumple con var_decl -> "TIPO" T_IDENTIFIER (T_ASSIGN | T_RASSIGN) expression T_SEMICOLON
    TokenType actual_type = peek(parser)->type;
    if (actual_type >= MIN_TYPE && actual_type <= MAX_TYPE) {
        advance(parser);
        if (!match(parser, T_IDENTIFIER)) {
            panic(ERR_SYNTAX, "Syntax error: you variable need a name");
        }
        char *var_name = strdup(get_token(parser->tokens, parser->current_index - 1)->lexeme);

        if (!match(parser, T_ASSIGN) && !match(parser, T_RASSIGN)) {
            panic(ERR_SYNTAX, "Syntax error: missing '='' or '<-' symbol");
        }
        TokenType assign_op = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *expr = expression_statement(parser);
        return create_var_decl_node(actual_type, var_name, expr, assign_op);
    }

    ast_node_t *expr = expression(parser);

    // cumple T_IDENTIFIER (T_ASSIGN | T_RASSIGN) expression
    if (expr->type == AST_IDENTIFIER && (match(parser, T_ASSIGN) || match(parser, T_RASSIGN))) {
        TokenType assign_op = get_token(parser->tokens, parser->current_index - 1)->type;
        ast_node_t *right_expr = expression_statement(parser);

        return create_assignment_node(expr->ast_node_value.identifier_expr.name, assign_op, right_expr);
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
        if ((actual_node->type != AST_BLOCK && actual_node->type != AST_IF_EXPR) && !match(parser, T_SEMICOLON) &&
            peek(parser)->type != T_EOF) {
            panic(ERR_SYNTAX, "Syntax error: ';' expected at the end of the statement");
        }
    }

    if (parser->flags & (DEBUG_MODE | PARSER_MODE)) {
        print_line_debug("== END OF PARSER ==");
        debug_ast(root, 0);
    }
    return root;
}