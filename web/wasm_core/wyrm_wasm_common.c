#include "wyrm_wasm_common.h"

#include <inttypes.h>
#include <math.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "utils/wyrm_vector.h"

#define WYRM_WEB_RET_BUF_SIZE 4096
#define WYRM_WEB_TYPE_BUF_SIZE 128
#define WYRM_WEB_MAX_VEC_ITEMS 64
#define WYRM_WEB_MAX_STR_PREVIEW 256
#define WYRM_WEB_MAX_DEPTH 4
#define WYRM_WEB_JBUF_DEFAULT_CAP 16384

static char g_last_return[WYRM_WEB_RET_BUF_SIZE];
static char g_last_return_type[WYRM_WEB_TYPE_BUF_SIZE];

/* ==========================================================================
 * Buffer dinámico de JSON (`json_buf_t`)
 * ========================================================================== */

void jbuf_init(json_buf_t *jb, size_t initial_cap) {
    size_t cap = initial_cap > 0 ? initial_cap : WYRM_WEB_JBUF_DEFAULT_CAP;
    jb->data = (char *)malloc(cap);
    if (jb->data == NULL) {
        jb->len = 0;
        jb->cap = 0;
        jb->oom = true;
        return;
    }
    jb->data[0] = '\0';
    jb->len = 0;
    jb->cap = cap;
    jb->oom = false;
}

void jbuf_free(json_buf_t *jb) {
    free(jb->data);
    jb->data = NULL;
    jb->len = 0;
    jb->cap = 0;
}

static bool jbuf_reserve(json_buf_t *jb, size_t extra) {
    if (jb->oom) {
        return false;
    }
    size_t needed = jb->len + extra + 1;
    if (needed <= jb->cap) {
        return true;
    }
    size_t next_cap = jb->cap > 0 ? jb->cap : WYRM_WEB_JBUF_DEFAULT_CAP;
    while (next_cap < needed) {
        if (next_cap > (SIZE_MAX / 2)) {
            jb->oom = true;
            return false;
        }
        next_cap *= 2;
    }
    char *resized = (char *)realloc(jb->data, next_cap);
    if (resized == NULL) {
        jb->oom = true;
        return false;
    }
    jb->data = resized;
    jb->cap = next_cap;
    return true;
}

static void jbuf_append_raw(json_buf_t *jb, const char *bytes, size_t n) {
    if (n == 0 || !jbuf_reserve(jb, n)) {
        return;
    }
    memcpy(jb->data + jb->len, bytes, n);
    jb->len += n;
    jb->data[jb->len] = '\0';
}

void jbuf_append(json_buf_t *jb, const char *s) {
    if (s == NULL) {
        return;
    }
    jbuf_append_raw(jb, s, strlen(s));
}

void jbuf_append_char(json_buf_t *jb, char c) {
    jbuf_append_raw(jb, &c, 1);
}

void jbuf_appendf(json_buf_t *jb, const char *fmt, ...) {
    if (jb->oom) {
        return;
    }
    va_list args;
    va_start(args, fmt);
    va_list args_copy;
    va_copy(args_copy, args);
    int needed = vsnprintf(NULL, 0, fmt, args);
    va_end(args);

    if (needed <= 0) {
        va_end(args_copy);
        return;
    }
    if (!jbuf_reserve(jb, (size_t)needed)) {
        va_end(args_copy);
        return;
    }
    vsnprintf(jb->data + jb->len, jb->cap - jb->len, fmt, args_copy);
    va_end(args_copy);
    jb->len += (size_t)needed;
}

void jbuf_append_escaped_str(json_buf_t *jb, const char *s) {
    jbuf_append_char(jb, '"');
    if (s != NULL) {
        for (const unsigned char *p = (const unsigned char *)s; *p != '\0'; p++) {
            unsigned char ch = *p;
            switch (ch) {
            case '"':
                jbuf_append_raw(jb, "\\\"", 2);
                break;
            case '\\':
                jbuf_append_raw(jb, "\\\\", 2);
                break;
            case '\b':
                jbuf_append_raw(jb, "\\b", 2);
                break;
            case '\f':
                jbuf_append_raw(jb, "\\f", 2);
                break;
            case '\n':
                jbuf_append_raw(jb, "\\n", 2);
                break;
            case '\r':
                jbuf_append_raw(jb, "\\r", 2);
                break;
            case '\t':
                jbuf_append_raw(jb, "\\t", 2);
                break;
            default:
                if (ch < 0x20) {
                    jbuf_appendf(jb, "\\u%04x", (unsigned int)ch);
                } else {
                    jbuf_append_char(jb, (char)ch);
                }
                break;
            }
        }
    }
    jbuf_append_char(jb, '"');
}

/* ==========================================================================
 * Formateo de `wyrm_value_t`, tipos, ancho en bits y bytes en hexadecimal
 * ========================================================================== */

const char *wyrm_web_type_name(wyrm_value_type_t type) {
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

static void buf_append(char *buf, size_t cap, size_t *offset, const char *fmt, ...)
    WYRM_PRINTF_LIKE(4, 5);

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
        buf_append(buf, cap, offset, "%s", wyrm_web_type_name(val.type));
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
            buf_append(buf, cap, offset, "%s", wyrm_web_type_name(vec->element_type));
        }
    } else {
        buf_append(buf, cap, offset, "%s", wyrm_web_type_name(vec->element_type));
    }
    buf_append(buf, cap, offset, ">");
}

void wyrm_web_format_type(wyrm_value_t val, char *buf, size_t cap) {
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

bool wyrm_web_is_printable_nat8_vec(const wyrm_vec_t *vec) {
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
            wyrm_web_type_name(val.value.wyrm_function_val.returned)
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
        if (wyrm_web_is_printable_nat8_vec(vec)) {
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

void wyrm_web_format_value(wyrm_value_t val, char *buf, size_t cap) {
    if (cap == 0) {
        return;
    }
    buf[0] = '\0';
    size_t offset = 0;
    format_wyrm_value_inner(val, buf, cap, &offset, 0);
}

static void format_bytes_hex(const uint8_t *bytes, size_t count, char *buf, size_t cap) {
    if (cap == 0) {
        return;
    }
    buf[0] = '\0';
    size_t pos = 0;
    for (size_t i = 0; i < count; i++) {
        int written = snprintf(
            buf + pos,
            cap - pos,
            i == 0 ? "%02X" : " %02X",
            (unsigned int)bytes[i]
        );
        if (written <= 0 || (size_t)written >= cap - pos) {
            break;
        }
        pos += (size_t)written;
    }
}

int wyrm_web_value_bits(wyrm_value_t val) {
    switch (val.type) {
    case VAL_VOID:
        return 0;
    case VAL_NAT8:
    case VAL_INT8:
    case VAL_BOOL:
        return 8;
    case VAL_NAT16:
    case VAL_INT16:
        return 16;
    case VAL_NAT32:
    case VAL_INT32:
    case VAL_FLOAT32:
        return 32;
    case VAL_NAT64:
    case VAL_INT64:
    case VAL_FLOAT64:
        return 64;
    case VAL_VEC:
    case VAL_FUNCTION:
        return (int)(sizeof(void *) * 8);
    default:
        return 0;
    }
}

void wyrm_web_format_hex(wyrm_value_t val, char *buf, size_t cap) {
    if (cap == 0) {
        return;
    }
    uint8_t raw[8];
    memset(raw, 0, sizeof(raw));
    buf[0] = '\0';

    switch (val.type) {
    case VAL_VOID:
        break;
    case VAL_NAT8:
        memcpy(raw, &val.value.nat8_val, 1);
        format_bytes_hex(raw, 1, buf, cap);
        break;
    case VAL_INT8:
        memcpy(raw, &val.value.int8_val, 1);
        format_bytes_hex(raw, 1, buf, cap);
        break;
    case VAL_BOOL: {
        uint8_t b = val.value.bool_val ? 1u : 0u;
        format_bytes_hex(&b, 1, buf, cap);
        break;
    }
    case VAL_NAT16:
        memcpy(raw, &val.value.nat16_val, 2);
        format_bytes_hex(raw, 2, buf, cap);
        break;
    case VAL_INT16:
        memcpy(raw, &val.value.int16_val, 2);
        format_bytes_hex(raw, 2, buf, cap);
        break;
    case VAL_NAT32:
        memcpy(raw, &val.value.nat32_val, 4);
        format_bytes_hex(raw, 4, buf, cap);
        break;
    case VAL_INT32:
        memcpy(raw, &val.value.int32_val, 4);
        format_bytes_hex(raw, 4, buf, cap);
        break;
    case VAL_FLOAT32: {
        float f32 = (float)val.value.float32_val;
        memcpy(raw, &f32, 4);
        format_bytes_hex(raw, 4, buf, cap);
        break;
    }
    case VAL_NAT64:
        memcpy(raw, &val.value.nat64_val, 8);
        format_bytes_hex(raw, 8, buf, cap);
        break;
    case VAL_INT64:
        memcpy(raw, &val.value.int64_val, 8);
        format_bytes_hex(raw, 8, buf, cap);
        break;
    case VAL_FLOAT64: {
        double f64 = (double)val.value.float64_val;
        memcpy(raw, &f64, 8);
        format_bytes_hex(raw, 8, buf, cap);
        break;
    }
    case VAL_VEC:
        snprintf(buf, cap, "0x%" PRIxPTR, (uintptr_t)val.value.vec_val);
        break;
    case VAL_FUNCTION:
        snprintf(buf, cap, "fn(%zu)", val.value.wyrm_function_val.arg_count);
        break;
    default:
        break;
    }
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

/* ==========================================================================
 * Estado compartido de último valor y tipo de retorno
 * ========================================================================== */

void wyrm_web_set_last_return(wyrm_value_t val) {
    wyrm_web_format_value(val, g_last_return, sizeof(g_last_return));
    wyrm_web_format_type(val, g_last_return_type, sizeof(g_last_return_type));
}

void wyrm_web_clear_last_return(void) {
    g_last_return[0] = '\0';
    g_last_return_type[0] = '\0';
}

EMSCRIPTEN_KEEPALIVE
const char *wyrm_web_last_return(void) {
    return g_last_return;
}

EMSCRIPTEN_KEEPALIVE
const char *wyrm_web_last_return_type(void) {
    return g_last_return_type;
}
