//! ATDD (red→green): Story 52.2 / FR110 — in-tree HLS commercial depth (Q1+Q2).
//!
//! Beyond FR95 loop-unroll MVP: multi-stage pipeline + `ii` + `fr110` marker.
//!
//! ```text
//! cargo test -p bitloom --test fr110_hls_commercial_depth
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom::hls::{
    HlsDataflowOp, InTreeScheduleKind, meets_fr110_commercial_depth, schedule_in_tree,
    schedule_in_tree_fr110,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr110_docs_contract() {
    let text = read("docs/fr110-hls-commercial-depth.md");
    assert!(text.contains("FR110") && text.contains("Bitloom"));
    assert!(
        text.contains("pipeline_stages") || text.contains("Q1"),
        "Q1 pipeline stages"
    );
    assert!(text.contains("ii") || text.contains("Q2"), "Q2 ii");
    assert!(
        (text.contains("FR95") || text.contains("MVP") || text.contains("unroll"))
            && (text.contains("≠") || text.contains("alone") || text.contains("不得")),
        "must contrast FR95 MVP alone ≠ FR110"
    );
    assert!(
        text.contains("Bambu") && (text.contains("≠") || text.contains("alone")),
        "must ban Bambu alone"
    );
}

#[test]
fn fr110_pipeline_meets_q1_q2() {
    let art = schedule_in_tree_fr110("pipe_add1", HlsDataflowOp::AddConst(1), 1, 2)
        .expect("FR110 pipeline stages=2");
    assert!(meets_fr110_commercial_depth(&art.kind));
    assert_eq!(art.stages.len(), 2);
    let ir = &art.schedule_ir;
    assert!(ir.contains("\"fr110\": true") || ir.contains("\"fr110\":true"));
    assert!(ir.contains("\"ii\": 1") || ir.contains("\"ii\":1"));
    assert!(
        ir.contains("\"pipeline_stages\": 2") || ir.contains("\"pipeline_stages\":2"),
        "Q1: {ir}"
    );
    assert!(ir.contains("pipeline"));
    assert!(!ir.to_lowercase().contains("bambu"));
}

#[test]
fn fr110_rejects_shallow_pipeline_not_silent() {
    let err = schedule_in_tree_fr110("shallow", HlsDataflowOp::Identity, 1, 1)
        .expect_err("stages=1 must fail FR110 gate");
    let msg = err.to_string();
    assert!(
        msg.contains("FR110") && (msg.contains("pipeline_stages") || msg.contains("stages")),
        "readable FR110 failure: {msg}"
    );
    assert!(!meets_fr110_commercial_depth(
        &InTreeScheduleKind::Pipeline {
            initiation_interval: 1,
            stages: 1,
        }
    ));
}

#[test]
fn fr110_mvp_loop_unroll_is_not_commercial_depth() {
    let art = schedule_in_tree(
        "map_add1",
        HlsDataflowOp::AddConst(1),
        InTreeScheduleKind::LoopUnroll { trip_count: 4 },
    )
    .expect("FR95 MVP still works");
    assert!(!meets_fr110_commercial_depth(&art.kind));
    assert!(
        !art.schedule_ir.contains("fr110"),
        "loop-unroll must not claim fr110"
    );
}

#[test]
fn fr110_nfr44_fr95_docs_still_present() {
    let text = read("docs/fr35-hls.md");
    assert!(
        text.contains("FR95") && (text.contains("in-tree") || text.contains("loop-unroll")),
        "FR95 MVP docs remain"
    );
}
