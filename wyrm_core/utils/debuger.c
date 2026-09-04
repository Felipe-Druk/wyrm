#include "debuger.h"
#include <stdio.h>
#include <stdarg.h>


void print_debug(const char* to_debug, ...){
    
    printf("[DEBUG]: ");

    va_list arg;
    va_start(arg, to_debug);
    vprintf(to_debug, arg);

    va_end(arg);
}