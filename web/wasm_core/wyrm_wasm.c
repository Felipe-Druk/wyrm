#include <inttypes.h>
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
#include "wyrmc.h"

#define WYRM_WEB_RET_BUF_SIZE 4096
#define WYRM_WEB_MAX_VEC_ITEMS 24
#define WYRM_WEB_MAX_STR_PREVIEW 96
#define WYRM_WEB_MAX_DEPTH 3
#define WYRM_WEB_TMP_SRC "/tmp/main.wyrm"
#define WYRM_WEB_TMP_WRC "/tmp/main.wrc"

static char g_last_return[WYRM_WEB_RET_BUF_SIZE];

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

static void format_wyrm_value_inner(
    wyrm_value_t val,
    char *buf,
    size_t cap,
    size_t *offset,
    int depth,
    bool include_type
) {
    switch (val.type) {
    case VAL_VOID:
        buf_append(buf, cap, offset, "void");
        break;
    case VAL_INT8:
        buf_append(buf, cap, offset, "%" PRId8, val.value.int8_val);
        if (include_type) {
            buf_append(buf, cap, offset, " (int8)");
        }
        break;
    case VAL_INT16:
        buf_append(buf, cap, offset, "%" PRId16, val.value.int16_val);
        if (include_type) {
            buf_append(buf, cap, offset, " (int16)");
        }
        break;
    case VAL_INT32:
        buf_append(buf, cap, offset, "%" PRId32, val.value.int32_val);
        if (include_type) {
            buf_append(buf, cap, offset, " (int32)");
        }
        break;
    case VAL_INT64:
        buf_append(buf, cap, offset, "%" PRId64, val.value.int64_val);
        if (include_type) {
            buf_append(buf, cap, offset, " (int64)");
        }
        break;
    case VAL_NAT8:
        buf_append(buf, cap, offset, "%" PRIu8, val.value.nat8_val);
        if (include_type) {
            if (val.value.nat8_val >= 32 && val.value.nat8_val <= 126) {
                buf_append(buf, cap, offset, " '%c' (nat8)", (char)val.value.nat8_val);
            } else {
                buf_append(buf, cap, offset, " (nat8)");
            }
        }
        break;
    case VAL_NAT16:
        buf_append(buf, cap, offset, "%" PRIu16, val.value.nat16_val);
        if (include_type) {
            buf_append(buf, cap, offset, " (nat16)");
        }
        break;
    case VAL_NAT32:
        buf_append(buf, cap, offset, "%" PRIu32, val.value.nat32_val);
        if (include_type) {
            buf_append(buf, cap, offset, " (nat32)");
        }
        break;
    case VAL_NAT64:
        buf_append(buf, cap, offset, "%" PRIu64, val.value.nat64_val);
        if (include_type) {
            buf_append(buf, cap, offset, " (nat64)");
        }
        break;
    case VAL_FLOAT32:
        buf_append(buf, cap, offset, "%g", (double)val.value.float32_val);
        if (include_type) {
            buf_append(buf, cap, offset, " (float32)");
        }
        break;
    case VAL_FLOAT64:
        buf_append(buf, cap, offset, "%g", (double)val.value.float64_val);
        if (include_type) {
            buf_append(buf, cap, offset, " (float64)");
        }
        break;
    case VAL_BOOL:
        buf_append(buf, cap, offset, "%s", val.value.bool_val ? "true" : "false");
        if (include_type) {
            buf_append(buf, cap, offset, " (bool)");
        }
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
        if (include_type) {
            buf_append(buf, cap, offset, " (function)");
        }
        break;
    case VAL_VEC: {
        const wyrm_vec_t *vec = val.value.vec_val;
        if (vec == NULL) {
            buf_append(buf, cap, offset, include_type ? "[] (vec)" : "[]");
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
                    format_wyrm_value_inner(*item, buf, cap, offset, depth + 1, false);
                }
            }
            if (total > shown) {
                buf_append(buf, cap, offset, ", ... (+%zu)", total - shown);
            }
            buf_append(buf, cap, offset, "]");
        }
        if (include_type) {
            buf_append(buf, cap, offset, " (vec<%s>)", value_type_name(vec->element_type));
        }
        break;
    }
    default:
        buf_append(buf, cap, offset, "<unknown>");
        break;
    }
}

static void format_wyrm_value(wyrm_value_t val, char *buf, size_t cap) {
    if (cap == 0) {
        return;
    }
    buf[0] = '\0';
    size_t offset = 0;
    format_wyrm_value_inner(val, buf, cap, &offset, 0, true);
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
int wyrm_web_exec(const char *source, int mode, int verbose) {
    g_last_return[0] = '\0';

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
        // Entorno limpio por ejecución en el intérprete
        terminate_wyrm();
        wyrm_value_t result = run_wyrm(source, flags);
        format_wyrm_value(result, g_last_return, sizeof(g_last_return));
        terminate_wyrm();
    } else if (mode == 1) {
        if (write_source_to_file(WYRM_WEB_TMP_SRC, source) != 0) {
            fprintf(stderr, "Error: no se pudo escribir %s en MEMFS.\n", WYRM_WEB_TMP_SRC);
            fflush(stdout);
            fflush(stderr);
            return 1;
        }

        wyrmc_t *wyrmc = create_wyrmc(WYRM_WEB_TMP_SRC, WYRM_WEB_TMP_WRC, flags);
        // create_wyrmc reinicia wyrmc->flags a 0; aplicamos explícitamente las flags de verbose
        set_wyrmc_flags(wyrmc, flags);
        compile_file(wyrmc);
        destroy_wyrmc(wyrmc);
        if (verbose != 0) {
            printf("\n");
            fflush(stdout);
        }

        wyrm_value_t result = run_wvm(WYRM_WEB_TMP_WRC, flags);
        format_wyrm_value(result, g_last_return, sizeof(g_last_return));
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
