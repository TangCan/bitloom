//! ATDD (red→green): Story 51.2 / FR109 — FSM state-visit coverage (M1–M4).
//!
//! Beyond FR105 Mux branch v2: register FSM labels, sample visits, emit
//! `state_hit` / `state_miss` under coverage v3 + FR109 marker.
//!
//! ```text
//! cargo test -p bitloom --test fr109_fsm_state_visit_coverage
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use bitloom_sim::{Sim, parse_branch_report, parse_state_report};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

fn fixture_hir() -> FrozenHir {
    let mut s = ElaborateSession::new("Fr109FsmFixture");
    s.begin_module("Fr109FsmFixture", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "count", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("count", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("elaborate")
}

#[test]
fn fr109_docs_contract() {
    let text = read("docs/fr109-fsm-state-visit-coverage.md");
    assert!(text.contains("FR109") && text.contains("Bitloom"));
    assert!(
        text.contains("state_hit") && text.contains("state_miss"),
        "M2: state_hit/miss dialect"
    );
    assert!(
        text.contains("FSM") || text.contains("state-visit") || text.contains("C3"),
        "M1: FSM / C3"
    );
    assert!(
        (text.contains("Mux") || text.contains("FR105") || text.contains("FR34"))
            && (text.contains("≠") || text.contains("alone") || text.contains("不得")),
        "must contrast Mux v2 / FR34 alone ≠ FR109"
    );
    assert!(
        text.contains("NFR47") || text.contains("Non-goals") || text.contains("Tywaves"),
        "NFR47 non-goals"
    );
}

#[test]
fn fr109_fsm_state_visit_fixture() {
    let mut sim = Sim::new(fixture_hir());
    sim.register_fsm_states("demo", ["Idle", "Busy", "Done"]);

    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("data_in", 0);
    sim.set_inputs(pv);
    sim.tick();
    sim.sample_state_visit("demo", "Idle");
    sim.tick();
    sim.sample_state_visit("demo", "Busy");
    // Done intentionally never visited → state_miss

    let report = sim.coverage_report();
    assert!(
        report.contains("# bitloom-sim coverage v3"),
        "M2: v3 header when FSM registered"
    );
    assert!(
        report.contains("FR109") && (report.contains("C3") || report.contains("FSM/state-visit")),
        "M2: FR109 / C3 family marker"
    );
    let (hits, misses) = parse_state_report(&report);
    assert!(
        hits.iter().any(|h| h == "fsm:demo:Idle") && hits.iter().any(|h| h == "fsm:demo:Busy"),
        "M1/M3: Idle+Busy hits; hits={hits:?}"
    );
    assert!(
        misses.iter().any(|m| m == "fsm:demo:Done"),
        "M4: readable Done miss; misses={misses:?}"
    );
    assert!(
        report.contains("state_hit ") && report.contains("state_miss "),
        "M4: both hit and miss lines present"
    );
}

#[test]
fn fr109_does_not_claim_mux_v2_alone_is_c3() {
    let mut sim = Sim::new(fixture_hir());
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("data_in", 0);
    sim.set_inputs(pv);
    sim.tick();
    let report = sim.coverage_report();
    assert!(
        report.starts_with("# bitloom-sim coverage v2"),
        "toggle-only path stays v2"
    );
    assert!(
        !report.contains("state_hit") && !report.contains("FR109"),
        "without FSM registration, must not emit C3/FR109 section"
    );
    let _ = parse_branch_report(&report);
}

#[test]
fn fr109_nfr44_fr105_mux_path_still_green() {
    let text = read("docs/fr105-sim-coverage-ext.md");
    assert!(
        text.contains("branch_hit") || text.contains("C2") || text.contains("Mux"),
        "FR105 Mux v2 docs remain"
    );
}
