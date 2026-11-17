#pragma once
#include "compiler.h"
#include "../utils/error.h"
#include "../wyrm_types/wyrmc_types/wyrm_opcode.h"
#include <string.h>

/*
Definimos internamente la fimra de "compile_node" para facilitar la recursion y esconder la interfaz de los modulos que
incluyan ""ompiler"
*/

#define ERRRESOLVER -1

// Compila el nodo segun su tipo,
void compile_node(ast_node_t *node, compiler_t *compiler);

// devuelve el indice de la variable, si esta en el "scope" local
int resolve_local(compiler_t *compiler, const char *name);

static inline int resolve_global(compiler_t *compiler, const char *name) {
    if (compiler->scope_level > 0 && compiler->globals.data != NULL) {
        wyrm_local_t *globals_arr = (wyrm_local_t *)compiler->globals.data;
        for (int i = (int)compiler->globals.size - 1; i >= 0; i--) {
            if (strcmp(globals_arr[i].name, name) == 0) {
                return i;
            }
        }
    }
    return ERRRESOLVER;
}

// devuleve el indcie de la función
int resolve_function(compiler_t *compiler, const char *name);

// Agrega una variable local
static inline uint16_t add_local_slot(compiler_t *compiler, char *name, wyrm_value_type_t type, int scope_offset) {
    uint16_t slot = (uint16_t)compiler->locals.size;
    wyrm_local_t local = {
        .name = name,
        .var_type = type,
        .scope_level = compiler->scope_level + scope_offset,
    };
    vector_push(&compiler->locals, &local);
    return slot;
}

// Escirbe un opcode antes de un slot
static inline void emit_local_op(wyrm_chunk_t *chunk, wyrm_opcode_t op, uint16_t slot) {
    chunk_write_byte(chunk, (uint8_t)op);
    chunk_write_uint16(chunk, slot);
}

// Agrega una constante
static inline void emit_constant_val(wyrm_chunk_t *chunk, wyrm_value_type_t type, int64_t val) {
    wyrm_value_t constant = {.type = type, .value.int64_val = val};
    uint16_t const_idx = chunk_add_constant(chunk, constant);
    chunk_write_byte(chunk, WYRM_OP_CONSTANT);
    chunk_write_uint16(chunk, const_idx);
}