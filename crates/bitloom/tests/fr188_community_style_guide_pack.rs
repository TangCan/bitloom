//! ATDD Story 121.2 / FR188 — community Style Guide pack beyond FR181.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_firrtl::{
    check_chisel_style_guide_pack_fr188, check_chisel_style_linter_fr181,
    emit_chisel_style_guide_pack_fr188, emit_chisel_style_linter_fr181,
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn hierarchy_fixture() -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new("t");
    s.begin_module("Child", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("y", "x", Span::default());
    s.end_process();
    s.end_module();

    s.begin_module("Top", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.add_instance(
        "u0",
        "Child",
        vec![
            ("clk".into(), "clk".into()),
            ("rst".into(), "rst".into()),
            ("x".into(), "x".into()),
            ("y".into(), "y".into()),
        ],
        vec![],
        Span::default(),
    );
    s.end_module();
    s.finish().unwrap()
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr188_docs_contract() {
    let docs = read("docs/fr188-community-style-guide-pack.md");
    assert!(docs.contains("FR188") && docs.contains("Bitloom"));
    assert!(docs.contains("FR181"));
    assert!(docs.contains("emit_chisel_style_guide_pack_fr188"));
    assert!(docs.contains("chisel-style-guide-pack-check"));
    assert!(
        docs.contains("AD-27") && (docs.contains("revised") || docs.contains("NFR90")),
        "must state AD-27 revised for FR188"
    );
    assert!(
        (docs.contains("alone") || docs.contains("≠")) && docs.contains("FR181"),
        "must forbid FR181 alone"
    );
    assert!(
        docs.contains("NFR91") || docs.contains("IDE") || docs.contains("全家桶"),
        "must leave fuller suite as NFR91"
    );
}

#[test]
fn fr188_spine_ad27_revised() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("FR188")
            && spine.contains("chisel-community-style-guide")
            && spine.contains("scalafmt-community")
            && spine.contains("2026-09-14"),
        "AD-27 must document FR188 pack"
    );
    assert!(
        spine.contains("NFR90") && spine.contains("FR188"),
        "must cite NFR90 / FR188 revise"
    );
}

#[test]
fn fr188_emit_and_check() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_style_guide_pack_fr188(&hir).expect("FR188 emit");
    let scala = &art.files[0].contents;
    assert!(scala.contains("FR188 community Style Guide"));
    assert!(scala.contains("chisel-community-style-guide") && scala.contains("scalafmt-community"));
    assert!(scala.contains("--- FR188 style-guide-pack ---"));
    assert!(scala.contains("FR181 Style Guide / linter"));
    check_chisel_style_guide_pack_fr188(scala, &hir).expect("FR188 check");
    check_chisel_style_linter_fr181(scala, &hir).expect("FR188 is FR181 superset");
}

#[test]
fn fr188_fr181_alone_fails_pack_check() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_style_linter_fr181(&hir).unwrap();
    check_chisel_style_linter_fr181(&art.files[0].contents, &hir).unwrap();
    let err =
        check_chisel_style_guide_pack_fr188(&art.files[0].contents, &hir).expect_err("not FR188");
    let msg = format!("{err}");
    assert!(msg.contains("FR188"));
}

#[test]
fn fr188_script_just_ci() {
    let just = read("Justfile");
    assert!(just.contains("chisel-style-guide-pack-check"));
    assert!(just.contains("chisel-style-guide-pack-check.sh"));
    let script = root().join("scripts/chisel-style-guide-pack-check.sh");
    assert!(script.is_file());
    let script_txt = read("scripts/chisel-style-guide-pack-check.sh");
    assert!(script_txt.contains("BITLOOM_STYLE_GUIDE_PACK_FORCE_MISSING"));
    assert!(script_txt.contains("chisel-style-linter-deepen-check"));
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("chisel-style-guide-pack:"),
        "must define required chisel-style-guide-pack job"
    );
    let idx = ci.find("chisel-style-guide-pack:").expect("job");
    let block = &ci[idx..idx.saturating_add(600).min(ci.len())];
    assert!(
        block.contains("chisel-style-guide-pack-check"),
        "CI must run chisel-style-guide-pack-check"
    );
    assert!(!block.contains("continue-on-error"));
}

#[test]
fn fr188_force_missing_nonzero() {
    let out = Command::new("bash")
        .arg(root().join("scripts/chisel-style-guide-pack-check.sh"))
        .current_dir(root())
        .env("BITLOOM_STYLE_GUIDE_PACK_FORCE_MISSING", "1")
        .output()
        .expect("run force-missing");
    assert!(
        !out.status.success(),
        "FORCE_MISSING must be non-zero; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn fr188_live_gate() {
    let out = Command::new("bash")
        .arg(root().join("scripts/chisel-style-guide-pack-check.sh"))
        .current_dir(root())
        .env_remove("BITLOOM_STYLE_GUIDE_PACK_FORCE_MISSING")
        .output()
        .expect("run live gate");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "live FR188 gate must pass; stdout={stdout} stderr={stderr}"
    );
}
