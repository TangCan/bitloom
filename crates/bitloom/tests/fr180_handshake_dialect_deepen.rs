//! ATDD Story 113.2 / FR180 — Handshake dialect deepen beyond FR129.
//!
//! ```text
//! cargo test -p bitloom --test fr180_handshake_dialect_deepen
//! ```

use bitloom::hls::{
    HlsDataflowOp, InTreeScheduleKind, meets_fr129_circt_handshake, meets_fr180_handshake_deepen,
    schedule_circt_handshake, schedule_circt_handshake_deepen, schedule_handshake_default,
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
fn fr180_docs_contract_forbid_fr129_alone() {
    let docs = read("docs/fr180-handshake-dialect-deepen.md");
    assert!(docs.contains("FR180") && docs.contains("Bitloom"));
    assert!(
        docs.contains("handshake.fork") && docs.contains("handshake.join"),
        "must pin fork+join"
    );
    assert!(
        docs.contains("FR129")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR129 distinct"
    );
    assert!(
        docs.contains("AD-25")
            && (docs.contains("NFR85") || docs.contains("revise") || docs.contains("修订")),
        "must note AD-25 revise / NFR85"
    );
    assert!(
        docs.contains("FORCE_MISSING") || docs.contains("BITLOOM_HANDSHAKE_DEEPEN_FORCE_MISSING"),
        "must document FORCE_MISSING"
    );
    assert!(
        docs.contains("NFR86") || docs.contains("full") || docs.contains("全家桶"),
        "must leave fuller dialect as NFR86"
    );
}

#[test]
fn fr180_spine_ad25_revised() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("FR180")
            && spine.contains("handshake.fork")
            && spine.contains("handshake.join"),
        "AD-25 must document FR180 deepen"
    );
    assert!(
        spine.contains("2026-09-14") && spine.contains("NFR85"),
        "AD-25 must have FR180 Revised stamp"
    );
}

#[test]
fn fr180_schedule_emits_fork_join() {
    let art = schedule_circt_handshake_deepen("hs_deepen", HlsDataflowOp::AddConst(1), 1, 2, 2)
        .expect("deepen schedule");
    assert!(meets_fr180_handshake_deepen(&art.kind));
    assert!(!meets_fr129_circt_handshake(&art.kind));
    let ir = &art.schedule_ir;
    assert!(ir.contains("\"fr180\": true") || ir.contains("\"fr180\":true"));
    assert!(ir.contains("handshake.fork") && ir.contains("handshake.join"));
    assert!(ir.contains("handshake.func") && ir.contains("handshake.buffer"));
    assert!(art.rtl_stub.contains("handshake.fork") && art.rtl_stub.contains("handshake.join"));
}

#[test]
fn fr180_rejects_fr129_alone_shape() {
    let fr129 = schedule_circt_handshake("hs129", HlsDataflowOp::AddConst(1), 1, 2, 2).unwrap();
    assert!(meets_fr129_circt_handshake(&fr129.kind));
    assert!(!meets_fr180_handshake_deepen(&fr129.kind));
    assert!(!fr129.schedule_ir.contains("handshake.fork"));

    let hs = schedule_handshake_default("hs121", HlsDataflowOp::Identity, 2).unwrap();
    assert!(!meets_fr180_handshake_deepen(&hs.kind));

    let err = schedule_circt_handshake_deepen("bad", HlsDataflowOp::Identity, 1, 1, 1)
        .expect_err("shallow deepen must fail");
    let msg = format!("{err}");
    assert!(
        msg.contains("FR180") && (msg.contains("alone") || msg.contains("clock_domains")),
        "readable reject; got {msg}"
    );
}

#[test]
fn fr180_force_missing_nonzero() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args([
            "bitloom",
            "hls",
            "--circt-handshake-deepen",
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
            "target/fr180-force-missing",
        ])
        .current_dir(root())
        .env("BITLOOM_HANDSHAKE_DEEPEN_FORCE_MISSING", "1")
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
fn fr180_cli_flag_present() {
    let main = read("crates/bitloom/src/main.rs");
    assert!(
        main.contains("circt_handshake_deepen") || main.contains("circt-handshake-deepen"),
        "CLI must expose deepen flag"
    );
    let _ = InTreeScheduleKind::CirctHandshakeDeepen {
        channels: 1,
        clock_domains: 2,
        elastic_depth: 1,
    };
}

#[test]
fn fr180_cli_smoke_optional() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args([
            "bitloom",
            "hls",
            "--circt-handshake-deepen",
            "--function",
            "hs_cli_deepen",
            "--dataflow",
            "add1",
            "--channels",
            "1",
            "--clock-domains",
            "2",
            "--elastic-depth",
            "2",
            "--out-dir",
            "target/fr180-cli-smoke",
        ])
        .current_dir(root())
        .output()
        .expect("run cli");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "cli deepen must pass; stdout={stdout} stderr={stderr}"
    );
    assert!(
        stdout.contains("fr180=true") || stdout.contains("handshake_deepen"),
        "stdout should cite FR180; got {stdout}"
    );
}
