#include "variable_declaration.h"
#include "../parser_internal.h"
#include "../../utils/error.h"

token_type_t parse_vec_inner_type(parser_t *parser) {
    if (!match(parser, T_LESS)) {
        panic(ERR_SYNTAX, "Syntax error: expected '<' after 'vec'");
    }

    token_type_t inner = peek(parser)->type;
    if (inner < MIN_TYPE || inner > MAX_TYPE) {
        panic(ERR_SYNTAX, "Syntax error: expected a valid type inside 'vec<...>'");
    }
    advance(parser);

    if (inner == T_VEC) {
        parse_vec_inner_type(parser);
    }

    if (!match(parser, T_GREATER)) {
        panic(ERR_SYNTAX, "Syntax error: expected '>' to close 'vec<...>'");
    }

    return inner;
}

ast_node_t *variable_declaration(parser_t *parser) {
    token_type_t actual_type = advance(parser)->type;
    token_type_t element_type = T_EOF;

    if (actual_type == T_VEC) {
        element_type = parse_vec_inner_type(parser);
    }

    if (!match(parser, T_IDENTIFIER)) {
        panic(ERR_SYNTAX, "Syntax error: variable needs a name");
    }
    char *name = get_token(parser->tokens, parser->current_index - 1)->lexeme;

    if (peek(parser)->type == T_SEMICOLON || peek(parser)->type == T_EOF) {
        ast_node_t *node = create_var_decl_node(actual_type, name, NULL, T_SEMICOLON);
        node->ast_node_value.var_decl_expr.element_type = element_type;
        return node;
    }

    if (!match(parser, T_ASSIGN) && !match(parser, T_LARROW)) {
        panic(ERR_SYNTAX, "Syntax error: missing '=' or '<-' symbol");
    }

    token_type_t assign_op = get_token(parser->tokens, parser->current_index - 1)->type;
    ast_node_t *expr = expression_statement(parser);

    ast_node_t *node = create_var_decl_node(actual_type, name, expr, assign_op);
    node->ast_node_value.var_decl_expr.element_type = element_type;
    return node;
}