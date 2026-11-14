#pragma once
#include "../wvm_internal.h"

void wvm_op_constant(wyrm_vm_t *vm);
void wvm_op_cast(wyrm_vm_t *vm);
void wvm_op_get_local(wyrm_vm_t *vm);
void wvm_op_set_local(wyrm_vm_t *vm);

void wvm_op_build_vec(wyrm_vm_t *vm);
void wvm_op_vec_cast(wyrm_vm_t *vm);
void wvm_op_vec_get(wyrm_vm_t *vm);
void wvm_op_vec_set(wyrm_vm_t *vm);
void wvm_op_vec_len(wyrm_vm_t *vm);