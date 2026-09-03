
#include "token_vector.h"


token_vector_t* create_token_vector(size_t capacity){
    
    token_vector_t* vector = malloc(sizeof(token_vector_t));
    if (vector == NULL) {
        return NULL;
    }
    vector->size = 0;
    vector->capacity = capacity;
    vector->tokens = malloc(capacity * sizeof(wyrm_token_t));
    if (vector->tokens == NULL) {
        free(vector);
        return NULL;
    }
    return vector;
}

int double_size_vector(token_vector_t* vector){
    vector->capacity *= 2;
    wyrm_token_t* new_tokens = realloc(vector->tokens, vector->capacity * sizeof(wyrm_token_t));
    if (new_tokens == NULL) {
        return ERROR_TOKEN_VECTOR;
    }
    vector->tokens = new_tokens;
    return 0;
}

int push_token(token_vector_t* vector, wyrm_token_t token){
    if (vector->size >= vector->capacity) {
        int result = double_size_vector(vector);
        if (result){
            return result;
        }
    }
    
    vector->tokens[vector->size++] = token;
    return 0;
}

wyrm_token_t* get_token(token_vector_t* vector, int index);

int clear_token_vector(token_vector_t* vector);

int destroy_token_vector(token_vector_t* vector){
    if (vector == NULL) {
        return ERROR_TOKEN_VECTOR;
    }
    free(vector->tokens);
    free(vector);
    return 0;
}