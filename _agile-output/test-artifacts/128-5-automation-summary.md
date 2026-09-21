---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-21'
storyId: '128.5'
detected_stack: backend
mode: BMad-Integrated
pact_mcp_reachable: false
pact_fallback_source: none
inputDocuments:
  - _agile-output/implementation-artifacts/128-5-缓冲-uart-与-m3-关闭.md
  - _agile-output/implementation-artifacts/spec-128-5-uart-csr.md
  - _agile-output/implementation-artifacts/epic-128-nfr14.md
  - _agile-output/test-artifacts/atdd-checklist-128-5-缓冲-uart-与-m3-关闭.md
  - _agile-output/test-artifacts/128-5-build-contract-matrix-audit.md
  - _agile-output/test-artifacts/128-5-code-review.md
  - _agile-output/test-artifacts/128-5-automate-knowledge-sha.json
  - _bmad/tea/config.yaml
---

# 128.5 自动化覆盖检查

Step01：Rust/Cargo backend，框架与测试存在；无HTTP/浏览器/移动或consumer-provider边界。Playwright/Pact flags均true但库/runner适用性门不满足，不安装或生成JS替代物。单次Pact工具列表探测空，未访问broker、无fallback provider。14篇知识与128.4已缓存版本逐SHA一致（含核心六篇和选择性测试），使用缓存；附加mandate重读截断不声称新全文加载。配置Richard/中文，hooks与persistent facts为空。当前采用Create模式继续已有完整授权。

已有实证：source-bound完整39命令PASS，普通15、专用formal3和四backend入口实际通过；ordinary/-O harness各10，API及两层审查完整，RX前后一拍capture双变异已用不变黄金检出。不以仅emit/ignored/模型自检当产品PASS。

## Step02 覆盖计划

|等级/目标|优先级|现有覆盖与扩展判据|
|---|---|---|
|API/静态软件边界|P0/P1|15端口、六描述、共享/重命名诊断、两FIFO复用、原文例/C11/旧接口。worker A仅找非重复缺口，不新建HTTP/Pact边界。|
|CSR/串行/队列/事件/reset|P0|三seed绝对deadline+VecDeque，3785拍12类边界，真实loopback/双实例IRQ、全宽direct注入、行为突变。worker B寻找遗漏的可观察行为，先确认现有向量是否已检出。|
|后端/形式/证据|P1|direct/FIRRTL/Chisel四入口、有限prove/三cover/原始综合、exact gate/日志/字节绑定。真实新缺口才加测试；不扩全协议形式、物理签核或native层级。|

选择性扩展：避免跨层重复、实现镜像和仅为增加数量的测试。若无新增P0/P1缺口，记录零新增并保留已完成实际验证；最终clean/fmt/workspace由主代理执行。现有审查已实测排除RX前后一拍采样的疑似缺口，不能将同一建议未经核对再列缺陷。所有覆盖矩阵以当前story/批准合同为准。

## 聚合与验证结果

两位worker均success，输出已保存在`128-5-automate-api-worker.json`与`128-5-automate-backend-worker.json`。新增测试0、fixture 0、产品和既有测试修改0；新增P0/P1/P2/P3均0。既有API 9项（P0 5/P1 4）不可算新增；UART共22入口，其中15普通、4专用backend、3专用formal。详细AC1–AC7映射见backend worker。没有发现非重复P0/P1缺口，不以零新增声称worker执行通过。并行审计没有串行基线，不声称百分比加速。

checklist：Cargo/Rust框架与CI命令已具备，固定seed及独立deadline/VecDeque黄金覆盖合同；真实多后端属于降级语义核对，非无意义重复。用现有source-bound 39命令PASS及normal/-O各10项harness实证验证，不重复运行未修改目标。HTTP、浏览器、auth、JS factory、package.json、selector/healing项目均N/A；硬件确定向量和逐拍多信号断言保留，不替换为faker或单断言网页模式。未生成测试所以无healing，无新增fixture或README需要维护。不存在浏览器session。worker临时结果已复制至test-artifacts；完整运行原始日志/VCD/命令已归档并逐成员独立核验，清理不会消除证据。

## Playwright Utils deviations

None。非Playwright Rust backend，checklist明确允许跳过，不存在未接线认证或HAR需求。

## Pact.js Utils deviations

None。无consumer-provider边界、未生成CDC合同、未访问broker。

复现沿用`128-5-build-runner.py`及`128-5-build-harness-tests.py`，环境与归档入口见`128-5-build-review.md`。范围仍为有限安全证明/direct极值注入，不主张完整协议形式、native层级仿真或物理MTBF。下一步按用户七步执行实际`cargo clean`、`cargo fmt --all`、`just test`，通过后关闭M3并单故事提交；本automate完成不提前关闭故事。
