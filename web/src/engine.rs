//! Motor de ejecución asíncrono del Playground de Wyrm mediante un Web Worker + WebAssembly.

use js_sys::{Object, Reflect};
use std::cell::RefCell;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{ErrorEvent, MessageEvent, Worker};

const WORKER_SCRIPT_URL: &str = "./wyrm_worker.js";

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

    /// Código numérico esperado por `wyrm_web_exec(source, mode, verbose)`.
    pub fn as_code(self) -> i32 {
        match self {
            Self::Interpreter => 0,
            Self::CompilerVm => 1,
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

struct WorkerHandle {
    worker: Worker,
    _on_message: Closure<dyn FnMut(MessageEvent)>,
    _on_error: Closure<dyn FnMut(ErrorEvent)>,
}

struct ActiveJob {
    id: u64,
    started_at_ms: f64,
    on_complete: Box<dyn FnOnce(ExecutionResult)>,
}

#[derive(Default)]
struct EngineState {
    next_id: u64,
    worker_handle: Option<WorkerHandle>,
    worker_broken: bool,
    active_job: Option<ActiveJob>,
}

thread_local! {
    static ENGINE: RefCell<EngineState> = RefCell::new(EngineState::default());
}

/// Ejecuta un programa Wyrm de forma asíncrona dentro de un Web Worker.
///
/// Si ya había un programa corriendo, detiene el Worker anterior antes de iniciar uno nuevo.
pub fn execute_async<F>(request: ExecutionRequest, on_complete: F)
where
    F: FnOnce(ExecutionResult) + 'static,
{
    let on_complete: Box<dyn FnOnce(ExecutionResult)> = Box::new(on_complete);

    let launch_outcome: Result<(), (Box<dyn FnOnce(ExecutionResult)>, ExecutionResult)> =
        ENGINE.with(|cell| {
            let mut state = cell.borrow_mut();

            // Si había una tarea corriendo o el worker previo falló, lo reiniciamos limpiamente
            if state.active_job.is_some() || state.worker_broken {
                terminate_worker_locked(&mut state);
                state.active_job = None;
            }

            if state.worker_handle.is_none() {
                match spawn_worker() {
                    Ok(handle) => state.worker_handle = Some(handle),
                    Err(err_msg) => {
                        return Err((on_complete, error_result(err_msg, 0.0)));
                    }
                }
            }

            state.next_id = state.next_id.wrapping_add(1).max(1);
            let job_id = state.next_id;
            let payload = build_worker_payload(job_id, &request);

            let post_res = state
                .worker_handle
                .as_ref()
                .expect("worker inicializado")
                .worker
                .post_message(&payload);

            match post_res {
                Ok(()) => {
                    state.active_job = Some(ActiveJob {
                        id: job_id,
                        started_at_ms: now_ms(),
                        on_complete,
                    });
                    Ok(())
                }
                Err(err) => {
                    terminate_worker_locked(&mut state);
                    Err((
                        on_complete,
                        error_result(
                            format!("No se pudo enviar el código al Web Worker: {err:?}"),
                            0.0,
                        ),
                    ))
                }
            }
        });

    if let Err((callback, result)) = launch_outcome {
        callback(result);
    }
}

/// Detiene el Web Worker en ejecución (corta bucles infinitos) e informa la cancelación al callback.
pub fn cancel() {
    let cancelled_job = ENGINE.with(|cell| {
        let mut state = cell.borrow_mut();
        terminate_worker_locked(&mut state);
        state.active_job.take()
    });

    if let Some(job) = cancelled_job {
        let elapsed_ms = (now_ms() - job.started_at_ms).max(0.0);
        (job.on_complete)(ExecutionResult {
            stdout: "Ejecución detenida por el usuario (Web Worker terminado).".to_string(),
            return_value: None,
            inspector_log: String::new(),
            elapsed_ms,
            is_error: true,
        });
    }
}

/// Detiene el Web Worker sin emitir mensaje de cancelación (útil al limpiar el Playground).
pub fn cancel_silent() {
    ENGINE.with(|cell| {
        let mut state = cell.borrow_mut();
        terminate_worker_locked(&mut state);
        state.active_job = None;
    });
}

fn terminate_worker_locked(state: &mut EngineState) {
    if let Some(handle) = state.worker_handle.take() {
        handle.worker.set_onmessage(None);
        handle.worker.set_onerror(None);
        handle.worker.terminate();
    }
    state.worker_broken = false;
}

fn spawn_worker() -> Result<WorkerHandle, String> {
    let worker = Worker::new(WORKER_SCRIPT_URL).map_err(|err| {
        format!(
            "No se pudo iniciar el Web Worker '{WORKER_SCRIPT_URL}': {err:?}"
        )
    })?;

    let on_message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        let data = event.data();
        let Some((msg_id, result)) = parse_worker_response(&data) else {
            return;
        };

        let matching_job = ENGINE.with(|cell| {
            let mut state = cell.borrow_mut();
            match state.active_job.as_ref() {
                Some(job) if job.id == msg_id => state.active_job.take(),
                _ => None,
            }
        });

        if let Some(job) = matching_job {
            (job.on_complete)(result);
        }
    });

    let on_error = Closure::<dyn FnMut(ErrorEvent)>::new(move |event: ErrorEvent| {
        let raw_msg = event.message();
        let detail = if raw_msg.is_empty() {
            "Error inesperado dentro del Web Worker de Wyrm.".to_string()
        } else {
            format!("Error en el Web Worker de Wyrm: {raw_msg}")
        };

        // Marcamos el worker como dañado y lo terminamos sin liberar el Closure actual en el stack
        let pending_job = ENGINE.with(|cell| {
            let mut state = cell.borrow_mut();
            state.worker_broken = true;
            if let Some(handle) = state.worker_handle.as_ref() {
                handle.worker.terminate();
            }
            state.active_job.take()
        });

        if let Some(job) = pending_job {
            let elapsed_ms = (now_ms() - job.started_at_ms).max(0.0);
            (job.on_complete)(error_result(detail, elapsed_ms));
        }
    });

    worker.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    worker.set_onerror(Some(on_error.as_ref().unchecked_ref()));

    Ok(WorkerHandle {
        worker,
        _on_message: on_message,
        _on_error: on_error,
    })
}

fn build_worker_payload(job_id: u64, request: &ExecutionRequest) -> JsValue {
    let payload = Object::new();
    let _ = Reflect::set(
        &payload,
        &JsValue::from_str("id"),
        &JsValue::from_f64(job_id as f64),
    );
    let _ = Reflect::set(
        &payload,
        &JsValue::from_str("code"),
        &JsValue::from_str(&request.source_code),
    );
    let _ = Reflect::set(
        &payload,
        &JsValue::from_str("mode"),
        &JsValue::from_f64(request.mode.as_code() as f64),
    );
    let _ = Reflect::set(
        &payload,
        &JsValue::from_str("verbose"),
        &JsValue::from_bool(request.verbose),
    );
    let _ = Reflect::set(
        &payload,
        &JsValue::from_str("stdin"),
        &JsValue::from_str(&request.stdin),
    );
    payload.into()
}

fn parse_worker_response(data: &JsValue) -> Option<(u64, ExecutionResult)> {
    let id = Reflect::get(data, &JsValue::from_str("id"))
        .ok()
        .and_then(|val| val.as_f64())
        .map(|num| num as u64)?;

    let stdout = get_string_prop(data, "stdout").unwrap_or_default();
    let return_value = get_string_prop(data, "return_value").filter(|val| !val.is_empty());
    let inspector_log = get_string_prop(data, "inspector_log").unwrap_or_default();
    let elapsed_ms = Reflect::get(data, &JsValue::from_str("elapsed_ms"))
        .ok()
        .and_then(|val| val.as_f64())
        .unwrap_or(0.0);
    let is_error = Reflect::get(data, &JsValue::from_str("is_error"))
        .ok()
        .and_then(|val| val.as_bool())
        .unwrap_or(false);

    Some((
        id,
        ExecutionResult {
            stdout,
            return_value,
            inspector_log,
            elapsed_ms,
            is_error,
        },
    ))
}

fn get_string_prop(target: &JsValue, key: &str) -> Option<String> {
    Reflect::get(target, &JsValue::from_str(key))
        .ok()
        .and_then(|val| val.as_string())
}

fn error_result(message: String, elapsed_ms: f64) -> ExecutionResult {
    ExecutionResult {
        stdout: message,
        return_value: None,
        inspector_log: String::new(),
        elapsed_ms,
        is_error: true,
    }
}

fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .unwrap_or(0.0)
}
