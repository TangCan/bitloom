//! FR79 夹具：DoubleFlop 可综合同步器真 RTL（AD-29）。
//!
//! - prelude [`DoubleFlop`] → HIR → emit 两级 `sync_ff*` 寄存器
//! - 目的域 tick（MVP：全局 [`bitloom_sim::Sim::tick`]）黄金延迟 = 2
//! - 非法未同步跨域仍 `rhdl::E0220`
//!
//! 与 `examples/clockdomain_skel`（FR52 `mark_cdc_bridge` 最小合同）对照见
//! `docs/fr79-doubleflop-cdc.md`。

use bitloom_prelude::{Diagnostics, DoubleFlop, Elaboratable, FrozenHir};

/// 产品夹具：文档化 DoubleFlop 真 RTL 模块。
pub struct DoubleFlopSkel;

impl Elaboratable for DoubleFlopSkel {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        DoubleFlop::elaborate()
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    DoubleFlopSkel::elaborate()
}

#[cfg(test)]
mod tests {
    use bitloom_hir::PortValues;
    use bitloom_prelude::{DoubleFlop, Elaboratable, ElaborateSession, GroundType, Span};
    use bitloom_sim::Sim;
    use bitloom_vlog::emit;

    use super::*;

    #[test]
    fn design_crate_depends_only_on_prelude() {
        let manifest = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));
        let deps = manifest
            .split("[dev-dependencies]")
            .next()
            .expect("deps section");
        assert!(
            deps.contains("bitloom-prelude"),
            "design must depend on bitloom-prelude"
        );
        for banned in [
            "bitloom-builder",
            "bitloom-hir",
            "bitloom-vlog",
            "bitloom-sim",
            "bitloom-macro",
            "bitloom ",
        ] {
            assert!(
                !deps.contains(banned),
                "design [dependencies] must not include {banned}"
            );
        }
    }

    #[test]
    fn elaborate_emit_two_stage_regs() {
        let hir = DoubleFlopSkel::elaborate().expect("elaborate");
        let art = emit(&hir);
        let v = &art.files[0].contents;
        assert!(v.contains("sync_ff0"), "{v}");
        assert!(v.contains("sync_ff1"), "{v}");
        assert!(v.contains("always @(posedge"), "{v}");
    }

    #[test]
    fn tick_golden_two_dst_cycle_latency() {
        let hir = DoubleFlopSkel::elaborate().expect("elaborate");
        let mut sim = Sim::new(hir);
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("din", 0);
        sim.set_inputs(pv.clone());
        sim.tick();

        pv.set("rst", 0);
        pv.set("din", 1);
        sim.set_inputs(pv.clone());
        sim.tick();
        assert_eq!(sim.ports().get("dout"), Some(0));
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("dout"), Some(1));
        assert_eq!(DoubleFlop::LATENCY_DST_TICKS, 2);
    }

    #[test]
    fn illegal_cross_domain_still_e0220() {
        let mut s = ElaborateSession::new("Illegal");
        s.begin_module("Illegal", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 1 }, Span::default());
        s.bind_domain("a", 0);
        s.bind_domain("y", 1);
        s.begin_combinational(Span::default());
        s.assign_net("y", "a", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("must fail");
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0220"), "{err}");
    }
}
