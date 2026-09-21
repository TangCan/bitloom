# Story127.3 build 证据（2026-09-21）

本阶段实现 FR196 的固定 AXI4-Lite→CSR 桥子集；不关闭127.4、M2、FR196整体或Phase24，不修改故事/sprint/goal，不clean/commit/push/publish。后续独立review、automate及完整clean/fmt/regression由主流程执行。Bitloom与samitbasu/rhdl无关。

## 产品实现与不变量

`ip::AxiLiteCsrBridge`独立elaborate与`define_module`共享同一个非捕获body，固定addr16/data32/WSTRB4、PROT输入存在但忽略。无新HIR或运行时closure。相同定义可复用，真实RTL的双实例测试验证独立状态。

AW/W/AR各一捕获槽，保留到CSR提交；capture消费沿不refill。空闲时仲裁后寄存offer，最早capture后第二个上升沿提交。offer/exec互斥隐式预留对应空B/R槽：从选定到完成期间没有别的执行能填入该槽。提交记录owner并切换优先级（reset后读优先）；CSR响应消费释放exec并填B/R槽；后续选择发生在之后的沿，不做响应/请求同沿交接。B/R各自消费后释放，不同类可独立前进。

选中地址捕获在提交前不会变化；额外offer_data/offer_strb快照保证**读**offer受阻时后来W捕获也不能改变CSR无关字段。五外部ready/valid仅连寄存占用状态，CSR req_valid/rsp_ready可按共同同步rst屏蔽。所有寄存器由既有builder同步reset优先清零；系统必须同步共同复位桥及所有leaf，不支持bridge-only reset。

## 实际执行

| 执行 | 实际结果 / 记录 |
|---|---|
| 两个桥测试目标普通入口 | exit0；9 passed、0 failed，2专用ignored不算形式通过；行为runner291.15s；`127-3-build-attempt-second.log`、`127-3-build-behavior.json` |
| 16固定seed×1000事务 | 全部完成；7955 AW/W/写/B，8045 AR/读/R，共16000提交与16000CSR消费；每seed gap/strobe/error命中与完整ledger在JSON和原始日志；seed墙钟为累计计时 |
| 原始通道定向 | Interpreter与Compiled各实际RTL回放；0/1/7/31偏斜、16WSTRB、8PROT、原始地址、00/10/11、offer锁定、轮转、独立响应槽及reset矩阵通过 |
| 真实CSR层级 | 102提交/102消费，58B/44R，CW14/TW8/EW1；黄金权限/字节/动态拒绝/快照/副作用/非法地址通过；双桥独立状态通过 |
| 层级unsupported补验 | Interpreter/Compiled/GeneratedFunctional均明确拒绝；随后同一真实层级RTL通过，exit0/0.11s；`127-3-build-attempt-hierarchy.log`。日志中的三次panic是被捕获且核对诊断的预期负例 |
| 最终专用formal/综合 | exit0；2 passed、0 failed、0 ignored；实测命令30.7623s（Rust runner29.64s）；`127-3-build-formal.{log,json}` |
| prelude-only文档例 | root实际执行exit0，3模块，compile4.45s；`127-3-build-example.{log,json}` |
| 既有FR193/194/195/196兼容 | exit0，78 passed/0 failed/0 ignored，6目标；实测206.4648s；`127-3-build-compatibility.{log,json}` |

普通入口精确命令、最终formal和兼容环境见各JSON；PATH前置`/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin`，`CARGO_PROFILE_TEST_OPT_LEVEL=1`、`BITLOOM_REQUIRE_RTL=1`。最终formal及兼容额外设置`PYTHONDONTWRITEBYTECODE=1`。普通行为记录的291.15s来自Rust runner，未伪造另一个shell测量；每seed elapsed为累计时间。普通运行期间只增加注释/格式以及随后独立运行的层级拒绝断言，产品行为未改。最终普通结构检查和专用formal使用最终测试源码。

## 形式、结构与工具

真实SBY/Yosys/Z3运行：**prove depth8 basecase+temporal induction均PASS**（无界安全证明，不是8拍BMC替代），**cover depth64 PASS**，五cover实际在6/8/8/9/10步命中：读写竞争完成、B受阻期间新读提交后B恢复、R受阻期间新写提交后R恢复、reset取消后新提交、B/R同时受阻。无BREADY/RREADY公平性约束。CSR peer是因果单响应、合法error00/10/11、错误数据0且≤7拍响应；不宣称整体活性或所有peer延迟模型。

原始port oracle由公开握手更新独立槽与计数；新增port occupancy等式和单执行预留断言。另立`INDUCTION_LEMMAS`只以assert证明内部寄存器与该独立参考的对应，未改变oracle更新规则或原始安全断言，也未添加DUT义务假设。所有辅助不变量同时通过basecase和induction。

无observer的原始RTL实际Yosys `hierarchy -check; proc; check -assert; synth; check -assert; stat`通过。精确组合/FF白名单拒绝latch、未知单元、未知常量及缺driver；150 FF、全部5个外部ready/valid无任何输入（含rst）组合路径。结构检查器负例通过；这不是仅看波形的结论。

实际工具版本原始日志在tool archive：Icarus/vvp12.0、Yosys0.33、SBY `yosys-0.47`、Z3 4.8.12；Rust/Cargo详细版本另存`127-3-build-rust-version.log`（Rust1.97.1）。工具/产品pin未改。

## 失败与恢复（没有抹去失败）

- 首个准备命令误用本机不存在的`python`，生成未执行，后续cargo仍报缺API，exit101；改用`python3`后实际生成桥。`127-3-build-attempt-first.log`保留该cargo失败，工具调用记录保留`python: command not found`。
- 原始depth32 port-only proof发生不可归纳反例（任意起点、长停顿时未约束的内部保留数据与参考脱离），basecase未报reachable错误，但180s超时；对应运行**失败**。
- 加port occupancy assertions仍不可归纳并超时；再加独立assert对应引理后induction成功，但depth32 basecase超过180s。三次尝试均exit101、内部timeout124，原始log/trace/config完整归档，不称PASS。
- 保留所有原始assert/assume与新增已证明引理，将prove起始depth改为8，完整basecase+induction及cover真实PASS；不是降低为有界证明。第一次成功30.60s，最终重复成功29.64s。各尝试源码的实际生成`design_formal.v`与配置可在各PID目录核验。

## 持久化与可复核性

`127-3-build-source-snapshot.tar.gz`含19成员，覆盖产品/测试、builder与CSR上下文、root文档/API/CI/示例脚本及示例日志。`127-3-build-tool-artifacts.tar.gz`含445成员，约21MB，保留全部bridge RTL/testbench、版本/命令/日志、正式证明/cover及失败轨迹、原始综合JSON。省略可由归档源码和工具命令重建的Icarus `simulation`可执行产物；没有省略失败日志或反例。既有兼容目标保留执行日志，产品相关桥运行则保留完整生成源码。

两个`*-manifest.json`均记录archive本身及逐成员SHA256，并已从压缩包回读每个成员核验；hash描述明确的archive snapshot，不声称后来live文件永远不变。保留这些文件后，主流程清理target不会丢失桥行为/形式/综合证据。源码在freeze后如因review修改，须另存新快照/验证，不覆盖本阶段历史。

## 未完成边界

本build不自行签署独立review或故事done。随机每事务排空，随机偏斜/停顿/负载；并发重叠来自定向和formal，不称随机穷举或无条件活性。形式变异测试尚未执行，仅有原始protocol monitor与结构检查负例；后续automate可补有意义变异。四窗decoder及M2不在本故事。FR189仍deferred、NFR91未清空。

## 兼容最终补记

6个既有目标依序FR193=12、FR194=5、FR195 ParamSyncFifo=31、FR195 RvRegSlice=9、FR196 CSR=12、FR196 config=9，共78 passed、0 failed、0 ignored；cargo exit0，实测206.4648s。FR193自身16seed×1000写/1004读及定向黄金序列全部真实通过。root另独立回读校验全部464个源码/工具archive成员，见`127-3-build-root-archive-verification.json`。无仍运行的build测试进程；本阶段不宣称完整workspace回归已经执行。
