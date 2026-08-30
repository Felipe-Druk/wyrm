#pragma once

/*
Como vamos a manjear un vector de toknes, usamos una estrucura auxilar,
sombre todo para no lidar con problemas de dimencionamiento y memoria.
*/

#include "../wyrm_token.h"


const int ERROR_TOKEN_VECTOR = -1;

typedef struct {
    int size;
    int capacity;
    wyrm_token_t* tokens;
} TokenVector;

typedef TokenVector token_vector_t;


// Crea un vector de tokens con la capacidad especificada
token_vector_t* create_token_vector(int capacity);

// Agrega un token al vector, devuelve ERROR_TOKEN_VECTOR en caso de error
int push_token(token_vector_t* vector, wyrm_token_t token);

// Devuelve el token en la posicion index, devuelve NULL en caso de error
wyrm_token_t* get_token(token_vector_t* vector, int index);

// limpia el vector, devuelve ERROR_TOKEN_VECTOR en caso de error
int clear_token_vector(token_vector_t* vector);

// Destuye todos los elementos, devuleve ERROR_TOKEN_VECTOR en caso de error
int destroy_token_vector(token_vector_t* vector);