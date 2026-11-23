#include <inttypes.h>
#include <math.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef __EMSCRIPTEN__
#include <emscripten/emscripten.h>
#else
#define EMSCRIPTEN_KEEPALIVE
#endif

#include "options.h"
#include "utils/wyrm_vector.h"
#include "wvm/wvm.h"
#include "wyrm.h"
#include "wyrm_types/wyrm_value/wyrm_value.h"
#include "wyrm_wasm_internal.h"
#include "wyrmc.h"

#define WYRM_WEB_RET_BUF_SIZE 4096
#define WYRM_WEB_TYPE_BUF_SIZE 128
#define WYRM_WEB_MAX_VEC_ITEMS 64
#define WYRM_WEB_MAX_STR_PREVIEW 256
#define WYRM_WEB_MAX_DEPTH 4
#define WYRM_WEB_TMP_SRC "/tmp/main.wyrm"
#define WYRM_WEB_TMP_WRC "/tmp/main.wrc"

static char g_last_return[WYRM_WEB_RET_BUF_SIZE];
static char g_last_return_type[WYRM_WEB_TYPE_BUF_SIZE];

static const char *value_type_name(wyrm_value_type_t type) {
    switch (type) {
    case VAL_VOID:
        return "void";
    case VAL_NAT8:
        return "nat8";
    case VAL_NAT16:
        return "nat16";
    case VAL_NAT32:
        return "nat32";
    case VAL_NAT64:
        return "nat64";
    case VAL_INT8:
        return "int8";
    case VAL_INT16:
        return "int16";
    case VAL_INT32:
        return "int32";
    case VAL_INT64:
        return "int64";
    case VAL_FLOAT32:
        return "float32";
    case VAL_FLOAT64:
        return "float64";
    case VAL_BOOL:
        return "bool";
    case VAL_FUNCTION:
        return "function";
    case VAL_VEC:
        return "vec";
    default:
        return "unknown";
    }
}

static void buf_append(char *buf, size_t cap, size_t *offset, const char *fmt, ...) {
    if (*offset >= cap) {
        return;
    }

    va_list args;
    va_start(args, fmt);
    int written = vsnprintf(buf + *offset, cap - *offset, fmt, args);
    va_end(args);

    if (written <= 0) {
        return;
    }
    if ((size_t)written >= cap - *offset) {
        *offset = cap - 1;
    } else {
        *offset += (size_t)written;
    }
}

static void format_wyrm_type_inner(
    wyrm_value_t val,
    char *buf,
    size_t cap,
    size_t *offset,
    int depth
) {
    if (val.type != VAL_VEC) {
        buf_append(buf, cap, offset, "%s", value_type_name(val.type));
        return;
    }

    const wyrm_vec_t *vec = val.value.vec_val;
    if (vec == NULL || depth >= WYRM_WEB_MAX_DEPTH) {
        buf_append(buf, cap, offset, "vec");
        return;
    }

    buf_append(buf, cap, offset, "vec<");
    if (vec->element_type == VAL_VEC && vec->items.size > 0) {
        const wyrm_value_t *first = (const wyrm_value_t *)vector_get(&vec->items, 0);
        if (first != NULL && first->type == VAL_VEC) {
            format_wyrm_type_inner(*first, buf, cap, offset, depth + 1);
        } else {
            buf_append(buf, cap, offset, "%s", value_type_name(vec->element_type));
        }
    } else {
        buf_append(buf, cap, offset, "%s", value_type_name(vec->element_type));
    }
    buf_append(buf, cap, offset, ">");
}

static void format_wyrm_type(wyrm_value_t val, char *buf, size_t cap) {
    if (cap == 0) {
        return;
    }
    buf[0] = '\0';
    size_t offset = 0;
    format_wyrm_type_inner(val, buf, cap, &offset, 0);
}

static void format_float_cli(double value, bool is_f32, char *buf, size_t cap, size_t *offset) {
    if (isnan(value)) {
        buf_append(buf, cap, offset, "NaN");
        return;
    }
    if (isinf(value)) {
        buf_append(buf, cap, offset, value < 0.0 ? "-inf" : "inf");
        return;
    }
    if (value == 0.0) {
        buf_append(buf, cap, offset, signbit(value) ? "-0" : "0");
        return;
    }
    buf_append(buf, cap, offset, is_f32 ? "%.7g" : "%.15g", value);
}

static bool is_printable_nat8_string(const wyrm_vec_t *vec) {
    if (vec == NULL || vec->element_type != VAL_NAT8 || vec->items.size < 2) {
        return false;
    }

    const wyrm_value_t *last = (const wyrm_value_t *)vector_get(&vec->items, vec->items.size - 1);
    if (last == NULL || last->type != VAL_NAT8 || last->value.nat8_val != 0) {
        return false;
    }

    for (size_t i = 0; i + 1 < vec->items.size; i++) {
        const wyrm_value_t *item = (const wyrm_value_t *)vector_get(&vec->items, i);
        if (item == NULL || item->type != VAL_NAT8) {
            return false;
        }
        uint8_t byte = item->value.nat8_val;
        bool printable = (byte >= 32 && byte <= 126) || byte == '\n' || byte == '\t' || byte == '\r';
        if (!printable) {
            return false;
        }
    }
    return true;
}

static void format_nat8_string(const wyrm_vec_t *vec, char *buf, size_t cap, size_t *offset) {
    size_t content_len = vec->items.size > 0 ? vec->items.size - 1 : 0;
    size_t shown = content_len > WYRM_WEB_MAX_STR_PREVIEW ? WYRM_WEB_MAX_STR_PREVIEW : content_len;

    buf_append(buf, cap, offset, "\"");
    for (size_t i = 0; i < shown; i++) {
        const wyrm_value_t *item = (const wyrm_value_t *)vector_get(&vec->items, i);
        uint8_t byte = item->value.nat8_val;
        if (byte == '\n') {
            buf_append(buf, cap, offset, "\\n");
        } else if (byte == '\t') {
            buf_append(buf, cap, offset, "\\t");
        } else if (byte == '\r') {
            buf_append(buf, cap, offset, "\\r");
        } else if (byte == '"' || byte == '\\') {
            buf_append(buf, cap, offset, "\\%c", (char)byte);
        } else {
            buf_append(buf, cap, offset, "%c", (char)byte);
        }
    }
    if (content_len > shown) {
        buf_append(buf, cap, offset, "...");
    }
    buf_append(buf, cap, offset, "\"");
}

/*
 * Formatea `wyrm_value_t` replicando `impl fmt::Display for WyrmValue` de `src/interpreter.rs`
 * (sin sufijo de tipo, con `∅` para VAL_VOID, enteros/naturales/floats/bool idénticos al CLI)
 * y extendiendo la vista de vectores y funciones para el Playground.
 */
static void format_wyrm_value_inner(
    wyrm_value_t val,
    char *buf,
    size_t cap,
    size_t *offset,
    int depth
) {
    switch (val.type) {
    case VAL_VOID:
        buf_append(buf, cap, offset, "\xE2\x88\x85");
        break;
    case VAL_INT8:
        buf_append(buf, cap, offset, "%" PRId8, val.value.int8_val);
        break;
    case VAL_INT16:
        buf_append(buf, cap, offset, "%" PRId16, val.value.int16_val);
        break;
    case VAL_INT32:
        buf_append(buf, cap, offset, "%" PRId32, val.value.int32_val);
        break;
    case VAL_INT64:
        buf_append(buf, cap, offset, "%" PRId64, val.value.int64_val);
        break;
    case VAL_NAT8:
        buf_append(buf, cap, offset, "%" PRIu8, val.value.nat8_val);
        break;
    case VAL_NAT16:
        buf_append(buf, cap, offset, "%" PRIu16, val.value.nat16_val);
        break;
    case VAL_NAT32:
        buf_append(buf, cap, offset, "%" PRIu32, val.value.nat32_val);
        break;
    case VAL_NAT64:
        buf_append(buf, cap, offset, "%" PRIu64, val.value.nat64_val);
        break;
    case VAL_FLOAT32:
        format_float_cli((double)val.value.float32_val, true, buf, cap, offset);
        break;
    case VAL_FLOAT64:
        format_float_cli((double)val.value.float64_val, false, buf, cap, offset);
        break;
    case VAL_BOOL:
        buf_append(buf, cap, offset, "%s", val.value.bool_val ? "true" : "false");
        break;
    case VAL_FUNCTION:
        buf_append(
            buf,
            cap,
            offset,
            "<fn(%zu) -> %s>",
            val.value.wyrm_function_val.arg_count,
            value_type_name(val.value.wyrm_function_val.returned)
        );
        break;
    case VAL_VEC: {
        const wyrm_vec_t *vec = val.value.vec_val;
        if (vec == NULL) {
            buf_append(buf, cap, offset, "[]");
            break;
        }
        if (depth >= WYRM_WEB_MAX_DEPTH) {
            buf_append(buf, cap, offset, "[...]");
            break;
        }
        if (is_printable_nat8_string(vec)) {
            format_nat8_string(vec, buf, cap, offset);
        } else {
            size_t total = vec->items.size;
            size_t shown = total > WYRM_WEB_MAX_VEC_ITEMS ? WYRM_WEB_MAX_VEC_ITEMS : total;
            buf_append(buf, cap, offset, "[");
            for (size_t i = 0; i < shown; i++) {
                if (i > 0) {
                    buf_append(buf, cap, offset, ", ");
                }
                const wyrm_value_t *item = (const wyrm_value_t *)vector_get(&vec->items, i);
                if (item != NULL) {
                    format_wyrm_value_inner(*item, buf, cap, offset, depth + 1);
                }
            }
            if (total > shown) {
                buf_append(buf, cap, offset, ", ... (+%zu)", total - shown);
            }
            buf_append(buf, cap, offset, "]");
        }
        break;
    }
    default:
        buf_append(buf, cap, offset, "<tipo desconocido>");
        break;
    }
}

static void capture_return_value(wyrm_value_t val) {
    g_last_return[0] = '\0';
    size_t offset = 0;
    format_wyrm_value_inner(val, g_last_return, sizeof(g_last_return), &offset, 0);
    format_wyrm_type(val, g_last_return_type, sizeof(g_last_return_type));
}

const char *wyrm_web_type_name(wyrm_value_type_t type) {
    return value_type_name(type);
}

void wyrm_web_format_type(wyrm_value_t val, char *buf, size_t cap) {
    format_wyrm_type(val, buf, cap);
}

void wyrm_web_format_value(wyrm_value_t val, char *buf, size_t cap) {
    if (cap == 0) {
        return;
    }
    buf[0] = '\0';
    size_t offset = 0;
    format_wyrm_value_inner(val, buf, cap, &offset, 0);
}

void wyrm_web_set_last_return(wyrm_value_t val) {
    capture_return_value(val);
}

void wyrm_web_clear_last_return(void) {
    g_last_return[0] = '\0';
    g_last_return_type[0] = '\0';
}

bool wyrm_web_is_printable_nat8_vec(const wyrm_vec_t *vec) {
    return is_printable_nat8_string(vec);
}

void wyrm_web_format_nat8_ascii(const wyrm_vec_t *vec, char *buf, size_t cap) {
    if (cap == 0) {
        return;
    }
    buf[0] = '\0';
    if (vec == NULL || vec->element_type != VAL_NAT8 || vec->items.size == 0) {
        return;
    }

    size_t content_len = vec->items.size;
    const wyrm_value_t *last = (const wyrm_value_t *)vector_get(&vec->items, content_len - 1);
    if (last != NULL && last->type == VAL_NAT8 && last->value.nat8_val == 0) {
        content_len--;
    }

    size_t shown = content_len > WYRM_WEB_MAX_STR_PREVIEW ? WYRM_WEB_MAX_STR_PREVIEW : content_len;
    size_t offset = 0;
    for (size_t i = 0; i < shown; i++) {
        const wyrm_value_t *item = (const wyrm_value_t *)vector_get(&vec->items, i);
        if (item == NULL) {
            continue;
        }
        uint8_t byte = item->value.nat8_val;
        if (byte == '\n') {
            buf_append(buf, cap, &offset, "\\n");
        } else if (byte == '\t') {
            buf_append(buf, cap, &offset, "\\t");
        } else if (byte == '\r') {
            buf_append(buf, cap, &offset, "\\r");
        } else if (byte == '"' || byte == '\\') {
            buf_append(buf, cap, &offset, "\\%c", (char)byte);
        } else if (byte >= 32 && byte <= 126) {
            buf_append(buf, cap, &offset, "%c", (char)byte);
        } else {
            buf_append(buf, cap, &offset, "\\x%02X", byte);
        }
    }
    if (content_len > shown) {
        buf_append(buf, cap, &offset, "...");
    }
}

static int write_source_to_file(const char *path, const char *source) {
    FILE *file = fopen(path, "wb");
    if (file == NULL) {
        return -1;
    }
    size_t len = strlen(source);
    if (len > 0 && fwrite(source, 1, len, file) != len) {
        fclose(file);
        return -1;
    }
    if (fclose(file) != 0) {
        return -1;
    }
    return 0;
}

EMSCRIPTEN_KEEPALIVE
const char *wyrm_web_last_return(void) {
    return g_last_return;
}

EMSCRIPTEN_KEEPALIVE
const char *wyrm_web_last_return_type(void) {
    return g_last_return_type;
}

EMSCRIPTEN_KEEPALIVE
int wyrm_web_exec(const char *source, int mode, int verbose) {
    g_last_return[0] = '\0';
    g_last_return_type[0] = '\0';

    if (source == NULL) {
        fprintf(stderr, "Error: código fuente nulo.\n");
        fflush(stdout);
        fflush(stderr);
        return 1;
    }

    const int flags = (verbose != 0)
        ? (VERBOSE_MODE | DEBUG_MODE | SCANNER_MODE | PARSER_MODE)
        : 0;

    if (mode == 0) {
        terminate_wyrm();
        wyrm_value_t result = run_wyrm(source, flags);
        // Capturar valor y tipo ANTES de liberar scope/vectores
        capture_return_value(result);
        terminate_wyrm();
    } else if (mode == 1) {
        if (write_source_to_file(WYRM_WEB_TMP_SRC, source) != 0) {
            fprintf(stderr, "Error: no se pudo escribir %s en MEMFS.\n", WYRM_WEB_TMP_SRC);
            fflush(stdout);
            fflush(stderr);
            return 1;
        }

        wyrmc_t *wyrmc = create_wyrmc(WYRM_WEB_TMP_SRC, WYRM_WEB_TMP_WRC, flags);
        set_wyrmc_flags(wyrmc, flags);
        compile_file(wyrmc);
        destroy_wyrmc(wyrmc);
        if (verbose != 0) {
            printf("\n");
            fflush(stdout);
        }

        wyrm_value_t result = run_wvm(WYRM_WEB_TMP_WRC, flags);
        // Capturar valor y tipo ANTES de liberar vectores de la VM
        capture_return_value(result);
        free_all_wyrm_vecs();

        remove(WYRM_WEB_TMP_SRC);
        remove(WYRM_WEB_TMP_WRC);
    } else {
        fprintf(stderr, "Error: modo de ejecución no soportado (%d).\n", mode);
        fflush(stdout);
        fflush(stderr);
        return 2;
    }

    fflush(stdout);
    fflush(stderr);
    return 0;
}
