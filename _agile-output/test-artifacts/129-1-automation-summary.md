---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: step-04-validate-and-summarize
lastSaved: '2026-09-22'
inputDocuments:
  - _bmad/tea/config.yaml
  - _agile-output/implementation-artifacts/129-1-组合系统与证据-nfr14.md
  - _agile-output/implementation-artifacts/spec-129-1-system-nfr14.md
  - _agile-output/test-artifacts/129-1-code-review.md
---
# Story129.1 automate

BMad integrated，backend（Cargo/Rust+Python subprocess+真实RTL），已有Cargo tests及Python过程gate，框架就绪。无浏览器、HTTP、移动或provider边界，Playwright/Pact flags不适用，不生成JS或伪Pact。工具列表一次检查：pact_mcp_reachable=false，fallback=none，无provider states；不重试。core知识原则：独立oracle、最低有效层级、避免重复、缺工具失败、可检错而非恒真、真实产物；硬件仿真周期非墙钟sleep。

自动模式能力检查：运行时有spawn_agent，已真实执行；无单独agent-team工具。requested auto/probe true，resolved subagent。两路API/后端worker，E2E/mobile不适用。

## Coverage plan

AC1–4/7人工风险合同审计已有逐项证据，不以文档关键词测试替代。AC6已有84精确诊断普通/优化测试，避免复制。AC5已有三后端26命令、独立VCD15采样及旧BFM/decoder真实入口；新增P0负控制验证现有TB确能拒绝复位断线/错误极性等实际RTL变异，复用原生成RTL，不修改产品。P1执行器失败边界检查聚焦本次code-review改变的超时/失败留证，不把mock工具成功当产品PASS。AC8完整clean/fmt/just test在第六步。

Worker A审计Python执行器/API契约，只针对真实未覆盖风险补必要检查；Worker B增加真实硬件负控制，保存基准PASS/变异FAIL及源码SHA和命令。当前不实现129.2系统，不扩大本故事范围。

## Aggregation

两worker输出schema已核success=true；API 6个P1、backend 3个P0（1正常控制+2故障变异），总9场景。没有新共享fixture或产品修改。测试已由worker写盘，内容主代理已读，直接采用；不重建重复测试。backend normal/-O各3通过，API 6通过；后续主代理复验。并行收益未测量，不宣称百分比。

## Validation / checklist

主代理复跑6个P1全部PASS；3个P0基准PASS、两变异均真实编译成功而VVP FATAL，具体诊断匹配。首次主代理PATH缺iverilog以非零失败，保留至129-1-automate-root-evidence.tar.gz；按已核工具PATH重跑成功，不将缺工具记skip。worker原始normal/-O证据见129-1-automate-reset-evidence.tar.gz及manifest。所有当前源码/runner SHA与报告匹配。无浏览器孤儿，临时进程由测试清理；永久证据归档保存在test-artifacts。

检查清单适用项：已有Rust/Python框架、AC对应、P0/P1、独立oracle、隔离临时目录、实际失败退出码、诊断、超时、归档及明示限制通过。JS/TS、页面选择器、HTTP/auth/provider、faker用户数据、package.json项N/A；没有为满足模板造假框架。9场景不与原84过程gate重复：验证oracle敏感度及执行器失效边界；RTL黄金固定数值为确定性硬件向量，seed=N/A。未声称总体覆盖百分比或性能收益。原系统贡献/CI仍属129.3。

运行：`python3 _agile-output/test-artifacts/129-1-automate-runner-check.py`；提供Icarus/vvp PATH后`python3 [-O] _agile-output/test-artifacts/129-1-automate-reset-controls.py --output-dir <新目录>`。后者直接读取随提交归档，无/tmp输入依赖。

Playwright Utils deviations: None（scope不适用）；Pact.js Utils deviations: None（无contract artifacts）。下一步骤为用户要求的clean/fmt/just test及单故事提交，暂未完成。
