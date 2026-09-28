#pragma once

// Errores graves que pueden ser disparados, codigos que no se comparten con estandar POSIX (de 3 a 126)
typedef enum { ERR_DIV_BY_ZERO = 3, ERR_UNKNOWN_OP = 4, ERR_TYPE_MISMATCH = 5 } WyrmErrorCode;

typedef WyrmErrorCode wyrm_error_code_t;

// muestra el mensaje por salida estandar y luego termina la ejecucion con el error_code
_Noreturn void panic(int error_code, const char *format, ...);