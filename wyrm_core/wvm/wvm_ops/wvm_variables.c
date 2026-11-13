#include "wvm_variables.h"

void wvm_op_constant(wyrm_vm_t *vm) {
    uint16_t const_idx = vm_read_uint16(vm);
    const wyrm_value_t *constants = (const wyrm_value_t *)vm->chunk->constants.data;
    vm_push(vm, constants[const_idx]);
}

void wvm_op_cast(wyrm_vm_t *vm) {
    wyrm_value_type_t target_type = (wyrm_value_type_t)vm_read_byte(vm);
    wyrm_value_t val = vm_pop(vm);
    vm_push(vm, cast_to_value_type(val, target_type));
}

void wvm_op_get_local(wyrm_vm_t *vm) {
    uint16_t slot = vm_read_uint16(vm);
    size_t base = vm_current_base_slot(vm);
    vm_push(vm, vm->stack[base + slot]);
}

void wvm_op_set_local(wyrm_vm_t *vm) {
    uint16_t slot = vm_read_uint16(vm);
    size_t base = vm_current_base_slot(vm);
    wyrm_value_t new_val = vm_pop(vm);
    wyrm_value_type_t target_type = vm->stack[base + slot].type;
    vm->stack[base + slot] = cast_to_value_type(new_val, target_type);
}