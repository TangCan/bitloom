---
title: 'Story 125.1 Phase 24 合同与接口'
type: chore
created: 2026-09-20
status: done
route: dispatch
baseline_commit: 29c52dfbfa403e0ae9930d1061e99cfe73bd41fc
review_loop_iteration: 0
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/planning-artifacts/phase24-composable-ip-proposal/change-set.md'
  - '{project-root}/_agile-output/planning-artifacts/research/technical-bitloom-composable-ip-ecosystem-2026-09-20/implementation-plan.md'
---
<frozen-after-approval>
## Intent
用户已批准完整 Phase 24 提案的规划落地与 M0 执行。125.1 将候选 FR192–201/NFR93–99/Epic125–130 登记为正式合同，固化接口和 Epic125 NFR14，为真实 RTL 红测及修复建立明确边界。
## Boundaries & Constraints
保留历史结项，Epic122/FR189 deferred 不变；后续126–130仅backlog。设计依赖仅prelude；工具钉不变；不写产品 RTL，不改旧接口。批准仅授权M0实现，不能宣称整个Phase24交付。一故事一提交，由主代理提交。
</frozen-after-approval>
## Code Map
- `planning-artifacts/sprint-change-proposal-2026-09-20-phase24-composable-ip.md` 与 `phase24-composable-ip-proposal/`：已批准完整文本与22故事映射，路径前缀均为 `_agile-output/`。
- `_agile-output/planning-artifacts/epics.md`：追加inventory及6epic/22story，保留原所有文本；frontmatter闭合需按实际文件检查，不能破坏既有格式。
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/`：prd/addendum追加指针及合同。
- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`：追加AD30/31。
- `_agile-output/implementation-artifacts/sprint-status.yaml`：development_status内新增22故事、6epic，125/125.1 in-progress，其余backlog；历史保持。
- `_agile-output/specs/spec-rhdl/SPEC.md`、`AGENTS.md`、`README.md`：追加准确计划状态/指针。
## Tasks & Acceptance
- [x] 上述正式文件应用已批准change-set的NEW部分，正式epic故事使用Given/When/Then验收，非OLD/NEW提案叙事。
- [x] 新建 `docs/ip/phase24-contract.md`：提炼计划§3为正式接口合同，补齐CSR位定义/WO读语义/宽度、ID版本决定，明确旧AXI与未来桥不同表面。全部CSR reset0；不提供ID/version寄存器。GPIO32位，IRQ5有效位；UART 8N1/FIFO深4，CTRL bit0使能，timer CTRL bit0 enable/bit1 periodic，其它位保留；其余可从计划合理固化，不引入新功能。
- [x] 新建 `_agile-output/implementation-artifacts/epic-125-nfr14.md`：上游限制、5–8人日估算、责任执行者Codex/项目owner Richard、接口兼容/RTL工具/协议风险、禁止降级、关闭条件。区分已经native观察与尚未RTL复现。
- [x] 新建 `scripts/check_phase24_gate.py`：检查6epic/22story存在，125未done时126–130不能ready/in-progress；关闭后各实现故事仍需对应NFR14 .1 done；FR189/Epic122保持deferred。标准库即可，不加依赖。
- [x] 提案标approved、记录用户批准日期及M0授权；附件保留历史提案文本，追加已落地指针；研究仍是历史研究，不改研究结果。
**Acceptance Criteria:** Given用户批准，When登记完成，Then正式合同、故事、架构和状态一致；Given gate未关闭，When检查状态，Then下游不得ready；Given历史Epic122，When登记Phase24，Then仍为deferred。
## Implementation Notes
用户批准已涵盖此故事；当前未提交文件均为本任务研究/提案，不是未知用户代码，允许一并按相关提交归档。不得执行git commit，由主代理做。
## Spec Change Log
## Review Triage Log
- medium / patch：风险故事Given自依赖，已删除其自身NFR14前置。
- medium / patch：128.5不能在Timer/GPIO未完成时关闭M3，已增加全故事完成条件。
## Verification
运行 `python3 scripts/check_phase24_gate.py`、`git diff --check`；报告改动文件及验证结果。不要对文档变更跑workspace测试。

## 完成记录
正式规划与接口已登记；gate正向、四类负向状态变异与diff空白检查通过。独立审阅发现风险故事自依赖及M3提前关闭两个问题，均已修正。M0产品验证仍待125.2/125.3。
