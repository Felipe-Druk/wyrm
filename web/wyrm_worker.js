/* global importScripts, createWyrmCore */
"use strict";

const WASM_SCRIPT_URL = "./wasm_dist/wyrm_core.js";
const WASM_ASSET_PREFIX = "./wasm_dist/";

let wasmLoadError = null;

try {
  importScripts(WASM_SCRIPT_URL);
  if (typeof createWyrmCore !== "function") {
    wasmLoadError =
      "El módulo wyrm_core.js se cargó, pero no expuso la fábrica createWyrmCore.";
  }
} catch (err) {
  const detail = err && err.message ? err.message : String(err);
  wasmLoadError =
    "El motor WebAssembly de wyrm_core aún no fue compilado.\n\n" +
    "Ejecutá el script de compilación antes de ejecutar programas:\n" +
    "  ./web/build_wasm.sh\n\n" +
    "Detalle técnico: " +
    detail;
}

const utf8Encoder = new TextEncoder();
const utf8Decoder = new TextDecoder("utf-8");

const GLUED_DEBUG_MARKERS = [
  "[DEBUG]: == END OF PARSER ==",
  "[DEBUG]: COMPILE AST....",
];

/**
 * Separa la traza de depuración del motor (líneas `[DEBUG]:`, encabezados `== ... ==`
 * y continuaciones `' ==` de caracteres de salto de línea) de la salida limpia del programa.
 */
function splitVerboseOutput(rawStdout) {
  if (!rawStdout) {
    return { stdout: "", inspectorLog: "" };
  }

  const lines = rawStdout.split("\n");
  const inspectorLines = [];
  const programLines = [];

  let matchedAnyDebug = false;
  let expectingNewlineCharClose = false;
  let lastWasDebug = false;

  for (let i = 0; i < lines.length; i += 1) {
    const line = lines[i];
    const trimmed = line.trim();

    if (expectingNewlineCharClose) {
      expectingNewlineCharClose = false;
      if (trimmed === "' ==") {
        inspectorLines.push(line);
        lastWasDebug = true;
        continue;
      }
    }

    let handledGlued = false;
    for (let m = 0; m < GLUED_DEBUG_MARKERS.length; m += 1) {
      const marker = GLUED_DEBUG_MARKERS[m];
      if (line.startsWith(marker) && line.length > marker.length) {
        inspectorLines.push(marker);
        programLines.push(line.slice(marker.length));
        matchedAnyDebug = true;
        lastWasDebug = false;
        handledGlued = true;
        break;
      }
    }
    if (handledGlued) {
      continue;
    }

    const isDebugPrefix = trimmed.startsWith("[DEBUG]:");
    const isSectionBanner =
      trimmed.startsWith("== ") && trimmed.endsWith("==");

    if (isDebugPrefix || isSectionBanner) {
      inspectorLines.push(line);
      matchedAnyDebug = true;
      lastWasDebug = true;

      if (
        isDebugPrefix &&
        trimmed.includes("current_char: '") &&
        !trimmed.endsWith("==")
      ) {
        expectingNewlineCharClose = true;
      }
      continue;
    }

    // Evita arrastrar líneas vacías emitidas por print_line / print_line_debug hacia stdout
    if (trimmed === "" && lastWasDebug) {
      continue;
    }

    lastWasDebug = false;
    programLines.push(line);
  }

  if (!matchedAnyDebug) {
    return {
      stdout: "",
      inspectorLog: rawStdout.trimEnd(),
    };
  }

  // Remueve saltos vacíos iniciales o finales dejados tras separar las líneas de depuración
  while (programLines.length > 0 && programLines[0] === "") {
    programLines.shift();
  }
  while (
    programLines.length > 0 &&
    programLines[programLines.length - 1] === ""
  ) {
    programLines.pop();
  }

  return {
    stdout: programLines.join("\n"),
    inspectorLog: inspectorLines.join("\n").trim(),
  };
}

self.onmessage = async (event) => {
  const payload = event.data || {};
  const id = payload.id;
  const action = payload.action === "trace" ? "trace" : "run";
  const code = typeof payload.code === "string" ? payload.code : "";
  const mode = Number(payload.mode) || 0;
  const verbose = Boolean(payload.verbose);
  const stdin = typeof payload.stdin === "string" ? payload.stdin : "";
  const maxSteps = Number(payload.max_steps) || 1500;

  if (wasmLoadError !== null) {
    self.postMessage({
      id,
      action,
      stdout: wasmLoadError,
      return_value: null,
      return_type: "",
      inspector_log: "",
      elapsed_ms: 0,
      is_error: true,
      trace_json: null,
    });
    return;
  }

  const stdinBytes = utf8Encoder.encode(stdin);
  let stdinOffset = 0;
  const stdoutBytes = [];
  const stderrBytes = [];
  const runtimeErrors = [];

  const stdinHook = () => {
    if (stdinOffset < stdinBytes.length) {
      return stdinBytes[stdinOffset++];
    }
    return null;
  };

  const stdoutHook = (charCode) => {
    if (charCode === null || charCode === undefined) {
      return;
    }
    stdoutBytes.push(charCode & 0xff);
  };

  const stderrHook = (charCode) => {
    if (charCode === null || charCode === undefined) {
      return;
    }
    stderrBytes.push(charCode & 0xff);
  };

  let instance;
  try {
    instance = await createWyrmCore({
      locateFile: (filePath) => WASM_ASSET_PREFIX + filePath,
      preRun: [
        (mod) => {
          const fs = mod && mod.FS ? mod.FS : null;
          if (fs && typeof fs.init === "function") {
            fs.init(stdinHook, stdoutHook, stderrHook);
          }
        },
      ],
      print: () => {},
      printErr: (text) => {
        if (text !== undefined && text !== null && String(text).length > 0) {
          runtimeErrors.push(String(text));
        }
      },
    });
  } catch (err) {
    const detail = err && err.message ? err.message : String(err);
    self.postMessage({
      id,
      action,
      stdout:
        "No se pudo inicializar el módulo WebAssembly de wyrm_core.\n" +
        "Verificá que exista web/wasm_dist/wyrm_core.wasm (ejecutá ./web/build_wasm.sh).\n\n" +
        "Detalle: " +
        detail,
      return_value: null,
      return_type: "",
      inspector_log: "",
      elapsed_ms: 0,
      is_error: true,
      trace_json: null,
    });
    return;
  }

  let isError = false;
  let exitStatusCode = 0;
  let returnValue = null;
  let returnType = "";
  let traceJson = null;
  let caughtExceptionText = "";

  const startedAt = performance.now();
  try {
    if (action === "trace") {
      if (typeof instance._wyrm_web_trace !== "function") {
        isError = true;
        caughtExceptionText =
          "El binario WASM cargado no incluye el motor paso a paso (wyrm_web_trace). Recompilá ejecutando ./web/build_wasm.sh.";
      } else {
        const traceStatus = instance.ccall(
          "wyrm_web_trace",
          "number",
          ["string", "number"],
          [code, maxSteps]
        );
        if (traceStatus !== 0) {
          isError = true;
          exitStatusCode = traceStatus;
        } else {
          const tracePtr = instance.ccall(
            "wyrm_web_trace_json",
            "number",
            [],
            []
          );
          if (tracePtr) {
            traceJson = instance.UTF8ToString(tracePtr);
          }
          const retPtr = instance.ccall(
            "wyrm_web_last_return",
            "number",
            [],
            []
          );
          if (retPtr) {
            const formatted = instance.UTF8ToString(retPtr);
            returnValue = formatted.length > 0 ? formatted : null;
          }
          const retTypePtr = instance.ccall(
            "wyrm_web_last_return_type",
            "number",
            [],
            []
          );
          if (retTypePtr) {
            returnType = instance.UTF8ToString(retTypePtr);
          }
        }
      }
    } else {
      const execStatus = instance.ccall(
        "wyrm_web_exec",
        "number",
        ["string", "number", "number"],
        [code, mode, verbose ? 1 : 0]
      );
      if (execStatus !== 0) {
        isError = true;
        exitStatusCode = execStatus;
      } else {
        const retPtr = instance.ccall("wyrm_web_last_return", "number", [], []);
        if (retPtr) {
          const formatted = instance.UTF8ToString(retPtr);
          returnValue = formatted.length > 0 ? formatted : null;
        }
        const retTypePtr = instance.ccall(
          "wyrm_web_last_return_type",
          "number",
          [],
          []
        );
        if (retTypePtr) {
          returnType = instance.UTF8ToString(retTypePtr);
        }
      }
    }
  } catch (err) {
    isError = true;
    if (err && (err.name === "ExitStatus" || typeof err.status === "number")) {
      exitStatusCode = Number(err.status);
    } else {
      caughtExceptionText = err && err.message ? err.message : String(err);
    }
  }
  const elapsedMs = Math.max(0, performance.now() - startedAt);

  const rawStdout = utf8Decoder.decode(new Uint8Array(stdoutBytes));
  const rawStderr = utf8Decoder.decode(new Uint8Array(stderrBytes)).trim();

  let finalStdout = rawStdout;
  let inspectorLog = "";

  if (action === "run" && verbose) {
    const separated = splitVerboseOutput(rawStdout);
    finalStdout = separated.stdout;
    inspectorLog = separated.inspectorLog;
  }

  if (isError) {
    const errorParts = [];
    if (finalStdout.trim().length > 0) {
      errorParts.push(finalStdout.trimEnd());
    }
    if (rawStderr.length > 0) {
      errorParts.push(rawStderr);
    } else if (runtimeErrors.length > 0) {
      errorParts.push(runtimeErrors.join("\n"));
    } else if (caughtExceptionText.length > 0) {
      errorParts.push("Error de ejecución: " + caughtExceptionText);
    } else {
      errorParts.push(
        "El motor finalizó con código de salida " + String(exitStatusCode) + "."
      );
    }
    finalStdout = errorParts.join("\n\n");
    returnValue = null;
    returnType = "";
    traceJson = null;
  }

  if (action === "trace") {
    self.postMessage({
      id,
      action: "trace",
      stdout: finalStdout,
      return_value: returnValue,
      return_type: returnType,
      elapsed_ms: elapsedMs,
      is_error: isError,
      trace_json: traceJson,
    });
    return;
  }

  self.postMessage({
    id,
    action: "run",
    stdout: finalStdout,
    return_value: returnValue,
    return_type: returnType,
    inspector_log: inspectorLog,
    elapsed_ms: elapsedMs,
    is_error: isError,
  });
};
