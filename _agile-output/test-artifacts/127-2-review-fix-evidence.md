# Story127.2 review fixes — separate evidence round

2026-09-21。以下是build review问题的最小修复及定向实跑，不替代主流程独立code-review、automate、最终clean/workspace与commit。本轮没有运行旧回归或修改任何story/sprint/goal状态。

## 修复与独立验收

| 问题 | 修复 | 本轮验收 |
|---|---|---|
| 不同block名字仍可产生相同完整C宏 | 保留固定拼写；每个寄存器/字段宏定义前加入`#ifdef`/明确`#error`；文档要求完整宏集合无重叠，block名不同不够 | A_B/C与A/B_C各自严格C11成功；双头两个include顺序均exit1且包含`Bitloom CSR macro collision: A_B_C_MASK`；原合法双头和重复include成功保留 |
| Markdown缺reject开关 | 追加Read reject / Write reject列 | 独立五寄存器布尔黄金，额外修改同寄存器两个开关为true校验 |
| 多字段产物缺黄金 | 补具名enable/mode/high三字段 | C/Markdown独立mask1/f0/80000000、RW/reset0/字段名与并集800000f1断言 |
| Leaf RW/W1C拒绝未验证 | 增加两种leaf write_reject启用配置及逐沿手写fixture | 两native引擎+真实RTL；拒绝/成功/零有效mask，response/error/commit/state，W1C自然event在拒绝和背压时继续、clear/set碰撞set优先，候选/mask不随reject/valid/pending变化 |
| sparse external仅native | 相同独立黄金frame同时驱动两native引擎与实际RTL | 全16WSTRB；peer保留位故意全1，read/candidate屏蔽保留位 |
| 最高地址仅elaborate | 增加低0、高0x8000、末字0xfffc各独立RW状态 | 两native+真实RTL；各成功写/读、commit/error和其他地址状态不变，无高位alias |
| 复用未证明实例状态私有 | 同一session两次define同名同配置，实际两个实例 | 两个module定义（top+leaf）与两个实例；A写/B保持，A背压时B写，同拍读独立值，A部分写不污染B，实际RTL |
| 缺reset取消cover | 新增cover-only历史flag，仅在已启动且pending+rst沿置位；后续成功新请求触发cover | 原安全assert/assume不变；真实prove/5cover PASS，取消后fresh cover在step4达到 |
| C编译超时只杀父进程 | GNU timeout建立进程组，60s/kill-after5s；每次保存command和实际exit | 全部C成功/失败命令原始`.command/.exit/.log`在工具归档；没有删旧日志 |

## 实跑（全部exit0）

环境沿用build工具路径与`CARGO_PROFILE_TEST_OPT_LEVEL=1`；工具版本未变（Rust/Cargo1.97.1、Icarus12.0、GCC13.3.0、Yosys0.33、SBY yosys-0.47、Z3 4.8.12）。精确命令/退出码见[commands](127-2-review-fix-commands.json)。

- `cargo test -p bitloom --test fr196_csr_config --test fr196_csr -- --nocapture`：[日志](127-2-review-fix-functional.log)，7行为+8配置=15 PASS，0 failed/ignored。包含原三个seed实际RTL回归及同session peer、新增fixture；其中leaf-reject初轮producer刺激后来修正，最终该项验收以以下独立重跑为准，其余14项未改。层级unsupported的捕获panic依然是预期负例。
- `cargo test -p bitloom --test fr196_csr_formal -- --ignored --nocapture`：[日志](127-2-review-fix-formal.log)，2实际工具测试PASS；depth16基例与k-induction成功；depth24五cover全部达到（成功/错误/W1C冲突step2，背压恢复/取消后fresh step4）；原始无observer综合/check PASS。未添加DUT正确性假设，无ready公平性假设，无活性证明宣称。
- `cargo test -p bitloom --test fr196_csr p0_leaf_owned_rejection_events_and_candidates_native_and_real_rtl -- --exact --nocapture`：[最终协议日志](127-2-review-fix-leaf-protocol-final.log)，exit0，1 PASS；两native引擎+真实RTL。初轮stall frame曾valid1而ready0，随后撤回未接受的新请求，不应作为协议合法验收；改为valid0（上一请求已提交，不另发请求），注释只说明reject切换与自然事件。保持valid的合法场景由原随机trace验收。只改此fixture并定向重跑；初轮源码在`127-2-review-fix-history/fr196_csr-before-protocol-correction.rs`，原日志和原testbench不删除。
- `python3 scripts/check_fr196_example.py`：[日志](127-2-review-fix-example.log)，修订后文档原文prelude-only编译运行PASS。
- 仅编辑Rust文件的`rustfmt --check`及`git diff --check`均exit0。没有扩大到workspace/旧M0/M1回归。

## 持久快照、原始工具产物与hash

**历史不可改写：** 编辑前逐项验证旧manifest中的源文件摘要，再保存[旧源码tar](127-2-build-source-snapshot.tar.gz)。tar member使用原仓库路径。原build证据正文、旧清单及当时验证日志在[build-history](127-2-build-history/)。原build logs/tool archive保持不变；`127-2-build-sha256.txt`仍是旧时点，不声称对今天的live-source有效。旧摘要复核读取旧tar member、history原正文和未变更的原日志。本轮未覆盖旧结果，build-evidence仅加历史解释和本轮链接。

**本轮源码：** [review-fix-snapshot](127-2-review-fix-snapshot/)保存本轮测试时的源码/文档/CI原文，使用内部原仓库相对路径；新manifest引用这些持久snapshot文件，不指向会继续被主流程编辑的live源文件。

**本轮真实工具产物：** [review-fix-tool-artifacts.tar.gz](127-2-review-fix-tool-artifacts.tar.gz)，SHA256 `5d52db668833ba1e39bebbd6c4c260b47202cd0334550ac6fbeba66b6b4051e2`，2,077,526 bytes。仅收本轮PID：CSR963107、C965473、formal963306。包含原始RTL、完整testbench、C头/消费者与显式碰撞错误、command/exit/log、formal observer/sby/status/cover traces、原始综合netlist。省略可重建simulation/.o、重复formal model/src中间文件；源及完整日志足以重建。没有以旧工具产物冒充新轮次。该archive中的leaf-reject-963107属于协议刺激修正前的历史；最终leaf-reject-978444见[补充工具归档](127-2-review-fix-leaf-protocol-final.tar.gz)，SHA256 `5461f073ecb45826eea2216c14415c8af979335e89e201eb0421538d919185b3`。最终源码snapshot已更新为修正后的fixture。

所有新证据、旧源码tar与本轮持久snapshot摘要在[review-fix-sha256](127-2-review-fix-sha256.txt)，实际校验见[verification](127-2-review-fix-sha256-verification.log)。新旧保留行为不意味着发布、FR196/M2关闭、Phase24交付或FR189恢复。
