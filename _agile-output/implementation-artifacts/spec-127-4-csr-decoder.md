---
title: '127.4 四窗CSR译码与M2验收'
type: 'feature'
created: '2026-09-21'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
baseline_commit: '91828619f66f787c63483dda4b0ee558bb7501e8'
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/127-4-静态译码与-m2-关闭.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-127-nfr14.md'
  - '{project-root}/_agile-output/test-artifacts/127-4-atdd-api-contract.md'
  - '{project-root}/_agile-output/test-artifacts/atdd-checklist-127-4-静态译码与-m2-关闭.md'
---

<frozen-after-approval reason="用户已授权逐故事完整七步；固定ATDD接口">

## Intent

**Problem:** 已有CSR叶与AXI-Lite桥缺少四窗连接，FR196/M2尚未完整验收。
**Approach:** 实现固定CsrDecoder及真实四叶组合，完成故事8条AC、ATDD全部待办，七步末关闭M2。

## Boundaries & Constraints

**Always:** 精确52端口、无捕获共享body、单session/FrozenHir；先完整16位命中再形成local地址，保留低两位；命中上下游同沿提交，锁owner到消费；miss单槽DECERR；共同同步reset。
**Never:** 不改旧bank/CSR/bridge、工具钉、包版本或IR；不实现Epic128外设、不补native层级。实现代理不改story/sprint/goal，不clean/commit/push。最终提交归主代理第七步。

## I/O & Edge-Case Matrix

| 场景 | 输入/状态 | 预期 | 错误处理 |
|---|---|---|---|
| 四窗 | 0000/0100/0200/0300各100 | 唯一叶、local偏移、同沿commit | 窗内hole/低位/权限SLVERR |
| 窗外 | 0400..ffff | 无叶请求、下一周期响应 | DECERR/读数据0 |
| 背压 | 请求/响应受阻、输入变化 | 保持payload/owner、单在途 | 消费沿不接新请求 |
| reset | 捕获/提交/owner/miss/B/R | 共同取消，恢复无重放 | reset沿不计传输 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/csr_decoder.rs`及`ip/mod.rs`：新固定ZST与重导出；复用`axi_lite_csr.rs`共享定义模式及builder位运算/寄存器，既有桥/CSR只读。builder的assign_slice/eq/sub与mux足够；Clock/Reset先于寄存器声明，body不begin/end模块。
- `crates/bitloom/tests/fr196_csr_decoder{,_formal,_integration}.rs`：9个ATDD入口；激活并补齐清单，辅助可拆同名目录；oracle按公开握手，不复制状态机。
- `docs/ip/csr-decoder.md`、`docs/ip/README.md`、`docs/public-api-1-0-surface.md`：中文契约、真实prelude-only例及逐符号FR142/minor。
- `scripts/check_fr196_decoder_example.py`：复用`check_fr194_example.check_example`；`.github/workflows/ci.yml`增专用formal、示例及失败产物，保留所有门禁。
- `_agile-output/implementation-artifacts/epic-127-closeout.md`：先写证据映射草稿；最后七步验收后由root更新M2及当前合同状态。

## Tasks & Acceptance

**Execution:**
- [x] `ip/csr_decoder.rs`及mod：固定API、52端口、命中/owner/miss/reset与共享入口，满足I/O矩阵。
- [x] `fr196_csr_decoder*`：两native、全65536地址、真实RTL/双实例/诊断、真实四叶组合、16seed×1000及并发、全部WSTRB/取消/快照/副作用。
- [x] `fr196_csr_decoder_formal.rs`：独立完整owner/response prove与cover，mutant敏感性；原始decoder/组合综合和桥5寄存边界。
- [x] 文档/API/脚本/CI：软件产物独立黄金与C消费者、可编译例、支持/气泡/reset边界；关闭映射草稿。
- [x] `_agile-output/test-artifacts/127-4-build-*`：实际命令、工具、结果/失败、种子/时间和target外源码/工具SHA归档。

**Acceptance Criteria:**
- Given 固定接口，when 定向/随机/背压/reset，then 故事AC1–5完整成立，不以9个初始骨架缩减范围。
- Given 因果合法peer，when 安全证明，then 无ready公平性/叶等待上界，失败写rdata不限，cover与prove分列。
- Given 真实RTL与软件产物，when 兼容/编译/综合，then AC6–7成立；AC8仅七步末核验关闭。

## Implementation Notes

Step01–03：有效epic127缓存与前127.3 done spec已读，主代理/只读代理调查完成。完整七步授权覆盖常规checkpoint及本故事已有脏树；保留整个单一目标，不拆减合同。基线为本frontmatter完整HEAD，尚无行为通过声明。

## Spec Change Log

## Review Triage Log


### 构建内审第一轮（2026-09-21）

三层全部返回：blind-hunter十项；edge-case-hunter `[]`；verification-gap无缺口。第三层首次启动遇容量限制，首层完成后以全新无上下文代理启动，全部结果齐备才分诊。

| 编号 | 判定 | 证据与处理 |
|---|---|---|
| B1 | medium | scoreboard仅查WO脉冲，未消费candidate/write_mask；TX/GPIO SET/CLEAR/IRQ TEST错误payload可能漏检。patch：成功WO提交时独立核对两输出。 |
| B2 | false | 真实桥在offer时保留响应槽，`csr_rsp_ready=exec&&!rst`；decoder响应在commit后才出现，此时exec已为1。因此合法非reset层级无法产生所要求CSR响应停顿；独立decoder已测长背压及取消，真实层级B/R受阻另测。不能force不可达状态冒充产品覆盖。 |
| B3 | medium | send函数自身消耗接受沿，repeat(gap)使实际分离为gap+1；W先gap0也不同拍。patch：按实际接受沿记录间距，驱动精确0/1/7/31并断言。 |
| B4 | low | 随机阶段末仅打印窗/错误/gap/顺序，WSTRB只断言非零，读写总数混有定向。patch：在并发前输出随机阶段独立16项计数与读写数。 |
| B5 | low | 并发仅local4，虽然定向及随机串行已测各权限/错误，交织副作用错误的组合缺少覆盖。patch：并发地址加入既有WO/W1C/RO/hole/miss并保持合法独立生产者。 |
| B6 | low | native已明确ready低时leaf valid保持，formal只在valid或commit时检查路由，缺少每周期精确valid向量。patch：基于端口历史pending和完整地址加入独立向量断言及ready门控mutant。 |
| B7 | low | 所有leaf error无条件排除01限制了无效响应payload；合法传输不需要此约束。patch：分别只在leaf rsp_valid时排除保留码。 |
| B8 | low | structure接受DFF_N/DFFE_N虽实际当前网表均上升沿，可能让以后负沿转换逃过此门禁。patch：移除负沿类型并加真实结构负例。 |
| B9 | false | 文档Rust示例从base数组计算base+4后与独立字面量[4,0x104,0x204,0x304]比较，原文已编译运行；改错任一文档base会失败。C宏接口按合同只输出local，C消费者另核对各local/mask；不存在所述文档base错误逃过全部检查的结果。 |
| B10 | low | Markdown当前只重复生成相等，未独立核对本故事四个正式布局的offset/mask文本。patch：用已有手写黄金表补每行名称/offset/mask断言。 |

八个保留项各自根因独立，均为测试观察/刺激的直接修正，不改公开API或产品状态；分派原实现代理最小补丁。无intent_gap、bad_spec或defer。M2仍未关闭。

## Design Notes

无待用户决定的意图缺口，无不可逆外部动作；新增公开入口与多层验证围绕同一目标，dispatch。工作区改动均为本故事已授权create/ATDD及目标账本，main延续逐故事本地提交。上下游同沿提交不加缓存；内部CSR组合路径允许，桥五个外部握手仍寄存。rsp_valid不组合rst门控，reset沿取消不计消费。真实四叶是CSR夹具，不宣称外设算法交付。

## Verification

PATH前置`/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin`；`CARGO_PROFILE_TEST_OPT_LEVEL=1 BITLOOM_REQUIRE_RTL=1 PYTHONDONTWRITEBYTECODE=1`。

- `cargo test -p bitloom --test fr196_csr_decoder --test fr196_csr_decoder_integration --test fr196_csr_decoder_formal -- --nocapture`：普通全部通过；专用ignored不算通过。
- `cargo test -p bitloom --test fr196_csr_decoder_formal -- --ignored --nocapture`：实际prove/cover/mutant/综合PASS；缺工具/超时/UNKNOWN不算通过。
- 实跑新文档例、FR193/194/195/CSR/bridge适用兼容及既有CSR/bridge专用证明；全面clean/fmt/just test留第六步。

### Step03 主代理差异与矩阵验收

主代理完成暂存完整差异的代码/文档语义检查；归档成员独立逐字节SHA回读见[核验记录](../test-artifacts/127-4-root-build-archive-verification.json)。四窗行由两引擎65536地址扫查及真实七模块集成覆盖；窗外行由miss持久响应/边界与高位alias反例覆盖；背压行由owner长等待、双实例隔离和真实AXI独立通道压力覆盖；reset行由各阶段取消、恢复访问及接受/提交/响应/消费守恒覆盖。上述测试均实际执行，普通唯一14入口、专用4入口，未将ignored或重复运行计为额外通过。

F1停顿cover绑定同一响应、F2消费/重填负例、F3提交时刻读快照、F4随机与并发独立命中计数均已检查最终源码及通过日志。91项普通兼容、7项旧CSR/桥专用回归和4个文档例通过。完整证据见[build记录](../test-artifacts/127-4-build-evidence.md)。AC8最终七步门禁仍待后续执行；本验收只完成build实现任务，不关闭M2。归档保留任务勾选前的历史spec，后续元数据差异不冒称归档原字节。

### 内审补丁回读与部分重验（尚在等待兼容回归）

B1/B3/B4/B5/B6/B7/B8/B10的五文件补丁由主代理完整回读，未改产品逻辑或公开接口。主流程重新执行普通14入口与专用4入口均exit0；7 cover、基例和归纳PASS；含新增ready门控在内4个port-only mutant均断言FAIL/exit2，control PASS。独立核对16seed×1000随机与4096并发、每seed16种WSTRB、随机读写总数和12348条实际AW/W接受周期记录，未用请求参数代替实测。证据分别为`127-4-root-postreview-{ordinary,formal}.json`及seed/formal核验JSON。完整兼容与文档验证仍运行，未完成build步骤。

### 内审后完整规格重验完成

主流程8条命令均exit0：普通14、专用4、兼容普通91、兼容专用7，4份文档原文例均通过。兼容普通593.916秒，专用兼容37.182秒。所有命令独立日志SHA已核对，补丁没有新失败；build内部审阅问题已处置，七步中的独立code-review/automate/clean-fmt-workspace/commit仍待后续。最终产物另存`127-4-root-postreview-artifacts.tar.gz`及成员清单，不覆盖首次build归档。

### Build终态

Step05完成；按用户七步顺序覆盖技能的提前commit建议，本阶段不提交。2530成员新归档逐字节SHA回读通过；完整build审阅及修补通过，无defer。story/sprint仅移review，M2仍待独立code-review、automate、clean/fmt/justtest和单故事提交。

### Review Findings — 独立code-review（2026-09-21）

四层均正常完成；容量不足时先完成的层释放名额后启动新的无上下文层。9 patch、1 false、0 decision-needed、0 defer。既有全流程授权覆盖直接修补，保持最终automate/regression/commit之前不关闭M2。完整逐项判定见[审查记录](../test-artifacts/127-4-code-review-triage.json)。

- [x] [Review][Patch] R1 区分归纳证明与port-only BMC [crates/bitloom/tests/fr196_csr_decoder_formal.rs:156] — low；证明observer含busy/miss/owner对应assert；名称port-only让维护者误读证明依赖。仅重命名和说明，不删归纳引理。
- [x] [Review][Patch] R2 每个owner完成及停顿恢复cover [crates/bitloom/tests/fr196_csr_decoder_formal.rs:133] — low；四leaf cover仅提交；当前停顿恢复可由miss满足，无法防止某leaf响应环境意外不可达。追加逐owner可达目标。
- [x] [Review][Patch] R3 区分owner与miss取消后的完成cover [crates/bitloom/tests/fr196_csr_decoder_formal.rs:134] — low；f_reset_cancel仅标记任意pending，原cover终点为新提交；追加两类取消后新响应完成。
- [x] [Review][Patch] R4 明确合法叶响应的因果时序 [docs/ip/csr-decoder.md:32] — low；文档只明确保持，formal明确prior-commit。连接自定义leaf的用户需要知道响应仅在请求接受沿后提供并保持至消费/reset。
- [x] [Review][Patch] R6 真实组合持续竞争下轮转检查 [crates/bitloom/tests/fr196_csr_decoder/stress.sv:133] — medium；有限两类流最终排空不排除固定优先；旧桥测试验证轮转，但新七模块组合仅显式检查reset读优先。追加双类确有资格时的定向grant顺序，不对缺响应容量的类别假设资格。
- [x] [Review][Patch] R7 C消费者独立全局地址字面量 [crates/bitloom/tests/fr196_csr_decoder_integration.rs:244] — low；现有C全局式右侧由相同base+offset构造，对caller base宏无独立检验；文档Rust基址已有检查，因此非已知产品错误。最小改进为测试消费者定义base宏，手写完整absolute黄金比较，不新增产品API。
- [x] [Review][Patch] R8 Markdown权限reset与owner黄金 [crates/bitloom/tests/fr196_csr_decoder_integration.rs:209] — low；当前独立文本检查仅name/offset/mask，错误权限/owner描述会误导寄存器集成人员。给同表补独立access/reset/owner预期。
- [x] [Review][Patch] R9 W/AR接受沿monitor反例 [crates/bitloom/tests/fr196_csr_decoder_integration.rs:335] — low；实际monitor实现覆盖接受沿，但当前只有AW接受沿负例；W/AR仅停顿负例。追加两条最小真实监测器故障刺激。
- [x] [Review][Patch] R10 miss保持与reset状态突变 [crates/bitloom/tests/fr196_csr_decoder_formal.rs:257] — low；目前实际mutation只选择/地址/数据；补丢失受阻miss与reset残留pending突变，证明observer对新增状态职责敏感。

#### Rejected

- R5 false — 真实桥预留响应槽且exec期间csr_rsp_ready恒为1（reset除外），真实decoder响应只在提交后出现，此时exec为1；合法非reset层级不存在所称阻塞恢复逃检路径。独立decoder已测试及证明响应保持，不用force不可能状态冒充组合行为。

### 独立code-review完成

四层审阅全部完成；9项修补、1项false、无defer或待决。主代理逐段回读补丁，重跑受影响普通6项、专用4项及文档例均exit0；17cover、6mutant真实断言反例、256次both-eligible轮转检查核对通过。387成员修补归档与当前文件逐字节一致。结果见`127-4-root-code-review-results.json`和`127-4-root-code-review-archive-verification.json`。按用户七步授权覆盖技能的提前done建议，story/sprint保持review；剩余automate、实际clean/fmt/justtest和单故事commit后才关闭M2。
