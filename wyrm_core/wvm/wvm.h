#pragma once
#include "../wyrm_types/wyrm_value/wyrm_value.h"
#include "../wyrm_types/wyrmc_types/wyrm_chunk.h"

/*
Archivo contendedor de la logica de ejecucion de archivos compilados (.wrc).
*/

typedef struct WyrmVm {
    wyrm_chunk_t *chunk;
    uint32_t ip;
    wyrm_value_t *stack;
    size_t stack_top;
    size_t stack_capacity;
    int flags;
} wyrm_vm_t;

wyrm_value_t run_wvm(const char *file_path, int flags);