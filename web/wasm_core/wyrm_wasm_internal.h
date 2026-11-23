#pragma once

#include <stdbool.h>
#include <stddef.h>

#include "wyrm_types/wyrm_value/wyrm_value.h"

const char *wyrm_web_type_name(wyrm_value_type_t type);
void wyrm_web_format_type(wyrm_value_t val, char *buf, size_t cap);
void wyrm_web_format_value(wyrm_value_t val, char *buf, size_t cap);
void wyrm_web_set_last_return(wyrm_value_t val);
void wyrm_web_clear_last_return(void);
bool wyrm_web_is_printable_nat8_vec(const wyrm_vec_t *vec);
void wyrm_web_format_nat8_ascii(const wyrm_vec_t *vec, char *buf, size_t cap);
