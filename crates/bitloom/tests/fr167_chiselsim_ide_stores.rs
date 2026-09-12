//! ATDD — Story 100.2 / FR167: ChiselSim coupling + multi IDE-store publish.
//!
//! ```text
//! cargo test -p bitloom --test fr167_chiselsim_ide_stores
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
fn fr167_docs_and_nfr14_require_both_a_and_b() {
    let docs = read("docs/fr167-chiselsim-ide-stores.md");
    assert!(docs.contains("FR167") && docs.contains("Bitloom"));
    assert!(docs.contains("ChiselSim") && docs.contains("--chiselsim"));
    assert!(docs.contains("Open VSX") && docs.contains("JetBrains"));
    assert!(
        docs.contains("FR162")
            && docs.contains("FR134")
            && (docs.contains("alone") || docs.contains("≠")),
        "docs must bound against FR162/FR134 alone"
    );
    assert!(
        docs.contains("bitloom.chiselsim") || docs.contains("bitloom.ide-store"),
        "docs must cite failure markers"
    );
    let nfr = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic100-chiselsim-ide-stores-fr167.md",
    );
    assert!(nfr.contains("(a)") && nfr.contains("(b)") && nfr.contains("皆"));
}

#[test]
fn fr167_chiselsim_writes_manifest_and_validates_root() {
    let out = tempfile_dir("fr167-cs-ok");
    let root = out.join("chiselsim-root");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("BITLOOM_CHISELSIM_OK"), "ok\n").unwrap();

    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    let status = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("4")
        .arg("--chiselsim")
        .env("BITLOOM_CHISELSIM_ROOT", &root)
        .env_remove("BITLOOM_CHISELSIM_FORCE_MISSING")
        .status()
        .expect("spawn --chiselsim");
    assert!(status.success(), "chiselsim with stub root must succeed");

    let manifest = fs::read_to_string(out.join("chiselsim.manifest.json")).unwrap();
    assert!(manifest.contains("data-bitloom-chiselsim") && manifest.contains("FR167"));
    assert!(manifest.contains("chisel3-chiselsim") || manifest.contains("chiselsim"));
    let install = fs::read_to_string(out.join("chiselsim.install.json")).unwrap();
    assert!(install.contains("BITLOOM_CHISELSIM_ROOT"));
    assert!(out.join("chiselsim.check.sh").is_file());
}

#[test]
fn fr167_chiselsim_force_missing_is_nonzero() {
    let out = tempfile_dir("fr167-cs-fail");
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    let output = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("2")
        .arg("--chiselsim")
        .env("BITLOOM_CHISELSIM_FORCE_MISSING", "1")
        .env_remove("BITLOOM_CHISELSIM_ROOT")
        .output()
        .expect("spawn FORCE_MISSING chiselsim");
    assert!(!output.status.success(), "FORCE_MISSING must be non-zero");
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("bitloom.chiselsim"),
        "stderr must cite bitloom.chiselsim*; got {err}"
    );
}

#[test]
fn fr167_ide_stores_writes_manifest_with_tokens() {
    let out = tempfile_dir("fr167-ide-ok");
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    let status = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("4")
        .arg("--ide-stores")
        .env("OVSX_PAT", "test-ovsx")
        .env("JETBRAINS_TOKEN", "test-jb")
        .env_remove("BITLOOM_IDE_STORE_PUBLISH_FORCE_MISSING")
        .status()
        .expect("spawn --ide-stores");
    assert!(status.success(), "ide-stores with tokens must succeed");

    let manifest = fs::read_to_string(out.join("tywaves.ide-stores.manifest.json")).unwrap();
    assert!(manifest.contains("data-bitloom-ide-stores") && manifest.contains("FR167"));
    assert!(manifest.contains("open-vsx") && manifest.contains("jetbrains"));
    let install = fs::read_to_string(out.join("tywaves.ide-stores.install.json")).unwrap();
    assert!(install.contains("OVSX_PAT") && install.contains("JETBRAINS_TOKEN"));
}

#[test]
fn fr167_ide_stores_missing_token_is_nonzero() {
    let out = tempfile_dir("fr167-ide-fail");
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    let output = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("2")
        .arg("--ide-stores")
        .env_remove("OVSX_PAT")
        .env_remove("JETBRAINS_TOKEN")
        .env_remove("BITLOOM_IDE_STORE_PUBLISH_FORCE_MISSING")
        .output()
        .expect("spawn ide-stores missing token");
    assert!(!output.status.success(), "missing tokens must be non-zero");
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("bitloom.ide-store-missing-token") || err.contains("ide-store"),
        "stderr must cite missing-token; got {err}"
    );
}

#[test]
fn fr167_publish_script_fails_without_tokens() {
    let script = workspace_root().join("scripts/publish-tywaves-ide-stores.sh");
    assert!(script.is_file());
    let mut perms = fs::metadata(&script).unwrap().permissions();
    perms.set_mode(0o755);
    let _ = fs::set_permissions(&script, perms);

    let output = Command::new(&script)
        .current_dir(workspace_root())
        .env_remove("OVSX_PAT")
        .env_remove("JETBRAINS_TOKEN")
        .env_remove("BITLOOM_IDE_STORE_PUBLISH_FORCE_MISSING")
        .output()
        .expect("run publish script");
    assert!(!output.status.success());
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("bitloom.ide-store-missing-token"),
        "publish script must refuse silent-Ok; got {err}"
    );

    let ok = Command::new(&script)
        .current_dir(workspace_root())
        .arg("dry-run")
        .env("OVSX_PAT", "test")
        .env("JETBRAINS_TOKEN", "test")
        .env_remove("BITLOOM_IDE_STORE_PUBLISH_FORCE_MISSING")
        .status()
        .expect("dry-run with tokens");
    assert!(ok.success(), "dry-run with tokens must succeed");
}

#[test]
fn fr167_cli_apis_expose_both_faces() {
    let m = bitloom::fr167::chiselsim_manifest("t");
    assert!(m.contains("data-bitloom-chiselsim") && m.contains("FR167"));
    let s = bitloom::fr167::ide_stores_manifest("t");
    assert!(s.contains("open-vsx") && s.contains("jetbrains") && s.contains("FR167"));
}
