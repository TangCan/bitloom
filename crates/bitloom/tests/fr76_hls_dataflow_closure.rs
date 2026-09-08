//! ATDD: FR76 HLS dataflow closures (Story 29.2 / Cap-R-62 / Cap-R-71).
//!
//! Recipe:
//! ```text
//! cargo test -p bitloom --test fr76_hls_dataflow_closure
//! ```
//!
//! Path: dissolve transform Fn → C (no residue) → emit-only and/or bambu-ci-stub → RTL.
//! AD-25: no in-tree scheduler. Missing backend must fail readably.

use std::path::PathBuf;
use std::process::Command;

use bitloom::hls::{
    HLS_BACKEND_VERSION, HlsDataflowClosureViolation, HlsDataflowOp, dissolve_dataflow_transform,
    resolve_bambu, run_hls_dataflow_with_backend, run_hls_dissolved,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn assert_no_closure_ir(label: &str, text: &str) {
    let lower = text.to_lowercase();
    assert!(
        !lower.contains("closure") && !lower.contains("callback") && !text.contains("||"),
        "{label}: must not contain closure/callback IR (NFR36):\n{text}"
    );
    assert!(
        !text.contains("Fn(") && !text.contains("Fn ("),
        "{label}: must not contain Fn IR (NFR36):\n{text}"
    );
}

#[test]
fn dissolve_map_xor_emit_only_has_no_fn_residue() {
    let out = workspace_root().join("target/fr76-hls-emit-only");
    let _ = std::fs::remove_dir_all(&out);
    let dissolved =
        dissolve_dataflow_transform("map_xor", &[], || HlsDataflowOp::XorConst(0xa5)).unwrap();
    assert_no_closure_ir("dissolved.c_source", &dissolved.c_source);
    assert!(
        dissolved.c_source.contains("^ 165u"),
        "expected xor const in C: {}",
        dissolved.c_source
    );
    assert!(dissolved.c_source.contains("FR76"));
    assert!(dissolved.c_source.contains("no scheduling in bitloom"));

    let c = run_hls_dissolved(&dissolved, &out, true).unwrap();
    assert_eq!(c.extension().and_then(|e| e.to_str()), Some("c"));
    let text = std::fs::read_to_string(&c).unwrap();
    assert_no_closure_ir("emit-only file", &text);
    assert!(text.contains(HLS_BACKEND_VERSION));
}

#[test]
fn algorithm_fixture_via_stub_yields_synthesizable_rtl() {
    let root = workspace_root();
    let stub = root.join("scripts/fixtures/bambu-ci-stub.sh");
    assert!(stub.is_file(), "missing {}", stub.display());
    let out = root.join("target/fr76-hls-stub");
    let _ = std::fs::remove_dir_all(&out);

    let rtl = run_hls_dataflow_with_backend("map_xor", "xor_a5", &out, false, Some(&stub))
        .expect("stub HLS run");
    assert!(
        rtl.extension().and_then(|e| e.to_str()) == Some("v")
            || rtl.extension().and_then(|e| e.to_str()) == Some("sv"),
        "expected synthesizable RTL, got {}",
        rtl.display()
    );
    let v = std::fs::read_to_string(&rtl).unwrap();
    assert!(
        v.contains("module") && (v.contains("assign") || v.contains("always")),
        "RTL should look synthesizable: {v}"
    );
    assert_no_closure_ir("stub .v", &v);

    let c = out.join("map_xor.c");
    let c_text = std::fs::read_to_string(&c).unwrap();
    assert_no_closure_ir("pre-schedule C", &c_text);
    assert!(c_text.contains("^ 165u"));
}

#[test]
fn missing_backend_errors_clearly_no_silent_success() {
    // Library resolve with no override: use a Command-scoped env for CLI (no process mutation).
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out_dir = workspace_root().join("target/fr76-hls-missing-backend");
    let _ = std::fs::remove_dir_all(&out_dir);
    let out = Command::new(bin)
        .args([
            "hls",
            "--function",
            "map_xor",
            "--dataflow",
            "xor_a5",
            "--out-dir",
            out_dir.to_str().unwrap(),
        ])
        .env_remove("BITLOOM_BAMBU_PATH")
        .env_remove("RHDL_BAMBU_PATH")
        .env_remove("BITLOOM_HLS_EMIT_ONLY")
        .env("PATH", "/usr/bin:/bin")
        .output()
        .expect("spawn");
    assert!(
        !out.status.success(),
        "expected failure; stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("bambu") || err.contains("BITLOOM_BAMBU_PATH"),
        "readable missing-backend error; stderr={err}"
    );
    assert!(
        !err.to_lowercase().contains("unsupported"),
        "must not silent-skip as unsupported; stderr={err}"
    );
    // Also ensure resolve_bambu documents the pin when env is empty in a fresh resolve attempt
    // (may succeed if ambient PATH has bambu — CLI path above is the AC evidence).
    let _ = resolve_bambu();
}

#[test]
fn constraint_class_rejects_capturing_before_schedule() {
    let err = dissolve_dataflow_transform(
        "bad",
        &[HlsDataflowClosureViolation::capturing(
            "captures mut counter",
        )],
        || HlsDataflowOp::AddConst(1),
    )
    .unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("rejected") || msg.contains("CapturingOrStateful"),
        "{msg}"
    );
}

#[test]
fn wrong_path_violation_is_checked() {
    let err = dissolve_dataflow_transform(
        "bad",
        &[HlsDataflowClosureViolation::wrong_path_synthesizable(
            "use SynthesizableClosure on comb/seq, not HlsFree",
        )],
        || HlsDataflowOp::Identity,
    )
    .unwrap_err();
    assert!(err.to_string().contains("WrongPathForSynthesizable"));
}

#[test]
fn cli_dataflow_emit_only_and_help() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let help = Command::new(bin)
        .args(["hls", "--help"])
        .output()
        .expect("help");
    assert!(help.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&help.stdout),
        String::from_utf8_lossy(&help.stderr)
    );
    assert!(
        text.contains("dataflow") || text.contains("--dataflow"),
        "help={text}"
    );

    let out_dir = workspace_root().join("target/fr76-cli-emit");
    let _ = std::fs::remove_dir_all(&out_dir);
    let out = Command::new(bin)
        .args([
            "hls",
            "--function",
            "map_xor",
            "--dataflow",
            "xor_a5",
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--emit-only",
        ])
        .env_remove("BITLOOM_BAMBU_PATH")
        .env("PATH", "/usr/bin:/bin")
        .output()
        .expect("cli emit-only");
    assert!(
        out.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let c = std::fs::read_to_string(out_dir.join("map_xor.c")).unwrap();
    assert_no_closure_ir("cli C", &c);
    assert!(c.contains("^ 165u"));
}
