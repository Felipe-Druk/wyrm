#include "wvm.h"
#include "wyrm_types/wyrmc_types/wyrm_chunk.h"
#include "wyrm_types/wyrmc_types/wyrm_opcode.h"
#include "utils/error.h"
#include <stdlib.h>

typedef struct WyrmVm {
    wyrm_chunk_t *chunk;
    uint32_t ip;
    wyrm_value_t *stack;
    size_t stack_top;
    size_t stack_capacity;
    int flags;
} wyrm_vm_t;

static void vm_push(wyrm_vm_t *vm, wyrm_value_t val) {
    if (vm->stack_top >= vm->stack_capacity) {
        vm->stack_capacity *= 2;
        vm->stack = realloc(vm->stack, sizeof(wyrm_value_t) * vm->stack_capacity);
        if (!vm->stack) {
            panic(ERR_OUT_MEMORY, "VM stack overflow");
        }
    }
    vm->stack[vm->stack_top++] = val;
}

static wyrm_value_t vm_pop(wyrm_vm_t *vm) {
    if (vm->stack_top == 0) {
        wyrm_value_t empty = {.type = VAL_VOID};
        return empty;
    }
    return vm->stack[--vm->stack_top];
}

static uint16_t vm_read_uint16(wyrm_vm_t *vm, const uint8_t *code) {
    uint16_t low = code[vm->ip++];
    uint16_t high = code[vm->ip++];
    return (uint16_t)(low | (high << 8));
}

// Castea el tope el valor del tope (para asignacion)
static wyrm_value_t cast_vm_value(wyrm_value_t val, wyrm_value_type_t target_type) {
    int64_t raw = val.value.int64_val; //  VAL_INT64 tipo por defecto de los literales
    wyrm_value_t res = {0};
    res.type = target_type;

    switch (target_type) {
    case VAL_NAT8:
        res.value.nat8_val = (uint8_t)raw;
        break;
    case VAL_NAT16:
        res.value.nat16_val = (uint16_t)raw;
        break;
    case VAL_NAT32:
        res.value.nat32_val = (uint32_t)raw;
        break;
    case VAL_NAT64:
        res.value.nat64_val = (uint64_t)raw;
        break;
    case VAL_INT8:
        res.value.int8_val = (int8_t)raw;
        break;
    case VAL_INT16:
        res.value.int16_val = (int16_t)raw;
        break;
    case VAL_INT32:
        res.value.int32_val = (int32_t)raw;
        break;
    case VAL_INT64:
        res.value.int64_val = (int64_t)raw;
        break;
    case VAL_BOOL:
        res.value.bool_val = (raw != 0);
        break;
    default:
        return val;
    }
    return res;
}

static wyrm_value_t vm_execute(wyrm_vm_t *vm) {
    const uint8_t *code = (const uint8_t *)vm->chunk->code.data;
    const wyrm_value_t *constants = (const wyrm_value_t *)vm->chunk->constants.data;

    while (vm->ip < vm->chunk->code.size) {
        uint8_t instruction = code[vm->ip++];

        switch ((wyrm_opcode_t)instruction) {
        case WYRM_OP_CONSTANT: {
            uint16_t const_idx = vm_read_uint16(vm, code);
            vm_push(vm, constants[const_idx]);
            break;
        }

        case WYRM_OP_CAST: {
            wyrm_value_type_t target_type = (wyrm_value_type_t)code[vm->ip++];
            wyrm_value_t val = vm_pop(vm);
            vm_push(vm, cast_vm_value(val, target_type));
            break;
        }

        case WYRM_OP_GET_LOCAL: {
            uint16_t slot = vm_read_uint16(vm, code);
            vm_push(vm, vm->stack[slot]);
            break;
        }

        case WYRM_OP_RETURN: {
            return vm_pop(vm);
        }

        default:
            panic(ERR_UNKNOWN_OP, "Unknown opcode in VM: 0x%02X", instruction);
        }
    }

    wyrm_value_t fallback = {.type = VAL_VOID};
    return fallback;
}

wyrm_value_t run_wvm(const char *file_path, int flags) {
    wyrm_chunk_t *chunk = read_chunk_from_file(file_path);

    wyrm_vm_t vm = {
        .chunk = chunk,
        .ip = 0,
        .stack = calloc(STACK_INITIAL_CAPACITY, sizeof(wyrm_value_t)),
        .stack_top = 0,
        .stack_capacity = STACK_INITIAL_CAPACITY,
        .flags = flags,
    };

    if (!vm.stack) {
        destroy_wyrm_chunk(chunk);
        panic(ERR_OUT_MEMORY, "Could not allocate VM stack");
    }

    wyrm_value_t result = vm_execute(&vm);

    free(vm.stack);
    destroy_wyrm_chunk(chunk);
    return result;
}