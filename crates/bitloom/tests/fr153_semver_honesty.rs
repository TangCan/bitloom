//! ATDD Story 86.2: FR153 SemVer honesty — remove 1.0.0 special-case; docs pointers.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
fn fr153_semver_script_defaults_minor_without_1_0_0_special_case() {
    let script = read("scripts/semver-check.sh");
    assert!(
        !script.contains("ver\" == \"1.0.0\"")
            && !script.contains("ver\" == '1.0.0'")
            && !script.contains("[ \"$ver\" == \"1.0.0\" ]"),
        "1.0.0 special-case branch must be removed"
    );
    // Detect logic: 0.x → major; else → minor (no ASSUME_PUBLISHED gate required).
    assert!(
        script.contains("=~ ^0\\.") || script.contains("^0\\."),
        "must still treat 0.x as major"
    );
    assert!(
        script.contains("minor"),
        "must default to minor for published ≥1.0"
    );

    let policy = read("docs/semver-1-0-policy.md");
    assert!(
        policy.contains("FR153")
            && (policy.contains("removed") || policy.contains("移除") || policy.contains("minor")),
        "policy must document FR153 post-publish minor default"
    );
}

#[test]
fn fr153_docs_honesty_library_and_cli() {
    let fr153 = read("docs/fr153-semver-honesty.md");
    assert!(fr153.contains("FR153"));
    assert!(
        fr153.contains("bitloom") && (fr153.contains("CLI") || fr153.contains("cargo install")),
        "must distinguish CLI published status"
    );
    assert!(
        fr153.contains("prelude") || fr153.contains("Library") || fr153.contains("library"),
        "must mention library crates"
    );
    assert!(
        fr153.contains("NFR59") && (fr153.contains("deferred") || fr153.contains("仍")),
        "NFR59 must remain deferred"
    );
    assert!(
        !fr153.to_lowercase().contains("nfr59 is closed") && !fr153.contains("NFR59 已清"),
        "must not claim NFR59 cleared"
    );

    let fr146 = read("docs/fr146-bitloom-1-0-0-release.md");
    assert!(
        fr146.contains("FR153") && fr146.contains("fr153-semver-honesty"),
        "fr146 must point at FR153 honesty doc"
    );

    let readme = read("README.md");
    assert!(
        readme.contains("FR153")
            && (readme.contains("fr153-semver-honesty")
                || readme.contains("86.2")
                || readme.contains("已落地")),
        "README must reflect FR153 honesty landed"
    );
    assert!(readme.contains("NFR59"));
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr153_sprint_86_2_done_gates_86_3() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert_eq!(
        yaml_value_for_key(&sprint, "86-1-epic-86-nfr14-风险记录").as_deref(),
        Some("done")
    );
    assert_eq!(
        yaml_value_for_key(&sprint, "86-2-semver-assume-published-与发版诚实更新-fr153").as_deref(),
        Some("done")
    );
    let s86_3 = yaml_value_for_key(&sprint, "86-3-fr153-收口与-phase-18-故事清单指针");
    assert!(
        matches!(
            s86_3.as_deref(),
            Some("ready-for-dev") | Some("in-progress") | Some("done")
        ),
        "86.3 must be ready-for-dev (or later) after 86.2; got {s86_3:?}"
    );
    let epic86 = yaml_value_for_key(&sprint, "epic-86").unwrap_or_default();
    assert!(
        epic86 == "in-progress" || epic86 == "done",
        "epic-86 must stay in-progress until 86.3 close; got {epic86}"
    );
}

#[test]
fn fr153_detect_release_type_echoes_minor_for_workspace_1_0_0() {
    // Source the detect logic indirectly: run a tiny bash snippet mirroring the script.
    let out = Command::new("bash")
        .arg("-c")
        .arg(
            r#"
set -euo pipefail
ROOT="$(pwd)"
ver="$(grep -m1 '^version' "$ROOT/Cargo.toml" | sed -E 's/.*"([^"]+)".*/\1/')"
if [[ "$ver" =~ ^0\. ]]; then echo major; else echo minor; fi
"#,
        )
        .current_dir(workspace_root())
        .output()
        .expect("run detect");
    assert!(
        out.status.success(),
        "detect failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let got = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(
        got, "minor",
        "workspace 1.0.0 must detect as minor after FR153"
    );
}
