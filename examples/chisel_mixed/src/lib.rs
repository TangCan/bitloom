//! FR46 mixed fixture: Bitloom-side RTL and an external Chisel/firtool-style `.fir`
//! both enter the same Verilog emit backend.

use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Bitloom-authored passthrough (design crate depends only on `bitloom-prelude`).
pub struct BitloomPass;

impl Elaboratable for BitloomPass {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("BitloomPass");
        s.begin_module("BitloomPass", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "x", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// Entry for `cargo bitloom build --package chisel_mixed`.
pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    BitloomPass::elaborate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitloom_prelude::Elaboratable;

    /// Documented mixed flow: Bitloom elaborate and external `.fir` import → same `bitloom_vlog::emit`.
    #[test]
    fn mixed_bitloom_and_fir_same_emit_backend() {
        let bitloom_hir = BitloomPass::elaborate().expect("elaborate Bitloom side");
        let bitloom_v = bitloom_vlog::emit(&bitloom_hir);
        assert!(
            bitloom_v.files[0].contents.contains("module BitloomPass"),
            "Bitloom side must emit Verilog"
        );

        // Same external hierarchy fixture used by FR46 library ATDD (firtool-style connects).
        let fir = include_str!("../../../crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
        assert!(fir.contains("y <= u0.y"));
        let fir_hir = rhdl_firrtl::import(fir).expect("import Chisel/.fir side");
        let fir_v = bitloom_vlog::emit(&fir_hir);
        assert!(
            fir_v
                .files
                .iter()
                .any(|f| f.contents.contains("module ExternalTop")
                    || f.contents.contains("module Child")),
            "imported .fir must emit Verilog via the same backend"
        );

        // Both also re-emit FIRRTL 6.0.0 (AD-3 path shared with import CLI).
        assert!(
            rhdl_firrtl::emit(&bitloom_hir).files[0]
                .contents
                .starts_with("FIRRTL version 6.0.0")
        );
        assert!(
            rhdl_firrtl::emit(&fir_hir).files[0]
                .contents
                .starts_with("FIRRTL version 6.0.0")
        );
    }

    #[test]
    fn design_crate_depends_only_on_prelude() {
        let toml = include_str!("../Cargo.toml");
        let deps = toml.split("[dev-dependencies]").next().unwrap_or(toml);
        assert!(
            deps.contains("bitloom-prelude"),
            "runtime deps must include bitloom-prelude"
        );
        assert!(
            !deps.contains("rhdl-firrtl") && !deps.contains("bitloom-vlog"),
            "design [dependencies] must not pull toolchain crates"
        );
    }
}
