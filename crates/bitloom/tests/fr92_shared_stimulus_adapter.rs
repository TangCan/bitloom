//! ATDD — Story 39.4 / FR92: multi-view shared stimulus + adapter template.
//!
//! Red until: FR92 docs, SharedStimulusScoreboard fixture (functional ≡ tick on
//! the same stimuli), adapter template cross-links, AD-5 non-claims, and Epic 39
//! NFR14 / sprint closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr92_shared_stimulus_adapter
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use bitloom_sim::{
    AbstractionView, SharedStimulusScoreboard, check_generated_bridge_with, reset_then_run,
};

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
    let mut s = ElaborateSession::new("Fr92Counter");
    s.begin_module("Fr92Counter", Span::default());
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
    s.finish().expect("Fr92Counter elaborate")
}

#[test]
fn fr92_docs_contract_shared_stimulus_and_adapter() {
    let text = read("docs/fr92-shared-stimulus-adapter.md");
    assert!(text.contains("FR92"), "FR92 doc must name FR92");
    assert!(
        text.contains("同刺激")
            || text.contains("共享刺激")
            || text.to_lowercase().contains("shared stimulus"),
        "FR92 doc must require shared stimulus"
    );
    assert!(
        text.contains("adapter")
            || text.contains("Adapter")
            || text.contains("桥接")
            || text.contains("start_wait_complete"),
        "FR92 doc must name bridge adapter template"
    );
    assert!(
        text.contains("Scoreboard")
            || text.contains("记分板")
            || text.contains("SharedStimulusScoreboard"),
        "FR92 doc must name scoreboard / SharedStimulusScoreboard"
    );
    assert!(text.contains("Bitloom"), "FR92 doc must brand Bitloom");
}

#[test]
fn fr92_docs_forbid_auto_equiv_and_systemc_tlm() {
    let text = read("docs/fr92-shared-stimulus-adapter.md");
    let lower = text.to_lowercase();
    assert!(
        (text.contains("不")
            || text.contains("not")
            || text.contains("NOT")
            || text.contains("禁"))
            && (lower.contains("systemc") || text.contains("TLM") || text.contains("TLM-2.0")),
        "FR92 doc must forbid SystemC TLM-2.0 product claim"
    );
    assert!(
        (text.contains("不")
            || text.contains("not")
            || text.contains("NOT")
            || text.contains("禁"))
            && (text.contains("FL≡RTL")
                || text.contains("FL==RTL")
                || lower.contains("formal")
                || text.contains("形式")
                || text.contains("自动等价")),
        "FR92 doc must forbid automatic formal FL≡RTL product claim"
    );
    assert!(
        text.contains("AD-5") || text.contains("AD5"),
        "FR92 doc must cite AD-5"
    );
}

#[test]
fn fr92_shared_scoreboard_functional_matches_tick() {
    let hir = counter_hir();
    let board = SharedStimulusScoreboard::from_stimuli(reset_then_run(4));
    assert!(
        board.stimuli.len() >= 2,
        "scoreboard must hold shared stimulus vectors"
    );
    assert!(
        board.check_generated(hir).is_pass(),
        "same stimuli must pass functional-sim path vs cycle-accurate tick (FR92)"
    );
}

#[test]
fn fr92_shared_scoreboard_deliberate_mismatch_fails() {
    let hir = counter_hir();
    let board = SharedStimulusScoreboard::from_stimuli(reset_then_run(2));
    struct Wrong;
    impl AbstractionView for Wrong {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("data_out", 0xEE);
            o
        }
    }
    let mut wrong = Wrong;
    assert!(
        !check_generated_bridge_with(hir, &mut wrong, board.stimuli.clone()).is_pass(),
        "deliberate mismatch on shared stimuli must fail"
    );
}

#[test]
fn fr92_adapter_template_cross_links_fr78() {
    let fr92 = read("docs/fr92-shared-stimulus-adapter.md");
    assert!(
        fr92.contains("FR78") || fr92.contains("fr78") || fr92.contains("start_wait_complete"),
        "FR92 doc must cross-link FR78 / start_wait_complete adapter template"
    );
    let fr78 = read("docs/fr78-bridge-adapter-closures.md");
    assert!(
        fr78.contains("FR92") || fr78.contains("fr92"),
        "FR78 doc must cross-link FR92 (Wave D shared-stimulus contract)"
    );
}

#[test]
fn fr92_nfr14_and_sprint_epic39_closed() {
    let nfr14 = read("_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md");
    for needle in ["- [x] **FR90：**", "- [x] **FR91：**", "- [x] **FR92：**"] {
        assert!(
            nfr14.contains(needle) || nfr14.contains(&needle.replace("：**", ":**")),
            "NFR14 must tick close checkbox matching {needle}"
        );
    }
    assert!(
        nfr14.contains("- [x] **HTML ≠ LSP：**")
            || nfr14.contains("- [x] **HTML ≠ LSP:**")
            || (nfr14.contains("[x]") && nfr14.contains("HTML") && nfr14.contains("LSP")),
        "NFR14 must tick HTML ≠ LSP close item"
    );
    assert!(
        nfr14.contains("- [x] **禁止事项未触发：**")
            || nfr14.contains("- [x] **禁止事项未触发:**")
            || (nfr14.contains("[x]") && nfr14.contains("禁止事项")),
        "NFR14 must tick forbid-list close item"
    );
    assert!(
        nfr14.contains("- [x] **品牌 / 依赖：**")
            || nfr14.contains("- [x] **品牌 / 依赖:**")
            || (nfr14.contains("[x]")
                && nfr14.contains("Bitloom")
                && nfr14.contains("bitloom-prelude")),
        "NFR14 must tick brand/deps close item"
    );

    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("39-4-多视图同刺激-adapter-模板-fr92: done")
            || sprint.contains("39-4-多视图同刺激与-adapter-模板-fr92: done"),
        "sprint must mark 39-4 done"
    );
    assert!(
        sprint
            .lines()
            .any(|l| l.trim() == "epic-39: done" || l.contains("epic-39: done")),
        "sprint must mark epic-39 done"
    );
}

#[test]
fn fr92_no_second_sim_semantics_or_tlm_product() {
    let fr92 = read("docs/fr92-shared-stimulus-adapter.md");
    assert!(
        fr92.contains("FR47")
            || fr92.contains("fr47")
            || fr92.contains("check_functional_equiv")
            || fr92.contains("SharedStimulusScoreboard"),
        "FR92 must reuse FR47 / shared scoreboard path — not invent a second sim semantics"
    );
    // Mentions of emit_tlm / SystemC must appear only as forbidden / non-goal.
    if fr92.to_lowercase().contains("emit_tlm") {
        let lower = fr92.to_lowercase();
        assert!(
            fr92.contains("不")
                || lower.contains("not")
                || lower.contains("non-goal")
                || lower.contains("forbid")
                || fr92.contains("禁")
                || fr92.contains("非目标"),
            "FR92 must not present emit_tlm as a product deliverable"
        );
    }
    // Spot-check: FR92 must not be the home of SystemC TLM product API.
    // FR101 (Epic 46) may export `systemc_tlm` / `emit_systemc_tlm_lt` — that is a
    // different FR; FR92 docs must still forbid treating shared-stimulus as TLM.
    let sim = read("crates/bitloom-sim/src/lib.rs");
    if sim.contains("mod systemc_tlm") || sim.contains("emit_systemc_tlm") {
        let fr101 = read("docs/fr101-systemc-tlm.md");
        assert!(
            fr101.contains("FR101") && fr101.contains("Bitloom"),
            "SystemC TLM product API must be owned by FR101 docs, not FR92"
        );
        assert!(
            (fr92.contains("不") && (fr92.contains("SystemC") || fr92.contains("TLM")))
                || (fr92.to_lowercase().contains("not")
                    && (fr92.contains("SystemC") || fr92.contains("TLM"))),
            "FR92 must still forbid SystemC TLM as its own contract when FR101 exists"
        );
    } else {
        assert!(
            !sim.contains("pub fn emit_tlm")
                && !sim.contains("fn emit_tlm")
                && !sim.contains("pub mod systemc"),
            "bitloom-sim must not grow unnamed SystemC TLM product API under FR92"
        );
    }
}
