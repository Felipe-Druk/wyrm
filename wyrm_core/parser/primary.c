#include "primary.h"
#include "parser_internal.h"
#include "../utils/error.h"

ast_node_t *call_expression(parser_t *parser, char *name) {
    size_t capacity = 4;
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
        char *name = previous->lexeme;
        if (match(parser, T_LPAREN)) {
            // cumple con call -> T_IDENTIFIER T_LPAREN (expression (T_COOMA expression)*)? T_RPAREN
            return call_expression(parser, name);
        }
        return create_identifier_node(name);
    }

    // Futuro error
    return NULL;
}