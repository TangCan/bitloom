//! Builder session: the only public mutation path into HIR (AD-13).

use std::collections::{HashMap, HashSet};

use bitloom_hir::{
    Assign, AssignExpr, AssignTarget, BuilderOwnedHir, Module, Port, PortDirection, Process,
    ProcessKind, Stmt,
};

pub use bitloom_hir::{
    Diagnostic, Diagnostics, Diagnostics as HirDiagnostics, FrozenHir, FrozenHir as Frozen,
    GroundType, SignalKind, Span,
};

/// Mask a mem init word to `width` bits (MVP: width ≤ 64).
pub fn mask_mem_word(word: u64, width: u32) -> u64 {
    if width == 0 {
        0
    } else if width >= 64 {
        word
    } else {
        word & ((1u64 << width) - 1)
    }
}

/// Elaborate-time LUT/ROM table builder (FR73). Runs `f(addr)` for each address;
/// returns plain words — the closure never enters HIR.
pub fn generate_mem_init_words<F>(depth: u32, width: u32, f: F) -> Vec<u64>
where
    F: Fn(usize) -> u64,
{
    (0..depth as usize)
        .map(|i| mask_mem_word(f(i), width))
        .collect()
}

/// Kind of hardware reference that must not be captured into elaborate-time
/// generator / factory closures (FR73 / NFR35 / AD-18).
///
/// Session `Wire` / `Reg` / port names map to these kinds via [`HwCaptureRef`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HwCaptureKind {
    Wire,
    Reg,
    /// Directed port or other named hardware signal handle.
    Signal,
}

/// Documented illegal-capture marker for Wire/Reg/signal refs (Story 27.3).
///
/// Elaborate-time generator APIs accept only **non-capturing** `Fn` that dissolve
/// before freeze. Capturing a hardware ref is diagnosed via
/// [`ElaborateSession::assert_no_hw_capture`] / [`ElaborateSession::reject_hw_capture`]
/// (`rhdl::E0142`) — not silently treated as a legal generator closure.
///
/// Cycle-accurate “capturing closure” bans remain [`ElaborateSession::reject_unsynthesizable`]
/// (`rhdl::E0141` / FR16).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HwCaptureRef {
    pub kind: HwCaptureKind,
    pub name: String,
}

impl HwCaptureRef {
    /// Map a Wire net name to the illegal-capture marker.
    pub fn wire(name: impl Into<String>) -> Self {
        Self {
            kind: HwCaptureKind::Wire,
            name: name.into(),
        }
    }

    /// Map a Reg name to the illegal-capture marker.
    pub fn reg(name: impl Into<String>) -> Self {
        Self {
            kind: HwCaptureKind::Reg,
            name: name.into(),
        }
    }

    /// Map a port / signal handle name to the illegal-capture marker.
    pub fn signal(name: impl Into<String>) -> Self {
        Self {
            kind: HwCaptureKind::Signal,
            name: name.into(),
        }
    }

    fn kind_label(&self) -> &'static str {
        match self.kind {
            HwCaptureKind::Wire => "Wire",
            HwCaptureKind::Reg => "Reg",
            HwCaptureKind::Signal => "Signal",
        }
    }
}

/// Documented constraints for closures that may enter the synthesizable hardware
/// path (FR74 / Cap-R-48…50 / AD-18).
///
/// A legal synthesizable closure is:
/// - **pure** (no side effects / I/O / threads) — Cap-R-50
/// - **no heap** (`Box` / software `Vec` / `String` / …) — Cap-R-48
/// - **no runtime capture state** (non-`const` captures; Wire/Reg still → E0142) — Cap-R-49
/// - dissolved to ordinary HIR **before** freeze — never a Rust closure object in
///   `tick` / FIRRTL / Chisel (NFR36 / Cap-R-58)
///
/// Story 28.1 delivers the **constraint surface + check hook** only; comb/seq
/// inline expansion is Story 28.2 / 28.3. Automatic rustc capture analysis is
/// out of scope — macros / ATDD / typed surfaces call
/// [`ElaborateSession::check_synthesizable_closure`].
///
/// Empty / simple stand-ins ([`LegalEmptyClosure`], [`LegalSimpleClosure`])
/// implement this marker with no violations (paves 28.2).
pub trait SynthesizableClosure {
    /// Documented violation tokens for Cap-R-60 checking. Empty = legal.
    fn synthesizable_closure_violations(&self) -> Vec<SynthesizableClosureViolation> {
        Vec::new()
    }
}

/// Positive stand-in: empty non-capturing closure (FR74 ATDD).
#[derive(Debug, Default, Clone, Copy)]
pub struct LegalEmptyClosure;

impl SynthesizableClosure for LegalEmptyClosure {}

/// Positive stand-in: simple pure unary transform (FR74; paves comb inline 28.2).
#[derive(Debug, Default, Clone, Copy)]
pub struct LegalSimpleClosure;

impl SynthesizableClosure for LegalSimpleClosure {}

/// Kind of SynthesizableClosure constraint breach (FR74 / Cap-R-48…50).
///
/// Distinct from FR16 [`ElaborateSession::reject_unsynthesizable`] (`rhdl::E0141`)
/// and hardware-ref capture [`ElaborateSession::reject_hw_capture`] (`rhdl::E0142`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthesizableClosureViolationKind {
    /// Heap allocation in closure body / environment (Cap-R-48) → `rhdl::E0143`.
    Heap,
    /// Runtime (non-const) capture state (Cap-R-49) → `rhdl::E0144`.
    RuntimeCaptureState,
    /// Impure / side-effecting body (Cap-R-50) → `rhdl::E0145`.
    Impure,
}

impl SynthesizableClosureViolationKind {
    pub fn code(self) -> &'static str {
        match self {
            Self::Heap => "rhdl::E0143",
            Self::RuntimeCaptureState => "rhdl::E0144",
            Self::Impure => "rhdl::E0145",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Heap => "heap allocation",
            Self::RuntimeCaptureState => "runtime capture state",
            Self::Impure => "impure / side-effecting body",
        }
    }
}

/// Documented SynthesizableClosure violation token (Story 28.1 / Cap-R-60).
///
/// Pass to [`ElaborateSession::reject_unsynthesizable_closure`] /
/// [`ElaborateSession::check_synthesizable_closure`]. Empty violation list =
/// legal empty/simple closure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesizableClosureViolation {
    pub kind: SynthesizableClosureViolationKind,
    pub detail: String,
}

impl SynthesizableClosureViolation {
    pub fn heap(detail: impl Into<String>) -> Self {
        Self {
            kind: SynthesizableClosureViolationKind::Heap,
            detail: detail.into(),
        }
    }

    pub fn runtime_capture_state(detail: impl Into<String>) -> Self {
        Self {
            kind: SynthesizableClosureViolationKind::RuntimeCaptureState,
            detail: detail.into(),
        }
    }

    pub fn impure(detail: impl Into<String>) -> Self {
        Self {
            kind: SynthesizableClosureViolationKind::Impure,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> &'static str {
        self.kind.code()
    }
}

/// Cap-R-60 check surface: diagnose SynthesizableClosure violations without a
/// session (CLI / `cargo bitloom check` can call this; session methods wrap it).
pub fn diagnose_synthesizable_closure_violations(
    violations: &[SynthesizableClosureViolation],
    span: Span,
) -> Diagnostics {
    let mut diags = Diagnostics::default();
    for v in violations {
        diags.push(Diagnostic {
            span,
            code: v.code().into(),
            en: format!(
                "synthesizable-closure violation: {} ({}); \
                 SynthesizableClosure requires pure, no-heap, no runtime capture state \
                 (FR74 / Cap-R-48…50)",
                v.kind.label(),
                v.detail
            ),
            zh: format!(
                "可综合闭包违规：{}（{}）；SynthesizableClosure 要求纯函数、无堆、无运行时捕获状态（FR74 / Cap-R-48…50）",
                v.kind.label(),
                v.detail
            ),
        });
    }
    diags
}

/// Plain instance spec produced by an elaborate-time factory `Fn` (FR73 / Cap-R-53).
///
/// Dissolves to ordinary [`bitloom_hir::Stmt::Instance`] via
/// [`ElaborateSession::generate_instances_from`] / [`ElaborateSession::add_instance`].
/// Never stored as a closure in FrozenHir (NFR36).
#[derive(Debug, Clone)]
pub struct GeneratedInstance {
    pub name: String,
    pub module: String,
    /// `(child_port, parent_net)` pairs — same as [`ElaborateSession::add_instance`].
    pub connects: Vec<(String, String)>,
    pub params: Vec<(String, u32)>,
}

impl GeneratedInstance {
    pub fn new(
        name: impl Into<String>,
        module: impl Into<String>,
        connects: Vec<(String, String)>,
        params: Vec<(String, u32)>,
    ) -> Self {
        Self {
            name: name.into(),
            module: module.into(),
            connects,
            params,
        }
    }
}

#[derive(Debug)]
enum ProcessState {
    Combinational {
        assigns: Vec<Assign>,
        /// Assignment sets along the current path stack for latch analysis.
        path_assigned: Vec<HashSet<String>>,
        /// Open branches: then-set collected so far, optional else placeholder.
        pending_branches: Vec<(HashSet<String>, bool)>,
        span: Span,
    },
    Sequential {
        assigns: Vec<Assign>,
        span: Span,
    },
}

/// Session token holding the unfrozen circuit (AD-13).
pub struct ElaborateSession {
    hir: BuilderOwnedHir,
    current: Option<Module>,
    /// name -> kind for the current module
    signals: HashMap<String, SignalKind>,
    /// name -> bit width for UInt/SInt (Clock/Reset/Bool use 1)
    widths: HashMap<String, u32>,
    /// Phantom clock-domain id per signal (AD-22); default 0.
    domains: HashMap<String, u32>,
    /// Signals that may legally cross domains (DoubleFlop/SyncFIFO bridges).
    cdc_bridges: HashSet<String>,
    clock_port: Option<String>,
    reset_port: Option<String>,
    process: Option<ProcessState>,
    errors: Diagnostics,
}

impl ElaborateSession {
    pub fn new(circuit_name: impl Into<String>) -> Self {
        Self {
            hir: BuilderOwnedHir::new(circuit_name),
            current: None,
            signals: HashMap::new(),
            widths: HashMap::new(),
            domains: HashMap::new(),
            cdc_bridges: HashSet::new(),
            clock_port: None,
            reset_port: None,
            process: None,
            errors: Diagnostics::default(),
        }
    }

    fn push_err(&mut self, d: Diagnostic) {
        self.errors.push(d);
    }

    pub fn begin_module(&mut self, name: impl Into<String>, span: Span) {
        self.signals.clear();
        self.widths.clear();
        self.domains.clear();
        self.cdc_bridges.clear();
        self.clock_port = None;
        self.reset_port = None;
        self.process = None;
        self.current = Some(Module {
            name: name.into(),
            ports: Vec::new(),
            body: Vec::new(),
            span,
        });
    }

    /// Bind a phantom clock-domain id to a signal (AD-22).
    pub fn bind_domain(&mut self, name: impl Into<String>, domain: u32) {
        self.domains.insert(name.into(), domain);
    }

    /// Mark a CDC bridge signal that may legally cross domains.
    pub fn mark_cdc_bridge(&mut self, name: impl Into<String>) {
        self.cdc_bridges.insert(name.into());
    }

    fn record_width(&mut self, name: &str, ty: &GroundType) {
        let w = match ty {
            GroundType::UInt { width } | GroundType::SInt { width } => *width,
            GroundType::Clock | GroundType::Reset | GroundType::Bool | GroundType::Analog => 1,
        };
        self.widths.insert(name.to_string(), w);
    }

    /// Fail-before-emit when a flattened leaf / port name collides (FR51).
    fn ensure_fresh_signal_name(&mut self, name: &str, span: Span) -> bool {
        if self.signals.contains_key(name) {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0152".into(),
                en: format!(
                    "flattened leaf/port name '{name}' collides with an existing signal (rename Bundle members or fields so `{{field}}_{{member}}` / `{{field}}_{{i}}` stay unique)"
                ),
                zh: format!(
                    "展平叶/端口名 '{name}' 与已有信号冲突（请调整 Bundle 成员或字段名，保证 `{{field}}_{{member}}` / `{{field}}_{{i}}` 唯一）"
                ),
            });
            return false;
        }
        true
    }

    pub fn add_input(&mut self, name: impl Into<String>, ty: GroundType, span: Span) {
        let name = name.into();
        if !self.ensure_fresh_signal_name(&name, span) {
            return;
        }
        if matches!(ty, GroundType::Clock) {
            self.clock_port = Some(name.clone());
        }
        if matches!(ty, GroundType::Reset) {
            self.reset_port = Some(name.clone());
        }
        self.signals.insert(name.clone(), SignalKind::Input);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.ports.push(Port {
                name,
                direction: PortDirection::Input,
                ty,
                span,
            });
        }
    }

    pub fn add_output(&mut self, name: impl Into<String>, ty: GroundType, span: Span) {
        let name = name.into();
        if !self.ensure_fresh_signal_name(&name, span) {
            return;
        }
        self.signals.insert(name.clone(), SignalKind::Output);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.ports.push(Port {
                name,
                direction: PortDirection::Output,
                ty,
                span,
            });
        }
    }

    /// Top-level InOut / Analog IO (FR27). Non-top uses are rejected at freeze.
    pub fn add_inout(&mut self, name: impl Into<String>, ty: GroundType, span: Span) {
        let name = name.into();
        if !self.ensure_fresh_signal_name(&name, span) {
            return;
        }
        self.signals.insert(name.clone(), SignalKind::Wire);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.ports.push(Port {
                name,
                direction: PortDirection::InOut,
                ty,
                span,
            });
        }
    }

    pub fn declare_wire(&mut self, name: impl Into<String>, ty: GroundType, span: Span) {
        let name = name.into();
        self.signals.insert(name.clone(), SignalKind::Wire);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.body.push(Stmt::WireDecl { name, ty, span });
        }
    }

    pub fn declare_reg(&mut self, name: impl Into<String>, ty: GroundType, span: Span) {
        let name = name.into();
        let (Some(clock), Some(reset)) = (self.clock_port.clone(), self.reset_port.clone()) else {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0124".into(),
                en: "cannot declare Reg before Clock and Reset ports are declared".into(),
                zh: "声明寄存器前必须先有 Clock 与 Reset 端口".into(),
            });
            return;
        };
        self.signals.insert(name.clone(), SignalKind::Reg);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.body.push(Stmt::RegDecl {
                name,
                ty,
                clock,
                reset,
                async_reset: false,
                has_enable: false,
                span,
            });
        }
    }

    /// Declare a register with optional async reset / clock enable (AD-23).
    pub fn declare_reg_ex(
        &mut self,
        name: impl Into<String>,
        ty: GroundType,
        async_reset: bool,
        has_enable: bool,
        span: Span,
    ) {
        let name = name.into();
        let (Some(clock), Some(reset)) = (self.clock_port.clone(), self.reset_port.clone()) else {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0124".into(),
                en: "cannot declare Reg before Clock and Reset ports are declared".into(),
                zh: "声明寄存器前必须先有 Clock 与 Reset 端口".into(),
            });
            return;
        };
        self.signals.insert(name.clone(), SignalKind::Reg);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.body.push(Stmt::RegDecl {
                name,
                ty,
                clock,
                reset,
                async_reset,
                has_enable,
                span,
            });
        }
    }

    /// Declare SyncReadMem (CHIRRTL-friendly; sync_read=true).
    pub fn declare_sync_read_mem(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        span: Span,
    ) {
        self.declare_mem_inner(name, depth, width, true, None, span);
    }

    /// Declare Mem (async-read / reg-file style; sync_read=false).
    pub fn declare_mem(&mut self, name: impl Into<String>, depth: u32, width: u32, span: Span) {
        self.declare_mem_inner(name, depth, width, false, None, span);
    }

    /// Declare Mem with elaborate-time init words (FR73). Prefer
    /// [`Self::declare_mem_with_init_fn`] for generator closures.
    pub fn declare_mem_with_init(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        init: Vec<u64>,
        span: Span,
    ) {
        self.declare_mem_inner(name, depth, width, false, Some(init), span);
    }

    /// Declare SyncReadMem with elaborate-time init words (FR73).
    pub fn declare_sync_read_mem_with_init(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        init: Vec<u64>,
        span: Span,
    ) {
        self.declare_mem_inner(name, depth, width, true, Some(init), span);
    }

    /// Elaborate-time Mem init generator (FR73 / AD-18).
    ///
    /// Runs `f(addr)` for each address inside the session, masks to `width` bits,
    /// and stores plain `Vec<u64>` on the HIR mem node. The closure does **not**
    /// enter FrozenHir (NFR36). Only **non-capturing** `Fn` is legal; capturing
    /// Wire/Reg/signal refs must be rejected via
    /// [`Self::assert_no_hw_capture`] (`rhdl::E0142`).
    pub fn declare_mem_with_init_fn<F>(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        f: F,
        span: Span,
    ) where
        F: Fn(usize) -> u64,
    {
        let init = generate_mem_init_words(depth, width, f);
        self.declare_mem_inner(name, depth, width, false, Some(init), span);
    }

    /// SyncReadMem variant of [`Self::declare_mem_with_init_fn`] (FR73).
    pub fn declare_sync_read_mem_with_init_fn<F>(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        f: F,
        span: Span,
    ) where
        F: Fn(usize) -> u64,
    {
        let init = generate_mem_init_words(depth, width, f);
        self.declare_mem_inner(name, depth, width, true, Some(init), span);
    }

    fn declare_mem_inner(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        sync_read: bool,
        init: Option<Vec<u64>>,
        span: Span,
    ) {
        let name = name.into();
        if depth == 0 || width == 0 {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0210".into(),
                en: "Mem depth and width must be non-zero".into(),
                zh: "Mem 的 depth 与 width 必须非零".into(),
            });
            return;
        }
        let init = match init {
            None => None,
            Some(words) => {
                if width > 64 {
                    self.push_err(Diagnostic {
                        span,
                        code: "rhdl::E0211".into(),
                        en: "Mem init path supports width ≤ 64 for this MVP".into(),
                        zh: "本 MVP 的 Mem 初值路径仅支持 width ≤ 64".into(),
                    });
                    return;
                }
                if words.len() != depth as usize {
                    self.push_err(Diagnostic {
                        span,
                        code: "rhdl::E0212".into(),
                        en: format!(
                            "Mem init length {} does not match depth {depth}",
                            words.len()
                        ),
                        zh: format!("Mem 初值长度 {} 与 depth {depth} 不一致", words.len()),
                    });
                    return;
                }
                Some(words.into_iter().map(|w| mask_mem_word(w, width)).collect())
            }
        };
        self.signals.insert(name.clone(), SignalKind::Wire);
        self.widths.insert(name.clone(), width);
        if let Some(m) = self.current.as_mut() {
            m.body.push(Stmt::MemDecl {
                name,
                depth,
                width,
                sync_read,
                init,
                span,
            });
        }
    }

    /// Same-width binary add. Returns Err diagnostic via session if widths differ.
    pub fn check_add(&mut self, lhs: &str, rhs: &str, span: Span) -> Option<u32> {
        let lw = self.widths.get(lhs).copied();
        let rw = self.widths.get(rhs).copied();
        match (lw, rw) {
            (Some(a), Some(b)) if a == b => Some(a),
            (Some(a), Some(b)) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0130".into(),
                    en: format!(
                        "add requires same width; '{lhs}' is {a}, '{rhs}' is {b} (use pad/trunc)"
                    ),
                    zh: format!("加法要求同位宽；'{lhs}' 为 {a}，'{rhs}' 为 {b}（请用 pad/trunc）"),
                });
                None
            }
            _ => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0113".into(),
                    en: format!("unknown signal in add ('{lhs}', '{rhs}')"),
                    zh: format!("加法中有未知信号（'{lhs}', '{rhs}'）"),
                });
                None
            }
        }
    }

    pub fn check_connect(&mut self, lhs: &str, rhs: &str, span: Span) -> Option<u32> {
        let lw = self.widths.get(lhs).copied();
        let rw = self.widths.get(rhs).copied();
        match (lw, rw) {
            (Some(a), Some(b)) if a == b => Some(a),
            (Some(a), Some(b)) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0131".into(),
                    en: format!(
                        "connect requires same width; '{lhs}' is {a}, '{rhs}' is {b} (use pad/trunc)"
                    ),
                    zh: format!(
                        "连接要求同位宽；'{lhs}' 为 {a}，'{rhs}' 为 {b}（请用 pad/trunc）"
                    ),
                });
                None
            }
            _ => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0113".into(),
                    en: format!("unknown signal in connect ('{lhs}', '{rhs}')"),
                    zh: format!("连接中有未知信号（'{lhs}', '{rhs}'）"),
                });
                None
            }
        }
    }

    /// Explicit zero-extend / sign-pad to a wider width; records result as a temp wire name.
    pub fn pad_to(
        &mut self,
        src: &str,
        to_width: u32,
        dest: impl Into<String>,
        span: Span,
    ) -> bool {
        let Some(from) = self.widths.get(src).copied() else {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0113".into(),
                en: format!("unknown signal '{src}' in pad"),
                zh: format!("pad 中未知信号 '{src}'"),
            });
            return false;
        };
        if to_width <= from {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0132".into(),
                en: format!("pad requires to_width > from_width ({to_width} <= {from})"),
                zh: format!("pad 要求目标位宽大于源位宽（{to_width} <= {from}）"),
            });
            return false;
        }
        let dest = dest.into();
        self.declare_wire(dest, GroundType::UInt { width: to_width }, span);
        let _ = bitloom_hir::Expr::Pad {
            from_width: from,
            to_width,
            span,
        };
        true
    }

    pub fn trunc_to(
        &mut self,
        src: &str,
        to_width: u32,
        dest: impl Into<String>,
        span: Span,
    ) -> bool {
        let Some(from) = self.widths.get(src).copied() else {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0113".into(),
                en: format!("unknown signal '{src}' in trunc"),
                zh: format!("trunc 中未知信号 '{src}'"),
            });
            return false;
        };
        if to_width >= from {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0133".into(),
                en: format!("trunc requires to_width < from_width ({to_width} >= {from})"),
                zh: format!("trunc 要求目标位宽小于源位宽（{to_width} >= {from}）"),
            });
            return false;
        }
        let dest = dest.into();
        self.declare_wire(dest, GroundType::UInt { width: to_width }, span);
        let _ = bitloom_hir::Expr::Trunc {
            from_width: from,
            to_width,
            span,
        };
        true
    }

    pub fn begin_combinational(&mut self, span: Span) {
        if self.process.is_some() {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0101".into(),
                en: "nested processes are not allowed".into(),
                zh: "不允许嵌套硬件过程".into(),
            });
            return;
        }
        self.process = Some(ProcessState::Combinational {
            assigns: Vec::new(),
            path_assigned: vec![HashSet::new()],
            pending_branches: Vec::new(),
            span,
        });
    }

    pub fn begin_sequential(&mut self, span: Span) {
        if self.process.is_some() {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0101".into(),
                en: "nested processes are not allowed".into(),
                zh: "不允许嵌套硬件过程".into(),
            });
            return;
        }
        self.process = Some(ProcessState::Sequential {
            assigns: Vec::new(),
            span,
        });
    }

    /// Start an if-branch inside a combinational process (latch analysis).
    pub fn begin_then(&mut self, span: Span) {
        let err = match self.process.as_mut() {
            Some(ProcessState::Combinational {
                path_assigned,
                pending_branches,
                ..
            }) => {
                pending_branches.push((HashSet::new(), false));
                path_assigned.push(HashSet::new());
                None
            }
            Some(ProcessState::Sequential { .. }) => Some(Diagnostic {
                span,
                code: "rhdl::E0102".into(),
                en: "branch tracking for latch checks is only valid in combinational processes"
                    .into(),
                zh: "仅组合过程支持 if/else 赋值完整性检查".into(),
            }),
            None => Some(Diagnostic {
                span,
                code: "rhdl::E0103".into(),
                en: "assignment control outside a marked combinational/sequential process".into(),
                zh: "在未标注的 comb/seq 过程外使用分支".into(),
            }),
        };
        if let Some(d) = err {
            self.push_err(d);
        }
    }

    pub fn begin_else(&mut self, span: Span) {
        let err = match self.process.as_mut() {
            Some(ProcessState::Combinational {
                path_assigned,
                pending_branches,
                ..
            }) => {
                let then_set = path_assigned.pop().unwrap_or_default();
                if let Some(last) = pending_branches.last_mut() {
                    last.0 = then_set;
                    last.1 = true;
                    path_assigned.push(HashSet::new());
                    None
                } else {
                    Some(Diagnostic {
                        span,
                        code: "rhdl::E0102".into(),
                        en: "else without an open combinational then-branch".into(),
                        zh: "else 没有对应的组合 then 分支".into(),
                    })
                }
            }
            _ => Some(Diagnostic {
                span,
                code: "rhdl::E0102".into(),
                en: "else without an open combinational then-branch".into(),
                zh: "else 没有对应的组合 then 分支".into(),
            }),
        };
        if let Some(d) = err {
            self.push_err(d);
        }
    }

    pub fn end_if(&mut self, span: Span) {
        let mut latch_errs = Vec::new();
        let err = match self.process.as_mut() {
            Some(ProcessState::Combinational {
                path_assigned,
                pending_branches,
                ..
            }) => {
                let current = path_assigned.pop().unwrap_or_default();
                let Some((stored_then, had_else)) = pending_branches.pop() else {
                    latch_errs.push(Diagnostic {
                        span,
                        code: "rhdl::E0102".into(),
                        en: "end_if without begin_then".into(),
                        zh: "end_if 缺少 begin_then".into(),
                    });
                    for d in latch_errs {
                        self.push_err(d);
                    }
                    return;
                };

                let (then_set, else_set) = if had_else {
                    (stored_then, current)
                } else {
                    (current, HashSet::new())
                };

                let union: HashSet<_> = then_set.union(&else_set).cloned().collect();
                let inter: HashSet<_> = then_set.intersection(&else_set).cloned().collect();
                for name in union.difference(&inter) {
                    latch_errs.push(Diagnostic {
                        span,
                        code: "rhdl::E0110".into(),
                        en: format!(
                            "incomplete combinational assignment to '{name}' (would infer a latch)"
                        ),
                        zh: format!("组合赋值不完整：'{name}'（会推断成 latch）"),
                    });
                }

                if let Some(parent) = path_assigned.last_mut() {
                    for n in inter {
                        parent.insert(n);
                    }
                }
                None
            }
            _ => Some(Diagnostic {
                span,
                code: "rhdl::E0102".into(),
                en: "end_if outside combinational process".into(),
                zh: "end_if 不在组合过程中".into(),
            }),
        };
        for d in latch_errs {
            self.push_err(d);
        }
        if let Some(d) = err {
            self.push_err(d);
        }
    }

    /// Combinational assign `dst = lhs + rhs` (same-width required).
    pub fn assign_add(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        let dst = dst.into();
        let lhs = lhs.into();
        let rhs = rhs.into();
        if self.check_add(&lhs, &rhs, span).is_none() {
            return;
        }
        // Reuse assign_net permission checks by temporarily pushing Add.
        let kind = self.signals.get(&dst).copied();
        let process_kind = match &self.process {
            Some(ProcessState::Combinational { .. }) => Some(ProcessKind::Combinational),
            Some(ProcessState::Sequential { .. }) => Some(ProcessKind::Sequential),
            None => None,
        };
        match process_kind {
            Some(ProcessKind::Combinational) => {
                match kind {
                    Some(SignalKind::Wire | SignalKind::Output) => {}
                    Some(SignalKind::Reg) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0111".into(),
                            en: format!("combinational process must not drive Reg '{dst}'"),
                            zh: format!("组合过程不能驱动寄存器 '{dst}'"),
                        });
                        return;
                    }
                    Some(SignalKind::Input) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0112".into(),
                            en: format!("cannot assign to input port '{dst}'"),
                            zh: format!("不能给输入端口 '{dst}' 赋值"),
                        });
                        return;
                    }
                    None => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0113".into(),
                            en: format!("unknown signal '{dst}'"),
                            zh: format!("未知信号 '{dst}'"),
                        });
                        return;
                    }
                }
                if let Some(ProcessState::Combinational {
                    assigns,
                    path_assigned,
                    ..
                }) = self.process.as_mut()
                {
                    assigns.push(Assign {
                        target: AssignTarget::Net(dst.clone()),
                        expr: AssignExpr::Add(lhs, rhs),
                        span,
                    });
                    if let Some(path) = path_assigned.last_mut() {
                        path.insert(dst);
                    }
                }
            }
            Some(ProcessKind::Sequential) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0114".into(),
                    en: format!("sequential process must not drive combinational net '{dst}'"),
                    zh: format!("时序过程不能驱动组合网 '{dst}'"),
                });
            }
            None => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0103".into(),
                    en: "assignment outside a marked combinational/sequential process".into(),
                    zh: "在未标注的 comb/seq 过程外赋值".into(),
                });
            }
        }
    }

    /// Combinational assign `dst = lit`.
    pub fn assign_lit(&mut self, dst: impl Into<String>, lit: u64, span: Span) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Lit(lit), span);
    }

    /// Combinational assign `dst = (lhs == rhs)` (0/1).
    pub fn assign_eq(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Eq(lhs.into(), rhs.into()), span);
    }

    /// Combinational assign `dst = sel ? t : f` (`sel != 0` is true).
    pub fn assign_mux(
        &mut self,
        dst: impl Into<String>,
        sel: impl Into<String>,
        t: impl Into<String>,
        f: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(
            dst.into(),
            AssignExpr::Mux {
                sel: sel.into(),
                t: t.into(),
                f: f.into(),
            },
            span,
        );
    }

    /// Combinational `dst = lhs - rhs`.
    pub fn assign_sub(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Sub(lhs.into(), rhs.into()), span);
    }

    /// Combinational `dst = lhs & rhs`.
    pub fn assign_and(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::And(lhs.into(), rhs.into()), span);
    }

    /// Combinational `dst = lhs | rhs`.
    pub fn assign_or(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Or(lhs.into(), rhs.into()), span);
    }

    /// Combinational `dst = lhs ^ rhs`.
    pub fn assign_xor(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Xor(lhs.into(), rhs.into()), span);
    }

    /// Combinational `dst = lhs << (rhs & 63)`.
    pub fn assign_shl(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Shl(lhs.into(), rhs.into()), span);
    }

    /// Combinational `dst = lhs >> (rhs & 63)` (logical).
    pub fn assign_shr(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Shr(lhs.into(), rhs.into()), span);
    }

    fn push_comb_net_expr(&mut self, dst: String, expr: AssignExpr, span: Span) {
        let kind = self.signals.get(&dst).copied();
        let process_kind = match &self.process {
            Some(ProcessState::Combinational { .. }) => Some(ProcessKind::Combinational),
            Some(ProcessState::Sequential { .. }) => Some(ProcessKind::Sequential),
            None => None,
        };
        match process_kind {
            Some(ProcessKind::Combinational) => {
                match kind {
                    Some(SignalKind::Wire | SignalKind::Output) => {}
                    Some(SignalKind::Reg) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0111".into(),
                            en: format!("combinational process must not drive Reg '{dst}'"),
                            zh: format!("组合过程不能驱动寄存器 '{dst}'"),
                        });
                        return;
                    }
                    Some(SignalKind::Input) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0112".into(),
                            en: format!("cannot assign to input port '{dst}'"),
                            zh: format!("不能给输入端口 '{dst}' 赋值"),
                        });
                        return;
                    }
                    None => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0113".into(),
                            en: format!("unknown signal '{dst}'"),
                            zh: format!("未知信号 '{dst}'"),
                        });
                        return;
                    }
                }
                if let Some(ProcessState::Combinational {
                    assigns,
                    path_assigned,
                    ..
                }) = self.process.as_mut()
                {
                    assigns.push(Assign {
                        target: AssignTarget::Net(dst.clone()),
                        expr,
                        span,
                    });
                    if let Some(path) = path_assigned.last_mut() {
                        path.insert(dst);
                    }
                }
            }
            Some(ProcessKind::Sequential) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0114".into(),
                    en: format!("sequential process must not drive combinational net '{dst}'"),
                    zh: format!("时序过程不能驱动组合网 '{dst}'"),
                });
            }
            None => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0103".into(),
                    en: "assignment outside a marked combinational/sequential process".into(),
                    zh: "在未标注的 comb/seq 过程外赋值".into(),
                });
            }
        }
    }

    /// Assign a combinational net / output / wire from `from`.
    pub fn assign_net(&mut self, name: impl Into<String>, from: impl Into<String>, span: Span) {
        let name = name.into();
        let from = from.into();
        let src_dom = self.domains.get(&from).copied().unwrap_or(0);
        let dst_dom = self.domains.get(&name).copied().unwrap_or(0);
        if src_dom != dst_dom
            && !self.cdc_bridges.contains(&name)
            && !self.cdc_bridges.contains(&from)
        {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0220".into(),
                en: format!(
                    "illegal clock-domain crossing '{from}'(D{src_dom}) → '{name}'(D{dst_dom}); use DoubleFlop/SyncFIFO"
                ),
                zh: format!(
                    "非法跨时钟域：'{from}'(D{src_dom}) → '{name}'(D{dst_dom})；请用 DoubleFlop/SyncFIFO"
                ),
            });
            return;
        }
        // Width gate (E0131) before emit — FR51 / FR22 same-width connects.
        if self.check_connect(&name, &from, span).is_none() {
            return;
        }
        let kind = self.signals.get(&name).copied();
        let process_kind = match &self.process {
            Some(ProcessState::Combinational { .. }) => Some(ProcessKind::Combinational),
            Some(ProcessState::Sequential { .. }) => Some(ProcessKind::Sequential),
            None => None,
        };

        match process_kind {
            Some(ProcessKind::Combinational) => {
                match kind {
                    Some(SignalKind::Reg) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0111".into(),
                            en: format!(
                                "combinational process must not drive Reg '{name}' (use Reg.d in sequential)"
                            ),
                            zh: format!(
                                "组合过程不能驱动寄存器 '{name}'（请在时序过程写 Reg.d）"
                            ),
                        });
                        return;
                    }
                    Some(SignalKind::Input) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0112".into(),
                            en: format!("cannot assign to input port '{name}'"),
                            zh: format!("不能给输入端口 '{name}' 赋值"),
                        });
                        return;
                    }
                    Some(SignalKind::Wire | SignalKind::Output) => {}
                    None => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0113".into(),
                            en: format!("unknown signal '{name}'"),
                            zh: format!("未知信号 '{name}'"),
                        });
                        return;
                    }
                }
                if let Some(ProcessState::Combinational {
                    assigns,
                    path_assigned,
                    ..
                }) = self.process.as_mut()
                {
                    assigns.push(Assign {
                        target: AssignTarget::Net(name.clone()),
                        expr: AssignExpr::Ref(from.clone()),
                        span,
                    });
                    if let Some(path) = path_assigned.last_mut() {
                        path.insert(name);
                    }
                }
            }
            Some(ProcessKind::Sequential) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0114".into(),
                    en: format!("sequential process must not drive combinational net '{name}'"),
                    zh: format!("时序过程不能驱动组合网 '{name}'"),
                });
            }
            None => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0103".into(),
                    en: "assignment outside a marked combinational/sequential process".into(),
                    zh: "在未标注的 comb/seq 过程外赋值".into(),
                });
            }
        }
    }

    /// Assign `Reg.d` next-state as `name + 1` (wrapping).
    pub fn assign_reg_d_inc(&mut self, name: impl Into<String>, span: Span) {
        self.assign_reg_d_expr(name, None, span);
    }

    /// Assign `Reg.d` from another signal.
    pub fn assign_reg_d_from(
        &mut self,
        name: impl Into<String>,
        from: impl Into<String>,
        span: Span,
    ) {
        self.assign_reg_d_expr(name, Some(from.into()), span);
    }

    /// Sequential SyncReadMem / Mem write: `mem[addr] <= data` (always enabled).
    pub fn assign_mem_write(
        &mut self,
        mem: impl Into<String>,
        addr: impl Into<String>,
        data: impl Into<String>,
        span: Span,
    ) {
        self.assign_mem_write_inner(mem.into(), addr.into(), data.into(), None, span);
    }

    /// Sequential mem write gated by `we` (`we != 0`).
    pub fn assign_mem_write_en(
        &mut self,
        mem: impl Into<String>,
        addr: impl Into<String>,
        data: impl Into<String>,
        we: impl Into<String>,
        span: Span,
    ) {
        self.assign_mem_write_inner(mem.into(), addr.into(), data.into(), Some(we.into()), span);
    }

    fn assign_mem_write_inner(
        &mut self,
        mem: String,
        addr: String,
        data: String,
        we: Option<String>,
        span: Span,
    ) {
        match &self.process {
            Some(ProcessState::Sequential { .. }) => {
                if let Some(ProcessState::Sequential { assigns, .. }) = self.process.as_mut() {
                    assigns.push(Assign {
                        target: AssignTarget::MemWrite { mem, addr, we },
                        expr: AssignExpr::Ref(data),
                        span,
                    });
                }
            }
            _ => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0211".into(),
                    en: "mem write must be inside a sequential process".into(),
                    zh: "mem 写必须在 sequential 过程内".into(),
                });
            }
        }
    }

    /// Sequential SyncReadMem read into a register (latency 1 on tick).
    pub fn assign_reg_d_mem_read(
        &mut self,
        reg: impl Into<String>,
        mem: impl Into<String>,
        addr: impl Into<String>,
        span: Span,
    ) {
        let reg = reg.into();
        let mem = mem.into();
        let addr = addr.into();
        match &self.process {
            Some(ProcessState::Sequential { .. }) => {
                if let Some(ProcessState::Sequential { assigns, .. }) = self.process.as_mut() {
                    assigns.push(Assign {
                        target: AssignTarget::RegD(reg),
                        expr: AssignExpr::MemRead { mem, addr },
                        span,
                    });
                }
            }
            _ => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0212".into(),
                    en: "sync mem read into Reg must be inside a sequential process".into(),
                    zh: "SyncReadMem 读入寄存器必须在 sequential 过程内".into(),
                });
            }
        }
    }

    fn assign_reg_d_expr(&mut self, name: impl Into<String>, from: Option<String>, span: Span) {
        let name = name.into();
        if let Some(ref src) = from
            && self.check_connect(&name, src, span).is_none()
        {
            return;
        }
        let kind = self.signals.get(&name).copied();
        let process_kind = match &self.process {
            Some(ProcessState::Combinational { .. }) => Some(ProcessKind::Combinational),
            Some(ProcessState::Sequential { .. }) => Some(ProcessKind::Sequential),
            None => None,
        };

        match process_kind {
            Some(ProcessKind::Sequential) => match kind {
                Some(SignalKind::Reg) => {
                    if let Some(ProcessState::Sequential { assigns, .. }) = self.process.as_mut() {
                        let expr = match from {
                            Some(src) => AssignExpr::Ref(src),
                            None => AssignExpr::Inc(name.clone()),
                        };
                        assigns.push(Assign {
                            target: AssignTarget::RegD(name),
                            expr,
                            span,
                        });
                    }
                }
                Some(_) => {
                    self.push_err(Diagnostic {
                        span,
                        code: "rhdl::E0115".into(),
                        en: format!("'{name}' is not a Reg; Reg.d requires a register"),
                        zh: format!("'{name}' 不是寄存器，不能写 Reg.d"),
                    });
                }
                None => {
                    self.push_err(Diagnostic {
                        span,
                        code: "rhdl::E0113".into(),
                        en: format!("unknown signal '{name}'"),
                        zh: format!("未知信号 '{name}'"),
                    });
                }
            },
            Some(ProcessKind::Combinational) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0116".into(),
                    en: format!("combinational process must not write Reg.d for '{name}'"),
                    zh: format!("组合过程不能写 '{name}' 的 Reg.d"),
                });
            }
            None => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0103".into(),
                    en: "assignment outside a marked combinational/sequential process".into(),
                    zh: "在未标注的 comb/seq 过程外赋值".into(),
                });
            }
        }
    }

    pub fn end_process(&mut self) {
        let Some(state) = self.process.take() else {
            return;
        };
        match state {
            ProcessState::Combinational {
                assigns,
                pending_branches,
                span,
                ..
            } => {
                if !pending_branches.is_empty() {
                    self.push_err(Diagnostic {
                        span,
                        code: "rhdl::E0102".into(),
                        en: "unclosed if/else in combinational process".into(),
                        zh: "组合过程中有未关闭的 if/else".into(),
                    });
                }
                if let Some(m) = self.current.as_mut() {
                    m.body.push(Stmt::Process(Process {
                        kind: ProcessKind::Combinational,
                        assigns,
                        span,
                    }));
                }
            }
            ProcessState::Sequential { assigns, span } => {
                if let Some(m) = self.current.as_mut() {
                    m.body.push(Stmt::Process(Process {
                        kind: ProcessKind::Sequential,
                        assigns,
                        span,
                    }));
                }
            }
        }
    }

    pub fn end_module(&mut self) {
        if self.process.is_some() {
            self.end_process();
        }
        if let Some(m) = self.current.take() {
            self.hir.add_module(m);
        }
        self.signals.clear();
        self.widths.clear();
        self.clock_port = None;
        self.reset_port = None;
    }

    pub fn finish(self) -> Result<FrozenHir, Diagnostics> {
        if !self.errors.is_empty() {
            return Err(self.errors);
        }
        bitloom_hir::seal_from_builder(self.hir)
    }

    /// Record a synthesizable-path violation (heap, threads, f64, capturing closure, …)
    /// as a structured diagnostic (`rhdl::E0141` / FR16).
    pub fn reject_unsynthesizable(&mut self, construct: &str, span: Span) {
        self.push_err(Diagnostic {
            span,
            code: "rhdl::E0141".into(),
            en: format!(
                "unsynthesizable construct '{construct}' is not allowed on the cycle-accurate path"
            ),
            zh: format!("周期精确路径不允许不可综合构造 '{construct}'"),
        });
    }

    /// Reject capturing a hardware Wire/Reg/signal ref into an elaborate-time
    /// generator or factory closure (FR73 / NFR35 / AD-18).
    ///
    /// Stable code: **`rhdl::E0142`**. Distinct from FR16 cycle-accurate
    /// [`Self::reject_unsynthesizable`] (`rhdl::E0141`).
    pub fn reject_hw_capture(&mut self, capture: &HwCaptureRef, span: Span) {
        let kind = capture.kind_label();
        self.push_err(Diagnostic {
            span,
            code: "rhdl::E0142".into(),
            en: format!(
                "illegal capture of hardware {kind} '{}' into elaborate-time generator closure; \
                 only non-capturing Fn that dissolves before freeze is allowed (FR73 / NFR35 / AD-18)",
                capture.name
            ),
            zh: format!(
                "不允许将硬件 {kind} '{}' 捕获进 elaborate-time 生成器闭包；\
                 仅允许冻前消解的非捕获 Fn（FR73 / NFR35 / AD-18）",
                capture.name
            ),
        });
    }

    /// Assert that no hardware Wire/Reg/signal refs were captured into a
    /// generator / factory context. Empty slice is a no-op (legal non-capturing path).
    ///
    /// Call when a documented illegal capture is present (ATDD / macros / typed
    /// handles). Any token → `rhdl::E0142` at [`Self::finish`].
    pub fn assert_no_hw_capture(&mut self, captures: &[HwCaptureRef], span: Span) {
        for c in captures {
            self.reject_hw_capture(c, span);
        }
    }

    /// Reject a documented SynthesizableClosure constraint breach (FR74 / Cap-R-60).
    ///
    /// Stable codes: **`rhdl::E0143`** (heap), **`rhdl::E0144`** (runtime capture
    /// state), **`rhdl::E0145`** (impure). Does **not** emit closure IR into HIR
    /// (NFR36) — diagnostics only. Distinct from E0141 / E0142.
    pub fn reject_unsynthesizable_closure(
        &mut self,
        violation: &SynthesizableClosureViolation,
        span: Span,
    ) {
        for d in diagnose_synthesizable_closure_violations(std::slice::from_ref(violation), span).0
        {
            self.push_err(d);
        }
    }

    /// Cap-R-60 check hook: record all documented SynthesizableClosure violations.
    ///
    /// Empty `violations` is a no-op (legal empty/simple closure — paves 28.2).
    /// Reachable from design crates via prelude and from future
    /// `cargo bitloom check` (or equivalent) wrappers.
    pub fn check_synthesizable_closure(
        &mut self,
        violations: &[SynthesizableClosureViolation],
        span: Span,
    ) {
        for v in violations {
            self.reject_unsynthesizable_closure(v, span);
        }
    }

    /// Cap-R-60 helper: run [`SynthesizableClosure::synthesizable_closure_violations`]
    /// and record any tokens (empty = pass).
    pub fn check_synthesizable_closure_marker<C: SynthesizableClosure>(
        &mut self,
        closure: &C,
        span: Span,
    ) {
        let vs = closure.synthesizable_closure_violations();
        self.check_synthesizable_closure(&vs, span);
    }

    /// Hierarchical instance (Story 2.2); not flattened at elaborate.
    pub fn add_instance(
        &mut self,
        name: impl Into<String>,
        module: impl Into<String>,
        connects: Vec<(String, String)>,
        params: Vec<(String, u32)>,
        span: Span,
    ) {
        use bitloom_hir::{Instance, PortConnect};
        let connects = connects
            .into_iter()
            .map(|(child_port, parent_net)| PortConnect {
                child_port,
                parent_net,
                span,
                dangling: false,
            })
            .collect();
        if let Some(m) = self.current.as_mut() {
            m.body.push(Stmt::Instance(Instance {
                name: name.into(),
                module: module.into(),
                connects,
                params,
                span,
            }));
        }
    }

    /// Elaborate-time module factory (FR73 / Cap-R-53).
    ///
    /// Invokes `f(i, session)` for each `i` in `0..count`. The factory should
    /// call [`Self::add_instance`] (and optional wire decls) so each iteration
    /// records a child instance plus type-safe port connects. Only ordinary
    /// `Stmt::Instance` / `PortConnect` remain after the loop — the closure
    /// does **not** enter FrozenHir (NFR36). Width/dir checks still run at
    /// `finish` (FR8 / existing instance validation). Capturing Wire/Reg into
    /// the factory is illegal — use [`Self::assert_no_hw_capture`] (`rhdl::E0142`).
    ///
    /// ```ignore
    /// session.generate_instances(4, |i, s| {
    ///     s.add_instance(
    ///         format!("u{i}"),
    ///         "Lane",
    ///         vec![
    ///             ("clk".into(), "clk".into()),
    ///             ("rst".into(), "rst".into()),
    ///             ("x".into(), format!("x{i}")),
    ///             ("y".into(), format!("y{i}")),
    ///         ],
    ///         vec![],
    ///         Span::default(),
    ///     );
    /// });
    /// ```
    pub fn generate_instances<F>(&mut self, count: usize, mut f: F)
    where
        F: FnMut(usize, &mut Self),
    {
        for i in 0..count {
            f(i, self);
        }
    }

    /// Pure-return factory variant (FR73 / Cap-R-53): `f(i)` returns an
    /// [`GeneratedInstance`] that is immediately recorded as HIR via
    /// [`Self::add_instance`]. Prefer when the factory needs no extra session
    /// side effects (wires, dangling marks).
    pub fn generate_instances_from<F>(&mut self, count: usize, f: F, span: Span)
    where
        F: Fn(usize) -> GeneratedInstance,
    {
        for i in 0..count {
            let g = f(i);
            self.add_instance(g.name, g.module, g.connects, g.params, span);
        }
    }

    pub fn add_dangling_input(
        &mut self,
        instance: &str,
        child_port: impl Into<String>,
        span: Span,
    ) {
        use bitloom_hir::PortConnect;
        if let Some(m) = self.current.as_mut() {
            for stmt in &mut m.body {
                if let Stmt::Instance(inst) = stmt {
                    if inst.name == instance {
                        inst.connects.push(PortConnect {
                            child_port: child_port.into(),
                            parent_net: String::new(),
                            span,
                            dangling: true,
                        });
                        return;
                    }
                }
            }
        }
        self.push_err(Diagnostic {
            span,
            code: "rhdl::E0201".into(),
            en: format!("unknown instance '{instance}' for dangling mark"),
            zh: format!("悬空标记找不到实例 '{instance}'"),
        });
    }
}

/// Trait implemented by `#[rhdl::top]` / design modules (AD-19 partial for 1.1).
pub trait Elaboratable {
    fn elaborate() -> Result<FrozenHir, Diagnostics>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_ports(s: &mut ElaborateSession) {
        s.begin_module("M", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
    }

    #[test]
    fn complete_comb_assign_ok() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn incomplete_branch_is_latch_error() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_combinational(Span::default());
        s.begin_then(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.begin_else(Span::default());
        // else does not assign data_out
        s.end_if(Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0110"),
            "expected latch diagnostic, got {err}"
        );
    }

    #[test]
    fn both_branches_assign_ok() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_combinational(Span::default());
        s.begin_then(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.begin_else(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_if(Span::default());
        s.end_process();
        s.end_module();
        let r = s.finish();
        assert!(r.is_ok(), "{:?}", r.err());
    }

    #[test]
    fn comb_cannot_write_reg_d() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_reg_d_inc("count", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0116"));
    }

    #[test]
    fn seq_cannot_drive_comb_net() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_sequential(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0114"));
    }

    #[test]
    fn assign_outside_process_rejected() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.assign_net("data_out", "data_in", Span::default());
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0103"));
    }

    #[test]
    fn seq_reg_d_ok() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("count", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn missing_clock_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("M", Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0120"));
    }

    #[test]
    fn missing_reset_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("M", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0121"));
    }

    #[test]
    fn mismatched_add_width_rejected() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_wire("a", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("b", GroundType::UInt { width: 16 }, Span::default());
        assert!(s.check_add("a", "b", Span::default()).is_none());
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0130"));
    }

    #[test]
    fn mismatched_assign_net_width_rejected() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.add_output("narrow", GroundType::UInt { width: 4 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("narrow", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0131"),
            "expected E0131, got {err}"
        );
    }

    #[test]
    fn mismatched_assign_reg_d_width_rejected() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("q_narrow", GroundType::UInt { width: 4 }, Span::default());
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("q_narrow", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0131"),
            "expected E0131 on Reg.d path, got {err}"
        );
    }

    #[test]
    fn pad_then_add_ok() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_wire("a", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("b", GroundType::UInt { width: 16 }, Span::default());
        assert!(s.pad_to("a", 16, "a_pad", Span::default()));
        assert_eq!(s.check_add("a_pad", "b", Span::default()), Some(16));
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn multi_drive_rejected() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0140"));
    }

    #[test]
    fn parameterized_widths_w8_and_w16() {
        fn elaborate_w(w: u32) -> bitloom_hir::FrozenHir {
            let mut s = ElaborateSession::new("t");
            s.begin_module(format!("Add{w}"), Span::default());
            s.add_input("clk", GroundType::Clock, Span::default());
            s.add_input("rst", GroundType::Reset, Span::default());
            s.add_input("a", GroundType::UInt { width: w }, Span::default());
            s.add_input("b", GroundType::UInt { width: w }, Span::default());
            s.add_output("y", GroundType::UInt { width: w }, Span::default());
            s.begin_combinational(Span::default());
            s.assign_net("y", "a", Span::default());
            s.end_process();
            s.end_module();
            s.finish().unwrap()
        }
        let h8 = elaborate_w(8);
        let h16 = elaborate_w(16);
        assert!(matches!(
            h8.circuit().modules[0].ports[2].ty,
            GroundType::UInt { width: 8 }
        ));
        assert!(matches!(
            h16.circuit().modules[0].ports[2].ty,
            GroundType::UInt { width: 16 }
        ));
    }

    #[test]
    fn hierarchy_instance_preserved() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Child", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "x", Span::default());
        s.end_process();
        s.end_module();

        s.begin_module("Parent", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.add_instance(
            "u0",
            "Child",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("x".into(), "x".into()),
                ("y".into(), "y".into()),
            ],
            vec![("W".into(), 8)],
            Span::default(),
        );
        s.end_module();
        let frozen = s.finish().unwrap();
        assert_eq!(frozen.circuit().modules.len(), 2);
        assert!(frozen.circuit().modules[1].body.iter().any(|st| matches!(
            st,
            bitloom_hir::Stmt::Instance(i) if i.name == "u0" && i.module == "Child"
        )));
    }

    #[test]
    fn generate_instances_factory_batches_children() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Lane", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "x", Span::default());
        s.end_process();
        s.end_module();

        s.begin_module("Parent", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        for i in 0..3 {
            s.add_input(
                format!("x{i}"),
                GroundType::UInt { width: 8 },
                Span::default(),
            );
            s.add_output(
                format!("y{i}"),
                GroundType::UInt { width: 8 },
                Span::default(),
            );
        }
        s.generate_instances(3, |i, sess| {
            sess.add_instance(
                format!("u{i}"),
                "Lane",
                vec![
                    ("clk".into(), "clk".into()),
                    ("rst".into(), "rst".into()),
                    ("x".into(), format!("x{i}")),
                    ("y".into(), format!("y{i}")),
                ],
                vec![],
                Span::default(),
            );
        });
        s.end_module();
        let frozen = s.finish().unwrap();
        let parent = frozen
            .circuit()
            .modules
            .iter()
            .find(|m| m.name == "Parent")
            .unwrap();
        let instances: Vec<_> = parent
            .body
            .iter()
            .filter_map(|st| match st {
                bitloom_hir::Stmt::Instance(i) => Some(i.name.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(instances, ["u0", "u1", "u2"]);
    }

    #[test]
    fn generate_instances_from_returns_plain_specs() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Lane", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.end_module();
        s.begin_module("Parent", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x0", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y0", GroundType::UInt { width: 8 }, Span::default());
        s.generate_instances_from(
            1,
            |_| {
                GeneratedInstance::new(
                    "u0",
                    "Lane",
                    vec![
                        ("clk".into(), "clk".into()),
                        ("rst".into(), "rst".into()),
                        ("x".into(), "x0".into()),
                        ("y".into(), "y0".into()),
                    ],
                    vec![],
                )
            },
            Span::default(),
        );
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn undriven_child_input_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Child", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.end_module();
        s.begin_module("Parent", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.add_instance(
            "u0",
            "Child",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("y".into(), "y".into()),
            ],
            vec![],
            Span::default(),
        );
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0202"));
    }

    #[test]
    fn sync_read_mem_declares_and_emits() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("MemTop", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_sync_read_mem("ram", 16, 8, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "ram", Span::default());
        s.end_process();
        s.end_module();
        let frozen = s.finish().unwrap();
        assert!(frozen.circuit().modules[0].body.iter().any(|st| matches!(
            st,
            bitloom_hir::Stmt::MemDecl {
                sync_read: true,
                ..
            }
        )));
    }

    #[test]
    fn mem_with_init_fn_stores_plain_words() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Lut", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_mem_with_init_fn("rom", 4, 8, |i| ((i * i) & 0xff) as u64, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "rom", Span::default());
        s.end_process();
        s.end_module();
        let frozen = s.finish().unwrap();
        let init = frozen.circuit().modules[0]
            .body
            .iter()
            .find_map(|st| match st {
                bitloom_hir::Stmt::MemDecl {
                    name,
                    init: Some(words),
                    ..
                } if name == "rom" => Some(words.clone()),
                _ => None,
            })
            .expect("rom init present");
        assert_eq!(init, vec![0, 1, 4, 9]);
    }

    #[test]
    fn mem_init_len_mismatch_fails() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Bad", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.declare_mem_with_init("rom", 4, 8, vec![1, 2], Span::default());
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0212"));
    }

    #[test]
    fn async_reset_and_enable_flags() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("M", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg_ex(
            "q",
            GroundType::UInt { width: 8 },
            true,
            true,
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("y", "q", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("q", Span::default());
        s.end_process();
        s.end_module();
        let frozen = s.finish().unwrap();
        assert!(frozen.circuit().modules[0].body.iter().any(|st| matches!(
            st,
            bitloom_hir::Stmt::RegDecl {
                async_reset: true,
                has_enable: true,
                ..
            }
        )));
    }

    #[test]
    fn illegal_domain_crossing_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Cdc", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.bind_domain("a", 0);
        s.bind_domain("y", 1);
        s.begin_combinational(Span::default());
        s.assign_net("y", "a", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0220"), "{err}");
    }

    #[test]
    fn cdc_bridge_allows_crossing() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("CdcOk", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.bind_domain("a", 0);
        s.bind_domain("y", 1);
        s.mark_cdc_bridge("y");
        s.begin_combinational(Span::default());
        s.assign_net("y", "a", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn unknown_parent_net_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Child", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.end_module();
        s.begin_module("Parent", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.add_instance(
            "u0",
            "Child",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("x".into(), "no_such_net".into()),
                ("y".into(), "y".into()),
            ],
            vec![],
            Span::default(),
        );
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0204"), "{err}");
    }

    #[test]
    fn hw_capture_wire_rejected_e0142() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_wire("w", GroundType::UInt { width: 8 }, Span::default());
        // Documented illegal capture of Wire into generator context.
        s.assert_no_hw_capture(&[HwCaptureRef::wire("w")], Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0142"),
            "expected E0142, got {err}"
        );
        assert!(
            err.0
                .iter()
                .any(|d| d.en.contains("Wire") && d.en.contains("w")),
            "diagnostic should name Wire 'w': {err}"
        );
    }

    #[test]
    fn hw_capture_reg_rejected_e0142() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("r", GroundType::UInt { width: 8 }, Span::default());
        s.reject_hw_capture(&HwCaptureRef::reg("r"), Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0
                .iter()
                .any(|d| d.code == "rhdl::E0142" && d.en.contains("Reg")),
            "expected E0142 Reg, got {err}"
        );
    }

    #[test]
    fn assert_no_hw_capture_empty_ok() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.assert_no_hw_capture(&[], Span::default());
        s.declare_mem_with_init_fn("rom", 2, 8, |i| i as u64, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn fr16_capturing_closure_still_e0141() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.reject_unsynthesizable("capturing closure", Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0141"),
            "FR16 capturing closure must stay E0141, got {err}"
        );
    }

    #[test]
    fn synthesizable_closure_heap_e0143() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.reject_unsynthesizable_closure(
            &SynthesizableClosureViolation::heap("Box<u8> in body"),
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0143"),
            "expected E0143, got {err}"
        );
    }

    #[test]
    fn synthesizable_closure_capture_state_e0144() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.check_synthesizable_closure(
            &[SynthesizableClosureViolation::runtime_capture_state(
                "captures local threshold",
            )],
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0144"),
            "expected E0144, got {err}"
        );
    }

    #[test]
    fn synthesizable_closure_impure_e0145() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.reject_unsynthesizable_closure(
            &SynthesizableClosureViolation::impure("file I/O"),
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0145"),
            "expected E0145, got {err}"
        );
    }

    #[test]
    fn legal_empty_and_simple_synthesizable_closure_pass() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.check_synthesizable_closure(&[], Span::default());
        s.check_synthesizable_closure_marker(&LegalEmptyClosure, Span::default());
        s.check_synthesizable_closure_marker(&LegalSimpleClosure, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok(), "legal empty/simple must pass");
    }

    #[test]
    fn diagnose_free_fn_cap_r60() {
        let diags = diagnose_synthesizable_closure_violations(
            &[SynthesizableClosureViolation::heap("String")],
            Span::default(),
        );
        assert!(diags.0.iter().any(|d| d.code == "rhdl::E0143"));
    }
}
