---
title: '20.2 架构 AD — FIRRTL→可编译 Chisel（FR28 条）'
type: 'chore'
created: '2026-08-21'
status: 'done'
baseline_commit: '666350e0c0447a9d2d8b7fdf299a724af5ed26fb'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-20-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/20-1-nfr14-风险记录-chisel-双向.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-chisel-bidirectional.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** 脊柱 AD-27 已存在但缺口未钉死（尤其历史 NFR9「不承诺可维护 Chisel」推翻关系、Open Q5 关闭条、`Parser.parse` 边界），实现可能仍按尽力失败或依赖已删 Scala Parser。

**Approach:** 核对并补齐 AD-27（FR28/FR46 验收条：可编译 + 端口/层次谓词；机械风格可；不恢复 Parser；记录 NFR9 推翻）；AGENTS 正向指针；ATDD 锁脊柱。不实现生成器。

## Boundaries & Constraints

**Always:** 验收=钉死 Chisel+firtool 下编译通过 + 公开端口名/宽/向与实例层次往返谓词；允许机械风格（Open Q5 已关闭）；不要求恢复 Chisel 5 前 Scala `Parser.parse` / `firrtl.Parser`；显式记录与历史 NFR9 的推翻；引用 FR28/FR46；品牌 Bitloom。

**Ask First:** 无。

**Never:** 实现 FIRRTL→Chisel 生成器（留给 20.3）；改选 FR46 选项 B/C 而不改风险记录；把 NFR10 调试再生写成产品互操作完成。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| AD-27 齐全 | Rule/Revised 含 FR28/FR46、NFR9 推翻、无 Parser 要求 | ATDD 绿 | N/A |
| 缺 NFR9 推翻 | 无 NFR9 / 推翻字样 | ATDD 红 | 补 Revised/Rule |
| 仍要求 Parser | Rule 要求恢复 Parser.parse | ATDD 红 | 改正为「不要求」 |

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/architecture/.../ARCHITECTURE-SPINE.md` L228–233 — AD-27 现有 Rule；缺 Revised / NFR9 推翻显式句
- `crates/bitloom/tests/ad5_rust_functional_sim_allowed.rs` / `ad20_bundle_vec_allowed.rs` — 脊柱 AD 切片 ATDD 样板
- `_agile-output/implementation-artifacts/nfr14-risk-chisel-bidirectional.md` — 20.1 门禁已存在（Given）
- `AGENTS.md` — Brand lock；尚无 AD-27 / FR28 指针
- `docs/fr28-chisel-best-effort.md` — 历史尽力文档（本故事只读；不改生成器）
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `20-2-架构-ad-firrtl-可编译-chisel-fr28-条`

## Tasks & Acceptance

**Execution:**
- [x] `ARCHITECTURE-SPINE.md` AD-27 -- 补齐验收条引用 FR28/FR46、Open Q5、NFR9 推翻、不要求 `Parser.parse`；加 Revised -- AC
- [x] `AGENTS.md` -- Brand lock 正向指针 AD-27 / FR28（可编译 Chisel；非尽力）-- 防漂移
- [x] `crates/bitloom/tests/ad27_compilable_chisel.rs` -- ATDD 门禁 -- 门禁
- [x] `sprint-status.yaml` -- 本故事 in-progress→review→done -- 追踪
- [x] `20-2-code-review.md` -- 对抗性审查 -- 流水线

**Acceptance Criteria:**
- Given 20.1 风险记录已存在，when 修订/核对 AD-27，then 写明 FrozenHir/`.fir` → 可编译 Chisel Scala；验收=钉死 Chisel+firtool 编译通过 + 端口/层次往返谓词；允许机械风格（Open Q5 已关闭）
- Given AD-27，when 读边界，then 不要求恢复 Chisel 5 前 Scala `Parser.parse` API
- Given AD-27，when 对照历史合同，then 记录与 NFR9「不承诺可维护 Chisel」的推翻关系
- Given 本故事，when 完成，then **未**实现代码生成器
- Given ATDD，when `cargo test -p bitloom --test ad27_compilable_chisel`，then 通过

## Spec Change Log

## Design Notes

AD-27 主体已 ADOPTED；本故事以核对 + 缺口补齐 + ATDD 为主。生成器实现属 Story 20.3。`docs/fr28-chisel-best-effort.md` 历史措辞可另故事改写，不在本范围强制。

## Verification

**Commands:**
- `cargo test -p bitloom --test ad27_compilable_chisel` -- expected: pass
- `cargo fmt --all && just test` -- expected: pass

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- AD-27 补 Revised + NFR9 推翻 + `Parser.parse` + Open Q5；引用 FR28/FR46
- AGENTS Brand lock 正向指针 AD-27 / FR28+FR46
- ATDD `ad27_compilable_chisel`；审查清理死变量
- testarch-automate：脊柱门禁已落地（非 N/A）— 见 [20-2-code-review.md](20-2-code-review.md)
- **未**实现 FR28 生成器

### File List

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`
- `AGENTS.md`
- `crates/bitloom/tests/ad27_compilable_chisel.rs`
- `_agile-output/implementation-artifacts/20-2-架构-ad-firrtl-可编译-chisel-fr28-条.md`
- `_agile-output/implementation-artifacts/20-2-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-08-21: 补齐 AD-27 FR28 可编译条 + NFR9 推翻 + ATDD（Story 20.2）

## Suggested Review Order

**脊柱 AD-27**

- Rule/Revised：可编译验收、Parser、NFR9 推翻
  [`ARCHITECTURE-SPINE.md`](../planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md)

**Brand lock**

- AGENTS 正向指针
  [`AGENTS.md`](../../AGENTS.md)

**ATDD**

- 脊柱切片断言
  [`ad27_compilable_chisel.rs:22`](../../crates/bitloom/tests/ad27_compilable_chisel.rs#L22)
