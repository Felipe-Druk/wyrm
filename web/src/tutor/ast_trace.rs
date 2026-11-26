//! Modelos de datos, burbujeo de valores y análisis de ámbitos para el Tutor AST de Wyrm.

use serde::Deserialize;
use std::collections::{HashMap, HashSet};

/// Sesión completa de traza AST producida por `wyrm_web_ast_trace_json()`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AstTraceSession {
    pub root_id: usize,
    pub nodes: Vec<AstNodeInfo>,
    pub steps: Vec<AstTraceStep>,
    pub truncated: bool,
    pub max_steps: usize,
    pub total_steps: usize,
}

/// Nodo estático del árbol sintáctico abstracto indexado en C.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AstNodeInfo {
    pub id: usize,
    pub parent_id: Option<usize>,
    #[serde(default)]
    pub edge: String,
    pub kind: String,
    pub label: String,
    #[serde(default)]
    pub detail: String,
    #[serde(default)]
    pub children: Vec<usize>,
}

/// Evento individual de evaluación (`enter` o `exit`) sobre un nodo del AST.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AstTraceStep {
    pub step: usize,
    pub node_id: usize,
    pub phase: String,
    pub depth: usize,
    #[serde(default)]
    pub active_path: Vec<usize>,
    pub result_type: Option<String>,
    pub result_value: Option<String>,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub scopes: Vec<AstScopeFrame>,
}

impl AstTraceStep {
    /// Indica si el paso corresponde a la entrada descendente (`enter`) en el nodo.
    pub fn is_enter(&self) -> bool {
        self.phase == "enter"
    }
}

/// Fotografía de un entorno léxico (`environment_t`) activo en un paso.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AstScopeFrame {
    pub depth: usize,
    pub label: String,
    #[serde(default)]
    pub vars: Vec<AstScopeVar>,
}

/// Entrada individual (binding nombre -> tipo + valor) dentro de un `environment_t`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AstScopeVar {
    pub name: String,
    #[serde(rename = "type")]
    pub value_type: String,
    pub value: String,
}

/// Último valor resuelto de un nodo del AST tras completar su fase `exit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstResolvedValue {
    pub value: String,
    pub value_type: String,
}

/// Tipo de mutación detectada en una variable respecto al paso inmediatamente anterior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeVarChange {
    Created,
    Updated,
}

/// Diferencias de ámbito entre `steps[i - 1]` y `steps[i]`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AstScopeDiff {
    pub changes: HashMap<(usize, String), ScopeVarChange>,
}

impl AstScopeDiff {
    /// Consulta si una variable de un ámbito concreto fue creada o modificada en el paso actual.
    pub fn change_for(&self, depth: usize, name: &str) -> Option<ScopeVarChange> {
        self.changes.get(&(depth, name.to_string())).copied()
    }
}

impl AstTraceSession {
    /// Parsea la cadena JSON producida por `wyrm_web_ast_trace_json()`.
    pub fn from_json(raw: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(raw)
            .map_err(|err| format!("No se pudo deserializar la traza AST de Wyrm Tutor: {err}"))
    }

    /// Devuelve la información estática de un nodo por su identificador.
    pub fn node(&self, node_id: usize) -> Option<&AstNodeInfo> {
        if let Some(candidate) = self.nodes.get(node_id) {
            if candidate.id == node_id {
                return Some(candidate);
            }
        }
        self.nodes.iter().find(|node| node.id == node_id)
    }

    /// Recolecta el mapa de valores resueltos (`exit`) vigentes hasta `step_idx` para mostrar
    /// el burbujeo post-orden de expresiones y subárboles ya evaluados.
    ///
    /// Al reingresar (`enter`) a un subárbol (por ejemplo en una nueva iteración de bucle o
    /// invocación recursiva), limpia los valores previos de ese subárbol para evitar datos obsoletos.
    pub fn resolved_values_at(&self, step_idx: usize) -> HashMap<usize, AstResolvedValue> {
        let mut resolved = HashMap::new();
        if self.steps.is_empty() {
            return resolved;
        }
        let end = step_idx.min(self.steps.len().saturating_sub(1));

        for step in &self.steps[..=end] {
            if step.is_enter() {
                self.clear_subtree_values(step.node_id, &mut resolved);
            } else if let (Some(val), Some(ty)) = (&step.result_value, &step.result_type) {
                resolved.insert(
                    step.node_id,
                    AstResolvedValue {
                        value: val.clone(),
                        value_type: ty.clone(),
                    },
                );
            }
        }

        resolved
    }

    /// Calcula el conjunto de nodos que fueron visitados al menos una vez en toda la traza.
    pub fn executed_nodes(&self) -> HashSet<usize> {
        self.steps.iter().map(|step| step.node_id).collect()
    }

    /// Devuelve el siguiente paso que evalúa `node_id` después de `current_step`, o bien
    /// envuelve al primer paso donde aparece `node_id` (útil al hacer clic en un nodo del SVG).
    pub fn next_step_for_node(&self, node_id: usize, current_step: usize) -> Option<usize> {
        if self.steps.is_empty() {
            return None;
        }
        let start = current_step.saturating_add(1);
        if start < self.steps.len() {
            if let Some(rel) = self.steps[start..]
                .iter()
                .position(|step| step.node_id == node_id)
            {
                return Some(start + rel);
            }
        }
        self.steps.iter().position(|step| step.node_id == node_id)
    }

    /// Calcula qué variables fueron creadas o reasignadas en `steps[step_idx]` respecto al paso previo.
    pub fn scope_diff_at(&self, step_idx: usize) -> AstScopeDiff {
        let Some(curr) = self.steps.get(step_idx) else {
            return AstScopeDiff::default();
        };
        if step_idx == 0 {
            return AstScopeDiff::default();
        }
        let Some(prev) = self.steps.get(step_idx - 1) else {
            return AstScopeDiff::default();
        };

        let mut changes = HashMap::new();
        for curr_frame in &curr.scopes {
            let prev_frame = prev
                .scopes
                .iter()
                .find(|frame| frame.depth == curr_frame.depth && frame.label == curr_frame.label);

            for var in &curr_frame.vars {
                match prev_frame.and_then(|pf| pf.vars.iter().find(|old| old.name == var.name)) {
                    None => {
                        changes.insert(
                            (curr_frame.depth, var.name.clone()),
                            ScopeVarChange::Created,
                        );
                    }
                    Some(old) => {
                        if old.value != var.value || old.value_type != var.value_type {
                            changes.insert(
                                (curr_frame.depth, var.name.clone()),
                                ScopeVarChange::Updated,
                            );
                        }
                    }
                }
            }
        }

        AstScopeDiff { changes }
    }

    fn clear_subtree_values(
        &self,
        root_id: usize,
        resolved: &mut HashMap<usize, AstResolvedValue>,
    ) {
        let mut stack = vec![root_id];
        let mut visited = HashSet::new();
        while let Some(id) = stack.pop() {
            if !visited.insert(id) {
                continue;
            }
            resolved.remove(&id);
            if let Some(info) = self.node(id) {
                for &child_id in &info.children {
                    stack.push(child_id);
                }
            }
        }
    }
}
