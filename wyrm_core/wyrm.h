#pragma once

#include "wyrm_value.h"

// Función principal, toma cualquier texto plano y lo ejecuta en el intérprete
// de Wyrm.
wyrm_value_t run_wyrm(const char *input, int flags);