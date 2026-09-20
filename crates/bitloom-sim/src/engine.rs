//! Tick engines: AST interpreter vs a compiled assign schedule (FR32).

use bitloom_hir::{AssignExpr, AssignTarget, FrozenHir, ProcessKind, Stmt};

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
    order_comb(&mut comb);
    CompiledKernel { seq, comb }
}

pub(crate) fn validate_single_module(hir: &FrozenHir) -> Result<(), &'static str> {
    if hir.circuit().modules.len() != 1
        || hir
            .circuit()
            .modules
            .iter()
            .any(|m| m.body.iter().any(|s| matches!(s, Stmt::Instance(_))))
    {
        Err("hierarchical simulation is unsupported: expected one module without instances")
    } else {
        Ok(())
    }
}

/// One shared dependency order for native and generated combinational execution.
/// Registers and memory banks are sources; only combinational targets form edges.
pub(crate) fn order_comb(comb: &mut Vec<(String, AssignExpr)>) {
    let assigns = std::mem::take(comb);
    let targets: std::collections::BTreeMap<_, _> = assigns
        .iter()
        .enumerate()
        .map(|(index, (name, _))| (name.as_str(), index))
        .collect();
    let mut waiting = vec![0usize; assigns.len()];
    let mut users = vec![Vec::new(); assigns.len()];
    for (index, (_, expr)) in assigns.iter().enumerate() {
        for source in dependencies(expr) {
            if let Some(&producer) = targets.get(source) {
                waiting[index] += 1;
                users[producer].push(index);
            }
        }
    }
    let mut ready: std::collections::BTreeSet<_> = waiting
        .iter()
        .enumerate()
        .filter_map(|(index, &count)| (count == 0).then_some(index))
        .collect();
    let mut order = Vec::with_capacity(assigns.len());
    while let Some(index) = ready.pop_first() {
        order.push(index);
        for &user in &users[index] {
            waiting[user] -= 1;
            if waiting[user] == 0 {
                ready.insert(user);
            }
        }
    }
    assert_eq!(
        order.len(),
        assigns.len(),
        "combinational cycle is unsupported by simulation"
    );
    let mut assigns: Vec<_> = assigns.into_iter().map(Some).collect();
    comb.extend(
        order
            .into_iter()
            .map(|index| assigns[index].take().unwrap()),
    );
}

fn dependencies(expr: &AssignExpr) -> Vec<&str> {
    match expr {
        AssignExpr::Lit(_) => vec![],
        AssignExpr::Ref(a) | AssignExpr::Inc(a) => vec![a],
        AssignExpr::Add(a, b)
        | AssignExpr::Sub(a, b)
        | AssignExpr::And(a, b)
        | AssignExpr::Or(a, b)
        | AssignExpr::Xor(a, b)
        | AssignExpr::Shl(a, b)
        | AssignExpr::Shr(a, b)
        | AssignExpr::Eq(a, b) => vec![a, b],
        AssignExpr::Ult { lhs, rhs, .. } | AssignExpr::Slt { lhs, rhs, .. } => vec![lhs, rhs],
        AssignExpr::Sar { value, shamt, .. } => vec![value, shamt],
        AssignExpr::Slice { src, .. }
        | AssignExpr::ZeroExtend { src, .. }
        | AssignExpr::SignExtend { src, .. } => vec![src],
        AssignExpr::Concat { high, low, .. } => vec![high, low],
        AssignExpr::Mux { sel, t, f } => vec![sel, t, f],
        AssignExpr::MemRead { addr, .. } => vec![addr],
    }
}
