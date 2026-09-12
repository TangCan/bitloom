//! ATDD / guardrail: Epic 105 NFR14 risk record for Phase 21
//! NFR76 leftovers / FR172 (Story 105.1 / AD-28 / NFR78–82).
//! Red if file missing, epic already closed, or 105.2+ ready before 105.1 done.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn yaml_value_for_key(sprint: &str, key: &str) -> Option<String> {
    let needle = format!("{key}:");
    for line in sprint.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(&needle) {
            return Some(rest.trim().to_string());
        }
    }
    None
}

#[test]
fn nfr14_risk_epic105_phase21_nfr76_leftovers_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic105-phase21-nfr76-leftovers.md",
    );
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(
        text.contains("上游约束") && (text.contains("(a)") || text.contains("（a）")),
        "risk record must include labeled field (a) 上游约束"
    );
    assert!(
        text.contains("粗工期带") && (text.contains("(b)") || text.contains("（b）")),
        "risk record must include labeled field (b) 粗工期带"
    );
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级"))
            && (text.contains("(c)") || text.contains("（c）")),
        "risk record must include labeled field (c) 禁止的静默降级清单"
    );
    assert!(
        text.contains("负责人") && (text.contains("(d)") || text.contains("（d）")),
        "risk record must include labeled field (d) 负责人"
    );

    let status_open = text.contains("状态")
        && (text.contains("open") || text.contains("in-progress") || text.contains("进行中"))
        && !text.contains("closed — Story 105.4")
        && !text.contains("closed — Epic 105");
    let status_closed = text.contains("closed — Story 105.4")
        || (text.contains("closed") && text.contains("Epic 105") && text.contains("闸门已开"));
    assert!(
        status_open || status_closed,
        "Story 105.1 risk record status must be open/in-progress or closed after Story 105.4"
    );

    let sprint_early = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint-status.yaml");
    let epic105_done =
        sprint_early.contains("epic-105: done") || sprint_early.contains("epic-105:done");
    if !epic105_done {
        assert!(
            status_open,
            "before epic-105 done, risk record must remain open/in-progress"
        );
    }

    assert!(
        text.contains("NFR78")
            && (text.contains("Phase 12") || text.contains("Phase 20") || text.contains("FR171"))
            && (text.contains("不得") || text.contains("保留") || text.contains("仍有效")),
        "risk record must state NFR78 Phase 12–20 isolation"
    );
    assert!(
        text.contains("已关闭")
            && (text.contains("FR94") || text.contains("FR171") || text.contains("Phase 20"))
            && (text.contains("失败") || text.contains("改写") || text.contains("不得")),
        "risk record must forbid rewriting FR94–171 closed as failed"
    );

    for fr in ["FR173", "FR174", "FR175", "FR176", "FR177"] {
        assert!(text.contains(fr), "risk record must summarize {fr}");
    }

    assert!(
        text.contains("Q1") && text.contains("FR173") && text.contains("FR176"),
        "risk record must document Q1 NFR76 four leftovers FR173–176"
    );
    assert!(
        text.contains("Q2")
            && text.contains("FR173")
            && (text.contains("AD-9") || text.contains("firtool") || text.contains("配对")),
        "risk record must document Q2 FR173 firtool bump + AD-9 pairing"
    );
    assert!(
        text.contains("Q3") && text.contains("NFR78"),
        "risk record must document Q3 NFR78 no rewrite"
    );
    assert!(
        text.contains("Q4") && (text.contains("FR142") || text.contains("FR177")),
        "risk record must document Q4 no silent FR142 expand / FR177 claims"
    );
    assert!(
        text.contains("Q5")
            && (text.contains("MSRV")
                || text.contains("NFR81")
                || text.contains("AD-9")
                || text.contains("FR172")),
        "risk record must document Q5 MSRV / NFR81 / FR172 gate"
    );

    assert!(
        (text.contains("106") || text.contains("106–110") || text.contains("106-110"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("缺"))
            && (text.contains("FR172") || text.contains("Epic 105") || text.contains("ready")),
        "risk record must forbid opening Epic 106–110 before FR172 / Epic 105 gate"
    );

    assert!(
        text.contains("FR142")
            && (text.contains("静默") || text.contains("扩大") || text.contains("不得")),
        "risk record must forbid silent FR142 expansion"
    );
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不是 FR")),
        "risk record must state git push is not an FR"
    );
    assert!(
        text.contains("NFR81")
            && (text.contains("子集") || text.contains("另开") || text.contains("静默")),
        "risk record must cite NFR81 subset / no silent expand"
    );

    assert!(
        text.contains("105.2")
            && text.contains("105.3")
            && text.contains("105.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 105.2–105.4 from ready without this record"
    );

    assert!(text.contains("FR172"), "risk record must cite FR172");
    assert!(
        text.contains("Epic 105") || text.contains("Epic105"),
        "risk record must name Epic 105"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR78") && text.contains("负责人"),
        "risk record must assign NFR78 ownership alongside NFR14"
    );
    assert!(
        (text.contains("NFR79") || text.contains("NFR81") || text.contains("NFR82"))
            && text.contains("负责人"),
        "risk record must assign NFR78–82 ownership"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        text.contains("bitloom-prelude") || text.contains("bitloom_prelude"),
        "risk record must cite bitloom-prelude design dependency boundary"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
    assert!(
        text.contains("105")
            && text.contains("110")
            && (text.contains("软序") || text.contains("→")),
        "risk record must document soft order involving 105…110"
    );

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint-status.yaml");
    let s105_1 = yaml_value_for_key(&sprint, "105-1-epic-105-nfr14-风险记录")
        .expect("105-1 key in sprint-status");
    let later = [
        "105-2-correct-course-prd-批准-phase-21-fr172",
        "105-3-同步-readme-deferred-路线图指针-fr172",
        "105-4-ad-指针与-epic-105-收口-fr172",
    ];
    if s105_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 105-1 is done (got {status})"
            );
        }
    }

    assert!(
        sprint.contains("epic-104: done") || sprint.contains("epic-104:done"),
        "Phase 20 Epic 104 must be done before Epic 105 NFR14"
    );
    let epic105_done_gate = sprint.contains("epic-105: done") || sprint.contains("epic-105:done");
    if !epic105_done_gate {
        for line in sprint.lines() {
            let t = line.trim();
            if t.starts_with("106-")
                || t.starts_with("107-")
                || t.starts_with("108-")
                || t.starts_with("109-")
                || t.starts_with("110-")
            {
                if t.contains("ready-for-dev") {
                    panic!(
                        "Epic 106–110 stories must not be ready-for-dev before epic-105 done: {t}"
                    );
                }
            }
        }
        for epic in 106..=110 {
            let key = format!("epic-{epic}");
            let st = yaml_value_for_key(&sprint, &key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until epic-105 done (got {st})"
            );
        }
    }
}
