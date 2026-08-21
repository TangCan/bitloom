//! ATDD Story 23.2: `cargo bitloom visualize` / `doc` — FR38 / FR49 / FR40.

use std::fs;
use std::process::Command;

#[test]
fn visualize_help_mentions_input_and_out_dir() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args(["visualize", "--help"])
        .output()
        .expect("run visualize --help");
    assert!(out.status.success(), "visualize --help failed");
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("--input") && text.contains("--out-dir"),
        "help must document --input and --out-dir:\n{text}"
    );
}

#[test]
fn doc_help_is_visualize_alias() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args(["doc", "--help"])
        .output()
        .expect("run doc --help");
    assert!(out.status.success(), "doc --help failed");
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("--input") && text.contains("--out-dir"),
        "doc help must document --input and --out-dir:\n{text}"
    );
}

#[test]
fn visualize_smoke_emits_nonempty_hierarchy_html() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let fir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../rhdl-firrtl/fixtures/external_hierarchy.fir"
    );
    let out_dir = tempfile_dir("viz");
    let out = Command::new(bin)
        .args([
            "visualize",
            "--input",
            fir,
            "--out-dir",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("run visualize smoke");
    assert!(
        out.status.success(),
        "visualize smoke failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let html_path = out_dir.join("hierarchy.html");
    let html = fs::read_to_string(&html_path).expect("hierarchy.html");
    assert!(!html.trim().is_empty(), "hierarchy.html must be non-empty");
    assert!(
        html.contains("Bitloom")
            && html.contains("Modules and ports")
            && html.contains("Instance hierarchy"),
        "hierarchy HTML missing required sections:\n{html}"
    );
    assert!(
        html.contains("port") && (html.contains("ExternalTop") || html.contains("Child")),
        "hierarchy must list modules/ports from fixture:\n{html}"
    );
    assert!(
        html.contains("u0") && html.contains("Child"),
        "hierarchy must include instance edge u0:Child:\n{html}"
    );
    let _ = fs::remove_dir_all(&out_dir);
}

fn tempfile_dir(tag: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!(
        "bitloom-{tag}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&p).unwrap();
    p
}
