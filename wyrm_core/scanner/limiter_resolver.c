#include "limiter_resolver.h"
#include <string.h>
#include <stdbool.h>

static bool should_insert_semicolon(const char *input, int index, token_vector_t *tokens) {
    if (!tokens->size) {
        return false;
    }

    TokenType last = get_token(tokens, tokens->size - 1)->type;

    if (last == T_IDENTIFIER || last == T_NUMBER || last == T_FLOAT_NUMBER || last == T_STRING || last == T_TRUE ||
        last == T_FALSE) {
        return true;
    }

    if (last == T_RPAREN) {
        int next = index + 1;
        while (input[next] == ' ' || input[next] == '\t' || input[next] == '\r' || input[next] == '\n') {
            next++;
        }
        return !(input[next] == '{' || (input[next] == '-' && input[next + 1] == '>'));
    }

    return false;
}

int resolver_limiter(char *input, int index, token_vector_t *tokens) {
    TokenType type;
    const char *lexeme;

    switch (input[index]) {
    case '\n':
        if (!should_insert_semicolon(input, index, tokens)) {
            return index + 1;
        }
        // fallthrough
    case ';': // sin break por mismo comportamietno
        type = T_SEMICOLON;
        lexeme = ";";
        break;
    case '(':
        type = T_LPAREN;
        lexeme = "(";
        break;
    case ')':
        type = T_RPAREN;
        lexeme = ")";
        break;
    case '{':
        type = T_LBRACE;
        lexeme = "{";
        break;
    case '}':
        type = T_RBRACE;
        lexeme = "}";
        break;
    case ',':
        type = T_COOMA;
        lexeme = ",";
        break;
    default:
        return index; // No es un limitador
    }

    wyrm_token_t limiter_token = {.type = type, .lexeme = strdup(lexeme)};

    int result = push_token(tokens, limiter_token);
    return result == ERROR_TOKEN_VECTOR ? -1 : index + 1;
}
