//! CLI smoke: product HLS path fails readably when bambu is missing (FR35).

use std::process::Command;

#[test]
fn hls_help_mentions_product_path() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args(["hls", "--help"])
        .output()
        .expect("spawn cargo-bitloom");
    assert!(out.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        text.contains("Bambu") || text.contains("bambu") || text.contains("HLS"),
        "help={text}"
    );
    assert!(
        text.contains("--emit-only") || text.contains("emit-only"),
        "help={text}"
    );
}

#[test]
fn hls_missing_bambu_reports_error() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args([
            "hls",
            "--function",
            "add",
            "--out-dir",
            "target/bitloom-hls-smoke",
        ])
        .env_remove("BITLOOM_BAMBU_PATH")
        .env_remove("RHDL_BAMBU_PATH")
        .env_remove("BITLOOM_HLS_EMIT_ONLY")
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
        err.contains("bambu")
            || err.contains("BITLOOM_BAMBU_PATH")
            || err.contains("RHDL_BAMBU_PATH"),
        "stderr={err}"
    );
    assert!(
        !err.to_lowercase().contains("unsupported"),
        "must not use permanent-unsupported messaging; stderr={err}"
    );
}

#[test]
fn hls_emit_only_writes_c_without_backend() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out_dir = "target/bitloom-hls-emit-only";
    let _ = std::fs::remove_dir_all(out_dir);
    let out = Command::new(bin)
        .args([
            "hls",
            "--function",
            "add",
            "--out-dir",
            out_dir,
            "--emit-only",
        ])
        .env_remove("BITLOOM_BAMBU_PATH")
        .env("PATH", "/usr/bin:/bin")
        .output()
        .expect("spawn cargo-bitloom");
    assert!(
        out.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let c = std::path::Path::new(out_dir).join("add.c");
    assert!(c.is_file(), "expected {}", c.display());
    let text = std::fs::read_to_string(&c).unwrap();
    assert!(text.contains("no scheduling in bitloom"));
    assert!(text.contains("2024.10"));
}
