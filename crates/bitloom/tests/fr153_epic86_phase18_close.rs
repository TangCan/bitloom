//! ATDD Story 86.3: FR153 / Epic 86 + Phase 18 closeout.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
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
fn fr153_nfr14_epic86_closed() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic86-fr153-semver-honesty.md");
    assert!(
        text.contains("closed — Story 86.3") || (text.contains("closed") && text.contains("86.3")),
        "NFR14 Epic 86 must be closed with Story 86.3"
    );
    for needle in [
        "- [x] **FR153：",
        "- [x] **NFR14 / NFR64–67：",
        "- [x] **Phase 18：",
    ] {
        assert!(
            text.contains(needle),
            "missing checked close condition: {needle}"
        );
    }
    assert!(text.contains("NFR59") && text.contains("deferred"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
}

#[test]
fn fr153_sprint_epic86_and_phase18_stories_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("86-3-fr153-收口与-phase-18-故事清单指针: done")
            || sprint.contains("86-3-fr153-收口与-phase-18-故事清单指针:done")
    );
    assert_eq!(
        yaml_value_for_key(&sprint, "epic-86").as_deref(),
        Some("done")
    );
    for prefix in ["84-", "85-", "86-"] {
        for line in sprint.lines() {
            let t = line.trim();
            if t.starts_with(prefix)
                && !t.contains("retrospective")
                && t.contains(':')
                && !t.contains("done")
            {
                panic!("Phase 18 story not done: {t}");
            }
        }
    }
    for epic in ["epic-84", "epic-85", "epic-86"] {
        assert_eq!(
            yaml_value_for_key(&sprint, epic).as_deref(),
            Some("done"),
            "{epic} must be done"
        );
    }
}

#[test]
fn fr153_epics_phase18_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(epics.contains("phase18Status: complete") || epics.contains("phase18Status:complete"));
    assert!(
        epics.contains("phase18Epic86Status: complete")
            || epics.contains("phase18Epic86Status:complete")
    );
}

#[test]
fn fr153_readme_deferred_phase18_complete_nfr59() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        (readme.contains("Phase 18") || deferred.contains("Phase 18"))
            && (readme.contains("FR148") || deferred.contains("FR148"))
            && (readme.contains("FR153") || deferred.contains("FR153")),
        "must cite FR148–153 for Phase 18 claims"
    );
    assert!(
        readme.contains("已关闭") || deferred.contains("已齐") || deferred.contains("已关闭"),
        "must declare Phase 18 stories complete"
    );
    assert!(readme.contains("NFR59") && deferred.contains("NFR59"));
    assert!(deferred.contains("NFR67") || readme.contains("NFR67"));
    assert!(readme.contains("Bitloom"));
    assert!(
        !readme.contains("NFR59 已清") && !deferred.contains("NFR59 已清"),
        "must not claim NFR59 cleared"
    );
}
