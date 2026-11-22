#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
WYRM_CORE_DIR="${REPO_ROOT}/wyrm_core"
WASM_BRIDGE="${SCRIPT_DIR}/wasm_core/wyrm_wasm.c"
OUT_DIR="${SCRIPT_DIR}/wasm_dist"

if ! command -v emcc >/dev/null 2>&1; then
    echo "Error: no se encontró 'emcc' en PATH." >&2
    echo "Activá Emscripten SDK (ej. 'source /ruta/a/emsdk/emsdk_env.sh') y volvé a ejecutar este script." >&2
    exit 1
fi

if [[ ! -d "${WYRM_CORE_DIR}" ]]; then
    echo "Error: no se encontró el directorio '${WYRM_CORE_DIR}'." >&2
    exit 1
fi

if [[ ! -f "${WASM_BRIDGE}" ]]; then
    echo "Error: no se encontró el puente C '${WASM_BRIDGE}'." >&2
    exit 1
fi

INCLUDE_FLAGS=()
while IFS= read -r dir; do
    INCLUDE_FLAGS+=("-I${dir}")
done < <(find "${WYRM_CORE_DIR}" -type d | sort)

C_SOURCES=()
while IFS= read -r src; do
    C_SOURCES+=("${src}")
done < <(find "${WYRM_CORE_DIR}" -type f -name "*.c" | sort)
C_SOURCES+=("${WASM_BRIDGE}")

# Detección de flags de compatibilidad para Clang/Emscripten (C23 en wvm.c y _Float32/_Float64 en wyrm_value.h)
COMPAT_FLAGS=()
if printf 'int f(int x){switch(x){case 0: int y = 1; return y; default: return 0;}}\n' \
    | emcc -std=gnu23 -x c -fsyntax-only - >/dev/null 2>&1; then
    COMPAT_FLAGS+=("-std=gnu23")
else
    COMPAT_FLAGS+=("-Wno-c23-extensions")
fi

if ! printf '_Float32 a = 0; _Float64 b = 0;\n' \
    | emcc "${COMPAT_FLAGS[@]}" -x c -fsyntax-only - >/dev/null 2>&1; then
    COMPAT_FLAGS+=("-D_Float32=float" "-D_Float64=double")
fi

mkdir -p "${OUT_DIR}"

echo "Compilando wyrm_core (${#C_SOURCES[@]} fuentes C) a WebAssembly..."
emcc \
    "${COMPAT_FLAGS[@]}" \
    "${INCLUDE_FLAGS[@]}" \
    "${C_SOURCES[@]}" \
    -O2 \
    -lm \
    -sMODULARIZE=1 \
    -sEXPORT_NAME=createWyrmCore \
    -sENVIRONMENT=web,worker \
    -sFORCE_FILESYSTEM=1 \
    -sALLOW_MEMORY_GROWTH=1 \
    -sEXPORTED_FUNCTIONS='["_wyrm_web_exec","_wyrm_web_last_return","_malloc","_free"]' \
    -sEXPORTED_RUNTIME_METHODS='["ccall","UTF8ToString","FS"]' \
    -o "${OUT_DIR}/wyrm_core.js"

echo "Compilación exitosa:"
echo "  - ${OUT_DIR}/wyrm_core.js"
echo "  - ${OUT_DIR}/wyrm_core.wasm"
