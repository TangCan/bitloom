//! ATDD / guardrail: NFR14 risk-record template must exist with required fields
//! (Story 19.1 / AD-28 / PRD NFR14).
//! Red if template missing; green after template lands.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_record_template_has_required_gate_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-record-template.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    // (a)–(d) mandatory NFR14 fields (heading or labeled)
    assert!(
        text.contains("上游约束") && (text.contains("(a)") || text.contains("（a）")),
        "template must include labeled field (a) 上游约束"
    );
    assert!(
        text.contains("粗工期带") && (text.contains("(b)") || text.contains("（b）")),
        "template must include labeled field (b) 粗工期带"
    );
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级"))
            && (text.contains("(c)") || text.contains("（c）")),
        "template must include labeled field (c) 禁止的静默降级清单"
    );
    assert!(
        text.contains("负责人") && (text.contains("(d)") || text.contains("（d）")),
        "template must include labeled field (d) 负责人"
    );

    // Disambiguate historical NFR14-crates (crates.io FCFS) from gate NFR14
    assert!(
        text.contains("NFR14-crates"),
        "template must name historical alias NFR14-crates"
    );
    assert!(
        text.contains("FCFS") || text.contains("crates.io"),
        "template must tie NFR14-crates to crates.io FCFS identity"
    );
    assert!(
        text.contains("本门禁") || text.contains("风险门禁"),
        "template must label current NFR14 as the risk gate (not crates FCFS)"
    );

    // Ready gate for FR46/47/48/49 (+ applicable FR50)
    for fr in ["FR46", "FR47", "FR48", "FR49", "FR50"] {
        assert!(
            text.contains(fr),
            "template must mention {fr} in the ready gate"
        );
    }
    assert!(
        text.contains("ready")
            && (text.contains("不得") || text.contains("缺记录") || text.contains("不得标")),
        "template must state missing record ⇒ must NOT mark epic/story ready"
    );

    // Parallel P3 / Chipyard-style maintenance overlay
    assert!(
        text.contains("Chipyard") && (text.contains("维护叠加") || text.contains("并行")),
        "template must note parallel P3 maintenance / Chipyard-style risk"
    );

    // Cite AD-28 / PRD NFR14
    assert!(text.contains("AD-28"), "template must cite AD-28");
    assert!(
        text.contains("PRD") && text.contains("NFR14"),
        "template must cite PRD NFR14"
    );
}
