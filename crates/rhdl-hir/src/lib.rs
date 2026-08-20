//! RHDL HIR: hierarchical FrozenHir and diagnostics.
//! Unfrozen circuit state is private to this crate.

use std::fmt;

/// Source span threaded from builder / macros (opaque for now).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub span: Span,
    pub code: String,
    pub en: String,
    pub zh: String,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({})", self.code, self.en, self.zh)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Diagnostics(pub Vec<Diagnostic>);

impl Diagnostics {
    pub fn push(&mut self, d: Diagnostic) {
        self.0.push(d);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in &self.0 {
            writeln!(f, "{d}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Diagnostics {}

/// Ground types for phase-1 HIR (AD-12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroundType {
    UInt {
        width: u32,
    },
    SInt {
        width: u32,
    },
    Clock,
    Reset,
    Bool,
    /// Top-level only (FR27).
    Analog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDirection {
    Input,
    Output,
    /// Top-level only (FR27).
    InOut,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
    pub name: String,
    pub direction: PortDirection,
    pub ty: GroundType,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessKind {
    Combinational,
    Sequential,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalKind {
    Wire,
    Reg,
    Output,
    Input,
}

/// Assignment target in a process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignTarget {
    /// Combinational net / output / wire.
    Net(String),
    /// Sequential next-state: `Reg.d`.
    RegD(String),
    /// Sequential memory write: `mem[addr] <= data` (addr is a net/reg name).
    MemWrite { mem: String, addr: String },
}

/// RHS of an assignment (phase-1 subset for emit + tick).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignExpr {
    /// Copy from a named port, wire, or reg.
    Ref(String),
    /// Integer literal.
    Lit(u64),
    /// `ref + 1` (wrapping), used for counters until full ALU lands.
    Inc(String),
    /// Same-width binary add of two refs.
    Add(String, String),
    /// Same-width binary subtract of two refs.
    Sub(String, String),
    /// Bitwise AND of two refs.
    And(String, String),
    /// Bitwise OR of two refs.
    Or(String, String),
    /// Bitwise XOR of two refs.
    Xor(String, String),
    /// Logical left shift: `a << (b & 63)`.
    Shl(String, String),
    /// Logical right shift: `a >> (b & 63)`.
    Shr(String, String),
    /// Equality compare of two refs → 0/1 (Bool).
    Eq(String, String),
    /// 2:1 mux: `sel != 0 ? t : f`.
    Mux { sel: String, t: String, f: String },
    /// Memory read `mem[addr]` (SyncReadMem: one-cycle latency when assigned in seq).
    MemRead { mem: String, addr: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign {
    pub target: AssignTarget,
    pub expr: AssignExpr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Process {
    pub kind: ProcessKind,
    pub assigns: Vec<Assign>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortConnect {
    pub child_port: String,
    pub parent_net: String,
    pub span: Span,
    /// Explicit dangling / unused input opt-out (FR9).
    pub dangling: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub name: String,
    pub module: String,
    pub connects: Vec<PortConnect>,
    pub params: Vec<(String, u32)>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    WireDecl {
        name: String,
        ty: GroundType,
        span: Span,
    },
    RegDecl {
        name: String,
        ty: GroundType,
        /// Bound clock port name (must be the module's Clock input).
        clock: String,
        /// Bound sync active-high reset port name.
        reset: String,
        /// AD-23: async reset envelope.
        async_reset: bool,
        /// AD-23: clock enable present.
        has_enable: bool,
        span: Span,
    },
    Process(Process),
    Instance(Instance),
    /// Single-clock memory (AD-21). `sync_read` distinguishes SyncReadMem vs Mem.
    MemDecl {
        name: String,
        depth: u32,
        width: u32,
        /// true => SyncReadMem (read latency 1); false => Mem (async read / reg file).
        sync_read: bool,
        span: Span,
    },
}

/// Explicit width-changing / arithmetic nodes (AD-18).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Same-width binary add; both operands must match `width`.
    Add { width: u32, span: Span },
    /// Same-width connect / drive.
    Connect { width: u32, span: Span },
    Pad {
        from_width: u32,
        to_width: u32,
        span: Span,
    },
    Trunc {
        from_width: u32,
        to_width: u32,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: String,
    pub ports: Vec<Port>,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circuit {
    pub name: String,
    pub modules: Vec<Module>,
}

/// Immutable circuit after freeze (AD-7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenHir {
    circuit: Circuit,
    pub abi_name: String,
}

impl FrozenHir {
    pub fn circuit(&self) -> &Circuit {
        &self.circuit
    }
}

/// Emitted backend file (AD-16).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmittedFile {
    pub path: String,
    pub contents: String,
}

/// Backend output bundle (AD-16).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    pub files: Vec<EmittedFile>,
    pub filelist: Vec<String>,
}

/// Port-level values shared by tick and functional models (AD-17).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PortValues {
    pub values: std::collections::BTreeMap<String, u64>,
}

impl PortValues {
    pub fn get(&self, name: &str) -> Option<u64> {
        self.values.get(name).copied()
    }

    pub fn set(&mut self, name: impl Into<String>, value: u64) {
        self.values.insert(name.into(), value);
    }
}

/// Mutable circuit owned only inside this crate during elaborate.
#[derive(Debug)]
pub(crate) struct Hir {
    pub circuit: Circuit,
}

impl Hir {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            circuit: Circuit {
                name: name.into(),
                modules: Vec::new(),
            },
        }
    }
}

/// Private freeze: only called from elaborate/import paths (AD-1, AD-7).
pub(crate) fn freeze(hir: Hir) -> Result<FrozenHir, Diagnostics> {
    if hir.circuit.modules.is_empty() {
        return Err(Diagnostics(vec![Diagnostic {
            span: Span::default(),
            code: "rhdl::E0001".into(),
            en: "circuit has no modules".into(),
            zh: "电路没有任何模块".into(),
        }]));
    }
    let mut diags = Diagnostics::default();
    for m in &hir.circuit.modules {
        validate_clock_reset(m, &mut diags);
        validate_unique_drivers(m, &mut diags);
        validate_instances(&hir.circuit, m, &mut diags);
        validate_special_io(&hir.circuit, m, &mut diags);
    }
    if !diags.is_empty() {
        return Err(diags);
    }
    let top = hir
        .circuit
        .modules
        .first()
        .map(|m| m.name.clone())
        .unwrap_or_else(|| hir.circuit.name.clone());
    Ok(FrozenHir {
        circuit: hir.circuit,
        abi_name: top,
    })
}

fn validate_special_io(circuit: &Circuit, m: &Module, diags: &mut Diagnostics) {
    let is_top = m.name == circuit.name;
    for p in &m.ports {
        let special =
            matches!(p.direction, PortDirection::InOut) || matches!(p.ty, GroundType::Analog);
        if special && !is_top {
            diags.push(Diagnostic {
                span: p.span,
                code: "rhdl::E0270".into(),
                en: format!(
                    "Analog/InOut port `{}` only allowed on top module `{}`",
                    p.name, circuit.name
                ),
                zh: format!(
                    "Analog/InOut 端口 `{}` 仅允许在顶层模块 `{}`",
                    p.name, circuit.name
                ),
            });
        }
    }
}

fn validate_clock_reset(m: &Module, diags: &mut Diagnostics) {
    let clocks: Vec<_> = m
        .ports
        .iter()
        .filter(|p| p.direction == PortDirection::Input && matches!(p.ty, GroundType::Clock))
        .collect();
    let resets: Vec<_> = m
        .ports
        .iter()
        .filter(|p| p.direction == PortDirection::Input && matches!(p.ty, GroundType::Reset))
        .collect();
    if clocks.len() != 1 {
        diags.push(Diagnostic {
            span: m.span,
            code: "rhdl::E0120".into(),
            en: format!(
                "module '{}' must have exactly one Clock input (found {})",
                m.name,
                clocks.len()
            ),
            zh: format!(
                "模块 '{}' 必须恰好有一个 Clock 输入（找到 {} 个）",
                m.name,
                clocks.len()
            ),
        });
    }
    if resets.len() != 1 {
        diags.push(Diagnostic {
            span: m.span,
            code: "rhdl::E0121".into(),
            en: format!(
                "module '{}' must have exactly one Reset input (found {})",
                m.name,
                resets.len()
            ),
            zh: format!(
                "模块 '{}' 必须恰好有一个 Reset 输入（找到 {} 个）",
                m.name,
                resets.len()
            ),
        });
    }
    if clocks.len() == 1 && resets.len() == 1 {
        let clk = &clocks[0].name;
        let rst = &resets[0].name;
        for stmt in &m.body {
            if let Stmt::RegDecl {
                name,
                clock,
                reset,
                span,
                ..
            } = stmt
            {
                if clock != clk {
                    diags.push(Diagnostic {
                        span: *span,
                        code: "rhdl::E0122".into(),
                        en: format!(
                            "Reg '{name}' must bind to module Clock '{clk}', not '{clock}'"
                        ),
                        zh: format!("寄存器 '{name}' 必须绑定模块时钟 '{clk}'，而不是 '{clock}'"),
                    });
                }
                if reset != rst {
                    diags.push(Diagnostic {
                        span: *span,
                        code: "rhdl::E0123".into(),
                        en: format!(
                            "Reg '{name}' must bind to module Reset '{rst}', not '{reset}'"
                        ),
                        zh: format!("寄存器 '{name}' 必须绑定模块复位 '{rst}'，而不是 '{reset}'"),
                    });
                }
            }
        }
    }
}

fn validate_unique_drivers(m: &Module, diags: &mut Diagnostics) {
    use std::collections::HashMap;
    // Multi-drive is across processes; within one process, if/else may assign the same net.
    let mut drivers: HashMap<String, Vec<Span>> = HashMap::new();
    for stmt in &m.body {
        if let Stmt::Process(p) = stmt {
            let mut seen_in_process = std::collections::HashSet::new();
            for a in &p.assigns {
                let key = match &a.target {
                    AssignTarget::Net(n) => n.clone(),
                    AssignTarget::RegD(n) => format!("{n}.d"),
                    AssignTarget::MemWrite { mem, addr } => format!("{mem}[{addr}]"),
                };
                if seen_in_process.insert(key.clone()) {
                    drivers.entry(key).or_default().push(a.span);
                }
            }
        }
    }
    for (net, spans) in drivers {
        if spans.len() > 1 {
            diags.push(Diagnostic {
                span: spans[1],
                code: "rhdl::E0140".into(),
                en: format!("multiple drivers for '{net}' ({} drivers)", spans.len()),
                zh: format!("'{net}' 有多个驱动（{} 个）", spans.len()),
            });
        }
    }
}

fn validate_instances(circuit: &Circuit, parent: &Module, diags: &mut Diagnostics) {
    use std::collections::HashMap;
    let modules: HashMap<&str, &Module> = circuit
        .modules
        .iter()
        .map(|m| (m.name.as_str(), m))
        .collect();
    for stmt in &parent.body {
        let Stmt::Instance(inst) = stmt else {
            continue;
        };
        let Some(child) = modules.get(inst.module.as_str()) else {
            diags.push(Diagnostic {
                span: inst.span,
                code: "rhdl::E0201".into(),
                en: format!("unknown child module '{}'", inst.module),
                zh: format!("未知子模块 '{}'", inst.module),
            });
            continue;
        };
        let connected: HashMap<&str, &PortConnect> = inst
            .connects
            .iter()
            .map(|c| (c.child_port.as_str(), c))
            .collect();
        for port in &child.ports {
            match connected.get(port.name.as_str()) {
                None if port.direction == PortDirection::Input => {
                    diags.push(Diagnostic {
                        span: inst.span,
                        code: "rhdl::E0202".into(),
                        en: format!(
                            "undriven child input '{}.{}' (mark dangling if intentional)",
                            inst.name, port.name
                        ),
                        zh: format!(
                            "子模块输入 '{}.{}' 未驱动（若故意悬空请标记 dangling）",
                            inst.name, port.name
                        ),
                    });
                }
                Some(c) if c.dangling && port.direction == PortDirection::Input => {}
                Some(c) => {
                    // Width check against parent net if present in parent ports/wires.
                    let parent_ty = parent
                        .ports
                        .iter()
                        .find(|p| p.name == c.parent_net)
                        .map(|p| &p.ty)
                        .or_else(|| {
                            parent.body.iter().find_map(|s| match s {
                                Stmt::WireDecl { name, ty, .. }
                                | Stmt::RegDecl { name, ty, .. }
                                    if name == &c.parent_net =>
                                {
                                    Some(ty)
                                }
                                _ => None,
                            })
                        });
                    if let Some(pty) = parent_ty {
                        let pw = width_of(pty);
                        let cw = width_of(&port.ty);
                        if pw != cw {
                            diags.push(Diagnostic {
                                span: c.span,
                                code: "rhdl::E0203".into(),
                                en: format!(
                                    "width mismatch connecting '{}' (parent {pw}) to '{}.{}' (child {cw})",
                                    c.parent_net, inst.name, port.name
                                ),
                                zh: format!(
                                    "连接位宽不匹配：'{}'（父 {pw}）→ '{}.{}'（子 {cw}）",
                                    c.parent_net, inst.name, port.name
                                ),
                            });
                        }
                    } else if !c.dangling {
                        diags.push(Diagnostic {
                            span: c.span,
                            code: "rhdl::E0204".into(),
                            en: format!(
                                "cannot resolve parent net '{}' when connecting to '{}.{}'",
                                c.parent_net, inst.name, port.name
                            ),
                            zh: format!(
                                "连接 '{}.{}' 时无法解析父网 '{}'",
                                inst.name, port.name, c.parent_net
                            ),
                        });
                    }
                }
                None => {}
            }
        }
    }
}

fn width_of(ty: &GroundType) -> u32 {
    match ty {
        GroundType::UInt { width } | GroundType::SInt { width } => *width,
        GroundType::Clock | GroundType::Reset | GroundType::Bool | GroundType::Analog => 1,
    }
}

/// Crate-internal entry used by builder to finish elaborate.
pub fn seal_from_builder(hir: BuilderOwnedHir) -> Result<FrozenHir, Diagnostics> {
    freeze(hir.0)
}

/// Opaque wrapper so builder can hold Hir without re-exporting the type.
pub struct BuilderOwnedHir(pub(crate) Hir);

impl BuilderOwnedHir {
    pub fn new(name: impl Into<String>) -> Self {
        Self(Hir::new(name))
    }

    pub fn add_module(&mut self, module: Module) {
        self.0.circuit.modules.push(module);
    }

    pub fn circuit_mut(&mut self) -> &mut Circuit {
        &mut self.0.circuit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freeze_requires_module() {
        let hir = BuilderOwnedHir::new("empty");
        assert!(seal_from_builder(hir).is_err());
    }

    #[test]
    fn freeze_simple_module() {
        let mut hir = BuilderOwnedHir::new("Top");
        hir.add_module(Module {
            name: "Top".into(),
            ports: vec![
                Port {
                    name: "clk".into(),
                    direction: PortDirection::Input,
                    ty: GroundType::Clock,
                    span: Span::default(),
                },
                Port {
                    name: "rst".into(),
                    direction: PortDirection::Input,
                    ty: GroundType::Reset,
                    span: Span::default(),
                },
                Port {
                    name: "data_in".into(),
                    direction: PortDirection::Input,
                    ty: GroundType::UInt { width: 8 },
                    span: Span::default(),
                },
                Port {
                    name: "data_out".into(),
                    direction: PortDirection::Output,
                    ty: GroundType::UInt { width: 8 },
                    span: Span::default(),
                },
            ],
            body: vec![],
            span: Span::default(),
        });
        let frozen = seal_from_builder(hir).unwrap();
        assert_eq!(frozen.abi_name, "Top");
        assert_eq!(frozen.circuit().modules[0].ports.len(), 4);
    }

    #[test]
    fn analog_on_top_ok() {
        let mut hir = BuilderOwnedHir::new("PadTop");
        hir.add_module(Module {
            name: "PadTop".into(),
            ports: vec![
                Port {
                    name: "clk".into(),
                    direction: PortDirection::Input,
                    ty: GroundType::Clock,
                    span: Span::default(),
                },
                Port {
                    name: "rst".into(),
                    direction: PortDirection::Input,
                    ty: GroundType::Reset,
                    span: Span::default(),
                },
                Port {
                    name: "pad".into(),
                    direction: PortDirection::InOut,
                    ty: GroundType::Analog,
                    span: Span::default(),
                },
            ],
            body: vec![],
            span: Span::default(),
        });
        assert!(seal_from_builder(hir).is_ok());
    }

    #[test]
    fn analog_on_non_top_rejected() {
        let mut hir = BuilderOwnedHir::new("Top");
        hir.add_module(Module {
            name: "Child".into(),
            ports: vec![
                Port {
                    name: "clk".into(),
                    direction: PortDirection::Input,
                    ty: GroundType::Clock,
                    span: Span::default(),
                },
                Port {
                    name: "rst".into(),
                    direction: PortDirection::Input,
                    ty: GroundType::Reset,
                    span: Span::default(),
                },
                Port {
                    name: "pad".into(),
                    direction: PortDirection::InOut,
                    ty: GroundType::Analog,
                    span: Span::default(),
                },
            ],
            body: vec![],
            span: Span::default(),
        });
        let err = seal_from_builder(hir).unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0270"));
    }
}
