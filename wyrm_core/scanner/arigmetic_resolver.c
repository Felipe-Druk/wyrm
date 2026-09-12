#include "arigmetic_resolver.h"

const int ERROR = -1; 

// por ahora no existe "++" asi que solo cargamos el token de add 
int rsolve_plus(char* input, int index, token_vector_t* tokens) {
    wyrm_token_t add_token = { .type = T_ADD, .lexeme = "+" };
    int result = push_token(tokens, add_token);
    return result != ERROR_TOKEN_VECTOR ?  index + 1 : ERROR;
}

int resolve_minus(char* input, int index, token_vector_t* tokens) {
    wyrm_token_t minus_token = { .type = T_SUB, .lexeme = "-" };
    int result = push_token(tokens, minus_token);
    return result != ERROR_TOKEN_VECTOR ?  index + 1 : ERROR;
}

int resolve_multiply(char* input, int index, token_vector_t* tokens) {
    wyrm_token_t multiply_token = { .type = T_MUL, .lexeme = "*" };
    int result = push_token(tokens, multiply_token);
    return result != ERROR_TOKEN_VECTOR ?  index + 1 : ERROR;
}

// por ahora no existe "//" asi que solo cargamos el token de divide
int resolve_divide(char* input, int index, token_vector_t* tokens) {
    wyrm_token_t divide_token = { .type = T_DIV, .lexeme = "/" };
    int result = push_token(tokens, divide_token);
    return result != ERROR_TOKEN_VECTOR ?  index + 1 : ERROR;
}

int resolve_power(char* input, int index, token_vector_t* tokens) {
    wyrm_token_t power_token = { .type = T_POW, .lexeme = "^" };
    int result = push_token(tokens, power_token);
    return result != ERROR_TOKEN_VECTOR ?  index + 1 : ERROR;
}

int resolve_modulo(char* input, int index, token_vector_t* tokens) {
    wyrm_token_t modulo_token = { .type = T_MOD, .lexeme = "%" };
    int result = push_token(tokens, modulo_token);
    return result != ERROR_TOKEN_VECTOR ?  index + 1 : ERROR;
}

int new_index(char* input, int index, token_vector_t* tokens, int (*resolver)(char*, int, token_vector_t*)) {
    int result = resolver(input, index, tokens);
    return result != ERROR ? result : ERROR;
}

int resolver_arigmetic_operator(char* input, int index, token_vector_t* tokens){
    char current_char = input[index];

    switch (current_char) {
        case '+':
            return new_index(input, index, tokens, rsolve_plus);
        case '-':
            return new_index(input, index, tokens, resolve_minus);
        case '*':
            return new_index(input, index, tokens, resolve_multiply);
        case '/':
            return new_index(input, index, tokens, resolve_divide);
        case '^':
            return new_index(input, index, tokens, resolve_power);
        case '%':
            return new_index(input, index, tokens, resolve_modulo);
        default:
            return index; // si no hay nada en este indice no hacemos nada
    }
}




