---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-23'
storyId: '130.3'
storyKey: '130-3-一个真实外部核适配及试点关闭'
storyFile: '_agile-output/implementation-artifacts/spec-130-3-real-external-core-pilot.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-130-3-一个真实外部核适配及试点关闭.md'
generatedTestFiles:
  - 'crates/bitloom/tests/fr200_external_ip_binding.rs'
  - 'crates/bitloom/tests/fr200_external_ip_behavior.rs'
---

# ATDD Checklist: Story 130.3

## Preflight

- Stack: backend Rust workspace (`Cargo.toml` and integration-test harness present).
- Generation mode: AI/sequential backend mode; browser/API workers are not applicable to RTL CLI behavior.
- Story: `spec-130-3-real-external-core-pilot.md` with clear BDD acceptance criteria.
- Core knowledge applied: backend unit/integration levels, TDD red-green-refactor, fail-closed verification, independent-oracle testing.

## TDD Red Phase

初次生成的四个 ignored/panic scaffold 只证明占位测试失败，不构成行为层 RED，本次关闭不把它们计入验收。四层审查发现 JSON-only 绑定绕过真实父模块后，按验收条件补成可执行断言；`130-3-review-binding-red.json` 只记录静态缺口，不冒充测试运行。真正的空壳/零输出 RED 由 `130-3-automation-simulator-mutants.log` 保存：真实模拟器分别在 reset 与首个有效数据步骤拒绝错误父模块。

## Acceptance Criteria Coverage

| AC | Red scaffold | Required green evidence |
|---|---|---|
| Binding matches lock | `binding_matches_locked_fifo_module_and_shape` | Deterministic binding JSON and exact module/source/parameter/port/reset comparison |
| Binding mutation fails closed | `binding_rejects_source_parameter_port_and_reset_drift` | Non-zero diagnostics; canonical artifacts unchanged |
| Independent FIFO behavior | `locked_fifo_matches_independent_model_for_reset_and_boundaries` | Real locked RTL plus separate reference model covering reset, empty/full, simultaneous read/write, width/depth |
| Offline pilot replay | `offline_pilot_replay_consumes_only_copied_lock_and_cache` | Isolated copied cache, no network/host cache, actual parse/compile/sim and tool/source evidence |

## Red-phase files

- `crates/bitloom/tests/fr200_external_ip_binding.rs`
- `crates/bitloom/tests/fr200_external_ip_behavior.rs`

## Risks and assumptions

- The stable `fifo_v3` port/parameter shape must be read from the lock, never copied from master documentation.
- Missing Yosys/bubblewrap/upstream test tooling is a hard evidence gap; it cannot be converted to PASS by ignoring the test.
- No browser, Playwright, Pact, or E2E artifacts are applicable to this backend RTL story.

## 修复后的实际测试

- 四项 real-tool integration 已显式激活通过；两项 hermetic integration 在普通测试下通过，默认四项 ignored 不计 PASS。证据见 `130-3-review-verification/fr200-provisioned-tests.log` 和 `hermetic-tests.log`。
- 真实 HIR 父模块、来源/全端口/参数/reset 完整映射、工具/helper hash、隔离复制 cache 均有实际断言。
- 每种 Python 模式各 22 个产品负测；独立 evidence consumer 各 18 个损坏证据负测，先验证真实正基线再判断预期失败原因。
- 独立队列模型实际 104 步；有数据复位和 flush 已补入。固定单参数集，不把其他 upstream 参数当 Bitloom 支持承诺。

这些是审查驱动的可执行验收修复，不能反写为初始 scaffold 已完成有效 TDD。上游完整套件、独立 Story130.2 关闭与全 workspace 回归仍必须由最终证据单独闭合；本 checklist 不代替它们。
