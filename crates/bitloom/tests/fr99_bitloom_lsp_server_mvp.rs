//! ATDD (red→green): Story 44.2 / FR99 — Bitloom LSP server MVP + editor wiring.
//!
//! Deliverable: installable/startable `bitloom-lsp` with minimal initialize/capabilities,
//! VS Code (or equivalent) wiring docs, and a reproducible stdio LSP session.
//! Does **not** implement keystroke full-elaborate diagnostics (44.3) or FR99 closeout (44.4).
//! Must **not** claim rust-analyzer alone completes this story.
//!
//! ```text
//! cargo test -p bitloom --test fr99_bitloom_lsp_server_mvp
//! ```

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn fr99_bitloom_lsp_crate_and_binary_exist() {
    let root = workspace_root();
    let crate_dir = root.join("crates/bitloom-lsp");
    assert!(
        crate_dir.is_dir(),
        "expected crates/bitloom-lsp directory for Bitloom language-server MVP"
    );
    let cargo = read("crates/bitloom-lsp/Cargo.toml");
    assert!(
        cargo.contains("name = \"bitloom-lsp\"") || cargo.contains("name = 'bitloom-lsp'"),
        "bitloom-lsp package name must be bitloom-lsp"
    );
    assert!(
        cargo.contains("[[bin]]")
            && (cargo.contains("name = \"bitloom-lsp\"") || cargo.contains("name = 'bitloom-lsp'")),
        "bitloom-lsp must declare [[bin]] name = \"bitloom-lsp\""
    );
    let workspace = read("Cargo.toml");
    assert!(
        workspace.contains("crates/bitloom-lsp") || workspace.contains("bitloom-lsp"),
        "workspace Cargo.toml must list crates/bitloom-lsp"
    );
}

#[test]
fn fr99_bitloom_lsp_initialize_session() {
    // Reproducible fixture: spawn bitloom-lsp over stdio, send initialize, expect result + capabilities.
    let mut child = Command::new("cargo")
        .args(["run", "-q", "-p", "bitloom-lsp", "--"])
        .current_dir(workspace_root())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn cargo run -p bitloom-lsp");

    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_body = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"clientInfo":{"name":"bitloom-fr99-fixture","version":"0"},"capabilities":{},"rootUri":null}}"#;
    let framed = format!("Content-Length: {}\r\n\r\n{}", init_body.len(), init_body);
    stdin
        .write_all(framed.as_bytes())
        .expect("write initialize");
    stdin.flush().expect("flush initialize");

    // Read one LSP message (headers + body).
    let mut header_buf = Vec::new();
    let mut byte = [0u8; 1];
    let deadline = std::time::Instant::now() + Duration::from_secs(120);
    while !header_buf.windows(4).any(|w| w == b"\r\n\r\n") {
        if std::time::Instant::now() > deadline {
            let _ = child.kill();
            panic!("timeout waiting for LSP headers from bitloom-lsp");
        }
        match stdout.read(&mut byte) {
            Ok(0) => {
                let mut err = String::new();
                if let Some(mut e) = child.stderr.take() {
                    let _ = e.read_to_string(&mut err);
                }
                let _ = child.kill();
                panic!("EOF before LSP headers; stderr={err}");
            }
            Ok(_) => header_buf.push(byte[0]),
            Err(e) => {
                let _ = child.kill();
                panic!("read headers: {e}");
            }
        }
    }
    let header_str = String::from_utf8_lossy(&header_buf);
    let content_length = header_str
        .lines()
        .find_map(|l| {
            let lower = l.to_ascii_lowercase();
            lower
                .strip_prefix("content-length:")
                .map(|v| v.trim().parse::<usize>().expect("content-length"))
        })
        .expect("Content-Length header");

    let mut body = vec![0u8; content_length];
    stdout.read_exact(&mut body).expect("read body");
    let body_str = String::from_utf8_lossy(&body);
    assert!(
        body_str.contains("\"result\"") || body_str.contains("capabilities"),
        "initialize response must include result/capabilities; got: {body_str}"
    );
    assert!(
        body_str.contains("capabilities") || body_str.contains("serverInfo"),
        "initialize must advertise capabilities or serverInfo; got: {body_str}"
    );

    // Best-effort shutdown so cargo run can exit.
    let shutdown = r#"{"jsonrpc":"2.0","id":2,"method":"shutdown","params":null}"#;
    let exit = r#"{"jsonrpc":"2.0","method":"exit","params":null}"#;
    let _ = stdin.write_all(
        format!(
            "Content-Length: {}\r\n\r\n{}Content-Length: {}\r\n\r\n{}",
            shutdown.len(),
            shutdown,
            exit.len(),
            exit
        )
        .as_bytes(),
    );
    let _ = stdin.flush();
    let _ = child.wait();
}

#[test]
fn fr99_bitloom_lsp_editor_wiring_docs() {
    let doc = read("docs/fr99-bitloom-lsp.md");
    assert!(
        doc.contains("bitloom-lsp") && (doc.contains("VS Code") || doc.contains("vscode")),
        "docs/fr99-bitloom-lsp.md must document bitloom-lsp and VS Code (or vscode) wiring"
    );
    assert!(
        doc.contains("initialize")
            || doc.contains("stdio")
            || doc.contains("language server")
            || doc.contains("language-server"),
        "wiring doc must mention initialize/stdio/language-server session"
    );
    assert!(doc.contains("Bitloom"), "wiring doc must brand Bitloom");
}

#[test]
fn fr99_bitloom_lsp_brand_and_not_rust_analyzer_alone() {
    let doc = read("docs/fr99-bitloom-lsp.md");
    assert!(doc.contains("Bitloom"), "must brand Bitloom");
    let forbids_ra_alone = (doc.contains("rust-analyzer") || doc.contains("FR90"))
        && (doc.contains("不得")
            || doc.contains("must not")
            || doc.contains("MUST NOT")
            || doc.contains("not")
            || doc.contains("≠")
            || doc.contains("不替代")
            || doc.contains("alone")
            || doc.contains("单独"));
    assert!(
        forbids_ra_alone,
        "docs must state rust-analyzer alone does not complete this story / FR99 MVP server"
    );
}

#[test]
fn fr99_bitloom_lsp_mvp_scope_guards() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("44-2-bitloom-lsp-服务器-mvp-fr99: done"),
        "sprint must mark 44-2 done"
    );
    // After 44.3, 44-3 is done; 44-4 + epic close remain open.
    assert!(
        sprint.contains("44-3-按键全-elaborate-诊断-符号-fr99: done")
            || sprint.contains("44-3-按键全-elaborate-诊断-符号-fr99: backlog"),
        "44-3 must be tracked (backlog during 44.2-only; done after 44.3)"
    );
    assert!(
        sprint.contains("44-4-fr99-收口与撤销-lsp-非目标: backlog"),
        "44-4 must remain backlog"
    );
    assert!(
        sprint.contains("epic-44: in-progress"),
        "epic-44 must stay in-progress"
    );

    let nfr14 =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md");
    // Epic 44 close checklist must NOT be fully ticked yet (44.4).
    assert!(
        nfr14.contains("- [ ] **44.2：**")
            || nfr14.contains("- [x] **44.2：**")
            || nfr14.contains("44.2"),
        "NFR14 must still discuss 44.2 close item"
    );
    // Full FR99 / Path B revocation boxes remain open (44.4).
    assert!(
        nfr14.contains("- [ ] **文档")
            || nfr14.contains("- [ ] **禁止")
            || nfr14.contains("Story 44.4"),
        "NFR14 Epic 44 close / Path B revocation must remain for 44.4"
    );

    // fr38 honesty: must not claim repo has zero language-server binary once 44.2 lands.
    let fr38 = read("docs/fr38-viz-lsp.md");
    assert!(
        !(fr38.contains("There is **no** Bitloom `language-server` binary in this repo")
            && !fr38.contains("Epic 44")
            && !fr38.contains("FR99")
            && !fr38.contains("bitloom-lsp")),
        "fr38 must not keep an unqualified 'no language-server binary' claim after 44.2"
    );
}
