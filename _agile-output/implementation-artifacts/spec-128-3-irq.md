---
title: '五路可组合事件 IRQ'
type: feature
created: '2026-09-21'
status: done
route: dispatch
baseline_commit: '9d2aff720025d5b084ca746de4b1c9938757b350'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/128-3-事件-irq.md'
  - '{project-root}/_agile-output/test-artifacts/atdd-checklist-128-3-事件-irq.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-128-nfr14.md'
---

<frozen-after-approval reason="用户已批准全部故事七步连续执行">

## Intent

交付128.3五路事件IRQ，供后续GPIO/UART及系统复用。完整合同为故事七AC及ATDD固定接口，基座CSR本身不代表IRQ交付。

## Boundaries & Constraints

固定 `ip::Irq`、14端口、四寄存器/五具名字段见ATDD。单session/一次finish；复用CSR唯一存储owner，prelude-only；保持工具钉及旧API。无第六源、无新native层级。仅关闭IRQ子集，FR197/M3仍开放、FR189 deferred不变；不发布/升版本，第七步才commit。

## I/O & Edge-Case Matrix

|场景|预期|
|---|---|
|原始五路事件|每沿锁存，mask不丢；重复不计数；irq为pending与enable归约|
|PENDING/ENABLE/TEST/RAW|local0/4/8/c，mask31，W1C/RW/WO/RO；TEST按成功提交所选位set；RAW只硬件|
|同沿竞争|硬件set胜clear；TEST与clear双写不可达；PENDING读旧值、RAW取当前输入|
|背压/reset|锁存响应、无refill；事件继续接收；同步reset清状态并取消在途|
|错误/字节|全部WSTRB，零有效mask合法无软件副作用；权限错误优先、洞/高地址不别名|
|Timer接线|EVENT仍高时清IRQ不重触发；新match重新置位|

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/csr/{mod,rtl}.rs`：复用描述、WO提交及W1C；WO无value、RO无candidate/mask，不改基座。
- `crates/bitloom-prelude/src/ip/timer.rs`：复用模块定义模式及真实match输入，保持旧行为。
- `crates/bitloom/tests/fr197_irq{,_api}.rs`：10入口，2目标E0432已实测；独立oracle与实际双实例/Timer图。
- `_agile-output/test-artifacts/128-2-build-runner.py`、`fr197_timer_formal.rs`：复用安装身份、三后端、独立ghost及负控制组织；不改历史证据。

## Tasks & Acceptance

- [x] `crates/bitloom-prelude/src/ip/irq.rs`及`ip/mod.rs`：实现固定接口与唯一CSR存储。
- [x] `crates/bitloom/tests/fr197_irq{,_api,_formal}.rs`：真实GREEN、三后端同黄金、prove/cover/原始综合及observer负控制。
- [x] `docs/ip/irq*`、`docs/public-api-1-0-surface.md`、`docs/ip/phase24-contract.md`：原文例、同源C/Markdown、FR142逐符号/minor及真实范围。
- [x] `.github/workflows/ci.yml`、`_agile-output/test-artifacts/128-3-build-*`：持续门禁、可移植runner和命令/原始证据归档。

Given七AC，When执行各门禁，Then独立参考、全部可达边界和三种真实RTL通过；native层级明确拒绝。Given任意合法背压，When形式验证，Then安全性无ready公平性假设；cover与综合独立记录。完整回归及独立审查由主代理完成。

## Implementation Notes

无意图缺口/不可逆操作；新增硬件/API/验证面，采用dispatch。当前脏文件均为本故事create/ATDD，持续授权覆盖常规checkpoint。RAW内部值软件混入若只发生TEST提交拍，单口读不可观察；结构检查及独立raw_value assert补足，不虚报不可达测试。

## Spec Change Log

## Review Triage Log

## Verification

`cargo test --locked -p bitloom --test fr197_irq --test fr197_irq_api`；专用后端、formal prove/cover/负控制、综合、prelude-only例/C头、`just semver-check`。runner按调用者PATH、RHDL_FIRTOOL_PATH目录、BITLOOM_SBY_SOURCE重核16安装文件，不硬编码机器路径；失败和ignored不记PASS。原始target证据在clean前归档校验。最后实际cargo clean→cargo fmt --all→just test，单故事commit。


本轮只读后端调查确认：全向量scoreboard当前仅direct，build须补独立FIRRTL/Chisel完整运行，不以组合短测试替代。CI保留Timer门禁并增加IRQ普通示例/C头、chisel-numeric完整向量+三图组合、formal-sby身份核验与专用证明/综合/负控制；失败工件覆盖所有新目录。formal对应pending/enable/raw_value必须assert，至少三种代表变异clear优先/TEST未门控/RAW错误来源须预期断言FAIL及VCD。

本轮可用调用环境：PATH前置 /tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:/tmp/bitloom-jvm-tools/bin；BITLOOM_SBY_SOURCE=/tmp/bitloom-1263-sby-src；RHDL_FIRTOOL_PATH=/home/richard/.cache/rhdl/firtool/1.159.0/bin；CARGO_PROFILE_TEST_OPT_LEVEL=1、BITLOOM_REQUIRE_RTL=1。这些仅本机执行输入，不写死可移植runner。实现代理不clean、不commit、不改sprint/总账；主代理负责后续独立review/automate/全回归及状态。保存每次命令、UTC、exit、环境/源码SHA，归档原始证据。

实施发现（2026-09-21）：真实IRQ Chisel JVM因既有Concat发射裸`Cat`、仅import chisel3._而失败（原始sbt.log已保留）。必要最小修复`crates/rhdl-firrtl/src/chisel.rs`将Concat限定为`chisel3.util.Cat`，补机械/风格emit回归并重跑真实IRQ全向量、FIRRTL及既有numeric门；不手改生成Scala、不改工具钉、不扩大多位Mux deferred范围。ATDD Timer组合夹具`matches`为SV关键字，改局部变量`match_count`；合并seed仅各suite首reset帧不比较沿前旧状态，所有reset沿后与其它帧断言保持。


主代理实施验收：完整diff记录/tmp/128-3-build-full-8n3m9_wh.diff；全文阅读产品/参考/API/形式/runner/文档、CI与emitter变更，并对ATDD原文格式化后逐diff审阅增量，黄金语义未削弱。已逐member比对归档1516文件与当前target原件、16源码SHA，全部相符（128-3-root-build-audit.json）。矩阵各行均有实际direct/后端/形式或组合通过记录；未使用ignored充当PASS。构建任务完成，开始build内部三路独立review，尚未完成七步。


### Build review逐项裁定

|ID|发现|裁定/路由|核实依据|
|---|---|---|---|
|B1|archive无输入也成功|medium / patch|脚本members空时仍写tar、集合/hash比较空集均通过，可能误报clean前保护|
|B2|basename排除run可能排掉未来证据|low / reject|当前18个run均以vvp解释器头开始，都是明确可再生可执行；其他名称来自同一已知runner。没有当前证据丢失。为未出现的任意文件加分类机制超出直接修正，日常故障未示出|
|B3|启动异常不记commands|medium / patch|subprocess.run抛OSError先于records.append，真实缺工具会只有空/旧记录|
|B4|例子产物比较失败不记commands|medium / patch|example命令成功写入后require比较，可使整个gate失败却JSON全成功，需独立比较步骤|
|B5|SBY包/bytecode可绕过字节身份|medium / patch|根复制安装添加sby_core包，16文件核验通过而实际sby导入未检查包并报自定义标记；128-3-build-review-shadow-repro.json。需隔离缓存及拒绝未核验导入路径，保留真实安装不动|
|B6|直接formal测试命令绕过身份门|low / patch|CI/runner正确，但文档直接test入口未区分未检查调用；文档指向required runner并说明直接入口边界|
|B7|Yosys/Z3版本只记录不强制|low / patch|实际固定本机Yosys0.33/Z34.8.12已记录；产品钉为Rust/firtool/Chisel与SBY源，不新增升级。文档明确host版本是记录策略而不是runner锁定承诺|
|B8|重复后端helper|low / reject|当前两路径同正确命令/端口适配，无已发生divergence；提出抽共享框架是非平凡维护重构，不修当前用户缺陷|
|B9|其它Chisel面没有真实JVM|false / reject|新增单测明示仅名字拼写防回归，文档明确字符串不等于行为；实际机械JVM独立通过，三面共用同emit_expr全限定API，未声称所有面执行JVM|
|B10|故事../../docs链接坏，应../../../|false / reject|根Path.resolve实测../../docs正是项目/docs且存在；审查建议多退一层会出仓库。原路径正确|
|E1|空archive通过|medium / patch（与B1同因）|同B1，逐项保留后合并处理|


三路报告与核实见128-3-build-review.md；B1/E1合并，其余patch各自落实。无意图缺口或规格重写。


主代理修补后复验（2026-09-21）：新runner全部命令exit0，活动ATDD9项、formal3项（23 cover及3预期反例）、全向量与三组合后端、FIRRTL21项、numeric实际JVM、SemVer、原文例及C头通过。完整命令/源码摘要见128-3-root-review-recheck.json；新归档128-3-review-fix-raw.tar.gz，2228成员逐字节对照原target再次通过，SHA256 989b72dd2d932ebc0eccce8d69b151d5e71ef1c18a6dd9eb16de90145c419557。B1/B3/B4/B5/B6/B7及E1修补关闭；历史日志/归档不改。第六步clean/fmt/全workspace仍按七步顺序随后执行，不以本轮定向复验替代。


### Review Findings

- [x] [Review][Patch] B1 归档遗漏example-artifacts — gates集合不含新增比较gate；example退出0而比较失败仍满足集合。
- [x] [Review][Patch] B2 历史PASS可替代当前源码结果 — passed跨所有commands.json并集，没有选定run或源码匹配，改变产品后旧PASS仍有效。
- [x] [Review][Patch] B3 formal状态只检查非空 — require_paths仅检查size，ERROR/UNKNOWN与负控制PASS都可满足该层检查；须关联本轮并核对期望。
- [x] [Review][Patch] B4 自定义runner目录被归档漏掉 — runner接受BITLOOM_IRQ_RERUN_DIR任意不存在路径；archive仅遍历五个固定target根，实际支持路径可能丢失。
- [x] [Review][Patch] B5 孤立manifest被覆盖 — 只保护archive.exists，最终manifest.write_text直接覆盖；直接补对两个目标的拒绝检查。
- [x] [Review][Patch] B6 TEST字节矩阵被全1状态遮蔽 — 三层循环每次先TEST31，addr8时pending已经31，selected-byte不作用也不可见；应空/混合pending分别起步。
- [x] [Review][Patch] B7 harness回归未接CI — CI三个IRQ入口均未执行128-3-build-harness-tests.py；身份和证据错误场景不会受持续回归保护。
- [x] [Review][Patch] B9 精确过滤零测试也成功 — runner仅依cargo退出0；Cargo过滤无匹配时合法返回0，缺少目标执行数检查。
- [x] [Review][Patch] B10 运行期间改源码不被识别 — source-sha256仅在gate之前写，没有结束比较，不能确认长门禁期间被编译的相关源码保持相同。

Rejected：B8 false — 非零low与零high可检出位序反转；一般非零high值域不由IRQ门禁声称覆盖。E1/E2逐项裁定与同因分组见128-3-code-review.md。


修补完成复核：9组patch全部落实并经主代理审阅。direct扩展为97,738帧，普通/-O工具回归各12项通过；新默认完整runner 20260921T103117.009451Z-237066全命令PASS，活动9项、formal3项、完整两后端及三组合、FIRRTL21项、numeric/semver/example/header全部通过，两exact目标各1次真实PASS。起止和当前566源文件SHA一致。原始归档128-3-code-review-raw.tar.gz由root独立逐字节复核3681成员，SHA256 7ef194ce9e68a300cd4ad42475f26349a9eeae051a108adbe9761630247ded08；详见128-3-root-code-review-recheck.json。代码审查阶段完成，0 decision-needed、9 fixed、0 defer、1 rejected。用户七步顺序优先：Story/sprint在automate及clean/fmt/workspace/commit之前仍review，不提前done。
