#include "wyrmc.h"
#include "utils/error.h"
#include <stdlib.h>

wyrmc_t *create_wyrmc(const char *input_file_path, const char *output_file_path, int flags) {
    (void)input_file_path;
    (void)output_file_path;
    (void)flags;
    wyrmc_t *wyrmc = malloc(sizeof(wyrmc_t));
    if (wyrmc == NULL) {
        panic(1, "Could not allocate memory for wyrmc_t");
    }
    return wyrmc;
}

void compile_file(wyrmc_t *wyrmc) { (void)wyrmc; }

void set_wyrmc_flags(wyrmc_t *wyrmc, int flags) {
    (void)wyrmc;
    (void)flags;
}

void destroy_wyrmc(wyrmc_t *wyrmc) {
    (void)wyrmc;
    free(wyrmc);
}