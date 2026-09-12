//! ATDD Story 89.2 / FR157 — automatic FSM label extraction.
//!
//! ```text
//! cargo test -p bitloom --test fr157_auto_fsm_labels
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom::fsm_labels::{FsmExtractError, extract_fsm_labels_from_source};
use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use bitloom_prelude::FsmLabels;
use bitloom_sim::{Sim, parse_state_report};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[allow(dead_code)]
#[bitloom_prelude::bitloom::fsm(name = "demo")]
enum DemoFsm {
    Idle,
    Busy,
    Done,
}

fn fixture_hir() -> FrozenHir {
    let mut s = ElaborateSession::new("Fr157FsmFixture");
    s.begin_module("Fr157FsmFixture", Span::default());
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
fn fr157_docs_contract() {
    let text = read("docs/fr157-auto-fsm-labels.md");
    assert!(text.contains("FR157") && text.contains("Bitloom"));
    assert!(
        text.contains("bitloom::fsm") || text.contains("#[bitloom::fsm]"),
        "must document #[bitloom::fsm]"
    );
    assert!(
        text.contains("FsmLabels") && text.contains("register_fsm_states"),
        "must document FsmLabels → FR109 register path"
    );
    assert!(
        text.contains("FR109")
            && (text.contains("≠") || text.contains("alone") || text.contains("不")),
        "must contrast FR109 alone ≠ FR157"
    );
    assert!(
        (text.contains("波形")
            || text.contains("Waveform")
            || text.contains("unlabeled")
            || text.contains("无标注")
            || text.contains("LCOV")
            || text.contains("GUI"))
            && (text.contains("Non-goals") || text.contains("NFR71") || text.contains("不得")),
        "must list NFR71 non-goals"
    );
}

#[test]
fn fr157_compile_time_fsm_labels_feed_fr109() {
    assert_eq!(DemoFsm::FSM_ID, "demo");
    assert_eq!(DemoFsm::state_labels(), &["Idle", "Busy", "Done"]);

    let mut sim = Sim::new(fixture_hir());
    sim.register_fsm_states(DemoFsm::FSM_ID, DemoFsm::state_labels().iter().copied());

    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("data_in", 0);
    sim.set_inputs(pv);
    sim.tick();
    sim.sample_state_visit(DemoFsm::FSM_ID, "Idle");
    sim.tick();
    sim.sample_state_visit(DemoFsm::FSM_ID, "Busy");

    let report = sim.coverage_report();
    assert!(
        report.contains("# bitloom-sim coverage v3") && report.contains("FR109"),
        "FR109 dialect must remain: {report}"
    );
    let (hits, misses) = parse_state_report(&report);
    assert!(
        hits.iter().any(|h| h == "fsm:demo:Idle") || hits.iter().any(|h| h == "fsm:demo:Busy"),
        "expected state_hit from FR157-fed labels: hits={hits:?}\n{report}"
    );
    assert!(
        misses.iter().any(|m| m == "fsm:demo:Done"),
        "unvisited Done must be state_miss: misses={misses:?}\n{report}"
    );
}

#[test]
fn fr157_offline_extract_from_source() {
    let src = r#"
        #[bitloom::fsm(name = "demo")]
        enum Demo { Idle, Busy, Done }
    "#;
    let sets = extract_fsm_labels_from_source(src).expect("extract");
    assert_eq!(sets.len(), 1);
    assert_eq!(sets[0].id, "demo");
    assert_eq!(sets[0].labels, vec!["Idle", "Busy", "Done"]);
    let lines = sets[0].to_registry_lines();
    assert_eq!(
        lines,
        vec![
            "fsm:demo:Idle".to_string(),
            "fsm:demo:Busy".to_string(),
            "fsm:demo:Done".to_string()
        ]
    );
}

#[test]
fn fr157_offline_extract_default_id_and_rhdl_alias() {
    let src = r#"
        #[rhdl::fsm]
        enum Traffic { Red, Green }
    "#;
    let sets = extract_fsm_labels_from_source(src).expect("extract");
    assert_eq!(sets[0].id, "Traffic");
    assert_eq!(sets[0].labels, vec!["Red", "Green"]);
}

#[test]
fn fr157_offline_extract_failures() {
    let missing = extract_fsm_labels_from_source("enum X { A }");
    assert!(matches!(missing, Err(FsmExtractError::NoAnnotatedFsm)));

    let empty = extract_fsm_labels_from_source("#[bitloom::fsm] enum Empty {}");
    assert!(matches!(empty, Err(FsmExtractError::EmptyVariants { .. })));

    let non_unit = extract_fsm_labels_from_source("#[bitloom::fsm] enum Bad { A(u8) }");
    assert!(matches!(
        non_unit,
        Err(FsmExtractError::NonUnitVariant { .. })
    ));
}
