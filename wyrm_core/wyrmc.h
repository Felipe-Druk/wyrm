#pragma once

/*
Archivo wyrmc, genera archivos bytecode (.wrc) a partir de archivos fuente (.wyrm)
Internamente llama al compilador pero solo expone la interfaz de compilación de archivos fuente a bytecode.
*/

typedef struct {
    char *input_file_path;
    char *output_file_path;
    int flags;
} wyrmc_t;

wyrmc_t *create_wyrmc(const char *input_file_path, const char *output_file_path, int flags);

void compile_file(wyrmc_t *wyrmc);

void set_wyrmc_flags(wyrmc_t *wyrmc, int flags);

void destroy_wyrmc(wyrmc_t *wyrmc);