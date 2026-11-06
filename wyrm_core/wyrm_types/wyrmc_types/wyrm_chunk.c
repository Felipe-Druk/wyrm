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