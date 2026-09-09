//! FR79 夹具：SyncFIFO 可综合跨域 FIFO 真 RTL（AD-29）。
//!
//! - prelude [`SyncFIFO`] → HIR → emit mem + 灰码 DoubleFlop + full/empty
//! - 写域 D0 / 读域 D1（phantom；全局 [`bitloom_sim::Sim::tick`] MVP）
//! - 非法未同步跨域仍 `rhdl::E0220`
//!
//! 与 `ip::SyncFifo`（FR82 单时钟）及 `clockdomain_skel`（FR52 最小合同）消歧见
//! `docs/fr79-syncfifo-cdc.md`。

use bitloom_prelude::{Diagnostics, Elaboratable, FrozenHir, SyncFIFO};

/// 产品夹具：文档化 SyncFIFO 真 RTL 模块。
pub struct SyncFifoSkel;

impl Elaboratable for SyncFifoSkel {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        SyncFIFO::<4, 8>::elaborate()
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    SyncFifoSkel::elaborate()
}

#[cfg(test)]
mod tests {
    use bitloom_hir::PortValues;
    use bitloom_prelude::{Elaboratable, ElaborateSession, GroundType, Span, SyncFIFO};
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
    fn elaborate_emit_fifo_and_sync() {
        let hir = SyncFifoSkel::elaborate().expect("elaborate");
        let art = emit(&hir);
        let v = &art.files[0].contents;
        assert!(v.contains("module SyncFIFO"), "{v}");
        assert!(v.contains("full") && v.contains("empty"), "{v}");
        assert!(v.contains("w2r_ff0") || v.contains("r2w_ff0"), "{v}");
        assert!(v.contains("always @(posedge"), "{v}");
    }

    #[test]
    fn tick_cross_domain_write_read() {
        let hir = SyncFifoSkel::elaborate().expect("elaborate");
        let mut sim = Sim::new(hir);
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("wr_en", 0);
        pv.set("rd_en", 0);
        pv.set("data_in", 0);
        sim.set_inputs(pv.clone());
        sim.settle();
        sim.tick();

        pv.set("rst", 0);
        pv.set("wr_en", 1);
        pv.set("data_in", 0x5A);
        sim.set_inputs(pv.clone());
        sim.settle();
        sim.tick();

        pv.set("wr_en", 0);
        for _ in 0..SyncFIFO::<4, 8>::LATENCY_PTR_SYNC_TICKS {
            sim.set_inputs(pv.clone());
            sim.settle();
            sim.tick();
        }
        assert_eq!(sim.ports().get("empty"), Some(0));

        pv.set("rd_en", 1);
        sim.set_inputs(pv);
        sim.settle();
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(0x5A));
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
