#![allow(unused_imports)]
use crate::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    SynthesizableClosureViolation, diagnose_synthesizable_closure_violations,
};

/// CRC-8 byte → residue for one table entry (MSB-first, poly XOR).
pub fn crc8_table_byte(byte: u8, poly: u8) -> u8 {
    let mut b = byte;
    for _ in 0..8 {
        if b & 0x80 != 0 {
            b = (b << 1) ^ poly;
        } else {
            b <<= 1;
        }
    }
    b
}

/// CRC-8 LUT ROM IP — FR77 / Cap-R-63 generator-closure customization overlay.
///
/// SyncReadMem depth-256 × 8: `addr` → registered `rdata`. Built on Epic 27
/// `declare_sync_read_mem_with_init_fn` / `generate_mem_init`.
///
/// - **Without user closure:** [`Crc8Lut::elaborate`] / [`Elaboratable::elaborate`]
///   use documented default poly [`Crc8Lut::DEFAULT_POLY`] (`0x07`, CRC-8/SMBUS-style).
/// - **With closure:** [`Crc8Lut::elaborate_with_table_fn`] runs `Fn(usize) -> u64`
///   at elaborate time; empty `violations` = legal [`crate::SynthesizableClosure`];
///   non-empty → clear diagnostics (no silent default).
///
/// Closures do **not** enter FrozenHir / emit (NFR36). Not comb/seq synthesizable
/// closure inline (Epic 28) — table generation only.
pub struct Crc8Lut;

impl Crc8Lut {
    pub const DEPTH: u32 = 256;
    pub const WIDTH: u32 = 8;
    /// Documented default polynomial when no customization closure is supplied.
    pub const DEFAULT_POLY: u8 = 0x07;

    /// Elaborate with documented default CRC-8/SMBUS-style poly `0x07` (no user Fn).
    pub fn elaborate_default() -> Result<FrozenHir, Diagnostics> {
        Self::elaborate_with_poly(Self::DEFAULT_POLY)
    }

    /// Elaborate with a fixed polynomial (table built inside the session; no user Fn).
    pub fn elaborate_with_poly(poly: u8) -> Result<FrozenHir, Diagnostics> {
        Self::elaborate_with_table_fn(&[], move |i| crc8_table_byte(i as u8, poly) as u64)
    }

    /// Customize the LUT via an elaborate-time generator closure (FR77 / Cap-R-63).
    ///
    /// `f(addr)` is evaluated for each address inside [`ElaborateSession`] and stored
    /// as plain `Vec<u64>` on the mem node — the Rust `Fn` does not survive freeze
    /// (NFR36). Pass empty `violations` for a legal synthesizable generator; any
    /// documented [`SynthesizableClosureViolation`] fails with clear diagnostics.
    pub fn elaborate_with_table_fn<F>(
        violations: &[SynthesizableClosureViolation],
        f: F,
    ) -> Result<FrozenHir, Diagnostics>
    where
        F: Fn(usize) -> u64,
    {
        if !violations.is_empty() {
            return Err(diagnose_synthesizable_closure_violations(
                violations,
                Span::default(),
            ));
        }

        let mut s = ElaborateSession::new("Crc8Lut");
        s.begin_module("Crc8Lut", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("addr", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("rdata", GroundType::UInt { width: 8 }, Span::default());

        s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
        // Epic 27 path: Fn dissolves to MemDecl.init before freeze (NFR36).
        s.declare_sync_read_mem_with_init_fn("lut", Self::DEPTH, Self::WIDTH, f, Span::default());

        s.begin_combinational(Span::default());
        s.assign_net("rdata", "q", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_mem_read("q", "lut", "addr", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

impl Elaboratable for Crc8Lut {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        Crc8Lut::elaborate_default()
    }
}
