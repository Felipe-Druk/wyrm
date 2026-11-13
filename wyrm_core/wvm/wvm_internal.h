#pragma once
#include "wvm.h"
#include "../wyrm_types/wyrmc_types/wyrm_opcode.h"
#include "../utils/error.h"
#include <stdlib.h>

// basicos para resolver cualquier operacion

static inline void vm_push(wyrm_vm_t *vm, wyrm_value_t val) {
    if (__builtin_expect(vm->stack_top >= vm->stack_capacity, 0)) {
        if (vm->stack_capacity >= STACK_MAX_CAPACITY) {
            panic(ERR_OVERFLOW, "VM Stack Overflow");
        }
        vm->stack_capacity *= 2;
        vm->stack = realloc(vm->stack, sizeof(wyrm_value_t) * vm->stack_capacity);
        if (!vm->stack) {
            panic(ERR_OUT_MEMORY, "Could not expand VM stack");
        }
    }
    vm->stack[vm->stack_top++] = val;
}

static inline wyrm_value_t vm_pop(wyrm_vm_t *vm) {
    if (__builtin_expect(vm->stack_top == 0, 0)) {
        wyrm_value_t empty = {.type = VAL_VOID};
        return empty;
    }
    return vm->stack[--vm->stack_top];
}

static inline uint8_t vm_read_byte(wyrm_vm_t *vm) { return vm->code[vm->ip++]; }

static inline uint16_t vm_read_uint16(wyrm_vm_t *vm) {
    uint16_t low = vm->code[vm->ip];
    uint16_t high = vm->code[vm->ip + 1];
    vm->ip += 2;
    return (uint16_t)(low | (high << 8));
}

size_t vm_current_base_slot(wyrm_vm_t *vm);