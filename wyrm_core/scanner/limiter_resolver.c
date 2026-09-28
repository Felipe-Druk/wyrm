#include "limiter_resolver.h"
#include <string.h>

const char SEMICOLON = ';';

int resolver_limiter(char *input, int index, token_vector_t *tokens) {
    char actual_char = input[index];
    if (actual_char == SEMICOLON) {
        wyrm_token_t type_token = {.type = T_SEMICOLON, .lexeme = strdup(";")};
        int result = push_token(tokens, type_token);
        return result == ERROR_TOKEN_VECTOR ? -1 : index + 1;
    }
    return index;
}
