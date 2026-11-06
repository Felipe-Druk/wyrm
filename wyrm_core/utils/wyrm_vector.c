#include "wyrm_vector.h"
#include "error.h"
#include <stdlib.h>
#include <string.h>

#define FACTORCAPACITY 2

wyrm_vector_t create_vector(size_t element_size, size_t initial_capacity) {
    wyrm_vector_t vec;
    vec.data = malloc(element_size * initial_capacity);
    if (vec.data == NULL) {
        panic(ERR_OUT_MEMORY, "Could not allocate memory for vector data");
    }
    vec.size = 0;
    vec.capacity = initial_capacity;
    vec.element_size = element_size;
    return vec;
}

void vector_push(wyrm_vector_t *vec, const void *element) {

    if (vec->size >= vec->capacity) {
        size_t new_capacity = vec->capacity * FACTORCAPACITY;
        void *new_data = realloc(vec->data, new_capacity * vec->element_size);
        if (new_data == NULL) {
            panic(ERR_OUT_MEMORY, "Could not allocate memory for vector data");
        }
        vec->data = new_data;
        vec->capacity = new_capacity;
    }
    memcpy((char *)vec->data + (vec->size * vec->element_size), element, vec->element_size);
    vec->size++;
}

void *vector_get(const wyrm_vector_t *vec, size_t index) {
    if (index >= vec->size) {
        panic(ERR_INVALID_INDEX, "Index out of bounds");
    }
    return (char *)vec->data + (index * vec->element_size);
}

void destroy_vector(wyrm_vector_t *vec) {
    free(vec->data);
    vec->data = NULL;
    vec->size = 0;
    vec->capacity = 0;
}