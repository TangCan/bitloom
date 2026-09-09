//! ATDD: Story 31.4 CDC depth docs close-out / UJ「CDC 深度」(FR79 / NFR37).
//!
//! Verifies follow-along docs, FR52 vs FR79 narrative, NFR14 Epic 31 close
//! checklist, and a testable「仅 ZST / mark_cdc_bridge」contrast against true
//! RTL elaborates — without inventing new prelude APIs.
//!
//! ```text
//! cargo test -p bitloom --test fr79_cdc_depth_closeout
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_hir::Stmt;
use bitloom_prelude::{DoubleFlop, Elaboratable, ElaborateSession, GroundType, Span, SyncFIFO};
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn fr79_cdc_depth_tutorial_names_fixtures_and_recipe() {
    let root = workspace_root();
    let tutorial = fs::read_to_string(root.join("docs/tutorials/cdc-depth.md"))
        .expect("docs/tutorials/cdc-depth.md must exist for UJ CDC 深度");

    assert!(
        tutorial.contains("CDC 深度") || tutorial.contains("cdc-depth"),
        "tutorial must identify CDC 深度"
    );
    assert!(
        tutorial.contains("fr79_doubleflop_rtl"),
        "tutorial must point at DoubleFlop golden ATDD"
    );
    assert!(
        tutorial.contains("fr79_syncfifo_rtl"),
        "tutorial must point at SyncFIFO golden ATDD"
    );
    assert!(
        tutorial.contains("E0220") || tutorial.contains("rhdl::E0220"),
        "tutorial must name illegal CDC negative"
    );
    assert!(
        tutorial.contains("just test") || tutorial.contains("cargo test --workspace"),
        "tutorial must document contributor recipe (just test / workspace)"
    );
    assert!(
        tutorial.contains("NFR37") && (tutorial.contains("FR52") || tutorial.contains("最小")),
        "tutorial must contrast FR79 depth vs FR52 minimal (NFR37)"
    );

    assert!(
        root.join("crates/bitloom/tests/fr79_doubleflop_rtl.rs")
            .is_file(),
        "DoubleFlop golden fixture must exist on disk"
    );
    assert!(
        root.join("crates/bitloom/tests/fr79_syncfifo_rtl.rs")
            .is_file(),
        "SyncFIFO golden fixture must exist on disk"
    );
    assert!(
        root.join("examples/clockdomain_skel/src/lib.rs").is_file(),
        "FR52 contrast fixture must exist on disk"
    );
}

#[test]
fn fr79_docs_and_readme_true_rtl_vs_historical_minimal() {
    let root = workspace_root();

    let df = fs::read_to_string(root.join("docs/fr79-doubleflop-cdc.md")).expect("fr79 df");
    let sf = fs::read_to_string(root.join("docs/fr79-syncfifo-cdc.md")).expect("fr79 sf");
    assert!(
        df.contains("NFR37") && df.contains("FR52") && df.contains("真 RTL"),
        "DoubleFlop doc must state true RTL vs FR52 / NFR37"
    );
    assert!(
        sf.contains("NFR37") && sf.contains("FR52") && sf.contains("真 RTL"),
        "SyncFIFO doc must state true RTL vs FR52 / NFR37"
    );
    assert!(
        df.contains("tutorials/cdc-depth.md") || df.contains("CDC 深度"),
        "fr79 DoubleFlop doc must link CDC 深度 follow-along"
    );
    assert!(
        sf.contains("tutorials/cdc-depth.md") || sf.contains("CDC 深度"),
        "fr79 SyncFIFO doc must link CDC 深度 follow-along"
    );

    let readme = fs::read_to_string(root.join("README.md")).expect("README");
    assert!(
        readme.contains("FR79")
            && (readme.contains("cdc-depth.md") || readme.contains("doubleflop_skel")),
        "README must index FR79 CDC depth (tutorial and/or true-RTL skels)"
    );
    assert!(
        readme.contains("FR52") && readme.contains("clockdomain_skel"),
        "README must keep FR52 clockdomain_skel as historical minimal"
    );

    let surface =
        fs::read_to_string(root.join("_agile-output/specs/spec-rhdl/language-surface.md"))
            .expect("language-surface");
    assert!(
        surface.contains("31.4") || surface.contains("fr79_cdc_depth_closeout"),
        "language-surface must mark Story 31.4 docs close-out"
    );
}

#[test]
fn fr79_nfr14_epic31_close_checklist_ticked() {
    let root = workspace_root();
    let nfr14 = fs::read_to_string(
        root.join("_agile-output/implementation-artifacts/nfr14-risk-epic31-cdc-true-rtl.md"),
    )
    .expect("nfr14 epic31");

    for needle in [
        "- [x] **FR79 DoubleFlop：",
        "- [x] **FR79 SyncFIFO",
        "- [x] **负例保留：",
        "- [x] **NFR37：",
        "- [x] **ATDD / 配方：",
        "- [x] **禁止事项未触发：",
    ] {
        assert!(
            nfr14.contains(needle),
            "NFR14 Epic 31 close checklist missing tick for: {needle}"
        );
    }
}

/// FR52-shaped legal CDC: `mark_cdc_bridge` only — no synchronizer RTL.
fn elaborate_fr52_bridge_only() -> bitloom_prelude::FrozenHir {
    let mut s = ElaborateSession::new("Fr52BridgeOnly");
    s.begin_module("Fr52BridgeOnly", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.bind_domain("a", 0);
    s.bind_domain("y", 1);
    s.mark_cdc_bridge("y");
    s.begin_combinational(Span::default());
    s.assign_net("y", "a", Span::default());
    s.end_process();
    s.end_module();
    s.finish()
        .expect("FR52 mark_cdc_bridge path must elaborate")
}

#[test]
fn fr79_contrast_true_rtl_vs_fr52_zst_bridge_emit() {
    // Marker types may still be ZST; depth is judged by netlist, not size_of alone.
    assert_eq!(
        std::mem::size_of::<DoubleFlop>(),
        0,
        "DoubleFlop marker type remains ZST; contrast is emit netlist"
    );
    assert_eq!(
        std::mem::size_of::<SyncFIFO<4, 8>>(),
        0,
        "SyncFIFO marker type remains ZST; contrast is emit netlist"
    );

    let df = DoubleFlop::elaborate().expect("DoubleFlop true RTL");
    let sf = SyncFIFO::<4, 8>::elaborate().expect("SyncFIFO true RTL");
    let fr52 = elaborate_fr52_bridge_only();

    let df_regs: Vec<&str> = df.circuit().modules[0]
        .body
        .iter()
        .filter_map(|st| match st {
            Stmt::RegDecl { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    assert!(
        df_regs.iter().any(|n| n.contains("ff0")) && df_regs.iter().any(|n| n.contains("ff1")),
        "FR79 DoubleFlop must emit two sync stages, got {df_regs:?}"
    );

    let sf_has_mem = sf.circuit().modules[0]
        .body
        .iter()
        .any(|st| matches!(st, Stmt::MemDecl { .. }));
    assert!(
        sf_has_mem,
        "FR79 SyncFIFO must emit FIFO mem (not empty ZST)"
    );

    let fr52_v = &emit(&fr52).files[0].contents;
    assert!(
        !fr52_v.contains("sync_ff0") && !fr52_v.contains("sync_ff1"),
        "FR52 mark_cdc_bridge path must not emit sync_ff* (仅 ZST/bridge): {fr52_v}"
    );

    let df_v = &emit(&df).files[0].contents;
    assert!(
        df_v.contains("sync_ff0") && df_v.contains("sync_ff1"),
        "FR79 DoubleFlop emit must show sync_ff*: {df_v}"
    );
}
