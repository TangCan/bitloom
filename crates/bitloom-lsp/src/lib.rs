//! Bitloom language-server library (FR99 / FR113).
//!
//! Full-design elaborate on a documented edit-trigger path, with a shallow
//! contrast mode for ATDD. FR113 adds Cargo-graph + metadata design-root discovery
//! beyond [`DesignFixture`]. Public brand: **Bitloom**. Unrelated to `samitbasu/rhdl`.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use bitloom_builder::{ElaborateSession, GroundType, HwCaptureRef, Span};
use bitloom_hir::{Diagnostic, FrozenHir};

mod discover;
pub use discover::{DiscoveredDesignRoot, discover_design_roots, discover_design_roots_under};

/// MVP module-count ceiling (NFR14 P4). Exceeding this must fail loudly.
pub const MVP_MAX_MODULES: usize = 8;

/// Interactive budget for fixture-scale designs (NFR14 P3), cold start excluded.
pub const MVP_INTERACTIVE_BUDGET: Duration = Duration::from_secs(2);

/// Analysis mode — full elaborate vs shallow/non-elaborate contrast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisMode {
    /// Runs `ElaborateSession::finish()` (full-design elaborate contract).
    FullElaborate,
    /// Lexical / non-elaborate path — must **not** call `finish()`.
    Shallow,
}

/// Documented MVP design roots used by the edit-trigger path and ATDD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignFixture {
    /// Small counter that elaborates successfully.
    OkCounter,
    /// Illegal HW capture — fails elaborate with `rhdl::E0142`.
    FailHwCapture,
    /// More modules than [`MVP_MAX_MODULES`].
    Oversized,
    /// Sleeps longer than the provided timeout (for P3 tests).
    SlowForTimeout,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HwSymbol {
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappedDiagnostic {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeResult {
    pub diagnostics: Vec<MappedDiagnostic>,
    pub symbols: Vec<HwSymbol>,
    /// `true` only when the full-elaborate path actually called `finish()`.
    pub called_finish: bool,
    pub timed_out: bool,
    pub oversized: bool,
    pub mode: AnalysisMode,
}

fn map_diag(d: &Diagnostic) -> MappedDiagnostic {
    MappedDiagnostic {
        code: d.code.clone(),
        message: format!("{}: {} ({})", d.code, d.en, d.zh),
    }
}

fn symbols_from_hir(hir: &FrozenHir) -> Vec<HwSymbol> {
    let mut out = Vec::new();
    for m in &hir.circuit().modules {
        out.push(HwSymbol {
            name: m.name.clone(),
            kind: "module".into(),
        });
        for p in &m.ports {
            out.push(HwSymbol {
                name: format!("{}.{}", m.name, p.name),
                kind: "port".into(),
            });
        }
    }
    out
}

fn build_ok_counter() -> ElaborateSession {
    let mut s = ElaborateSession::new("Fr99OkCounter");
    s.begin_module("Fr99OkCounter", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("q", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("q", "count", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("count", Span::default());
    s.end_process();
    s.end_module();
    s
}

fn build_fail_hw_capture() -> ElaborateSession {
    let mut s = ElaborateSession::new("Fr99FailCapture");
    s.begin_module("Fr99FailCapture", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_wire("w", GroundType::UInt { width: 8 }, Span::default());
    s.assert_no_hw_capture(&[HwCaptureRef::wire("w")], Span::default());
    s.declare_mem_with_init_fn("rom", 4, 8, |i| i as u64, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("y", "rom", Span::default());
    s.end_process();
    s.end_module();
    s
}

fn build_oversized() -> ElaborateSession {
    let mut s = ElaborateSession::new("Fr99Oversized");
    for i in 0..=MVP_MAX_MODULES {
        let name = format!("Mod{i}");
        s.begin_module(&name, Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_output("y", GroundType::Bool, Span::default());
        s.begin_combinational(Span::default());
        s.assign_lit("y", 0, Span::default());
        s.end_process();
        s.end_module();
    }
    s
}

/// Run analysis for a documented design fixture.
///
/// * [`AnalysisMode::FullElaborate`] calls `ElaborateSession::finish()`.
/// * [`AnalysisMode::Shallow`] returns without calling `finish()` (ATDD contrast).
pub fn analyze(mode: AnalysisMode, fixture: DesignFixture, timeout: Duration) -> AnalyzeResult {
    let started = Instant::now();

    if matches!(fixture, DesignFixture::SlowForTimeout) {
        if mode == AnalysisMode::Shallow {
            return AnalyzeResult {
                diagnostics: vec![],
                symbols: vec![],
                called_finish: false,
                timed_out: false,
                oversized: false,
                mode,
            };
        }
        // Simulate an elaborate that exceeds the interactive budget.
        std::thread::sleep(timeout.saturating_add(Duration::from_millis(50)));
        if started.elapsed() > timeout {
            return AnalyzeResult {
                diagnostics: vec![MappedDiagnostic {
                    code: "bitloom-lsp.timeout".into(),
                    message: format!(
                        "bitloom-lsp.timeout: full-design elaborate exceeded interactive budget ({timeout:?}); cold start may be excluded per NFR14 P3"
                    ),
                }],
                symbols: vec![],
                called_finish: false,
                timed_out: true,
                oversized: false,
                mode,
            };
        }
    }

    let session = match fixture {
        DesignFixture::OkCounter => build_ok_counter(),
        DesignFixture::FailHwCapture => build_fail_hw_capture(),
        DesignFixture::Oversized => build_oversized(),
        DesignFixture::SlowForTimeout => build_ok_counter(),
    };

    // P4: refuse oversized before pretending to finish successfully.
    let mods = match fixture {
        DesignFixture::Oversized => MVP_MAX_MODULES + 1,
        DesignFixture::OkCounter | DesignFixture::FailHwCapture | DesignFixture::SlowForTimeout => {
            1
        }
    };
    if mods > MVP_MAX_MODULES {
        return AnalyzeResult {
            diagnostics: vec![MappedDiagnostic {
                code: "bitloom-lsp.oversized".into(),
                message: format!(
                    "bitloom-lsp.oversized: design has {mods} modules; MVP ceiling is {MVP_MAX_MODULES} (NFR14 P4) — refusing to pretend full elaborate succeeded"
                ),
            }],
            symbols: vec![],
            called_finish: false,
            timed_out: false,
            oversized: true,
            mode,
        };
    }

    match mode {
        AnalysisMode::Shallow => AnalyzeResult {
            diagnostics: vec![],
            symbols: vec![],
            called_finish: false,
            timed_out: false,
            oversized: false,
            mode,
        },
        AnalysisMode::FullElaborate => {
            if started.elapsed() > timeout {
                return AnalyzeResult {
                    diagnostics: vec![MappedDiagnostic {
                        code: "bitloom-lsp.timeout".into(),
                        message: format!(
                            "bitloom-lsp.timeout: full-design elaborate exceeded interactive budget ({timeout:?})"
                        ),
                    }],
                    symbols: vec![],
                    called_finish: false,
                    timed_out: true,
                    oversized: false,
                    mode,
                };
            }
            match session.finish() {
                Ok(hir) => AnalyzeResult {
                    diagnostics: vec![],
                    symbols: symbols_from_hir(&hir),
                    called_finish: true,
                    timed_out: false,
                    oversized: false,
                    mode,
                },
                Err(diags) => AnalyzeResult {
                    diagnostics: diags.0.iter().map(map_diag).collect(),
                    symbols: vec![],
                    called_finish: true,
                    timed_out: false,
                    oversized: false,
                    mode,
                },
            }
        }
    }
}

/// Default edit-trigger fixture when the client saves a workspace document (P1: didSave).
/// FR99 DesignFixture path — kept for MVP regression (NFR44).
pub fn analyze_on_did_save() -> AnalyzeResult {
    analyze(
        AnalysisMode::FullElaborate,
        DesignFixture::OkCounter,
        MVP_INTERACTIVE_BUDGET,
    )
}

/// Resolve an elaborate session for a discovered `root_id` (FR113 metadata / FR118 syn-scan registry).
///
/// Root ids are documented entry names (文档等价 of `.rs`/type paths from metadata or
/// `#[bitloom::top]` syn-scan).
fn session_for_discovered_root_id(root_id: &str) -> Option<(ElaborateSession, usize)> {
    match root_id {
        "Fr113OkCounter" => Some((build_fr113_ok_counter(), 1)),
        "Fr113FailHwCapture" => Some((build_fr113_fail_hw_capture(), 1)),
        "Fr113Oversized" => Some((build_oversized(), MVP_MAX_MODULES + 1)),
        "Fr118OkCounter" => Some((build_fr118_ok_counter(), 1)),
        "Fr118FailHwCapture" => Some((build_fr118_fail_hw_capture(), 1)),
        _ => None,
    }
}

fn build_fr113_ok_counter() -> ElaborateSession {
    let mut s = ElaborateSession::new("Fr113OkCounter");
    s.begin_module("Fr113OkCounter", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("q", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("q", "count", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("count", Span::default());
    s.end_process();
    s.end_module();
    s
}

fn build_fr113_fail_hw_capture() -> ElaborateSession {
    let mut s = ElaborateSession::new("Fr113FailCapture");
    s.begin_module("Fr113FailCapture", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_wire("w", GroundType::UInt { width: 8 }, Span::default());
    s.assert_no_hw_capture(&[HwCaptureRef::wire("w")], Span::default());
    s.declare_mem_with_init_fn("rom", 4, 8, |i| i as u64, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("y", "rom", Span::default());
    s.end_process();
    s.end_module();
    s
}

fn build_fr118_ok_counter() -> ElaborateSession {
    let mut s = ElaborateSession::new("Fr118OkCounter");
    s.begin_module("Fr118OkCounter", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("q", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("q", "count", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("count", Span::default());
    s.end_process();
    s.end_module();
    s
}

fn build_fr118_fail_hw_capture() -> ElaborateSession {
    let mut s = ElaborateSession::new("Fr118FailCapture");
    s.begin_module("Fr118FailCapture", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_wire("w", GroundType::UInt { width: 8 }, Span::default());
    s.assert_no_hw_capture(&[HwCaptureRef::wire("w")], Span::default());
    s.declare_mem_with_init_fn("rom", 4, 8, |i| i as u64, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("y", "rom", Span::default());
    s.end_process();
    s.end_module();
    s
}

/// Full-design elaborate for a Cargo-metadata discovered root (FR113).
///
/// Does **not** take a [`DesignFixture`]. Unknown `root_id` → readable fail without
/// pretending `finish` succeeded.
pub fn analyze_discovered_root(
    mode: AnalysisMode,
    root: &DiscoveredDesignRoot,
    timeout: Duration,
) -> AnalyzeResult {
    let started = Instant::now();
    let Some((session, mods)) = session_for_discovered_root_id(&root.root_id) else {
        return AnalyzeResult {
            diagnostics: vec![MappedDiagnostic {
                code: "bitloom-lsp.unknown-design-root".into(),
                message: format!(
                    "bitloom-lsp.unknown-design-root: package `{}` root_id `{}` is not a registered FR113/FR118 elaborate entry",
                    root.package_name, root.root_id
                ),
            }],
            symbols: vec![],
            called_finish: false,
            timed_out: false,
            oversized: false,
            mode,
        };
    };

    if mods > MVP_MAX_MODULES {
        return AnalyzeResult {
            diagnostics: vec![MappedDiagnostic {
                code: "bitloom-lsp.oversized".into(),
                message: format!(
                    "bitloom-lsp.oversized: discovered root `{}` has {mods} modules; MVP ceiling is {MVP_MAX_MODULES} (NFR14 P4)",
                    root.root_id
                ),
            }],
            symbols: vec![],
            called_finish: false,
            timed_out: false,
            oversized: true,
            mode,
        };
    }

    match mode {
        AnalysisMode::Shallow => AnalyzeResult {
            diagnostics: vec![],
            symbols: vec![],
            called_finish: false,
            timed_out: false,
            oversized: false,
            mode,
        },
        AnalysisMode::FullElaborate => {
            if started.elapsed() > timeout {
                return AnalyzeResult {
                    diagnostics: vec![MappedDiagnostic {
                        code: "bitloom-lsp.timeout".into(),
                        message: format!(
                            "bitloom-lsp.timeout: discovered-root elaborate exceeded interactive budget ({timeout:?})"
                        ),
                    }],
                    symbols: vec![],
                    called_finish: false,
                    timed_out: true,
                    oversized: false,
                    mode,
                };
            }
            match session.finish() {
                Ok(hir) => AnalyzeResult {
                    diagnostics: vec![],
                    symbols: symbols_from_hir(&hir),
                    called_finish: true,
                    timed_out: false,
                    oversized: false,
                    mode,
                },
                Err(diags) => AnalyzeResult {
                    diagnostics: diags.0.iter().map(map_diag).collect(),
                    symbols: vec![],
                    called_finish: true,
                    timed_out: false,
                    oversized: false,
                    mode,
                },
            }
        }
    }
}

/// Discover roots under `workspace` and fully elaborate the first root (FR113).
///
/// Empty discovery → `bitloom-lsp.no-design-roots` (readable; not silent Ok).
pub fn analyze_workspace_design_roots(
    mode: AnalysisMode,
    workspace: impl AsRef<Path>,
    timeout: Duration,
) -> AnalyzeResult {
    let workspace = workspace.as_ref();
    match discover_design_roots(workspace) {
        Ok(roots) if roots.is_empty() => AnalyzeResult {
            diagnostics: vec![MappedDiagnostic {
                code: "bitloom-lsp.no-design-roots".into(),
                message: format!(
                    "bitloom-lsp.no-design-roots: no metadata `design_roots` and no `#[bitloom::top]` syn-scan hits under {} (FR113/FR118); DesignFixture-only ≠ deepen",
                    workspace.display()
                ),
            }],
            symbols: vec![],
            called_finish: false,
            timed_out: false,
            oversized: false,
            mode,
        },
        Ok(roots) => analyze_discovered_root(mode, &roots[0], timeout),
        Err(e) => AnalyzeResult {
            diagnostics: vec![MappedDiagnostic {
                code: "bitloom-lsp.discover-failed".into(),
                message: format!(
                    "bitloom-lsp.discover-failed: Cargo-graph discovery under {}: {e}",
                    workspace.display()
                ),
            }],
            symbols: vec![],
            called_finish: false,
            timed_out: false,
            oversized: false,
            mode,
        },
    }
}

/// didSave path: prefer FR113/FR118 discovery when `path_hint` finds a Cargo
/// package/workspace with metadata or syn-scan roots; otherwise fall back to FR99
/// [`analyze_on_did_save`].
pub fn analyze_on_did_save_at(path_hint: Option<&Path>) -> AnalyzeResult {
    if let Some(hint) = path_hint {
        if let Some(cargo_root) = find_cargo_root(hint) {
            if let Ok(roots) = discover_design_roots(&cargo_root) {
                if !roots.is_empty() {
                    return analyze_discovered_root(
                        AnalysisMode::FullElaborate,
                        &roots[0],
                        MVP_INTERACTIVE_BUDGET,
                    );
                }
            }
        }
    }
    analyze_on_did_save()
}

fn find_cargo_root(start: &Path) -> Option<PathBuf> {
    let mut cur = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.to_path_buf()
    };
    loop {
        if cur.join("Cargo.toml").is_file() {
            return Some(cur);
        }
        if !cur.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_ok_calls_finish_and_symbols() {
        let r = analyze(
            AnalysisMode::FullElaborate,
            DesignFixture::OkCounter,
            MVP_INTERACTIVE_BUDGET,
        );
        assert!(r.called_finish);
        assert!(r.diagnostics.is_empty());
        assert!(r.symbols.iter().any(|s| s.name == "Fr99OkCounter"));
        assert!(r.symbols.iter().any(|s| s.kind == "port"));
    }

    #[test]
    fn shallow_does_not_call_finish() {
        let r = analyze(
            AnalysisMode::Shallow,
            DesignFixture::FailHwCapture,
            MVP_INTERACTIVE_BUDGET,
        );
        assert!(!r.called_finish);
        assert!(r.diagnostics.is_empty());
    }

    #[test]
    fn discovered_ok_counter_calls_finish() {
        let root = DiscoveredDesignRoot {
            package_name: "fixture".into(),
            package_dir: PathBuf::from("."),
            root_id: "Fr113OkCounter".into(),
        };
        let r = analyze_discovered_root(AnalysisMode::FullElaborate, &root, MVP_INTERACTIVE_BUDGET);
        assert!(r.called_finish);
        assert!(r.symbols.iter().any(|s| s.name == "Fr113OkCounter"));
    }
}
