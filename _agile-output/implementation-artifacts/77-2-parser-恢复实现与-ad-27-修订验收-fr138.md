---
title: '77.2 Parser 恢复实现与 AD-27 修订验收（FR138）'
type: 'feature'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '1564e1b'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic77-restore-parser-ad27.md'
  - '{project-root}/_agile-output/implementation-artifacts/77-1-epic-77-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/76-2-外部-circt-编译-仿真门禁实现与验收-fr137.md'
  - '{project-root}/_agile-output/implementation-artifacts/70-2-ad-27-修订-若需-style-guide-parser-路径-fr130.md'
  - '{project-root}/docs/fr130-style-guide.md'
  - '{project-root}/docs/fr137-external-circt-gate.md'
  - '{project-root}/scripts/circt-external-check.sh'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase16-nfr55-final-closeout.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR138 要求将恢复废弃 Scala `Parser.parse` / `firrtl.Parser`（或文档钉死的产品等价 API）作为**产品关闭条件**，并**必须**再修订 AD-27（NFR58）。上游已删 Parser（chipsalliance/chisel#4899）；现行 AD-27 仍禁止静默恢复。不得以 FR97 / FR111 / FR122 O1–O4 / FR130 Style Guide（Parser 未恢复）alone、docs-only、或未修订 AD-27 关闭本 FR。

**Approach:** 按 NFR14 **P1–P4** 落地：
- **P1** 钉死产品等价 API **`BitloomFirrtlParser.parse`**（显式 ≡ 历史 `firrtl.Parser.parse` / `Parser.parse`）+ 文档化工作流（代表性 `.fir` → Parser 等价 → 可检查 parse 结果）+ Chisel **7.14.0** / firtool **1.155.0** 配对；CLI/`just`/夹具可检查
- **P2** **再修订 AD-27** 明确允许本产品关闭条件；Correct Course 痕迹复用 Phase 16 `correctCoursePhase16Approved: 2026-09-11`
- **P3** 缺 Parser / API 不可用 / 版本或配对不符 → **非零** + **可读**失败
- **P4** ATDD 覆盖 P1–P3；≠ FR97/111/122/130 alone；≠ docs-only

实现层：CLI/interop/fixture（`scripts/parser-restore-check.sh` + `just parser-restore-check` + Scala façade 夹具）；**不**把 Scala 运行时塞进设计 crate。Epic 77 关闭勾选 → **77.3**。

## Boundaries & Constraints

**Always:** P1–P4；AD-27 再修订（NFR58）先于/同步于宣称完成；Correct Course Phase 16 痕迹；Chisel 7.14.0 ↔ firtool-1.155.0；品牌 Bitloom；设计 crate → `bitloom-prelude` only；NFR56 回归 FR97/111/122/130；FR130 Style Guide 关闭仍有效（S3「Parser 未恢复」对 FR130 完成面仍成立；FR138 产品路径 supersede「不得以未恢复宣称 Parser 产品关闭」）。

**Ask First:** 若撤回 FR138（永久禁止 Parser）或更换选定 API/工作流/版本配对 — 须改 NFR14 + Correct Course / AD-27。

**Never:** 未修订 AD-27 标 77.2 done / 宣称 FR138 关闭；以 FR97/111/122 O1–O4 / FR130 Style Guide alone 关闭 FR138；docs-only；把 Scala 运行时依赖塞进设计 crate；勾选 Epic 77 关闭（→ 77.3）；改写 FR97/111/122/130「已关闭」为失败；信任 PATH 随机 firtool/scala；CIRCT HEAD / 未配对升钉。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| P1 API | docs + AD-27 + shim | `BitloomFirrtlParser.parse` ≡ `Parser.parse` / `firrtl.Parser` 显式对照 | 缺名 → ATDD 红 |
| P1 workflow | `.fir` → parse | `just parser-restore-check` / script 可复现 | 失败非零 |
| P1 pairing | Chisel 7.14.0 + firtool-1.155.0 | docs/script/AD-27 钉死 | 漂移 → 红 |
| P2 AD-27 | spine revise | 允许 FR138 Parser 产品关闭；Correct Course 痕迹 | 未修订 → 不得 done |
| P3 missing | `BITLOOM_PARSER_FORCE_MISSING=1` | exit ≠0 + 可读 | 禁止 exit 0 |
| P3 mismatch | stub firtool ≠ 1.155.0 | exit ≠0 + 可读 version | 禁止 skip |
| Parse OK | 钉死 `.fir` + 合格 firtool | `-parse-only` 成功 / stamp | firtool 失败 → 红 |
| alone 禁令 | 仅 FR130/122/111/97 | ≠ FR138 关闭 | ATDD 边界 |
| 回归 | FR97/111/122/130 测试 | 全绿（NFR56） | 回滚或修复 |

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` — **UPDATE** AD-27：允许 FR138 Parser / 产品等价 API 作为关闭条件（NFR58）；保留 FR130 S3「未恢复」历史完成面措辞
- `docs/fr138-parser-restore.md` — **NEW** 产品合同（API 对照、工作流、配对、失败语义、≠ FR130 alone）
- `docs/fr130-style-guide.md` — **UPDATE** 交叉链：FR138 产品路径 supersede「以未恢复宣称 Parser 关闭」；Style Guide 关闭仍有效
- `scripts/parser-restore-check.sh` — **NEW** FR138 门禁（AD-9 firtool；`-parse-only`；Chisel 7.14.0 配对声明；FORCE_MISSING / 版本不符非零）
- `Justfile` — **UPDATE** `parser-restore-check` 目标
- `crates/rhdl-firrtl/fixtures/fr138_parser_restore.fir` — **NEW** 代表 `.fir`（firtool-1.155.0 `-parse-only`）
- `crates/rhdl-firrtl/testdata/fr138_bitloom_firrtl_parser.scala` — **NEW** Scala façade：`BitloomFirrtlParser.parse` + 文档对照历史 `firrtl.Parser.parse`
- `crates/bitloom/tests/fr138_parser_restore_ad27.rs` — ATDD P1–P4
- `_agile-output/implementation-artifacts/nfr14-risk-epic77-restore-parser-ad27.md` — **不**勾选 Epic 77 关闭（→ 77.3）

## Story

As a Chisel 互操作用户,
I want 恢复 `Parser.parse`（或文档等价）作为产品路径,
So that Parser 依赖成为可检查的关闭条件而非默认禁止项。

## Acceptance Criteria

1. Given Story 77.1, when 先/同步修订 ARCHITECTURE-SPINE **AD-27**（NFR58），并交付 Parser 产品路径 + ATDD/夹具, then 验收覆盖 P1–P4；AD-27 明确允许本产品关闭条件；FR97/FR111/FR122/FR130 回归不破（NFR56）
2. And 不得在未修订 AD-27 的情况下标本故事 done；公开品牌 Bitloom
3. And ≠ FR97/FR111/FR122 O1–O4 alone；≠ FR130 Style Guide alone；≠ docs-only；不关闭 Epic 77（→ 77.3）
4. And Correct Course 痕迹存在（可复用 Phase 16 `correctCoursePhase16Approved: 2026-09-11`）
5. And 缺 Parser / 版本不符 → 非零可读失败；产品等价 API 名称在 docs/fr138-* 与 AD-27 显式对照

## Tasks / Subtasks

- [x] T1: 再修订 AD-27（允许 FR138 / 钉死 `BitloomFirrtlParser.parse` ≡ `Parser.parse`；Correct Course 指针）（AC: 1–2, 4–5）
- [x] T2: `scripts/parser-restore-check.sh` + `just parser-restore-check` + `.fir` 夹具 + Scala façade（AC: 1, 3, 5）
- [x] T3: `docs/fr138-parser-restore.md` + fr130 交叉链（AC: 1, 3–5）
- [x] T4: ATDD `fr138_parser_restore_ad27.rs`（P1–P4）（AC: 1–5）
- [x] T5: `cargo clean && cargo fmt --all && just test`；code-review Approve；automation-summary；sprint 77-2 done；**不**启动 77.3

## Dev Notes

### Architecture / AD

- **NFR58 CRITICAL：** 未修订 AD-27 ⇒ 不得标 77.2 done / 宣称 FR138 关闭。
- **上游事实：** Scala `firrtl.Parser.parse` 已随 CIRCT 迁移删除（chisel#4899）。产品路径交付**文档钉死的等价 API** `BitloomFirrtlParser.parse`，实现 = AD-9 `firtool-1.155.0 -parse-only`（CIRCT 官方推荐替代）。
- **≠ FR130：** S1–S4 Style Guide（含 S3「Parser 未恢复」）关闭仍有效；alone ≠ FR138。FR138 **翻转**的是「Parser 作为产品关闭条件」。
- **对照 FR137：** 复用 AD-9 ensure / FORCE_MISSING / 版本不符非零体例；目标 = parse（`-parse-only`），≠ 编译门禁 alone。
- **NFR56：** 不得破坏 FR97/111/122/130（含 `emit_chisel_style_guide_fr130` 仍含「Parser.parse not required / not restored」）。
- **NFR59：** 完整社区 linter 全家桶、任意 Chisel 版本 Parser 回迁、macos/windows Parser 资产、firtool-1.156.0+ 仍未列入。

### Locked shape (P1–P4) — copy from NFR14

| ID | 钉死 |
| --- | --- |
| **P1** | Parser（或文档等价）API + 工作流 + Chisel **7.14.0** / firtool **1.155.0** |
| **P2** | 再修订 AD-27 + Correct Course（可复用 Phase 16） |
| **P3** | 缺 Parser / 版本不符 → **非零** + **可读** |
| **P4** | ATDD 覆盖 P1–P3；≠ FR97/111/122/130 alone；≠ docs-only |

### Recommended script behavior

1. Banner: FR138 Parser restore gate (Bitloom; ≠ FR130/122/111/97 alone)
2. `BITLOOM_PARSER_FORCE_MISSING=1` → stderr readable + exit 1
3. Resolve firtool via AD-9 (`RHDL_FIRTOOL_PATH` or `cargo run -p bitloom -- firtool ensure`) — never bare PATH as success
4. `firtool --version` must contain `1.155.0`；docs/script assert Chisel **7.14.0** pairing
5. `$FIRTOOL -parse-only` on `crates/rhdl-firrtl/fixtures/fr138_parser_restore.fir` → stamp under `target/parser-restore-check/`
6. Success log naming `BitloomFirrtlParser.parse` product-equivalent; any failure → non-zero

### Project Structure Notes

- 不改设计 crate 依赖图；Parser 在 CLI/scripts/testdata 层。
- 不触 77.3 收口勾选；不把 FR130 S3 emit 注释改成「已恢复」冒充 Style Guide 关闭面变更。

### References

- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic77-restore-parser-ad27.md`]
- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 77 / Story 77.2]
- [Source: `ARCHITECTURE-SPINE.md` — AD-27 / AD-9]
- [Source: `scripts/circt-external-check.sh` — AD-9 fail-closed mirror]
- [Source: `docs/fr130-style-guide.md`]
- [Source: `AGENTS.md` — Bitloom / bitloom-prelude]

## Testing

- `cargo test -p bitloom --test fr138_parser_restore_ad27`
- 本地：`BITLOOM_PARSER_FORCE_MISSING=1 just parser-restore-check` → 非零
- 回归：`fr130_style_guide`；`fr122_*`；`fr111_*`；`fr97_*`
- `cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Agent Model Used

Composer (Auto)

### Debug Log References

### Completion Notes List

- P1–P4：`BitloomFirrtlParser.parse` ≡ `Parser.parse`；`just parser-restore-check` / firtool-1.155.0 `-parse-only`；Chisel 7.14.0 配对；缺工具/版本不符非零可读
- NFR58：AD-27 2026-09-11 再修订；Correct Course Phase 16 痕迹复用
- ATDD 7 passed；code-review **Approve**；**未**启动 77.3 / 未勾选 Epic 77 关闭
- NFR56：FR97/111/122/130 回归绿；FR130 Style Guide（Parser 未恢复）完成面仍有效

### File List

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`
- `AGENTS.md`
- `docs/fr138-parser-restore.md`
- `docs/fr130-style-guide.md`
- `scripts/parser-restore-check.sh`
- `Justfile`
- `crates/rhdl-firrtl/fixtures/fr138_parser_restore.fir`
- `crates/rhdl-firrtl/testdata/fr138_bitloom_firrtl_parser.scala`
- `crates/bitloom/tests/fr138_parser_restore_ad27.rs`
- `_agile-output/implementation-artifacts/nfr14-risk-epic77-restore-parser-ad27.md`
- `_agile-output/implementation-artifacts/77-2-parser-恢复实现与-ad-27-修订验收-fr138.md`
- `_agile-output/implementation-artifacts/atdd-checklist-77-2-fr138-parser-restore-ad27.md`
- `_agile-output/implementation-artifacts/77-2-code-review.md`
- `_agile-output/implementation-artifacts/77-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`