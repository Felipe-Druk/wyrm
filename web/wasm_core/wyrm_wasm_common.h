#pragma once

#include <stdbool.h>
#include <stddef.h>

#include "wyrm_types/wyrm_value/wyrm_value.h"

#ifdef __EMSCRIPTEN__
#include <emscripten/emscripten.h>
#else
#ifndef EMSCRIPTEN_KEEPALIVE
#define EMSCRIPTEN_KEEPALIVE
#endif
#endif

#if defined(__GNUC__) || defined(__clang__)
#define WYRM_PRINTF_LIKE(fmt_idx, first_arg) __attribute__((format(printf, fmt_idx, first_arg)))
#else
#define WYRM_PRINTF_LIKE(fmt_idx, first_arg)
#endif

typedef struct {
    char *data;
    size_t len;
    size_t cap;
    bool oom;
} json_buf_t;

void jbuf_init(json_buf_t *jb, size_t initial_cap);
void jbuf_free(json_buf_t *jb);
void jbuf_append(json_buf_t *jb, const char *s);
void jbuf_append_char(json_buf_t *jb, char c);
void jbuf_appendf(json_buf_t *jb, const char *fmt, ...) WYRM_PRINTF_LIKE(2, 3);
void jbuf_append_escaped_str(json_buf_t *jb, const char *s);

const char *wyrm_web_type_name(wyrm_value_type_t type);
void wyrm_web_format_type(wyrm_value_t val, char *buf, size_t cap);
void wyrm_web_format_value(wyrm_value_t val, char *buf, size_t cap);
int wyrm_web_value_bits(wyrm_value_t val);
void wyrm_web_format_hex(wyrm_value_t val, char *buf, size_t cap);
bool wyrm_web_is_printable_nat8_vec(const wyrm_vec_t *vec);
void wyrm_web_format_nat8_ascii(const wyrm_vec_t *vec, char *buf, size_t cap);

void wyrm_web_set_last_return(wyrm_value_t val);
void wyrm_web_clear_last_return(void);

EMSCRIPTEN_KEEPALIVE const char *wyrm_web_last_return(void);
EMSCRIPTEN_KEEPALIVE const char *wyrm_web_last_return_type(void);
