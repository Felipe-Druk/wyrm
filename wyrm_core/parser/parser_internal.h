#pragma once
#include "parser.h"

/*
Queremos "esconder" las funciones internas del parser,
Definimos las funciones que son utilizadas internamente por el parser, pero no queremos que sean visible desde fuera
del módulo Son static inline para que el compilador pueda optimizar su uso y evitar llamadas innecesarias, como son
funciones pequeñas son perfectas para ser inline
*/

// puede devolver NULL
static inline wyrm_token_t *peek(parser_t *parser) { return get_token(parser->tokens, parser->current_index); }

// avanza el parser al siguiente token y devuelve el token actual
static inline wyrm_token_t *advance(parser_t *parser) {
    wyrm_token_t *current = peek(parser);
    if (current->type != T_EOF) {
        parser->current_index++;
    }
    return current;
}

// avanza el parser al siguiente token y devuelve el token actual
static inline int match(parser_t *parser, TokenType expected_type_token) {
    if (peek(parser)->type == expected_type_token) {
        advance(parser);
        return 1;
    }
    return 0;
}
