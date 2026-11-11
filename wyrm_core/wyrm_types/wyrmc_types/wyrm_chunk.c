#include "wyrm_chunk.h"
#include "../../utils/error.h"
#include <stdlib.h>

const size_t INICIAL_CAPACITY = 16;

wyrm_chunk_t *create_wyrm_chunk() {
    wyrm_chunk_t *chunk = malloc(sizeof(wyrm_chunk_t));
    if (chunk == NULL) {
        panic(ERR_OUT_MEMORY, "Could not allocate memory for wyrm_chunk_t");
    }
    chunk->code = create_vector(sizeof(uint8_t), INICIAL_CAPACITY);
    chunk->constants = create_vector(sizeof(wyrm_value_t), INICIAL_CAPACITY);
    chunk->functions = create_vector(sizeof(wyrm_func_entry_t), INICIAL_CAPACITY);
    chunk->lines = NULL; // Inicializamos a NULL, se asignará memoria cuando se agreguen líneas
    return chunk;
}

void chunk_write_byte(wyrm_chunk_t *chunk, uint8_t byte) { vector_push(&chunk->code, &byte); }

void chunk_write_uint16(wyrm_chunk_t *chunk, uint16_t value) {
    uint8_t low = (uint8_t)(value & 0xFF);
    uint8_t high = (uint8_t)((value >> 8) & 0xFF);
    chunk_write_byte(chunk, low);
    chunk_write_byte(chunk, high);
}

uint16_t chunk_add_constant(wyrm_chunk_t *chunk, wyrm_value_t value) {
    vector_push(&chunk->constants, &value);
    return (uint16_t)(chunk->constants.size - 1);
}

void destroy_wyrm_chunk(wyrm_chunk_t *chunk) {
    if (chunk == NULL)
        return;
    destroy_vector(&chunk->code);
    destroy_vector(&chunk->constants);
    destroy_vector(&chunk->functions);
    free(chunk->lines);
    free(chunk);
}