---
title: '35.2 C ABI 超出 Counter demo（FR83）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '639d369'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md'
  - '{project-root}/_agile-output/implementation-artifacts/35-1-epic-35-nfr14-风险记录.md'
  - '{project-root}/docs/fr33-c-abi.md'
  - '{project-root}/crates/rhdl-cabi/src/lib.rs'
  - '{project-root}/crates/rhdl-cabi/include/rhdl_cabi.h'
warnings: []
deferred:
  - 'Arbitrary user FrozenHir load-from-file via C ABI → future deepen (not FR83 close)'
  - 'SystemC TLM-2.0 as contracted path → AD-5 forbids'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `rhdl-cabi` hardcodes Counter-only DUT (`counter_hir()` / `rhdl_sim_new`); FR33 historical done ≠ FR83 depth (NFR37). No documented second DUT or generic generate/link path; silent failures risk (null/unknown → `0` with no diagnosis).

**Approach:** Land NFR14 FR83 **选项 A**：文档化第二 DUT（`Adder`）+ 命名 DUT 选择入口 `rhdl_sim_new_dut`（通用 generate/link 消费路径的最小合同）；自动化夹具证明非 Counter-only；文档钉死导出符号、生命周期、错误模式；未知 DUT / 空句柄须可诊断失败（非 silent 成功）。不引入 SystemC（AD-5）。

## Boundaries & Constraints

**Always:** Story 35.1 gate satisfied; FR83 Option A (second DUT **or** documented select path + non-Counter fixture); Bitloom brand; C symbol prefix `rhdl_*` ABI stability; clear failure diagnosis; AD-5 (no SystemC contract); AD-18 (no capturing closures into `tick`).

**Ask First:** 无。

**Never:** Claim full arbitrary-HIR C loader as delivered; use Counter-only demo to close FR83; Option B defer while claiming depth; SystemC TLM-2.0 path; silent success on unknown DUT.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Legacy Counter | `rhdl_sim_new()` | Counter DUT; golden rst+3 → data_out=3 | unchanged FR33 |
| Second DUT | `rhdl_sim_new_dut("Adder")` | Adder; a=5,b=7 → sum=12 both views | null + last_error if OOM |
| Unknown DUT | `rhdl_sim_new_dut("Nope")` | null handle | `rhdl_last_error()` non-empty |
| Null handle ops | free/set/tick/get on null | no crash; get→0 | last_error set |
| Docs | fr33 (+ FR83) | symbols, lifetime, error modes | ATDD / doc scan |

</frozen-after-approval>

## Code Map

- `crates/rhdl-cabi/src/lib.rs` — DUT registry; `rhdl_sim_new_dut`; last_error; Adder HIR + abs
- `crates/rhdl-cabi/include/rhdl_cabi.h` — C prototypes
- `crates/rhdl-cabi/tests/harness.c` — Counter (compat)
- `crates/rhdl-cabi/tests/harness_adder.c` — non-Counter C fixture
- `crates/rhdl-cabi/tests/c_harness.rs` — compile/link both harnesses + unknown-DUT fail
- `docs/fr33-c-abi.md` — symbols / lifetime / errors / FR83 Option A
- `crates/bitloom/tests/fr83_c_abi_beyond_counter.rs` — ATDD docs + API surface
- `_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md` — mark FR83 已选 A

## Story

As a 嵌入式 / 验证集成者,
I want cdylib ABI 超出硬编码 Counter,
So that FR33 深度可验收。

## Acceptance Criteria

1. Given Story 35.1, when 支持文档化第二 DUT 或通用 generate/link 路径（FR83）, then 自动化夹具证明非 Counter-only
2. And 文档说明导出符号、生命周期与错误模式
3. And 失败时诊断明确（非 silent 成功）

## Tasks / Subtasks

- [x] T1: `rhdl_sim_new_dut` + Counter/Adder registry + `rhdl_last_error`（AC: 1, 3）
- [x] T2: Adder C harness + Rust golden; unknown-DUT fail fixture（AC: 1, 3）
- [x] T3: `docs/fr33-c-abi.md` FR83 deepen（AC: 2）
- [x] T4: ATDD `fr83_c_abi_beyond_counter`；NFR14 FR83 已选 A；review / sprint done

## Dev Notes

- Prefer Option A over defer; second DUT name **`Adder`** (comb `sum = a + b`, width 8) — golden ≠ Counter `data_out==3`.
- Keep `rhdl_sim_new()` → Counter for FR33 harness compatibility.
- No capturing closures in `tick` (AD-18); Adder abs is plain Rust fn, dissolved before freeze like Counter.
- AD-5: Rust functional/abs path only — not SystemC.
- Do not run full `cargo clean && just test`; prefer `cargo test -p rhdl-cabi` and targeted bitloom ATDD.

### Project Structure Notes

- Stay inside `crates/rhdl-cabi` + `docs/fr33-c-abi.md` + bitloom ATDD; no new crate.

### References

- [Source: `_agile-output/planning-artifacts/epics.md` Story 35.2]
- [Source: `nfr14-risk-epic35-residual-partials.md` FR83 Option A]
- [Source: ARCHITECTURE-SPINE AD-5 / AD-18]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- `cargo test -p rhdl-cabi` (unit + C harnesses)
- `cargo test -p bitloom --test fr83_c_abi_beyond_counter`

### Completion Notes List

- FR83 Option A: `Adder` DUT + `rhdl_sim_new_dut` + `rhdl_last_error`
- Fixtures: `harness_adder.c`, Rust goldens, unknown-DUT fail
- Docs expanded; NFR14 FR83 已选 A + close checkbox
- Code review Approve; no commit (parent)

### File List

- `crates/rhdl-cabi/src/lib.rs`
- `crates/rhdl-cabi/include/rhdl_cabi.h`
- `crates/rhdl-cabi/tests/harness_adder.c`
- `crates/rhdl-cabi/tests/c_harness.rs`
- `docs/fr33-c-abi.md`
- `crates/bitloom/tests/fr83_c_abi_beyond_counter.rs`
- `_agile-output/implementation-artifacts/35-2-c-abi-超出-counter-demo-fr83.md`
- `_agile-output/implementation-artifacts/35-2-code-review.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev)
- 2026-09-09: FR83 Option A implemented; review Approve; done

## Suggested Review Order

**lib.rs DUT path** → **C harnesses** → **docs** → **ATDD**
