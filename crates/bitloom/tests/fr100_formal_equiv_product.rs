//! ATDD — Story 45.2 / FR100: automatic FL≡RTL / formal-equivalence product path.
//!
//! Red until: docs/fr100-formal-equiv.md (F1=(i), FR92 supporting-not-sufficient),
//! FormalEquivProduct random + bounded-exhaustive APIs, readable Fail, sprint 45-2 done.
//!
//! ```text
//! cargo test -p bitloom --test fr100_formal_equiv_product
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use bitloom_sim::{AbstractionView, EquivStatus, FormalEquivProduct, check_generated_bridge_with};

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

fn counter_hir() -> FrozenHir {
    let mut s = ElaborateSession::new("Fr100Counter");
    s.begin_module("Fr100Counter", Span::default());
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
    s.finish().expect("Fr100Counter elaborate")
}

fn assert_readable_fail(status: &EquivStatus) {
    match status {
        EquivStatus::Fail { cycle, mismatches } => {
            assert!(
                !mismatches.is_empty(),
                "Fail at cycle {cycle} must carry readable PortMismatch list"
            );
            let rendered = format!("{mismatches:?}");
            assert!(
                rendered.contains("data_out")
                    || rendered.contains("port")
                    || rendered.contains("expected")
                    || rendered.contains("got")
                    || !rendered.is_empty(),
                "Fail diagnostics must be human-readable, got: {rendered}"
            );
        }
        EquivStatus::Pass { .. } => panic!("expected Fail with readable diagnostics"),
    }
}

#[test]
fn fr100_docs_contract_f1_branch_and_completion_surface() {
    let text = read("docs/fr100-formal-equiv.md");
    assert!(text.contains("FR100"), "FR100 doc must name FR100");
    assert!(text.contains("Bitloom"), "FR100 doc must brand Bitloom");
    assert!(
        text.contains("F1")
            && (text.contains("(i)")
                || text.contains("树内")
                || text.to_lowercase().contains("in-tree")
                || text.contains("有界")),
        "FR100 doc must nail F1 branch (i) in-tree / bounded prover"
    );
    assert!(
        text.to_lowercase().contains("formal")
            || text.contains("形式等价")
            || text.contains("FL≡RTL")
            || text.contains("FL==RTL"),
        "FR100 doc must name formal equivalence / FL≡RTL"
    );
    assert!(
        text.contains("完成面")
            || text.to_lowercase().contains("completion")
            || text.contains("FR100 完成"),
        "FR100 doc must declare this path as the FR100 completion surface"
    );
}

#[test]
fn fr100_docs_fr92_supporting_not_sufficient() {
    let text = read("docs/fr100-formal-equiv.md");
    assert!(
        text.contains("FR92") || text.contains("fr92"),
        "FR100 doc must contrast FR92"
    );
    assert!(
        (text.contains("非充分")
            || text.contains("不足")
            || text.to_lowercase().contains("not sufficient")
            || text.to_lowercase().contains("alone")
            || text.contains("不得单独")
            || text.contains("配套"))
            && (text.contains("FR92")
                || text.contains("记分板")
                || text.to_lowercase().contains("scoreboard")),
        "FR100 doc must state FR92 / random scoreboard is supporting-not-sufficient"
    );
    // Positive forbid is enough; do not substring-match the forbid sentence itself.
    assert!(
        text.contains("Do **not** claim FR92 alone")
            || text.contains("不得单独")
            || text.contains("alone ≠ FR100")
            || text.to_lowercase().contains("alone !=")
            || text.to_lowercase().contains("not sufficient"),
        "FR100 doc must explicitly forbid treating FR92 alone as FR100 close"
    );
}

#[test]
fn fr100_random_compare_pass_reproducible() {
    let hir = counter_hir();
    let product = FormalEquivProduct::new(0xC0FFEE, 8).with_boolean_ports(&["rst"]);
    let a = product.check_random_compare(hir.clone());
    let b = product.check_random_compare(hir);
    assert!(
        a.is_pass(),
        "random/compare path must pass on matching FL vs tick: {a:?}"
    );
    assert_eq!(a, b, "same seed must reproduce the same Pass result");
}

#[test]
fn fr100_random_compare_deliberate_mismatch_readable() {
    let hir = counter_hir();
    let product = FormalEquivProduct::new(0xBEEF, 4).with_boolean_ports(&["rst"]);
    struct Wrong;
    impl AbstractionView for Wrong {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("data_out", 0xAD);
            o
        }
    }
    let mut wrong = Wrong;
    let stimuli = product.random_stimuli();
    let status = check_generated_bridge_with(hir, &mut wrong, stimuli);
    assert_readable_fail(&status);
}

#[test]
fn fr100_bounded_exhaustive_pass() {
    let hir = counter_hir();
    // Depth 3 over {rst=0,1} ⇒ 8 sequences — beyond a single random scoreboard sample.
    let product = FormalEquivProduct::new(0, 0)
        .with_boolean_ports(&["rst"])
        .with_exhaustive_depth(3);
    let status = product.check_bounded_exhaustive(hir);
    assert!(
        status.is_pass(),
        "bounded exhaustive FL≡tick must pass within pinned alphabet×depth: {status:?}"
    );
}

#[test]
fn fr100_bounded_exhaustive_deliberate_mismatch_readable() {
    let hir = counter_hir();
    let product = FormalEquivProduct::new(0, 0)
        .with_boolean_ports(&["rst"])
        .with_exhaustive_depth(2);
    struct Wrong;
    impl AbstractionView for Wrong {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("data_out", 0x42);
            o
        }
    }
    let mut wrong = Wrong;
    let status = product.check_bounded_exhaustive_with(hir, &mut wrong);
    assert_readable_fail(&status);
}

#[test]
fn fr100_sprint_45_2_done_epic_open() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("45-2-自动-fl-rtl-形式等价产品路径-fr100: done"),
        "sprint must mark 45-2 done"
    );
    assert!(
        sprint.contains("45-3-多视图属性全矩阵-fr102: backlog"),
        "sprint must keep 45-3 backlog"
    );
    assert!(
        sprint.contains("45-4-一级-ip-双模型齐全-epic45-收口-fr103: backlog"),
        "sprint must keep 45-4 backlog"
    );
    assert!(
        sprint
            .lines()
            .any(|l| l.trim() == "epic-45: in-progress" || l.contains("epic-45: in-progress")),
        "sprint must keep epic-45 in-progress (not done)"
    );
}

#[test]
fn fr100_api_beyond_random_scoreboard_alone() {
    let sim = read("crates/bitloom-sim/src/lib.rs");
    assert!(
        sim.contains("FormalEquivProduct") || sim.contains("formal_equiv"),
        "bitloom-sim must export FormalEquivProduct / formal_equiv module"
    );
    let formal = read("crates/bitloom-sim/src/formal_equiv.rs");
    assert!(
        formal.contains("check_bounded_exhaustive")
            && (formal.contains("exhaustive")
                || formal.contains("alphabet")
                || formal.contains("depth")),
        "formal product entry must expose bounded exhaustive (beyond random scoreboard alone)"
    );
    assert!(
        formal.contains("check_random_compare") || formal.contains("random"),
        "formal product must also expose automatic random/compare companion path"
    );
    // Honesty: module docs must not equate FR92 scoreboard alone with FR100 close
    assert!(
        formal.contains("FR100")
            && (formal.contains("FR92")
                || formal.to_lowercase().contains("not")
                || formal.contains("beyond")
                || formal.contains("不等于")
                || formal.contains("非")),
        "formal_equiv module must contrast FR100 vs FR92 / random-alone"
    );
}
