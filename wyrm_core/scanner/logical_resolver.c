#include "logical_resolver.h"

int resolve_equal(char *input, int index, token_vector_t *tokens) {
    wyrm_token_t token = {.type = T_ASSIGN, .lexeme = "="};
    int new_index = index + 1;
    if (input[new_index] == '=') {
        token.type = T_EQUAL;
        token.lexeme = "==";
        new_index++;
    } else {
        token.type = T_ASSIGN;
        token.lexeme = "=";
    }

    int result = push_token(tokens, token);
    return result != ERROR_TOKEN_VECTOR ? new_index : -1;
}
int resolve_less(char *input, int index, token_vector_t *tokens) {
    int new_index = index + 1;
    char nex = input[new_index];
    wyrm_token_t token;
    switch (nex) {
    case '-':
        token.type = T_RASSIGN;
        token.lexeme = "<-";
        new_index++;
        break;
    case '=':
        token.type = T_LESS_EQUAL;
        token.lexeme = "<=";
        new_index++;
        break;
    default:
        token.type = T_LESS;
        token.lexeme = "<";
        break;
    }
    int result = push_token(tokens, token);
    return result != ERROR_TOKEN_VECTOR ? new_index : -1;
}

int resolve_greater(char *input, int index, token_vector_t *tokens) {
    int new_index = index + 1;
    wyrm_token_t token;

    if (input[new_index] == '=') {
        token.type = T_GREATER_EQUAL;
        token.lexeme = ">=";
        new_index++;
    } else {
        token.type = T_GREATER;
        token.lexeme = ">";
    }

    int result = push_token(tokens, token);
    return result != ERROR_TOKEN_VECTOR ? new_index : -1;
}

int resolve_not(char *input, int index, token_vector_t *tokens) {
    int new_index = index + 1;
    wyrm_token_t token;

    if (input[new_index] == '=') {
        token.type = T_NOT_EQUAL;
        token.lexeme = "~=";
        new_index++;
        int result = push_token(tokens, token);
        return result != ERROR_TOKEN_VECTOR ? new_index : -1;
    }

    return -1;
}

int resolver_logical_operator(char *input, int index, token_vector_t *tokens) {
    switch (input[index]) {
    case '=':
        return resolve_equal(input, index, tokens);
    case '<':
        return resolve_less(input, index, tokens);
        break;
    case '>':
        return resolve_greater(input, index, tokens);
    case '~':
        return resolve_not(input, index, tokens);
    default:
        break;
    }
    return index;
}