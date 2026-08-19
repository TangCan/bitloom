//! Tick engines: AST interpreter vs a compiled assign schedule (FR32).

use rhdl_hir::{AssignExpr, AssignTarget, FrozenHir, ProcessKind, Stmt};

/// Select how `Sim::tick` evaluates FrozenHir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TickEngine {
    /// Walk module processes each cycle (debugging-friendly).
    #[default]
    Interpreter,
    /// Execute a linearized assign schedule compiled at `Sim` construction.
    Compiled,
}

impl TickEngine {
    /// Parse CLI / docs names: `interpreter` | `compiled`.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "interpreter" | "interp" => Some(Self::Interpreter),
            "compiled" | "compile" => Some(Self::Compiled),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Interpreter => "interpreter",
            Self::Compiled => "compiled",
        }
    }
}

pub(crate) struct CompiledKernel {
    pub seq: Vec<(String, AssignExpr)>,
    pub comb: Vec<(String, AssignExpr)>,
}

pub(crate) fn compile(hir: &FrozenHir) -> CompiledKernel {
    let mut seq = Vec::new();
    let mut comb = Vec::new();
    if let Some(m) = hir.circuit().modules.first() {
        for stmt in &m.body {
            if let Stmt::Process(p) = stmt {
                for a in &p.assigns {
                    match (p.kind, &a.target) {
                        (ProcessKind::Sequential, AssignTarget::RegD(name)) => {
                            seq.push((name.clone(), a.expr.clone()));
                        }
                        (ProcessKind::Combinational, AssignTarget::Net(name)) => {
                            comb.push((name.clone(), a.expr.clone()));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    CompiledKernel { seq, comb }
}
