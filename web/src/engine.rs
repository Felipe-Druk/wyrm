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
    pub return_type: String,
    pub inspector_log: String,
    pub elapsed_ms: f64,
    pub is_error: bool,
}

/// Parámetros de entrada para trazar paso a paso la ejecución en la Wyrm VM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceRequest {
    pub source_code: String,
    pub stdin: String,
    pub max_steps: u32,
}

/// Resultado estructurado de una traza paso a paso devuelto por el Web Worker.
#[derive(Debug, Clone, PartialEq)]
pub struct TraceExecutionResult {
    pub stdout: String,
    pub return_value: Option<String>,
    pub return_type: String,
    pub elapsed_ms: f64,
    pub is_error: bool,
    pub trace_json: Option<String>,
}

struct WorkerHandle {
    worker: Worker,
    _on_message: Closure<dyn FnMut(MessageEvent)>,
    _on_error: Closure<dyn FnMut(ErrorEvent)>,
}

enum JobCallback {
    Run(Box<dyn FnOnce(ExecutionResult)>),
    Trace(Box<dyn FnOnce(TraceExecutionResult)>),
}

struct ActiveJob {
    id: u64,
    started_at_ms: f64,
    callback: JobCallback,
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
    let callback = JobCallback::Run(Box::new(on_complete));
    launch_worker_job(callback, |job_id| build_worker_payload(job_id, &request));
}

/// Ejecuta y traza paso a paso un programa Wyrm en la VM dentro del Web Worker.
///
/// Comparte el mismo mecanismo de cancelación (`cancel()` / `■ DETENER`) que `execute_async`.
pub fn execute_trace_async<F>(request: TraceRequest, on_complete: F)
where
    F: FnOnce(TraceExecutionResult) + 'static,
{
    let callback = JobCallback::Trace(Box::new(on_complete));
    launch_worker_job(callback, |job_id| build_trace_payload(job_id, &request));
}

fn launch_worker_job<P>(callback: JobCallback, build_payload: P)
where
    P: FnOnce(u64) -> JsValue,
{
    let launch_outcome: Result<(), (JobCallback, String)> = ENGINE.with(|cell| {
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
                    return Err((callback, err_msg));
                }
            }
        }

        state.next_id = state.next_id.wrapping_add(1).max(1);
        let job_id = state.next_id;
        let payload = build_payload(job_id);

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
                    callback,
                });
                Ok(())
            }
            Err(err) => {
                terminate_worker_locked(&mut state);
                Err((
                    callback,
                    format!("No se pudo enviar el código al Web Worker: {err:?}"),
                ))
            }
        }
    });

    if let Err((cb, err_msg)) = launch_outcome {
        dispatch_error_callback(cb, err_msg, 0.0);
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
        dispatch_error_callback(
            job.callback,
            "Ejecución detenida por el usuario (Web Worker terminado).".to_string(),
            elapsed_ms,
        );
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

fn dispatch_error_callback(callback: JobCallback, message: String, elapsed_ms: f64) {
    match callback {
        JobCallback::Run(cb) => cb(error_result(message, elapsed_ms)),
        JobCallback::Trace(cb) => cb(trace_error_result(message, elapsed_ms)),
    }
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
        format!("No se pudo iniciar el Web Worker '{WORKER_SCRIPT_URL}': {err:?}")
    })?;

    let on_message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        let data = event.data();
        let Some(msg_id) = get_u64_prop(&data, "id") else {
            return;
        };

        let matching_job = ENGINE.with(|cell| {
            let mut state = cell.borrow_mut();
            match state.active_job.as_ref() {
                Some(job) if job.id == msg_id => state.active_job.take(),
                _ => None,
            }
        });

        let Some(job) = matching_job else {
            return;
        };

        match job.callback {
            JobCallback::Run(cb) => cb(parse_run_response(&data)),
            JobCallback::Trace(cb) => cb(parse_trace_response(&data)),
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
            dispatch_error_callback(job.callback, detail, elapsed_ms);
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
        &JsValue::from_str("action"),
        &JsValue::from_str("run"),
    );
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

fn build_trace_payload(job_id: u64, request: &TraceRequest) -> JsValue {
    let payload = Object::new();
    let _ = Reflect::set(
        &payload,
        &JsValue::from_str("action"),
        &JsValue::from_str("trace"),
    );
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
        &JsValue::from_str("stdin"),
        &JsValue::from_str(&request.stdin),
    );
    let _ = Reflect::set(
        &payload,
        &JsValue::from_str("max_steps"),
        &JsValue::from_f64(request.max_steps as f64),
    );
    payload.into()
}

fn parse_run_response(data: &JsValue) -> ExecutionResult {
    let stdout = get_string_prop(data, "stdout").unwrap_or_default();
    let return_value = get_string_prop(data, "return_value").filter(|val| !val.is_empty());
    let return_type = get_string_prop(data, "return_type").unwrap_or_default();
    let inspector_log = get_string_prop(data, "inspector_log").unwrap_or_default();
    let elapsed_ms = get_f64_prop(data, "elapsed_ms").unwrap_or(0.0);
    let is_error = get_bool_prop(data, "is_error").unwrap_or(false);

    ExecutionResult {
        stdout,
        return_value,
        return_type,
        inspector_log,
        elapsed_ms,
        is_error,
    }
}

fn parse_trace_response(data: &JsValue) -> TraceExecutionResult {
    let stdout = get_string_prop(data, "stdout").unwrap_or_default();
    let return_value = get_string_prop(data, "return_value").filter(|val| !val.is_empty());
    let return_type = get_string_prop(data, "return_type").unwrap_or_default();
    let elapsed_ms = get_f64_prop(data, "elapsed_ms").unwrap_or(0.0);
    let is_error = get_bool_prop(data, "is_error").unwrap_or(false);
    let trace_json = get_string_prop(data, "trace_json").filter(|val| !val.is_empty());

    TraceExecutionResult {
        stdout,
        return_value,
        return_type,
        elapsed_ms,
        is_error,
        trace_json,
    }
}

fn get_u64_prop(target: &JsValue, key: &str) -> Option<u64> {
    get_f64_prop(target, key).map(|num| num as u64)
}

fn get_f64_prop(target: &JsValue, key: &str) -> Option<f64> {
    Reflect::get(target, &JsValue::from_str(key))
        .ok()
        .and_then(|val| val.as_f64())
}

fn get_bool_prop(target: &JsValue, key: &str) -> Option<bool> {
    Reflect::get(target, &JsValue::from_str(key))
        .ok()
        .and_then(|val| val.as_bool())
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
        return_type: String::new(),
        inspector_log: String::new(),
        elapsed_ms,
        is_error: true,
    }
}

fn trace_error_result(message: String, elapsed_ms: f64) -> TraceExecutionResult {
    TraceExecutionResult {
        stdout: message,
        return_value: None,
        return_type: String::new(),
        elapsed_ms,
        is_error: true,
        trace_json: None,
    }
}

fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .unwrap_or(0.0)
}
