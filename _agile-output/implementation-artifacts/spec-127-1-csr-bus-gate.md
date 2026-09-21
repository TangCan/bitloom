---
title: '127.1 CSR 与总线风险闸门'
type: 'chore'
created: '2026-09-21'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
baseline_commit: 'e88a22bef266b52174be51f831d7ef510b65f108'
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/127-1-csr-总线-nfr14.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-127-context.md'
  - '{project-root}/docs/ip/phase24-contract.md'
  - '{project-root}/_agile-output/test-artifacts/127-1-atdd-manual-acceptance.md'
---

<frozen-after-approval reason="用户已批准整体七步执行">

## Intent

**Problem:** M1 已关闭，CSR 功能实现尚缺可审计的 NFR14 记录。
**Approach:** 完成风险正文与真实工具/状态门禁证据，为127.2提供明确前置。

## Boundaries & Constraints

**Always:** 覆盖故事全部六项 AC 与人工 M01–M14；保留固定合同及历史状态，区分工具可用性和产品验证。
**Never:** 本次不实现 CSR、不改产品/API/工具钉、不关闭 FR196；不 clean、提交、推送或改 sprint/goal。主代理负责后续审查、automate、完整回归及提交。

</frozen-after-approval>

## Code Map

- `docs/ip/phase24-contract.md`：接口权威；故事中已列完整风险分解。
- `_agile-output/implementation-artifacts/nfr14-risk-record-template.md`：必填(a)–(d)。
- `scripts/phase24_axi_bfm_probe.py` 与 `docs/ip/phase24-axi-bfm-probe.md`：复用独立 stub，五通道暂停/reset取消；不是新 CSR。
- `scripts/check_phase24_gate.py`：复用状态门禁；跨故事依赖需人工审计。
- `_agile-output/test-artifacts/127-1-atdd-gate.py`：27个临时状态正负样例；保持实际 sprint 字节不变。

## Tasks & Acceptance

**Execution:**
- [x] `_agile-output/implementation-artifacts/epic-127-nfr14.md`：填约束、估算、owner、维护成本、接口/依赖矩阵、停止与失败动作；全部合同不得遗漏。
- [x] `_agile-output/test-artifacts/127-1-build-*`：记录 UTC、完整命令、路径、版本、返回码；重跑 BFM 与 gate，保存原始日志和 JUnit/stub 到 target 之外以供未来 clean 后复核。
- [x] `_agile-output/test-artifacts/127-1-atdd-manual-acceptance.md`：逐项实质核验并引用证据；M13七步未结束保持待验，其他缺证据也不得提前勾选。

**Acceptance Criteria:**
- Given 正式合同，when 阅读正文，then NFR14(a)–(d)与故事接口/风险全部可追溯，无工具/架构未解决阻塞才可声明风险内容有效。
- Given 固定环境，when 实际运行探针，then 保存真实退出码与输出；失败保持可见并修复，禁止 skip 替代。
- Given gate，when 执行27场景，then 正负均符合预期且实际 sprint 未变；手工核对额外依赖。
- Given 尚无 CSR 实现，when 汇报，then 不宣称 FR196 已交付，M13等待主代理后续完成。

## Implementation Notes

2026-09-21：风险正文、27场景门禁与真实BFM探针完成；实施证据见127-1-build-evidence.md。pip缺失保留exit1，metadata替代查询成功。人工M13待后续七步关闭，无产品修改。

2026-09-21 build：完成风险正文(a)–(d)、完整接口/依赖/停止矩阵，固定环境版本与安装源码核验；27状态场景及BFM exit0，JUnit/stub归档target外。首次pip版本查询exit1保留，以importlib.metadata实际五包查询解决；没有安装或升级。人工清单追加逐项自检，独立裁定留后续review，M13明确待验。未改sprint/goal/产品/API，未clean/commit。入口：`_agile-output/test-artifacts/127-1-build-evidence.md`。

## Spec Change Log

## Review Triage Log

2026-09-21 build三路审查：blind 10项，edge []，verification无缺口。平台线程限制使第三路在前两路返回后启动，收齐后统一裁定。

| ID | 裁定 / 证据 / 动作 |
|---|---|
|B1|false：实际gate不检查跨epic功能依赖；127 fixture只重置127，不影响128–130各自.1，后续合法状态不会破坏正例。|
|B2|medium / patch：Python -O会移除断言仍打印PASS；入口拒绝优化模式，负测验证非零且无PASS。|
|B3|low / patch：合法标识符表述未列保留字与生成宏碰撞，补明确命名空间/大小写碰撞诊断，交127.2测试。|
|B4|low / patch：适配方案缺畸形编码失败边界，补未知schema、缺索引、计数和字节范围诊断责任。|
|B5|false：未承诺无界硬件或资源保证，文本只论证有限描述可用有限参数表达；127.2支持规模在功能规格确定，当前未引入不受限运行时入口。|
|B6|low / patch：补127.2最小C消费者及多头文件共同编译，验证生成物可用性。|
|B7|low / patch：补127.2/3/4各自安全proof与cover责任，保留活性假设边界。|
|B8|false：AC要求命令/退出码/原始日志而非新收集器；probes.json和日志已有完整argv，BFM脚本自身严格验证JUnit；增加重复收集框架不必要。|
|B9|low / patch：同步故事实施状态、T1–T4和当前证据入口，历史创建/ATDD明确标时点。|
|B10|medium / patch：各100字节存在十进制歧义，改0x100并给首末地址显式0x。|

## Verification

- `/tmp/bitloom-phase24-bfm-py312/bin/python scripts/phase24_axi_bfm_probe.py`，PATH 前置 `/tmp/bitloom-maintenance-tools/bin`，使用有限 timeout 并 kill-after；实际 JUnit 无 fail/error/skip。
- `python3 _agile-output/test-artifacts/127-1-atdd-gate.py`：27场景通过。
- 探测 rustc/cargo、Python及固定包、iverilog/vvp、sby/yosys/z3。SBY候选 `/tmp/bitloom-1263-sby-installed/bin/sby`；环境不存在则记录并修复，不猜版本。
- 人工逐条对照完整故事与正式合同。无意图缺口，无不可逆操作；正文与证据涉及多文件，采用 dispatch。仅在已批准范围内自行解决细节。
