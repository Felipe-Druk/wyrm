#include "limiter_resolver.h"

#define LEXEME_SEMICOLON ";"
const char SEMICOLON = ';';

int resolver_limiter(char* input, int index, token_vector_t* tokens){
    char actual_char = input[index];
    if(actual_char == SEMICOLON){
        wyrm_token_t type_token = { .type =T_SEMICOLON, .lexeme = LEXEME_SEMICOLON};
        int result = push_token(tokens, type_token);
        result == ERROR_TOKEN_VECTOR ? ERROR_TOKEN_VECTOR : index +1; 
    }
    return index;
}

