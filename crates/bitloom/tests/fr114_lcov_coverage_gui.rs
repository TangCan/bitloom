//! ATDD: FR114 LCOV + in-tree coverage GUI (Story 56.2).
//!
//! cargo test -p bitloom --test fr114_lcov_coverage_gui

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::PortValues;
use bitloom_sim::{Coverage, Sim, write_coverage_artifacts};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

fn tempfile_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bitloom-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn mux_hir() -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new("Fr114Mux");
    s.begin_module("Fr114Mux", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("sel", GroundType::Bool, Span::default());
    s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
    s.add_input("b", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_mux("y", "sel", "a", "b", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("elaborate")
}

#[test]
fn fr114_docs_subset_b_and_bans() {
    let text = read("docs/fr114-lcov-coverage-gui.md");
    assert!(text.contains("FR114") && text.contains("Bitloom"));
    assert!(text.contains("LCOV") && text.contains("coverage.html"));
    assert!(
        text.contains("deferred") && text.contains("Tywaves"),
        "A Tywaves must stay deferred"
    );
    assert!(
        text.contains("FR104")
            && (text.contains("alone") || text.contains("≠") || text.contains("I1")),
        "must ban FR104 alone"
    );
    assert!(text.contains("FR105") || text.contains("Mux"));
    assert!(text.contains("cargo bitloom coverage"));
}

#[test]
fn fr114_write_lcov_and_gui_from_sim() {
    let mut sim = Sim::new(mux_hir());
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("sel", 0);
    pv.set("a", 1);
    pv.set("b", 2);
    sim.set_inputs(pv);
    sim.tick();
    sim.tick();

    let out = tempfile_dir("fr114-lib");
    let (lcov, html) = sim.write_coverage_artifacts(&out).expect("write FR114");
    let lcov_text = fs::read_to_string(lcov).unwrap();
    assert!(lcov_text.contains("TN:bitloom-sim"));
    assert!(lcov_text.contains("DA:"));
    assert!(lcov_text.contains("end_of_record"));
    assert!(lcov_text.contains("LH:"));
    let html_text = fs::read_to_string(html).unwrap();
    assert!(html_text.contains("data-bitloom-coverage-gui"));
    assert!(html_text.contains("Bitloom"));
    assert!(html_text.contains("cov-search"));
    assert!(
        html_text.contains("hit") || html_text.contains("miss"),
        "GUI must list coverage statuses"
    );
}

#[test]
fn fr114_empty_coverage_fails_readable() {
    let cov = Coverage::default();
    let out = tempfile_dir("fr114-empty");
    let err = write_coverage_artifacts(&cov, &out).expect_err("empty must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("coverage-empty") || msg.contains("FR114"),
        "readable empty fail: {msg}"
    );
    assert!(!out.join("coverage.lcov").is_file());
}

#[test]
fn fr114_cli_coverage_emits_artifacts() {
    let out = tempfile_dir("fr114-cli");
    let bin = workspace_root().join("target/debug/cargo-bitloom");
    let status = Command::new(env!("CARGO"))
        .args(["run", "-q", "-p", "bitloom", "--", "coverage", "--out-dir"])
        .arg(&out)
        .arg("--ticks")
        .arg("3")
        .status()
        .expect("run coverage");
    assert!(status.success(), "cargo bitloom coverage failed");
    assert!(out.join("coverage.lcov").is_file());
    assert!(out.join("coverage.html").is_file());
    let html = fs::read_to_string(out.join("coverage.html")).unwrap();
    assert!(html.contains("data-bitloom-coverage-gui"));
    let _ = bin;
}

#[test]
fn fr114_fr104_wave_still_available() {
    let doc = read("docs/fr104-interactive-wave.md");
    assert!(doc.contains("interactive.html"));
    let cli = read("crates/bitloom/src/main.rs");
    assert!(cli.contains("Commands::Wave") || cli.contains("run_wave"));
}
