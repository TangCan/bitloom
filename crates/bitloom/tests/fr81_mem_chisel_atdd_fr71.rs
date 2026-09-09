//! ATDD close-out: FR81 Path A Mem contract fixtures + FR71 regression (Story 33.4).
//!
//! Red if Path A emit ATDD missing, FR71 no-Mem golden/recipe weakened, Mem Path A
//! Scala fixture absent, Mem↔Chisel docs incomplete, or NFR14 Epic 33 close unticked.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn fr81_path_a_emit_atdd_source_still_covers_decision() {
    let path = workspace_root().join("crates/bitloom/tests/fr81_path_a_mem_chisel_emit.rs");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert!(
        text.contains("fr81_path_a_sync_read_mem_emits_chisel") && text.contains("SyncReadMem("),
        "Path A SyncReadMem emit ATDD must remain"
    );
    assert!(
        text.contains("fr81_path_a_async_mem_with_init_emits_chisel")
            && text.contains("Mem(")
            && text.contains("VecInit"),
        "Path A Mem+init emit ATDD must remain"
    );
    assert!(
        text.contains("fr81_path_a_out_of_subset_still_e0901") && text.contains("rhdl::E0901"),
        "OOS E0901 ATDD must remain"
    );
    assert!(
        text.contains("fr81_path_a_no_mem_counter_still_emits"),
        "no-Mem counter emit ATDD must remain (FR71 adjacency)"
    );
}

#[test]
fn fr71_no_mem_golden_and_just_recipe_unchanged_contract() {
    let root = workspace_root();
    let golden = root.join("crates/rhdl-firrtl/testdata/fr28_golden_counter.scala");
    let text =
        fs::read_to_string(&golden).unwrap_or_else(|e| panic!("read {}: {e}", golden.display()));
    assert!(
        text.contains("Fr28GoldenCounter")
            && text.contains("extends Module")
            && !text.contains("SyncReadMem")
            && !text.contains("Mem("),
        "FR71 golden must stay no-Mem counter"
    );

    let justfile = fs::read_to_string(root.join("Justfile")).expect("Justfile");
    assert!(
        justfile.contains("chisel-fr28-jvm:")
            && justfile.contains("fr28_golden_counter.scala")
            && justfile.contains("chisel-fr28-compile-required"),
        "just chisel-fr28-jvm must still target FR71 counter golden via required script"
    );

    let ci = fs::read_to_string(root.join(".github/workflows/ci.yml")).expect("ci.yml");
    assert!(
        ci.contains("fr28-chisel-jvm:") && ci.contains("fr28_golden_counter.scala"),
        "GHA fr28-chisel-jvm must keep required counter compile"
    );
    assert!(
        !ci.contains("continue-on-error:")
            && !ci
                .lines()
                .any(|l| l.contains("BITLOOM_CHISEL_JVM_SKIP") && !l.trim_start().starts_with('#')),
        "GHA must not set continue-on-error or BITLOOM_CHISEL_JVM_SKIP on the FR71 job"
    );
}

#[test]
fn fr81_path_a_mem_jvm_contract_fixture_exists() {
    let path = workspace_root().join("crates/rhdl-firrtl/testdata/fr81_path_a_sync_read_mem.scala");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert!(
        text.contains("SyncReadMem(4, UInt(8.W))")
            && text.contains("extends Module")
            && (text.contains("7.14.0") || text.contains("NFR12") || text.contains("AD-9")),
        "Mem Path A fixture must be SyncReadMem Module under NFR12 pin narrative:\n{text}"
    );
    assert!(
        text.contains("ram.write(") || text.contains(".write("),
        "fixture should exercise mem write for compile smoke:\n{text}"
    );
    assert!(
        text.contains(".read("),
        "fixture should exercise mem read:\n{text}"
    );

    let justfile = fs::read_to_string(workspace_root().join("Justfile")).expect("Justfile");
    assert!(
        justfile.contains("chisel-fr81-mem-jvm")
            && justfile.contains("fr81_path_a_sync_read_mem.scala"),
        "Justfile must offer optional Mem Path A JVM recipe (not replacing FR71 counter)"
    );
}

#[test]
fn fr81_mem_chisel_boundary_docs_updated() {
    let root = workspace_root();
    let fr28 = fs::read_to_string(root.join("docs/fr28-chisel-compilable.md")).expect("fr28 doc");
    assert!(
        fr28.contains("FR81")
            && (fr28.contains("Path A") || fr28.contains("子集"))
            && fr28.contains("E0901")
            && (fr28.contains("fr81_path_a_sync_read_mem") || fr28.contains("Mem Path A")),
        "fr28-chisel-compilable.md must document Mem Path A fixture + E0901 boundary"
    );
    assert!(
        fr28.contains("FR71")
            && fr28.contains("fr28_golden_counter")
            && fr28.contains("chisel-fr28-jvm"),
        "docs must keep FR71 counter / just chisel-fr28-jvm as required gate"
    );

    let surface =
        fs::read_to_string(root.join("_agile-output/specs/spec-rhdl/language-surface.md"))
            .expect("language-surface");
    assert!(
        surface.contains("FR81")
            && surface.contains("Path A")
            && surface.contains("E0901")
            && (surface.contains("33.4")
                || surface.contains("Epic 33")
                || surface.contains("Story 33")),
        "language-surface must lock Mem↔Chisel Path A / E0901 with Epic 33 close"
    );

    let readme = fs::read_to_string(root.join("README.md")).expect("README");
    assert!(
        readme.contains("FR81")
            || (readme.contains("FR28") && readme.contains("fr28-chisel-compilable")),
        "README must keep FR28/FR81 Chisel docs discoverable"
    );
}

#[test]
fn fr81_nfr14_epic33_close_checklist_ticked() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let close = text
        .split("### Epic 33 关闭条件")
        .nth(1)
        .expect("Epic 33 close section");
    for needle in ["FR81", "实现", "NFR12", "FR71", "NFR37", "禁止事项未触发"] {
        assert!(
            close.contains(needle),
            "close checklist must mention {needle}"
        );
    }
    let unchecked = close
        .lines()
        .filter(|l| l.trim_start().starts_with("- [ ]"))
        .count();
    assert_eq!(
        unchecked, 0,
        "Epic 33 close conditions must all be checked [x]; found {unchecked} unchecked:\n{close}"
    );
    assert!(
        close
            .lines()
            .filter(|l| l.trim_start().starts_with("- [x]"))
            .count()
            >= 6,
        "expected at least 6 ticked close conditions"
    );
    assert!(
        !close.contains("_待 Story 33.4_") && (close.contains("33.4") || close.contains("证据")),
        "NFR14 evidence line must be filled by Story 33.4"
    );
}

#[test]
fn fr71_script_contract_atdd_still_passes() {
    let root = workspace_root();
    let status = Command::new("bash")
        .arg(root.join("scripts/test-just-chisel-fr28-jvm.sh"))
        .current_dir(&root)
        .status()
        .expect("spawn test-just-chisel-fr28-jvm.sh");
    assert!(
        status.success(),
        "FR71 just/script ATDD must stay green (Story 33.4 regression)"
    );
}
