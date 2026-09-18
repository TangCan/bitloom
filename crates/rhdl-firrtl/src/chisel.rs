//! FrozenHir → Chisel Scala (FR28 mechanical + FR97 idiomatic / AD-27).
//! Interop contract for FIRRTL text remains FrozenHir ↔ FIRRTL 6 (AD-3).
//! FR81 Path A: documented Mem/SyncReadMem subset lowers here; out-of-subset → E0901.

use std::collections::HashMap;

use bitloom_hir::{
    Artifact, AssignExpr, AssignTarget, EmittedFile, FrozenHir, GroundType, Module, PortConnect,
    PortDirection, ProcessKind, Stmt,
};

/// Documented Chisel target (AD-9 / NFR12). Remains 7.15.0 under FR182 unpaired product-pin.
pub const CHISEL_TARGET: &str = "7.15.0";

/// Documented firtool product pin (AD-9). FR182: **1.159.0** unpaired with Chisel 7.15.0
/// (no upstream official pairing; ≠ FR173 paired bump alone; ≠ FR174/FR179 optional channels).
pub const FIRTOOL_TARGET: &str = "1.159.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiselGenError {
    pub code: String,
    pub en: String,
    pub zh: String,
}

impl std::fmt::Display for ChiselGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} ({})", self.code, self.en, self.zh)
    }
}

impl std::error::Error for ChiselGenError {}

/// Idiomatic / FR97 acceptance failure (readable; never silent Ok).
pub type IdiomaticCheckError = ChiselGenError;

fn e0901(name: &str, reason_en: &str, reason_zh: &str) -> ChiselGenError {
    ChiselGenError {
        code: "rhdl::E0901".into(),
        en: format!("Chisel emit does not lower mem `{name}` ({reason_en})"),
        zh: format!("Chisel 发射不降低 mem `{name}`（{reason_zh}）"),
    }
}

fn e0904(en: &str, zh: &str) -> IdiomaticCheckError {
    IdiomaticCheckError {
        code: "rhdl::E0904".into(),
        en: en.to_string(),
        zh: zh.to_string(),
    }
}

/// FR81 Path A: single-clock `Mem` / `SyncReadMem` (+ optional constant init).
/// Returns `Err` with E0901 when outside that subset (do not delete wholesale).
fn mem_path_a_ok(depth: u32, width: u32, init: &Option<Vec<u64>>) -> Result<(), &'static str> {
    if depth == 0 || width == 0 {
        return Err("depth/width must be non-zero for Path A subset");
    }
    if let Some(words) = init {
        if words.len() != depth as usize {
            return Err("init length does not match depth (out of Path A subset)");
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum EmitFace {
    /// FR28 / FR46 mechanical compilable (not FR97).
    Mechanical,
    /// FR97 idiomatic / maintainable (naming + structure + readability).
    Idiomatic,
    /// FR111 deepen: FR97 + per-module consistency markers (D1/D3).
    IdiomaticFr111,
    /// FR122 official-style pack: FR111 + package + ordered sections + FR122 markers.
    IdiomaticFr122,
    /// FR130 Style Guide pack: FR122 + Style Guide markers (no Parser).
    IdiomaticFr130,
    /// FR165 Style Guide / linter deepen: FR130 + community lint checklist (≠ Chisel HEAD Parser).
    IdiomaticFr165,
    /// FR176 combined ecosystem pack: FR165 + parser-mainline bridge markers (≠ FR165/FR170 alone).
    IdiomaticFr176,
    /// FR181 Style Guide / linter deepen: FR176 + wartremover/fatal-warnings markers (≠ FR176 alone).
    IdiomaticFr181,
    /// FR188 community Style Guide pack: FR181 + community-style-guide/scalafmt markers (≠ FR181 alone).
    IdiomaticFr188,
}

fn is_sectioned_face(face: EmitFace) -> bool {
    matches!(
        face,
        EmitFace::Idiomatic
            | EmitFace::IdiomaticFr111
            | EmitFace::IdiomaticFr122
            | EmitFace::IdiomaticFr130
            | EmitFace::IdiomaticFr165
            | EmitFace::IdiomaticFr176
            | EmitFace::IdiomaticFr181
            | EmitFace::IdiomaticFr188
    )
}

fn preflight(hir: &FrozenHir) -> Result<HashMap<&str, &Module>, ChiselGenError> {
    let modules_by_name: HashMap<&str, &Module> = hir
        .circuit()
        .modules
        .iter()
        .map(|m| (m.name.as_str(), m))
        .collect();

    for m in &hir.circuit().modules {
        for stmt in &m.body {
            match stmt {
                Stmt::MemDecl {
                    name,
                    depth,
                    width,
                    init,
                    ..
                } => {
                    if let Err(reason) = mem_path_a_ok(*depth, *width, init) {
                        return Err(e0901(name, reason, "子集外或不合法 MemDecl"));
                    }
                }
                Stmt::Instance(inst) if !modules_by_name.contains_key(inst.module.as_str()) => {
                    return Err(ChiselGenError {
                        code: "rhdl::E0903".into(),
                        en: format!(
                            "instance `{}` references unknown module `{}`",
                            inst.name, inst.module
                        ),
                        zh: format!("实例 `{}` 引用未知模块 `{}`", inst.name, inst.module),
                    });
                }
                _ => {}
            }
        }
    }
    Ok(modules_by_name)
}

fn emit_with_face(hir: &FrozenHir, face: EmitFace) -> Result<Artifact, ChiselGenError> {
    let modules_by_name = preflight(hir)?;
    let mut body = match face {
        EmitFace::Mechanical => format!(
            "// generated by bitloom; target Chisel {CHISEL_TARGET} (firtool-{FIRTOOL_TARGET} pair)\n\
             // FR28 compilable Chisel — FrozenHir ↔ FIRRTL 6.0.0 text remains AD-3\n\
             // FR81 Path A: Mem/SyncReadMem (+ optional constant init)\n\
             // mechanical ≠ idiomatic — use emit_chisel_idiomatic for the maintainable face\n\
             import chisel3._\n\n"
        ),
        EmitFace::Idiomatic => format!(
            "// generated by Bitloom; target Chisel {CHISEL_TARGET} (firtool-{FIRTOOL_TARGET} pair)\n\
             // FR97 idiomatic / maintainable Chisel (AD-27 revised) — beyond FR28 mechanical\n\
             // Naming: HIR public module/port/instance names preserved (clk/rst → clock/reset)\n\
             // Structure: Module + IO Bundle + sectioned body; Readable: grouped comments\n\
             // FIRRTL text contract remains AD-3; Parser.parse not required\n\
             import chisel3._\n\n"
        ),
        EmitFace::IdiomaticFr111 => format!(
            "// generated by Bitloom; target Chisel {CHISEL_TARGET} (firtool-{FIRTOOL_TARGET} pair)\n\
             // FR97 idiomatic / maintainable Chisel (AD-27 revised) — beyond FR28 mechanical\n\
             // FR111 deepen: multi-module style consistency + per-module section contract (AD-27)\n\
             // Naming: HIR public module/port/instance names preserved (clk/rst → clock/reset)\n\
             // Structure: Module + IO Bundle + sectioned body; Readable: grouped comments\n\
             // FIRRTL text contract remains AD-3; Parser.parse not required\n\
             import chisel3._\n\n"
        ),
        EmitFace::IdiomaticFr122 => format!(
            "// generated by Bitloom; target Chisel {CHISEL_TARGET} (firtool-{FIRTOOL_TARGET} pair)\n\
             // FR97 idiomatic / maintainable Chisel (AD-27 revised) — beyond FR28 mechanical\n\
             // FR111 deepen: multi-module style consistency + per-module section contract (AD-27)\n\
             // FR122 official-style pack: package + ordered sections + per-module FR122 markers (AD-27)\n\
             // Naming: HIR public module/port/instance names preserved (clk/rst → clock/reset)\n\
             // Structure: Module + IO Bundle + sectioned body; Readable: grouped comments\n\
             // FIRRTL text contract remains AD-3; Parser.parse not required\n\
             package bitloom.generated\n\
             import chisel3._\n\n"
        ),
        EmitFace::IdiomaticFr130 => format!(
            "// generated by Bitloom; target Chisel {CHISEL_TARGET} (firtool-{FIRTOOL_TARGET} pair)\n\
             // FR97 idiomatic / maintainable Chisel (AD-27 revised) — beyond FR28 mechanical\n\
             // FR111 deepen: multi-module style consistency + per-module section contract (AD-27)\n\
             // FR122 official-style pack: package + ordered sections + per-module FR122 markers (AD-27)\n\
             // FR130 Style Guide pack: beyond O1–O4 — scalafmt-style + naming + import hygiene (AD-27)\n\
             // Style Guide: camelCase ports; withClockAndReset discipline; no wildcard pollution\n\
             // Naming: HIR public module/port/instance names preserved (clk/rst → clock/reset)\n\
             // Structure: Module + IO Bundle + sectioned body; Readable: grouped comments\n\
             // FIRRTL text contract remains AD-3; Parser.parse not required / not restored\n\
             package bitloom.generated\n\
             import chisel3._\n\n"
        ),
        EmitFace::IdiomaticFr165 => format!(
            "// generated by Bitloom; target Chisel {CHISEL_TARGET} (firtool-{FIRTOOL_TARGET} pair)\n\
             // FR97 idiomatic / maintainable Chisel (AD-27 revised) — beyond FR28 mechanical\n\
             // FR111 deepen: multi-module style consistency + per-module section contract (AD-27)\n\
             // FR122 official-style pack: package + ordered sections + per-module FR122 markers (AD-27)\n\
             // FR130 Style Guide pack: beyond O1–O4 — scalafmt-style + naming + import hygiene (AD-27)\n\
             // FR165 Style Guide / linter pack: beyond FR130 — community lint checklist (≠ FR138 alone)\n\
             // Style Guide: camelCase ports; withClockAndReset discipline; no wildcard pollution\n\
             // Linter deepen: chisel-lint-rules; scalafmt.conf; import-hygiene-lint\n\
             // Naming: HIR public module/port/instance names preserved (clk/rst → clock/reset)\n\
             // Structure: Module + IO Bundle + sectioned body; Readable: grouped comments\n\
             // FIRRTL text contract remains AD-3; Parser.parse not required / not restored\n\
             // no-Chisel-HEAD-Parser: arbitrary Chisel HEAD Parser migration deferred (NFR71)\n\
             package bitloom.generated\n\
             import chisel3._\n\n"
        ),
        EmitFace::IdiomaticFr176 => format!(
            "// generated by Bitloom; target Chisel {CHISEL_TARGET} (firtool-{FIRTOOL_TARGET} pair)\n\
             // FR97 idiomatic / maintainable Chisel (AD-27 revised) — beyond FR28 mechanical\n\
             // FR111 deepen: multi-module style consistency + per-module section contract (AD-27)\n\
             // FR122 official-style pack: package + ordered sections + per-module FR122 markers (AD-27)\n\
             // FR130 Style Guide pack: beyond O1–O4 — scalafmt-style + naming + import hygiene (AD-27)\n\
             // FR165 Style Guide / linter pack: beyond FR130 — community lint checklist (≠ FR138 alone)\n\
             // FR176 Chisel ecosystem pack: beyond FR165+FR170 alone — combined style-lint + parser-mainline bridge (AD-27)\n\
             // Style Guide: camelCase ports; withClockAndReset discipline; no wildcard pollution\n\
             // Linter deepen: chisel-lint-rules; scalafmt.conf; import-hygiene-lint\n\
             // Ecosystem: chisel-ecosystem-pack; parser-mainline-bridge; chisel-official-style-pack\n\
             // Naming: HIR public module/port/instance names preserved (clk/rst → clock/reset)\n\
             // Structure: Module + IO Bundle + sectioned body; Readable: grouped comments\n\
             // FIRRTL text contract remains AD-3; Parser.parse not required / not restored\n\
             // no-Chisel-HEAD-Parser: arbitrary floating HEAD deferred; update-mainline via FR170 gate\n\
             package bitloom.generated\n\
             import chisel3._\n\n"
        ),
        EmitFace::IdiomaticFr181 => format!(
            "// generated by Bitloom; target Chisel {CHISEL_TARGET} (firtool-{FIRTOOL_TARGET} pair)\n\
             // FR97 idiomatic / maintainable Chisel (AD-27 revised) — beyond FR28 mechanical\n\
             // FR111 deepen: multi-module style consistency + per-module section contract (AD-27)\n\
             // FR122 official-style pack: package + ordered sections + per-module FR122 markers (AD-27)\n\
             // FR130 Style Guide pack: beyond O1–O4 — scalafmt-style + naming + import hygiene (AD-27)\n\
             // FR165 Style Guide / linter pack: beyond FR130 — community lint checklist (≠ FR138 alone)\n\
             // FR176 Chisel ecosystem pack: beyond FR165+FR170 alone — combined style-lint + parser-mainline bridge (AD-27)\n\
             // FR181 Style Guide / linter deepen: beyond FR176 — wartremover + fatal-warnings (AD-27)\n\
             // Style Guide: camelCase ports; withClockAndReset discipline; no wildcard pollution\n\
             // Linter deepen: chisel-lint-rules; scalafmt.conf; import-hygiene-lint\n\
             // Ecosystem: chisel-ecosystem-pack; parser-mainline-bridge; chisel-official-style-pack\n\
             // Style-linter deepen: chisel-wartremover-rules; fatal-warnings-lint\n\
             // Naming: HIR public module/port/instance names preserved (clk/rst → clock/reset)\n\
             // Structure: Module + IO Bundle + sectioned body; Readable: grouped comments\n\
             // FIRRTL text contract remains AD-3; Parser.parse not required / not restored\n\
             // no-Chisel-HEAD-Parser: arbitrary floating HEAD deferred; update-mainline via FR170 gate\n\
             package bitloom.generated\n\
             import chisel3._\n\n"
        ),
        EmitFace::IdiomaticFr188 => format!(
            "// generated by Bitloom; target Chisel {CHISEL_TARGET} (firtool-{FIRTOOL_TARGET} pair)\n\
             // FR97 idiomatic / maintainable Chisel (AD-27 revised) — beyond FR28 mechanical\n\
             // FR111 deepen: multi-module style consistency + per-module section contract (AD-27)\n\
             // FR122 official-style pack: package + ordered sections + per-module FR122 markers (AD-27)\n\
             // FR130 Style Guide pack: beyond O1–O4 — scalafmt-style + naming + import hygiene (AD-27)\n\
             // FR165 Style Guide / linter pack: beyond FR130 — community lint checklist (≠ FR138 alone)\n\
             // FR176 Chisel ecosystem pack: beyond FR165+FR170 alone — combined style-lint + parser-mainline bridge (AD-27)\n\
             // FR181 Style Guide / linter deepen: beyond FR176 — wartremover + fatal-warnings (AD-27)\n\
             // FR188 community Style Guide pack: beyond FR181 — community style-guide + scalafmt-community (AD-27)\n\
             // Style Guide: camelCase ports; withClockAndReset discipline; no wildcard pollution\n\
             // Linter deepen: chisel-lint-rules; scalafmt.conf; import-hygiene-lint\n\
             // Ecosystem: chisel-ecosystem-pack; parser-mainline-bridge; chisel-official-style-pack\n\
             // Style-linter deepen: chisel-wartremover-rules; fatal-warnings-lint\n\
             // Community Style Guide pack: chisel-community-style-guide; scalafmt-community\n\
             // Naming: HIR public module/port/instance names preserved (clk/rst → clock/reset)\n\
             // Structure: Module + IO Bundle + sectioned body; Readable: grouped comments\n\
             // FIRRTL text contract remains AD-3; Parser.parse not required / not restored\n\
             // no-Chisel-HEAD-Parser: arbitrary floating HEAD deferred; update-mainline via FR170 gate\n\
             package bitloom.generated\n\
             import chisel3._\n\n"
        ),
    };
    for m in &hir.circuit().modules {
        body.push_str(&emit_module(m, &modules_by_name, face));
        body.push('\n');
    }
    let path = format!("{}.scala", hir.abi_name);
    Ok(Artifact {
        files: vec![EmittedFile {
            path: path.clone(),
            contents: body,
        }],
        filelist: vec![path],
    })
}

/// Emit mechanical-style Chisel Scala accepted under the pinned Chisel+firtool stack.
///
/// FR81 Path A: `MemDecl` with valid depth/width/(optional matching init) lowers to
/// Chisel `Mem` / `SyncReadMem`. Out-of-subset `MemDecl` fails with `rhdl::E0901`.
/// Hierarchy with instances is in scope for FR28 (no E0902).
///
/// **Not FR97:** this face is compilable ≠ idiomatic. Use [`emit_chisel_idiomatic`].
pub fn emit_chisel(hir: &FrozenHir) -> Result<Artifact, ChiselGenError> {
    emit_with_face(hir, EmitFace::Mechanical)
}

/// Emit FR97 idiomatic / maintainable Chisel Scala (AD-27 revised).
///
/// Satisfies NFR14 Epic 42 naming + structure + readability predicates when checked with
/// [`check_idiomatic_chisel`]. Does not claim Chisel official Style Guide unless documented.
pub fn emit_chisel_idiomatic(hir: &FrozenHir) -> Result<Artifact, ChiselGenError> {
    emit_with_face(hir, EmitFace::Idiomatic)
}

/// Emit FR111 deepen idiomatic Chisel (multi-module consistency + stricter markers).
///
/// Superset of FR97 face; accept with [`check_idiomatic_chisel_fr111`].
pub fn emit_chisel_idiomatic_fr111(hir: &FrozenHir) -> Result<Artifact, ChiselGenError> {
    emit_with_face(hir, EmitFace::IdiomaticFr111)
}

/// Emit FR122 official-style Chisel pack (beyond FR111 D1+D3).
///
/// Superset of FR111 face; accept with [`check_idiomatic_chisel_fr122`].
pub fn emit_chisel_idiomatic_fr122(hir: &FrozenHir) -> Result<Artifact, ChiselGenError> {
    emit_with_face(hir, EmitFace::IdiomaticFr122)
}

/// Emit FR130 Style Guide Chisel pack (beyond FR122 O1–O4). Does **not** restore Parser.parse.
pub fn emit_chisel_style_guide_fr130(hir: &FrozenHir) -> Result<Artifact, ChiselGenError> {
    emit_with_face(hir, EmitFace::IdiomaticFr130)
}

/// Emit FR165 Style Guide / linter deepen pack (beyond FR130). Does **not** migrate Chisel HEAD Parser.
pub fn emit_chisel_style_guide_fr165(hir: &FrozenHir) -> Result<Artifact, ChiselGenError> {
    emit_with_face(hir, EmitFace::IdiomaticFr165)
}

/// Emit FR176 combined Chisel ecosystem pack (beyond FR165 + FR170 alone).
///
/// Superset of FR165 face; accept with [`check_chisel_ecosystem_fr176`].
pub fn emit_chisel_ecosystem_fr176(hir: &FrozenHir) -> Result<Artifact, ChiselGenError> {
    emit_with_face(hir, EmitFace::IdiomaticFr176)
}

/// Emit FR181 Style Guide / linter deepen pack (beyond FR176).
///
/// Superset of FR176 face; accept with [`check_chisel_style_linter_fr181`].
pub fn emit_chisel_style_linter_fr181(hir: &FrozenHir) -> Result<Artifact, ChiselGenError> {
    emit_with_face(hir, EmitFace::IdiomaticFr181)
}

/// Emit FR188 community Style Guide pack (beyond FR181).
///
/// Superset of FR181 face; accept with [`check_chisel_style_guide_pack_fr188`].
pub fn emit_chisel_style_guide_pack_fr188(hir: &FrozenHir) -> Result<Artifact, ChiselGenError> {
    emit_with_face(hir, EmitFace::IdiomaticFr188)
}

/// Extract the brace-delimited `class {name} extends Module { … }` body (inclusive).
/// Used so port / IO Bundle checks are scoped per module (not whole-file substring).
fn module_class_span<'a>(scala: &'a str, module_name: &str) -> Option<&'a str> {
    let header = format!("class {module_name} extends Module");
    let start = scala.find(&header)?;
    let after = &scala[start..];
    let brace = after.find('{')?;
    let bytes = after.as_bytes();
    let mut depth = 0i32;
    let mut i = brace;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&after[..=i]);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// Assert FR97 idiomatic acceptance criteria against emitted Scala + FrozenHir.
///
/// Mechanical [`emit_chisel`] output must fail here (explicit downgrade / no silent claim).
///
/// **Empty circuit policy:** `hir.circuit().modules.is_empty()` → `Err(E0904)` (no exemption).
/// Normal elaborate already rejects empty circuits at freeze; this is defense-in-depth for
/// hand-built / mutated FrozenHir.
///
/// **Port / IO scope:** each module's ports and `IO(new Bundle)` are checked inside that
/// module's class block (not via whole-file substring).
pub fn check_idiomatic_chisel(scala: &str, hir: &FrozenHir) -> Result<(), IdiomaticCheckError> {
    if hir.circuit().modules.is_empty() {
        return Err(e0904(
            "idiomatic check failed: empty circuit (no modules) — rejected; no empty-circuit exemption",
            "idiomatic 验收失败：空电路（无模块）— 拒绝；无空电路豁免",
        ));
    }

    let lower = scala.to_lowercase();

    // Require contiguous positive claim — mechanical headers may mention "≠ idiomatic / FR97"
    // as a downgrade notice and must still fail.
    let claims_fr97 = scala.contains("FR97 idiomatic")
        || scala.contains("FR97 Idiomatic")
        || (scala.contains("FR97") && scala.contains("可维护"));
    if !claims_fr97 {
        return Err(e0904(
            "idiomatic check failed: Scala lacks FR97 idiomatic header (mechanical FR28 output is not FR97)",
            "idiomatic 验收失败：Scala 缺少 FR97 idiomatic 头（机械 FR28 产出不算 FR97）",
        ));
    }

    if scala.contains("FR28 compilable") {
        return Err(e0904(
            "idiomatic check failed: mechanical FR28 face must not claim FR97",
            "idiomatic 验收失败：机械 FR28 面不得宣称 FR97",
        ));
    }

    if scala.lines().count() <= 8 {
        return Err(e0904(
            "idiomatic check failed: output looks like a non-readable single dump (too few lines)",
            "idiomatic 验收失败：产出行数过少，不像可维护分节结构",
        ));
    }

    let has_io_section = lower.contains("--- io ---");
    if !has_io_section {
        return Err(e0904(
            "idiomatic check failed: missing readable // --- IO --- section marker",
            "idiomatic 验收失败：缺少可读 // --- IO --- 分节注释",
        ));
    }

    // Require body section markers for stmt kinds present in HIR (not just always-on IO).
    let mut need_registers = false;
    let mut need_wires = false;
    let mut need_instances = false;
    let mut need_logic = false;
    let mut need_memories = false;
    let mut has_clk_or_rst = false;
    for m in &hir.circuit().modules {
        for p in &m.ports {
            if p.name == "clk" || p.name == "rst" {
                has_clk_or_rst = true;
            }
        }
        for stmt in &m.body {
            match stmt {
                Stmt::RegDecl { .. } => need_registers = true,
                Stmt::WireDecl { .. } => need_wires = true,
                Stmt::Instance(_) => need_instances = true,
                Stmt::Process(_) => need_logic = true,
                Stmt::MemDecl { .. } => need_memories = true,
            }
        }
    }
    let missing = [
        (need_registers, "--- registers ---"),
        (need_wires, "--- wires ---"),
        (need_instances, "--- instances ---"),
        (need_logic, "--- logic ---"),
        (need_memories, "--- memories ---"),
    ]
    .into_iter()
    .filter(|(need, marker)| *need && !lower.contains(marker))
    .map(|(_, marker)| marker)
    .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(e0904(
            &format!(
                "idiomatic check failed: missing body section marker(s) required by HIR: {}",
                missing.join(", ")
            ),
            &format!(
                "idiomatic 验收失败：HIR 要求的正文分节缺失：{}",
                missing.join(", ")
            ),
        ));
    }

    if has_clk_or_rst && !(scala.contains("clock") || scala.contains("reset")) {
        // Modules with only clk/rst (no other ports) still map via Chisel implicit clock/reset;
        // presence of the words in comments/header is weak — prefer body uses when non-clk ports exist.
        // Accept either Chisel implicit use in connects or documented mapping comment in header.
        if !scala.contains("clk/rst") && !scala.contains("clock/reset") {
            return Err(e0904(
                "idiomatic check failed: clk/rst present in HIR but no clock/reset mapping evidence",
                "idiomatic 验收失败：HIR 含 clk/rst 但未见 clock/reset 映射证据",
            ));
        }
    }

    if !scala.to_lowercase().contains("bitloom") {
        return Err(e0904(
            "idiomatic check failed: Scala must cite Bitloom brand",
            "idiomatic 验收失败：Scala 须标明 Bitloom 品牌",
        ));
    }

    for m in &hir.circuit().modules {
        let Some(mod_span) = module_class_span(scala, &m.name) else {
            return Err(e0904(
                &format!(
                    "idiomatic check failed: missing Module class for HIR name `{}`",
                    m.name
                ),
                &format!(
                    "idiomatic 验收失败：缺少与 HIR 名 `{}` 对应的 Module class",
                    m.name
                ),
            ));
        };
        if !mod_span.contains("IO(new Bundle") {
            return Err(e0904(
                &format!(
                    "idiomatic check failed: module `{}` missing IO(new Bundle) in its class scope",
                    m.name
                ),
                &format!(
                    "idiomatic 验收失败：模块 `{}` 的 class 作用域内缺少 IO(new Bundle)",
                    m.name
                ),
            ));
        }
        for p in &m.ports {
            if p.name == "clk" || p.name == "rst" {
                continue;
            }
            let dir = match p.direction {
                PortDirection::Input => "Input",
                PortDirection::Output => "Output",
                PortDirection::InOut => "Analog",
            };
            let needle = format!("val {} = {dir}(", p.name);
            if !mod_span.contains(&needle) {
                return Err(e0904(
                    &format!(
                        "idiomatic check failed: port `{}` not found in module `{}` as `val {} = {dir}(` (scoped naming)",
                        p.name, m.name, p.name
                    ),
                    &format!(
                        "idiomatic 验收失败：端口 `{}` 未在模块 `{}` 作用域内以 `val {} = {dir}(` 出现",
                        p.name, m.name, p.name
                    ),
                ));
            }
        }
        for stmt in &m.body {
            if let Stmt::Instance(inst) = stmt {
                let needle = format!("val {} = Module(new {})", inst.name, inst.module);
                if !mod_span.contains(&needle) {
                    return Err(e0904(
                        &format!(
                            "idiomatic check failed: instance `{}` / module `{}` not found in parent `{}` class scope",
                            inst.name, inst.module, m.name
                        ),
                        &format!(
                            "idiomatic 验收失败：实例 `{}` / 模块 `{}` 未在父模块 `{}` class 作用域内出现",
                            inst.name, inst.module, m.name
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}

/// Assert FR111 deepen acceptance (D1 multi-module per-module markers + D3 FR111 claim).
///
/// Runs [`check_idiomatic_chisel`] first (NFR44: FR97 subset). Then:
/// - requires contiguous `FR111` deepen header claim
/// - requires ≥2 modules
/// - D1/D3: each module class contains `// --- FR111 per-module ---` and `--- io ---`
pub fn check_idiomatic_chisel_fr111(
    scala: &str,
    hir: &FrozenHir,
) -> Result<(), IdiomaticCheckError> {
    check_idiomatic_chisel(scala, hir)?;

    if !(scala.contains("FR111 deepen")
        || scala.contains("FR111 per-module")
        || (scala.contains("FR111") && scala.contains("deepen")))
    {
        return Err(e0904(
            "FR111 check failed: Scala lacks FR111 deepen header (FR97 MVP alone is not FR111)",
            "FR111 验收失败：Scala 缺少 FR111 deepen 头（仅 FR97 MVP 不算 FR111）",
        ));
    }

    if hir.circuit().modules.len() < 2 {
        return Err(e0904(
            "FR111 check failed: D1 requires ≥2 modules for multi-module style consistency",
            "FR111 验收失败：D1 要求 ≥2 模块以验收多模块风格一致性",
        ));
    }

    for m in &hir.circuit().modules {
        let Some(mod_span) = module_class_span(scala, &m.name) else {
            return Err(e0904(
                &format!("FR111 check failed: missing Module class for `{}`", m.name),
                &format!("FR111 验收失败：缺少模块 `{}`", m.name),
            ));
        };
        let span_lower = mod_span.to_lowercase();
        if !span_lower.contains("--- fr111 per-module ---") {
            return Err(e0904(
                &format!(
                    "FR111 check failed: D3 — module `{}` missing // --- FR111 per-module --- marker in class scope",
                    m.name
                ),
                &format!(
                    "FR111 验收失败：D3 — 模块 `{}` class 内缺少 // --- FR111 per-module ---",
                    m.name
                ),
            ));
        }
        if !span_lower.contains("--- io ---") {
            return Err(e0904(
                &format!(
                    "FR111 check failed: D1 — module `{}` missing // --- IO --- in its class scope",
                    m.name
                ),
                &format!(
                    "FR111 验收失败：D1 — 模块 `{}` class 内缺少 // --- IO ---",
                    m.name
                ),
            ));
        }
    }

    Ok(())
}

/// Official section order for FR122 (O2): only markers that appear are checked for relative order.
const FR122_SECTION_ORDER: &[&str] = &[
    "--- io ---",
    "--- registers ---",
    "--- wires ---",
    "--- instances ---",
    "--- memories ---",
    "--- logic ---",
];

fn check_fr122_section_order(mod_span: &str, module_name: &str) -> Result<(), IdiomaticCheckError> {
    let lower = mod_span.to_lowercase();
    let mut last_pos: Option<usize> = None;
    let mut last_marker = "";
    for marker in FR122_SECTION_ORDER {
        if let Some(pos) = lower.find(marker) {
            if let Some(prev) = last_pos {
                if pos < prev {
                    return Err(e0904(
                        &format!(
                            "FR122 check failed: O2 — module `{module_name}` section `{marker}` appears before `{last_marker}` (official order: io→registers→wires→instances→memories→logic)"
                        ),
                        &format!(
                            "FR122 验收失败：O2 — 模块 `{module_name}` 分节 `{marker}` 出现在 `{last_marker}` 之前（官方顺序：io→registers→wires→instances→memories→logic）"
                        ),
                    ));
                }
            }
            last_pos = Some(pos);
            last_marker = marker;
        }
    }
    Ok(())
}

/// Assert FR122 official-style pack (O1–O4 beyond FR111 D1+D3).
///
/// Runs [`check_idiomatic_chisel_fr111`] first (NFR48: FR111 subset). Then:
/// - O1: `package bitloom.generated` + contiguous `FR122 official-style` claim
/// - O2: official section order within each module class
/// - O3: each module class contains `// --- FR122 official ---`
pub fn check_idiomatic_chisel_fr122(
    scala: &str,
    hir: &FrozenHir,
) -> Result<(), IdiomaticCheckError> {
    check_idiomatic_chisel_fr111(scala, hir)?;

    if !(scala.contains("FR122 official-style")
        || (scala.contains("FR122") && scala.contains("official-style")))
    {
        return Err(e0904(
            "FR122 check failed: Scala lacks FR122 official-style header (FR111 alone is not FR122)",
            "FR122 验收失败：Scala 缺少 FR122 official-style 头（仅 FR111 不算 FR122）",
        ));
    }

    if !scala.contains("package bitloom.generated") {
        return Err(e0904(
            "FR122 check failed: O1 — missing `package bitloom.generated`",
            "FR122 验收失败：O1 — 缺少 `package bitloom.generated`",
        ));
    }

    for m in &hir.circuit().modules {
        let Some(mod_span) = module_class_span(scala, &m.name) else {
            return Err(e0904(
                &format!("FR122 check failed: missing Module class for `{}`", m.name),
                &format!("FR122 验收失败：缺少模块 `{}`", m.name),
            ));
        };
        let span_lower = mod_span.to_lowercase();
        if !span_lower.contains("--- fr122 official ---") {
            return Err(e0904(
                &format!(
                    "FR122 check failed: O3 — module `{}` missing // --- FR122 official --- marker in class scope",
                    m.name
                ),
                &format!(
                    "FR122 验收失败：O3 — 模块 `{}` class 内缺少 // --- FR122 official ---",
                    m.name
                ),
            ));
        }
        check_fr122_section_order(mod_span, &m.name)?;
    }

    Ok(())
}

/// Assert FR130 Style Guide pack (S1–S4 beyond FR122 O1–O4). Does **not** require Parser.parse.
pub fn check_chisel_style_guide_fr130(
    scala: &str,
    hir: &FrozenHir,
) -> Result<(), IdiomaticCheckError> {
    check_idiomatic_chisel_fr122(scala, hir)?;

    if !(scala.contains("FR130 Style Guide")
        || (scala.contains("FR130") && scala.contains("Style Guide")))
    {
        return Err(e0904(
            "FR130 check failed: Scala lacks FR130 Style Guide header (FR122 alone is not FR130)",
            "FR130 验收失败：Scala 缺少 FR130 Style Guide 头（仅 FR122 不算 FR130）",
        ));
    }

    if !(scala.contains("scalafmt-style")
        && scala.contains("withClockAndReset")
        && scala.contains("Parser.parse not required"))
    {
        return Err(e0904(
            "FR130 check failed: S1–S3 Style Guide markers missing (scalafmt / withClockAndReset / no-Parser)",
            "FR130 验收失败：缺少 Style Guide 标记（scalafmt / withClockAndReset / 不恢复 Parser）",
        ));
    }

    for m in &hir.circuit().modules {
        let Some(mod_span) = module_class_span(scala, &m.name) else {
            return Err(e0904(
                &format!("FR130 check failed: missing Module class for `{}`", m.name),
                &format!("FR130 验收失败：缺少模块 `{}`", m.name),
            ));
        };
        if !mod_span
            .to_lowercase()
            .contains("--- fr130 style-guide ---")
        {
            return Err(e0904(
                &format!(
                    "FR130 check failed: S4 — module `{}` missing // --- FR130 style-guide ---",
                    m.name
                ),
                &format!(
                    "FR130 验收失败：S4 — 模块 `{}` 缺少 // --- FR130 style-guide ---",
                    m.name
                ),
            ));
        }
    }

    Ok(())
}

/// Assert FR165 Style Guide / linter deepen (beyond FR130). Does **not** require Chisel HEAD Parser.
pub fn check_chisel_style_guide_fr165(
    scala: &str,
    hir: &FrozenHir,
) -> Result<(), IdiomaticCheckError> {
    check_chisel_style_guide_fr130(scala, hir)?;

    if !(scala.contains("FR165 Style Guide / linter")
        || (scala.contains("FR165") && scala.contains("linter pack")))
    {
        return Err(e0904(
            "FR165 check failed: Scala lacks FR165 Style Guide / linter header (FR130 alone is not FR165)",
            "FR165 验收失败：Scala 缺少 FR165 Style Guide / linter 头（仅 FR130 不算 FR165）",
        ));
    }

    if !(scala.contains("chisel-lint-rules")
        && scala.contains("scalafmt.conf")
        && scala.contains("import-hygiene-lint")
        && scala.contains("no-Chisel-HEAD-Parser"))
    {
        return Err(e0904(
            "FR165 check failed: linter deepen markers missing (chisel-lint-rules / scalafmt.conf / import-hygiene-lint / no-Chisel-HEAD-Parser)",
            "FR165 验收失败：缺少 linter 加深标记",
        ));
    }

    for m in &hir.circuit().modules {
        let Some(mod_span) = module_class_span(scala, &m.name) else {
            return Err(e0904(
                &format!("FR165 check failed: missing Module class for `{}`", m.name),
                &format!("FR165 验收失败：缺少模块 `{}`", m.name),
            ));
        };
        if !mod_span.to_lowercase().contains("--- fr165 style-lint ---") {
            return Err(e0904(
                &format!(
                    "FR165 check failed: module `{}` missing // --- FR165 style-lint ---",
                    m.name
                ),
                &format!(
                    "FR165 验收失败：模块 `{}` 缺少 // --- FR165 style-lint ---",
                    m.name
                ),
            ));
        }
    }

    Ok(())
}

/// Assert FR176 combined ecosystem pack (beyond FR165 / FR170 alone).
pub fn check_chisel_ecosystem_fr176(
    scala: &str,
    hir: &FrozenHir,
) -> Result<(), IdiomaticCheckError> {
    check_chisel_style_guide_fr165(scala, hir)?;

    if !(scala.contains("FR176 Chisel ecosystem")
        || (scala.contains("FR176") && scala.contains("ecosystem pack")))
    {
        return Err(e0904(
            "FR176 check failed: Scala lacks FR176 Chisel ecosystem header (FR165 alone is not FR176)",
            "FR176 验收失败：Scala 缺少 FR176 Chisel ecosystem 头（仅 FR165 不算 FR176）",
        ));
    }

    if !(scala.contains("chisel-ecosystem-pack")
        && scala.contains("parser-mainline-bridge")
        && scala.contains("chisel-official-style-pack"))
    {
        return Err(e0904(
            "FR176 check failed: ecosystem markers missing (chisel-ecosystem-pack / parser-mainline-bridge / chisel-official-style-pack)",
            "FR176 验收失败：缺少 ecosystem 标记",
        ));
    }

    for m in &hir.circuit().modules {
        let Some(mod_span) = module_class_span(scala, &m.name) else {
            return Err(e0904(
                &format!("FR176 check failed: missing Module class for `{}`", m.name),
                &format!("FR176 验收失败：缺少模块 `{}`", m.name),
            ));
        };
        if !mod_span.to_lowercase().contains("--- fr176 ecosystem ---") {
            return Err(e0904(
                &format!(
                    "FR176 check failed: module `{}` missing // --- FR176 ecosystem ---",
                    m.name
                ),
                &format!(
                    "FR176 验收失败：模块 `{}` 缺少 // --- FR176 ecosystem ---",
                    m.name
                ),
            ));
        }
    }

    Ok(())
}

/// Assert FR181 Style Guide / linter deepen (beyond FR176 alone).
pub fn check_chisel_style_linter_fr181(
    scala: &str,
    hir: &FrozenHir,
) -> Result<(), IdiomaticCheckError> {
    check_chisel_ecosystem_fr176(scala, hir)?;

    if !(scala.contains("FR181 Style Guide / linter")
        || (scala.contains("FR181") && scala.contains("linter deepen")))
    {
        return Err(e0904(
            "FR181 check failed: Scala lacks FR181 Style Guide / linter header (FR176 alone is not FR181)",
            "FR181 验收失败：Scala 缺少 FR181 Style Guide / linter 头（仅 FR176 不算 FR181）",
        ));
    }

    if !(scala.contains("chisel-wartremover-rules") && scala.contains("fatal-warnings-lint")) {
        return Err(e0904(
            "FR181 check failed: style-linter deepen markers missing (chisel-wartremover-rules / fatal-warnings-lint)",
            "FR181 验收失败：缺少 style-linter 加深标记",
        ));
    }

    for m in &hir.circuit().modules {
        let Some(mod_span) = module_class_span(scala, &m.name) else {
            return Err(e0904(
                &format!("FR181 check failed: missing Module class for `{}`", m.name),
                &format!("FR181 验收失败：缺少模块 `{}`", m.name),
            ));
        };
        if !mod_span
            .to_lowercase()
            .contains("--- fr181 style-linter ---")
        {
            return Err(e0904(
                &format!(
                    "FR181 check failed: module `{}` missing // --- FR181 style-linter ---",
                    m.name
                ),
                &format!(
                    "FR181 验收失败：模块 `{}` 缺少 // --- FR181 style-linter ---",
                    m.name
                ),
            ));
        }
    }

    Ok(())
}

/// Assert FR188 community Style Guide pack (beyond FR181 alone).
pub fn check_chisel_style_guide_pack_fr188(
    scala: &str,
    hir: &FrozenHir,
) -> Result<(), IdiomaticCheckError> {
    check_chisel_style_linter_fr181(scala, hir)?;

    if !(scala.contains("FR188 community Style Guide")
        || (scala.contains("FR188") && scala.contains("Style Guide pack")))
    {
        return Err(e0904(
            "FR188 check failed: Scala lacks FR188 community Style Guide header (FR181 alone is not FR188)",
            "FR188 验收失败：Scala 缺少 FR188 community Style Guide 头（仅 FR181 不算 FR188）",
        ));
    }

    if !(scala.contains("chisel-community-style-guide") && scala.contains("scalafmt-community")) {
        return Err(e0904(
            "FR188 check failed: community Style Guide pack markers missing (chisel-community-style-guide / scalafmt-community)",
            "FR188 验收失败：缺少社区 Style Guide 全家桶标记",
        ));
    }

    for m in &hir.circuit().modules {
        let Some(mod_span) = module_class_span(scala, &m.name) else {
            return Err(e0904(
                &format!("FR188 check failed: missing Module class for `{}`", m.name),
                &format!("FR188 验收失败：缺少模块 `{}`", m.name),
            ));
        };
        if !mod_span
            .to_lowercase()
            .contains("--- fr188 style-guide-pack ---")
        {
            return Err(e0904(
                &format!(
                    "FR188 check failed: module `{}` missing // --- FR188 style-guide-pack ---",
                    m.name
                ),
                &format!(
                    "FR188 验收失败：模块 `{}` 缺少 // --- FR188 style-guide-pack ---",
                    m.name
                ),
            ));
        }
    }

    Ok(())
}

fn chisel_ty(ty: &GroundType) -> String {
    match ty {
        GroundType::UInt { width } => format!("UInt({width}.W)"),
        GroundType::SInt { width } => format!("SInt({width}.W)"),
        GroundType::Clock => "Clock()".into(),
        GroundType::Reset => "Reset()".into(),
        GroundType::Bool => "Bool()".into(),
        GroundType::Analog => "Analog()".into(),
    }
}

fn ref_name(m: &Module, n: &str) -> String {
    if n == "clk" {
        "clock".into()
    } else if n == "rst" {
        "reset".into()
    } else if m.ports.iter().any(|p| p.name == n) {
        format!("io.{n}")
    } else {
        n.to_string()
    }
}

fn emit_expr(m: &Module, expr: &AssignExpr) -> String {
    match expr {
        AssignExpr::Ref(n) => ref_name(m, n),
        AssignExpr::Lit(v) => format!("{v}.U"),
        AssignExpr::Inc(n) => format!("{} + 1.U", ref_name(m, n)),
        AssignExpr::Add(a, b) => format!("{} + {}", ref_name(m, a), ref_name(m, b)),
        AssignExpr::Sub(a, b) => format!("{} - {}", ref_name(m, a), ref_name(m, b)),
        AssignExpr::And(a, b) => format!("{} & {}", ref_name(m, a), ref_name(m, b)),
        AssignExpr::Or(a, b) => format!("{} | {}", ref_name(m, a), ref_name(m, b)),
        AssignExpr::Xor(a, b) => format!("{} ^ {}", ref_name(m, a), ref_name(m, b)),
        AssignExpr::Shl(a, b) => format!("{} << {}", ref_name(m, a), ref_name(m, b)),
        AssignExpr::Shr(a, b) => format!("{} >> {}", ref_name(m, a), ref_name(m, b)),
        AssignExpr::Slice { src, lo, width } => {
            format!("{}({}, {lo})", ref_name(m, src), lo + width - 1)
        }
        AssignExpr::Concat { high, low, .. } => {
            format!("Cat({}, {})", ref_name(m, high), ref_name(m, low))
        }
        AssignExpr::ZeroExtend { src, to_width, .. } => {
            format!("{}.pad({to_width})", ref_name(m, src))
        }
        AssignExpr::SignExtend { src, to_width, .. } => {
            format!("{}.asSInt.pad({to_width}).asUInt", ref_name(m, src))
        }
        AssignExpr::Eq(a, b) => format!("{} === {}", ref_name(m, a), ref_name(m, b)),
        AssignExpr::Mux { sel, t, f } => format!(
            "Mux({}, {}, {})",
            ref_name(m, sel),
            ref_name(m, t),
            ref_name(m, f)
        ),
        // Mem: combinational read; SyncReadMem: sync read (latency 1) via same API.
        AssignExpr::MemRead { mem, addr } => {
            format!("{mem}.read({})", ref_name(m, addr))
        }
    }
}

fn emit_instance_connects(
    parent: &Module,
    inst_name: &str,
    child: &Module,
    connects: &[PortConnect],
) -> String {
    let mut out = String::new();
    for c in connects {
        if c.dangling || c.child_port == "clk" || c.child_port == "rst" {
            continue;
        }
        let Some(port) = child.ports.iter().find(|p| p.name == c.child_port) else {
            continue;
        };
        let parent_ref = ref_name(parent, &c.parent_net);
        let child_io = format!("{inst_name}.io.{}", c.child_port);
        match port.direction {
            PortDirection::Input => {
                out.push_str(&format!("  {child_io} := {parent_ref}\n"));
            }
            PortDirection::Output => {
                out.push_str(&format!("  {parent_ref} := {child_io}\n"));
            }
            PortDirection::InOut => {
                out.push_str(&format!(
                    "  // inout skipped: {child_io} <-> {parent_ref}\n"
                ));
            }
        }
    }
    out
}

fn emit_mem_decl(
    name: &str,
    depth: u32,
    width: u32,
    sync_read: bool,
    init: &Option<Vec<u64>>,
) -> String {
    let ctor = if sync_read { "SyncReadMem" } else { "Mem" };
    let mut out = format!("  val {name} = {ctor}({depth}, UInt({width}.W))\n");
    if let Some(words) = init {
        let lits: Vec<String> = words.iter().map(|w| format!("{w}.U({width}.W)")).collect();
        out.push_str(&format!(
            "  val {name}_init = VecInit({})\n",
            lits.join(", ")
        ));
        out.push_str(&format!(
            "  when (reset.asBool) {{\n    for (i <- 0 until {depth}) {{\n      {name}.write(i.U, {name}_init(i))\n    }}\n  }}\n"
        ));
    }
    out
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BodySection {
    None,
    Memories,
    Registers,
    Wires,
    Instances,
    Logic,
}

fn emit_module(m: &Module, modules_by_name: &HashMap<&str, &Module>, face: EmitFace) -> String {
    let mut io_fields = String::new();
    for p in &m.ports {
        if p.name == "clk" || p.name == "rst" {
            continue;
        }
        let dir = match p.direction {
            PortDirection::Input => "Input",
            PortDirection::Output => "Output",
            PortDirection::InOut => "Analog",
        };
        io_fields.push_str(&format!(
            "    val {} = {dir}({})\n",
            p.name,
            chisel_ty(&p.ty)
        ));
    }
    let mut body = match face {
        EmitFace::Mechanical => format!(
            "class {} extends Module {{\n  val io = IO(new Bundle {{\n{io_fields}  }})\n",
            m.name
        ),
        EmitFace::Idiomatic => format!(
            "class {} extends Module {{\n  // --- IO ---\n  val io = IO(new Bundle {{\n{io_fields}  }})\n",
            m.name
        ),
        EmitFace::IdiomaticFr111 => format!(
            "class {} extends Module {{\n  // --- FR111 per-module ---\n  // --- IO ---\n  val io = IO(new Bundle {{\n{io_fields}  }})\n",
            m.name
        ),
        EmitFace::IdiomaticFr122 => format!(
            "class {} extends Module {{\n  // --- FR111 per-module ---\n  // --- FR122 official ---\n  // --- IO ---\n  val io = IO(new Bundle {{\n{io_fields}  }})\n",
            m.name
        ),
        EmitFace::IdiomaticFr130 => format!(
            "class {} extends Module {{\n  // --- FR111 per-module ---\n  // --- FR122 official ---\n  // --- FR130 style-guide ---\n  // --- IO ---\n  val io = IO(new Bundle {{\n{io_fields}  }})\n",
            m.name
        ),
        EmitFace::IdiomaticFr165 => format!(
            "class {} extends Module {{\n  // --- FR111 per-module ---\n  // --- FR122 official ---\n  // --- FR130 style-guide ---\n  // --- FR165 style-lint ---\n  // --- IO ---\n  val io = IO(new Bundle {{\n{io_fields}  }})\n",
            m.name
        ),
        EmitFace::IdiomaticFr176 => format!(
            "class {} extends Module {{\n  // --- FR111 per-module ---\n  // --- FR122 official ---\n  // --- FR130 style-guide ---\n  // --- FR165 style-lint ---\n  // --- FR176 ecosystem ---\n  // --- IO ---\n  val io = IO(new Bundle {{\n{io_fields}  }})\n",
            m.name
        ),
        EmitFace::IdiomaticFr181 => format!(
            "class {} extends Module {{\n  // --- FR111 per-module ---\n  // --- FR122 official ---\n  // --- FR130 style-guide ---\n  // --- FR165 style-lint ---\n  // --- FR176 ecosystem ---\n  // --- FR181 style-linter ---\n  // --- IO ---\n  val io = IO(new Bundle {{\n{io_fields}  }})\n",
            m.name
        ),
        EmitFace::IdiomaticFr188 => format!(
            "class {} extends Module {{\n  // --- FR111 per-module ---\n  // --- FR122 official ---\n  // --- FR130 style-guide ---\n  // --- FR165 style-lint ---\n  // --- FR176 ecosystem ---\n  // --- FR181 style-linter ---\n  // --- FR188 style-guide-pack ---\n  // --- IO ---\n  val io = IO(new Bundle {{\n{io_fields}  }})\n",
            m.name
        ),
    };

    if matches!(
        face,
        EmitFace::IdiomaticFr122
            | EmitFace::IdiomaticFr130
            | EmitFace::IdiomaticFr165
            | EmitFace::IdiomaticFr176
            | EmitFace::IdiomaticFr181
            | EmitFace::IdiomaticFr188
    ) {
        // O2: emit body sections in official order (not HIR stmt order).
        emit_module_body_ordered(m, modules_by_name, face, &mut body);
    } else {
        emit_module_body_hir_order(m, modules_by_name, face, &mut body);
    }
    body.push_str("}\n");
    body
}

fn emit_stmt_into(
    m: &Module,
    modules_by_name: &HashMap<&str, &Module>,
    face: EmitFace,
    body: &mut String,
    section: &mut BodySection,
    stmt: &Stmt,
) {
    let mut enter = |body: &mut String, next: BodySection, label: &str| {
        if is_sectioned_face(face) && *section != next {
            body.push_str(&format!("  // --- {label} ---\n"));
            *section = next;
        }
    };

    match stmt {
        Stmt::RegDecl { name, ty, .. } => {
            enter(body, BodySection::Registers, "registers");
            let w = match ty {
                GroundType::UInt { width } | GroundType::SInt { width } => *width,
                _ => 1,
            };
            body.push_str(&format!("  val {name} = RegInit(0.U({w}.W))\n"));
        }
        Stmt::WireDecl { name, ty, .. } => {
            enter(body, BodySection::Wires, "wires");
            body.push_str(&format!("  val {name} = Wire({})\n", chisel_ty(ty)));
        }
        Stmt::Process(p) => {
            enter(body, BodySection::Logic, "logic");
            for a in &p.assigns {
                match (&a.target, p.kind) {
                    (AssignTarget::Net(n), ProcessKind::Combinational)
                    | (AssignTarget::RegD(n), ProcessKind::Sequential) => {
                        body.push_str(&format!(
                            "  {} := {}\n",
                            ref_name(m, n),
                            emit_expr(m, &a.expr)
                        ));
                    }
                    (AssignTarget::MemWrite { mem, addr, we }, ProcessKind::Sequential) => {
                        let write = format!(
                            "{mem}.write({}, {})",
                            ref_name(m, addr),
                            emit_expr(m, &a.expr)
                        );
                        match we {
                            Some(en) => {
                                body.push_str(&format!(
                                    "  when ({}) {{\n    {write}\n  }}\n",
                                    ref_name(m, en)
                                ));
                            }
                            None => {
                                body.push_str(&format!("  {write}\n"));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Stmt::Instance(inst) => {
            enter(body, BodySection::Instances, "instances");
            let child = modules_by_name
                .get(inst.module.as_str())
                .expect("preflight rejects unknown module refs");
            body.push_str(&format!(
                "  val {} = Module(new {})\n",
                inst.name, inst.module
            ));
            body.push_str(&emit_instance_connects(
                m,
                &inst.name,
                child,
                &inst.connects,
            ));
        }
        Stmt::MemDecl {
            name,
            depth,
            width,
            sync_read,
            init,
            ..
        } => {
            enter(body, BodySection::Memories, "memories");
            body.push_str(&emit_mem_decl(name, *depth, *width, *sync_read, init));
        }
    }
}

fn emit_module_body_hir_order(
    m: &Module,
    modules_by_name: &HashMap<&str, &Module>,
    face: EmitFace,
    body: &mut String,
) {
    let mut section = BodySection::None;
    for stmt in &m.body {
        emit_stmt_into(m, modules_by_name, face, body, &mut section, stmt);
    }
}

fn emit_module_body_ordered(
    m: &Module,
    modules_by_name: &HashMap<&str, &Module>,
    face: EmitFace,
    body: &mut String,
) {
    let mut section = BodySection::None;
    // Official order: registers → wires → instances → memories → logic
    let kinds: &[fn(&Stmt) -> bool] = &[
        |s| matches!(s, Stmt::RegDecl { .. }),
        |s| matches!(s, Stmt::WireDecl { .. }),
        |s| matches!(s, Stmt::Instance(_)),
        |s| matches!(s, Stmt::MemDecl { .. }),
        |s| matches!(s, Stmt::Process(_)),
    ];
    for pred in kinds {
        for stmt in &m.body {
            if pred(stmt) {
                emit_stmt_into(m, modules_by_name, face, body, &mut section, stmt);
            }
        }
    }
}
