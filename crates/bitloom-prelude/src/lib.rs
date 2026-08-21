//! Design-facing surface. Designs depend only on this crate.

pub use bitloom_builder::{Elaboratable, ElaborateSession};
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
/// [`ElaborateSession::mark_cdc_bridge`]（[`DoubleFlop`] / [`SyncFIFO`]），else
/// `finish` → `rhdl::E0220`.
///
/// Fixture: `examples/clockdomain_skel`. Sim: global `Sim::tick` is the MVP
/// per-domain tick stand-in.
#[derive(Debug, Clone, Copy, Default)]
pub struct ClockDomain<const ID: u32>;

/// 语言级 CDC 原语叙事锚点（AD-22 / FR52）。
///
/// 不生成真实双触发器 RTL；合法跨域今日为 session
/// [`ElaborateSession::mark_cdc_bridge`]（诊断文案指向 DoubleFlop/SyncFIFO）。
#[derive(Debug, Clone, Copy)]
pub struct DoubleFlop;

/// 语言级 CDC FIFO 叙事锚点（AD-22 / FR52）。
///
/// 非一级 SyncFIFO IP；与 [`DoubleFlop`] 同为 `mark_cdc_bridge` 文档等价名。
#[derive(Debug, Clone, Copy)]
pub struct SyncFIFO<const DEPTH: u32, const WIDTH: u32>;

/// CHIRRTL-friendly SyncReadMem surface marker (AD-21).
#[derive(Debug, Clone, Copy)]
pub struct SyncReadMem<const DEPTH: u32, const WIDTH: u32>;

/// CHIRRTL-friendly Mem (async-read) surface marker (AD-21).
#[derive(Debug, Clone, Copy)]
pub struct Mem<const DEPTH: u32, const WIDTH: u32>;

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
