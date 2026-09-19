#include "logical_resolver.h"

int resolve_equal(char* input, int index, token_vector_t* tokens) {
    wyrm_token_t token = { .type = T_ASSIGN, .lexeme = "=" };
    int new_index = index+1;
    if (input[new_index] == '='){
        new_index++;
    }

    int result = push_token(tokens, token);
    return result != ERROR_TOKEN_VECTOR ?  new_index : -1;
}
int resolve_less(char* input, int index, token_vector_t* tokens){
    int new_index = index+1;
    char nex = input[new_index];
     wyrm_token_t token;
    switch (nex)
    {
    case '-':
        token.type = T_RASSIGN;
        token.lexeme = "<-";
        new_index++;
        break;
    
    default:
        break;
    }
    int result = push_token(tokens, token);
    return result != ERROR_TOKEN_VECTOR ?  new_index : -1;
}

int resolver_logical_operator(char* input, int index, token_vector_t* tokens){
    switch (input[index])
    {
    case '=':
        return resolve_equal(input, index, tokens);
    case '<':
        return resolve_less(input, index, tokens);
        break;
    default:
        break;
    }
    return index;
}