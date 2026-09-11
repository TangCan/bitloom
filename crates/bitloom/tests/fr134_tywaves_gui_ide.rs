//! ATDD — Story 73.2 / FR134: upstream Tywaves GUI / IDE plugin depth (G1–G4).
//!
//! ```text
//! cargo test -p bitloom --test fr134_tywaves_gui_ide
//! ```

use std::fs;
use std::os::unix::fs::PermissionsExt;
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
fn fr134_docs_and_nfr14_g1_g4() {
    let docs = read("docs/fr134-upstream-tywaves-gui-ide.md");
    assert!(docs.contains("FR134") && docs.contains("Bitloom"));
    assert!(docs.contains("--tywaves-gui"));
    assert!(docs.contains("tywaves.gui") && docs.contains("tywaves.ide-plugin"));
    assert!(
        docs.contains("FR125") && (docs.contains("alone") || docs.contains("≠")),
        "docs must bound against FR125 alone"
    );
    let nfr = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic73-upstream-tywaves-gui-ide.md",
    );
    assert!(nfr.contains("G1") && nfr.contains("G4"));
}

#[test]
fn fr134_wave_tywaves_gui_writes_manifest() {
    let out = tempfile_dir("fr134-ok");
    let gui_root = out.join("gui-root");
    fs::create_dir_all(&gui_root).unwrap();
    // Stub install marker: package present for G3 success path.
    fs::write(gui_root.join("BITLOOM_TYWAVES_GUI_OK"), "ok\n").unwrap();

    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    let status = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("4")
        .arg("--tywaves-gui")
        .env("BITLOOM_TYWAVES_GUI_ROOT", &gui_root)
        .env_remove("BITLOOM_TYWAVES_GUI_FORCE_MISSING")
        .env_remove("BITLOOM_TYWAVES_FORCE_MISSING")
        .status()
        .expect("spawn cargo bitloom wave --tywaves-gui");
    assert!(
        status.success(),
        "wave --tywaves-gui with stub GUI root must succeed"
    );

    let manifest = fs::read_to_string(out.join("tywaves.gui.manifest.json"))
        .expect("tywaves.gui.manifest.json");
    assert!(manifest.contains("\"product\": \"Bitloom\""));
    assert!(manifest.contains("\"fr\": \"FR134\""));
    assert!(manifest.contains("schemaVersion"));
    assert!(manifest.contains("tywaves.gui"));
    assert!(manifest.contains("tywaves.ide-plugin"));
    assert!(manifest.contains("data-bitloom-tywaves-gui"));

    let install =
        fs::read_to_string(out.join("tywaves.gui.install.json")).expect("tywaves.gui.install.json");
    assert!(
        install.contains("version") && install.contains("channel"),
        "G1 install descriptor must pin version + channel"
    );
    assert!(
        install.contains("gui") || install.contains("tywaves.gui"),
        "install descriptor must describe GUI package"
    );
    assert!(
        install.contains("ide-plugin")
            || install.contains("tywaves.ide-plugin")
            || install.contains("marketplace"),
        "install descriptor must describe IDE plugin channel"
    );
    assert!(out.join("tywaves.gui.install.sh").is_file());

    // FR125 sidecar still present when GUI depth path runs (builds on T1–T4)
    assert!(out.join("wave.tywaves.json").is_file());
    assert!(out.join("tywaves.launch.sh").is_file());
    // FR117 / FR104
    assert!(out.join("typed-wave.html").is_file());
    assert!(out.join("interactive.html").is_file());
}

#[test]
fn fr134_gui_install_descriptor_pins_version_channel() {
    // Unit-level: emitted install.json must exceed BITLOOM_TYWAVES_BIN alone.
    let out = tempfile_dir("fr134-g1");
    let gui_root = out.join("gui-root");
    fs::create_dir_all(&gui_root).unwrap();
    fs::write(gui_root.join("BITLOOM_TYWAVES_GUI_OK"), "ok\n").unwrap();
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    let status = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("2")
        .arg("--tywaves-gui")
        .env("BITLOOM_TYWAVES_GUI_ROOT", &gui_root)
        .env_remove("BITLOOM_TYWAVES_GUI_FORCE_MISSING")
        .status()
        .expect("spawn");
    assert!(status.success());
    let install = fs::read_to_string(out.join("tywaves.gui.install.json")).unwrap();
    assert!(
        !install.contains("BITLOOM_TYWAVES_BIN") || install.contains("channel"),
        "G1 must pin distribution channel beyond single-bin launch"
    );
    assert!(
        install.contains("surfer") || install.contains("tywaves") || install.contains("gitlab"),
        "G1 must name upstream GUI package identity"
    );
}

#[test]
fn fr134_force_missing_is_nonzero_readable() {
    let out = tempfile_dir("fr134-miss");
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    let output = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("2")
        .arg("--tywaves-gui")
        .env("BITLOOM_TYWAVES_GUI_FORCE_MISSING", "1")
        .env_remove("BITLOOM_TYWAVES_GUI_ROOT")
        .output()
        .expect("spawn");
    assert!(
        !output.status.success(),
        "GUI FORCE_MISSING must be non-zero"
    );
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("bitloom.tywaves"),
        "failure must be readable with bitloom.tywaves: {err}"
    );
    // Manifests still written before fail (G2 checkable even on fail)
    assert!(
        out.join("tywaves.gui.manifest.json").is_file(),
        "manifest must be written even when GUI live path fails"
    );
}

#[test]
fn fr134_not_satisfied_by_fr125_alone() {
    let out = tempfile_dir("fr134-no-gui");
    let stub = out.join("tywaves-stub.sh");
    fs::write(&stub, "#!/bin/sh\nexit 0\n").unwrap();
    let mut perms = fs::metadata(&stub).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&stub, perms).unwrap();

    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    let status = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("2")
        .arg("--tywaves")
        .env("BITLOOM_TYWAVES_BIN", &stub)
        .env_remove("BITLOOM_TYWAVES_FORCE_MISSING")
        .env_remove("BITLOOM_TYWAVES_GUI_FORCE_MISSING")
        .status()
        .expect("spawn");
    assert!(status.success());
    assert!(out.join("wave.tywaves.json").is_file());
    assert!(
        !out.join("tywaves.gui.manifest.json").is_file(),
        "FR125 alone must not emit FR134 GUI manifest"
    );
}

#[test]
fn fr134_tywaves_not_in_design_crate_deps() {
    let prelude = read("crates/bitloom-prelude/Cargo.toml");
    assert!(
        !prelude.to_lowercase().contains("tywaves"),
        "Tywaves runtime must not enter design-crate (bitloom-prelude) deps"
    );
}
