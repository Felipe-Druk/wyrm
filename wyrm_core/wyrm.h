#pragma once


// Opciones varias para hacer debug de Wyrm
typedef enum {
    VERBOSE_MODE = 1 << 0,
    DEBUG_MODE = 1 << 1,
    SCANNER_MODE = 1 << 2,
    PARSER_MODE = 1 << 3,
} Options;

// Función principal, toma cualquier texto plano y lo ejecuta en el interprete de Wyrm.
void run_wyrm(const char* input, int flags);