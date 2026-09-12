//! FR158 — third-party LCOV GUI path via `genhtml` (lcov package).
//!
//! Consumes Bitloom `coverage.lcov` (FR114). Does **not** replace the in-tree
//! `coverage.html` GUI.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Explicit FR158 failure (never silent success without genhtml).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LcovGuiError {
    /// `genhtml` not found on PATH.
    GenhtmlNotFound,
    /// LCOV input missing or unreadable.
    LcovMissing(String),
    /// `genhtml` ran but failed or did not produce an HTML entry.
    GenhtmlFailed(String),
}

impl std::fmt::Display for LcovGuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GenhtmlNotFound => write!(
                f,
                "FR158: `genhtml` not found on PATH (install `lcov` package, e.g. `apt install lcov`); \
                 Bitloom in-tree coverage.html (FR114) is not a substitute for this third-party path"
            ),
            Self::LcovMissing(p) => write!(f, "FR158: LCOV input missing or unreadable: {p}"),
            Self::GenhtmlFailed(e) => write!(f, "FR158: genhtml failed: {e}"),
        }
    }
}

impl std::error::Error for LcovGuiError {}

/// Locate `genhtml` on PATH (FR158 probe).
pub fn find_genhtml() -> Option<PathBuf> {
    which("genhtml")
}

fn which(bin: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(bin);
        if candidate.is_file() {
            return Some(candidate);
        }
        // Windows would need .exe; this product path is Unix CI/desktop first.
    }
    None
}

/// Run third-party `genhtml` on a Bitloom `coverage.lcov` (FR158).
///
/// Writes HTML under `html_out_dir` (creates it). Returns path to `index.html`
/// (or the first `*.html` entry if `index.html` is absent).
pub fn run_genhtml(lcov: &Path, html_out_dir: &Path) -> Result<PathBuf, LcovGuiError> {
    if !lcov.is_file() {
        return Err(LcovGuiError::LcovMissing(lcov.display().to_string()));
    }
    let genhtml = find_genhtml().ok_or(LcovGuiError::GenhtmlNotFound)?;

    std::fs::create_dir_all(html_out_dir).map_err(|e| {
        LcovGuiError::GenhtmlFailed(format!("create {}: {e}", html_out_dir.display()))
    })?;

    let out = Command::new(&genhtml)
        .arg("-o")
        .arg(html_out_dir)
        .arg(lcov)
        .output()
        .map_err(|e| LcovGuiError::GenhtmlFailed(format!("spawn {}: {e}", genhtml.display())))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let stdout = String::from_utf8_lossy(&out.stdout);
        return Err(LcovGuiError::GenhtmlFailed(format!(
            "exit {:?}; stderr={stderr}; stdout={stdout}",
            out.status.code()
        )));
    }

    let index = html_out_dir.join("index.html");
    if index.is_file() {
        return Ok(index);
    }
    // Some genhtml layouts nest; accept any html under out dir.
    if let Ok(rd) = std::fs::read_dir(html_out_dir) {
        for ent in rd.flatten() {
            let p = ent.path();
            if p.extension().and_then(|e| e.to_str()) == Some("html") {
                return Ok(p);
            }
        }
    }
    Err(LcovGuiError::GenhtmlFailed(format!(
        "no HTML produced under {}",
        html_out_dir.display()
    )))
}
