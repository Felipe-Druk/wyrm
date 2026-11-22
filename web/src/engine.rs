//! Interfaz desacoplada del motor de ejecución de Wyrm para el Playground web.
//!
//! En la siguiente fase, la implementación mock de [`execute`] se reemplazará por el puente
//! hacia el binario WebAssembly compilado desde `wyrm_core` (`wyrm.js` / `wyrm.wasm`).

/// Modo de ejecución elegido en el Playground.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    /// Evalúa el AST directamente con el intérprete de `wyrm_core`.
    Interpreter,
    /// Compila el programa a bytecode `.wrc` y lo ejecuta en la Wyrm VM.
    CompilerVm,
}

impl ExecutionMode {
    /// Etiqueta legible para mostrar en la interfaz.
    pub fn label(self) -> &'static str {
        match self {
            Self::Interpreter => "Intérprete",
            Self::CompilerVm => "Compilador + VM",
        }
    }

    /// Descripción del pipeline interno correspondiente al modo elegido.
    pub fn pipeline_summary(self) -> &'static str {
        match self {
            Self::Interpreter => "Scanner -> Parser (AST) -> Intérprete",
            Self::CompilerVm => "Scanner -> Parser (AST) -> Compilador (.wrc) -> Wyrm VM",
        }
    }
}

/// Parámetros de entrada para invocar una ejecución de código Wyrm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionRequest {
    pub source_code: String,
    pub mode: ExecutionMode,
    pub verbose: bool,
    pub stdin: String,
}

/// Resultado estructurado de una ejecución devuelto a la interfaz del Playground.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionResult {
    pub stdout: String,
    pub return_value: Option<String>,
    pub inspector_log: String,
    pub elapsed_ms: f64,
    pub is_error: bool,
}

/// Ejecuta un programa Wyrm usando la configuración solicitada.
///
/// # Punto de integración WASM (Fase siguiente)
/// Cuando `wyrm.js` y `wyrm.wasm` compilados desde `wyrm_core` estén listos en el
/// entorno del navegador, esta función invocará directamente el puente `wasm-bindgen` / JS
/// pasando `source_code`, `mode`, `verbose` y `stdin` (vinculado al descriptor virtual `fd 0`)
/// y devolverá el `ExecutionResult` real sin modificar ningún componente de `app.rs`.
pub fn execute(request: &ExecutionRequest) -> ExecutionResult {
    let started_at_ms = now_ms();

    // TODO(wasm-bridge): Reemplazar `execute_mock` por la llamada al módulo `wyrm.wasm`:
    //   - Interpreter => `wyrm_eval_interpreter(source, stdin, verbose)`
    //   - CompilerVm  => `wyrm_compile_and_run_vm(source, stdin, verbose)`
    execute_mock(request, started_at_ms)
}

fn execute_mock(request: &ExecutionRequest, started_at_ms: f64) -> ExecutionResult {
    let line_count = request.source_code.split('\n').count();
    let source_bytes = request.source_code.len();
    let source_chars = request.source_code.chars().count();
    let stdin_bytes = request.stdin.len();

    let stdin_summary = if request.stdin.is_empty() {
        "(vacía — 0 bytes)".to_string()
    } else {
        let preview: String = request.stdin.chars().take(64).collect();
        let ellipsis = if request.stdin.chars().count() > 64 {
            "..."
        } else {
            ""
        };
        format!("{} bytes ({:?}{})", stdin_bytes, preview, ellipsis)
    };

    let stdout = format!(
        "[wyrm-web :: mock] El motor WASM de wyrm_core aún no está enlazado.\n\
         \n\
         Parámetros recibidos por el motor:\n\
         • Modo      : {} ({})\n\
         • Verbose   : {}\n\
         • Código    : {} líneas | {} caracteres | {} bytes\n\
         • stdin fd0 : {}\n\
         \n\
         En la próxima fase se conectará wyrm.wasm / wyrm.js en `web/src/engine.rs`.",
        request.mode.label(),
        request.mode.pipeline_summary(),
        if request.verbose {
            "habilitado"
        } else {
            "deshabilitado"
        },
        line_count,
        source_chars,
        source_bytes,
        stdin_summary,
    );

    let inspector_log = if request.verbose {
        let mode_stage = match request.mode {
            ExecutionMode::Interpreter => {
                "[DEBUG][AST]    Aquí se mostrará el árbol sintáctico evaluado por el intérprete."
            }
            ExecutionMode::CompilerVm => {
                "[DEBUG][WRC]    Aquí se mostrarán las constantes, opcodes .wrc y la traza de pila de la WVM."
            }
        };

        let first_non_empty = request
            .source_code
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("(programa vacío)");

        format!(
            "[MOCK INSPECTOR :: VERBOSE]\n\
             [DEBUG][ENGINE] Estado      : Motor WASM de wyrm_core pendiente de enlace\n\
             [DEBUG][MODE]   Pipeline    : {}\n\
             [DEBUG][SOURCE] Tamaño      : {} líneas ({} bytes)\n\
             [DEBUG][SOURCE] Inicio      : {}\n\
             [DEBUG][STDIN]  Buffer fd 0 : {} bytes\n\
             [DEBUG][LEXER]  Aquí se listarán los tokens emitidos por el scanner de Wyrm.\n\
             {}",
            request.mode.pipeline_summary(),
            line_count,
            source_bytes,
            first_non_empty,
            stdin_bytes,
            mode_stage,
        )
    } else {
        String::new()
    };

    let elapsed_ms = (now_ms() - started_at_ms).max(0.0);

    ExecutionResult {
        stdout,
        return_value: Some("mock::void (WASM pendiente)".to_string()),
        inspector_log,
        elapsed_ms,
        is_error: false,
    }
}

fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .unwrap_or(0.0)
}
