//! ATDD Story 77.2 / FR138 — Parser restore + AD-27 revise (P1–P4).
//!
//! cargo test -p bitloom --test fr138_parser_restore_ad27

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
fn fr138_p1_api_workflow_pairing() {
    let docs = read("docs/fr138-parser-restore.md");
    assert!(docs.contains("FR138") && docs.contains("Bitloom"));
    assert!(
        docs.contains("BitloomFirrtlParser.parse") || docs.contains("BitloomFirrtlParser"),
        "docs must name product-equivalent API BitloomFirrtlParser.parse"
    );
    assert!(
        docs.contains("Parser.parse") || docs.contains("firrtl.Parser"),
        "docs must contrast historical Parser.parse / firrtl.Parser"
    );
    assert!(
        docs.contains("7.15.0") && docs.contains("1.158.0"),
        "docs must pin Chisel 7.15.0 + firtool-1.158.0"
    );
    assert!(
        docs.contains("parse-only")
            || docs.contains("-parse-only")
            || docs.contains("工作流")
            || docs.contains("workflow"),
        "docs must document parse workflow"
    );

    let script = read("scripts/parser-restore-check.sh");
    assert!(
        script.contains("1.158.0"),
        "script must pin firtool 1.158.0"
    );
    assert!(
        script.contains("7.15.0"),
        "script must declare Chisel 7.15.0 pairing"
    );
    assert!(
        script.contains("parse-only") || script.contains("-parse-only"),
        "script must invoke firtool -parse-only"
    );
    assert!(
        script.contains("BitloomFirrtlParser") || script.contains("Parser.parse"),
        "script must name product API / historical equivalent"
    );
    assert!(
        script.contains("firtool ensure") || script.contains("RHDL_FIRTOOL_PATH"),
        "must resolve via AD-9 ensure or RHDL_FIRTOOL_PATH (≠ PATH random)"
    );

    let shim = root().join("crates/rhdl-firrtl/testdata/fr138_bitloom_firrtl_parser.scala");
    assert!(
        shim.is_file(),
        "missing Scala façade fr138_bitloom_firrtl_parser.scala"
    );
    let shim_txt = fs::read_to_string(&shim).unwrap();
    assert!(
        shim_txt.contains("BitloomFirrtlParser")
            && (shim_txt.contains("Parser.parse") || shim_txt.contains("firrtl.Parser")),
        "Scala façade must name BitloomFirrtlParser and historical Parser"
    );

    let fir = root().join("crates/rhdl-firrtl/fixtures/fr138_parser_restore.fir");
    assert!(fir.is_file(), "missing FR138 FIR fixture");
}

#[test]
fn fr138_p2_ad27_revised_and_correct_course() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(spine.contains("AD-27") && spine.contains("FR138"));
    assert!(
        spine.contains("BitloomFirrtlParser")
            || (spine.contains("Parser.parse")
                && (spine.contains("允许")
                    || spine.contains("产品关闭")
                    || spine.contains("FR138"))),
        "AD-27 must allow FR138 Parser / product-equivalent as close condition"
    );
    assert!(
        spine.contains("2026-09-11") && (spine.contains("FR138") || spine.contains("Phase 16")),
        "AD-27 must carry 2026-09-11 FR138 revise stamp"
    );
    // FR130 Style Guide historical face must remain mentionable (NFR56)
    assert!(
        spine.contains("FR130")
            && (spine.contains("不恢复")
                || spine.contains("not restored")
                || spine.contains("仍禁止")
                || spine.contains("Style Guide")),
        "AD-27 must preserve FR130 Style Guide / historical non-restore wording"
    );

    let docs = read("docs/fr138-parser-restore.md");
    assert!(
        docs.contains("correctCoursePhase16Approved")
            || docs.contains("2026-09-11")
            || docs.contains("Correct Course"),
        "docs must cite Correct Course Phase 16 trail"
    );
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("correctCoursePhase16Approved") || sprint.contains("2026-09-11"),
        "sprint / gate trail for Correct Course must exist"
    );
}

#[test]
fn fr138_p3_force_missing_nonzero_readable() {
    let script = root().join("scripts/parser-restore-check.sh");
    assert!(script.is_file(), "missing scripts/parser-restore-check.sh");
    let out = Command::new("bash")
        .arg(&script)
        .env("BITLOOM_PARSER_FORCE_MISSING", "1")
        .current_dir(root())
        .output()
        .expect("spawn parser-restore-check");
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
            || err.contains("refusing silent")
            || err.contains("Parser"),
        "failure must be readable: {err}"
    );
}

#[test]
fn fr138_p3_version_mismatch_nonzero() {
    let script = root().join("scripts/parser-restore-check.sh");
    let tmp = root().join("target/fr138-fake-firtool");
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
        .env_remove("BITLOOM_PARSER_FORCE_MISSING")
        .current_dir(root())
        .output()
        .expect("spawn parser-restore-check");
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
fn fr138_p4_not_satisfied_by_fr130_or_prior_alone() {
    let docs = read("docs/fr138-parser-restore.md");
    assert!(docs.contains("FR138") && docs.contains("Bitloom"));
    assert!(
        docs.contains("FR130")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("不得")),
        "must boundary vs FR130 alone"
    );
    for prior in ["FR122", "FR111", "FR97"] {
        assert!(docs.contains(prior), "docs must mention {prior} boundary");
    }
    assert!(
        docs.contains("docs-only") || docs.contains("≠ docs") || docs.contains("不得 docs"),
        "must ban docs-only close"
    );
    let script = read("scripts/parser-restore-check.sh");
    assert!(
        script.contains("firtool")
            && (script.contains("parse-only") || script.contains("-parse-only")),
        "gate must run firtool -parse-only, not docs-only"
    );
}

#[test]
fn fr138_docs_and_just_path() {
    let docs = read("docs/fr138-parser-restore.md");
    assert!(
        docs.contains("just parser-restore-check") || docs.contains("parser-restore-check"),
        "must document just / script path"
    );
    assert!(
        docs.contains("NFR58") || docs.contains("AD-27"),
        "must cite AD-27 / NFR58"
    );
    let just = read("Justfile");
    assert!(
        just.contains("parser-restore-check"),
        "Justfile must document just parser-restore-check"
    );
    assert!(
        just.contains("scripts/parser-restore-check.sh"),
        "just target must invoke the gate script"
    );
}

#[test]
fn fr138_script_never_silent_skip() {
    let script = read("scripts/parser-restore-check.sh");
    assert!(
        !script.contains("skipping smoke") && !script.contains("skipping"),
        "FR138 gate must not silent-skip"
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
