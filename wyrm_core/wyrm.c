#include "wyrm.h"
#include "scanner/scanner.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>



void run_wyrm(const char* input, int flags){
    char* input_copy = malloc(sizeof(char) * (strlen(input) + 1));
    if (input_copy == NULL) {
        fprintf(stderr, "Error: No se pudo asignar memoria para la copia de entrada.\n");
        return;
    }
    strcpy(input_copy, input);

    token_vector_t* tokens = scanner_scan(input_copy);
    if (tokens == NULL) {
        fprintf(stderr, "Error: No se pudo crear el vector de tokens.\n");
        free(input_copy);
        return;
    }
    
}