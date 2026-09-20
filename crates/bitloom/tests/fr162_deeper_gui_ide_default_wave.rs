//! ATDD: FR162 — deeper GUI/IDE default wave primary surface (Story 94.2).
//!
//! ```text
//! cargo test -p bitloom --test fr162_deeper_gui_ide_default_wave
//! ```

use std::fs;
use std::path::PathBuf;
use std::process::Command;

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

fn cargo_bitloom() -> Command {
    let mut c = Command::new(env!("CARGO"));
    c.arg("run").arg("-q").arg("-p").arg("bitloom").arg("--");
    c
}

#[test]
fn fr162_docs_contract_forbid_fr134_alone() {
    let text = read("docs/fr162-deeper-gui-ide-default-wave.md");
    assert!(text.contains("FR162") && text.contains("Bitloom"));
    assert!(
        text.contains("primary") || text.contains("默认") || text.contains("default"),
        "must declare default/primary surface"
    );
    assert!(
        text.contains("tywaves.gui.manifest.json") || text.contains("tywaves.gui"),
        "must name FR134-level GUI artifacts"
    );
    assert!(
        text.contains("typed-wave")
            && (text.contains("secondary") || text.contains("次要") || text.contains("alone")),
        "typed-wave must not be sole completion face"
    );
    assert!(
        text.contains("FR134")
            && (text.contains("alone") || text.contains("≠") || text.contains("仍")),
        "must keep FR134 distinct / alone ban"
    );
    assert!(
        text.contains("--no-tywaves-gui") || text.contains("opt"),
        "must document opt-out or opt path"
    );
    let nfr =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic94-deeper-gui-ide-fr162.md");
    assert!(nfr.contains("FR162") && nfr.contains("选定"));
}

#[test]
fn fr162_default_wave_emits_gui_primary() {
    let out = tempfile_dir("fr162-default");
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_wave_counter.fir");
    let status = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("4")
        // no --tywaves-gui: FR162 default primary
        .env_remove("BITLOOM_TYWAVES_GUI_FORCE_MISSING")
        .env_remove("BITLOOM_TYWAVES_FORCE_MISSING")
        .env_remove("BITLOOM_TYWAVES_GUI_ROOT")
        .status()
        .expect("spawn default wave");
    assert!(
        status.success(),
        "default wave (FR162 primary) must succeed"
    );

    let manifest = fs::read_to_string(out.join("tywaves.gui.manifest.json"))
        .expect("default wave must emit tywaves.gui.manifest.json");
    assert!(manifest.contains("\"fr\": \"FR134\"") || manifest.contains("FR134"));
    assert!(manifest.contains("tywaves.gui"));
    assert!(out.join("tywaves.gui.install.json").is_file());
    assert!(out.join("wave.tywaves.json").is_file());
    // Secondary surfaces still present
    assert!(out.join("typed-wave.html").is_file());
    assert!(out.join("interactive.html").is_file());
    assert!(out.join("wave.vcd").is_file());
}

#[test]
fn fr162_default_force_missing_nonzero_readable() {
    let out = tempfile_dir("fr162-miss");
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_wave_counter.fir");
    let output = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("2")
        .env("BITLOOM_TYWAVES_GUI_FORCE_MISSING", "1")
        .env_remove("BITLOOM_TYWAVES_GUI_ROOT")
        .output()
        .expect("spawn");
    assert!(
        !output.status.success(),
        "FR162 primary FORCE_MISSING must be non-zero"
    );
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("bitloom.tywaves"),
        "failure must be readable with bitloom.tywaves: {err}"
    );
    assert!(
        out.join("tywaves.gui.manifest.json").is_file(),
        "manifest must be written even when primary live path fails"
    );
}

#[test]
fn fr162_no_tywaves_gui_opts_out_of_primary() {
    let out = tempfile_dir("fr162-optout");
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_wave_counter.fir");
    let status = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("2")
        .arg("--no-tywaves-gui")
        .env_remove("BITLOOM_TYWAVES_GUI_FORCE_MISSING")
        .env_remove("BITLOOM_TYWAVES_FORCE_MISSING")
        .status()
        .expect("spawn");
    assert!(status.success());
    assert!(out.join("typed-wave.html").is_file());
    assert!(
        !out.join("tywaves.gui.manifest.json").is_file(),
        "--no-tywaves-gui must skip GUI primary"
    );
    assert!(
        !out.join("wave.tywaves.json").is_file(),
        "opt-out without --tywaves must not emit FR125 sidecar"
    );
}

#[test]
fn fr162_cli_help_documents_default_and_optout() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args(["wave", "--help"])
        .output()
        .expect("wave --help");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("--no-tywaves-gui") || text.contains("no-tywaves-gui"),
        "help must document opt-out:\n{text}"
    );
}
