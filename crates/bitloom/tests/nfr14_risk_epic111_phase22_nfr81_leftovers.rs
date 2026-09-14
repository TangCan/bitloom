//! ATDD / guardrail: Epic 111 NFR14 risk record for Phase 22
//! NFR81 leftovers / FR178 (Story 111.1 / AD-28 / NFR83–87).
//! Red if file missing, epic already closed, or 111.2+ ready before 111.1 done.

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
fn nfr14_risk_epic111_phase22_nfr81_leftovers_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic111-phase22-nfr81-leftovers.md",
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
        && !text.contains("closed — Story 111.4")
        && !text.contains("closed — Epic 111");
    let status_closed = text.contains("closed — Story 111.4")
        || (text.contains("closed") && text.contains("Epic 111") && text.contains("闸门已开"));
    assert!(
        status_open || status_closed,
        "Story 111.1 risk record status must be open/in-progress or closed after Story 111.4"
    );

    let sprint_early = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint-status.yaml");
    let epic111_done =
        sprint_early.contains("epic-111: done") || sprint_early.contains("epic-111:done");
    if !epic111_done {
        assert!(
            status_open,
            "before epic-111 done, risk record must remain open/in-progress"
        );
    }

    assert!(
        text.contains("NFR83")
            && (text.contains("Phase 12") || text.contains("Phase 21") || text.contains("FR177"))
            && (text.contains("不得") || text.contains("保留") || text.contains("仍有效")),
        "risk record must state NFR83 Phase 12–21 isolation"
    );
    assert!(
        text.contains("已关闭")
            && (text.contains("FR94") || text.contains("FR177") || text.contains("Phase 21"))
            && (text.contains("失败") || text.contains("改写") || text.contains("不得")),
        "risk record must forbid rewriting FR94–177 closed as failed"
    );

    for fr in ["FR179", "FR180", "FR181", "FR182", "FR183", "FR184"] {
        assert!(text.contains(fr), "risk record must summarize {fr}");
    }

    assert!(
        text.contains("Q1") && text.contains("FR179") && text.contains("FR183"),
        "risk record must document Q1 NFR81 five leftovers FR179–183"
    );
    assert!(
        text.contains("Q5") && text.contains("FR183") && text.contains("FR142"),
        "risk record must document Q5 FR142=independent FR183"
    );
    assert!(
        text.contains("Q7")
            && text.contains("FR182")
            && (text.contains("AD-9") || text.contains("unpaired") || text.contains("产品钉")),
        "risk record must document Q7 FR182 unpaired product-pin + AD-9"
    );
    assert!(
        text.contains("Q3") && text.contains("NFR83"),
        "risk record must document Q3 NFR83 no rewrite"
    );
    assert!(
        text.contains("Q4") && (text.contains("FR184") || text.contains("NFR87")),
        "risk record must document Q4 FR184 / NFR87 claims"
    );
    assert!(
        text.contains("Q6")
            && (text.contains("MSRV") || text.contains("NFR86") || text.contains("FR178")),
        "risk record must document Q6 MSRV / NFR86 / FR178 gate"
    );

    assert!(
        (text.contains("112") || text.contains("112–117") || text.contains("112-117"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("缺"))
            && (text.contains("FR178") || text.contains("Epic 111") || text.contains("ready")),
        "risk record must forbid opening Epic 112–117 before FR178 / Epic 111 gate"
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
        text.contains("NFR86")
            && (text.contains("子集") || text.contains("另开") || text.contains("静默")),
        "risk record must cite NFR86 subset / no silent expand"
    );

    assert!(
        text.contains("111.2")
            && text.contains("111.3")
            && text.contains("111.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 111.2–111.4 from ready without this record"
    );

    assert!(text.contains("FR178"), "risk record must cite FR178");
    assert!(
        text.contains("Epic 111") || text.contains("Epic111"),
        "risk record must name Epic 111"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR83") && text.contains("负责人"),
        "risk record must assign NFR83 ownership alongside NFR14"
    );
    assert!(
        (text.contains("NFR84") || text.contains("NFR86") || text.contains("NFR87"))
            && text.contains("负责人"),
        "risk record must assign NFR83–87 ownership"
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
        text.contains("111")
            && text.contains("117")
            && (text.contains("软序") || text.contains("→")),
        "risk record must document soft order involving 111…117"
    );

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint-status.yaml");
    let s111_1 = yaml_value_for_key(&sprint, "111-1-epic-111-nfr14-风险记录")
        .expect("111-1 key in sprint-status");
    let later = [
        "111-2-correct-course-prd-批准-phase-22-fr178",
        "111-3-同步-readme-deferred-路线图指针-fr178",
        "111-4-ad-指针与-epic-111-收口-fr178",
    ];
    if s111_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 111-1 is done (got {status})"
            );
        }
    }

    assert!(
        sprint.contains("epic-110: done") || sprint.contains("epic-110:done"),
        "Phase 21 Epic 110 must be done before Epic 111 NFR14"
    );
    let epic111_done_gate = sprint.contains("epic-111: done") || sprint.contains("epic-111:done");
    if !epic111_done_gate {
        for line in sprint.lines() {
            let t = line.trim();
            if t.starts_with("112-")
                || t.starts_with("113-")
                || t.starts_with("114-")
                || t.starts_with("115-")
                || t.starts_with("116-")
                || t.starts_with("117-")
            {
                if t.contains("ready-for-dev") {
                    panic!(
                        "Epic 112–117 stories must not be ready-for-dev before epic-111 done: {t}"
                    );
                }
            }
        }
        for epic in 112..=117 {
            let key = format!("epic-{epic}");
            let st = yaml_value_for_key(&sprint, &key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until epic-111 done (got {st})"
            );
        }
    }
}
