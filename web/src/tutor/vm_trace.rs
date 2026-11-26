//! Modelos de datos y análisis de diferencias (diff) para las trazas paso a paso de Wyrm Tutor.

use serde::Deserialize;
use std::collections::HashSet;

/// Sesión completa de traza devuelta por `wyrm_web_trace_json()`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TraceSession {
    pub instructions: Vec<TraceInstruction>,
    pub constants: Vec<TraceConstant>,
    pub steps: Vec<TraceStep>,
    pub truncated: bool,
    pub max_steps: usize,
    pub total_steps: usize,
}

/// Instrucción desensamblada del chunk `.wrc`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TraceInstruction {
    pub ip: u32,
    pub opcode: String,
    pub operand: String,
    pub line: usize,
}

/// Entrada del pool de constantes del chunk `.wrc`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TraceConstant {
    pub index: usize,
    #[serde(rename = "type")]
    pub value_type: String,
    pub value: String,
}

/// Instantánea del estado completo de la VM tras ejecutar una instrucción (o paso 0 inicial).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TraceStep {
    pub step: usize,
    pub ip_before: u32,
    pub ip_after: u32,
    pub opcode: String,
    pub operand: String,
    pub line: usize,
    #[serde(default)]
    pub stdout_len: usize,
    pub frames: Vec<TraceFrame>,
    pub stack: Vec<TraceStackSlot>,
    pub heap: Vec<TraceHeapVec>,
}

/// Marco de llamada activo en la pila de ejecución.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TraceFrame {
    pub depth: usize,
    #[serde(default)]
    pub name: String,
    pub base_slot: usize,
    pub return_ip: Option<u32>,
    pub return_type: String,
}

/// Celda individual del stack con valor formateado, ancho de bits y bytes en hexadecimal.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TraceStackSlot {
    pub index: usize,
    pub frame_depth: usize,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub value_type: String,
    pub value: String,
    pub bits: u16,
    pub hex: String,
    pub vec_ptr: Option<String>,
}

/// Vector vivo en heap referenciado desde el stack durante el paso actual.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TraceHeapVec {
    pub ptr: String,
    pub elem_type: String,
    pub len: usize,
    pub ascii: Option<String>,
    pub elements: Vec<TraceHeapElem>,
}

/// Elemento individual dentro de un `vec<T>` en heap.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TraceHeapElem {
    pub index: usize,
    pub value: String,
    pub hex: String,
}

/// Cambios detectados entre dos pasos consecutivos (`steps[i - 1]` y `steps[i]`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StepDiff {
    pub changed_stack_slots: HashSet<usize>,
    pub changed_heap_ptrs: HashSet<String>,
    pub changed_heap_elems: HashSet<(String, usize)>,
    pub prev_stack_len: usize,
    pub curr_stack_len: usize,
}

impl TraceSession {
    /// Parsea la cadena JSON producida por `wyrm_web_trace_json()`.
    pub fn from_json(raw: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(raw)
            .map_err(|err| format!("No se pudo deserializar la traza de Wyrm Tutor: {err}"))
    }

    /// Devuelve el índice en `instructions` cuyo `ip` coincide con el offset indicado.
    pub fn instruction_index_for_ip(&self, ip: u32) -> Option<usize> {
        self.instructions.iter().position(|instr| instr.ip == ip)
    }

    /// Calcula las diferencias entre `steps[step_idx - 1]` y `steps[step_idx]`.
    pub fn diff_at(&self, step_idx: usize) -> StepDiff {
        let Some(curr) = self.steps.get(step_idx) else {
            return StepDiff::default();
        };
        let prev = if step_idx > 0 {
            self.steps.get(step_idx - 1)
        } else {
            None
        };
        compute_step_diff(prev, curr)
    }
}

/// Compara dos instantáneas consecutivas para identificar slots de stack y elementos de heap modificados.
pub fn compute_step_diff(prev: Option<&TraceStep>, curr: &TraceStep) -> StepDiff {
    let mut diff = StepDiff {
        changed_stack_slots: HashSet::new(),
        changed_heap_ptrs: HashSet::new(),
        changed_heap_elems: HashSet::new(),
        prev_stack_len: prev.map_or(0, |p| p.stack.len()),
        curr_stack_len: curr.stack.len(),
    };

    let Some(prev_step) = prev else {
        return diff;
    };

    for slot in &curr.stack {
        let changed = match prev_step.stack.get(slot.index) {
            None => true,
            Some(old) => {
                old.value_type != slot.value_type
                    || old.hex != slot.hex
                    || old.value != slot.value
                    || old.vec_ptr != slot.vec_ptr
            }
        };
        if changed {
            diff.changed_stack_slots.insert(slot.index);
        }
    }

    for vec in &curr.heap {
        match prev_step.heap.iter().find(|old| old.ptr == vec.ptr) {
            None => {
                diff.changed_heap_ptrs.insert(vec.ptr.clone());
                for elem in &vec.elements {
                    diff.changed_heap_elems
                        .insert((vec.ptr.clone(), elem.index));
                }
            }
            Some(old_vec) => {
                let mut vec_changed = old_vec.len != vec.len || old_vec.elem_type != vec.elem_type;
                for elem in &vec.elements {
                    let elem_changed = match old_vec.elements.get(elem.index) {
                        None => true,
                        Some(old_elem) => old_elem.hex != elem.hex || old_elem.value != elem.value,
                    };
                    if elem_changed {
                        vec_changed = true;
                        diff.changed_heap_elems
                            .insert((vec.ptr.clone(), elem.index));
                    }
                }
                if vec_changed {
                    diff.changed_heap_ptrs.insert(vec.ptr.clone());
                }
            }
        }
    }

    diff
}
