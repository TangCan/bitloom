//! Design-facing surface. Designs depend only on this crate.

pub use bitloom_builder::{Elaboratable, ElaborateSession};
pub use bitloom_hir::{Diagnostics, FrozenHir, GroundType, PortDirection, PortValues, Span};

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
pub trait PortField {
    fn describe() -> (PortDir, GroundType);
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

/// Clash-style phantom clock domain marker (AD-22).
#[derive(Debug, Clone, Copy, Default)]
pub struct ClockDomain<const ID: u32>;

/// Language-level CDC primitives (AD-22).
#[derive(Debug, Clone, Copy)]
pub struct DoubleFlop;

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

impl<T: AsGround> PortField for Input<T> {
    fn describe() -> (PortDir, GroundType) {
        (PortDir::Input, T::ground())
    }
}

impl<T: AsGround> PortField for Output<T> {
    fn describe() -> (PortDir, GroundType) {
        (PortDir::Output, T::ground())
    }
}
