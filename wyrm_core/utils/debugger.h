#pragma once

/*
Debugger para aislar dependencies y ofrecer mayores opciones
*/

// pint inteligente, agrea el prefijo [DEBUG], es compatible con el formato de
// printf estandar
void print_debug(const char *to_debug, ...);