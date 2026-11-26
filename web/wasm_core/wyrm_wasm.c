#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "options.h"
#include "wvm/wvm.h"
#include "wyrm.h"
#include "wyrm_types/wyrm_value/wyrm_value.h"
#include "wyrm_wasm_common.h"
#include "wyrmc.h"

#define WYRM_WEB_TMP_SRC "/tmp/main.wyrm"
#define WYRM_WEB_TMP_WRC "/tmp/main.wrc"

static int write_source_to_file(const char *path, const char *source) {
    FILE *file = fopen(path, "wb");
    if (file == NULL) {
        return -1;
    }
    size_t len = strlen(source);
    if (len > 0 && fwrite(source, 1, len, file) != len) {
        fclose(file);
        return -1;
    }
    if (fclose(file) != 0) {
        return -1;
    }
    return 0;
}

EMSCRIPTEN_KEEPALIVE
int wyrm_web_exec(const char *source, int mode, int verbose) {
    wyrm_web_clear_last_return();

    if (source == NULL) {
        fprintf(stderr, "Error: código fuente nulo.\n");
        fflush(stdout);
        fflush(stderr);
        return 1;
    }

    const int flags = (verbose != 0)
        ? (VERBOSE_MODE | DEBUG_MODE | SCANNER_MODE | PARSER_MODE)
        : 0;

    if (mode == 0) {
        terminate_wyrm();
        wyrm_value_t result = run_wyrm(source, flags);
        // Capturar valor y tipo ANTES de liberar scope/vectores
        wyrm_web_set_last_return(result);
        terminate_wyrm();
    } else if (mode == 1) {
        if (write_source_to_file(WYRM_WEB_TMP_SRC, source) != 0) {
            fprintf(stderr, "Error: no se pudo escribir %s en MEMFS.\n", WYRM_WEB_TMP_SRC);
            fflush(stdout);
            fflush(stderr);
            return 1;
        }

        wyrmc_t *wyrmc = create_wyrmc(WYRM_WEB_TMP_SRC, WYRM_WEB_TMP_WRC, flags);
        set_wyrmc_flags(wyrmc, flags);
        compile_file(wyrmc);
        destroy_wyrmc(wyrmc);
        if (verbose != 0) {
            printf("\n");
            fflush(stdout);
        }

        wyrm_value_t result = run_wvm(WYRM_WEB_TMP_WRC, flags);
        // Capturar valor y tipo ANTES de liberar vectores de la VM
        wyrm_web_set_last_return(result);
        free_all_wyrm_vecs();

        remove(WYRM_WEB_TMP_SRC);
        remove(WYRM_WEB_TMP_WRC);
    } else {
        fprintf(stderr, "Error: modo de ejecución no soportado (%d).\n", mode);
        fflush(stdout);
        fflush(stderr);
        return 2;
    }

    fflush(stdout);
    fflush(stderr);
    return 0;
}
