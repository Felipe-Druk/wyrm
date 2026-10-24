#include "variable_declaration.h"
#include "parser_internal.h"
#include "../utils/error.h"

ast_node_t *variable_declaration(parser_t *parser) {
    token_type_t actual_type = advance(parser)->type;
    if (!match(parser, T_IDENTIFIER)) {
        panic(ERR_SYNTAX, "Syntax error: variable needs a name");
    }
    char *name = get_token(parser->tokens, parser->current_index - 1)->lexeme;
    if (peek(parser)->type == T_SEMICOLON || peek(parser)->type == T_EOF) {
        return create_var_decl_node(actual_type, name, NULL, T_SEMICOLON);
    }
    if (!match(parser, T_ASSIGN) && !match(parser, T_LARROW)) {
        panic(ERR_SYNTAX, "Syntax error: missing '=' or '<-' symbol");
    }

    token_type_t assign_op = get_token(parser->tokens, parser->current_index - 1)->type;
    ast_node_t *expr = expression_statement(parser);
    return create_var_decl_node(actual_type, name, expr, assign_op);
}