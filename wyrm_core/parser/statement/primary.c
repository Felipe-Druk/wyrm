#include "primary.h"
#include "../parser_internal.h"
#include "../../utils/error.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define DEFAULTCAPACITY 4

ast_node_t *call_expression(parser_t *parser, char *name) {
    size_t capacity = DEFAULTCAPACITY;
    size_t arg_count = 0;
    ast_node_t **args = malloc(sizeof(ast_node_t *) * capacity);
    if (peek(parser)->type != T_RPAREN) {
        do {
            if (arg_count >= capacity) {
                capacity *= 2;
                args = realloc(args, sizeof(ast_node_t *) * capacity);
            }
            args[arg_count++] = expression(parser);
        } while (match(parser, T_COOMA));
    }

    if (!match(parser, T_RPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: Expected ')' after arguments");
    }

    return create_call_node(name, arg_count, args);
}

static ast_node_t *vec_literal_expression(parser_t *parser) {
    ast_node_t *vec_node = create_block_node(DEFAULTCAPACITY);
    vec_node->type = AST_VEC_LITERAL;

    if (peek(parser)->type != T_RBRACKET) {
        do {
            if (block_is_full(&vec_node->ast_node_value.block_expr)) {
                resize_block(&vec_node->ast_node_value.block_expr);
            }
            vec_node->ast_node_value.block_expr.nodes[vec_node->ast_node_value.block_expr.size++] = expression(parser);
        } while (match(parser, T_COOMA));
    }

    if (!match(parser, T_RBRACKET)) {
        panic(ERR_SYNTAX, "Syntax error: Expected ']' after vector elements");
    }

    return vec_node;
}

// permite vec[i], matriz[i][j], ..
static ast_node_t *parse_postfix_index(parser_t *parser, ast_node_t *expr) {
    while (expr != NULL && match(parser, T_LBRACKET)) {
        ast_node_t *index_expr = expression(parser);
        if (!match(parser, T_RBRACKET)) {
            panic(ERR_SYNTAX, "Syntax error: Expected ']' after vector index");
        }
        expr = create_binary_node(expr, T_LBRACKET, index_expr);
        expr->type = AST_VEC_ACCESS;
    }
    return expr;
}

static ast_node_t *string_literal_expression(const char *str) {
    size_t len = strlen(str);
    ast_node_t *vec_node = create_block_node(len + 1);
    vec_node->type = AST_VEC_LITERAL;

    char num_buf[8];
    for (size_t i = 0; i < len; i++) {
        snprintf(num_buf, sizeof(num_buf), "%u", (unsigned char)str[i]);
        vec_node->ast_node_value.block_expr.nodes[vec_node->ast_node_value.block_expr.size++] =
            create_number_node(num_buf, T_NUMBER);
    }

    vec_node->ast_node_value.block_expr.nodes[vec_node->ast_node_value.block_expr.size++] =
        create_number_node("0", T_NUMBER); // Agrega un 0 al final del string

    return vec_node;
}

ast_node_t *primary(parser_t *parser) {

    if (match(parser, T_TRUE)) {
        return create_bool_node(true);
    }
    if (match(parser, T_FALSE)) {
        return create_bool_node(false);
    }

    if (match(parser, T_LBRACKET)) {
        ast_node_t *vec_lit = vec_literal_expression(parser);
        return parse_postfix_index(parser, vec_lit);
    }

    if (match(parser, T_LPAREN)) {
        ast_node_t *expr = expression(parser);
        if (!match(parser, T_RPAREN)) {
            panic(ERR_SYNTAX, "Syntax error: Expected ')' after expression");
        }
        return parse_postfix_index(parser, expr);
    }

    if (match(parser, T_NUMBER) || match(parser, T_FLOAT_NUMBER)) {
        wyrm_token_t *previous = get_token(parser->tokens, parser->current_index - 1);
        return create_number_node(previous->lexeme, previous->type);
    }

    if (match(parser, T_IDENTIFIER)) {
        wyrm_token_t *previous = get_token(parser->tokens, parser->current_index - 1);
        char *name = previous->lexeme;
        ast_node_t *expr = NULL;
        if (match(parser, T_LPAREN)) {
            // cumple con call -> T_IDENTIFIER T_LPAREN (expression (T_COOMA expression)*)? T_RPAREN
            expr = call_expression(parser, name);
        } else {
            expr = create_identifier_node(name);
        }
        return parse_postfix_index(parser, expr);
    }

    if (match(parser, T_STRING)) {
        wyrm_token_t *previous = get_token(parser->tokens, parser->current_index - 1);
        ast_node_t *str_vec = string_literal_expression(previous->lexeme);
        return parse_postfix_index(parser, str_vec);
    }

    // Futuro error
    return NULL;
}