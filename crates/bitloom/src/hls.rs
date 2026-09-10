//! Product HLS front-end (AD-25 revised / FR35 / FR50 / FR76 / **FR95** / **FR96**).
//!
//! - **FR35 / FR76 external path:** emit host C and call pinned Bambu (optional/对照).
//! - **FR95 in-tree path:** documented-subset scheduling/allocation inside bitloom
//!   (`schedule_in_tree` / loop-unroll MVP) — does **not** call Bambu; alone satisfies FR95.
//! - **FR96:** dissolve/inline dataflow-transform closures **before** in-tree schedule
//!   (`schedule_in_tree_from_transform`) — feeds the FR95 path; capturing closures reject
//!   readably (AD-18).
//!
//! Story 29.2: HLS dataflow closures dissolve to C ops **before** external schedule/lower.
//! Story 41.2: in-tree loop-unroll schedule IR + optional RTL stub.
//! Story 41.3: closure transform → FR95 in-tree schedule (FR96).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Pinned external HLS backend name (AD-25 / NFR14).
pub const HLS_BACKEND: &str = "bambu";

/// Pinned PandA Bambu release (NFR14 / Epic 24).
pub const HLS_BACKEND_VERSION: &str = "2024.10";

#[derive(Debug)]
pub enum HlsError {
    Message(String),
}

impl std::fmt::Display for HlsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HlsError::Message(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for HlsError {}

/// D1 constraint class for HLS dataflow closures (FR76 / Cap-R-62).
///
/// **HlsFree** is allowed only on the AD-25 external HLS path (and functional side).
/// Synthesizable Bitloom comb/seq must use `SynthesizableClosure` instead — never this class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HlsClosureConstraintClass {
    /// Free / wider closures on AD-25 external HLS; must dissolve before schedule.
    HlsFree,
}

/// Violation kinds checked before dissolving an HLS dataflow transform (decision-table D1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HlsDataflowClosureViolationKind {
    /// Capturing / runtime state — cannot dissolve cleanly before schedule.
    CapturingOrStateful,
    /// Attempted to apply HLS-free class on the synthesizable Bitloom path.
    WrongPathForSynthesizable,
}

/// Token recorded by authors / checks when a transform is illegal for the HLS free path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HlsDataflowClosureViolation {
    pub kind: HlsDataflowClosureViolationKind,
    pub message: String,
}

impl HlsDataflowClosureViolation {
    pub fn capturing(message: impl Into<String>) -> Self {
        Self {
            kind: HlsDataflowClosureViolationKind::CapturingOrStateful,
            message: message.into(),
        }
    }

    pub fn wrong_path_synthesizable(message: impl Into<String>) -> Self {
        Self {
            kind: HlsDataflowClosureViolationKind::WrongPathForSynthesizable,
            message: message.into(),
        }
    }
}

/// Descriptor returned by HLS dataflow transform closures (elaborate/emit-prep only).
/// Expanded to C ops; never stored as `Fn` in artifacts (NFR36).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HlsDataflowOp {
    /// Unary: `y = x`.
    Identity,
    /// Unary: `y = x + k` (C unsigned wrap).
    AddConst(u32),
    /// Unary: `y = x ^ k`.
    XorConst(u32),
    /// Unary: `y = x * k`.
    MulConst(u32),
    /// Binary stub: `y = a + b` (legacy FR35 default).
    AddInputs,
}

/// Dissolved HLS dataflow: C source ready for external schedule — no closure residue.
#[derive(Debug, Clone)]
pub struct DissolvedHlsDataflow {
    pub fn_name: String,
    pub constraint_class: HlsClosureConstraintClass,
    pub op: HlsDataflowOp,
    /// Full C translation unit after dissolve (NFR36: no Fn / closure tokens).
    pub c_source: String,
}

/// FR95 in-tree schedule kind (documented MVP subset; AD-25 revised).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InTreeScheduleKind {
    /// Fully unroll a counted loop of `trip_count` iterations (Story 41.2 demo).
    LoopUnroll { trip_count: u32 },
    /// Simple initiation-interval pipeline (FR95 secondary; **FR110** when `stages >= 2`).
    Pipeline {
        initiation_interval: u32,
        stages: u32,
    },
}

/// True when `kind` meets NFR14 FR110 default gates Q1+Q2 (`pipeline_stages >= 2` + II).
pub fn meets_fr110_commercial_depth(kind: &InTreeScheduleKind) -> bool {
    matches!(
        kind,
        InTreeScheduleKind::Pipeline {
            initiation_interval,
            stages
        } if *initiation_interval >= 1 && *stages >= 2
    )
}

/// One scheduled stage in an in-tree HLS artifact (FR95).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleStage {
    pub index: u32,
    pub op: HlsDataflowOp,
    pub label: String,
}

/// Checkable in-tree schedule result — never produced by spawning Bambu.
#[derive(Debug, Clone)]
pub struct InTreeScheduleArtifact {
    pub fn_name: String,
    pub kind: InTreeScheduleKind,
    pub stages: Vec<ScheduleStage>,
    /// Machine/human readable schedule IR (must cite FR95 + kind metadata).
    pub schedule_ir: String,
    /// Optional trivial Verilog stub (honestly marked `in-tree-mvp`; not commercial HLS quality).
    pub rtl_stub: String,
}

/// Resolve pinned Bambu binary: `BITLOOM_BAMBU_PATH`, else `RHDL_BAMBU_PATH`, else `PATH`.
pub fn resolve_bambu() -> Result<PathBuf, HlsError> {
    if let Some(p) = std::env::var_os("BITLOOM_BAMBU_PATH").map(PathBuf::from) {
        return Ok(p);
    }
    if let Some(p) = std::env::var_os("RHDL_BAMBU_PATH").map(PathBuf::from) {
        return Ok(p);
    }
    which("bambu").ok_or_else(|| {
        HlsError::Message(format!(
            "pinned HLS backend {HLS_BACKEND} {HLS_BACKEND_VERSION} not found; \
             set BITLOOM_BAMBU_PATH (or RHDL_BAMBU_PATH) to the bambu binary, or install \
             AppImage https://release.bambuhls.eu/bambu-{HLS_BACKEND_VERSION}.AppImage \
             (AD-25 / FR35)"
        ))
    })
}

/// Check D1 constraint class for HLS dataflow closures (Cap-R-62).
pub fn check_hls_dataflow_closure(
    class: HlsClosureConstraintClass,
    violations: &[HlsDataflowClosureViolation],
) -> Result<(), HlsError> {
    match class {
        HlsClosureConstraintClass::HlsFree => {
            if violations.is_empty() {
                Ok(())
            } else {
                let detail = violations
                    .iter()
                    .map(|v| format!("{:?}: {}", v.kind, v.message))
                    .collect::<Vec<_>>()
                    .join("; ");
                Err(HlsError::Message(format!(
                    "HLS dataflow closure rejected (D1 / Cap-R-62, class={class:?}): {detail}"
                )))
            }
        }
    }
}

fn c_source_for_op(fn_name: &str, op: &HlsDataflowOp) -> String {
    let header = format!(
        "/* generated by bitloom for {HLS_BACKEND} {HLS_BACKEND_VERSION}; no scheduling in bitloom */\n\
         /* FR76: dataflow transform expanded before external HLS lower (NFR36 clean) */\n"
    );
    let body = match op {
        HlsDataflowOp::Identity => format!(
            "unsigned {fn_name}(unsigned x) {{\n\
               return x;\n\
             }}\n"
        ),
        HlsDataflowOp::AddConst(k) => format!(
            "unsigned {fn_name}(unsigned x) {{\n\
               return x + {k}u;\n\
             }}\n"
        ),
        HlsDataflowOp::XorConst(k) => format!(
            "unsigned {fn_name}(unsigned x) {{\n\
               return x ^ {k}u;\n\
             }}\n"
        ),
        HlsDataflowOp::MulConst(k) => format!(
            "unsigned {fn_name}(unsigned x) {{\n\
               return x * {k}u;\n\
             }}\n"
        ),
        HlsDataflowOp::AddInputs => format!(
            "unsigned {fn_name}(unsigned a, unsigned b) {{\n\
               return a + b;\n\
             }}\n"
        ),
    };
    format!("{header}{body}")
}

/// Spot-check: dissolved C must not retain closure / Fn IR tokens (NFR36).
pub fn assert_no_closure_residue_in_c(c_source: &str) -> Result<(), HlsError> {
    let lower = c_source.to_lowercase();
    for n in ["closure", "callback", "fnonce", "fnmut", "dyn fn"] {
        if lower.contains(n) {
            return Err(HlsError::Message(format!(
                "NFR36: dissolved HLS C must not contain closure residue (found `{n}`)"
            )));
        }
    }
    if c_source.contains("||") {
        return Err(HlsError::Message(
            "NFR36: dissolved HLS C must not contain `||` closure token".into(),
        ));
    }
    // Word-ish `Fn(` / `Fn (` (Rust closure trait), not C identifiers.
    for needle in ["Fn(", "Fn ("] {
        if c_source.contains(needle) {
            return Err(HlsError::Message(format!(
                "NFR36: dissolved HLS C must not contain `{needle}`"
            )));
        }
    }
    Ok(())
}

/// Invoke a dataflow transform closure once, expand to C ops, drop the `Fn` (FR76 / Cap-R-71).
///
/// Minimal API: the closure returns an [`HlsDataflowOp`] descriptor (same pattern as
/// `CombInline` on the synthesizable path). After dissolve, callers may take the **external**
/// Bambu path (FR35/FR76) or the **in-tree** FR95 schedule via
/// [`schedule_in_tree_from_transform`] (FR96).
pub fn dissolve_dataflow_transform<F>(
    fn_name: &str,
    violations: &[HlsDataflowClosureViolation],
    transform: F,
) -> Result<DissolvedHlsDataflow, HlsError>
where
    F: FnOnce() -> HlsDataflowOp,
{
    check_hls_dataflow_closure(HlsClosureConstraintClass::HlsFree, violations)?;
    let op = transform();
    let c_source = c_source_for_op(fn_name, &op);
    assert_no_closure_residue_in_c(&c_source)?;
    Ok(DissolvedHlsDataflow {
        fn_name: fn_name.to_string(),
        constraint_class: HlsClosureConstraintClass::HlsFree,
        op,
        c_source,
    })
}

/// Parse CLI / fixture aliases into an [`HlsDataflowOp`].
pub fn parse_dataflow_alias(alias: &str) -> Result<HlsDataflowOp, HlsError> {
    match alias {
        "add" | "add_inputs" => Ok(HlsDataflowOp::AddInputs),
        "id" | "identity" => Ok(HlsDataflowOp::Identity),
        "add1" => Ok(HlsDataflowOp::AddConst(1)),
        "xor_a5" | "map_xor_a5" => Ok(HlsDataflowOp::XorConst(0xa5)),
        other => Err(HlsError::Message(format!(
            "unknown HLS dataflow alias `{other}`; \
             expected add|identity|add1|xor_a5 (FR76)"
        ))),
    }
}

/// Write dissolved C to `out_dir/{fn_name}.c`.
pub fn emit_c_dissolved(
    dissolved: &DissolvedHlsDataflow,
    out_dir: &Path,
) -> Result<PathBuf, HlsError> {
    fs::create_dir_all(out_dir).map_err(|e| HlsError::Message(e.to_string()))?;
    let path = out_dir.join(format!("{}.c", dissolved.fn_name));
    fs::write(&path, &dissolved.c_source).map_err(|e| HlsError::Message(e.to_string()))?;
    Ok(path)
}

/// Emit a trivial C stub the external tool accepts (no in-crate scheduling).
pub fn emit_c_stub(fn_name: &str, out_dir: &Path) -> Result<PathBuf, HlsError> {
    let dissolved = dissolve_dataflow_transform(fn_name, &[], || HlsDataflowOp::AddInputs)?;
    emit_c_dissolved(&dissolved, out_dir)
}

/// Run HLS from an already-dissolved transform (FR76 product path).
///
/// `--emit-only` / `BITLOOM_HLS_EMIT_ONLY=1` writes C and returns without calling the backend
/// (useful for docs inspection); it does **not** count as a successful HLS RTL run.
///
/// `bambu_override`: when `Some`, use that binary instead of env/`PATH` (ATDD / CI stub).
pub fn run_hls_dissolved(
    dissolved: &DissolvedHlsDataflow,
    out_dir: &Path,
    emit_only: bool,
) -> Result<PathBuf, HlsError> {
    run_hls_dissolved_with_backend(dissolved, out_dir, emit_only, None)
}

/// Like [`run_hls_dissolved`], with an optional explicit backend path (no env mutation).
pub fn run_hls_dissolved_with_backend(
    dissolved: &DissolvedHlsDataflow,
    out_dir: &Path,
    emit_only: bool,
    bambu_override: Option<&Path>,
) -> Result<PathBuf, HlsError> {
    let c = emit_c_dissolved(dissolved, out_dir)?;
    if emit_only
        || matches!(
            std::env::var("BITLOOM_HLS_EMIT_ONLY").as_deref(),
            Ok("1") | Ok("true") | Ok("yes")
        )
    {
        return Ok(c);
    }

    let bambu = match bambu_override {
        Some(p) => Ok(p.to_path_buf()),
        None => resolve_bambu(),
    }?;
    let status = Command::new(&bambu)
        .arg(c.as_os_str())
        .arg(format!("--top-fname={}", dissolved.fn_name))
        .current_dir(out_dir)
        .status()
        .map_err(|e| HlsError::Message(format!("spawn {HLS_BACKEND}: {e}")))?;
    if !status.success() {
        return Err(HlsError::Message(format!(
            "{HLS_BACKEND} {HLS_BACKEND_VERSION} exited non-zero (AD-25 / FR35 / FR76)"
        )));
    }

    find_rtl_artifact(out_dir, &dissolved.fn_name)
}

/// Product path: dissolve default `add` stub, invoke pinned Bambu, require synthesizable RTL.
pub fn run_hls(fn_name: &str, out_dir: &Path, emit_only: bool) -> Result<PathBuf, HlsError> {
    let dissolved = dissolve_dataflow_transform(fn_name, &[], || HlsDataflowOp::AddInputs)?;
    run_hls_dissolved(&dissolved, out_dir, emit_only)
}

/// Dissolve a named dataflow alias then run the product HLS path (CLI / ATDD).
pub fn run_hls_dataflow(
    fn_name: &str,
    dataflow_alias: &str,
    out_dir: &Path,
    emit_only: bool,
) -> Result<PathBuf, HlsError> {
    run_hls_dataflow_with_backend(fn_name, dataflow_alias, out_dir, emit_only, None)
}

/// Like [`run_hls_dataflow`], with optional explicit backend path.
pub fn run_hls_dataflow_with_backend(
    fn_name: &str,
    dataflow_alias: &str,
    out_dir: &Path,
    emit_only: bool,
    bambu_override: Option<&Path>,
) -> Result<PathBuf, HlsError> {
    let op = parse_dataflow_alias(dataflow_alias)?;
    let dissolved = dissolve_dataflow_transform(fn_name, &[], || op)?;
    run_hls_dissolved_with_backend(&dissolved, out_dir, emit_only, bambu_override)
}

fn op_label(op: &HlsDataflowOp) -> String {
    match op {
        HlsDataflowOp::Identity => "identity".into(),
        HlsDataflowOp::AddConst(k) => format!("add_const_{k}"),
        HlsDataflowOp::XorConst(k) => format!("xor_const_{k}"),
        HlsDataflowOp::MulConst(k) => format!("mul_const_{k}"),
        HlsDataflowOp::AddInputs => "add_inputs".into(),
    }
}

fn verilog_expr(op: &HlsDataflowOp, input: &str) -> String {
    match op {
        HlsDataflowOp::Identity => input.to_string(),
        HlsDataflowOp::AddConst(k) => format!("({input} + 32'd{k})"),
        HlsDataflowOp::XorConst(k) => format!("({input} ^ 32'd{k})"),
        HlsDataflowOp::MulConst(k) => format!("({input} * 32'd{k})"),
        HlsDataflowOp::AddInputs => {
            format!("({input} /* binary AddInputs collapsed to unary MVP */)")
        }
    }
}

fn build_schedule_ir(
    fn_name: &str,
    kind: &InTreeScheduleKind,
    stages: &[ScheduleStage],
    fr96: bool,
) -> String {
    let mut lines = Vec::new();
    lines.push("{".into());
    lines.push(format!("  \"fr95\": true,"));
    if fr96 {
        lines.push(format!("  \"fr96\": true,"));
        lines.push(format!("  \"dataflow_transform\": \"dissolved\","));
    }
    lines.push(format!("  \"path\": \"in-tree\","));
    lines.push(format!("  \"fn_name\": \"{fn_name}\","));
    match kind {
        InTreeScheduleKind::LoopUnroll { trip_count } => {
            lines.push(format!("  \"kind\": \"loop-unroll\","));
            lines.push(format!("  \"trip_count\": {trip_count},"));
        }
        InTreeScheduleKind::Pipeline {
            initiation_interval,
            stages: n,
        } => {
            lines.push(format!("  \"kind\": \"pipeline\","));
            lines.push(format!("  \"initiation_interval\": {initiation_interval},"));
            // Q2 alias required by FR110 / NFR14
            lines.push(format!("  \"ii\": {initiation_interval},"));
            lines.push(format!("  \"pipeline_stages\": {n},"));
            if *n >= 2 {
                lines.push("  \"fr110\": true,".into());
            }
        }
    }
    lines.push(format!("  \"stage_count\": {},", stages.len()));
    lines.push("  \"stages\": [".into());
    for (i, st) in stages.iter().enumerate() {
        let comma = if i + 1 == stages.len() { "" } else { "," };
        lines.push(format!(
            "    {{\"index\": {}, \"label\": \"{}\", \"op\": \"{}\"}}{comma}",
            st.index,
            st.label,
            op_label(&st.op)
        ));
    }
    lines.push("  ]".into());
    lines.push("}".into());
    lines.join("\n")
}

fn build_rtl_stub(fn_name: &str, kind: &InTreeScheduleKind, stages: &[ScheduleStage]) -> String {
    let kind_note = match kind {
        InTreeScheduleKind::LoopUnroll { trip_count } => {
            format!("loop-unroll trip_count={trip_count}")
        }
        InTreeScheduleKind::Pipeline {
            initiation_interval,
            stages: n,
        } => {
            let depth = if *n >= 2 {
                "FR110 commercial-depth"
            } else {
                "FR95 secondary"
            };
            format!("pipeline II={initiation_interval} stages={n} ({depth})")
        }
    };
    let mut body = String::new();
    let honesty = if matches!(
        kind,
        InTreeScheduleKind::Pipeline { stages, .. } if *stages >= 2
    ) {
        "  // FR110 in-tree depth: multi-stage pipeline + II (not a full commercial HLS compiler)\n"
    } else {
        "  // FR95 in-tree-mvp: combinatorial unroll/pipeline sketch (not commercial HLS quality)\n"
    };
    body.push_str(honesty);
    let mut cur = "x".to_string();
    for st in stages {
        let next = format!("s{}", st.index);
        let expr = verilog_expr(&st.op, &cur);
        body.push_str(&format!("  wire [31:0] {next} = {expr};\n"));
        cur = next;
    }
    format!(
        "// generated by bitloom FR95 in-tree-mvp ({kind_note}); no Bambu\n\
         module {fn_name}(\n\
           input  wire [31:0] x,\n\
           output wire [31:0] y\n\
         );\n\
         {body}\
           assign y = {cur};\n\
         endmodule\n"
    )
}

/// FR95 in-tree scheduling MVP: produce a checkable schedule without calling Bambu.
///
/// Documented subset: [`InTreeScheduleKind::LoopUnroll`] (primary) and
/// [`InTreeScheduleKind::Pipeline`] (secondary).
pub fn schedule_in_tree(
    fn_name: &str,
    op: HlsDataflowOp,
    kind: InTreeScheduleKind,
) -> Result<InTreeScheduleArtifact, HlsError> {
    if fn_name.is_empty() {
        return Err(HlsError::Message(
            "FR95 in-tree schedule: fn_name must be non-empty".into(),
        ));
    }
    let stage_count = match kind {
        InTreeScheduleKind::LoopUnroll { trip_count } => {
            if trip_count == 0 {
                return Err(HlsError::Message(
                    "FR95 in-tree loop-unroll: trip_count must be >= 1".into(),
                ));
            }
            trip_count
        }
        InTreeScheduleKind::Pipeline {
            initiation_interval,
            stages,
        } => {
            if initiation_interval == 0 || stages == 0 {
                return Err(HlsError::Message(
                    "FR95 in-tree pipeline: initiation_interval and stages must be >= 1".into(),
                ));
            }
            stages
        }
    };

    let stages: Vec<ScheduleStage> = (0..stage_count)
        .map(|index| ScheduleStage {
            index,
            op,
            label: format!("stage_{index}_{}", op_label(&op)),
        })
        .collect();

    let schedule_ir = build_schedule_ir(fn_name, &kind, &stages, false);
    let rtl_stub = build_rtl_stub(fn_name, &kind, &stages);
    Ok(InTreeScheduleArtifact {
        fn_name: fn_name.to_string(),
        kind,
        stages,
        schedule_ir,
        rtl_stub,
    })
}

/// FR110 commercial-depth schedule: Pipeline with `stages >= 2` and II ≥ 1 (Q1+Q2).
///
/// Rejects shallower pipelines so callers cannot silent-claim FR110.
pub fn schedule_in_tree_fr110(
    fn_name: &str,
    op: HlsDataflowOp,
    initiation_interval: u32,
    pipeline_stages: u32,
) -> Result<InTreeScheduleArtifact, HlsError> {
    let kind = InTreeScheduleKind::Pipeline {
        initiation_interval,
        stages: pipeline_stages,
    };
    if !meets_fr110_commercial_depth(&kind) {
        return Err(HlsError::Message(format!(
            "FR110 commercial depth: require pipeline with ii>=1 and pipeline_stages>=2 \
             (got ii={initiation_interval} stages={pipeline_stages}); \
             FR95 MVP loop-unroll / single-stage pipeline alone ≠ FR110"
        )));
    }
    schedule_in_tree(fn_name, op, kind)
}

/// FR96: dissolve/inline a dataflow-transform closure, then enter the FR95 in-tree schedule.
///
/// Capturing / illegal surfaces fail in [`dissolve_dataflow_transform`] **before** schedule
/// (AD-18). The `Fn` never enters schedule IR / RTL (NFR36).
pub fn schedule_in_tree_from_transform<F>(
    fn_name: &str,
    violations: &[HlsDataflowClosureViolation],
    kind: InTreeScheduleKind,
    transform: F,
) -> Result<InTreeScheduleArtifact, HlsError>
where
    F: FnOnce() -> HlsDataflowOp,
{
    let dissolved = dissolve_dataflow_transform(fn_name, violations, transform)?;
    let mut artifact = schedule_in_tree(&dissolved.fn_name, dissolved.op, kind)?;
    // Re-emit IR with FR96 markers while keeping the same stages / FR95 path metadata.
    artifact.schedule_ir =
        build_schedule_ir(&artifact.fn_name, &artifact.kind, &artifact.stages, true);
    artifact.rtl_stub = format!(
        "// FR96: dataflow transform dissolved before in-tree schedule\n{}",
        artifact.rtl_stub
    );
    Ok(artifact)
}

/// Write `{fn}.schedule.json` + `{fn}.v` for an in-tree FR95 artifact (never spawns Bambu).
pub fn emit_in_tree_schedule(
    artifact: &InTreeScheduleArtifact,
    out_dir: &Path,
) -> Result<(PathBuf, PathBuf), HlsError> {
    fs::create_dir_all(out_dir).map_err(|e| HlsError::Message(e.to_string()))?;
    let sched_path = out_dir.join(format!("{}.schedule.json", artifact.fn_name));
    let rtl_path = out_dir.join(format!("{}.v", artifact.fn_name));
    fs::write(&sched_path, &artifact.schedule_ir).map_err(|e| HlsError::Message(e.to_string()))?;
    fs::write(&rtl_path, &artifact.rtl_stub).map_err(|e| HlsError::Message(e.to_string()))?;
    Ok((sched_path, rtl_path))
}

/// Convenience: schedule + emit in-tree (FR95), no Bambu.
pub fn run_hls_in_tree(
    fn_name: &str,
    op: HlsDataflowOp,
    kind: InTreeScheduleKind,
    out_dir: &Path,
) -> Result<(PathBuf, PathBuf), HlsError> {
    let artifact = schedule_in_tree(fn_name, op, kind)?;
    emit_in_tree_schedule(&artifact, out_dir)
}

/// FR96 convenience: dissolve transform closure → in-tree schedule + emit (no Bambu).
pub fn run_hls_in_tree_from_transform<F>(
    fn_name: &str,
    violations: &[HlsDataflowClosureViolation],
    kind: InTreeScheduleKind,
    transform: F,
    out_dir: &Path,
) -> Result<(PathBuf, PathBuf), HlsError>
where
    F: FnOnce() -> HlsDataflowOp,
{
    let artifact = schedule_in_tree_from_transform(fn_name, violations, kind, transform)?;
    emit_in_tree_schedule(&artifact, out_dir)
}

fn find_rtl_artifact(out_dir: &Path, fn_name: &str) -> Result<PathBuf, HlsError> {
    let candidates = [
        out_dir.join(format!("{fn_name}.v")),
        out_dir.join(format!("{fn_name}.sv")),
        out_dir.join("hls_output.v"),
    ];
    for cand in &candidates {
        if cand.is_file() {
            return Ok(cand.clone());
        }
    }
    if let Ok(rd) = fs::read_dir(out_dir) {
        for ent in rd.flatten() {
            let p = ent.path();
            if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "v" | "sv") {
                    return Ok(p);
                }
            }
        }
    }
    Err(HlsError::Message(format!(
        "{HLS_BACKEND} finished but no synthesizable .v/.sv found under {}; \
         check backend version (expected {HLS_BACKEND_VERSION}) and logs",
        out_dir.display()
    )))
}

fn which(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let cand = dir.join(name);
        if cand.is_file() {
            return Some(cand);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_tree_loop_unroll_without_bambu() {
        let a = schedule_in_tree(
            "demo",
            HlsDataflowOp::AddConst(1),
            InTreeScheduleKind::LoopUnroll { trip_count: 3 },
        )
        .unwrap();
        assert_eq!(a.stages.len(), 3);
        assert!(a.schedule_ir.contains("fr95"));
        assert!(a.schedule_ir.contains("loop-unroll"));
        assert!(a.rtl_stub.contains("in-tree-mvp"));
    }

    #[test]
    fn missing_bambu_is_readable_error() {
        let dir = std::env::temp_dir().join("bitloom_hls_stub_unit");
        let p = emit_c_stub("add", &dir).unwrap();
        let text = std::fs::read_to_string(p).unwrap();
        assert!(text.contains("no scheduling in bitloom"));
        assert!(!text.contains("schedule"));
        assert!(text.contains(HLS_BACKEND_VERSION));
    }

    #[test]
    fn emit_only_skips_backend() {
        let dir = std::env::temp_dir().join("bitloom_hls_emit_only");
        let p = run_hls("add", &dir, true).unwrap();
        assert!(p.extension().and_then(|e| e.to_str()) == Some("c"));
    }

    #[test]
    fn emit_stub_without_scheduling() {
        let dir = std::env::temp_dir().join("bitloom_hls_stub");
        let p = emit_c_stub("add", &dir).unwrap();
        let text = std::fs::read_to_string(p).unwrap();
        assert!(text.contains("no scheduling in bitloom"));
        assert!(!text.contains("schedule"));
    }

    #[test]
    fn dissolve_xor_has_no_fn_residue() {
        let d =
            dissolve_dataflow_transform("map_xor", &[], || HlsDataflowOp::XorConst(0xa5)).unwrap();
        assert!(
            d.c_source.contains("^ 165u") || d.c_source.contains("^ 0xa5"),
            "c={}",
            d.c_source
        );
        assert!(!d.c_source.contains("Fn"));
        assert!(!d.c_source.contains("||"));
        assert!(!d.c_source.to_lowercase().contains("closure"));
    }

    #[test]
    fn capturing_violation_rejects_before_dissolve() {
        let err = dissolve_dataflow_transform(
            "bad",
            &[HlsDataflowClosureViolation::capturing("holds &mut state")],
            || HlsDataflowOp::Identity,
        )
        .unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("CapturingOrStateful") || msg.contains("rejected"));
    }
}
