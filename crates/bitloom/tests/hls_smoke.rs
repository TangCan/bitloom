//! CLI smoke: HLS with enable but no bambu on PATH.

use std::process::Command;

#[test]
fn hls_missing_bambu_reports_error() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args([
            "hls",
            "--function",
            "add",
            "--out-dir",
            "target/rhdl-hls-smoke",
        ])
        .env("RHDL_HLS_ENABLE", "1")
        .env_remove("RHDL_BAMBU_PATH")
        .env("PATH", "/usr/bin:/bin") // unlikely to contain bambu
        .output()
        .expect("spawn cargo-bitloom");
    assert!(
        !out.status.success(),
        "expected failure when bambu missing; stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("bambu") || err.contains("RHDL_BAMBU_PATH"),
        "stderr={err}"
    );
}
