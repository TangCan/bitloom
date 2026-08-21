---
title: '20.5 import CLI + 混合夹具（FR40 / FR46 腿 3）'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: '1d9f2c6279b6d18c6d906148482a2a16d343ec40'
review_loop_iteration: 1
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-20-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/20-4-反向导入-chisel-fir-bitloom-fr46-腿-2.md'
  - '{project-root}/docs/fr46-chisel-import.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR46 库能力已有，但缺产品入口 `import` CLI 与「一侧 Bitloom、一侧 Chisel/.fir」进入同一后端的文档化混合夹具。

**Approach:** 交付 `cargo bitloom import`（`--help` + smoke），混合夹具证明 Bitloom elaborate 与外部 `.fir` import 走同一 emit 路径；README/文档指向该流程；Epic 20 故事全部 done 后标 epic-20 done。

## Boundaries & Constraints

**Always:** Bitloom 品牌；`--help` + smoke；混合夹具进同一 emit/后端；设计 crate 仅 `bitloom-prelude`；不静默违反 NFR14。

**Ask First:** 若 CLI 必须调用 JVM/Chisel 才能 smoke。

**Never:** 解析 Scala；冒充已交付 visualize/wave；恢复 Parser.parse。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Help | `import --help` | 用法含 input/out-dir | N/A |
| Smoke | 有效 `.fir` | 写出 `.v`（同 build 后端） | 坏 fir → 非零 + 诊断 |
| Mixed fixture | Bitloom 模块 + `.fir` | 两者均可 `emit` | N/A |

</frozen-after-approval>

## Code Map

- `crates/bitloom/src/main.rs` — `Import` 子命令
- `crates/bitloom/Cargo.toml` — 依赖 `rhdl-firrtl` / `bitloom-vlog`
- `crates/bitloom/tests/import_cli.rs` — `--help` + smoke ATDD
- `examples/chisel_mixed/` — 混合夹具（Bitloom + `.fir` → 同一 emit）
- `docs/fr40-cli-verbs.md` / `docs/fr46-chisel-import.md` / `README.md`
- Sprint：`20-5-…` + `epic-20: done`

## Tasks & Acceptance

**Execution:**
- [x] `crates/bitloom/src/main.rs` + `Cargo.toml` -- `import` 子命令读 `.fir`→FrozenHir→emit `.v` -- FR40
- [x] `crates/bitloom/tests/import_cli.rs` -- help + smoke -- ATDD
- [x] `examples/chisel_mixed/` -- Bitloom + 外部 `.fir` 同一后端 -- FR46 腿 3
- [x] `docs/fr40-cli-verbs.md` + `fr46` + `README.md` -- 文档入口 -- UJ-4
- [x] `sprint-status.yaml` -- 20.5 done + epic-20 done -- 追踪
- [x] `20-5-code-review.md` -- 对抗性审查 -- 流水线

**Acceptance Criteria:**
- Given 20.3–20.4 库能力，when `cargo bitloom import`，then `--help` + smoke 绿（FR40）
- Given 混合夹具，when Bitloom 与 Chisel/`.fir` 产物，then 进入同一 emit/后端路径（FR46）
- Given README/教程，when 查双向流程，then 至少一处指向（UJ-4）
- Given Epic 20 全部故事 done，when 收尾，then `epic-20: done`

## Spec Change Log

- 2026-08-21: 审查 — defer CLI 不写 Scala；defer 夹具 monorepo 路径

## Design Notes

`import` 只吃 `.fir`（Chisel→firtool 产出）；写出与 `build` 相同的 Yosys-friendly `.v`。混合夹具设计侧只依赖 `bitloom-prelude`。

## Verification

**Commands:**
- `cargo test -p bitloom --test import_cli` -- expected: pass — **PASS**
- `cargo test -p chisel_mixed` -- expected: pass — **PASS**
- `cargo fmt --all && just test` -- expected: pass

## Review Triage Log

### 2026-08-21 — Formal review pass
- defer: 2（CLI 无 Scala；include_str 路径）
- addressed: 见 [20-5-code-review.md](20-5-code-review.md)

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- `cargo bitloom import --input … --out-dir … [--also-fir] [--also-chisel]`
- `examples/chisel_mixed` 混合夹具
- FR40/FR46/README 文档；epic-20 done

### File List

- `crates/bitloom/src/main.rs`
- `crates/bitloom/Cargo.toml`
- `crates/bitloom/tests/import_cli.rs`
- `examples/chisel_mixed/`
- `Cargo.toml`
- `docs/fr40-cli-verbs.md`
- `docs/fr46-chisel-import.md`
- `README.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/20-5-import-cli-混合夹具-fr40-fr46-腿-3.md`
- `_agile-output/implementation-artifacts/20-5-code-review.md`
- `_agile-output/implementation-artifacts/deferred-work.md`

## Change Log

- 2026-08-21: import CLI + 混合夹具；Epic 20 收口（Story 20.5）
