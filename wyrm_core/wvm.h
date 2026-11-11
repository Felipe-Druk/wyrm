#pragma once
#include "wyrm_types/wyrm_value/wyrm_value.h"
/*
Archivo contendedor de la logica de ejecucion de archivos compilados (.wrc).
*/

wyrm_value_t run_wvm(const char *file_path, int flags);