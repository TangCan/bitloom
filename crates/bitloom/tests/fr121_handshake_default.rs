//! ATDD (red→green): Story 62.2 / FR121 — Handshake default synthesizable path.
//!
//! ```text
//! cargo test -p bitloom --test fr121_handshake_default
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom::hls::{
    HlsDataflowClosureViolation, HlsDataflowOp, InTreeScheduleKind, meets_fr110_commercial_depth,
    meets_fr121_handshake, schedule_handshake_default, schedule_handshake_from_transform,
    schedule_in_tree, schedule_in_tree_fr110,
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
fn fr121_ad25_revised_allows_handshake_default() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    let ad25 = spine
        .split("### AD-25")
        .nth(1)
        .and_then(|s| s.split("### AD-26").next())
        .expect("AD-25 section");
    assert!(
        ad25.contains("FR121")
            && (ad25.contains("Handshake") || ad25.contains("handshake") || ad25.contains("动态")),
        "AD-25 must cite FR121 Handshake"
    );
    assert!(
        ad25.contains("Revised") && ad25.contains("2026-09-10") && ad25.contains("FR121"),
        "AD-25 must carry FR121 Revised stamp"
    );
    assert!(
        ad25.contains("允许")
            && (ad25.contains("Handshake") || ad25.contains("动态数据流"))
            && (ad25.contains("默认可综合") || ad25.contains("默认")),
        "AD-25 Rule must allow Handshake/dynamic DF as default synthesizable"
    );
    assert!(
        !ad25.contains("仍禁止 Handshake/动态数据流作为默认可综合"),
        "AD-25 must not retain old forbid-Handshake-default rule clause"
    );
}

#[test]
fn fr121_docs_contract() {
    let text = read("docs/fr121-handshake-default.md");
    assert!(text.contains("FR121") && text.contains("Bitloom"));
    assert!(
        text.contains("Handshake") || text.contains("handshake"),
        "must name Handshake"
    );
    assert!(
        text.contains("ready") && text.contains("valid"),
        "must document ready/valid"
    );
    assert!(
        text.contains("AD-18") || text.contains("dissolve") || text.contains("溶解"),
        "must cite AD-18 dissolve"
    );
    assert!(
        (text.contains("FR95") || text.contains("FR110"))
            && (text.contains("≠") || text.contains("alone") || text.contains("不得")),
        "must contrast FR95/FR110 alone ≠ FR121"
    );
    assert!(
        text.contains("AD-25") && (text.contains("修订") || text.contains("Revised")),
        "must cite AD-25 revise"
    );
}

#[test]
fn fr121_handshake_schedule_emits_ready_valid() {
    let art = schedule_handshake_default("hs_add1", HlsDataflowOp::AddConst(1), 1)
        .expect("FR121 handshake");
    assert!(meets_fr121_handshake(&art.kind));
    let ir = &art.schedule_ir;
    assert!(
        ir.contains("\"fr121\": true") || ir.contains("\"fr121\":true"),
        "IR must mark fr121: {ir}"
    );
    assert!(
        ir.contains("\"handshake\": true") || ir.contains("\"handshake\":true"),
        "IR must mark handshake: {ir}"
    );
    assert!(
        ir.contains("handshake-dynamic-df") || ir.contains("\"semantics\""),
        "IR must declare handshake-dynamic-df semantics: {ir}"
    );
    let rtl = &art.rtl_stub;
    assert!(
        rtl.contains("valid") && rtl.contains("ready"),
        "RTL must expose ready/valid ports: {rtl}"
    );
    assert!(
        rtl.to_lowercase().contains("handshake") || rtl.contains("FR121"),
        "RTL must honestly label Handshake/FR121"
    );
    assert!(!ir.to_lowercase().contains("bambu"));
}

#[test]
fn fr121_dissolve_then_handshake_ad18() {
    let art = schedule_handshake_from_transform("hs_xor", &[], 1, || HlsDataflowOp::XorConst(0xa5))
        .expect("dissolve → handshake");
    assert!(meets_fr121_handshake(&art.kind));
    assert!(
        art.schedule_ir.contains("fr96")
            || art.rtl_stub.contains("FR96")
            || art.rtl_stub.contains("dissolv"),
        "must show dissolve before handshake schedule"
    );
    assert!(art.schedule_ir.contains("fr121"));
}

#[test]
fn fr121_capturing_fails_before_schedule() {
    let err = schedule_handshake_from_transform(
        "bad_hs",
        &[HlsDataflowClosureViolation::capturing("holds &mut state")],
        1,
        || HlsDataflowOp::Identity,
    )
    .expect_err("capturing must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("Capturing") || msg.contains("rejected") || msg.contains("AD-18"),
        "readable capturing failure: {msg}"
    );
}

#[test]
fn fr121_rejects_zero_channels_not_silent() {
    let err = schedule_handshake_default("empty_ch", HlsDataflowOp::Identity, 0)
        .expect_err("channels=0 must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("FR121") && (msg.contains("channel") || msg.contains("通道")),
        "readable FR121 failure: {msg}"
    );
}

#[test]
fn fr121_mvp_and_fr110_still_work_nfr48() {
    let mvp = schedule_in_tree(
        "map_add1",
        HlsDataflowOp::AddConst(1),
        InTreeScheduleKind::LoopUnroll { trip_count: 4 },
    )
    .expect("FR95 MVP still works");
    assert!(!meets_fr121_handshake(&mvp.kind));
    assert!(
        !mvp.schedule_ir.contains("fr121"),
        "loop-unroll must not claim fr121"
    );

    let fr110 = schedule_in_tree_fr110("pipe_add1", HlsDataflowOp::AddConst(1), 1, 2)
        .expect("FR110 still works");
    assert!(meets_fr110_commercial_depth(&fr110.kind));
    assert!(!meets_fr121_handshake(&fr110.kind));
    assert!(
        !fr110.schedule_ir.contains("fr121"),
        "FR110 pipeline must not silent-claim fr121"
    );
}

#[test]
fn fr121_nfr14_h_checklist_present() {
    let nfr = read("_agile-output/implementation-artifacts/nfr14-risk-epic62-handshake-default.md");
    for h in ["H1", "H2", "H3", "H4"] {
        assert!(nfr.contains(h), "NFR14 must retain {h}");
    }
}
