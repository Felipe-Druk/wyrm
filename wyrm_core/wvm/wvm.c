#include "wvm_internal.h"
#include <stdlib.h>
#include "wvm_ops/wvm_variables.h"
#include "wvm_ops/wvm_operations.h"
#include "wvm_control_flow.h"

void vm_push(wyrm_vm_t *vm, wyrm_value_t val) {
    if (vm->stack_top >= vm->stack_capacity) {
        vm->stack_capacity *= 2;
        vm->stack = realloc(vm->stack, sizeof(wyrm_value_t) * vm->stack_capacity);
        if (!vm->stack) {
            panic(ERR_OUT_MEMORY, "VM stack overflow");
        }
    }
    vm->stack[vm->stack_top++] = val;
}

wyrm_value_t vm_pop(wyrm_vm_t *vm) {
    if (vm->stack_top == 0) {
        wyrm_value_t empty = {.type = VAL_VOID};
        return empty;
    }
    return vm->stack[--vm->stack_top];
}

uint8_t vm_read_byte(wyrm_vm_t *vm) {
    uint8_t *byte_ptr = (uint8_t *)vector_get(&vm->chunk->code, vm->ip++);
    if (byte_ptr == NULL) {
        panic(ERR_OVERFLOW, "VM instruction pointer out of bounds: %u", vm->ip - 1);
    }
    return *byte_ptr;
}

uint16_t vm_read_uint16(wyrm_vm_t *vm) {
    uint16_t low = vm_read_byte(vm);
    uint16_t high = vm_read_byte(vm);
    return (uint16_t)(low | (high << 8));
}

size_t vm_current_base_slot(wyrm_vm_t *vm) {
    if (vm->frame_count == 0) {
        return 0;
    }
    return vm->frames[vm->frame_count - 1].base_slot;
}

// Castea el tope el valor del tope (para asignacion)
wyrm_value_t cast_vm_value(wyrm_value_t val, wyrm_value_type_t target_type) {
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

    while (vm->ip < vm->chunk->code.size) {
        uint8_t instruction = code[vm->ip++];

        switch (instruction) {
        case WYRM_OP_CONSTANT:
            wvm_op_constant(vm);
            break;
        case WYRM_OP_CAST:
            wvm_op_cast(vm);
            break;
        case WYRM_OP_GET_LOCAL:
            wvm_op_get_local(vm);
            break;
        case WYRM_OP_SET_LOCAL:
            wvm_op_set_local(vm);
            break;

        case WYRM_OP_ADD:
        case WYRM_OP_SUB:
        case WYRM_OP_MUL:
        case WYRM_OP_DIV:
        case WYRM_OP_MOD:
            wvm_op_binary_arith(vm, instruction);
            break;

        case WYRM_OP_AND:
        case WYRM_OP_OR:
            wvm_op_binary_logic(vm, instruction);
            break;

        case WYRM_OP_NOT:
            wvm_op_not(vm);
            break;

        case WYRM_OP_EQUAL:
        case WYRM_OP_LESS:
        case WYRM_OP_GREATER:
            wvm_op_binary_cmp(vm, instruction);
            break;
        case WYRM_OP_JUMP:
            wvm_op_jump(vm);
            break;

        case WYRM_OP_JUMP_IF:
            wvm_op_jump_if(vm);
            break;

        case WYRM_OP_JUMP_IF_NOT:
            wvm_op_jump_if_not(vm);
            break;
        case WYRM_OP_CALL:
            wvm_op_call(vm);
            break;
        case WYRM_OP_RETURN:
            wyrm_value_t final_result = {.type = VAL_VOID};
            if (wvm_op_return(vm, &final_result)) {
                return final_result;
            }
            break;

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