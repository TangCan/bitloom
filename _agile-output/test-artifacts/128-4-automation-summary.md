---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-21'
storyId: '128.4'
inputDocuments:
  - '_agile-output/implementation-artifacts/128-4-gpio-csr-wrapper.md'
  - '_agile-output/implementation-artifacts/spec-128-4-gpio-csr.md'
  - '_agile-output/implementation-artifacts/epic-128-nfr14.md'
  - '_agile-output/test-artifacts/atdd-checklist-128-4-gpio-csr-wrapper.md'
  - '_agile-output/test-artifacts/128-4-code-review.md'
  - '_agile-output/test-artifacts/128-4-automate-knowledge-sha.json'
  - '_bmad/tea/config.yaml'
pact_mcp_reachable: false
fallback_source: none
---
# Story128.4 自动化覆盖补强

Step01：BMad integrated/Create；Rust/Cargo backend，现有integration tests及RTL/formal框架就绪。hooks/persistent均空，Chinese/Richard，auto/probe=true。相关故事/规格/ATDD/风险及实际测试已读；13篇已完整加载知识与本skill副本SHA逐项相同，新增全文selective-testing。Playwright/Pact mandate按scope不适用Rust硬件测试，不引入JS/UI/HTTP边界。一次Pact工具列表probe为空，未联系broker、不推断provider。Playwright Utils deviations: None（不适用）；Pact.js Utils deviations: None（无consumer/provider边界）。现阶段仅只读分析，完整runner运行期间不启动第二套GPIO测试、不改相关源码。

Step02 coverage plan：硬件公开API为GpioCsr四符号、16端口、六CSR；无HTTP/db/message服务边界，Provider Endpoint Map不适用。选择selective P0/P1缺口审计，避免重复ATDD和刚完成的审查补强。

|AC|层次/优先级|既有覆盖及此次任务|
|---|---|---|
|1,5,6|Rust API/集成 P0/P1|精确端口/描述/诊断/共享及重命名/native拒绝/旧GPIO+FL/软件金样；worker A独立检查遗漏，仅提出可实证缺口|
|2,3,4,5,6|逐拍RTL+形式 P0|三seed55151帧、全32位/16WSTRB/方向同步/事件碰撞/背压/取消、真实三组合；worker B检查因果与负控制敏感性，避免重复|
|7|证据与持续门禁 P1|17 harness normal/-O、exact/name/count、固定seed计数、反例/后端日志强校验；已由review补强，不再复制测试|

无已知必须新增的P0/P1用例；若worker证实缺口则最小补测并实际执行，若覆盖充分则保留现有测试与证据映射，不制造重复用例。源码修改和额外GPIO运行须等待当前完整runner结束。

Step03/03C：auto/probe实际协作工具可用，resolved agent-team；API和backend两个worker均success，JSON全文读取并原字节保存在128-4-automate-worker-{api,backend}.json。新增0测试/0fixture，各AC已有对应覆盖证据，无已证实非重复P0/P1缺口；不添加JS或HTTP scaffolding。总计数只报新增0，不把既有测试冒充新增。并行相对提速未测量，不使用模板40–70%猜测。接下来核验现行完整run及checklist，不额外重复已通过的测试。

Step04/checklist：Rust Cargo框架就绪，AC映射/层次/优先级/确定性隔离/现有fixture已核对；本阶段无新增源码，不另造重复测试。JS/DOM/auth/faker/Pact/浏览器清理条目不适用，无browser session。worker临时JSON原文已复制test-artifacts；summary在本目录。

现行源码完整run `20260921T121311.335406Z-902189`实际全部gate PASS：11普通测试、3形式专用、两个后端exact各1、既有FIRRTL/numeric/SemVer/example/C全部通过；17 harness普通/-O已在针对性修补运行通过。source_complete=true，raw归档及主代理独立复核见128-4-code-review-root-audit.json。此阶段新增0测试（P0/P1/P2/P3均0）、0fixture、0源码改动，保留已有覆盖，不声明穷尽覆盖率。

运行入口：`python3 _agile-output/test-artifacts/128-4-build-runner.py`，所需工具环境见build证据；逐target命令见run commands。下一步为用户明确要求的cargo clean/fmt/just test和单故事commit，不另启动可选test-review/trace。最终全workspace尚待，Story/M3尚未关闭。

## Playwright Utils deviations
None（Rust硬件测试，非Playwright runner）。

## Pact.js Utils deviations
None（无consumer/provider服务边界，不生成Pact）。
