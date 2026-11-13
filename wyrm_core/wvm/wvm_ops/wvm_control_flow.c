#include "wvm_control_flow.h"

void wvm_op_jump(wyrm_vm_t *vm) {
    uint16_t target = vm_read_uint16(vm);
    vm->ip = target;
}

void wvm_op_jump_if(wyrm_vm_t *vm) {
    uint16_t target = vm_read_uint16(vm);
    wyrm_value_t condition = vm_pop(vm);

    if (condition.type != VAL_BOOL) {
        panic(ERR_TYPE_MISMATCH, "Condition in OP_JUMP_IF must be a boolean");
    }

    if (condition.value.bool_val) {
        vm->ip = target;
    }
}

void wvm_op_jump_if_not(wyrm_vm_t *vm) {
    uint16_t target = vm_read_uint16(vm);
    wyrm_value_t condition = vm_pop(vm);

    if (condition.type != VAL_BOOL) {
        panic(ERR_TYPE_MISMATCH, "Condition in OP_JUMP_IF_NOT must be a boolean");
    }

    if (!condition.value.bool_val) {
        vm->ip = target;
    }
}

void wvm_op_call(wyrm_vm_t *vm) {
    uint16_t func_id = vm_read_uint16(vm);
    wyrm_func_entry_t *func = (wyrm_func_entry_t *)vector_get(&vm->chunk->functions, func_id);
    if (func == NULL) {
        panic(ERR_UNDEFINED_VAR, "Invalid function ID in OP_CALL: %u", func_id);
    }

    if (vm->frame_count >= FRAMES_MAX) {
        panic(ERR_OVERFLOW, "VM CallFrame overflow (max recursion depth reached)");
    }

    // Nuevo Frame
    wyrm_call_frame_t *frame = &vm->frames[vm->frame_count++];
    frame->return_ip = vm->ip;
    frame->base_slot = vm->stack_top - func->arg_count;
    frame->return_type = func->return_type;

    // Salteamos a la base de la función
    vm->ip = func->code_offset;
}

bool wvm_op_return(wyrm_vm_t *vm, wyrm_value_t *final_result) {
    if (vm->frame_count == 0) {
        *final_result = vm_pop(vm);
        return true;
    }
    wyrm_call_frame_t frame = vm->frames[--vm->frame_count];

    wyrm_value_t ret_val = {.type = VAL_VOID};
    if (frame.return_type != VAL_VOID) {
        ret_val = vm_pop(vm);
        if (ret_val.type != frame.return_type) {
            ret_val = cast_to_value_type(ret_val, frame.return_type);
        }
    }

    vm->stack_top = frame.base_slot;
    vm_push(vm, ret_val);
    vm->ip = frame.return_ip;
    return false;
}