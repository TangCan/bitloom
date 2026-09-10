//! ATDD: Epic 54 NFR14 for FR112 formal/dual-model depth (Story 54.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic54_formal_dual_model_depth_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic54-formal-dual-model-depth.md",
    );
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR112"));
    // Must nail one of A/B/C
    assert!(
        text.contains("选定") || text.contains("钉死") || text.contains("Selected"),
        "must nail selected deepen branch"
    );
    assert!(
        text.contains("MemRead")
            || text.contains("GeneratedFunctional")
            || text.contains("SymbiYosys")
            || text.contains("手写 FL"),
        "must name a deepen path class"
    );
    assert!(
        text.contains("(B)") && (text.contains("选定") || text.contains("完成面")),
        "Story 54.1 must select branch B as FR112 face"
    );
    assert!(
        (text.contains("deferred") || text.contains("未选"))
            && (text.contains("(A)") || text.contains("SymbiYosys"))
            && (text.contains("(C)") || text.contains("手写")),
        "unselected A/C must stay deferred"
    );
    assert!(
        text.contains("FR92")
            && (text.contains("不得") || text.contains("alone") || text.contains("≠")),
        "must ban FR92 alone"
    );
    assert!(
        (text.contains("adapter") || text.contains("模板") || text.contains("FR78"))
            && (text.contains("不得") || text.contains("alone")),
        "must ban adapter-template alone"
    );
    assert!(
        text.contains("FR107") || text.contains("SystemC") || text.contains("AT"),
        "must isolate ≠ SystemC AT"
    );
    assert!(
        text.contains("54.2")
            && text.contains("54.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR47"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("FR100") && (text.contains("仍") || text.contains("不得")));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(
        text.contains("证明义务") || text.contains("夹具") || text.contains("工具"),
        "must document proof/fixture/tool obligations"
    );
}
