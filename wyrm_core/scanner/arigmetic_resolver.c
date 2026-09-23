#include "arigmetic_resolver.h"

const int ERROR = -1;

// por ahora no existe "++" asi que solo cargamos el token de add
int rsolve_plus(char *input, int index, token_vector_t *tokens) {
    (void)input; // engañmos al linter, por ahora
    wyrm_token_t add_token = {.type = T_ADD, .lexeme = "+"};
    int result = push_token(tokens, add_token);
    return result != ERROR_TOKEN_VECTOR ? index + 1 : ERROR;
}

// en el futuro reconocera "->"
int resolve_minus(char *input, int index, token_vector_t *tokens) {
    (void)input; // engañmos al linter, por ahora
    wyrm_token_t minus_token = {.type = T_SUB, .lexeme = "-"};
    int result = push_token(tokens, minus_token);
    return result != ERROR_TOKEN_VECTOR ? index + 1 : ERROR;
}

int resolve_multiply(int index, token_vector_t *tokens) {
    wyrm_token_t multiply_token = {.type = T_MUL, .lexeme = "*"};
    int result = push_token(tokens, multiply_token);
    return result != ERROR_TOKEN_VECTOR ? index + 1 : ERROR;
}

int jump_line(const char *input, int index) {
    size_t new_index = index;
    while (input[new_index] != '\n' && input[new_index] != '\0') {
        new_index++;
    }
    return new_index;
}

int resolve_divide(char *input, int index, token_vector_t *tokens) {
    if (input[index + 1] == '/') {
        return jump_line(input, index); // simplemente ignoramos la linea si hay un comentario
    }
    wyrm_token_t divide_token = {.type = T_DIV, .lexeme = "/"};
    int result = push_token(tokens, divide_token);
    return result != ERROR_TOKEN_VECTOR ? index + 1 : ERROR;
}

int resolve_power(int index, token_vector_t *tokens) {
    wyrm_token_t power_token = {.type = T_POW, .lexeme = "^"};
    int result = push_token(tokens, power_token);
    return result != ERROR_TOKEN_VECTOR ? index + 1 : ERROR;
}

int resolve_modulo(int index, token_vector_t *tokens) {
    wyrm_token_t modulo_token = {.type = T_MOD, .lexeme = "%"};
    int result = push_token(tokens, modulo_token);
    return result != ERROR_TOKEN_VECTOR ? index + 1 : ERROR;
}

int resolver_arigmetic_operator(char *input, int index, token_vector_t *tokens) {
    char current_char = input[index];

    switch (current_char) {
    case '+':
        return rsolve_plus(input, index, tokens);
    case '-':
        return resolve_minus(input, index, tokens);
    case '*':
        return resolve_multiply(index, tokens);
    case '/':
        return resolve_divide(input, index, tokens);
    case '^':
        return resolve_power(index, tokens);
    case '%':
        return resolve_modulo(index, tokens);
    default:
        return index; // si no hay nada en este indice no hacemos nada
    }
}
