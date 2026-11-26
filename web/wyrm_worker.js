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

function readCStringExport(instance, exportName) {
  const ptr = instance.ccall(exportName, "number", [], []);
  return ptr ? instance.UTF8ToString(ptr) : "";
}

function readLastReturn(instance) {
  const formatted = readCStringExport(instance, "wyrm_web_last_return");
  const returnType = readCStringExport(instance, "wyrm_web_last_return_type");
  return {
    returnValue: formatted.length > 0 ? formatted : null,
    returnType,
  };
}

/**
 * Instancia `wyrm_core`, configura `FS.init` para `stdin`/`stdout`/`stderr`,
 * mide `performance.now()` y captura `ExitStatus` ante `panic()`.
 */
async function runWasmInvocation({ stdinText, invoke }) {
  const stdinBytes = utf8Encoder.encode(stdinText);
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
    if (charCode !== null && charCode !== undefined) {
      stdoutBytes.push(charCode & 0xff);
    }
  };

  const stderrHook = (charCode) => {
    if (charCode !== null && charCode !== undefined) {
      stderrBytes.push(charCode & 0xff);
    }
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
    return {
      initError:
        "No se pudo inicializar el módulo WebAssembly de wyrm_core.\n" +
        "Verificá que exista web/wasm_dist/wyrm_core.wasm (ejecutá ./web/build_wasm.sh).\n\n" +
        "Detalle: " +
        detail,
    };
  }

  let isError = false;
  let exitStatusCode = 0;
  let caughtExceptionText = "";
  let returnValue = null;
  let returnType = "";
  let extra = {};

  const startedAt = performance.now();
  try {
    const outcome = invoke(instance) || {};
    if (outcome.missingSymbolMessage) {
      isError = true;
      caughtExceptionText = outcome.missingSymbolMessage;
    } else if (outcome.status !== 0) {
      isError = true;
      exitStatusCode = outcome.status;
    } else {
      const lastRet = readLastReturn(instance);
      returnValue = lastRet.returnValue;
      returnType = lastRet.returnType;
      if (outcome.extra) {
        extra = outcome.extra;
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

  return {
    initError: null,
    isError,
    exitStatusCode,
    caughtExceptionText,
    runtimeErrors,
    rawStdout: utf8Decoder.decode(new Uint8Array(stdoutBytes)),
    rawStderr: utf8Decoder.decode(new Uint8Array(stderrBytes)).trim(),
    returnValue,
    returnType,
    elapsedMs,
    extra,
  };
}

function buildErrorStdout(finalStdout, invocation) {
  const errorParts = [];
  if (finalStdout.trim().length > 0) {
    errorParts.push(finalStdout.trimEnd());
  }
  if (invocation.rawStderr.length > 0) {
    errorParts.push(invocation.rawStderr);
  } else if (invocation.runtimeErrors.length > 0) {
    errorParts.push(invocation.runtimeErrors.join("\n"));
  } else if (invocation.caughtExceptionText.length > 0) {
    errorParts.push("Error de ejecución: " + invocation.caughtExceptionText);
  } else {
    errorParts.push(
      "El motor finalizó con código de salida " +
        String(invocation.exitStatusCode) +
        "."
    );
  }
  return errorParts.join("\n\n");
}

function postFailure(id, action, message) {
  self.postMessage({
    id,
    action,
    stdout: message,
    return_value: null,
    return_type: "",
    inspector_log: "",
    elapsed_ms: 0,
    is_error: true,
    trace_json: null,
    ast_trace_json: null,
  });
}

self.onmessage = async (event) => {
  const payload = event.data || {};
  const id = payload.id;
  const action =
    payload.action === "trace"
      ? "trace"
      : payload.action === "ast_trace"
        ? "ast_trace"
        : "run";
  const code = typeof payload.code === "string" ? payload.code : "";
  const mode = Number(payload.mode) || 0;
  const verbose = Boolean(payload.verbose);
  const stdin = typeof payload.stdin === "string" ? payload.stdin : "";
  const maxSteps = Number(payload.max_steps) || 1500;

  if (wasmLoadError !== null) {
    postFailure(id, action, wasmLoadError);
    return;
  }

  const invocation = await runWasmInvocation({
    stdinText: stdin,
    invoke: (instance) => {
      if (action === "trace") {
        if (typeof instance._wyrm_web_trace !== "function") {
          return {
            missingSymbolMessage:
              "El binario WASM cargado no incluye el motor paso a paso (wyrm_web_trace). Recompilá ejecutando ./web/build_wasm.sh.",
          };
        }
        const status = instance.ccall(
          "wyrm_web_trace",
          "number",
          ["string", "number"],
          [code, maxSteps]
        );
        return {
          status,
          extra:
            status === 0
              ? { traceJson: readCStringExport(instance, "wyrm_web_trace_json") || null }
              : {},
        };
      }

      if (action === "ast_trace") {
        if (typeof instance._wyrm_web_ast_trace !== "function") {
          return {
            missingSymbolMessage:
              "El binario WASM cargado no incluye el motor paso a paso del AST (wyrm_web_ast_trace). Recompilá ejecutando ./web/build_wasm.sh.",
          };
        }
        const status = instance.ccall(
          "wyrm_web_ast_trace",
          "number",
          ["string", "number"],
          [code, maxSteps]
        );
        return {
          status,
          extra:
            status === 0
              ? { astTraceJson: readCStringExport(instance, "wyrm_web_ast_trace_json") || null }
              : {},
        };
      }

      const status = instance.ccall(
        "wyrm_web_exec",
        "number",
        ["string", "number", "number"],
        [code, mode, verbose ? 1 : 0]
      );
      return { status };
    },
  });

  if (invocation.initError !== null) {
    postFailure(id, action, invocation.initError);
    return;
  }

  let finalStdout = invocation.rawStdout;
  let inspectorLog = "";
  let returnValue = invocation.returnValue;
  let returnType = invocation.returnType;
  let traceJson = invocation.extra.traceJson || null;
  let astTraceJson = invocation.extra.astTraceJson || null;

  if (action === "run" && verbose) {
    const separated = splitVerboseOutput(invocation.rawStdout);
    finalStdout = separated.stdout;
    inspectorLog = separated.inspectorLog;
  }

  if (invocation.isError) {
    finalStdout = buildErrorStdout(finalStdout, invocation);
    returnValue = null;
    returnType = "";
    traceJson = null;
    astTraceJson = null;
  }

  const baseResponse = {
    id,
    action,
    stdout: finalStdout,
    return_value: returnValue,
    return_type: returnType,
    elapsed_ms: invocation.elapsedMs,
    is_error: invocation.isError,
  };

  if (action === "trace") {
    self.postMessage({ ...baseResponse, trace_json: traceJson });
  } else if (action === "ast_trace") {
    self.postMessage({ ...baseResponse, ast_trace_json: astTraceJson });
  } else {
    self.postMessage({ ...baseResponse, inspector_log: inspectorLog });
  }
};
