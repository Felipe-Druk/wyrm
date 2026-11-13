#pragma once
#include "wvm.h"
#include "../wyrm_types/wyrmc_types/wyrm_opcode.h"
#include "../utils/error.h"

// basicos para resolver cualquier operacion
void vm_push(wyrm_vm_t *vm, wyrm_value_t val);
wyrm_value_t vm_pop(wyrm_vm_t *vm);
uint8_t vm_read_byte(wyrm_vm_t *vm);
uint16_t vm_read_uint16(wyrm_vm_t *vm);
size_t vm_current_base_slot(wyrm_vm_t *vm);