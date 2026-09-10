//! ATDD: Epic 51 NFR14 for FR109 FSM/state-visit coverage (Story 51.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic51_fsm_state_visit_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic51-fsm-state-visit-coverage.md",
    );
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR109"));
    assert!(
        text.contains("FSM") || text.contains("state-visit") || text.contains("状态"),
        "must nail FSM / state-visit metric"
    );
    assert!(
        text.contains("报告") || text.contains("M2") || text.contains("格式"),
        "must nail report format"
    );
    assert!(
        text.contains("夹具") || text.contains("M3"),
        "must nail fixture scope"
    );
    assert!(
        text.contains("不得仅改文档")
            || (text.contains("文档") && text.contains("口号") && text.contains("不得")),
        "must ban docs-only / slogan close of FR109"
    );
    assert!(
        (text.contains("Mux") || text.contains("C2") || text.contains("FR105"))
            && (text.contains("不得") || text.contains("否")),
        "must ban Mux v2 alone as C3"
    );
    assert!(
        text.contains("FR34") && (text.contains("不得") || text.contains("否")),
        "must ban FR34 toggle alone as C3"
    );
    assert!(
        text.contains("51.2")
            && text.contains("51.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR47"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("FR105") && (text.contains("仍") || text.contains("不得")));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(
        text.contains("M1") && text.contains("M2") && text.contains("M3"),
        "must expose M1–M3 acceptance table"
    );
}
