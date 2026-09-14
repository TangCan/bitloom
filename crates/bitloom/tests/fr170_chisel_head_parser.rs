//! ATDD Story 103.2 / FR170 — Chisel update-mainline / HEAD Parser migration.
//!
//! ```text
//! cargo test -p bitloom --test fr170_chisel_head_parser
//! ```

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
fn fr170_docs_contract_forbid_fr138_alone() {
    let docs = read("docs/fr170-chisel-head-parser.md");
    assert!(docs.contains("FR170") && docs.contains("Bitloom"));
    assert!(
        docs.contains("update-mainline") || docs.contains("parseUpdateMainline"),
        "must name update-mainline Parser path"
    );
    assert!(
        docs.contains("6.0.0") && (docs.contains("FIRRTL") || docs.contains("version")),
        "must declare FIRRTL 6.0.0 migration delta"
    );
    assert!(
        docs.contains("FR138")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR138 distinct"
    );
    assert!(
        docs.contains("FR165")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR165 distinct"
    );
    assert!(docs.contains("7.15.0") && docs.contains("1.159.0"));
    assert!(
        docs.contains("parser-head-migration") || docs.contains("parser-head-migration-check"),
        "must name migration check path / CI"
    );
    assert!(
        docs.contains("AD-27")
            && (docs.contains("revised") || docs.contains("修订") || docs.contains("2026-09-12")),
        "must require AD-27 revise"
    );
    assert!(
        (docs.contains("AD-9") || docs.contains("firtool"))
            && (docs.contains("NFR76")
                || docs.contains("unchanged")
                || docs.contains("不")
                || docs.contains("Deferred")
                || docs.contains("deferred")),
        "must keep AD-9 / unpaired HEAD bump honesty"
    );
}

#[test]
fn fr170_script_and_just_path() {
    let script = read("scripts/parser-head-migration-check.sh");
    assert!(script.contains("1.159.0") && script.contains("7.15.0"));
    assert!(script.contains("FR170"));
    assert!(
        script.contains("parseUpdateMainline") || script.contains("update-mainline"),
        "must name update-mainline API"
    );
    assert!(
        script.contains("BITLOOM_PARSER_HEAD_FORCE_MISSING"),
        "must honor FORCE_MISSING"
    );
    assert!(
        script.contains("6.0.0") && script.contains("-parse-only"),
        "must parse FIRRTL 6.0.0 via -parse-only"
    );
    let just = read("Justfile");
    assert!(just.contains("parser-head-migration-check"));
    assert!(just.contains("scripts/parser-head-migration-check.sh"));
}

#[test]
fn fr170_ci_required_job_no_continue_on_error() {
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("parser-head-migration:"),
        "must define required parser-head-migration job"
    );
    let idx = ci
        .find("parser-head-migration:")
        .expect("parser-head-migration");
    let block: String = ci[idx..].lines().take(40).collect::<Vec<_>>().join("\n");
    assert!(
        !block.contains("continue-on-error"),
        "FR170 required job must not continue-on-error"
    );
    assert!(
        block.contains("parser-head-migration-check"),
        "CI must run parser-head-migration-check"
    );
}

#[test]
fn fr170_force_missing_nonzero_readable() {
    let script = root().join("scripts/parser-head-migration-check.sh");
    assert!(script.is_file());
    let out = Command::new("bash")
        .arg(&script)
        .env("BITLOOM_PARSER_HEAD_FORCE_MISSING", "1")
        .current_dir(root())
        .output()
        .expect("spawn");
    assert!(!out.status.success(), "FORCE_MISSING must fail non-zero");
    let err = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        err.contains("FORCE_MISSING")
            || err.contains("unavailable")
            || err.contains("missing")
            || err.contains("refusing silent"),
        "failure must be readable: {err}"
    );
}

#[test]
fn fr170_not_satisfied_by_fr138_or_fr165_alone() {
    let fr138 = read("scripts/parser-restore-check.sh");
    let fr170 = read("scripts/parser-head-migration-check.sh");
    assert!(fr138.contains("FR138"));
    assert!(
        fr170.contains("FR170")
            && fr170.contains("6.0.0")
            && !fr170.contains("fr138_parser_restore.fir"),
        "FR170 must be a distinct FIRRTL 6.0.0 path, not FR138 re-run"
    );
    let fr165 = read("docs/fr165-deeper-chisel-parser-ecosystem.md");
    assert!(
        fr165.contains("HEAD") && (fr165.contains("deferred") || fr165.contains("NFR71")),
        "FR165 historical deferral honesty must remain"
    );
}

#[test]
fn fr170_ad27_revised_for_fr170() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("FR170")
            && (spine.contains("parseUpdateMainline")
                || spine.contains("update-mainline")
                || spine.contains("更新主线")),
        "AD-27 / spine must authorize FR170 update-mainline Parser"
    );
    assert!(
        spine.contains("2026-09-12") && spine.contains("AD-27"),
        "AD-27 must show 2026-09-12 revise for FR170"
    );
}

#[test]
fn fr170_scala_facade_and_fixture() {
    let scala = read("crates/rhdl-firrtl/testdata/fr170_bitloom_firrtl_parser_mainline.scala");
    assert!(
        scala.contains("parseUpdateMainline") && scala.contains("BitloomFirrtlParser"),
        "Scala façade must name parseUpdateMainline"
    );
    let fir = root().join("crates/rhdl-firrtl/fixtures/fr170_chisel_head_parser.fir");
    assert!(fir.is_file());
    let fir_txt = fs::read_to_string(&fir).unwrap();
    assert!(
        fir_txt.contains("FIRRTL version 6.0.0"),
        "fixture must be FIRRTL 6.0.0"
    );
}

#[test]
fn fr170_live_migration_check_when_firtool_available() {
    let script = root().join("scripts/parser-head-migration-check.sh");
    let out = Command::new("bash")
        .arg(&script)
        .env_remove("BITLOOM_PARSER_HEAD_FORCE_MISSING")
        .current_dir(root())
        .output()
        .expect("spawn live head migration");
    assert!(
        out.status.success(),
        "live parser-head-migration-check must pass; stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stamp = root().join("target/parser-head-migration-check/fr170_head_parser.parse-ok");
    assert!(stamp.is_file(), "must write parse-ok stamp");
}
