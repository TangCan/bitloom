//! ATDD: Epic 58 NFR14 for FR117 Tywaves-level typed IDE waveform (Story 58.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic58_tywaves_typed_ide_waveform_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic58-tywaves-typed-ide-waveform.md",
    );
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR117"));

    assert!(
        text.contains("选定") || text.contains("钉死") || text.contains("Selected"),
        "must nail selected deepen subset"
    );
    assert!(
        text.contains("(B)")
            && (text.contains("自研") || text.contains("等价") || text.contains("typed IDE")),
        "Story 58.1 must select branch B in-house equivalent typed IDE waveform as FR117 face"
    );
    assert!(
        (text.contains("deferred") || text.contains("未选"))
            && (text.contains("(A)") || text.contains("Tywaves")),
        "unselected A Tywaves first-class must stay deferred"
    );
    assert!(
        text.contains("FR104")
            && (text.contains("I1") || text.contains("interactive") || text.contains("alone")),
        "must ban FR104 I1–I3 alone"
    );
    assert!(
        (text.contains("GTKWave") || text.contains("VCD") || text.contains("Surfer"))
            && (text.contains("不得") || text.contains("alone") || text.contains("≠")),
        "must ban VCD/GTKWave alone"
    );
    assert!(
        text.contains("FR114")
            && (text.contains("LCOV") || text.contains("coverage") || text.contains("覆盖率"))
            && (text.contains("不得") || text.contains("alone") || text.contains("≠")),
        "must ban FR114 LCOV/coverage.html alone"
    );
    assert!(
        text.contains("docs-only")
            || text.contains("仅改文档")
            || (text.contains("文档") && text.contains("不得")),
        "must ban docs-only close"
    );
    assert!(
        text.contains("58.2")
            && text.contains("58.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR51"));
    assert!(text.contains("NFR48"));
    assert!(text.contains("NFR14-crates"));
    assert!(
        text.contains("证明义务") || text.contains("夹具") || text.contains("工具"),
        "must document proof/fixture/tool obligations"
    );
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(
        text.contains("bitloom-prelude") || text.contains("bitloom_prelude"),
        "must cite bitloom-prelude design dependency boundary"
    );
}
