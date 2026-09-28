#include "parser.h"
#include "../options.h"
#include "debugger_parser.h"
#include "../utils/error.h"
#include <stdlib.h>

const size_t INITIAL_CAPACITY = 5;

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
    if (match(parser, T_SUB)) {
        ast_node_t *right_expr = unary(parser); // para resolver cadena de negadores
        return create_unary_node(T_SUB, right_expr);
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

ast_node_t *expression(parser_t *parser) {
    ast_node_t *term_expr = term(parser);
    return term_expr;
}

ast_node_t *expression_statement(parser_t *parser) {
    ast_node_t *expr = expression(parser);

    return expr;
}

ast_node_t *parser_parse(parser_t *parser) {
    // Arriba los especiales

    ast_node_t *root = create_block_node(INITIAL_CAPACITY);

    while (peek(parser)->type != T_EOF) {
        ast_node_t *actual_node = expression_statement(parser);
        if (block_is_full(&root->ast_node_value.block_expr)) {
            resize_block(&root->ast_node_value.block_expr);
        }
        root->ast_node_value.block_expr.nodes[root->ast_node_value.block_expr.size++] = actual_node;
        if (!match(parser, T_SEMICOLON) && peek(parser)->type != T_EOF) {
            panic(ERR_SYNTAX, "Syntax error: ';' expected at the end of the statement");
        }
    }

    if (parser->flags & (DEBUG_MODE | PARSER_MODE)) {
        print_line_debug("== END OF PARSER ==");
        debug_ast(root, 0);
    }
    return root;
}