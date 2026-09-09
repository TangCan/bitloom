//! FR79 / AD-29 — DoubleFlop synthesizable CDC synchronizer (Story 31.2).
//!
//! ATDD: prelude API → HIR → emit recognizable two-stage sync regs; dst-domain
//! tick golden delay; illegal unsync cross-domain still E0220; not empty ZST.

use bitloom_hir::{PortValues, Stmt};
use bitloom_prelude::{Diagnostics, DoubleFlop, Elaboratable, ElaborateSession, GroundType, Span};
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn assert_e0220(err: &Diagnostics) {
    let hit = err.0.iter().find(|d| d.code == "rhdl::E0220");
    assert!(hit.is_some(), "expected rhdl::E0220, got {err}");
    let d = hit.unwrap();
    assert!(
        d.en.contains("DoubleFlop") || d.en.contains("SyncFIFO"),
        "EN diagnostic should name DoubleFlop/SyncFIFO: {}",
        d.en
    );
}

#[test]
fn fr79_doubleflop_stages_documented_as_two() {
    assert_eq!(DoubleFlop::STAGES, 2, "documented stage count must be 2");
    assert_eq!(
        DoubleFlop::LATENCY_DST_TICKS,
        2,
        "dout latency must be 2 destination-domain ticks"
    );
}

#[test]
fn fr79_doubleflop_elaborate_emits_two_sync_regs() {
    let hir = DoubleFlop::elaborate().expect("DoubleFlop must elaborate");
    let body = &hir.circuit().modules[0].body;
    let reg_names: Vec<&str> = body
        .iter()
        .filter_map(|st| match st {
            Stmt::RegDecl { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    assert!(
        reg_names
            .iter()
            .any(|n| n.contains("ff0") || n.ends_with("_0")),
        "must declare stage-0 sync reg, got {reg_names:?}"
    );
    assert!(
        reg_names
            .iter()
            .any(|n| n.contains("ff1") || n.ends_with("_1")),
        "must declare stage-1 sync reg, got {reg_names:?}"
    );
    assert!(
        reg_names.len() >= 2,
        "NFR37: must not be empty ZST — need ≥2 regs, got {reg_names:?}"
    );

    let art = emit(&hir);
    assert!(
        art.filelist.iter().any(|f| f.contains("DoubleFlop")),
        "emit filelist should name DoubleFlop: {:?}",
        art.filelist
    );
    let v = &art.files[0].contents;
    assert!(v.contains("module DoubleFlop"), "{v}");
    assert!(
        v.contains("reg") && (v.contains("ff0") || v.contains("sync")),
        "emit .v must show recognisable sync registers: {v}"
    );
    assert!(
        v.contains("always @(posedge"),
        "emit must have sequential always for sync flops: {v}"
    );
}

#[test]
fn fr79_doubleflop_tick_golden_two_dst_cycle_delay() {
    let hir = DoubleFlop::elaborate().expect("elaborate");
    let mut sim = Sim::new(hir);
    let mut pv = PortValues::default();

    // Reset
    pv.set("rst", 1);
    pv.set("din", 0);
    sim.set_inputs(pv.clone());
    sim.tick();
    assert_eq!(sim.ports().get("dout"), Some(0));

    // Release reset; assert din=1. Latency = 2 dst ticks (Sim::tick MVP).
    pv.set("rst", 0);
    pv.set("din", 1);
    sim.set_inputs(pv.clone());
    sim.tick(); // dst tick 1: ff0 samples din; dout still 0
    assert_eq!(
        sim.ports().get("dout"),
        Some(0),
        "after 1 dst tick dout must still be 0 (2-flop latency)"
    );
    sim.set_inputs(pv);
    sim.tick(); // dst tick 2: ff1 samples ff0 → dout=1
    assert_eq!(
        sim.ports().get("dout"),
        Some(1),
        "after 2 dst ticks dout must follow din"
    );
}

#[test]
fn fr79_illegal_unsync_cross_domain_still_e0220() {
    let mut s = ElaborateSession::new("CdcIllegal");
    s.begin_module("CdcIllegal", Span::default());
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
    let err = s.finish().expect_err("illegal CDC must fail freeze");
    assert_e0220(&err);
}

#[test]
fn fr79_doubleflop_not_empty_zst_netlist() {
    let hir = DoubleFlop::elaborate().expect("elaborate");
    let has_reg = hir.circuit().modules[0]
        .body
        .iter()
        .any(|st| matches!(st, Stmt::RegDecl { .. }));
    assert!(
        has_reg,
        "NFR37: DoubleFlop must produce RegDecl netlist, not empty ZST"
    );
    // std::mem::size_of::<DoubleFlop>() may still be 0 — that is fine if elaborate emits regs.
    let _marker = DoubleFlop;
    let _ = _marker;
}
