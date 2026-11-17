#include "wvm_variables.h"
#include <unistd.h>
#include <stdlib.h>

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

static void wvm_assign_at_index(wyrm_vm_t *vm, size_t target_idx) {
    wyrm_value_t new_val = vm_pop(vm);
    wyrm_value_t *dest = &vm->stack[target_idx];

    if (dest->type == VAL_VEC && dest->value.vec_val != NULL && new_val.type == VAL_VEC &&
        new_val.value.vec_val != NULL) {
        wyrm_value_type_t elem_type = dest->value.vec_val->element_type;
        wyrm_vec_t *new_vec = new_val.value.vec_val;
        if (elem_type != VAL_VOID) {
            new_vec->element_type = elem_type;
            for (size_t i = 0; i < new_vec->items.size; i++) {
                wyrm_value_t *item_ptr = (wyrm_value_t *)vector_get(&new_vec->items, i);
                *item_ptr = cast_to_value_type(*item_ptr, elem_type);
            }
        }
        *dest = new_val;
    } else if (new_val.type == dest->type || dest->type == VAL_VOID) {
        *dest = new_val;
    } else {
        *dest = cast_to_value_type(new_val, dest->type);
    }

    if (target_idx >= vm->stack_top) {
        vm->stack_top = target_idx + 1;
    }
}

void wvm_op_get_local(wyrm_vm_t *vm) {
    uint16_t slot = vm_read_uint16(vm);
    size_t base = vm_current_base_slot(vm);
    vm_push(vm, vm->stack[base + slot]);
}

void wvm_op_set_local(wyrm_vm_t *vm) {
    uint16_t slot = vm_read_uint16(vm);
    size_t base = vm_current_base_slot(vm);
    wvm_assign_at_index(vm, base + slot);
}

void wvm_op_get_global(wyrm_vm_t *vm) {
    uint16_t slot = vm_read_uint16(vm);
    vm_push(vm, vm->stack[slot]);
}

void wvm_op_set_global(wyrm_vm_t *vm) {
    uint16_t slot = vm_read_uint16(vm);
    wvm_assign_at_index(vm, (size_t)slot);
}

void wvm_op_build_vec(wyrm_vm_t *vm) {
    uint16_t count = vm_read_uint16(vm);
    wyrm_vec_t *vec = create_wyrm_vec(VAL_INT64, count);

    size_t start_idx = vm->stack_top - count;
    for (uint16_t i = 0; i < count; i++) {
        wyrm_value_t item = vm->stack[start_idx + i];
        if (i == 0) {
            vec->element_type = item.type;
        }
        vector_push(&vec->items, &item);
    }
    vm->stack_top -= count;

    wyrm_value_t res = {0};
    res.type = VAL_VEC;
    res.value.vec_val = vec;
    vm_push(vm, res);
}

void wvm_op_vec_cast(wyrm_vm_t *vm) {
    wyrm_value_type_t elem_type = (wyrm_value_type_t)vm_read_byte(vm);
    wyrm_value_t *top = &vm->stack[vm->stack_top - 1];
    if (top->type == VAL_VEC && top->value.vec_val != NULL && elem_type != VAL_VOID) {
        wyrm_vec_t *vec = top->value.vec_val;
        vec->element_type = elem_type;
        for (size_t i = 0; i < vec->items.size; i++) {
            wyrm_value_t *item_ptr = (wyrm_value_t *)vector_get(&vec->items, i);
            *item_ptr = cast_to_value_type(*item_ptr, elem_type);
        }
    }
}

void wvm_op_vec_get(wyrm_vm_t *vm) {
    wyrm_value_t idx_val = vm_pop(vm);
    wyrm_value_t vec_val = vm_pop(vm);

    if (vec_val.type != VAL_VEC || vec_val.value.vec_val == NULL) {
        panic(ERR_TYPE_MISMATCH, "Attempting to index a non-vector in VM");
    }

    int64_t idx = get_cast_int64(idx_val);
    wyrm_vec_t *vec = vec_val.value.vec_val;

    if (idx < 0 || (size_t)idx >= vec->items.size) {
        panic(ERR_OVERFLOW, "VM error: Vector index %lld out of bounds (size: %zu)", (long long)idx, vec->items.size);
    }

    wyrm_value_t elem = *(wyrm_value_t *)vector_get(&vec->items, (size_t)idx);
    vm_push(vm, elem);
}

void wvm_op_vec_set(wyrm_vm_t *vm) {
    wyrm_value_t new_val = vm_pop(vm);
    wyrm_value_t idx_val = vm_pop(vm);
    wyrm_value_t vec_val = vm_pop(vm);

    if (vec_val.type != VAL_VEC || vec_val.value.vec_val == NULL) {
        panic(ERR_TYPE_MISMATCH, "Attempting to assign to index of a non-vector in VM");
    }

    int64_t idx = get_cast_int64(idx_val);
    wyrm_vec_t *vec = vec_val.value.vec_val;

    if (idx < 0 || (size_t)idx >= vec->items.size) {
        panic(ERR_OVERFLOW, "VM error: Vector index %lld out of bounds (size: %zu)", (long long)idx, vec->items.size);
    }

    wyrm_value_t casted = cast_to_value_type(new_val, vec->element_type);
    wyrm_value_t *slot = (wyrm_value_t *)vector_get(&vec->items, (size_t)idx);
    *slot = casted;
}

void wvm_op_vec_len(wyrm_vm_t *vm) {
    wyrm_value_t vec_val = vm_pop(vm);
    if (vec_val.type != VAL_VEC || vec_val.value.vec_val == NULL) {
        panic(ERR_TYPE_MISMATCH, "OP_VEC_LEN requires a vector in VM");
    }
    wyrm_value_t len_val = {0};
    len_val.type = VAL_INT64;
    len_val.value.int64_val = (int64_t)vec_val.value.vec_val->items.size;
    vm_push(vm, len_val);
}

void wvm_op_write(wyrm_vm_t *vm) {
    wyrm_value_t buf_val = vm_pop(vm);
    wyrm_value_t fd_val = vm_pop(vm);

    if (buf_val.type != VAL_VEC || buf_val.value.vec_val == NULL) {
        panic(ERR_TYPE_MISMATCH, "Second argument to 'write' in VM must be a vector");
    }

    int fd = (int)get_cast_int64(fd_val);
    wyrm_vec_t *vec = buf_val.value.vec_val;
    size_t len = vec->items.size;

    uint8_t *raw_buf = NULL;
    if (len > 0) {
        raw_buf = malloc(len);
        for (size_t i = 0; i < len; i++) {
            wyrm_value_t item = *(wyrm_value_t *)vector_get(&vec->items, i);
            raw_buf[i] = (uint8_t)get_cast_int64(item);
        }
        if (raw_buf[len - 1] == 0) {
            len--;
        }
    }

    ssize_t written = (len > 0) ? write(fd, raw_buf, len) : 0;
    free(raw_buf);

    wyrm_value_t res = {
        .type = VAL_INT64,
        .value.int64_val = (int64_t)written,
    };
    vm_push(vm, res);
}

void wvm_op_read(wyrm_vm_t *vm) {
    wyrm_value_t count_val = vm_pop(vm);
    wyrm_value_t fd_val = vm_pop(vm);

    int fd = (int)get_cast_int64(fd_val);
    int64_t count = get_cast_int64(count_val);
    if (count < 0) {
        panic(ERR_OVERFLOW, "VM error: 'read' byte count cannot be negative");
    }

    uint8_t *raw_buf = NULL;
    ssize_t bytes_read = 0;
    if (count > 0) {
        raw_buf = malloc((size_t)count);
        if (raw_buf == NULL) {
            panic(ERR_OUT_MEMORY, "Could not allocate buffer for 'read' in VM");
        }
        bytes_read = read(fd, raw_buf, (size_t)count);
        if (bytes_read < 0) {
            bytes_read = 0;
        }
    }

    size_t initial_cap = (bytes_read > 0) ? (size_t)bytes_read : 1;
    wyrm_vec_t *vec = create_wyrm_vec(VAL_NAT8, initial_cap);
    for (ssize_t i = 0; i < bytes_read; i++) {
        wyrm_value_t item = {
            .type = VAL_NAT8,
            .value.nat8_val = raw_buf[i],
        };
        vector_push(&vec->items, &item);
    }
    free(raw_buf);

    wyrm_value_t res = {
        .type = VAL_VEC,
        .value.vec_val = vec,
    };
    vm_push(vm, res);
}