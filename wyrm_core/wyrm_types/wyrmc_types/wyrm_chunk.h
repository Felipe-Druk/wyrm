#pragma once
#include "../wyrm_value/wyrm_value.h"
#include "../../utils/wyrm_vector.h"

typedef struct {
    uint8_t arg_count;             // Parámetros formales que espera la función
    wyrm_value_type_t return_type; // Tipo de retorno de la función
    uint32_t code_offset;          // Byte de code[] donde arranca la función
    uint32_t code_length;          // largo bytes de la función
} wyrm_func_entry_t;

typedef struct WyrmChunk {
    wyrm_vector_t code; // Arreglo dinámico de bytes de código
    // Pool de Constantes
    wyrm_vector_t constants; // Arreglo dinámico de valores constantes
    // Catálogo de Funciones declaradas
    wyrm_vector_t functions; // Arreglo dinámico de entradas de funciones
    // Información de depuración
    size_t *lines; // Número de línea en código .wyrm por cada byte de code[]
} wyrm_chunk_t;

// Reserva en moemoria espacio para un chunk
wyrm_chunk_t *create_wyrm_chunk();

// Agrega un byte al vector de bytes
void chunk_write_byte(wyrm_chunk_t *chunk, uint8_t byte);

// Agrega un los dos bytes al vector de bytes
void chunk_write_uint16(wyrm_chunk_t *chunk, uint16_t value);

// Agrega una constante al vector de constantes
uint16_t chunk_add_constant(wyrm_chunk_t *chunk, wyrm_value_t value);

// Libera toda la memoria reservada para el chunk
void destroy_wyrm_chunk(wyrm_chunk_t *chunk);