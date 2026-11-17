#include "char_resolver.h"
#include "../../utils/error.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

int resolve_string(char *input, int index, token_vector_t *tokens) {
    int end = index;
    while (input[end] != '\0' && input[end] != '"') {
        end++;
    }
    if (input[end] != '"') {
        panic(ERR_SYNTAX, "Expected closing single quote \" for character literal");
    }

    size_t len = (size_t)(end - index);
    char *str_lexeme = strndup(input + index, len);
    wyrm_token_t str_token = {.type = T_STRING, .lexeme = str_lexeme};
    if (push_token(tokens, str_token) == ERROR_TOKEN_VECTOR) {
        free(str_lexeme);
        panic(ERR_OUT_MEMORY, "Bad token in string resolver");
    }
    return end + 1;
}

int resolver_char(char *input, int index, token_vector_t *tokens) {
    if (input[index] == '"') {
        return resolve_string(input, index + 1, tokens);
    }

    if (input[index] != '\'') {
        return index;
    }
    int new_index = index + 1;
    if (input[new_index] == '\0') {
        panic(ERR_SYNTAX, "Unterminated character literal");
    }

    uint8_t char_val = 0;
    if (input[new_index] == '\\') {
        new_index++;
        switch (input[new_index]) {
        case '0':
            char_val = '\0';
            break;
        case 'n':
            char_val = '\n';
            break;
        case 't':
            char_val = '\t';
            break;
        case 'r':
            char_val = '\r';
            break;
        case '\\':
            char_val = '\\';
            break;
        case '\'':
            char_val = '\'';
            break;
        default:
            panic(ERR_SYNTAX, "Unknown escape sequence '\\%c'", input[new_index]);
        }
        new_index++;
    } else {
        char_val = (uint8_t)input[new_index];
        new_index++;
    }

    if (input[new_index] != '\'') {
        panic(ERR_SYNTAX, "Expected closing single quote ''' for character literal");
    }

    char buffer[8];
    snprintf(buffer, sizeof(buffer), "%u", char_val);

    wyrm_token_t token = {
        .type = T_NUMBER,
        .lexeme = strdup(buffer),
    };

    if (push_token(tokens, token) == ERROR_TOKEN_VECTOR) {
        free(token.lexeme);
        return -1;
    }
    new_index++; // cierra comilla simple
    return new_index;
}