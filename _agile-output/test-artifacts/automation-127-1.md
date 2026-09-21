---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-21'
inputDocuments:
  - _bmad/tea/config.yaml
  - _agile-output/implementation-artifacts/127-1-csr-总线-nfr14.md
  - _agile-output/implementation-artifacts/epic-127-nfr14.md
  - _agile-output/implementation-artifacts/spec-127-1-csr-bus-gate.md
  - _agile-output/test-artifacts/127-1-independent-review.md
  - _agile-output/test-artifacts/127-1-build-evidence.md
---

# Story127.1 automate

Richard，本次Create沿用七步授权。resolver成功；prepend/append/persistent_facts/on_complete均为空。backend（Rust Cargo workspace + Python标准库门禁 + cocotb）；已有测试框架可用。输出按故事隔离，避免覆盖别的automation-summary。

范围：P1进程CLI集成门禁，补全部advance状态、M0假关闭、FR189篡改；P1既有固定BFM工具探针重新执行并归档对应JUnit。AC1/5机器覆盖，AC4工具探针；AC2/3/6实质人工审阅沿用独立review，不生成文档字符串镜像测试。已有27场景和历史RED/GREEN日志原样保留；本轮只增加覆盖，不制造产品红测。

无HTTP、浏览器、数据库或consumer/provider边界；Playwright/Pact库不适用，不新增依赖。pact_mcp_reachable=false（一次工具清单检查，SmartBear不存在）；无provider states，也无broker调用。范围是过程门禁/工具可运行性，非CSR产品RTL、formal或PPA验收。

执行模式：requested auto，probe true；collaboration可启动subagent，无独立agent-team runtime，因此resolved subagent。API worker已启动，backend同时启动因线程容量失败；等待API结束后分批启动，不伪称并行完成。

Confidence: 9/10。依据：现有127-1-atdd-gate.py隔离fixture、正式故事AC1/5和check_phase24_gate.py的明确状态集合及诊断。未知：新增矩阵实跑尚待；不推断产品功能覆盖。

聚合：API worker生成0项（N/A），backend成功生成117个P1集成场景，其中90新增，原27保留。复用replace_state和TemporaryDirectory，不增加共享fixture。两个worker JSON和summary已归档127-1-automation-*。分批工作不宣称40–70%提速。


验证结果（2026-09-21 UTC）：117/117进程门禁通过（5.441秒，exit0）；Python -O正确拒绝（exit1，0.203秒）；固定BFM探针1/1通过（312ns仿真，0.391秒墙钟，exit0）。每次完整argv、UTC、PATH、退出码与原始输出保存在127-1-automation-{gate,optimized,bfm}.log；对应gate JUnit为127-1-automation-gate.xml，BFM原生JUnit/stub在127-1-automation-bfm-artifacts/。真实sprint字节/SHA256未变化。所有结果均在target外，SHA256清单另存。

新增覆盖：7种风险未done状态×3个后续故事×其余4种advance状态=84；M0中每个故事backlog却Epic标done=3；Epic122/122.2/122.3分别篡改done=3。原ready-for-dev矩阵等27例仍在，不把重复执行算新增。FR189负例仅验证这3种done篡改，不声称穷举全部历史非法状态。

首次新增测试在109个成功场景后因fixture正则同时匹配epic-122和epic-122-retrospective而中止，首次log/XML/worker源码快照均保留为127-1-automation-gate-initial.*。修复仅使Epic完整键精确匹配；以“-”结尾的故事prefix仍匹配标题。最终117例全部经过真实gate。不是产品RED，不更改gate实现，不覆盖历史ATDD/build记录。XML由实跑日志和退出码生成，首次包含runner failure，最终严格核对117个PASS后写入；BFM XML直接复制本轮原生结果。

适用checklist通过：BMad集成输入/框架/现有ATDD已读；P1 CLI集成用例具有真实返回码和诊断断言；临时状态自动清理，无硬等待/skip/focus；有限30秒单调用与120秒外层timeout；失败证据保留；无需新fixture/依赖、README命令列于本报告。知识依据：test-levels-framework、test-priorities-matrix、data-factories（有意义域常量可固定）、selective-testing、ci-burn-in、fixture-architecture、test-quality、evidence-integrity以及库mandate范围规则。自动heal未启用；本次明确修复fixture缺陷后单次重跑，无重试掩盖。无需浏览器会话；没有HTTP/CDC/UI因此相关条目N/A，不导入TS框架，不创建package scripts。固定BFM写共享target目录，必须串行运行；它不具并行安全性，不宣称完整CI已通过。

运行方式：`PYTHONDONTWRITEBYTECODE=1 /tmp/bitloom-phase24-bfm-py312/bin/python _agile-output/test-artifacts/127-1-atdd-gate.py`；BFM使用相同Python执行`scripts/phase24_axi_bfm_probe.py`，PATH前置`/tmp/bitloom-maintenance-tools/bin`，外层`timeout --kill-after=10s 120s`。固定环境是本机条件，不是跨机器保证。

Playwright Utils deviations: None（非Playwright Python/Rust运行器，适用性N/A）。Pact.js Utils deviations: None（无consumer/provider边界，没有生成Pact）。新增API/浏览器测试0、P1进程场景117、复用工具探针1，不计算源码覆盖率，不声称新CSR RTL/formal/综合/PPA验收。AC2/3与其余AC6仍按人工审阅和七步后续关闭。建议下一步按既定第6步clean/fmt/工作区回归，再第7步单故事提交；由主代理负责，automate不改story/sprint/goal、不clean/commit/push。
