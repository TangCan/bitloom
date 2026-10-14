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
    pub seq: Vec<(String, AssignExpr, Option<usize>)>,
    pub fast_comb: Vec<ResolvedAssign>,
}

pub(crate) struct ResolvedAssign {
    pub target_name: String,
    pub target_id: usize,
    pub target_width: u32,
    pub mux_coverage_id: Option<usize>,
    pub expr: ResolvedExpr,
}

#[derive(Clone, Debug)]
pub(crate) enum ResolvedExpr {
    Ref(usize),
    Lit(u64),
    Inc(usize),
    Add(usize, usize),
    Sub(usize, usize),
    And(usize, usize),
    Or(usize, usize),
    Xor(usize, usize),
    Shl(usize, usize),
    Shr(usize, usize),
    Ult {
        lhs: usize,
        rhs: usize,
        width: u32,
    },
    Slt {
        lhs: usize,
        rhs: usize,
        width: u32,
    },
    Sar {
        value: usize,
        shamt: usize,
        width: u32,
    },
    Slice {
        src: usize,
        lo: u32,
        width: u32,
    },
    Concat {
        high: usize,
        low: usize,
        low_width: u32,
    },
    ZeroExtend {
        src: usize,
        to_width: u32,
    },
    SignExtend {
        src: usize,
        from_width: u32,
        to_width: u32,
    },
    Eq(usize, usize),
    Mux {
        sel: usize,
        t: usize,
        f: usize,
    },
    MemRead {
        mem: usize,
        addr: usize,
    },
}

pub(crate) fn compile(
    hir: &FrozenHir,
    mux_coverage_ids: &std::collections::HashMap<String, usize>,
    signal_ids: &std::collections::HashMap<String, usize>,
    memory_ids: &std::collections::HashMap<String, usize>,
    widths: &std::collections::BTreeMap<String, u32>,
) -> CompiledKernel {
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
    let seq = seq
        .into_iter()
        .map(|(name, expr)| {
            let mux_id = mux_selector(&expr).and_then(|sel| mux_coverage_ids.get(sel).copied());
            (name, expr, mux_id)
        })
        .collect();
    let fast_comb = comb
        .into_iter()
        .map(|(name, expr)| {
            let mux_coverage_id =
                mux_selector(&expr).and_then(|sel| mux_coverage_ids.get(sel).copied());
            ResolvedAssign {
                target_name: name.clone(),
                target_id: signal_id(&name, signal_ids),
                target_width: widths.get(&name).copied().unwrap_or(64),
                mux_coverage_id,
                expr: resolve_expr(&expr, signal_ids, memory_ids),
            }
        })
        .collect();
    CompiledKernel { seq, fast_comb }
}

fn signal_id(name: &str, ids: &std::collections::HashMap<String, usize>) -> usize {
    ids.get(name).copied().unwrap_or(usize::MAX)
}

fn resolve_expr(
    expr: &AssignExpr,
    signals: &std::collections::HashMap<String, usize>,
    memories: &std::collections::HashMap<String, usize>,
) -> ResolvedExpr {
    use AssignExpr as A;
    use ResolvedExpr as R;
    let id = |name: &str| signal_id(name, signals);
    match expr {
        A::Ref(a) => R::Ref(id(a)),
        A::Lit(v) => R::Lit(*v),
        A::Inc(a) => R::Inc(id(a)),
        A::Add(a, b) => R::Add(id(a), id(b)),
        A::Sub(a, b) => R::Sub(id(a), id(b)),
        A::And(a, b) => R::And(id(a), id(b)),
        A::Or(a, b) => R::Or(id(a), id(b)),
        A::Xor(a, b) => R::Xor(id(a), id(b)),
        A::Shl(a, b) => R::Shl(id(a), id(b)),
        A::Shr(a, b) => R::Shr(id(a), id(b)),
        A::Ult { lhs, rhs, width } => R::Ult {
            lhs: id(lhs),
            rhs: id(rhs),
            width: *width,
        },
        A::Slt { lhs, rhs, width } => R::Slt {
            lhs: id(lhs),
            rhs: id(rhs),
            width: *width,
        },
        A::Sar {
            value,
            shamt,
            width,
        } => R::Sar {
            value: id(value),
            shamt: id(shamt),
            width: *width,
        },
        A::Slice { src, lo, width } => R::Slice {
            src: id(src),
            lo: *lo,
            width: *width,
        },
        A::Concat {
            high,
            low,
            low_width,
        } => R::Concat {
            high: id(high),
            low: id(low),
            low_width: *low_width,
        },
        A::ZeroExtend { src, to_width, .. } => R::ZeroExtend {
            src: id(src),
            to_width: *to_width,
        },
        A::SignExtend {
            src,
            from_width,
            to_width,
        } => R::SignExtend {
            src: id(src),
            from_width: *from_width,
            to_width: *to_width,
        },
        A::Eq(a, b) => R::Eq(id(a), id(b)),
        A::Mux { sel, t, f } => R::Mux {
            sel: id(sel),
            t: id(t),
            f: id(f),
        },
        A::MemRead { mem, addr } => R::MemRead {
            mem: memories.get(mem).copied().unwrap_or(usize::MAX),
            addr: id(addr),
        },
    }
}

fn mux_selector(expr: &AssignExpr) -> Option<&str> {
    if let AssignExpr::Mux { sel, .. } = expr {
        Some(sel)
    } else {
        None
    }
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
