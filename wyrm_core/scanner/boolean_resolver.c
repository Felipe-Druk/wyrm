#include "boolean_resolver.h" // Asegúrate de crear su correspondiente .h
#include <ctype.h>
#include <string.h>

const char *bool_keywords[] = {
    "true", "false", "and", "or", "not", "xor", "if", "else",
    NULL // Marca el final
};

// Vector de tokens types, respetando el orden de arriba
const TokenType bool_tokens[] = {T_TRUE, T_FALSE, T_AND, T_OR, T_NOT, T_XOR, T_IF, T_ELSE};

int charge_bool_keyword(token_vector_t *tokens, size_t index) {
    wyrm_token_t token = {.type = bool_tokens[index], .lexeme = strdup(bool_keywords[index])};
    return push_token(tokens, token);
}

int resolver_boolean(char *input, int index, token_vector_t *tokens) {
    size_t i = 0;
    size_t actual_len = 0;
    char nex_char;

    while (bool_keywords[i] != NULL) {
        actual_len = strlen(bool_keywords[i]);
        if (!strncmp(input + index, bool_keywords[i], actual_len)) {
            nex_char = input[index + actual_len];

            if (!isalnum(nex_char) && nex_char != '_') {
                if (charge_bool_keyword(tokens, i) != ERROR_TOKEN_VECTOR) {
                    return index + actual_len;
                }
                return -1;
            }
        }
        i++;
    }
    return index;
}