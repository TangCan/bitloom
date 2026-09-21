---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04-generate-tests', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
status: complete
lastSaved: '2026-09-21'
storyId: '128.1'
storyKey: '128-1-外设-nfr14'
storyFile: '_agile-output/implementation-artifacts/128-1-外设-nfr14.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-128-1-外设-nfr14.md'
generatedTestFiles:
  - _agile-output/test-artifacts/128-1-atdd-gate.py
inputDocuments:
  - '_agile-output/implementation-artifacts/128-1-外设-nfr14.md'
  - 'docs/ip/phase24-contract.md'
  - '_bmad/tea/config.yaml'
  - 'Cargo.toml'
  - 'scripts/check_phase24_gate.py'
---

# Story128.1 ATDD

Step01完成：Create模式，六项AC已批准（全目标连续授权），Rust/Cargo后台库与Python标准库状态门禁，无UI/mobile/HTTP。根Cargo配置、既有127.1 ATDD模式、NFR14模板、旧UART/GPIO/CSR接口与状态脚本已读。主要产物是风险正文+工具/基座探针+独立人工有效性审阅；不生成虚构产品红测或文档字符串单测。

activation resolver hooks/persistent均空。TEA配置utils/pact=true、pact_mcp=mcp、执行/栈/browser=auto；语言中文。知识索引与mandate三篇、data-factories、test-quality、test-levels-framework、test-priorities-matrix、ci-burn-in、pact-mcp、confidence-gate、evidence-integrity沿用本会话已完整读取内容，已逐文件SHA核对ATDD与automate副本字节一致；新增component-tdd与test-healing-patterns全文读完。UI机制不适用，不将RTL误当React Testing Library，不启用浏览器healing/fixme。

Pact MCP一次工具清单为空，pact_mcp_reachable=false（能力不可用，并非broker网络失败），不重复探测，无provider边界/推断states。Rust/Python不满足JS工具runner门；无需加载无关Playwright/Pact API profile或安装依赖。无需用户再次确认已批准故事。

Step02完成：backend采用AI generation，依据已批准六AC/正式合同/真实脚本，不录制UI或伪造HTTP服务。

## Step03 策略

| AC | 层级/优先级 | 真实验收方法 |
|---|---|---|
|1|人工/文档 P0|风险正文初始缺失观测；逐项核对M0/M1/M2、当前授权、未来状态和历史deferred|
|2|人工 P0|完整审阅(a)–(d)、10.5–17人日、owner/停止/维护面；字段存在不代替内容有效|
|3|人工 P0|逐条合同/源码对照Timer32、IRQ5、GPIO32、UART32分频/双FIFO、提交/快照/共同reset/旧API边界|
|4|实际工具探针 P0（build执行）|工具/固定BFM与既有CSR真实RTL；ATDD只列输入、输出和失败条件，不宣称新FR197行为|
|5|Python进程集成 P0|35场景：当前1、7个非done NFR14状态×4功能ready共28拒绝、4合法未来正例、M0负例1、提前Epic关闭负例1；临时文件隔离并校验真实sprint字节/SHA未变|
|6|人工 P0|独立审阅/automate/真实clean-fmt-workspace/单故事commit；只关闭128.1，M3不关闭|

Confidence: 9。Rationale: `scripts/check_phase24_gate.py`全文与既有127.1过程集成模式已读，已批准128.1六AC及正式合同给出清晰要求。Unknowns: build工具探针结果和正文有效性未执行，保留未测。RED为缺少指定风险正文的实际观测；既有gate已实现故应GREEN，不故意破坏成熟gate或将历史绿色伪装成新失败。此适配遵守AC5与证据诚实要求，非新外设产品红绿证明。无API端点/E2E/browser/contract测试，无重复unit镜像。

Step04调度：requestedMode=auto、capability_probe=true，runtime支持collaboration subagent，无独立agent-team接口，resolvedMode=subagent；A `/root/peripheral1281_atdd_gate` 与B `/root/peripheral1281_atdd_manual`成功并行启动。timestamp标签1281-20260921；输出/tmp/tea-atdd-{api,e2e}-tests-1281-20260921.json。A生成不运行，由主流程聚合后实际运行35场景；B仅实际观测正文缺失并记录人工标准。无并行性能测量，不宣称加速比例。

## Step04/04c完成

两worker成功，A源文件与JSON逐字一致并回写；B完整人工清单与JSON一致。35个P0进程场景（未执行）与15个人工项（unreviewed）分开计数；E2E为0。实际缺正文检查exit1，仅表示文档RED。Python临时fixture/helper已包含于脚本，不创建JS merged-fixtures或新共享依赖。workerJSON归档至故事目录。

Playwright Utils deviations: None（Rust/Python runner不适用，无待接认证/HAR/webhook）。Pact.js Utils deviations: None（无consumer/provider产物）。test.skip仅适用新测试scaffold，本故事批准AC5要求真实现有gate，故诚实注明existingGREEN例外，不能制造失败或把已有行为漏测。无placeholder断言；每个负例核对完整诊断和exit。

后续build须填写风险正文(a)–(d)、真实工具/基座探针、逐项人工审阅；35场景只验状态流程，不证明正文有效或外设实现。第七步才commit。

## Step05 最终验证与交接

`python3 _agile-output/test-artifacts/128-1-atdd-gate.py`实际exit0，1.971s，35个P0场景全部通过；逐场景原始诊断、真实sprint字节/SHA保护见128-1-atdd-gate.log/json。RED仍为风险正文缺失exit1，不能据35个GREEN状态场景称正文有效。15个人工项及工具/基座探针未执行，build后逐项复核；未创建外设、未skip或伪造产品失败。

Checklist按实际栈核对完成：六AC有映射、故事/产物链接/前置信息完整；进程场景一个状态规则同时断言exit/stdout/stderr，临时fixture自动清理、标准库无新增依赖；固定状态词是合同数据，不用faker替换。UI selectors、mock endpoints、network-first、browser teardown、JS package配置均N/A。无浏览器进程；workerJSON已复制到故事证据目录，不仅留/tmp。脚本原生assert在优化Python下会被禁用，入口显式拒绝-O/PYTHONOPTIMIZE，避免假绿。

实施顺序：T1正文前置与(a)–(d)；T2精确接口/风险矩阵；T3真实工具及既有基座RTL/BFM；T4重跑35场景和独立跨故事边；T5独立review、automate、clean/fmt/justtest、单故事commit。复核无未解决架构/工具阻塞才关闭128.1；范围估算0.5–1有效人日，整个Epic10.5–17。参考完整人工清单`128-1-atdd-manual-acceptance.md`。用户要求下一技能为`bmad-build`，此处不使用旧dev-story替代。整体目标继续active。
