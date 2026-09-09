//! ATDD: Story 30.4 multi-view closure docs close-out / UJ「桥接半程」.
//!
//! Verifies user docs name the follow-along fixtures, terminology table,
//! and Epic 27/28 cross-links — without inventing new prelude APIs.
//!
//! ```text
//! cargo test -p bitloom --test fr78_bridge_half_followalong
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn fr78_bridge_half_tutorial_names_fixtures() {
    let root = workspace_root();
    let tutorial = fs::read_to_string(root.join("docs/tutorials/bridge-half.md"))
        .expect("docs/tutorials/bridge-half.md must exist for UJ 桥接半程");

    assert!(
        tutorial.contains("桥接半程") || tutorial.contains("bridge-half"),
        "tutorial must identify 桥接半程"
    );
    assert!(
        tutorial.contains("fr78_bridge_adapter_start_wait_complete"),
        "tutorial must point at Story 30.2 fixture"
    );
    assert!(
        tutorial.contains("fr78_fr47_dual_view_coverify"),
        "tutorial must point at Story 30.3 coverify fixture"
    );
    assert!(
        tutorial.contains("start_wait_complete"),
        "tutorial must name the FR78 template"
    );

    assert!(
        root.join("crates/bitloom/tests/fr78_bridge_adapter_start_wait_complete.rs")
            .is_file(),
        "follow-along fixture A must exist on disk"
    );
    assert!(
        root.join("crates/bitloom/tests/fr78_fr47_dual_view_coverify.rs")
            .is_file(),
        "follow-along fixture B must exist on disk"
    );
}

#[test]
fn fr78_docs_terminology_table_four_rows() {
    let root = workspace_root();
    let fr78 = fs::read_to_string(root.join("docs/fr78-bridge-adapter-closures.md"))
        .expect("docs/fr78-bridge-adapter-closures.md");

    assert!(
        fr78.contains("生成器闭包") && fr78.contains("FR73"),
        "terminology table must include 生成器闭包 / FR73"
    );
    assert!(
        fr78.contains("FR47") && fr78.contains("sim generators"),
        "terminology table must include FR47 sim generators"
    );
    assert!(
        fr78.contains("Phase 7") && (fr78.contains("闭环") || fr78.contains("closed")),
        "terminology table must include Phase 7 闭环"
    );
    assert!(
        fr78.contains("FR78") && (fr78.contains("本模板") || fr78.contains("start_wait_complete")),
        "terminology table must include 本模板 / FR78"
    );
    assert!(
        fr78.contains("tutorials/bridge-half.md") || fr78.contains("桥接半程"),
        "fr78 doc must link UJ 桥接半程"
    );
}

#[test]
fn fr78_docs_cross_link_epic27_28_and_readme() {
    let root = workspace_root();
    let fr78 =
        fs::read_to_string(root.join("docs/fr78-bridge-adapter-closures.md")).expect("fr78 doc");
    assert!(
        fr78.contains("fr22-construct-bar.md")
            || (fr78.contains("Epic 27") && fr78.contains("Epic 28")),
        "fr78 must cross-link Epic 27/28 generator/synthesizable closures"
    );
    assert!(
        fr78.contains("FR74") || fr78.contains("SynthesizableClosure"),
        "fr78 must mention synthesizable closures (Epic 28)"
    );

    let readme = fs::read_to_string(root.join("README.md")).expect("README");
    assert!(
        readme.contains("FR78") && readme.contains("bridge-half.md"),
        "README must index FR78 + 桥接半程 tutorial"
    );

    let surface =
        fs::read_to_string(root.join("_agile-output/specs/spec-rhdl/language-surface.md"))
            .expect("language-surface");
    assert!(
        surface.contains("30.4") || surface.contains("fr78_bridge_half_followalong"),
        "language-surface must mark Story 30.4 docs close-out"
    );

    let nfr14 = fs::read_to_string(root.join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic30-bridge-adapter-closures.md",
    ))
    .expect("nfr14 epic30");
    assert!(
        nfr14.contains("- [x] **FR78：")
            && nfr14.contains("- [x] **视图边界：")
            && nfr14.contains("- [x] **FR47 联验：")
            && nfr14.contains("- [x] **NFR36 / FR16：")
            && nfr14.contains("- [x] **禁止事项未触发："),
        "NFR14 Epic 30 close checklist must be fully ticked"
    );
}
