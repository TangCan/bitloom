---
title: '76.2 外部 CIRCT 编译/仿真门禁实现与验收（FR137）'
type: 'feature'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '9031e5c'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic76-external-circt-compile-sim-gate.md'
  - '{project-root}/_agile-output/implementation-artifacts/76-1-epic-76-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/67-2-默认-ci-真-sby-门禁实现与验收-fr127.md'
  - '{project-root}/docs/fr127-forced-sby-ci.md'
  - '{project-root}/docs/fr129-circt-handshake.md'
  - '{project-root}/crates/bitloom/src/firtool.rs'
  - '{project-root}/scripts/firtool-smoke.sh'
  - '{project-root}/scripts/formal-sby-check.sh'
  - '{project-root}/.github/workflows/ci.yml'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR137 要求超出 FR129 C1–C4（树内 Handshake 方言标记）的**完整外部 CIRCT 编译与/或仿真门禁**；当前仅有可选 `firtool-smoke`（缺工具静默 skip）与 FR129 树内 API，不得以 FR95/96 / FR110 / FR121 / FR129 alone、docs-only 或 `continue-on-error` 关闭本 FR。

**Approach:** 按 NFR14 **E1–E4** 落地 **编译门禁 MVP**（仿真加深未选 → 文档写明；仍满足 E1–E4）：
- **E1** 钉死 **firtool-1.155.0** + tag/`firrtl-bin-linux-x64`（或 AD-9 `firtool ensure` 缓存）；≠ PATH 随机；≠ CIRCT HEAD
- **E2** CI **required** job `circt-external` **与** 文档钉死路径 `just circt-external-check`（无 `continue-on-error`）
- **E3** 缺工具 / 版本不符 → 非零 + 可读失败；`BITLOOM_CIRCT_FORCE_MISSING=1` 可强制；禁止 silent skip
- **E4** ATDD 覆盖 E1–E3；≠ FR129/121/110/95/96 alone；≠ docs-only

**NFR58：** 同步脊柱指针 / ops 文档 / CI 合同后再标本故事完成。Epic 76 关闭勾选 → **76.3**。

## Boundaries & Constraints

**Always:** E1–E4；firtool-1.155.0 / AD-9；编译门禁 MVP；品牌 Bitloom；设计 crate → `bitloom-prelude` only；NFR56 回归 FR95/110/121/129；NFR58 运维同步。

**Ask First:** 若强制改选定版本（非 1.155.0）或强制本 epic 必须含仿真门禁 — 须修订 NFR14。

**Never:** 以 FR95/96 / FR110 / FR121 / FR129 C1–C4 alone 关闭 FR137；docs-only；`continue-on-error`；信任 PATH 随机 firtool；勾选 Epic 76 关闭（→ 76.3）；把 CIRCT 运行时依赖塞进设计 crate；改写 FR95/110/121/129 已关闭为失败；CIRCT HEAD。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| E1 pin | docs/script/CI | firtool-1.155.0 + tag/`firrtl-bin-linux-x64` / AD-9 ensure | 漂移 → ATDD 红 |
| E2 CI | `circt-external` job | ensure + `circt-external-check`；无 continue-on-error | job 红 |
| E2 local | `just circt-external-check` | 同脚本路径 | 非零失败 |
| E3 missing | `BITLOOM_CIRCT_FORCE_MISSING=1` 或无 firtool | exit ≠0 + 可读 error | 禁止 exit 0 |
| E3 mismatch | stub firtool 非 1.155.0 | exit ≠0 + 可读 version 不符 | 禁止 skip |
| Compile OK | 钉死 `.fir` + 合格 firtool | 写出 `.v` / 成功日志 | firtool 失败 → 红 |
| alone 禁令 | 仅 FR129/121/110/95/96 | ≠ FR137 关闭 | ATDD 边界 |
| 回归 | FR95/110/121/129 测试 | 全绿（NFR56） | 回滚或修复 |

</frozen-after-approval>

## Code Map

- `scripts/circt-external-check.sh` — **NEW** FR137 外部 CIRCT **编译**门禁（AD-9 ensure / `RHDL_FIRTOOL_PATH`；版本检查；编译代表 `.fir`；缺工具非零）
- `Justfile` — **UPDATE** `circt-external-check` 目标
- `.github/workflows/ci.yml` — **UPDATE** required job `circt-external`（install/ensure firtool + run check；无 continue-on-error）
- `docs/fr137-external-circt-gate.md` — **NEW** 最小产品/CI 契约（选定 = 编译门禁；仿真未选；收口勾选仍 → 76.3）
- `docs/fr129-circt-handshake.md` — **UPDATE** 交叉链 FR137 外部门禁（≠ 树内 C1–C4 alone）
- `README.md` — **UPDATE** Phase 16 / 合同表指针 FR137 门禁路径
- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` — **UPDATE** AD-9 / Phase 16 指针到 FR137 CI 门禁（NFR58）
- `crates/bitloom/tests/fr137_external_circt_compile_sim_gate.rs` — ATDD E1–E4
- `crates/rhdl-firrtl/fixtures/fr137_external_circt_gate.fir` — **NEW** FR137 编译门禁代表 `.fir`（firtool-1.155.0 可编译；`public` 顶层）
- `crates/rhdl-firrtl/fixtures/external_hierarchy.fir` — **只读** 既有层次夹具（非本门禁默认输入）
- `_agile-output/implementation-artifacts/nfr14-risk-epic76-external-circt-compile-sim-gate.md` — **不**勾选 Epic 76 关闭（→ 76.3）

## Story

As a 工具链用户,
I want 风险记录钉死的外部 CIRCT 编译/仿真门禁可用,
So that Handshake/HLS 路径具备可复现外部 CIRCT 关闭条件。

## Acceptance Criteria

1. Given Story 76.1, when 交付门禁路径 + ATDD/夹具并同步 CI/运维文档（NFR58）, then 验收覆盖 E1–E4；FR95/FR110/FR121/FR129 回归不破（NFR56）
2. And 缺工具须非零可读失败；公开品牌 Bitloom
3. And ≠ FR129 C1–C4 alone；≠ FR121/110/95/96 alone；≠ docs-only；不关闭 Epic 76（→ 76.3）
4. And 本 epic 选定 = **编译门禁 MVP**（仿真未选须在 docs 写明）

## Tasks / Subtasks

- [x] T1: `scripts/circt-external-check.sh` + `just circt-external-check`（AD-9 ensure、版本钉死、编译 `.fir`、FORCE_MISSING / 版本不符非零）（AC: 1–2, 4）
- [x] T2: CI required job `circt-external`（无 continue-on-error）+ docs/spine/README NFR58 同步（AC: 1, 3–4）
- [x] T3: ATDD `fr137_external_circt_compile_sim_gate.rs`（E1–E4）（AC: 1–3）
- [x] T4: `cargo clean && cargo fmt --all && just test`；code-review Approve；automation-summary；sprint 76-2 done；**不**启动 76.3

## Dev Notes

### Architecture / AD

- **AD-9：** 仅 CLI 下载/缓存/调用 firtool-1.155.0；`RHDL_FIRTOOL_PATH` 覆盖；禁止默认信任 PATH。
- **≠ FR129：** C1–C4 = 树内方言标记；本 FR = **外部** firtool 编译门禁。
- **对照 FR127：** `formal-sby` required job 体例可复用；目标 ≠ sby。
- **现有坑：** `scripts/firtool-smoke.sh` 缺工具 `exit 0` — **不得**复用为 FR137 门禁；新脚本必须 fail-closed。
- **NFR58：** 脊柱 / docs / CI 合同同步后再宣称实现完成。
- **NFR56：** 不得破坏 FR95/110/121/129。
- **NFR59：** 仿真门禁、全 MLIR lower、非 linux-x64 资产、firtool-1.156.0+ 仍未列入。

### Locked shape (E1–E4) — copy from NFR14

| ID | 钉死 |
| --- | --- |
| **E1** | firtool-1.155.0 + tag/`firrtl-bin-linux-x64`（或 AD-9 缓存）；≠ PATH 随机；≠ CIRCT HEAD |
| **E2** | CI required job **与/或** `just circt-external-check`；外部 CIRCT **编译**；无 continue-on-error |
| **E3** | 缺工具 / 版本不符 → **非零** + **可读**；禁止静默跳过 |
| **E4** | ATDD 覆盖 E1–E3；≠ FR129/121/110/95/96 alone；≠ docs-only |

### Mirror pattern (FR127)

```text
formal-sby job + ci-install-sby.sh + formal-sby-check.sh + docs/fr127
  → circt-external job + cargo bitloom firtool ensure (AD-9) + circt-external-check.sh + docs/fr137
```

### Recommended script behavior

1. Banner: FR137 external CIRCT **compile** gate (Bitloom; ≠ FR129 alone)
2. `BITLOOM_CIRCT_FORCE_MISSING=1` → stderr readable + exit 1
3. Resolve binary via `cargo run -p bitloom -- firtool ensure`（或已设 `RHDL_FIRTOOL_PATH`）— **never** bare `command -v firtool` as success path
4. `firtool --version` must contain `1.155.0`
5. Compile `crates/rhdl-firrtl/fixtures/external_hierarchy.fir`（或文档钉死等价）→ `target/circt-external-check/*.v`
6. Success log; any failure → non-zero

### Project Structure Notes

- 不改设计 crate 依赖图；门禁在 CLI/CI/scripts 层。
- 不触 76.3 收口勾选；不把 firtool-smoke 的 silent skip 改成「已满足 FR137」 alone。

### References

- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic76-external-circt-compile-sim-gate.md`]
- [Source: `crates/bitloom/src/firtool.rs` — AD-9 ensure]
- [Source: `.github/workflows/ci.yml` — formal-sby required job]
- [Source: `docs/fr127-forced-sby-ci.md`]
- [Source: `docs/fr129-circt-handshake.md`]
- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 76 / Story 76.2]
- [Source: `AGENTS.md` — Bitloom / bitloom-prelude]

## Testing

- `cargo test -p bitloom --test fr137_external_circt_compile_sim_gate`
- 本地：`BITLOOM_CIRCT_FORCE_MISSING=1 just circt-external-check` → 非零
- 回归：`fr129_circt_handshake`；`fr121_*`；`fr110_*` / HLS；相关 FR95 测试
- `cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Agent Model Used

Composer (Auto)

### Debug Log References

### Completion Notes List

- E1–E4：firtool-1.155.0 / AD-9；CI `circt-external` + `just circt-external-check`；缺工具/版本不符非零可读；选定 **编译门禁 MVP**（仿真未选 / NFR59）
- NFR58：spine AD-9 + Phase 16、docs/fr137、README、fr129 交叉链
- ATDD 7 passed；code-review **Approve**；**未**启动 76.3 / 未勾选 Epic 76 关闭

### File List

- `scripts/circt-external-check.sh`
- `Justfile`
- `.github/workflows/ci.yml`
- `crates/rhdl-firrtl/fixtures/fr137_external_circt_gate.fir`
- `crates/bitloom/tests/fr137_external_circt_compile_sim_gate.rs`
- `docs/fr137-external-circt-gate.md`
- `docs/fr129-circt-handshake.md`
- `README.md`
- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic76-external-circt-compile-sim-gate.md`
- `_agile-output/implementation-artifacts/76-2-外部-circt-编译-仿真门禁实现与验收-fr137.md`
- `_agile-output/implementation-artifacts/atdd-checklist-76-2-fr137-external-circt-compile-sim-gate.md`
- `_agile-output/implementation-artifacts/76-2-code-review.md`
- `_agile-output/implementation-artifacts/76-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`