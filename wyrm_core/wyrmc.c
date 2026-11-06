#include "wyrmc.h"
#include "utils/error.h"
#include "scanner/scanner.h"
#include "parser/parser.h"
#include "compiler/compiler.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define MAGICNUMBER "WYRM"

wyrmc_t *create_wyrmc(const char *input_file_path, const char *output_file_path, int flags) {
    wyrmc_t *wyrmc = malloc(sizeof(wyrmc_t));
    if (wyrmc == NULL) {
        panic(1, "Could not allocate memory for wyrmc_t");
    }
    wyrmc->input_file_path = strdup(input_file_path);
    wyrmc->output_file_path = strdup(output_file_path);
    wyrmc->flags = flags;
    return wyrmc;
}

char *input_file_to_string(wyrmc_t *wyrmc) {
    FILE *file = fopen(wyrmc->input_file_path, "rb");
    if (!file) {
        panic(ERR_FILE_NOT_FOUND, "Could not open input file '%s'", wyrmc->input_file_path);
    }

    // tamaño del archivo
    fseek(file, 0L, SEEK_END);
    size_t file_size = ftell(file);
    rewind(file);

    // Reservamos memoria +1 para el character nulo al final
    char *buffer = (char *)malloc(file_size + 1);
    if (!buffer) {
        fclose(file);
        panic(ERR_OUT_MEMORY, "Could not allocate memory for input file buffer");
    }

    size_t bytes_read = fread(buffer, sizeof(char), file_size, file);
    buffer[bytes_read] = '\0';

    fclose(file);
    return buffer;
}

void compile_file(wyrmc_t *wyrmc) {
    scanner_t scanner = {.tokens = create_token_vector(16), .flags = wyrmc->flags};
    char *input = input_file_to_string(wyrmc);
    token_vector_t *tokens = scanner_scan(input, &scanner);
    parser_t *parser = create_parser(tokens);
    ast_node_t *root = parser_parse(parser);
    compiler_t *compiler = create_compiler(root, wyrmc->flags);
    wyrm_chunk_t *chunk = compile_ast(compiler);
    (void)chunk; // Evita advertencias de variable no utilizada

    FILE *output_file = fopen(wyrmc->output_file_path, "wb");
    if (output_file == NULL) {
        free(input);
        panic(ERR_FILE_WRITE, "Could not open output file for writing");
    }
    fprintf(output_file, "%s", MAGICNUMBER); // Escribimos el magic number
    fclose(output_file);
    free(input);
}

void set_wyrmc_flags(wyrmc_t *wyrmc, int flags) {
    (void)wyrmc;
    (void)flags;
}

void destroy_wyrmc(wyrmc_t *wyrmc) {
    (void)wyrmc;
    free(wyrmc);
}