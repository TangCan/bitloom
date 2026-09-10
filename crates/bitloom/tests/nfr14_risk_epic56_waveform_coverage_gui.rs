//! ATDD: Epic 56 NFR14 for FR114 waveform/coverage GUI deepen (Story 56.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic56_waveform_coverage_gui_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic56-waveform-coverage-gui.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR114"));

    assert!(
        text.contains("选定") || text.contains("钉死") || text.contains("Selected"),
        "must nail selected deepen subset"
    );
    assert!(
        text.contains("(B)")
            && (text.contains("LCOV") || text.contains("覆盖率 GUI") || text.contains("coverage")),
        "Story 56.1 must select branch B LCOV/coverage GUI as FR114 face"
    );
    assert!(
        (text.contains("deferred") || text.contains("未选"))
            && (text.contains("(A)") || text.contains("Tywaves")),
        "unselected A Tywaves must stay deferred"
    );
    assert!(
        (text.contains("GTKWave") || text.contains("timing") || text.contains("VCD"))
            && (text.contains("不得") || text.contains("alone") || text.contains("≠")),
        "must ban timing/VCD/GTKWave alone"
    );
    assert!(
        text.contains("FR104")
            && (text.contains("I1") || text.contains("interactive") || text.contains("alone")),
        "must ban FR104 I1–I3 alone"
    );
    assert!(
        text.contains("FR105")
            && (text.contains("Mux") || text.contains("alone") || text.contains("≠")),
        "must ban FR105 Mux v2 alone"
    );
    assert!(
        text.contains("docs-only")
            || text.contains("仅改文档")
            || (text.contains("文档") && text.contains("不得")),
        "must ban docs-only close"
    );
    assert!(
        text.contains("56.2")
            && text.contains("56.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR47"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("FR104") && (text.contains("仍") || text.contains("不得")));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(
        text.contains("证明义务") || text.contains("夹具") || text.contains("工具"),
        "must document proof/fixture/tool obligations"
    );
}
