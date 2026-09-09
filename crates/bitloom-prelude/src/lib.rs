//! Design-facing surface. Designs depend only on this crate.

pub use bitloom_builder::{
    CombInline, Elaboratable, ElaborateSession, GeneratedInstance, HwCaptureKind, HwCaptureRef,
    LegalEmptyClosure, LegalSimpleClosure, SeqInline, SeqOwnershipViolation,
    SeqOwnershipViolationKind, SynthesizableClosure, SynthesizableClosureViolation,
    SynthesizableClosureViolationKind, diagnose_seq_ownership_violations,
    diagnose_synthesizable_closure_violations,
};
pub use bitloom_hir::{Diagnostics, FrozenHir, GroundType, PortDirection, PortValues, Span};

/// First-class IP stubs (FR37 / FR48): FIFO, UART, … via this prelude only.
pub mod ip;

/// Re-export attributes via this crate's `rhdl` facade.
pub mod rhdl {
    pub use bitloom_macro::{
        abstraction, both, bridge, combinational, functional_model, hls, module, process,
        sequential, top,
    };
}

/// Host-only simulation view kind (FR29). Never a FrozenHir node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewKind {
    FunctionalModel,
    Bridge,
    Abstraction,
    Both,
}

/// Marker implemented by `#[rhdl::bridge]` / `#[rhdl::abstraction]` / `#[rhdl::both]` /
/// `#[rhdl::functional_model]`. These types do not enter HIR.
pub trait HostView {
    const KIND: ViewKind;
}

/// Host/bridge handshake template: start action → tick until not busy (FR78 / Cap-R-65).
///
/// **View boundary (Cap-R-66/67 / NFR36):** `start_fn` is a **free host closure** — it may
/// set signal-level fields on `Self`. The cycle-accurate path continues only via
/// [`Self::tick`] / ordinary pins/`PortValues`. The `Fn` **never** enters FrozenHir or
/// `Sim::tick` as a closure object. Not SynthesizableClosure (≠ FR74/75) and not an
/// elaborate-time generator (≠ FR73). SystemC TLM is not a product path (AD-5).
///
/// See `docs/fr78-bridge-adapter-closures.md`.
pub trait StartWaitComplete {
    /// Whether the handshake is still in progress (e.g. `tx_busy`).
    fn is_busy(&self) -> bool;

    /// Advance one cycle on the cycle-accurate side (ordinary signal update only).
    fn tick(&mut self);

    /// Run `start_fn` once, commit with one [`tick`](Self::tick), then tick while
    /// [`is_busy`](Self::is_busy) (documented equivalent of requirements
    /// `start → while busy { tick }` — the mandatory first tick lets registered
    /// busy flags assert before the wait loop).
    fn start_wait_complete<F>(&mut self, start_fn: F)
    where
        F: FnOnce(&mut Self),
    {
        start_fn(self);
        self.tick();
        while self.is_busy() {
            self.tick();
        }
    }
}

/// Free-function form of [`StartWaitComplete::start_wait_complete`] (same FR78 contract).
///
/// Prefer the trait when the adapter owns `tick` / `is_busy`; use this when wiring an
/// external sim/`PortValues` driver without implementing the trait.
pub fn start_wait_complete<H, F>(
    host: &mut H,
    mut tick: impl FnMut(&mut H),
    is_busy: impl Fn(&H) -> bool,
    start_fn: F,
) where
    F: FnOnce(&mut H),
{
    start_fn(host);
    tick(host);
    while is_busy(host) {
        tick(host);
    }
}

/// Marker retained by `#[rhdl::hls]` expansions (FR35).
#[derive(Debug, Clone, Copy)]
pub struct HlsMark;

/// Compile-time mark expanded by `#[combinational]` / `#[sequential]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessKindMark {
    Combinational,
    Sequential,
}

/// Port direction recorded by `PortField`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDir {
    Input,
    Output,
}

/// Marker trait: only `Input<T>` / `Output<T>` implement this.
/// Bare `UInt<N>` etc. cannot be used as module fields with `#[rhdl::module]`.
///
/// Composites (`Bundle`, [`HwVec`]) flatten to scalar leaf ports before HIR
/// (FR51 / AD-20). Leaf names: `{field}_{member}` / `{field}_{i}`.
pub trait PortField {
    /// Flatten this directed port field into scalar `(leaf_name, dir, ground)` rows.
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)>;
}

/// Hardware ground types (language surface).
#[derive(Debug, Clone, Copy, Default)]
pub struct Bool;

#[derive(Debug, Clone, Copy)]
pub struct Bits<const N: u32>;

#[derive(Debug, Clone, Copy)]
pub struct UInt<const N: u32>;

#[derive(Debug, Clone, Copy)]
pub struct SInt<const N: u32>;

#[derive(Debug, Clone, Copy, Default)]
pub struct Clock;

#[derive(Debug, Clone, Copy, Default)]
pub struct Reset;

/// Directed input port wrapper (AD-18).
#[derive(Debug, Clone, Copy, Default)]
pub struct Input<T>(pub T);

/// Directed output port wrapper (AD-18).
#[derive(Debug, Clone, Copy, Default)]
pub struct Output<T>(pub T);

/// Documented synthesizable named aggregate (FR51).
///
/// Implementors declare **ground** leaves; `Input<Self>` / `Output<Self>` flatten to
/// `{field}_{member}` scalar HIR ports. Does not extend public HIR with Bundle nodes.
///
/// **OUT OF SCOPE (MVP):** nested `Bundle` members and `HwVec<Bundle, _>` — leaves are
/// `GroundType` only; `HwVec` elements must be ground types. **`#[derive(Bundle)]` is not
/// available** — hand-write [`Bundle::leaves`] (documented defer).
pub trait Bundle {
    /// Leaf members `(member_name, GroundType)` — ground only; no nested Bundle.
    fn leaves() -> &'static [(&'static str, GroundType)];
}

/// Hardware vector; documented as the synthesizable `Vec<T,N>` equivalent (FR51).
///
/// Named `HwVec` to avoid collision with heap [`alloc::vec::Vec`] / E0141.
/// `Input<HwVec<T,N>>` flattens to `{field}_0` … `{field}_{N-1}`.
/// Element type must be ground (`Bool` / `UInt` / …); **`HwVec<Bundle, _>` is OUT OF SCOPE**.
#[derive(Debug, Clone, Copy, Default)]
pub struct HwVec<T, const N: u32>(pub core::marker::PhantomData<T>);

/// Clash-style phantom clock-domain marker (AD-22 / FR52).
///
/// **Product surface:** `ClockDomain::<ID>` (this ZST) +
/// [`ElaborateSession::bind_domain`] session domain tags — not a separate
/// `Signal<D, T>` wrapper type. Default modules remain single-clock + **sync
/// active-high** [`Reset`]（AD-15）. Sync/async reset via
/// [`ElaborateSession::declare_reg_ex`] `async_reset`; legal CDC via
/// [`DoubleFlop`]（FR79 真 RTL）或 [`ElaborateSession::mark_cdc_bridge`] /
/// [`SyncFIFO`]（31.3 前叙事），else `finish` → `rhdl::E0220`.
///
/// Fixture: `examples/clockdomain_skel`（FR52 最小合同）；`examples/doubleflop_skel`
///（FR79 真 RTL）。Sim: global `Sim::tick` is the MVP per-domain tick stand-in.
#[derive(Debug, Clone, Copy, Default)]
pub struct ClockDomain<const ID: u32>;

/// 语言级 CDC 两级同步器（FR79 / AD-29；加深 FR23/FR52）。
///
/// **真 RTL：** [`Elaboratable::elaborate`] 发出可综合模块，含 `sync_ff0` /
/// `sync_ff1` 两级寄存器（不得仅以空 ZST + [`ElaborateSession::mark_cdc_bridge`]
/// 交差深度合同）。亦可在会话内用
/// [`ElaborateSession::declare_double_flop_stages`] +
/// [`ElaborateSession::connect_double_flop`] 嵌入更大模块。
///
/// **级数 / 延迟合同：** [`Self::STAGES`] = 2；稳定 `din` 后经
/// [`Self::LATENCY_DST_TICKS`] 个**目的域** tick（MVP：全局
/// `bitloom_sim::Sim::tick`）才在 `dout` 可见。
///
/// **合同外：** 不保证物理亚稳态消除或 MTBF（见
/// `nfr14-risk-epic31-cdc-true-rtl.md`）。
///
/// `SyncFIFO` 真 RTL 见 Epic 31.3；在此之前 `SyncFIFO` 仍可为 bridge 叙事锚点。
#[derive(Debug, Clone, Copy, Default)]
pub struct DoubleFlop;

impl DoubleFlop {
    /// Documented synchronizer stage count (FR79).
    pub const STAGES: u32 = 2;
    /// Destination-domain ticks until `dout` follows a stable `din`.
    pub const LATENCY_DST_TICKS: u32 = 2;
}

impl Elaboratable for DoubleFlop {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        Self::elaborate_width(1)
    }
}

impl DoubleFlop {
    /// Elaborate a DoubleFlop with `width`-bit `din`/`dout` (default path uses 1).
    pub fn elaborate_width(width: u32) -> Result<FrozenHir, Diagnostics> {
        let ty = GroundType::UInt { width };
        let mut s = ElaborateSession::new("DoubleFlop");
        s.begin_module("DoubleFlop", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("din", ty.clone(), Span::default());
        s.add_output("dout", ty.clone(), Span::default());
        // Phantom domains: din = src (0), sync chain + dout = dst (1).
        s.bind_domain("din", 0);
        s.bind_domain("dout", 1);
        let (ff0, ff1) = s.declare_double_flop_stages("sync", ty, 1, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("dout", &ff1, Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.connect_double_flop(&ff0, &ff1, "din", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// 语言级 CDC FIFO 叙事锚点（AD-22 / FR52）。
///
/// 非一级 SyncFIFO IP；Epic 31.3 前仍为 `mark_cdc_bridge` 文档等价名。
/// 与 [`DoubleFlop`]（已真 RTL）深度不对等 — 见 NFR37 / FR79。
#[derive(Debug, Clone, Copy)]
pub struct SyncFIFO<const DEPTH: u32, const WIDTH: u32>;

/// CHIRRTL-friendly SyncReadMem surface marker (AD-21).
#[derive(Debug, Clone, Copy)]
pub struct SyncReadMem<const DEPTH: u32, const WIDTH: u32>;

/// CHIRRTL-friendly Mem (async-read) surface marker (AD-21).
#[derive(Debug, Clone, Copy)]
pub struct Mem<const DEPTH: u32, const WIDTH: u32>;

/// Elaborate-time LUT/ROM/Mem init generator (FR73 / AD-18).
///
/// Runs `f(addr)` for each address and returns plain words for
/// [`ElaborateSession::declare_mem_with_init`] /
/// [`ElaborateSession::declare_sync_read_mem_with_init`]. Prefer session
/// helpers [`ElaborateSession::declare_mem_with_init_fn`] when declaring
/// and initializing in one step. The closure dissolves before freeze —
/// FrozenHir stores only `Vec<u64>` (NFR36).
///
/// **Non-capturing only:** do not capture Wire/Reg/signal refs. Documented
/// illegal capture → [`ElaborateSession::assert_no_hw_capture`] (`rhdl::E0142`).
///
/// ```ignore
/// use bitloom_prelude::{generate_mem_init, ElaborateSession, GroundType, Span};
/// let init = generate_mem_init(16, 8, |i| ((i * i) & 0xff) as u64);
/// session.declare_mem_with_init("rom", 16, 8, init, Span::default());
/// // or: session.declare_mem_with_init_fn("rom", 16, 8, |i| ..., Span::default());
/// ```
pub use bitloom_builder::{generate_mem_init_words as generate_mem_init, mask_mem_word};

/// Module-factory entry points live on [`ElaborateSession`]:
/// [`ElaborateSession::generate_instances`] and
/// [`ElaborateSession::generate_instances_from`] (+ [`GeneratedInstance`])
/// (FR73 / Cap-R-53). Factory `Fn` runs at elaborate time and dissolves to
/// ordinary instance/connect HIR (NFR36). Capturing Wire/Reg →
/// [`ElaborateSession::assert_no_hw_capture`] (`rhdl::E0142`).
///
/// **HwCaptureRef mapping (NFR35):** Wire → [`HwCaptureRef::wire`], Reg →
/// [`HwCaptureRef::reg`], port/signal → [`HwCaptureRef::signal`]. See
/// language-surface “Elaborate-time vs capturing”.

/// SynthesizableClosure constraints + Cap-R-60 check hook (FR74 / Epic 28.1).
///
/// Marker trait [`SynthesizableClosure`] documents Cap-R-48…50 (pure, no heap,
/// no runtime capture state). Call
/// [`ElaborateSession::check_synthesizable_closure`] /
/// [`ElaborateSession::reject_unsynthesizable_closure`] (or free
/// [`diagnose_synthesizable_closure_violations`]) when a documented breach is
/// present. Stable codes: **`rhdl::E0143`** (heap), **`rhdl::E0144`** (runtime
/// capture state), **`rhdl::E0145`** (impure). Legal empty/simple stand-ins:
/// [`LegalEmptyClosure`], [`LegalSimpleClosure`]. Does **not** add closure IR
/// to FrozenHir / backends (NFR36).
///
/// **Comb inline (FR75 / Cap-R-55 / Epic 28.2):**
/// [`ElaborateSession::inline_comb_fn`] / [`ElaborateSession::inline_comb_fn_marker`]
/// + [`CombInline`] — Cap-R-60 check then elaborate-time expand into ordinary
/// `assign_*` / Wire assigns.
///
/// **Seq inline (FR75 / Cap-R-56 / Cap-R-70 / Epic 28.3):**
/// [`ElaborateSession::inline_seq_fn`] / [`ElaborateSession::inline_seq_fn_marker`]
/// + [`SeqInline`] — Cap-R-60 + Cap-R-70 (`rhdl::E0146`) then expand into ordinary
/// `Reg.d` / sequential assigns (NFR36).

trait AsGround {
    fn ground() -> GroundType;
}

impl AsGround for Bool {
    fn ground() -> GroundType {
        GroundType::Bool
    }
}

impl AsGround for Clock {
    fn ground() -> GroundType {
        GroundType::Clock
    }
}

impl AsGround for Reset {
    fn ground() -> GroundType {
        GroundType::Reset
    }
}

impl<const N: u32> AsGround for Bits<N> {
    fn ground() -> GroundType {
        GroundType::UInt { width: N }
    }
}

impl<const N: u32> AsGround for UInt<N> {
    fn ground() -> GroundType {
        GroundType::UInt { width: N }
    }
}

impl<const N: u32> AsGround for SInt<N> {
    fn ground() -> GroundType {
        GroundType::SInt { width: N }
    }
}

fn scalar_leaves(field: &str, dir: PortDir, gt: GroundType) -> Vec<(String, PortDir, GroundType)> {
    vec![(field.to_string(), dir, gt)]
}

fn bundle_leaves<B: Bundle>(field: &str, dir: PortDir) -> Vec<(String, PortDir, GroundType)> {
    B::leaves()
        .iter()
        .map(|(member, gt)| (format!("{field}_{member}"), dir, gt.clone()))
        .collect()
}

fn hwvec_leaves<T: AsGround, const N: u32>(
    field: &str,
    dir: PortDir,
) -> Vec<(String, PortDir, GroundType)> {
    const {
        assert!(N > 0, "HwVec length must be non-zero");
    }
    (0..N)
        .map(|i| (format!("{field}_{i}"), dir, T::ground()))
        .collect()
}

macro_rules! impl_ground_port_field {
    ($ty:ty) => {
        impl PortField for Input<$ty> {
            fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
                scalar_leaves(field, PortDir::Input, <$ty as AsGround>::ground())
            }
        }
        impl PortField for Output<$ty> {
            fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
                scalar_leaves(field, PortDir::Output, <$ty as AsGround>::ground())
            }
        }
    };
}

impl_ground_port_field!(Bool);
impl_ground_port_field!(Clock);
impl_ground_port_field!(Reset);

impl<const N: u32> PortField for Input<Bits<N>> {
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
        scalar_leaves(field, PortDir::Input, Bits::<N>::ground())
    }
}
impl<const N: u32> PortField for Output<Bits<N>> {
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
        scalar_leaves(field, PortDir::Output, Bits::<N>::ground())
    }
}

impl<const N: u32> PortField for Input<UInt<N>> {
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
        scalar_leaves(field, PortDir::Input, UInt::<N>::ground())
    }
}
impl<const N: u32> PortField for Output<UInt<N>> {
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
        scalar_leaves(field, PortDir::Output, UInt::<N>::ground())
    }
}

impl<const N: u32> PortField for Input<SInt<N>> {
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
        scalar_leaves(field, PortDir::Input, SInt::<N>::ground())
    }
}
impl<const N: u32> PortField for Output<SInt<N>> {
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
        scalar_leaves(field, PortDir::Output, SInt::<N>::ground())
    }
}

impl<T: Bundle> PortField for Input<T> {
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
        bundle_leaves::<T>(field, PortDir::Input)
    }
}
impl<T: Bundle> PortField for Output<T> {
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
        bundle_leaves::<T>(field, PortDir::Output)
    }
}

impl<T: AsGround, const N: u32> PortField for Input<HwVec<T, N>> {
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
        hwvec_leaves::<T, N>(field, PortDir::Input)
    }
}
impl<T: AsGround, const N: u32> PortField for Output<HwVec<T, N>> {
    fn flatten(field: &str) -> Vec<(String, PortDir, GroundType)> {
        hwvec_leaves::<T, N>(field, PortDir::Output)
    }
}

/// Register a directed port field, flattening composites to scalar HIR ports.
pub fn add_port_field<P: PortField>(session: &mut ElaborateSession, field: &str, span: Span) {
    for (name, dir, gt) in P::flatten(field) {
        match dir {
            PortDir::Input => session.add_input(name, gt, span),
            PortDir::Output => session.add_output(name, gt, span),
        }
    }
}
