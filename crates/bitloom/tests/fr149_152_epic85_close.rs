//! ATDD Story 85.6: FR149–152 / Epic 85 closeout; FR153 remains Epic 86.

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
fn fr149_152_epic85_nfr14_closed() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic85-cli-install-fr149-152.md");
    assert!(
        text.contains("closed — Story 85.6") || (text.contains("closed") && text.contains("85.6")),
        "NFR14 Epic 85 must be closed with Story 85.6"
    );
    for needle in [
        "- [x] **FR149：",
        "- [x] **FR150：",
        "- [x] **FR152(b)：",
        "- [x] **FR151：",
        "- [x] **FR153：",
    ] {
        assert!(
            text.contains(needle),
            "missing checked close condition: {needle}"
        );
    }
    assert!(
        text.contains("FR153") && (text.contains("Epic 86") || text.contains("86")),
        "must leave FR153 to Epic 86"
    );
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("NFR59"));
}

#[test]
fn fr149_152_sprint_epic85_done_86_1_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("85-6-fr149-152-收口与文档指针: done")
            || sprint.contains("85-6-fr149-152-收口与文档指针:done")
    );
    assert!(sprint.contains("epic-85: done") || sprint.contains("epic-85:done"));
    let s86_1 = yaml_value_for_key(&sprint, "86-1-epic-86-nfr14-风险记录");
    assert!(
        matches!(
            s86_1.as_deref(),
            Some("ready-for-dev") | Some("in-progress") | Some("done")
        ),
        "86.1 must be ready-for-dev (or later) after Epic 85 close; got {s86_1:?}"
    );
    let s86_2 = yaml_value_for_key(&sprint, "86-2-semver-assume-published-与发版诚实更新-fr153");
    if s86_1.as_deref() != Some("done") {
        assert_eq!(
            s86_2.as_deref(),
            Some("backlog"),
            "86.2 stays backlog until 86.1 done"
        );
    } else {
        assert!(
            matches!(
                s86_2.as_deref(),
                Some("backlog") | Some("ready-for-dev") | Some("in-progress") | Some("done")
            ),
            "86.2 may advance after 86.1 done; got {s86_2:?}"
        );
    }
}

#[test]
fn fr149_152_epics_phase18_epic85_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase18Epic85Status: complete")
            || epics.contains("phase18Epic85Status:complete")
    );
}

#[test]
fn fr149_152_readme_deferred_cli_installable_fr153_pending() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        readme.contains("cargo install bitloom")
            && (readme.contains("FR151") || readme.contains("已可") || readme.contains("已上架")),
        "README must claim CLI installable after FR151"
    );
    assert!(
        readme.contains("FR153")
            || deferred.contains("FR153")
            || PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../docs/fr153-semver-honesty.md")
                .exists(),
        "must mention FR153 / Epic 86 honesty path"
    );
    assert!(readme.contains("NFR59") && deferred.contains("NFR59"));
    assert!(readme.contains("Bitloom"));
}
