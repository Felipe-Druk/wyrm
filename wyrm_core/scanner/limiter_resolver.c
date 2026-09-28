#include "limiter_resolver.h"
#include <string.h>

int resolver_limiter(char *input, int index, token_vector_t *tokens) {
    char actual_char = input[index];
    wyrm_token_t limiter_token;

    switch (actual_char) {
    case ';':
        limiter_token.type = T_SEMICOLON;
        limiter_token.lexeme = strdup(";");
        break;
    case '(':
        limiter_token.type = T_LPAREN;
        limiter_token.lexeme = strdup("(");
        break;
    case ')':
        limiter_token.type = T_RPAREN;
        limiter_token.lexeme = strdup(")");
        break;
    case '{':
        limiter_token.type = T_LBRACE;
        limiter_token.lexeme = strdup("{");
        break;
    case '}':
        limiter_token.type = T_RBRACE;
        limiter_token.lexeme = strdup("}");
        break;
    default:
        return index; // no hay limitador
    }

    int result = push_token(tokens, limiter_token);
    return result == ERROR_TOKEN_VECTOR ? -1 : index + 1;
}
