#include "wyrmc.h"
#include "utils/error.h"
#include "scanner/scanner.h"
#include "parser/parser.h"
#include "compiler/compiler.h"
#include "options.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

wyrmc_t *create_wyrmc(const char *input_file_path, const char *output_file_path, int flags) {
    wyrmc_t *wyrmc = malloc(sizeof(wyrmc_t));
    if (wyrmc == NULL) {
        panic(1, "Could not allocate memory for wyrmc_t");
    }
    if (flags) {
        wyrmc->flags = VERBOSE_MODE | PARSER_MODE | SCANNER_MODE | DEBUG_MODE;
    }
    wyrmc->flags = 0;
    wyrmc->input_file_path = strdup(input_file_path);
    wyrmc->output_file_path = strdup(output_file_path);
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
    scanner_t scanner = {.tokens = NULL, .flags = wyrmc->flags};
    char *input = input_file_to_string(wyrmc);
    token_vector_t *tokens = scanner_scan(input, &scanner);
    parser_t *parser = create_parser(tokens);
    if (wyrmc->flags & (PARSER_MODE | DEBUG_MODE)) {
        parser_set_flags(parser, wyrmc->flags);
    }
    ast_node_t *root = parser_parse(parser);
    compiler_t *compiler = create_compiler(root, wyrmc->flags);
    compile_ast(compiler);
    write_chunk_to_file(wyrmc->output_file_path, compiler->chunk);

    destroy_compiler(compiler);
    destroy_parser(parser);
    destroy_token_vector(tokens);
    free(input);
}

void set_wyrmc_flags(wyrmc_t *wyrmc, int flags) { wyrmc->flags = flags; }

void destroy_wyrmc(wyrmc_t *wyrmc) {
    if (wyrmc == NULL)
        return;
    free(wyrmc->input_file_path);
    free(wyrmc->output_file_path);
    free(wyrmc);
}