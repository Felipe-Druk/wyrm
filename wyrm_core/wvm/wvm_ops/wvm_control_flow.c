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