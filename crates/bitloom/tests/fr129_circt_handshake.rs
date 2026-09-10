//! ATDD Story 69.2 / FR129 — CIRCT Handshake + multi-clock elastic buffers.
//!
//! ```text
//! cargo test -p bitloom --test fr129_circt_handshake
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom::hls::{
    HlsDataflowOp, InTreeScheduleKind, meets_fr121_handshake, meets_fr129_circt_handshake,
    schedule_circt_handshake, schedule_handshake_default,
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn fr129_ad25_revised() {
    let spine = fs::read_to_string(
        root().join(
            "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
        ),
    )
    .unwrap();
    assert!(spine.contains("FR129") && spine.contains("AD-25"));
    assert!(spine.contains("CIRCT") && (spine.contains("elastic") || spine.contains("弹性")));
    assert!(
        spine.contains("Phase 15 / **FR129**")
            || spine.contains("Phase 15 / FR129")
            || (spine.contains("FR129") && spine.contains("NFR54") && spine.contains("2026-09-10"))
    );
}

#[test]
fn fr129_docs_contract() {
    let docs = fs::read_to_string(root().join("docs/fr129-circt-handshake.md")).unwrap();
    assert!(docs.contains("FR129") && docs.contains("Bitloom"));
    assert!(docs.contains("C1") && docs.contains("C4"));
    assert!(
        docs.contains("FR121")
            && (docs.contains("beyond") || docs.contains("Beyond") || docs.contains("alone"))
    );
    assert!(docs.contains("AD-25"));
    assert!(docs.contains("handshake.func") && docs.contains("handshake.buffer"));
    assert!(docs.contains("schedule_circt_handshake") && docs.contains("--circt-handshake"));
}

#[test]
fn fr129_schedule_emits_circt_markers() {
    let art = schedule_circt_handshake("hs_circt", HlsDataflowOp::AddConst(1), 1, 2, 2)
        .expect("FR129 schedule");
    assert!(meets_fr129_circt_handshake(&art.kind));
    let ir = &art.schedule_ir;
    assert!(ir.contains("\"fr129\": true") || ir.contains("\"fr129\":true"));
    assert!(ir.contains("circt") && ir.contains("handshake.func"));
    assert!(ir.contains("handshake.buffer") || ir.contains("elastic_depth"));
    assert!(ir.contains("clock_domains"));
    assert!(
        ir.contains("elastic_buffers") || ir.contains("elastic_depth"),
        "IR must declare elastic: {ir}"
    );
    let rtl = &art.rtl_stub;
    assert!(
        (rtl.contains("clk0") && rtl.contains("clk1"))
            || (rtl.contains("clk_a") && rtl.contains("clk_b")),
        "RTL must expose ≥2 clocks: {rtl}"
    );
    assert!(rtl.contains("FR129") || rtl.contains("elastic") || rtl.contains("handshake.buffer"));
}

#[test]
fn fr129_rejects_fr121_alone_shape() {
    let err = schedule_circt_handshake("bad", HlsDataflowOp::Identity, 1, 1, 1)
        .expect_err("clock_domains=1 ≠ FR129");
    let msg = format!("{err}");
    assert!(msg.contains("FR129") && msg.contains("clock_domains"));
    let hs = schedule_handshake_default("hs", HlsDataflowOp::Identity, 1).unwrap();
    assert!(meets_fr121_handshake(&hs.kind));
    assert!(!meets_fr129_circt_handshake(&hs.kind));
    assert!(!meets_fr129_circt_handshake(
        &InTreeScheduleKind::Handshake { channels: 4 }
    ));
}

#[test]
fn fr129_fr121_docs_boundary() {
    let fr121 = fs::read_to_string(root().join("docs/fr121-handshake-default.md")).unwrap();
    assert!(fr121.contains("closed") || fr121.contains("已关闭"));
}
