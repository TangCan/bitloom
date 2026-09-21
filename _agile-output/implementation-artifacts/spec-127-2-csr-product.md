---
title: '127.2 静态 CSR 描述与可组合叶节点'
type: 'feature'
created: '2026-09-21'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
baseline_commit: '77ea01392e5be74b9f2622009ad146edb2468ea9'
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/127-2-csr-描述-rtl-与软件地址产物.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-127-nfr14.md'
  - '{project-root}/_agile-output/test-artifacts/127-2-atdd-api-contract.md'
  - '{project-root}/_agile-output/test-artifacts/atdd-checklist-127-2-csr-描述-rtl-与软件地址产物.md'
---

<frozen-after-approval reason="用户已授权全部故事七步串行，API已在ATDD确定">

## Intent

**Problem:** 新CSR叶节点与软件地址产物尚不存在，后续外设无法按统一提交协议组合。
**Approach:** 实现上述API契约全部类型/方法/端口，静态描述同源生成RTL、Markdown、C头；完成故事8条AC。

## Boundaries & Constraints

**Always:** 单session/FrozenHir、非捕获fn、完整配置身份；单响应槽、提交沿副作用、读快照、reset优先；外部owner没有重复存储；全部16WSTRB、动态拒绝与set优先准确。
**Never:** 不实现桥/四窗decoder/外设产品，不改旧IP/工具钉/版本，不新增IR或native层级。实现代理不改story/sprint/goal状态，不clean、commit、push。主代理完成后续七步。

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/mod.rs`：加csr私有模块及重导出；新`ip/csr.rs`或`ip/csr/`实现配置、验证、codec和共享body。
- `crates/bitloom-builder/src/lib.rs`：复用define_module、slice/concat/zero_extend/位运算；helper错误poison，预校验失败可恢复；callback不能begin/end，Span稳定。无需改builder。
- `crates/bitloom/tests/fr196_csr{,_config,_formal}.rs`：既有ATDD12测试，保持独立oracle；仅能凭具体证据修测试缺陷，不能削弱预期。
- `docs/ip/csr.md`、`docs/ip/README.md`、`docs/public-api-1-0-surface.md`：完整prelude-only例、使用限制、逐符号FR142/minor；公开文档保持与samitbasu/rhdl无关声明。
- `.github/workflows/ci.yml`：先完整读，扩既有formal-sby job以真实运行FR196 --ignored并上传失败产物。默认workspace无需新增SBY依赖。

## Tasks & Acceptance

**Execution:**
- [x] `ip/csr*`：实现固定签名、全部标识符/生成名/offset/mask/reset/owner/event/reject合法性与规范排序，三种产物一致；codec私有畸形schema/额外重复缺键/count/索引/枚举/byte单测。
- [x] `ip/csr*`：共享组合叶；候选值与mask不依赖commit/reject；同沿pulse，RO/external保留位屏蔽，W1C自然事件不被拒绝或背压抑制。
- [x] `docs/ip/csr.md`及索引/API表：可编译例与准确范围；添加定向示例校验入口（可复用scripts/check_fr194_example.py模式）。不更新Phase24关闭宣称。
- [x] `.github/workflows/ci.yml`及test-artifacts/127-2-build-*：真实12测试及私有codec单测，C11/原始RTL综合、prove与cover均成功，原始命令/版本/退出码/失败与最终证据归档target外并hash；旧AXI/M1定向回归。

**Acceptance Criteria:**
- Given 不合法配置，when validate/emit/define，then Diagnostics一致，预校验不留下半模块；完整合法描述重排后产物和复用一致。
- Given 请求与peer状态，when clock提交/停顿/reset，then 独立native两引擎与真实RTL的沿前/沿后值、计数全部符合固定契约；层级peer只用实际RTL。
- Given 同源配置，when 生成C/Markdown/RTL，then 独立黄金和单/双头C11编译通过，无地址别名/宏碰撞。
- Given 固定工具，when 实跑formal prove/cover及无observer综合，then 各自退出成功并记录假设/深度/局限，ignored不计通过。

## Implementation Notes

实现、API与文档按固定契约完成。主代理只读核验定义体、codec、状态更新、端口与文档；10项功能/配置、3项私有codec、2项真实formal/综合、62项旧M0/M1回归与文档例通过。formal首次失败为observer少一个右花括号，仅修语法；首次RTL失败为工具PATH缺失，均保留日志，未降低性质。工具归档及25项摘要由主代理复核通过，证据见[build记录](../test-artifacts/127-2-build-evidence.md)。故事AC8的后续独立review/automate/完整回归/commit仍由七步主流程完成，当前不宣称故事done。

## Spec Change Log

## Review Triage Log

2026-09-21 build三路均返回后统一裁决；blind 10、edge 1、verification 1。固定公开宏拼写不变，跨块集成必须选择不重叠的完整宏集合；不以改API掩盖冲突。

| ID | verdict | evidence / route |
|---|---|---|
| B1 | medium | A_B/C与A/B_C确实拼成相同宏；单块validator看不到其他块。patch：生成头对已定义宏显式#error，文档明确完整宏集须不相交，增加碰撞失败/合法双头测试；保留固定宏格式。 |
| B2 | low | Markdown未记录reject开关，动态错误行为不同的配置表相同。patch：加入read/write reject列。 |
| B3 | medium | W1C行为bank未开reject，codec不是行为验证。patch：与V1合并增加被拒清除和自然事件同拍测试。 |
| B4 | medium | sparse external RW仅native，真实RTLpeer mask全1。patch：同一独立黄金场景通过Icarus。 |
| B5 | medium | 0xfffc仅elaborate正例、行为负例；高地址合法解码尚无行为断言。patch：加入最高地址及相隔高位的实际读写。 |
| B6 | medium | 多field只有确定性测试，可能稳定输出错误字段常量。patch：独立多字段C/Markdown黄金。 |
| B7 | low | 真实组合只有一个CSR实例；define复用已验证但状态隔离未实测。patch：最小双实例RTL独立写读测试。 |
| B8 | low | reset安全已有oracle/形式属性，当前四cover确无取消恢复。patch：增加pending reset后新请求cover，不冒充活性证明。 |
| B9 | medium | C timeout只kill驱动，子进程可残留。patch：复用GNU timeout进程组，保存exit。 |
| B10 | low | goal表127.2前空行中断table。patch：主代理删除空行。 |
| E1 | medium | 与B1同一可达跨块宏冲突，合并相同修复。 |
| V1 | medium | 预核验的regression gap：leaf RW/W1C write_reject未行为覆盖。patch：开启两类reject并验证拒绝/成功/零mask及W1C事件。 |
| M1 | medium | 主代理复核新directed刺激：pending时valid=1后无reset撤回，违反producer稳定。patch：该stall帧valid=0，保留reject变化与事件累积；持续valid由既有合法trace覆盖，定向实跑重验。 |


## Verification

PATH前置`/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin`，`CARGO_PROFILE_TEST_OPT_LEVEL=1`。缺工具是失败，先按已有固定安装规则恢复。

- `cargo test -p bitloom --test fr196_csr_config --test fr196_csr -- --nocapture`：10测试及真实Icarus/C11通过。
- `cargo test -p bitloom --test fr196_csr_formal -- --ignored --nocapture`：2项实际SBY/Z3 proof/cover与Yosys原始综合通过；完整归纳结论与cover分别保存。
- `cargo test -p bitloom-prelude csr`：私有codec负例/往返通过。
- 旧M0与M1测试按现有实际测试target定向执行；prelude-only文档例编译运行。最后完整clean/fmt/just test由主代理在第6步执行。

无意图缺口或不可逆动作；多文件同一CSR目标，dispatch。ATDD已有真实E0432红（仅缺API），尚无产品行为通过。正常自选实现细节，但不得改变固定契约。

## Build完成记录

三层review全部修复，无defer。主代理复核新源码、独立黄金、合法producer修正和26项持久摘要全部OK；修补后15功能/配置、2真实formal/综合与文档例通过，唯一后续修改fixture定向重跑1项通过。详见[修补证据](../test-artifacts/127-2-review-fix-evidence.md)。Spec done仅表示build完成，story仍review；用户七步顺序覆盖技能内部提前commit，最终clean/fmt/just test后才提交。on_complete为空。

### 独立 Review Findings

完整11项逐条裁决、证据与route见[独立审查](../test-artifacts/127-2-independent-review.md)，行动状态记录在[story](127-2-csr-描述-rtl-与软件地址产物.md)的Review Findings。R8/R9主代理已修复，其余patch由实施代理处理；不提前将story done。

## 最终七步状态

独立review的10项patch全部处理、automate28项定向通过，最终clean/fmt/just test 1787 passed/0 failed/18 ignored；4个CSR专用入口另有实际PASS，不把ignored当通过。最终命令、限制、日志与归档见[最终验证](../test-artifacts/127-2-final-verification.md)。仅127.2随本提交关闭，Epic127/M2保持未关闭。
