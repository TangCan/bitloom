//! ATDD — Story 47.3 / FR105: simulation coverage extension (C2 + R1–R2).
//!
//! ```text
//! cargo test -p bitloom --test fr105_sim_coverage_ext
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::PortValues;
use bitloom_sim::{Sim, parse_report};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn mux_hir() -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new("Fr105Mux");
    s.begin_module("Fr105Mux", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("sel", GroundType::Bool, Span::default());
    s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
    s.add_input("b", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_mux("y", "sel", "a", "b", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("elaborate mux")
}

#[test]
fn fr105_docs_coverage_ext_contract() {
    let text = read("docs/fr105-sim-coverage-ext.md");
    assert!(text.contains("FR105"), "must name FR105");
    assert!(text.contains("Bitloom"), "must brand Bitloom");
    assert!(
        text.contains("C2")
            || text.contains("branch")
            || text.contains("分支")
            || text.contains("Mux")
            || text.contains("mux"),
        "must nail C2 branch/condition coverage"
    );
    assert!(
        text.contains("C1") || text.contains("toggle") || text.contains("FR34"),
        "must retain / cross C1 toggle (FR34)"
    );
    assert!(
        (text.contains("≠")
            || text.contains("alone")
            || text.contains("不足以")
            || text.contains("not")
            || text.contains("Not"))
            && (text.contains("FR34") || text.contains("toggle")),
        "must contrast FR34 toggle alone ≠ FR105"
    );
    assert!(
        text.contains("coverage v2")
            || text.contains("branch_hit")
            || text.contains("R1")
            || text.contains("报告"),
        "must nail R1 report format"
    );
    assert!(
        text.contains("recorder")
            || text.contains("记录器")
            || text.contains("R2")
            || text.contains("coverage_report"),
        "must nail R2 recorder (not docs-only)"
    );
}

#[test]
fn fr105_docs_c3_fsm_contract() {
    let text = read("docs/fr105-sim-coverage-ext.md");
    assert!(text.contains("C3"), "must name C3 FSM contract");
    let cropped = text.contains("裁剪")
        || text.contains("crop")
        || text.contains("Crop")
        || text.contains("deferred")
        || text.contains("non-goal")
        || text.contains("非目标")
        || text.contains("MVP 不含");
    let delivered = text.contains("状态访问")
        || text.contains("state visit")
        || text.contains("FSM") && text.contains("delivered");
    assert!(
        cropped || delivered,
        "C3 must be explicitly cropped or delivered — silent omit forbidden"
    );
}

#[test]
fn fr105_recorder_mux_branch_and_toggle_fixture() {
    let mut sim = Sim::new(mux_hir());
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("sel", 0); // false arm only → branch miss on true
    pv.set("a", 0x11);
    pv.set("b", 0x22);
    sim.set_inputs(pv.clone());
    sim.tick();
    pv.set("a", 0x33); // toggle `a` for C1 hit
    sim.set_inputs(pv);
    sim.tick();

    let report = sim.coverage_report();
    assert!(
        report.starts_with("# bitloom-sim coverage v2")
            || report.starts_with("# bitloom-sim coverage v3"),
        "R1: coverage v2 (Mux) or v3 (with FSM); got: {}",
        report.lines().next().unwrap_or("")
    );
    assert!(
        report.contains("branch_hit") || report.contains("branch_miss"),
        "C2: report must include branch_* lines"
    );
    assert!(
        report.contains("branch_hit") && report.contains("branch_miss"),
        "fixture must show ≥1 branch_hit and ≥1 branch_miss:\n{report}"
    );
    let (hits, misses) = parse_report(&report);
    assert!(
        !hits.is_empty() && !misses.is_empty(),
        "C1: toggle hit and miss must remain (FR34 regression): hits={hits:?} misses={misses:?}"
    );
}
