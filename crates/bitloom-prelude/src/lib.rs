//! Design-facing surface. Designs depend only on this crate.

pub use bitloom_builder::{
    CombInline, Elaboratable, ElaborateSession, GeneratedInstance, HwCaptureKind, HwCaptureRef,
    LegalEmptyClosure, LegalSimpleClosure, SeqInline, SeqOwnershipViolation,
    SeqOwnershipViolationKind, SynthesizableClosure, SynthesizableClosureViolation,
    SynthesizableClosureViolationKind, diagnose_seq_ownership_violations,
    diagnose_synthesizable_closure_violations,
};
pub use bitloom_hir::{Diagnostics, FrozenHir, GroundType, PortDirection, PortValues, Span};
/// `#[derive(Bundle)]` (FR80) — same name as the [`Bundle`] trait (macro vs type namespace).
pub use bitloom_macro::Bundle;

/// First-class IP stubs (FR37 / FR48): FIFO, UART, … via this prelude only.
pub mod ip;

/// Re-export attributes via this crate's `rhdl` facade.
///
/// **FR102 multi-view matrix:** `functional_model` / `abstraction` / `bridge` / `both`
/// are type attributes; `functional_state` is an **inert field attribute** recognized
/// by [`module`](rhdl::module) and HostView macros (stripped on expand; never HIR).
/// See `docs/fr102-multiview-attribute-matrix.md`.
///
/// **FR157:** [`fsm`](rhdl::fsm) marks an FSM state enum for automatic label extraction.
pub mod rhdl {
    pub use bitloom_macro::{
        abstraction, both, bridge, combinational, fsm, functional_model, hls, module, process,
        sequential, top,
    };
}

/// Bitloom-branded attribute facade (FR118 / FR157 text paths use `#[bitloom::…]`).
///
/// Prefer this for new design annotations; [`rhdl`] remains the transitional alias.
pub mod bitloom {
    pub use bitloom_macro::{fsm, top};
}

/// FR157: compile-time FSM state label set (from `#[bitloom::fsm]` / `#[rhdl::fsm]`).
///
/// Feed `bitloom_sim::Sim::register_fsm_states` with [`Self::FSM_ID`] and
/// [`Self::state_labels`] — does **not** replace FR109 visit sampling.
pub trait FsmLabels {
    /// FSM instance id used in coverage keys (`fsm:<id>:<label>`).
    const FSM_ID: &'static str;
    /// Ordered state labels (enum variant names).
    fn state_labels() -> &'static [&'static str];
}

/// Host-only simulation view kind (FR29 / FR102). Never a FrozenHir node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewKind {
    FunctionalModel,
    Bridge,
    Abstraction,
    Both,
}

/// Marker implemented by `#[rhdl::bridge]` / `#[rhdl::abstraction]` / `#[rhdl::both]` /
/// `#[rhdl::functional_model]`. These types do not enter HIR.
///
/// Soft fields on modules / host views use inert `#[functional_state]` /
/// `#[rhdl::functional_state]` (FR102) — never registered as FrozenHir ports.
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
/// (FR51 / FR80 / AD-20). Leaf names: `{field}_{member}` / `{field}_{nested}_{leaf}` /
/// `{field}_{i}`.
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

/// Documented synthesizable named aggregate (FR51 / FR80).
///
/// Implementors declare ground leaves and optional **one-level** nested Bundle
/// members via [`Self::nested_bundles`]. `Input<Self>` / `Output<Self>` flatten to
/// `{field}_{member}` or `{field}_{nested}_{leaf}` scalar HIR ports. Does not extend
/// public HIR with Bundle nodes.
///
/// **Nesting (FR80 / AD-20):** at least one documented nesting level is in scope —
/// child Bundles are referenced by their ground [`Self::leaves`] functions. Deeper
/// nesting (≥2 levels) is deferred / non-goal for the Epic 32 default contract; do
/// not claim arbitrary depth.
///
/// **Derive (FR80):** `#[derive(Bundle)]` is available via this prelude (AD-6 —
/// design crates must not depend on `bitloom-macro` / CLI). Supported: named-field
/// structs; ground fields `Bool` / `Clock` / `Reset` / `UInt<N>` / `SInt<N>` /
/// `Bits<N>`; other simple path types as one-level nested Bundles. Rejected with
/// stable `rhdl::E0180` diagnostics: enums, tuple/unit structs, generics on the
/// struct, `HwVec<_>` / `Input<_>` / `Output<_>` fields, and non-path field types.
///
/// **Still OUT OF SCOPE:** `HwVec<Bundle, _>` — `HwVec` elements must be ground.
pub trait Bundle {
    /// Ground leaf members at this Bundle level `(member_name, GroundType)`.
    fn leaves() -> &'static [(&'static str, GroundType)];

    /// One-level nested Bundle members: `(member_name, nested_leaves_fn)`.
    ///
    /// Typical entry: `("stream", Stream::leaves)`. Nested leaves must be ground;
    /// the nested fn does not recurse into further `nested_bundles` under the Epic 32
    /// one-level contract.
    fn nested_bundles() -> &'static [(&'static str, fn() -> &'static [(&'static str, GroundType)])]
    {
        &[]
    }
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
/// [`DoubleFlop`] / [`SyncFIFO`]（FR79 真 RTL）或 [`ElaborateSession::mark_cdc_bridge`]
///（FR52 最小合同），else `finish` → `rhdl::E0220`.
///
/// Fixture: `examples/clockdomain_skel`（FR52 最小合同）；`examples/doubleflop_skel`
/// / `examples/syncfifo_skel`（FR79 真 RTL）。Sim: global `Sim::tick` is the MVP
/// per-domain tick stand-in.
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
/// 多位/流式 CDC 见 [`SyncFIFO`]（FR79）。
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

/// 语言级跨域 FIFO（FR79 / AD-29；加深 FR23/FR52）。
///
/// **真 RTL：** [`Elaboratable::elaborate`]（默认 [`SyncFIFO`]`<4, 8>`）发出可综合
/// 模块：`mem` + 二进制/灰码指针 + 经 [`DoubleFlop`] 级数同步的跨域灰码 +
/// `full`/`empty`（不得仅以空 ZST + [`ElaborateSession::mark_cdc_bridge`] 交差）。
///
/// **文档化最小子集：** `DEPTH=4`、`WIDTH=8`；单物理 `clk`/`rst` + phantom 写域 D0 /
/// 读域 D1（与 DoubleFlop MVP 一致）；全局 `Sim::tick` ≡ 按域 tick。
///
/// **延迟：** 灰码指针同步 [`Self::LATENCY_PTR_SYNC_TICKS`]（= DoubleFlop 2）；
/// 注册读 [`Self::LATENCY_REG_READ_TICKS`]。详见 `docs/fr79-syncfifo-cdc.md`。
///
/// **≠** [`ip::SyncFifo`]（FR82 单时钟一级 IP）。合同外：非 MTBF / 非双物理时钟端口矩阵。
#[derive(Debug, Clone, Copy)]
pub struct SyncFIFO<const DEPTH: u32, const WIDTH: u32>;

impl<const DEPTH: u32, const WIDTH: u32> SyncFIFO<DEPTH, WIDTH> {
    /// Documented FIFO depth (FR79 MVP default 4).
    pub const DEPTH: u32 = DEPTH;
    /// Documented data width (FR79 MVP default 8).
    pub const WIDTH: u32 = WIDTH;
    /// Binary/gray pointer width = clog2(DEPTH)+1 (DEPTH must be power of two).
    pub const PTR_WIDTH: u32 = {
        match DEPTH {
            4 => 3,
            _ => 3, // MVP documents DEPTH=4 only; other depths are Ask First.
        }
    };
    /// Destination-domain ticks for gray pointer DoubleFlop sync (empty/full observe).
    pub const LATENCY_PTR_SYNC_TICKS: u32 = DoubleFlop::LATENCY_DST_TICKS;
    /// Registered mem-read latency to `data_out` after an accepted `rd_en`.
    pub const LATENCY_REG_READ_TICKS: u32 = 1;
}

impl Elaboratable for SyncFIFO<4, 8> {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        Self::elaborate_cdc()
    }
}

impl SyncFIFO<4, 8> {
    /// Elaborate the FR79 CDC SyncFIFO MVP (DEPTH=4, WIDTH=8).
    pub fn elaborate_cdc() -> Result<FrozenHir, Diagnostics> {
        const DEPTH: u32 = 4;
        const WIDTH: u32 = 8;
        const PTR_W: u32 = 3;
        let data_ty = GroundType::UInt { width: WIDTH };
        let ptr_ty = GroundType::UInt { width: PTR_W };
        let bit_ty = GroundType::UInt { width: 1 };

        let mut s = ElaborateSession::new("SyncFIFO");
        s.begin_module("SyncFIFO", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("wr_en", bit_ty.clone(), Span::default());
        s.add_input("rd_en", bit_ty.clone(), Span::default());
        s.add_input("data_in", data_ty.clone(), Span::default());
        s.add_output("data_out", data_ty.clone(), Span::default());
        s.add_output("full", bit_ty.clone(), Span::default());
        s.add_output("empty", bit_ty.clone(), Span::default());

        // Write domain D0, read domain D1 (phantom; single physical clk MVP).
        s.bind_domain("wr_en", 0);
        s.bind_domain("data_in", 0);
        s.bind_domain("full", 0);
        s.bind_domain("rd_en", 1);
        s.bind_domain("data_out", 1);
        s.bind_domain("empty", 1);

        s.declare_mem("ram", DEPTH, WIDTH, Span::default());
        s.declare_reg("wr_ptr", ptr_ty.clone(), Span::default());
        s.declare_reg("rd_ptr", ptr_ty.clone(), Span::default());
        s.declare_reg("dout", data_ty, Span::default());
        s.bind_domain("wr_ptr", 0);
        s.bind_domain("rd_ptr", 1);
        s.bind_domain("dout", 1);

        // DoubleFlop sync gray(wr) → read domain; gray(rd) → write domain.
        let (w2r_ff0, w2r_ff1) =
            s.declare_double_flop_stages("w2r", ptr_ty.clone(), 1, Span::default());
        let (r2w_ff0, r2w_ff1) =
            s.declare_double_flop_stages("r2w", ptr_ty.clone(), 0, Span::default());

        // Constants / comb helpers
        s.declare_wire("c0_1", bit_ty.clone(), Span::default());
        s.declare_wire("c1_1", bit_ty.clone(), Span::default());
        s.declare_wire("c0_3", ptr_ty.clone(), Span::default());
        s.declare_wire("c1_3", ptr_ty.clone(), Span::default());
        s.declare_wire("c3_3", ptr_ty.clone(), Span::default());
        s.declare_wire("c7_3", ptr_ty.clone(), Span::default());
        s.declare_wire("wr_ptr_p1", ptr_ty.clone(), Span::default());
        s.declare_wire("rd_ptr_p1", ptr_ty.clone(), Span::default());
        s.declare_wire("wr_ptr_shr", ptr_ty.clone(), Span::default());
        s.declare_wire("rd_ptr_shr", ptr_ty.clone(), Span::default());
        s.declare_wire("wr_gray", ptr_ty.clone(), Span::default());
        s.declare_wire("rd_gray", ptr_ty.clone(), Span::default());
        s.declare_wire("wr_addr", ptr_ty.clone(), Span::default());
        s.declare_wire("rd_addr", ptr_ty.clone(), Span::default());
        s.declare_wire("rd_sync_shr", ptr_ty.clone(), Span::default());
        s.declare_wire("rd_sync_hi_x", ptr_ty.clone(), Span::default());
        s.declare_wire("rd_sync_hi", ptr_ty.clone(), Span::default());
        s.declare_wire("rd_sync_hi_shl", ptr_ty.clone(), Span::default());
        s.declare_wire("rd_sync_lo", ptr_ty.clone(), Span::default());
        s.declare_wire("rd_fold", ptr_ty.clone(), Span::default());
        s.declare_wire("can_wr", bit_ty.clone(), Span::default());
        s.declare_wire("can_rd", bit_ty.clone(), Span::default());
        s.declare_wire("next_wr", ptr_ty.clone(), Span::default());
        s.declare_wire("next_rd", ptr_ty.clone(), Span::default());
        s.declare_wire("not_full", bit_ty.clone(), Span::default());
        s.declare_wire("not_empty", bit_ty, Span::default());

        s.bind_domain("wr_gray", 0);
        s.bind_domain("rd_fold", 0);
        s.bind_domain("can_wr", 0);
        s.bind_domain("next_wr", 0);
        s.bind_domain("rd_gray", 1);
        s.bind_domain("can_rd", 1);
        s.bind_domain("next_rd", 1);
        // Bridged sync outputs already marked via declare_double_flop_stages.
        s.mark_cdc_bridge("wr_gray");
        s.mark_cdc_bridge("rd_gray");

        s.begin_combinational(Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        s.assign_lit("c0_3", 0, Span::default());
        s.assign_lit("c1_3", 1, Span::default());
        s.assign_lit("c3_3", 3, Span::default());
        s.assign_lit("c7_3", 7, Span::default());

        s.assign_add("wr_ptr_p1", "wr_ptr", "c1_3", Span::default());
        s.assign_add("rd_ptr_p1", "rd_ptr", "c1_3", Span::default());
        s.assign_and("wr_addr", "wr_ptr", "c3_3", Span::default());
        s.assign_and("rd_addr", "rd_ptr", "c3_3", Span::default());

        // gray = bin ^ (bin >> 1)
        s.assign_shr("wr_ptr_shr", "wr_ptr", "c1_3", Span::default());
        s.assign_xor("wr_gray", "wr_ptr", "wr_ptr_shr", Span::default());
        s.assign_shr("rd_ptr_shr", "rd_ptr", "c1_3", Span::default());
        s.assign_xor("rd_gray", "rd_ptr", "rd_ptr_shr", Span::default());

        // rd_fold = {~rd_sync[2:1], rd_sync[0]} for Cummings full compare
        // = ((~(rd>>1) & 3) << 1) | (rd & 1)
        s.assign_shr("rd_sync_shr", &r2w_ff1, "c1_3", Span::default());
        s.assign_xor("rd_sync_hi_x", "rd_sync_shr", "c7_3", Span::default());
        s.assign_and("rd_sync_hi", "rd_sync_hi_x", "c3_3", Span::default());
        s.assign_shl("rd_sync_hi_shl", "rd_sync_hi", "c1_3", Span::default());
        s.assign_and("rd_sync_lo", &r2w_ff1, "c1_3", Span::default());
        s.assign_or("rd_fold", "rd_sync_hi_shl", "rd_sync_lo", Span::default());

        s.assign_eq("full", "wr_gray", "rd_fold", Span::default());
        s.assign_eq("empty", "rd_gray", &w2r_ff1, Span::default());
        // not_full / not_empty via mux against constants (no bitwise not on 1-bit eq)
        s.assign_mux("not_full", "full", "c0_1", "c1_1", Span::default());
        s.assign_mux("not_empty", "empty", "c0_1", "c1_1", Span::default());
        s.assign_and("can_wr", "wr_en", "not_full", Span::default());
        s.assign_and("can_rd", "rd_en", "not_empty", Span::default());
        s.assign_mux("next_wr", "can_wr", "wr_ptr_p1", "wr_ptr", Span::default());
        s.assign_mux("next_rd", "can_rd", "rd_ptr_p1", "rd_ptr", Span::default());
        s.assign_net("data_out", "dout", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_mem_write_en("ram", "wr_addr", "data_in", "can_wr", Span::default());
        s.assign_reg_d_mem_read("dout", "ram", "rd_addr", Span::default());
        s.assign_reg_d_from("wr_ptr", "next_wr", Span::default());
        s.assign_reg_d_from("rd_ptr", "next_rd", Span::default());
        s.connect_double_flop(&w2r_ff0, &w2r_ff1, "wr_gray", Span::default());
        s.connect_double_flop(&r2w_ff0, &r2w_ff1, "rd_gray", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

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
    let mut out: Vec<(String, PortDir, GroundType)> = B::leaves()
        .iter()
        .map(|(member, gt)| (format!("{field}_{member}"), dir, gt.clone()))
        .collect();
    for (nested, nested_leaves) in B::nested_bundles() {
        for (leaf, gt) in nested_leaves() {
            out.push((format!("{field}_{nested}_{leaf}"), dir, gt.clone()));
        }
    }
    out
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
