#include "wyrm_chunk.h"
#include "../../utils/error.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

const size_t INICIAL_CAPACITY = 16;
#define MASK4BYTES 0xFFFF
#define MASK2BYTES 0xFF
#define SHIFT8 8
#define WYRM_MAGIC_SIZE 4

static void read_exact(void *dest, size_t size, size_t count, FILE *file, const char *err_msg) {
    if (fread(dest, size, count, file) != count) {
        fclose(file);
        panic(ERR_IO, "Corrupted bytecode file: %s", err_msg);
    }
}

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

size_t chunk_emit_jump(wyrm_chunk_t *chunk, uint8_t opcode) {
    chunk_write_byte(chunk, opcode);
    size_t operand_offset = chunk->code.size;
    chunk_write_uint16(chunk, MASK4BYTES);
    return operand_offset;
}

void chunk_patch_jump(wyrm_chunk_t *chunk, size_t operand_offset) {
    uint16_t jump_target = (uint16_t)chunk->code.size;
    uint8_t *code_bytes = (uint8_t *)chunk->code.data;
    code_bytes[operand_offset] = (uint8_t)(jump_target & MASK2BYTES);
    code_bytes[operand_offset + 1] = (uint8_t)((jump_target >> SHIFT8) & MASK2BYTES);
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

wyrm_chunk_t *read_chunk_from_file(const char *input_path) {
    FILE *file = fopen(input_path, "rb");
    if (!file) {
        panic(ERR_FILE_NOT_FOUND, "Could not open bytecode file: %s", input_path);
    }

    char magic[WYRM_MAGIC_SIZE];
    uint16_t version = 0;
    read_exact(magic, sizeof(char), WYRM_MAGIC_SIZE, file, "missing magic header");
    if (memcmp(magic, WYRM_MAGIC, WYRM_MAGIC_SIZE) != 0) {
        fclose(file);
        panic(ERR_IO, "Invalid WYRM magic number in '%s'", input_path);
    }

    read_exact(&version, sizeof(uint16_t), 1, file, "missing version");
    if (version != WYRMC_VERSION) {
        fclose(file);
        panic(ERR_IO, "Unsupported bytecode version: 0x%04X", version);
    }

    wyrm_chunk_t *chunk = create_wyrm_chunk();

    // Constantes
    uint16_t const_count = 0;
    read_exact(&const_count, sizeof(uint16_t), 1, file, "const_count");
    for (uint16_t i = 0; i < const_count; i++) {
        uint8_t type_tag = 0;
        int64_t raw_val = 0;
        read_exact(&type_tag, sizeof(uint8_t), 1, file, "constant type");
        read_exact(&raw_val, sizeof(int64_t), 1, file, "constant value");

        wyrm_value_t val = {.type = (wyrm_value_type_t)type_tag, .value.int64_val = raw_val};
        chunk_add_constant(chunk, val);
    }

    // Funciones
    uint16_t func_count = 0;
    read_exact(&func_count, sizeof(uint16_t), 1, file, "func_count");
    for (uint16_t i = 0; i < func_count; i++) {
        wyrm_func_entry_t entry = {0};
        uint8_t ret_type = 0;
        read_exact(&entry.arg_count, sizeof(uint8_t), 1, file, "func arg_count");
        read_exact(&ret_type, sizeof(uint8_t), 1, file, "func return_type");
        read_exact(&entry.code_offset, sizeof(uint32_t), 1, file, "func code_offset");
        read_exact(&entry.code_length, sizeof(uint32_t), 1, file, "func code_length");
        entry.return_type = (wyrm_value_type_t)ret_type;
        vector_push(&chunk->functions, &entry);
    }

    // Code
    uint32_t code_size = 0;
    read_exact(&code_size, sizeof(uint32_t), 1, file, "code_size");
    for (uint32_t i = 0; i < code_size; i++) {
        uint8_t byte = 0;
        read_exact(&byte, sizeof(uint8_t), 1, file, "bytecode instruction");
        chunk_write_byte(chunk, byte);
    }

    fclose(file);
    return chunk;
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