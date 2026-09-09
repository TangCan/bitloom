//! Elaborate-time closure / capture / inline surfaces (FR73–FR75 / Cap-R-*).
//!
//! Extracted from `lib.rs` as a size-hygiene first step (epic-27/28 builder split
//! assessment). Behavior unchanged — re-exported at crate root.

use bitloom_hir::{Diagnostic, Diagnostics, Span};

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

    pub(crate) fn kind_label(&self) -> &'static str {
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
/// Comb inline (Story 28.2 / Cap-R-55): call
/// [`ElaborateSession::inline_comb_fn`] after Cap-R-60 check.
/// Seq inline (Story 28.3 / Cap-R-56): [`ElaborateSession::inline_seq_fn`]
/// with Cap-R-70 ownership checks. Automatic rustc capture analysis is out of
/// scope — macros / ATDD / typed surfaces pass violation tokens to
/// [`ElaborateSession::check_synthesizable_closure`] /
/// [`ElaborateSession::check_seq_ownership`].
///
/// Empty / simple stand-ins ([`LegalEmptyClosure`], [`LegalSimpleClosure`])
/// implement this marker with no violations.
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

/// Positive stand-in: simple pure unary transform (FR74 / FR75 comb inline).
#[derive(Debug, Default, Clone, Copy)]
pub struct LegalSimpleClosure;

impl SynthesizableClosure for LegalSimpleClosure {}

/// Elaborate-time description of a combinational RHS (FR75 / Cap-R-55).
///
/// Produced by [`ElaborateSession::inline_comb_fn`] closures and immediately
/// lowered to ordinary [`AssignExpr`] via existing `assign_*` APIs. Never
/// stored as a Rust `Fn` in FrozenHir (NFR36). Also reusable as the RHS of
/// [`SeqInline::Comb`] for sequential Reg.d inline (Cap-R-56).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombInline {
    /// Copy from a named net / port / reg.
    Ref(String),
    /// Integer literal.
    Lit(u64),
    /// Same-width add.
    Add(String, String),
    /// Same-width subtract.
    Sub(String, String),
    /// Bitwise AND.
    And(String, String),
    /// Bitwise OR.
    Or(String, String),
    /// Bitwise XOR.
    Xor(String, String),
    /// Equality → 0/1 Bool.
    Eq(String, String),
    /// 2:1 mux (`sel != 0 ? t : f`).
    Mux { sel: String, t: String, f: String },
}

/// Elaborate-time description of a sequential `Reg.d` next-state (FR75 / Cap-R-56).
///
/// Produced by [`ElaborateSession::inline_seq_fn`] and immediately lowered to
/// ordinary sequential [`AssignExpr`] targeting [`AssignTarget::RegD`]. Never
/// stored as a Rust `Fn` in FrozenHir (NFR36).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeqInline {
    /// Wrapping `dst + 1` (same as [`ElaborateSession::assign_reg_d_inc`]).
    Inc,
    /// Comb-shaped RHS lowered onto `Reg.d` (reuses [`CombInline`]).
    Comb(CombInline),
}

impl From<CombInline> for SeqInline {
    fn from(c: CombInline) -> Self {
        Self::Comb(c)
    }
}

/// Cap-R-70 ownership breach inside sequential synthesizable-closure inline.
///
/// Distinct from Cap-R-60 [`SynthesizableClosureViolation`] (E0143–E0145) and
/// freeze multi-drive [`rhdl::E0140`] (AD-4, across processes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeqOwnershipViolationKind {
    /// Illegal extra mutable borrow / second `Reg.d` write in the same
    /// sequential process (or documented equivalent) → `rhdl::E0146`.
    IllegalMutableBorrow,
}

impl SeqOwnershipViolationKind {
    pub fn code(self) -> &'static str {
        match self {
            Self::IllegalMutableBorrow => "rhdl::E0146",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::IllegalMutableBorrow => "illegal mutable signal borrow",
        }
    }
}

/// Documented Cap-R-70 ownership violation token (Story 28.3).
///
/// Pass to [`ElaborateSession::check_seq_ownership`] /
/// [`ElaborateSession::inline_seq_fn`]. Empty list = no tokenized breach;
/// session still auto-detects a second `Reg.d` write on the inline destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeqOwnershipViolation {
    pub kind: SeqOwnershipViolationKind,
    pub detail: String,
}

impl SeqOwnershipViolation {
    pub fn illegal_mutable_borrow(detail: impl Into<String>) -> Self {
        Self {
            kind: SeqOwnershipViolationKind::IllegalMutableBorrow,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> &'static str {
        self.kind.code()
    }
}

/// Cap-R-70 check surface: diagnose seq-ownership violations without a session.
pub fn diagnose_seq_ownership_violations(
    violations: &[SeqOwnershipViolation],
    span: Span,
) -> Diagnostics {
    let mut diags = Diagnostics::default();
    for v in violations {
        diags.push(Diagnostic {
            span,
            code: v.code().into(),
            en: format!(
                "sequential synthesizable-closure ownership violation: {} ({}); \
                 Cap-R-70 forbids illegal extra mutable signal borrows / multi-drive \
                 patterns inside seq inline (FR75)",
                v.kind.label(),
                v.detail
            ),
            zh: format!(
                "时序可综合闭包所有权违规：{}（{}）；Cap-R-70 禁止闭包体内额外非法可变信号借用/多驱动（FR75）",
                v.kind.label(),
                v.detail
            ),
        });
    }
    diags
}

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
