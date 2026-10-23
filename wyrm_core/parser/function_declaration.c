#include "function_declaration.h"
#include "../utils/error.h"
#include "parser_internal.h"
#include "block_statement.h"
#include <string.h>

static const size_t INITIAL_CAPACITY = 4;

void resize_function_args(wyrm_function_t *func_def, size_t new_capacity) {
    func_def->args = realloc(func_def->args, sizeof(wyrm_function_arg_t) * new_capacity);
    if (func_def->args == NULL) {
        panic(ERR_MEMORY, "Memory allocation failed while resizing function arguments");
    }
}

// luego de `(`, capturamos los tipos de los argumentos de la función ( TTPE1,TYPE2,...)
void capture_function_args_type(parser_t *parser, wyrm_function_t *func_def) {
    size_t capacity = INITIAL_CAPACITY;

    if (peek(parser)->type != T_RPAREN) {
        do {
            token_type_t actual_type = peek(parser)->type;
            if (actual_type >= MIN_TYPE && actual_type <= MAX_TYPE) {
                advance(parser);
                if (func_def->arg_count >= capacity) {
                    capacity *= 2;
                    resize_function_args(func_def, capacity);
                }
                func_def->args[func_def->arg_count].type = token_to_value_type(actual_type);
                func_def->args[func_def->arg_count].name = NULL;
                func_def->arg_count++;
            } else {
                panic(ERR_SYNTAX, "Syntax error: unexpected token in function argument types");
            }
        } while (match(parser, T_COOMA));
    }

    if (!match(parser, T_RPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: expected ')' after function argument types");
    }
}

// luego de "->" capturamos el tipo de retorno de la función
void capture_function_return_type(parser_t *parser, wyrm_function_t *func_def) {
    token_type_t actual_type = peek(parser)->type;
    if (actual_type >= MIN_TYPE && actual_type <= MAX_TYPE) {
        advance(parser);
        func_def->returned = token_to_value_type(actual_type);
    } else {
        panic(ERR_SYNTAX, "Syntax error: unexpected token in function return type");
    }
}

// luego de "nombre = (args) ->" capturamos los nombres de los argumentos de la función (ARG1,ARG2,...)
void capture_function_arg_names(parser_t *parser, wyrm_function_t *func_def) {
    if (!match(parser, T_LPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: expected '(' before function argument names");
    }

    size_t name_count = 0;
    if (peek(parser)->type != T_RPAREN) {
        do {
            if (!match(parser, T_IDENTIFIER)) {
                panic(ERR_SYNTAX, "Syntax error: function argument needs a name");
            }
            if (name_count >= func_def->arg_count) {
                panic(ERR_SYNTAX, "Syntax error: more argument names than types declared");
            }
            char *arg_name = get_token(parser->tokens, parser->current_index - 1)->lexeme;
            func_def->args[name_count].name = strdup(arg_name);
            name_count++;
        } while (match(parser, T_COOMA));
    }

    if (name_count != func_def->arg_count) {
        panic(ERR_SYNTAX, "Syntax error: argument count mismatch in function declaration");
    }

    if (!match(parser, T_RPAREN)) {
        panic(ERR_SYNTAX, "Syntax error: expected ')' after function argument names");
    }
}

ast_node_t *function_declaration(parser_t *parser) {
    wyrm_function_t func_def;
    func_def.returned = token_to_value_type(peek(parser)->type);
    func_def.arg_count = 0;
    func_def.args = calloc(INITIAL_CAPACITY, sizeof(wyrm_function_arg_t));

    capture_function_args_type(parser, &func_def);

    if (!match(parser, T_RARROW)) {
        panic(ERR_SYNTAX, "Syntax error: expected '->' after function arguments");
    }

    capture_function_return_type(parser, &func_def);
    if (!match(parser, T_IDENTIFIER)) {
        panic(ERR_SYNTAX, "Syntax error: function needs a name");
    }
    char *func_name = get_token(parser->tokens, parser->current_index - 1)->lexeme;

    if (!match(parser, T_ASSIGN) && !match(parser, T_LARROW)) {
        panic(ERR_SYNTAX, "Syntax error: expected '=' or '<-' after function name");
    }
    capture_function_arg_names(parser, &func_def);

    if (!match(parser, T_RARROW)) {
        panic(ERR_SYNTAX, "Syntax error: expected '->' before function body");
    }
    if (!match(parser, T_LBRACE)) {
        panic(ERR_SYNTAX, "Syntax error: expected '{' to start function body");
    }

    func_def.body = block_statement(parser);
    return create_function_decl_node(func_name, func_def);
}
