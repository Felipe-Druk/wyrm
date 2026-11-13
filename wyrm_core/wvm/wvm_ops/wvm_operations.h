#pragma once
#include "../wvm_internal.h"

void wvm_op_binary_arith(wyrm_vm_t *vm, wyrm_opcode_t op);
void wvm_op_binary_logic(wyrm_vm_t *vm, wyrm_opcode_t op);
void wvm_op_binary_cmp(wyrm_vm_t *vm, wyrm_opcode_t op);
void wvm_op_not(wyrm_vm_t *vm);