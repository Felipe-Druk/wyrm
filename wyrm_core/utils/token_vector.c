
#include "token_vector.h"


token_vector_t* create_token_vector(int capacity);

int push_token(token_vector_t* vector, wyrm_token_t token);

wyrm_token_t* get_token(token_vector_t* vector, int index);

int clear_token_vector(token_vector_t* vector);

int destroy_token_vector(token_vector_t* vector);