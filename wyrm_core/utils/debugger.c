#include "debugger.h"
#include <stdarg.h>
#include <stdio.h>

void print_debug(const char *to_debug, ...) {

    printf("[DEBUG]: ");

    va_list arg;
    va_start(arg, to_debug);
    vprintf(to_debug, arg);

    va_end(arg);
}

void print_line(const char *to_debug, ...) {
    printf("\n");

    va_list arg;
    va_start(arg, to_debug);
    vprintf(to_debug, arg);

    va_end(arg);
}

void print_line_debug(const char *to_debug, ...) {
    printf("\n[DEBUG]: ");
    va_list arg;
    va_start(arg, to_debug);
    vprintf(to_debug, arg);

    va_end(arg);
}

void print_jump_line() { printf("\n"); }