---
title: '19.2 修订 AD-5（允许生成 Rust 功能模拟器）'
type: 'chore'
created: '2026-08-21'
status: 'done'
baseline_revision: '1244cfe696cf9f3bd81356a3216fd44eb9486834'
review_loop_iteration: 0
followup_review_recommended: false
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-19-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/19-1-nfr14-风险记录模板.md'
warnings: []
deferred:
  - summary: >-
      历史 docs/fr29、早期 epics FR14/FR29 等仍写「无 HIR→TLM」而未指向 AD-5/FR47 解锁。
    evidence: |-
      Blind hunter / intent-alignment reading (3)；code review 已标 out of scope；现行合同以脊柱 AD-5 + PRD 推翻表为准。
    location: >-
      docs/fr29-*.md; epics.md early FR14
    severity: medium
  - summary: >-
      bitloom-sim `no_hir_to_tlm_api` 注释仍仅提 FR14/FR29，未对照 AD-5 Rust 生成允许 / SystemC 非合同拆分。
    evidence: |-
      Blind hunter；注释对 SystemC TLM 入口仍正确，属叙事漂移风险。
    location: >-
      crates/bitloom-sim (no_hir_to_tlm_api)
    severity: low
  - summary: >-
      规划 epics.md Story 19.2 Given 仍用「仍写禁止」现在时，修订后未改写史态。
    evidence: |-
      Blind hunter；规划归档措辞，不影响脊柱合同。
    location: >-
      _agile-output/planning-artifacts/epics.md Story 19.2
    severity: low
  - summary: >-
      AGENTS 托管 bmad:context 块未提及 AD-5/FR47（Brand lock 已有指针）。
    evidence: |-
      Blind hunter；AC 仅要求若仍复述旧禁则同步；托管块无旧禁。
    severity: medium
  - summary: >-
      更广「项目上下文」语料未全部扫进 ATDD（仅 AGENTS.md）。
    evidence: |-
      Intent-alignment surface divergence reading (3)；意图「若仍禁则同步」已由 Brand lock + 无旧禁断言覆盖。
    severity: medium
---

<intent-contract>

## Intent

**Problem:** 历史 AD-5 / FR14 读法「禁止从 HIR 降低 TLM / 禁止 HIR→功能模拟器生成」与 PRD ①C / FR47 冲突；虽脊柱已有 2026-08-21 修订段，仍须核对引用完整、项目上下文无旧禁令，并用 ATDD 锁住合同。

**Approach:** 核对并补齐 AD-5（允许生成 Rust 功能模拟器 crate；不承诺 SystemC TLM-2.0；周期精确仅 FrozenHir `tick`；修订说明引用 PRD FR47 与推翻表）；同步 AGENTS 若仍复述旧禁；落地文档门禁测试。不实现 FR47 生成器。

## Boundaries & Constraints

**Always:** 周期精确仿真只从 `FrozenHir` `tick`；功能模拟器形态 = 手写或**生成的 Rust crate**；SystemC TLM-2.0 非合同；修订可追溯到 PRD FR47 / 推翻表；品牌 Bitloom。

**Block If:** 需要在本故事实现 FR47 CLI/API 生成器；或要把 SystemC TLM-2.0 升为强制合同。

**Never:** 实现 FR47 生成器；把「禁止 HIR→TLM」重新写成阻断 FR47 的依据；另立 HIR 或改仿真运行时行为。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| AD-5 已修订且引用齐全 | 脊柱 AD-5 段 | ATDD 绿；故事只补缺口 | 无 |
| Revised 缺 FR47/推翻表 | 仅写 ①C / Epic | 补全 Revised 引用后绿 | ATDD 红直至补齐 |
| AGENTS 仍写旧禁 | 含「禁止 HIR→功能模拟」 | 更正或指向脊柱 | ATDD 断言无旧禁 |
| 误把 TLM 禁令当 FR47 禁 | Rule 写禁止一切功能模拟生成 | 不得保留为现行 Rule | ATDD 失败 |

</intent-contract>

## Code Map

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` L64–69 — AD-5 Rule/Revised；Capability map L310、Deferred L324 已对齐「手写或生成 Rust」
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` §0 推翻表行「禁止从 HIR 生成 TLM / 功能模拟器」→ FR47；FR47 本体
- `AGENTS.md` — Brand lock 正向指针（AD-5 / FR47）；无旧禁
- `crates/bitloom/tests/nfr14_risk_record_template.rs` — 文档门禁 ATDD 样板（workspace_root + 读文件断言）
- `crates/bitloom/tests/ad2_design_dep_bitloom_prelude.rs` — 脊柱 AD 段切片断言样板
- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/team-walkthrough.html` — 已写生成 Rust 功能模拟器（只读核对）
- `_agile-output/implementation-artifacts/sprint-status.yaml` — key `19-2-修订-ad-5-允许生成-rust-功能模拟器`

## Tasks & Acceptance

**Execution:**
- [x] `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` -- 核对 AD-5；Revised 补齐 **PRD FR47** 与 **推翻表**；Rule 已允许生成 Rust、不承诺 SystemC TLM-2.0、周期精确仅 FrozenHir -- AC 对齐 ①C
- [x] `AGENTS.md` -- 无旧禁；Brand lock 增加正向指针（AD-5 / FR47 允许生成 Rust 功能模拟器；不承诺 SystemC TLM-2.0）-- 防上下文漂移
- [x] `crates/bitloom/tests/ad5_rust_functional_sim_allowed.rs` -- 新增 ATDD 门禁 -- 门禁
- [x] `_agile-output/implementation-artifacts/sprint-status.yaml` -- `19-2-…`：in-progress → review → done -- 冲刺追踪
- [x] `_agile-output/implementation-artifacts/19-2-code-review.md` -- 对抗性审查笔记 -- 流水线要求

**Acceptance Criteria:**
- Given 脊柱含 AD-5，when 阅读 Rule/Revised，then 允许工具链生成 Rust 功能模拟器 crate；显式不强制/不承诺 SystemC TLM-2.0；周期精确仅从 FrozenHir `tick`
- Given 修订说明，when 对照 PRD，then 引用 **PRD FR47** 与 **推翻表**；历史「禁止 HIR→TLM」不再作为阻断 FR47 的依据
- Given `AGENTS.md` / 项目上下文，when 扫描旧禁令，then 已更正或指向脊柱（无「禁止 HIR→功能模拟器生成」阻断读法）
- Given 本故事范围，when 完成，then **未**实现 FR47 生成器本身
- Given ATDD 文件存在，when `cargo test -p bitloom --test ad5_rust_functional_sim_allowed`，then 通过

## Spec Change Log

- 2026-08-21: Revised 补 FR47 + 推翻表引用；AGENTS Brand lock 正向指针；ATDD `ad5_rust_functional_sim_allowed`；审查见 [19-2-code-review.md](19-2-code-review.md)

## Review Triage Log

- 对抗性审查：accept；ATDD Rule 旧禁短语断言收紧（避免误伤「不要求降低 SystemC TLM-2.0」）— 见 [19-2-code-review.md](19-2-code-review.md)

### 2026-08-21 — Review pass
- intent_gap: 0
- bad_spec: 0
- patch: 2: (high 0, medium 0, low 2)
- defer: 5: (high 0, medium 3, low 2)
- reject: 8
- addressed_findings:
  - `[low]` `[patch]` ATDD 对 AGENTS 的 TLM 断言收紧为须含 not contracted / 不承诺 等语义，而非仅出现 TLM 字样
  - `[low]` `[patch]` File List 补记 `epic-19-context.md`

## Design Notes

AD-5 主体已在 2026-08-21 修订；本故事以**核对 + 引用补齐 + ATDD 锁合同**为主，避免重写整段。生成器实现属 Epic 21。

## Verification

**Commands:**
- `cargo test -p bitloom --test ad5_rust_functional_sim_allowed` -- expected: PASS — **PASS (2026-08-21)**
- `cargo fmt --all && just test` -- expected: 全绿 — **PASS (2026-08-21)**

## Auto Run Result

- **Summary:** AD-5 Revised 补齐 PRD FR47 + §0 推翻表引用，并写明旧「禁止 HIR→TLM」不阻断 FR47；AGENTS Brand lock 正向指针；ATDD 锁合同。未实现 FR47 生成器。
- **Files:** ARCHITECTURE-SPINE AD-5；AGENTS.md；`ad5_rust_functional_sim_allowed.rs`；sprint-status；story + code-review；epic-19-context。
- **Review:** patches 2 low（ATDD TLM 语义收紧、File List）；deferred 5；reject 8；intent_gap/bad_spec 0。
- **Follow-up review:** false（patched: high 0, medium 0, low 2；score 2 < 5）。
- **Verification:** targeted ATDD PASS；`cargo fmt --all && just test` PASS。
- **Risks:** 历史 fr29/epics 措辞与 bitloom-sim 注释叙事仍可能误导；合同以脊柱 AD-5 为准。

## Dev Agent Record

### Completion Notes

- AD-5 Rule 已齐全；仅补 Revised：显式 **PRD FR47** + §0 **推翻表**，并写明旧禁不阻断 FR47
- AGENTS Brand lock 增加 AD-5/FR47 正向指针（无旧禁可删）
- ATDD 锁住允许生成 Rust、FR47、非 SystemC 合同、FrozenHir tick、Revised 引用、AGENTS 正向指针
- **未**实现 FR47 生成器

### File List

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`
- `AGENTS.md`
- `crates/bitloom/tests/ad5_rust_functional_sim_allowed.rs`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/19-2-code-review.md`
- `_agile-output/implementation-artifacts/19-2-修订-ad-5-允许生成-rust-功能模拟器.md`
- `_agile-output/implementation-artifacts/epic-19-context.md`
