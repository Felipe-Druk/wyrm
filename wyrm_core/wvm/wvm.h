#pragma once
#include "../wyrm_types/wyrm_value/wyrm_value.h"
#include "../wyrm_types/wyrmc_types/wyrm_chunk.h"

/*
Archivo contendedor de la logica de ejecucion de archivos compilados (.wrc).
*/

#define STACK_INITIAL_CAPACITY 1024
#define STACK_MAX_CAPACITY 65536
#define FRAMES_MAX 256

typedef struct wyrmCallFrame {
    uint32_t return_ip;            // Instrucción a la que debe volver tras OP_RETURN
    size_t base_slot;              // Posición en el stack donde empieza el argumento 0
    wyrm_value_type_t return_type; // Tipo de retorno declarado por la función
} wyrm_call_frame_t;

typedef struct WyrmVm {
    wyrm_chunk_t *chunk;
    uint32_t ip;
    wyrm_value_t *stack;
    size_t stack_top;
    size_t stack_capacity;

    wyrm_call_frame_t frames[FRAMES_MAX];
    size_t frame_count;

    int flags;
} wyrm_vm_t;

wyrm_value_t run_wvm(const char *file_path, int flags);