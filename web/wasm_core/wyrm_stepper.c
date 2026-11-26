#include <inttypes.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "compiler/compiler.h"
#include "options.h"
#include "parser/parser.h"
#include "scanner/scanner.h"
#include "utils/error.h"
#include "utils/wyrm_vector.h"
#include "wvm/wvm.h"
#include "wvm/wvm_internal.h"
#include "wvm/wvm_ops/wvm_control_flow.h"
#include "wvm/wvm_ops/wvm_operations.h"
#include "wvm/wvm_ops/wvm_variables.h"
#include "wyrm_types/wyrmc_types/wyrm_chunk.h"
#include "wyrm_types/wyrmc_types/wyrm_opcode.h"
#include "wyrm_wasm_common.h"

#define STEPPER_DEFAULT_MAX_STEPS 1500
#define STEPPER_HARD_MAX_STEPS 20000
#define STEPPER_MAX_HEAP_VECS 64
#define STEPPER_MAX_HEAP_ELEMS 64
#define STEPPER_INIT_JSON_CAP 16384

static char *g_trace_json = NULL;

static const char *lookup_func_name(const compiler_t *compiler, uint16_t func_id) {
    if (compiler == NULL || compiler->func_symbols.data == NULL) {
        return NULL;
    }
    const wyrm_func_symbol_t *syms = (const wyrm_func_symbol_t *)compiler->func_symbols.data;
    for (size_t i = 0; i < compiler->func_symbols.size; i++) {
        if (syms[i].func_id == func_id && syms[i].name != NULL && syms[i].name[0] != '\0') {
            return syms[i].name;
        }
    }
    return NULL;
}

static const char *lookup_script_local_name(const compiler_t *compiler, size_t slot_index) {
    if (compiler == NULL || compiler->locals.data == NULL) {
        return NULL;
    }
    if (slot_index >= compiler->locals.size) {
        return NULL;
    }
    const wyrm_local_t *locals = (const wyrm_local_t *)compiler->locals.data;
    if (locals[slot_index].name != NULL && locals[slot_index].name[0] != '\0') {
        return locals[slot_index].name;
    }
    return NULL;
}

static uint16_t read_u16_at(const uint8_t *code, size_t code_size, uint32_t offset) {
    if ((size_t)offset + 1 >= code_size) {
        return 0;
    }
    return (uint16_t)((uint16_t)code[offset] | ((uint16_t)code[offset + 1] << 8));
}

static const char *opcode_name(wyrm_opcode_t op) {
    switch (op) {
    case WYRM_OP_PUSH:
        return "OP_PUSH";
    case WYRM_OP_POP:
        return "OP_POP";
    case WYRM_OP_ADD:
        return "OP_ADD";
    case WYRM_OP_SUB:
        return "OP_SUB";
    case WYRM_OP_MUL:
        return "OP_MUL";
    case WYRM_OP_DIV:
        return "OP_DIV";
    case WYRM_OP_MOD:
        return "OP_MOD";
    case WYRM_OP_AND:
        return "OP_AND";
    case WYRM_OP_OR:
        return "OP_OR";
    case WYRM_OP_NOT:
        return "OP_NOT";
    case WYRM_OP_JUMP:
        return "OP_JUMP";
    case WYRM_OP_JUMP_IF:
        return "OP_JUMP_IF";
    case WYRM_OP_JUMP_IF_NOT:
        return "OP_JUMP_IF_NOT";
    case WYRM_OP_CONSTANT:
        return "OP_CONSTANT";
    case WYRM_OP_CAST:
        return "OP_CAST";
    case WYRM_OP_CALL:
        return "OP_CALL";
    case WYRM_OP_RETURN:
        return "OP_RETURN";
    case WYRM_OP_GET_LOCAL:
        return "OP_GET_LOCAL";
    case WYRM_OP_SET_LOCAL:
        return "OP_SET_LOCAL";
    case WYRM_OP_LESS:
        return "OP_LESS";
    case WYRM_OP_GREATER:
        return "OP_GREATER";
    case WYRM_OP_EQUAL:
        return "OP_EQUAL";
    case WYRM_OP_NOT_EQUAL:
        return "OP_NOT_EQUAL";
    case WYRM_OP_POW:
        return "OP_POW";
    case WYRM_OP_BUILD_VEC:
        return "OP_BUILD_VEC";
    case WYRM_OP_VEC_CAST:
        return "OP_VEC_CAST";
    case WYRM_OP_VEC_GET:
        return "OP_VEC_GET";
    case WYRM_OP_VEC_SET:
        return "OP_VEC_SET";
    case WYRM_OP_VEC_LEN:
        return "OP_VEC_LEN";
    case WYRM_OP_WRITE:
        return "OP_WRITE";
    case WYRM_OP_READ:
        return "OP_READ";
    case WYRM_OP_GET_GLOBAL:
        return "OP_GET_GLOBAL";
    case WYRM_OP_SET_GLOBAL:
        return "OP_SET_GLOBAL";
    default:
        return "OP_UNKNOWN";
    }
}

static uint32_t decode_instruction(
    const wyrm_chunk_t *chunk,
    const compiler_t *compiler,
    uint32_t ip,
    const char **out_opcode,
    char *operand_buf,
    size_t operand_cap
) {
    operand_buf[0] = '\0';
    const uint8_t *code = (const uint8_t *)chunk->code.data;
    size_t code_size = chunk->code.size;
    if (ip >= code_size) {
        *out_opcode = "OP_EOF";
        return 1;
    }

    wyrm_opcode_t op = (wyrm_opcode_t)code[ip];
    *out_opcode = opcode_name(op);

    switch (op) {
    case WYRM_OP_CONSTANT: {
        uint16_t idx = read_u16_at(code, code_size, ip + 1);
        if (idx < chunk->constants.size) {
            const wyrm_value_t *cval = (const wyrm_value_t *)vector_get(&chunk->constants, idx);
            char val_preview[96];
            wyrm_web_format_value(*cval, val_preview, sizeof(val_preview));
            snprintf(operand_buf, operand_cap, "#%u (%s)", (unsigned int)idx, val_preview);
        } else {
            snprintf(operand_buf, operand_cap, "#%u", (unsigned int)idx);
        }
        return 3;
    }
    case WYRM_OP_CAST: {
        uint8_t raw_type = ((size_t)ip + 1 < code_size) ? code[ip + 1] : 0;
        snprintf(operand_buf, operand_cap, "%s", wyrm_web_type_name((wyrm_value_type_t)raw_type));
        return 2;
    }
    case WYRM_OP_VEC_CAST: {
        uint8_t raw_type = ((size_t)ip + 1 < code_size) ? code[ip + 1] : 0;
        snprintf(operand_buf, operand_cap, "vec<%s>", wyrm_web_type_name((wyrm_value_type_t)raw_type));
        return 2;
    }
    case WYRM_OP_JUMP:
    case WYRM_OP_JUMP_IF:
    case WYRM_OP_JUMP_IF_NOT: {
        uint16_t target = read_u16_at(code, code_size, ip + 1);
        snprintf(operand_buf, operand_cap, "-> 0x%04X", (unsigned int)target);
        return 3;
    }
    case WYRM_OP_CALL: {
        uint16_t func_id = read_u16_at(code, code_size, ip + 1);
        const char *fname = lookup_func_name(compiler, func_id);
        if (fname != NULL) {
            snprintf(operand_buf, operand_cap, "fn#%u (%s)", (unsigned int)func_id, fname);
        } else {
            snprintf(operand_buf, operand_cap, "fn#%u", (unsigned int)func_id);
        }
        return 3;
    }
    case WYRM_OP_GET_LOCAL:
    case WYRM_OP_SET_LOCAL: {
        uint16_t slot = read_u16_at(code, code_size, ip + 1);
        snprintf(operand_buf, operand_cap, "slot[%u]", (unsigned int)slot);
        return 3;
    }
    case WYRM_OP_GET_GLOBAL:
    case WYRM_OP_SET_GLOBAL: {
        uint16_t slot = read_u16_at(code, code_size, ip + 1);
        const char *gname = lookup_script_local_name(compiler, slot);
        if (gname != NULL) {
            snprintf(operand_buf, operand_cap, "global[%u] (%s)", (unsigned int)slot, gname);
        } else {
            snprintf(operand_buf, operand_cap, "global[%u]", (unsigned int)slot);
        }
        return 3;
    }
    case WYRM_OP_BUILD_VEC: {
        uint16_t count = read_u16_at(code, code_size, ip + 1);
        snprintf(operand_buf, operand_cap, "len=%u", (unsigned int)count);
        return 3;
    }
    default:
        return 1;
    }
}

static size_t chunk_line_at(const wyrm_chunk_t *chunk, uint32_t ip) {
    if (chunk == NULL || chunk->lines == NULL || ip >= chunk->code.size) {
        return 0;
    }
    return chunk->lines[ip];
}

static size_t slot_frame_depth(const wyrm_vm_t *vm, size_t slot_index) {
    size_t depth = 0;
    for (size_t f = 0; f < vm->frame_count; f++) {
        if (slot_index >= vm->frames[f].base_slot) {
            depth = f + 1;
        }
    }
    return depth;
}

static void collect_vec_unique(
    const wyrm_vec_t *vec,
    const wyrm_vec_t **list,
    size_t *count
) {
    if (vec == NULL || *count >= STEPPER_MAX_HEAP_VECS) {
        return;
    }
    for (size_t i = 0; i < *count; i++) {
        if (list[i] == vec) {
            return;
        }
    }
    list[*count] = vec;
    (*count)++;

    if (vec->element_type == VAL_VEC) {
        size_t lim = vec->items.size > STEPPER_MAX_HEAP_ELEMS
            ? STEPPER_MAX_HEAP_ELEMS
            : vec->items.size;
        for (size_t i = 0; i < lim && *count < STEPPER_MAX_HEAP_VECS; i++) {
            const wyrm_value_t *item = (const wyrm_value_t *)vector_get(&vec->items, i);
            if (item != NULL && item->type == VAL_VEC && item->value.vec_val != NULL) {
                collect_vec_unique(item->value.vec_val, list, count);
            }
        }
    }
}

static void emit_step_json(
    json_buf_t *jb,
    const wyrm_vm_t *vm,
    const compiler_t *compiler,
    const uint16_t *frame_func_ids,
    int step_idx,
    uint32_t ip_before,
    uint32_t ip_after,
    const char *op_str,
    const char *operand_str,
    size_t line,
    size_t stdout_len
) {
    jbuf_appendf(
        jb,
        "{\"step\":%d,\"ip_before\":%" PRIu32 ",\"ip_after\":%" PRIu32 ",\"opcode\":",
        step_idx,
        ip_before,
        ip_after
    );
    jbuf_append_escaped_str(jb, op_str);
    jbuf_append(jb, ",\"operand\":");
    jbuf_append_escaped_str(jb, operand_str);
    jbuf_appendf(jb, ",\"line\":%zu,\"stdout_len\":%zu,\"frames\":[", line, stdout_len);

    // Frame raiz <script> en depth 0
    jbuf_append(
        jb,
        "{\"depth\":0,\"name\":\"<script>\",\"base_slot\":0,\"return_ip\":null,\"return_type\":\"void\"}"
    );
    for (size_t f = 0; f < vm->frame_count; f++) {
        const wyrm_call_frame_t *fr = &vm->frames[f];
        char name_buf[96];
        const char *fname = lookup_func_name(compiler, frame_func_ids[f]);
        if (fname != NULL) {
            snprintf(name_buf, sizeof(name_buf), "%s", fname);
        } else {
            snprintf(name_buf, sizeof(name_buf), "fn#%u", (unsigned int)frame_func_ids[f]);
        }
        jbuf_appendf(jb, ",{\"depth\":%zu,\"name\":", f + 1);
        jbuf_append_escaped_str(jb, name_buf);
        jbuf_appendf(
            jb,
            ",\"base_slot\":%zu,\"return_ip\":%" PRIu32 ",\"return_type\":",
            fr->base_slot,
            fr->return_ip
        );
        jbuf_append_escaped_str(jb, wyrm_web_type_name(fr->return_type));
        jbuf_append_char(jb, '}');
    }

    // Stack
    jbuf_append(jb, "],\"stack\":[");
    const wyrm_vec_t *heap_vecs[STEPPER_MAX_HEAP_VECS];
    size_t heap_count = 0;

    char type_buf[128];
    char val_buf[512];
    char hex_buf[64];
    char ptr_buf[40];

    for (size_t i = 0; i < vm->stack_top; i++) {
        if (i > 0) {
            jbuf_append_char(jb, ',');
        }
        wyrm_value_t slot_val = vm->stack[i];
        size_t fdepth = slot_frame_depth(vm, i);
        const char *slot_name = (fdepth == 0) ? lookup_script_local_name(compiler, i) : NULL;

        wyrm_web_format_type(slot_val, type_buf, sizeof(type_buf));
        wyrm_web_format_value(slot_val, val_buf, sizeof(val_buf));
        int bits = wyrm_web_value_bits(slot_val);
        wyrm_web_format_hex(slot_val, hex_buf, sizeof(hex_buf));

        jbuf_appendf(jb, "{\"index\":%zu,\"frame_depth\":%zu,\"name\":", i, fdepth);
        if (slot_name != NULL) {
            jbuf_append_escaped_str(jb, slot_name);
        } else {
            jbuf_append(jb, "null");
        }
        jbuf_append(jb, ",\"type\":");
        jbuf_append_escaped_str(jb, type_buf);
        jbuf_append(jb, ",\"value\":");
        jbuf_append_escaped_str(jb, val_buf);
        jbuf_appendf(jb, ",\"bits\":%d,\"hex\":", bits);
        jbuf_append_escaped_str(jb, hex_buf);
        jbuf_append(jb, ",\"vec_ptr\":");

        if (slot_val.type == VAL_VEC && slot_val.value.vec_val != NULL) {
            snprintf(ptr_buf, sizeof(ptr_buf), "0x%" PRIxPTR, (uintptr_t)slot_val.value.vec_val);
            jbuf_append_escaped_str(jb, ptr_buf);
            collect_vec_unique(slot_val.value.vec_val, heap_vecs, &heap_count);
        } else {
            jbuf_append(jb, "null");
        }
        jbuf_append_char(jb, '}');
    }

    // Heap deduplicado
    jbuf_append(jb, "],\"heap\":[");
    char elem_val[256];
    char elem_hex[64];
    char ascii_buf[512];

    for (size_t h = 0; h < heap_count; h++) {
        if (h > 0) {
            jbuf_append_char(jb, ',');
        }
        const wyrm_vec_t *vec = heap_vecs[h];
        snprintf(ptr_buf, sizeof(ptr_buf), "0x%" PRIxPTR, (uintptr_t)vec);

        jbuf_append(jb, "{\"ptr\":");
        jbuf_append_escaped_str(jb, ptr_buf);
        jbuf_append(jb, ",\"elem_type\":");
        jbuf_append_escaped_str(jb, wyrm_web_type_name(vec->element_type));
        jbuf_appendf(jb, ",\"len\":%zu,\"ascii\":", vec->items.size);

        if (vec->element_type == VAL_NAT8) {
            wyrm_web_format_nat8_ascii(vec, ascii_buf, sizeof(ascii_buf));
            jbuf_append_escaped_str(jb, ascii_buf);
        } else {
            jbuf_append(jb, "null");
        }

        jbuf_append(jb, ",\"elements\":[");
        size_t shown = vec->items.size > STEPPER_MAX_HEAP_ELEMS
            ? STEPPER_MAX_HEAP_ELEMS
            : vec->items.size;
        for (size_t e = 0; e < shown; e++) {
            if (e > 0) {
                jbuf_append_char(jb, ',');
            }
            const wyrm_value_t *item = (const wyrm_value_t *)vector_get(&vec->items, e);
            if (item != NULL) {
                wyrm_web_format_value(*item, elem_val, sizeof(elem_val));
                wyrm_web_format_hex(*item, elem_hex, sizeof(elem_hex));
            } else {
                elem_val[0] = '\0';
                elem_hex[0] = '\0';
            }
            jbuf_appendf(jb, "{\"index\":%zu,\"value\":", e);
            jbuf_append_escaped_str(jb, elem_val);
            jbuf_append(jb, ",\"hex\":");
            jbuf_append_escaped_str(jb, elem_hex);
            jbuf_append_char(jb, '}');
        }
        jbuf_append(jb, "]}");
    }

    jbuf_append(jb, "]}");
}

static void emit_chunk_disassembly(
    json_buf_t *jb,
    const wyrm_chunk_t *chunk,
    const compiler_t *compiler
) {
    jbuf_append(jb, "\"instructions\":[");
    uint32_t ip = 0;
    bool first = true;
    const char *op_str = NULL;
    char operand_buf[160];

    while (ip < chunk->code.size) {
        if (!first) {
            jbuf_append_char(jb, ',');
        }
        first = false;
        uint32_t size = decode_instruction(chunk, compiler, ip, &op_str, operand_buf, sizeof(operand_buf));
        size_t line = chunk_line_at(chunk, ip);

        jbuf_appendf(jb, "{\"ip\":%" PRIu32 ",\"opcode\":", ip);
        jbuf_append_escaped_str(jb, op_str);
        jbuf_append(jb, ",\"operand\":");
        jbuf_append_escaped_str(jb, operand_buf);
        jbuf_appendf(jb, ",\"line\":%zu}", line);

        ip += (size > 0) ? size : 1;
    }

    jbuf_append(jb, "],\"constants\":[");
    char ctype_buf[128];
    char cval_buf[256];
    for (size_t i = 0; i < chunk->constants.size; i++) {
        if (i > 0) {
            jbuf_append_char(jb, ',');
        }
        const wyrm_value_t *cval = (const wyrm_value_t *)vector_get(&chunk->constants, i);
        wyrm_web_format_type(*cval, ctype_buf, sizeof(ctype_buf));
        wyrm_web_format_value(*cval, cval_buf, sizeof(cval_buf));
        jbuf_appendf(jb, "{\"index\":%zu,\"type\":", i);
        jbuf_append_escaped_str(jb, ctype_buf);
        jbuf_append(jb, ",\"value\":");
        jbuf_append_escaped_str(jb, cval_buf);
        jbuf_append_char(jb, '}');
    }
    jbuf_append(jb, "],");
}

EMSCRIPTEN_KEEPALIVE
const char *wyrm_web_trace_json(void) {
    return (g_trace_json != NULL) ? g_trace_json : "";
}

EMSCRIPTEN_KEEPALIVE
int wyrm_web_trace(const char *source, int max_steps) {
    free(g_trace_json);
    g_trace_json = NULL;
    wyrm_web_clear_last_return();

    if (source == NULL) {
        fprintf(stderr, "Error: código fuente nulo.\n");
        fflush(stdout);
        fflush(stderr);
        return 1;
    }

    if (max_steps <= 0) {
        max_steps = STEPPER_DEFAULT_MAX_STEPS;
    } else if (max_steps > STEPPER_HARD_MAX_STEPS) {
        max_steps = STEPPER_HARD_MAX_STEPS;
    }

    free_all_wyrm_vecs();

    char *input = strdup(source);
    if (input == NULL) {
        fprintf(stderr, "Error: sin memoria para copiar el código fuente.\n");
        fflush(stderr);
        return 1;
    }

    scanner_t scanner = {.tokens = NULL, .flags = 0};
    token_vector_t *tokens = scanner_scan(input, &scanner);
    parser_t *parser = create_parser(tokens);
    ast_node_t *root = parser_parse(parser);
    compiler_t *compiler = create_compiler(root, 0);
    compiler->globals = (wyrm_vector_t){0};
    compile_ast(compiler);

    wyrm_chunk_t *chunk = get_compiled_chunk(compiler);

    wyrm_vm_t vm = {
        .chunk = chunk,
        .code = chunk->code.size > 0 ? (const uint8_t *)vector_get(&chunk->code, 0) : NULL,
        .code_size = chunk->code.size,
        .constants = chunk->constants.size > 0 ? (const wyrm_value_t *)vector_get(&chunk->constants, 0) : NULL,
        .ip = 0,
        .stack = (wyrm_value_t *)calloc(STACK_INITIAL_CAPACITY, sizeof(wyrm_value_t)),
        .stack_top = 0,
        .stack_capacity = STACK_INITIAL_CAPACITY,
        .base_slot = 0,
        .frame_count = 0,
        .flags = 0,
    };

    if (vm.stack == NULL) {
        destroy_compiler(compiler);
        destroy_parser(parser);
        destroy_token_vector(tokens);
        free(input);
        fprintf(stderr, "Error: sin memoria para el stack de la VM.\n");
        fflush(stderr);
        return 1;
    }

    uint16_t frame_func_ids[FRAMES_MAX];
    memset(frame_func_ids, 0, sizeof(frame_func_ids));

    json_buf_t jb;
    jbuf_init(&jb, STEPPER_INIT_JSON_CAP);
    jbuf_append_char(&jb, '{');
    emit_chunk_disassembly(&jb, chunk, compiler);
    jbuf_append(&jb, "\"steps\":[");

    // Paso 0: estado inicial previo a la primera instrucción
    emit_step_json(
        &jb,
        &vm,
        compiler,
        frame_func_ids,
        0,
        0,
        0,
        "INIT",
        "",
        chunk_line_at(chunk, 0),
        0
    );

    int step_count = 0;
    bool truncated = false;
    bool halted = false;
    size_t stdout_len = 0;
    wyrm_value_t final_result = {.type = VAL_VOID};

    while (vm.ip < vm.code_size && !halted) {
        if (step_count >= max_steps) {
            truncated = true;
            break;
        }

        uint32_t ip_before = vm.ip;
        const char *op_str = NULL;
        char operand_buf[160];
        decode_instruction(chunk, compiler, ip_before, &op_str, operand_buf, sizeof(operand_buf));
        size_t line = chunk_line_at(chunk, ip_before);

        wyrm_opcode_t instruction = (wyrm_opcode_t)vm_read_byte(&vm);
        uint16_t call_func_id = 0;
        int write_fd = -1;

        if (instruction == WYRM_OP_CALL) {
            call_func_id = read_u16_at(vm.code, vm.code_size, ip_before + 1);
        } else if (instruction == WYRM_OP_WRITE && vm.stack_top >= 2) {
            write_fd = (int)get_cast_int64(vm.stack[vm.stack_top - 2]);
        }

        switch (instruction) {
        case WYRM_OP_POP:
            vm_pop(&vm);
            break;
        case WYRM_OP_CONSTANT:
            wvm_op_constant(&vm);
            break;
        case WYRM_OP_CAST:
            wvm_op_cast(&vm);
            break;
        case WYRM_OP_GET_LOCAL:
            wvm_op_get_local(&vm);
            break;
        case WYRM_OP_SET_LOCAL:
            wvm_op_set_local(&vm);
            break;
        case WYRM_OP_GET_GLOBAL:
            wvm_op_get_global(&vm);
            break;
        case WYRM_OP_SET_GLOBAL:
            wvm_op_set_global(&vm);
            break;
        case WYRM_OP_BUILD_VEC:
            wvm_op_build_vec(&vm);
            break;
        case WYRM_OP_VEC_CAST:
            wvm_op_vec_cast(&vm);
            break;
        case WYRM_OP_VEC_GET:
            wvm_op_vec_get(&vm);
            break;
        case WYRM_OP_VEC_SET:
            wvm_op_vec_set(&vm);
            break;
        case WYRM_OP_VEC_LEN:
            wvm_op_vec_len(&vm);
            break;
        case WYRM_OP_ADD:
        case WYRM_OP_SUB:
        case WYRM_OP_MUL:
        case WYRM_OP_DIV:
        case WYRM_OP_MOD:
        case WYRM_OP_POW:
            wvm_op_binary_arith(&vm, instruction);
            break;
        case WYRM_OP_AND:
        case WYRM_OP_OR:
            wvm_op_binary_logic(&vm, instruction);
            break;
        case WYRM_OP_NOT:
            wvm_op_not(&vm);
            break;
        case WYRM_OP_EQUAL:
        case WYRM_OP_LESS:
        case WYRM_OP_GREATER:
            wvm_op_binary_cmp(&vm, instruction);
            break;
        case WYRM_OP_JUMP:
            wvm_op_jump(&vm);
            break;
        case WYRM_OP_JUMP_IF:
            wvm_op_jump_if(&vm);
            break;
        case WYRM_OP_JUMP_IF_NOT:
            wvm_op_jump_if_not(&vm);
            break;
        case WYRM_OP_CALL:
            wvm_op_call(&vm);
            if (vm.frame_count > 0 && vm.frame_count <= FRAMES_MAX) {
                frame_func_ids[vm.frame_count - 1] = call_func_id;
            }
            break;
        case WYRM_OP_WRITE:
            wvm_op_write(&vm);
            if (write_fd == 1 && vm.stack_top > 0) {
                int64_t written = vm.stack[vm.stack_top - 1].value.int64_val;
                if (written > 0) {
                    stdout_len += (size_t)written;
                }
                fflush(stdout);
            }
            break;
        case WYRM_OP_READ:
            wvm_op_read(&vm);
            break;
        case WYRM_OP_RETURN:
            if (wvm_op_return(&vm, &final_result)) {
                halted = true;
            }
            break;
        default:
            panic(ERR_UNKNOWN_OP, "Unknown opcode in VM stepper: 0x%02X", (unsigned int)instruction);
        }

        step_count++;
        jbuf_append_char(&jb, ',');
        emit_step_json(
            &jb,
            &vm,
            compiler,
            frame_func_ids,
            step_count,
            ip_before,
            vm.ip,
            op_str,
            operand_buf,
            line,
            stdout_len
        );
    }

    jbuf_appendf(
        &jb,
        "],\"truncated\":%s,\"max_steps\":%d,\"total_steps\":%d}",
        truncated ? "true" : "false",
        max_steps,
        step_count
    );

    // Capturar retorno ANTES de liberar vectores de la VM
    wyrm_web_set_last_return(final_result);

    free_all_wyrm_vecs();
    free(vm.stack);
    destroy_compiler(compiler);
    destroy_parser(parser);
    destroy_token_vector(tokens);
    free(input);

    if (jb.oom) {
        jbuf_free(&jb);
        fprintf(stderr, "Error: sin memoria al serializar la traza JSON.\n");
        fflush(stderr);
        return 1;
    }

    g_trace_json = jb.data;
    fflush(stdout);
    fflush(stderr);
    return 0;
}
