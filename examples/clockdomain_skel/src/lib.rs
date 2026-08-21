//! FR52 夹具：ClockDomain / CDC 产品叙事（AD-22）。
//!
//! - 域绑定：`ClockDomain::<ID>` 叙事锚点 + `bind_domain`
//! - 非法跨域无 bridge → `finish` 失败（`rhdl::E0220`）
//! - 合法跨域：`mark_cdc_bridge`（文档等价 [`DoubleFlop`] / [`SyncFIFO`]）→ emit → tick
//! - 同步·异步复位：`declare_reg_ex`（`async_reset` false/true）；极性 = 默认同步高有效 [`Reset`]（AD-15）
//!
//! **仿真步进：** MVP 使用全局 [`bitloom_sim::Sim::tick`]；文档上等价于「按域 tick」
//!（尚未提供独立 per-domain tick 引擎）。

use bitloom_prelude::{
    ClockDomain, Diagnostics, DoubleFlop, Elaboratable, ElaborateSession, FrozenHir, GroundType,
    Span, SyncFIFO,
};

/// 叙事：源域 `ClockDomain::<0>` 与宿域 `ClockDomain::<1>`。
type SrcDomain = ClockDomain<0>;
type DstDomain = ClockDomain<1>;

/// 构建与 `ClockDomainSkel` 相同的多域网表；`with_bridge` 控制是否 `mark_cdc_bridge("y")`。
fn elaborate_cdc(with_bridge: bool) -> Result<FrozenHir, Diagnostics> {
    let _src: SrcDomain = ClockDomain;
    let _dst: DstDomain = ClockDomain;
    // 叙事锚点：合法路径文档名 = DoubleFlop / SyncFIFO（实现为 mark_cdc_bridge）
    let _df = DoubleFlop;
    let _fifo: SyncFIFO<4, 8> = SyncFIFO;

    let mut s = ElaborateSession::new("ClockDomainSkel");
    s.begin_module("ClockDomainSkel", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());

    // ClockDomain::<0> / ClockDomain::<1> 叙事 → session 域标签
    s.bind_domain("a", 0);
    s.bind_domain("y", 1);
    if with_bridge {
        // 合法跨域：桥接信号标记（诊断文案指向 DoubleFlop/SyncFIFO）
        s.mark_cdc_bridge("y");
    }

    // 同步·异步复位并排展示（AD-15 极性高有效；AD-23 async 标志）
    s.declare_reg_ex(
        "q_sync",
        GroundType::UInt { width: 8 },
        false, // sync reset
        false,
        Span::default(),
    );
    s.declare_reg_ex(
        "q_async",
        GroundType::UInt { width: 8 },
        true, // async_reset
        false,
        Span::default(),
    );
    s.bind_domain("q_sync", 0);
    s.bind_domain("q_async", 0);

    s.begin_combinational(Span::default());
    // 直接跨域 assign（与 Design Notes / builder `cdc_bridge_allows_crossing` 同形）
    s.assign_net("y", "a", Span::default());
    s.end_process();

    s.begin_sequential(Span::default());
    s.assign_reg_d_from("q_sync", "a", Span::default());
    s.assign_reg_d_from("q_async", "a", Span::default());
    s.end_process();

    s.end_module();
    s.finish()
}

/// 合法 CDC 路径夹具：经 `mark_cdc_bridge`（DoubleFlop / SyncFIFO 文档等价）。
///
/// `DoubleFlop` / `SyncFIFO` 为语言级 ZST 锚点，不生成真实同步器 RTL IP。
pub struct ClockDomainSkel;

impl Elaboratable for ClockDomainSkel {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        elaborate_cdc(true)
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    ClockDomainSkel::elaborate()
}

#[cfg(test)]
mod tests {
    use bitloom_hir::PortValues;
    use bitloom_prelude::{Elaboratable, ElaborateSession, GroundType, Span};
    use bitloom_sim::Sim;
    use bitloom_vlog::emit;

    use super::*;

    fn assert_e0220(err: &bitloom_prelude::Diagnostics) {
        let hit = err.0.iter().find(|d| d.code == "rhdl::E0220");
        assert!(hit.is_some(), "expected rhdl::E0220, got {err}");
        let d = hit.unwrap();
        assert!(
            d.en.contains("DoubleFlop") || d.en.contains("SyncFIFO"),
            "EN diagnostic should name DoubleFlop/SyncFIFO: {}",
            d.en
        );
        assert!(
            d.zh.contains("DoubleFlop") || d.zh.contains("SyncFIFO"),
            "ZH diagnostic should name DoubleFlop/SyncFIFO: {}",
            d.zh
        );
    }

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
    fn illegal_cross_domain_assign_fails_with_e0220() {
        let mut s = ElaborateSession::new("CdcIllegal");
        s.begin_module("CdcIllegal", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.bind_domain("a", 0); // ClockDomain::<0>
        s.bind_domain("y", 1); // ClockDomain::<1>
        // 无 mark_cdc_bridge → 非法跨域
        s.begin_combinational(Span::default());
        s.assign_net("y", "a", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("illegal CDC must fail freeze");
        assert_e0220(&err);
        // 无 FrozenHir → 不得 emit（finish Err，无 hir 可传给 emit）
    }

    #[test]
    fn fixture_without_bridge_fails_with_e0220() {
        // 与产品夹具同形网表，仅去掉 mark_cdc_bridge → 钉死合法路径依赖 bridge
        let err = elaborate_cdc(false).expect_err("fixture twin without bridge must fail");
        assert_e0220(&err);
    }

    #[test]
    fn domain_bind_narrative_with_clockdomain_markers() {
        let _src: SrcDomain = ClockDomain;
        let _dst: DstDomain = ClockDomain;
        let mut s = ElaborateSession::new("BindDemo");
        s.begin_module("BindDemo", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.bind_domain("a", 0);
        s.bind_domain("y", 0); // 同域
        s.begin_combinational(Span::default());
        s.assign_net("y", "a", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok(), "same-domain assign must succeed");
    }

    #[test]
    fn sync_and_async_reset_shown_in_fixture_hir() {
        let hir = ClockDomainSkel::elaborate().expect("elaborate");
        assert!(
            !hir.circuit().modules.is_empty(),
            "fixture must emit at least one module"
        );
        let body = &hir.circuit().modules[0].body;
        assert!(
            body.iter().any(|st| matches!(
                st,
                bitloom_hir::Stmt::RegDecl {
                    name,
                    async_reset: false,
                    ..
                } if name == "q_sync"
            )),
            "fixture must declare sync reset reg"
        );
        assert!(
            body.iter().any(|st| matches!(
                st,
                bitloom_hir::Stmt::RegDecl {
                    name,
                    async_reset: true,
                    ..
                } if name == "q_async"
            )),
            "fixture must declare async_reset via declare_reg_ex"
        );
    }

    #[test]
    fn legal_cdc_elaborate_emit_tick() {
        let hir = ClockDomainSkel::elaborate().expect("elaborate with mark_cdc_bridge");
        let art = emit(&hir);
        assert_eq!(art.filelist, vec!["ClockDomainSkel.v"]);
        assert!(!art.files.is_empty(), "emit must produce files");
        let v = &art.files[0].contents;
        assert!(v.contains("module ClockDomainSkel"));
        assert!(
            v.contains("async_reset"),
            "emit should annotate async_reset"
        );

        // 全局 Sim::tick = 按域 tick 的 MVP 等价（见 crate 文档）
        let mut sim = Sim::new(hir);
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("a", 0);
        sim.set_inputs(pv.clone());
        sim.tick();
        assert_eq!(sim.ports().get("y"), Some(0));

        pv.set("rst", 0);
        pv.set("a", 0xA5);
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("y"), Some(0xA5));
    }
}
