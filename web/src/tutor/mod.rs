//! Subsistema Wyrm Tutor: trazas y paneles interactivos para la VM y el AST.

pub mod ast_layout;
pub mod ast_panel;
pub mod ast_trace;
pub mod vm_panel;
pub mod vm_trace;

pub use ast_panel::AstTutorPanel;
pub use ast_trace::AstTraceSession;
pub use vm_panel::TutorPanel;
pub use vm_trace::TraceSession;
