//! ATDD Story 120.2 / FR187 — Handshake lower deepen beyond FR180.
//!
//! ```text
//! cargo test -p bitloom --test fr187_handshake_lower_deepen
//! ```

use bitloom::hls::{
    HlsDataflowOp, InTreeScheduleKind, meets_fr129_circt_handshake, meets_fr180_handshake_deepen,
    meets_fr187_handshake_lower_deepen, schedule_circt_handshake, schedule_circt_handshake_deepen,
    schedule_circt_handshake_lower_deepen, schedule_handshake_default,
};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr187_docs_contract_forbid_fr180_alone() {
    let docs = read("docs/fr187-handshake-lower-deepen.md");
    assert!(docs.contains("FR187") && docs.contains("Bitloom"));
    assert!(
        docs.contains("handshake.branch") && docs.contains("handshake.merge"),
        "must pin branch+merge"
    );
    assert!(
        docs.contains("FR180")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR180 distinct"
    );
    assert!(
        docs.contains("AD-25")
            && (docs.contains("NFR90") || docs.contains("revise") || docs.contains("修订")),
        "must note AD-25 revise / NFR90"
    );
    assert!(
        docs.contains("FORCE_MISSING") || docs.contains("BITLOOM_HANDSHAKE_LOWER_FORCE_MISSING"),
        "must document FORCE_MISSING"
    );
    assert!(
        docs.contains("NFR91") || docs.contains("full") || docs.contains("全家桶"),
        "must leave fuller lower as NFR91"
    );
}

#[test]
fn fr187_spine_ad25_revised() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("FR187")
            && spine.contains("handshake.branch")
            && spine.contains("handshake.merge"),
        "AD-25 must document FR187 lower deepen"
    );
    assert!(
        spine.contains("2026-09-14") && spine.contains("NFR90") && spine.contains("FR187"),
        "AD-25 must have FR187 Revised stamp / NFR90"
    );
}

#[test]
fn fr187_schedule_emits_branch_merge() {
    let art =
        schedule_circt_handshake_lower_deepen("hs_lower", HlsDataflowOp::AddConst(1), 1, 2, 2)
            .expect("lower deepen schedule");
    assert!(meets_fr187_handshake_lower_deepen(&art.kind));
    assert!(!meets_fr180_handshake_deepen(&art.kind));
    assert!(!meets_fr129_circt_handshake(&art.kind));
    let ir = &art.schedule_ir;
    assert!(ir.contains("\"fr187\": true") || ir.contains("\"fr187\":true"));
    assert!(ir.contains("handshake.branch") && ir.contains("handshake.merge"));
    assert!(ir.contains("handshake.fork") && ir.contains("handshake.join"));
    assert!(ir.contains("handshake.func") && ir.contains("handshake.buffer"));
    assert!(art.rtl_stub.contains("handshake.branch") && art.rtl_stub.contains("handshake.merge"));
}

#[test]
fn fr187_rejects_fr180_alone_shape() {
    let fr180 =
        schedule_circt_handshake_deepen("hs180", HlsDataflowOp::AddConst(1), 1, 2, 2).unwrap();
    assert!(meets_fr180_handshake_deepen(&fr180.kind));
    assert!(!meets_fr187_handshake_lower_deepen(&fr180.kind));
    assert!(!fr180.schedule_ir.contains("handshake.branch"));

    let fr129 = schedule_circt_handshake("hs129", HlsDataflowOp::AddConst(1), 1, 2, 2).unwrap();
    assert!(!meets_fr187_handshake_lower_deepen(&fr129.kind));

    let hs = schedule_handshake_default("hs121", HlsDataflowOp::Identity, 2).unwrap();
    assert!(!meets_fr187_handshake_lower_deepen(&hs.kind));

    let err = schedule_circt_handshake_lower_deepen("bad", HlsDataflowOp::Identity, 1, 1, 1)
        .expect_err("shallow lower deepen must fail");
    let msg = format!("{err}");
    assert!(
        msg.contains("FR187") && (msg.contains("alone") || msg.contains("clock_domains")),
        "readable reject; got {msg}"
    );
}

#[test]
fn fr187_force_missing_nonzero() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args([
            "bitloom",
            "hls",
            "--circt-handshake-lower-deepen",
            "--function",
            "hs_force_missing",
            "--dataflow",
            "add1",
            "--channels",
            "1",
            "--clock-domains",
            "2",
            "--elastic-depth",
            "2",
            "--out-dir",
            "target/fr187-force-missing",
        ])
        .current_dir(root())
        .env("BITLOOM_HANDSHAKE_LOWER_FORCE_MISSING", "1")
        .output()
        .expect("run force-missing cli");
    assert!(
        !out.status.success(),
        "FORCE_MISSING must be non-zero; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("FORCE_MISSING") || err.contains("refusing") || err.contains("unavailable"),
        "stderr must be readable; got {err}"
    );
}

#[test]
fn fr187_cli_flag_present() {
    let main = read("crates/bitloom/src/main.rs");
    assert!(
        main.contains("circt_handshake_lower_deepen")
            || main.contains("circt-handshake-lower-deepen"),
        "CLI must expose lower deepen flag"
    );
    let _ = InTreeScheduleKind::CirctHandshakeLowerDeepen {
        channels: 1,
        clock_domains: 2,
        elastic_depth: 1,
    };
}

#[test]
fn fr187_cli_smoke_optional() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args([
            "bitloom",
            "hls",
            "--circt-handshake-lower-deepen",
            "--function",
            "hs_cli_lower",
            "--dataflow",
            "add1",
            "--channels",
            "1",
            "--clock-domains",
            "2",
            "--elastic-depth",
            "2",
            "--out-dir",
            "target/fr187-cli-smoke",
        ])
        .current_dir(root())
        .output()
        .expect("run cli");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "cli lower deepen must pass; stdout={stdout} stderr={stderr}"
    );
    assert!(
        stdout.contains("fr187=true") || stdout.contains("handshake_lower_deepen"),
        "stdout should cite FR187; got {stdout}"
    );
}
