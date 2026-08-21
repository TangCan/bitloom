---
title: '19.4 实现 Bundle / Vec 可综合路径（FR51）'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_revision: 'cd0cb4fa35956c3631ceace70da2596344e1cacf'
baseline_commit: 'cd0cb4fa35956c3631ceace70da2596344e1cacf'
review_loop_iteration: 1
followup_review_recommended: false
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-19-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/19-3-修订-ad-20-允许-bundle-vec.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** AD-20 / FR51 已允许可综合 `Bundle`/`Vec`，但 prelude / builder / emit 仍无文档化复合端口表面；设计者只能写标量端口。

**Approach:** 在 `bitloom-prelude` 提供文档化的 `Bundle` 与 `HwVec<T,N>`（`Vec<T,N>` 等价），elaborate 时展平为标量 HIR 端口；夹具走 emit `.v` → `tick`；位宽/方向错误在 emit 前失败。不扩展公开 HIR Bundle 节点。

## Boundaries & Constraints

**Always:** 公开表面与 emit 语义一致（展平标量端口）；宽/向不匹配 emit 前失败；设计 crate 仅依赖 `bitloom-prelude`；品牌 Bitloom。

**Ask First:** 若必须扩展公开 `GroundType`/HIR Bundle 节点（而非展平）才能验收。

**Never:** 另立第二种公开 HIR； silently 允许无检查的复合连接；把堆 `Vec`/`Box` 当硬件聚合；实现 FR52 ClockDomain 叙事。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Happy Bundle/HwVec | 含复合端口的模块 | elaborate → 叶端口 → emit `.v` → tick | N/A |
| Width mismatch | 复合叶连接同位宽失败 | emit 前诊断（如 E0131） | 不得 emit |
| Dir mismatch | 向输入赋值 / 错误方向 | emit 前失败（如 E0112） | 不得 emit |
| Design deps | 夹具 Cargo.toml | 仅 `bitloom-prelude`（dev 可 sim/vlog） | N/A |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/lib.rs` — `PortField` / `AsGround` / `Input`/`Output`；扩展 `Bundle`、`HwVec`、多叶 `flatten`
- `crates/bitloom-macro/src/lib.rs` — `#[rhdl::module]` 现调 `describe()`；改为按 `flatten` 注册多叶
- `crates/bitloom-builder/src/lib.rs` — `add_input`/`add_output`/`check_connect`(E0131)/`assign_net`(E0112)；可选 `add_port_field` 辅助
- `crates/bitloom-hir` / `bitloom-vlog` / `bitloom-sim` — 保持标量端口（只读复用）
- `examples/fifo_skel` — elaborate→emit→tick 样板
- `examples/counter_ports` — `#[module]` + PortField 样板
- `_agile-output/specs/spec-rhdl/language-surface.md` — Composite / FR51 合同文
- `_agile-output/planning-artifacts/architecture/.../ARCHITECTURE-SPINE.md` AD-20 — 允许展平实现

## Tasks & Acceptance

**Execution:**
- [x] `crates/bitloom-prelude/src/lib.rs` -- 文档化 `Bundle` + `HwVec<T,N>`；`PortField::flatten` 展平叶端口 -- FR51 表面
- [x] `crates/bitloom-macro/src/lib.rs` -- module 宏按 flatten 注册叶 -- 宏路径一致
- [x] `crates/bitloom-builder/src/lib.rs` -- 必要时辅助注册；宽/向门禁复用 E0131/E0112 -- emit 前失败
- [x] `examples/bundle_vec_skel/` -- 新夹具：Bundle 与/或 HwVec → elaborate → emit → tick；负向宽/向测试 -- AC fixture
- [x] `Cargo.toml` -- 注册 example member -- 工作区可测
- [x] `_agile-output/specs/spec-rhdl/language-surface.md` -- 钉死 `HwVec` 为 `Vec<T,N>` 文档等价与展平命名 -- 文档合同
- [x] `_agile-output/implementation-artifacts/sprint-status.yaml` -- backlog → ready-for-dev → in-progress → review → done -- 冲刺追踪

**Acceptance Criteria:**
- Given Story 19.3 已合入，when 使用文档化 Bundle / HwVec（Vec 等价），then 至少一夹具 elaborate → emit `.v` → tick
- Given 位宽或方向不匹配，when elaborate/连接，then emit 前失败
- Given 设计 crate，when 检查依赖，then 仅 `bitloom-prelude`（dev-deps 除外）
- Given 相关路径，when `cargo test` / 夹具测试，then 通过

## Spec Change Log

- 2026-08-21: 实现 FR51 展平路径（`Bundle`/`HwVec`/`PortField::flatten`）；`assign_net`/`assign_reg_d_from` 接 E0131；夹具 `bundle_vec_skel`；language-surface 钉死命名。

## Design Notes

HIR 保持标量 `GroundType` 端口（AD-12/AD-20 允许展平）。公开命名：`HwVec<T,N>` 文档等价 `Vec<T,N>`，避免与堆 `Vec` / E0141 冲突。叶命名：`{field}_{member}` / `{field}_{i}`。

```rust
#[derive(Bundle)] // 或手写 Bundle impl
struct Stream { /* data: UInt<8>, valid: Bool — 以 GroundType 叶表实现 */ }

Input<Stream>           // → stream_data, stream_valid
Input<HwVec<UInt<8>, 4>> // → lanes_0 .. lanes_3
```

## Verification

**Commands:**
- `cargo test -p bundle_vec_skel` -- expected: PASS（含 emit/tick 与 mismatch） — **PASS (2026-08-21)**
- `cargo test -p bitloom-prelude -p bitloom-macro -p bitloom-builder` -- expected: PASS — **PASS (2026-08-21)**
- `cargo fmt --all && just test` -- expected: 全绿 — **PASS (2026-08-21)**

## Review Triage Log

### 2026-08-21 — Formal review pass（blind / edge / verification-gap）
- intent_gap: 0
- bad_spec: 0
- patch: 5: (high 0, medium 1, low 4)
- defer: 4: (high 0, medium 1, low 3)
- reject: 若干（README、spine 再钉名、堆 Vec trybuild、全 assign 路径统一门禁等）
- addressed_findings:
  - `[medium]` `[patch]` Reg.d 路径 E0131 单测 + 夹具
  - `[low]` `[patch]` crate 文档 HTML 实体；HwVec N>0；language-surface MVP 边界；宏路径 emit

### 2026-08-21 — Implementer self-review（前一轮）
- intent_gap: 0
- bad_spec: 0
- patch: 3: (high 0, medium 1, low 2)
- defer: 3
- addressed_findings:
  - `[medium]` `[patch]` `assign_reg_d_from` 同步走 E0131
  - `[low]` `[patch]` builder `mismatched_assign_net_width_rejected`；sprint `last_updated`

## Dev Agent Record

### Completion Notes

- `Bundle` + `HwVec<T,N>`（`Vec<T,N>` 文档等价）经 `PortField::flatten` 展平为标量 HIR 端口
- `#[rhdl::module]` 按叶注册；`add_port_field` 辅助在 prelude
- 宽/向：`assign_net` / `assign_reg_d_from` → E0131；向输入赋值 → E0112
- 夹具 `examples/bundle_vec_skel`：elaborate → emit → tick + 负向宽/向 + 仅 prelude 依赖
- **未**扩展公开 HIR `GroundType` Bundle 节点；**未**交付 `#[derive(Bundle)]`（手写 impl）

### File List

- `crates/bitloom-prelude/src/lib.rs`
- `crates/bitloom-macro/src/lib.rs`
- `crates/bitloom-builder/src/lib.rs`
- `examples/bundle_vec_skel/`
- `examples/counter_ports/tests/ui/bare_uint_port.stderr`
- `Cargo.toml`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/19-4-实现-bundle-vec-可综合路径-fr51.md`
- `_agile-output/implementation-artifacts/19-4-code-review.md`

## Suggested Review Order

**公开表面与展平**

- FR51 入口：`Bundle` / `HwVec` / `PortField::flatten` 合同
  [`lib.rs:88`](../../crates/bitloom-prelude/src/lib.rs#L88)

- `HwVec` 拒绝 N=0
  [`lib.rs:174`](../../crates/bitloom-prelude/src/lib.rs#L174)

- 叶注册辅助（设计 crate 手写 elaborate）
  [`lib.rs:257`](../../crates/bitloom-prelude/src/lib.rs#L257)

**宏与 builder 门禁**

- module 宏按 flatten 多叶注册
  [`lib.rs:41`](../../crates/bitloom-macro/src/lib.rs#L41)

- `assign_net` 接入 E0131 同位宽
  [`lib.rs:847`](../../crates/bitloom-builder/src/lib.rs#L847)

- `assign_reg_d_from` 同步宽门禁
  [`lib.rs:1030`](../../crates/bitloom-builder/src/lib.rs#L1030)

**夹具与文档**

- Bundle 手写 + emit/tick / 宽向负向
  [`lib.rs:13`](../../examples/bundle_vec_skel/src/lib.rs#L13)

- `HwVec` 文档等价与叶命名合同
  [`language-surface.md:13`](../../_agile-output/specs/spec-rhdl/language-surface.md#L13)

## Auto Run Result

- **Summary:** FR51 展平路径落地：`Bundle` + `HwVec` → 标量 HIR → emit/tick；宽/向 emit 前失败；设计仅依赖 prelude。
- **Review:** intent_gap/bad_spec 0；patch 若干已修；deferred 4；follow-up review false。
- **Verification:** `cargo test -p bundle_vec_skel`；`cargo fmt --all && just test`。
