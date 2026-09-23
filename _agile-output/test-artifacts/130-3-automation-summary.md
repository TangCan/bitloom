---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-23'
story: '130.3'
mode: BMad-integrated
stack: backend
executionMode: sequential
pact_mcp_reachable: false
inputDocuments:
  - _bmad/tea/config.yaml
  - _agile-output/implementation-artifacts/spec-130-3-real-external-core-pilot.md
  - crates/bitloom/src/external_ip_tests.rs
  - scripts/phase24-external-ip-pilot.py
  - scripts/phase24-external-ip-replay.py
---

# Story130.3 自动化扩展

## 预检与适用范围

Richard，本轮只扩展 Rust/Python 后端证据消费层。`Cargo.toml`、Rust 内置测试与 `crates/bitloom/tests/` 已存在；既有 binding/behavior/P0 pilot 测试可复用。没有本故事相关 HTTP provider、浏览器、移动端或 Pact 边界，不生成相应模板。

工作流 resolver 的 prepend/append/persistent facts/on_complete 均为空。已读取配置、技能步骤、既有 ATDD/spec、实现与验证脚本；依据 test-levels、test-priorities、data-factories、selective-testing、ci-burn-in、test-quality、evidence-integrity 与 confidence-gate 选择覆盖。Playwright/Pact 两个 mandate 的语言/runner/安装条件不适用于本 Rust/Python 套件；无工具替代偏差。Pact broker: unreachable (SmartBear MCP tools not available)，无 provider state 需要推导。

执行模式：请求 auto，capability probe=true；当前四个 agent slots 全占用（parent、acceptance、verification、本 agent），本 worker 无可用子槽，因此按技能 fallback 顺序执行 API worker（无适用目标，0文件）再 backend worker，不声称并行提速。

## 目标与信心

Confidence: 9/10。依据 `spec-130-3-real-external-core-pilot.md` 的真实 RTL/禁网/身份要求和两个实际 Python producer，可精确确认消费字段。Unknowns：父任务最终归档路径尚未固定；验证器接受显式 work 路径，不猜测 canonical 完成状态。

| ID | 优先级/层级 | 覆盖 | 分工 |
|---|---|---|---|
| 130.3-INT-EVIDENCE | P0 / evidence contract | 状态/退出码/完整stdout、非零循环、wrapper/source hash、工具及真实编译 helper、禁网复制输入/只读缓存 | 本轮新增独立消费与负测 |
| 130.3-UNIT-EXISTING | P0 / Rust unit | Bender、端口/参数/reset、真实 HIR 父模块、输出保护 | 复用实现修复的单测，留原始执行日志 |
| 130.3-INT-MUTANT | P0 / RTL oracle | 空父模块与零输出父模块必须被真实模拟器拒绝 | 显式运行既有专用 ignored 测试 |
| 130.3-E2E-PILOT | P0 / process | 干净fetch、normal/-O、coherent P0、隔离复制cache | verification worker 维护；本轮读取结果并在sentinel修复后重跑--reuse，不重复联网 |

证据 validator 不能证明抗恶意签名伪造；其职责是拒绝无效/空洞/不自洽归档，产品行为仍由实际 Rust/RTL/isolated pilot gates 证明。

## 生成与聚合

API worker 按 sequential 模式完成范围分析：0 tests、0 files，无适用 HTTP/Pact 边界。Backend worker 生成 1 个独立 Python evidence consumer：1 个真实正基线 + 18 个 P0 变异场景，共19场景。结果经 `/tmp/tea-automate-{api,backend}-tests-130-3-2026-09-23.json` 和 aggregate JSON 交接；没有声称启动子 agent。夹具直接读取显式 provisioned pilot 工作目录，变异只作用于 deepcopy，未写原 lock/cache/evidence。

新增 `scripts/phase24-external-ip-automation-test.py`；消费完整阶段记录、source/lock/wrapper 关联、模拟器与编译 helper 身份、非零完整模型循环、真实隔离挂载和 cacheIntegrity。18项分别为零循环、仅成功标记、失败退出码、失败输出混入PASS、缺模拟器、缺编译helper、空wrapper、wrapper hash漂移、source hash漂移、缺source closure、缺tool identity、缺copied input、暴露宿主根、缺禁网、cache变化、缺实际probe、缺精确host targets、零时长。所有条件用显式异常，`python -O` 不会删除校验。

按父任务追加修复 `130-2-evidence-consumer.py` 的历史 HOME/XDG 假设；现核验 `/tmp/home`、`/tmp/xdg-cache`、`/scratch` 与 read-only `/input`、`/cache`，还要求 sibling isolation probe 和原/复制cache摘要一致。FR199不强制FR200模拟器字段；若后续lock带模拟器字段则不得残缺。不要求 binding/behavior，不扩大 source-only 合同。新增4个隔离证据负测，旧7个场景保留。

## 实际验证

所有命令退出码为0；未运行新联网获取，未重建 canonical evidence。

| 命令 | 实际结果 | 原始归档 |
|---|---|---|
| `cargo test -p bitloom --bin cargo-bitloom external_ip::tests -- --nocapture` | 10 passed / 0 failed / 1 ignored；ignored未计PASS | `130-3-automation-rust-unit.log` |
| `cargo test -p bitloom --bin cargo-bitloom p0_behavior_oracle_kills_empty_and_zero_output_wrappers -- --ignored --nocapture` | 1 passed；empty在异步reset检测失败，zero-output在count=1后检测失败，真实iverilog/vvp执行 | `130-3-automation-simulator-mutants.log` |
| `python3 scripts/phase24-external-ip-automation-test.py --work /tmp/bitloom-130-3-review-pilot-final --self-test --out _agile-output/test-artifacts/130-3-automation-evidence-normal.json` | 实际normal evidence正基线104 cycles；18/18变异被正确理由拒绝 | `130-3-automation-evidence-normal.log` / `.json` |
| `python3 -O scripts/phase24-external-ip-automation-test.py --work /tmp/bitloom-130-3-review-pilot-final --mode optimized --self-test --out _agile-output/test-artifacts/130-3-automation-evidence-optimized.json` | 实际optimized evidence正基线104 cycles；18/18变异被正确理由拒绝 | `130-3-automation-evidence-optimized.log` / `.json` |
| `python3 _agile-output/test-artifacts/130-2-evidence-consumer.py --fixtures /tmp/bitloom-130-3-automation-fixture/source.json /tmp/bitloom-130-3-automation-fixture/source.lock.json /tmp/bitloom-130-3-automation-fixture/online.json /tmp/bitloom-130-3-automation-fixture/offline.json` | 11 unittest cases passed；各场景在normal/-O子进程各执行，共22次；无skip | `130-3-automation-legacy-evidence.log` |

Legacy临时fixture的 `online.json` 来自真实pilot的 `fetch.json`，stdout/stderr/exit/command不变，仅依该fetch产生的lock补充旧consumer要求的metadata字段，文件内标记 `enrichmentNote`；不是重新执行fetch，也不是canonical证据更新。新consumer直接读未加工pilot的normal/optimized原记录。

## Checklist 与边界

- 框架就绪、目标/优先级/实际断言对应、独立证据层无重复Rust实现；没有新增依赖、mock RTL、浏览器、DB或Pact fixture。
- 既有专用ignored测试被单独显式执行，原始失败诊断证明空壳与零输出不能冒充行为PASS。
- 无硬等待、无随机数据竞争、无共享可变fixture；新增文件小于1000行；metadata检查不依赖Python assert。历史consumer以unittest assertion执行，normal/-O均验证。
- Playwright Utils deviations：None（runner/language适用条件不成立）。Pact.js Utils deviations：N/A（无consumer/provider边界）。无浏览器session需要关闭。
- 未声称一般参数、native层级模拟、抗恶意签名伪造或全Phase24完成。循环数表示模型采样步骤，不伪称独立formal assertion计数。实际上游formal与最终clean/fmt/workspace回归仍由父任务整体验收。
- 未做无必要burn-in；normal/-O分别验证优化模式不删除守卫。工作流未自动调用ATDD或test-review。

下一步：父任务归档最终源身份与执行证据，完成独立FR199关闭、更晚FR200关闭及指定clean/fmt/regression；本自动化结果不能替代那些门禁。

## Sentinel 修复后最终复验

宿主 sentinel 改在独立 `bitloom-host-only-*` 临时目录，不再写入原 checkout/cache。复核 probe 精确传入原 checkout、原cache、原manifest、独立sentinel目录/文件5个已知存在目标；consumer拒绝缺失目标或sentinel回到受保护输入。`130-3-review-verification/readonly-input-replay.json` 是原checkout与cache外层只读挂载下实际成功的source-only replay，确认无需修改原输入。

随后按顺序实际运行 `python3 scripts/phase24-external-ip-pilot.py --work /tmp/bitloom-130-3-review-pilot-final --reuse` 和 `python3 -O scripts/phase24-external-ip-pilot.py --work /tmp/bitloom-130-3-review-pilot-final --reuse`，均exit0 / `FR200_PILOT_PASS`。原始stdout见 `130-3-automation-pilot-normal.log`、`130-3-automation-pilot-optimized.log`；完整各阶段/负测记录替换到 `130-3-review-verification/normal/`、`optimized/`。首次真实fetch.json保留，未重新联网。

在两次pilot恢复原fixture后再执行表列consumer命令，normal/-O各19场景（1真实基线104 cycles + 18拒绝变异）与legacy11cases/22子进程全部通过。加强挂载allowlist后，原“缺input”变异先触发未知mount诊断；已改为真正移除三项input挂载参数，使测试只依预期“copied inputs”拒绝理由通过。

Justfile和CI已在两种pilot完成后调用normal/-O独立consumer自测。保留>=86模型采样步作为既有最低界，当前原始实际结果是104。此复验不代替父任务全workspace回归。
