#include "wyrmc.h"
#include "utils/error.h"
#include "scanner/scanner.h"
#include "parser/parser.h"
#include "compiler/compiler.h"
#include "options.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define WYRM_MAGIC "WYRM"
#define WYRM_MAGIC_SIZE 4
#define WYRMC_VERSION ((uint16_t)0x0100)

wyrmc_t *create_wyrmc(const char *input_file_path, const char *output_file_path, int flags) {
    wyrmc_t *wyrmc = malloc(sizeof(wyrmc_t));
    if (wyrmc == NULL) {
        panic(1, "Could not allocate memory for wyrmc_t");
    }
    if (flags) {
        wyrmc->flags = VERBOSE_MODE | PARSER_MODE | SCANNER_MODE | DEBUG_MODE;
    }
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

void write_chunk_to_file(const char *output_path, wyrm_chunk_t *chunk) {
    FILE *file = fopen(output_path, "wb");
    if (!file) {
        panic(ERR_IO, "Could not open output file: %s", output_path);
    }

    // Header
    const uint16_t version = WYRMC_VERSION;
    fwrite(WYRM_MAGIC, sizeof(char), WYRM_MAGIC_SIZE, file);
    fwrite(&version, sizeof(uint16_t), 1, file);

    // Constantes
    uint16_t const_count = (uint16_t)chunk->constants.size;
    fwrite(&const_count, sizeof(uint16_t), 1, file);

    wyrm_value_t *constants_arr = (wyrm_value_t *)chunk->constants.data;
    for (uint16_t i = 0; i < const_count; i++) {
        uint8_t type_tag = (uint8_t)constants_arr[i].type;
        fwrite(&type_tag, sizeof(uint8_t), 1, file);
        fwrite(&constants_arr[i].value.int64_val, sizeof(int64_t), 1, file);
    }

    // Funciones
    uint16_t func_count = (uint16_t)chunk->functions.size;
    fwrite(&func_count, sizeof(uint16_t), 1, file);

    wyrm_func_entry_t *funcs_arr = (wyrm_func_entry_t *)chunk->functions.data;
    for (uint16_t i = 0; i < func_count; i++) {
        uint8_t ret_type = (uint8_t)funcs_arr[i].return_type;
        fwrite(&funcs_arr[i].arg_count, sizeof(uint8_t), 1, file);
        fwrite(&ret_type, sizeof(uint8_t), 1, file);
        fwrite(&funcs_arr[i].code_offset, sizeof(uint32_t), 1, file);
        fwrite(&funcs_arr[i].code_length, sizeof(uint32_t), 1, file);
    }

    // Code
    uint32_t code_size = (uint32_t)chunk->code.size;
    fwrite(&code_size, sizeof(uint32_t), 1, file);
    if (code_size > 0) {
        fwrite(chunk->code.data, sizeof(uint8_t), code_size, file);
    }

    fclose(file);
}

void compile_file(wyrmc_t *wyrmc) {
    scanner_t scanner = {.tokens = create_token_vector(16), .flags = wyrmc->flags};
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
}

void set_wyrmc_flags(wyrmc_t *wyrmc, int flags) { wyrmc->flags = flags; }

void destroy_wyrmc(wyrmc_t *wyrmc) {
    (void)wyrmc;
    free(wyrmc);
}