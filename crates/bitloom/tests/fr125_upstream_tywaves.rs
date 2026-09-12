//! ATDD — Story 65.2 / FR125: upstream Tywaves first-class path (T1–T4).
//!
//! ```text
//! cargo test -p bitloom --test fr125_upstream_tywaves
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
fn fr125_docs_and_nfr14_t1_t4() {
    let docs = read("docs/fr125-upstream-tywaves.md");
    assert!(docs.contains("FR125") && docs.contains("Bitloom"));
    assert!(docs.contains("wave.tywaves.json") && docs.contains("--tywaves"));
    assert!(docs.contains("BITLOOM_TYWAVES_BIN"));
    assert!(docs.contains("FR117") && (docs.contains("alone") || docs.contains("≠")));
    let nfr = read("_agile-output/implementation-artifacts/nfr14-risk-epic65-upstream-tywaves.md");
    assert!(nfr.contains("T1") && nfr.contains("T4"));
}

#[test]
fn fr125_wave_tywaves_writes_sidecar_and_keeps_fr117() {
    let out = tempfile_dir("fr125-ok");
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
        .arg("4")
        .arg("--tywaves")
        .env("BITLOOM_TYWAVES_BIN", &stub)
        .env_remove("BITLOOM_TYWAVES_FORCE_MISSING")
        .status()
        .expect("spawn cargo bitloom wave --tywaves");
    assert!(status.success(), "wave --tywaves with stub must succeed");

    let tywaves = fs::read_to_string(out.join("wave.tywaves.json")).expect("wave.tywaves.json");
    assert!(tywaves.contains("data-bitloom-tywaves"));
    assert!(tywaves.contains("\"fr\": \"FR125\""));
    assert!(tywaves.contains("schemaVersion"));
    assert!(tywaves.contains("\"viewer\": \"tywaves\""));
    assert!(tywaves.contains("\"ty\""));
    assert!(out.join("tywaves.launch.sh").is_file());
    // FR117 / FR104 artifacts still present (NFR52)
    assert!(out.join("typed-wave.html").is_file());
    assert!(out.join("wave.typed.json").is_file());
    assert!(out.join("interactive.html").is_file());
}

#[test]
fn fr125_force_missing_is_nonzero_readable() {
    let out = tempfile_dir("fr125-miss");
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    let output = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("2")
        .arg("--tywaves")
        .env("BITLOOM_TYWAVES_FORCE_MISSING", "1")
        .env_remove("BITLOOM_TYWAVES_BIN")
        .output()
        .expect("spawn");
    assert!(!output.status.success(), "FORCE_MISSING must be non-zero");
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("bitloom.tywaves"),
        "failure must be readable with bitloom.tywaves: {err}"
    );
    // Sidecar still written before fail (T1)
    assert!(
        out.join("wave.tywaves.json").is_file(),
        "sidecar must be written even when live-open fails"
    );
}

#[test]
fn fr125_not_satisfied_by_fr117_alone() {
    let out = tempfile_dir("fr125-no-flag");
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    let status = cargo_bitloom()
        .arg("wave")
        .arg("--input")
        .arg(&fir)
        .arg("--out-dir")
        .arg(&out)
        .arg("--ticks")
        .arg("2")
        // FR162 makes GUI/sidecar default; opt out to prove FR117 alone ≠ FR125.
        .arg("--no-tywaves-gui")
        .env_remove("BITLOOM_TYWAVES_FORCE_MISSING")
        .status()
        .expect("spawn");
    assert!(status.success());
    assert!(out.join("typed-wave.html").is_file());
    assert!(
        !out.join("wave.tywaves.json").is_file(),
        "with --no-tywaves-gui and without --tywaves must not emit FR125 sidecar (FR117 alone ≠ FR125)"
    );
}
