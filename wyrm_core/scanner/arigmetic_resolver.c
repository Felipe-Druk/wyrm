#include "arigmetic_resolver.h"

const int ERROR = -1; 

// por ahora no existe "++" asi que solo cargamos el token de add 
int rsolve_plus(char* input, int index, token_vector_t* tokens) {
    wyrm_token_t add_token = { .type = T_ADD, .lexeme = "+" };
    int result = push_token(tokens, add_token);
    return index + 1;
}



int resolver_arigmetic_operator(char* input, int index, token_vector_t* tokens){
    char current_char = input[index];

    switch (current_char) {
        case '+':
           int new_index = rsolve_plus(input, index, tokens);
            return new_index != ERROR ? new_index : ERROR;
            break;
        case '-':
            break;
        case '*':
            break;
        case '/':
            break;
        case '^':
            break;
        case '%':
            break;
        default:
            return index; // Error: unrecognized operator
    }
}




