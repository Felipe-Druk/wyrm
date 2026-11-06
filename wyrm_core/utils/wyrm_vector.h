#pragma once

#include <stddef.h>

/*
C no tiene soporte nativo para vectors dinámicos, por lo que implementamos un vector genérico
que puede almacenar elementos de cualquier tipo, caracterizado por el puntero void
*/

// Estructura de datos para manejar un vector dinámico de elementos de cualquier tipo
typedef struct {
    void *data;
    size_t size;         // Cantidad de elementos almacenados
    size_t capacity;     // Capacidad actual en número de elementos
    size_t element_size; // Tamaño en bytes
} wyrm_vector_t;

// Inicializa el vector con el tamaño del tipo de dato y una capacidad inicial
wyrm_vector_t create_vector(size_t element_size, size_t initial_capacity);

// Inserta un elemento copiando sus bytes al final del arreglo
void vector_push(wyrm_vector_t *vec, const void *element);

// Obtiene un puntero directo al elemento en la posición 'index', el usuario debe hacer (void *) cast al tipo correcto
void *vector_get(const wyrm_vector_t *vec, size_t index);

// Libera la memoria del búfer interno
void destroy_vector(wyrm_vector_t *vec);