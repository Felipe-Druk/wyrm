#pragma once

/*
Debuger para aislar dependencias y ofrecer mayores opciones
*/


// pint inteligente, agrea el prefijo [DEBUG], es compaitble con el formato de printf estandar
void print_debug(const char* to_debug, ...);