#pragma once
#include "../wvm_internal.h"

void wvm_op_jump(wyrm_vm_t *vm);
void wvm_op_jump_if(wyrm_vm_t *vm);
void wvm_op_jump_if_not(wyrm_vm_t *vm);
void wvm_op_call(wyrm_vm_t *vm);
bool wvm_op_return(wyrm_vm_t *vm, wyrm_value_t *final_result);