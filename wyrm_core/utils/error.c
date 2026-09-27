#include "error.h"

#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>

_Noreturn void panic(int error_code, const char *format, ...) {
    fprintf(stderr, "Wyrm Panic [%d]: ", error_code);

    va_list args;
    va_start(args, format);
    vfprintf(stderr, format, args);
    va_end(args);

    fprintf(stderr, "\n");

    exit(error_code);
}