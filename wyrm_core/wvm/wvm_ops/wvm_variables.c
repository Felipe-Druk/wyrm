#include "wvm_variables.h"

void wvm_op_constant(wyrm_vm_t *vm) {
    uint16_t const_idx = vm_read_uint16(vm);
    vm_push(vm, vm->constants[const_idx]);
}

void wvm_op_cast(wyrm_vm_t *vm) {
    wyrm_value_type_t target_type = (wyrm_value_type_t)vm_read_byte(vm);
    wyrm_value_t *top = &vm->stack[vm->stack_top - 1];
    if (top->type != target_type) {
        *top = cast_to_value_type(*top, target_type);
    }
}

void wvm_op_get_local(wyrm_vm_t *vm) {
    uint16_t slot = vm_read_uint16(vm);
    vm_push(vm, vm->stack[vm->base_slot + slot]);
}

void wvm_op_set_local(wyrm_vm_t *vm) {
    uint16_t slot = vm_read_uint16(vm);
    wyrm_value_t new_val = vm_pop(vm);
    wyrm_value_t *dest = &vm->stack[vm->base_slot + slot];
    if (new_val.type == dest->type) {
        *dest = new_val;
    } else {
        *dest = cast_to_value_type(new_val, dest->type);
    }
}