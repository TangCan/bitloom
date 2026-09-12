//! ATDD Story 76.2 / FR137 — external CIRCT compile gate (E1–E4).
//!
//! cargo test -p bitloom --test fr137_external_circt_compile_sim_gate

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr137_e1_firtool_version_channel_pinned() {
    let script = read("scripts/circt-external-check.sh");
    assert!(
        script.contains("1.158.0") || script.contains("FIRTOOL_VERSION"),
        "script must pin firtool 1.158.0"
    );
    assert!(
        script.contains("firtool-1.158.0")
            || script.contains("firrtl-bin-linux-x64")
            || script.contains("firtool ensure")
            || script.contains("RHDL_FIRTOOL"),
        "script must use AD-9 channel / ensure / override (≠ PATH random)"
    );
    assert!(
        !script.contains("CIRCT HEAD") || script.contains("≠") || script.contains("not CIRCT HEAD"),
        "must not adopt CIRCT HEAD"
    );
    // Must not treat bare PATH firtool as the success resolver
    assert!(
        script.contains("RHDL_FIRTOOL_PATH")
            || script.contains("firtool ensure")
            || script.contains("ensure_firtool"),
        "must resolve via AD-9 ensure or RHDL_FIRTOOL_PATH"
    );
    let docs = read("docs/fr137-external-circt-gate.md");
    assert!(docs.contains("1.158.0"));
    assert!(
        docs.contains("firrtl-bin-linux-x64") || docs.contains("firtool-1.158.0"),
        "docs must name release channel"
    );
    assert!(
        docs.contains("PATH")
            && (docs.contains("不信任") || docs.contains("never") || docs.contains("≠")),
        "docs must reject PATH-random firtool"
    );
}

#[test]
fn fr137_e2_ci_required_job_and_just_path() {
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("circt-external:"),
        "must define required circt-external job"
    );
    let idx = ci.find("circt-external:").expect("circt-external");
    let block: String = ci[idx..].lines().take(50).collect::<Vec<_>>().join("\n");
    assert!(
        !block.contains("continue-on-error"),
        "FR137 required job must not continue-on-error"
    );
    assert!(
        block.contains("circt-external-check"),
        "CI must run circt-external-check"
    );
    assert!(block.contains("timeout-minutes"));

    let just = read("Justfile");
    assert!(
        just.contains("circt-external-check"),
        "Justfile must document just circt-external-check"
    );
    assert!(
        just.contains("scripts/circt-external-check.sh"),
        "just target must invoke the gate script"
    );
}

#[test]
fn fr137_e3_force_missing_nonzero_readable() {
    let script = root().join("scripts/circt-external-check.sh");
    assert!(script.is_file(), "missing scripts/circt-external-check.sh");
    let out = Command::new("bash")
        .arg(&script)
        .env("BITLOOM_CIRCT_FORCE_MISSING", "1")
        .current_dir(root())
        .output()
        .expect("spawn circt-external-check");
    assert!(
        !out.status.success(),
        "FORCE_MISSING must fail non-zero; stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let err = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        err.to_lowercase().contains("missing")
            || err.contains("unavailable")
            || err.contains("FORCE_MISSING")
            || err.contains("refusing silent"),
        "failure must be readable: {err}"
    );
}

#[test]
fn fr137_e3_version_mismatch_nonzero() {
    let script = root().join("scripts/circt-external-check.sh");
    let tmp = root().join("target/fr137-fake-firtool");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    let fake = tmp.join("firtool");
    fs::write(
        &fake,
        "#!/usr/bin/env bash\necho 'CIRCT firtool version 9.99.0'\nexit 0\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&fake).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&fake, perms).unwrap();
    }
    let out = Command::new("bash")
        .arg(&script)
        .env("RHDL_FIRTOOL_PATH", &tmp)
        .env_remove("BITLOOM_CIRCT_FORCE_MISSING")
        .current_dir(root())
        .output()
        .expect("spawn circt-external-check");
    assert!(
        !out.status.success(),
        "version mismatch must fail; stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let err = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        err.contains("1.158.0")
            || err.to_lowercase().contains("version")
            || err.contains("mismatch")
            || err.contains("expected"),
        "mismatch failure must mention version: {err}"
    );
}

#[test]
fn fr137_e4_not_satisfied_by_fr129_or_prior_alone() {
    let docs = read("docs/fr137-external-circt-gate.md");
    assert!(docs.contains("FR137") && docs.contains("Bitloom"));
    assert!(
        docs.contains("FR129")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("不得")),
        "must boundary vs FR129 alone"
    );
    for prior in ["FR121", "FR110", "FR95", "FR96"] {
        assert!(docs.contains(prior), "docs must mention {prior} boundary");
    }
    assert!(
        docs.contains("docs-only") || docs.contains("≠ docs") || docs.contains("不得 docs"),
        "must ban docs-only close"
    );
    // Script must actually invoke firtool compile — not docs-only
    let script = read("scripts/circt-external-check.sh");
    assert!(
        script.contains("firtool")
            && (script.contains(".fir") || script.contains("FIR=") || script.contains("compile")),
        "gate must run external firtool compile, not docs-only"
    );
}

#[test]
fn fr137_docs_compile_gate_mvp_contract() {
    let docs = read("docs/fr137-external-circt-gate.md");
    assert!(
        docs.contains("编译") || docs.contains("compile"),
        "must document compile-gate selection"
    );
    assert!(
        docs.contains("仿真") || docs.contains("sim"),
        "must state sim selection (selected or deferred)"
    );
    assert!(
        docs.contains("circt-external") && docs.contains("just circt-external-check"),
        "must document CI job and just path"
    );
    assert!(
        docs.contains("NFR58") || docs.contains("AD-9"),
        "NFR58 / AD-9 ops sync pointer"
    );
}

#[test]
fn fr137_script_never_silent_skip() {
    let script = read("scripts/circt-external-check.sh");
    // Contrast with firtool-smoke which exits 0 when missing
    assert!(
        !script.contains("skipping smoke") && !script.contains("skipping"),
        "FR137 gate must not silent-skip like firtool-smoke"
    );
    assert!(
        script.contains("exit 1") || script.contains("exit 2"),
        "must have explicit non-zero exits"
    );
    assert!(
        script.contains("refusing silent") || script.contains("FORCE_MISSING"),
        "must refuse silent success"
    );
}
