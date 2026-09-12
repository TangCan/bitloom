//! ATDD Story 90.2 / FR158 — third-party genhtml LCOV GUI path.
//!
//! ```text
//! cargo test -p bitloom --test fr158_third_party_lcov_gui
//! ```

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

use bitloom::lcov_gui::{LcovGuiError, find_genhtml, run_genhtml};
use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::PortValues;
use bitloom_sim::Sim;

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
    let mut s = ElaborateSession::new("Fr158Mux");
    s.begin_module("Fr158Mux", Span::default());
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

fn write_fixture_lcov(dir: &std::path::Path) -> PathBuf {
    let mut sim = Sim::new(mux_hir());
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("sel", 0);
    pv.set("a", 1);
    pv.set("b", 2);
    sim.set_inputs(pv);
    sim.tick();
    let (lcov, _) = sim.write_coverage_artifacts(dir).expect("FR114 lcov");
    lcov
}

#[test]
fn fr158_docs_contract() {
    let text = read("docs/fr158-third-party-lcov-gui.md");
    assert!(text.contains("FR158") && text.contains("Bitloom"));
    assert!(text.contains("genhtml") && text.contains("coverage.lcov"));
    assert!(
        text.contains("--genhtml") && text.contains("cargo bitloom coverage"),
        "must document CLI --genhtml"
    );
    assert!(
        text.contains("FR114")
            && (text.contains("≠") || text.contains("alone") || text.contains("不")),
        "must contrast FR114 alone ≠ FR158"
    );
    assert!(
        (text.contains("not found") || text.contains("PATH") || text.contains("缺"))
            && (text.contains("lcov") || text.contains("install")),
        "must document missing-tool semantics"
    );
    assert!(
        text.contains("FR142") || text.contains("extends") || text.contains("子命令"),
        "must note FR142 / coverage flag extension"
    );
}

#[test]
fn fr158_missing_genhtml_is_explicit_error() {
    let dir = tempfile_dir("fr158-miss");
    let lcov = write_fixture_lcov(&dir);
    let html_out = dir.join("genhtml");

    // Empty PATH → genhtml not found (even if host has it).
    let prev = std::env::var_os("PATH");
    // SAFETY: test-only PATH override; restored below.
    unsafe { std::env::set_var("PATH", "") };
    let err = run_genhtml(&lcov, &html_out).expect_err("must fail without genhtml");
    match prev {
        Some(p) => unsafe { std::env::set_var("PATH", p) },
        None => unsafe { std::env::remove_var("PATH") },
    }
    assert!(
        matches!(err, LcovGuiError::GenhtmlNotFound),
        "expected GenhtmlNotFound, got {err}"
    );
    let msg = err.to_string();
    assert!(
        msg.contains("genhtml") && msg.contains("FR158") && msg.contains("FR114"),
        "error must mention FR158/genhtml and not substitute FR114: {msg}"
    );
}

#[test]
fn fr158_fake_genhtml_produces_html() {
    let dir = tempfile_dir("fr158-ok");
    let lcov = write_fixture_lcov(&dir);
    let bin_dir = dir.join("bin");
    fs::create_dir_all(&bin_dir).unwrap();
    let fake = bin_dir.join("genhtml");
    // Minimal stub: `genhtml -o OUT LCOV` → write OUT/index.html
    fs::write(
        &fake,
        r#"#!/bin/sh
out=""
while [ $# -gt 0 ]; do
  case "$1" in
    -o) out="$2"; shift 2 ;;
    *) shift ;;
  esac
done
mkdir -p "$out"
echo '<html><body>FR158 fake genhtml</body></html>' > "$out/index.html"
"#,
    )
    .unwrap();
    let mut perms = fs::metadata(&fake).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&fake, perms).unwrap();

    let prev = std::env::var_os("PATH");
    unsafe { std::env::set_var("PATH", bin_dir.as_os_str()) };
    assert!(find_genhtml().is_some());
    let html_out = dir.join("genhtml");
    let entry = run_genhtml(&lcov, &html_out).expect("fake genhtml");
    match prev {
        Some(p) => unsafe { std::env::set_var("PATH", p) },
        None => unsafe { std::env::remove_var("PATH") },
    }
    assert!(entry.is_file(), "expected HTML at {}", entry.display());
    let body = fs::read_to_string(&entry).unwrap();
    assert!(body.contains("FR158") || body.contains("html"));
}

#[test]
fn fr158_cli_genhtml_flag_help() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args(["bitloom", "coverage", "--help"])
        .output()
        .expect("help");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "coverage --help failed: {text}");
    assert!(
        text.contains("--genhtml"),
        "CLI help must list --genhtml: {text}"
    );
}

#[test]
fn fr158_lcov_missing_errors() {
    let dir = tempfile_dir("fr158-nolcov");
    let err = run_genhtml(&dir.join("nope.lcov"), &dir.join("out")).unwrap_err();
    assert!(matches!(err, LcovGuiError::LcovMissing(_)));
}
