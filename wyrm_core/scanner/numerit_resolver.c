#include "numerit_resolver.h"
#include <string.h>

int is_digit(char c) {
    return c >= '0' && c <= '9';
}

int chrage_number(int is_float, char* lexeme, token_vector_t* tokens) {
    wyrm_token_t number_token;
    number_token.type = is_float ?  T_FLOAT_NUMBER : T_NUMBER;
    number_token.lexeme = lexeme;
    int result = push_token(tokens, number_token);
    return result != ERROR_TOKEN_VECTOR ? 0 : -1;
}

int resolver_numerit(char* input, int index, token_vector_t* tokens){

    int is_float = 0;
    int temp_index = index;
    int valid_number = 1;

    while (input[temp_index] != '\0') {

        if (input[temp_index] == '.') {
            if (is_float) {
                // doble punto
                valid_number = 0;
                break;
            }
            is_float = 1;
        } else if (!is_digit(input[temp_index])) {
            break;
        }
        temp_index++;
    }
    if (!valid_number || temp_index == index) {
        return index; 
    }

    char* lexeme = strndup(input + index, temp_index - index);
    int result = chrage_number(is_float, lexeme, tokens);
    
    return temp_index;
}