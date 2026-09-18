#include "identifier_resolver.h"

#include <string.h>
#include <ctype.h>

#define END_STRING '\0'

int charge_identifier(token_vector_t* tokens,const char* input, size_t new_index, size_t actual_index) {

    size_t length = new_index - actual_index;

    char* lexeme = malloc(sizeof(char) * length);
    if (lexeme == NULL) {
        return -1;
    }

    strncpy(lexeme, input + actual_index, length);

    lexeme[length] = '\0';

    wyrm_token_t type_token = { .type =T_IDENTIFIER, .lexeme = lexeme};
    int result = push_token(tokens, type_token);
    return result;
}

int resolver_identidier(char* input, int index, token_vector_t* tokens){
    size_t i = index;
    while(isalpha(input[i]) || input[i] == '_'){
       i++;
    }
    if(i !=  index){
        if (charge_identifier(tokens, input , i, index) >= 0){
            return i;
        }
    }
    return index;
}