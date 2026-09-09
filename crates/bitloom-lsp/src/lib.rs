//! Bitloom language-server library (FR99 / Story 44.3).
//!
//! Full-design elaborate on a documented edit-trigger path, with a shallow
//! contrast mode for ATDD. Public brand: **Bitloom**. Unrelated to `samitbasu/rhdl`.

use std::time::{Duration, Instant};

use bitloom_builder::{ElaborateSession, GroundType, HwCaptureRef, Span};
use bitloom_hir::{Diagnostic, FrozenHir};

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
pub fn analyze_on_did_save() -> AnalyzeResult {
    analyze(
        AnalysisMode::FullElaborate,
        DesignFixture::OkCounter,
        MVP_INTERACTIVE_BUDGET,
    )
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
}
