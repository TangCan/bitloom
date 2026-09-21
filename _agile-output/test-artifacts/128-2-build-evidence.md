# Story128.2 build证据

2026-09-21；实现Timer32子集。真实七步尚待主流程code-review→automate→clean/fmt/完整workspace→单故事commit。本文件不关闭FR197整体/M3/Phase24；FR189 deferred/NFR91保持，不push/publish/改版本或工具钉。Bitloom与samitbasu/rhdl无关。

## 实现与失败修复

`ip/timer.rs`同session预定义固定CSR，再定义Timer；唯一owner为Timer CTRL/COUNT与CSR COMPARE/EVENT。候选值只由沿前值和字节mask产生，commit门控自然计数，不存在match/commit回流candidate的组合环。模32先加后比较、COMPARE0回绕、软件写优先、零有效mask不暂停、同拍EVENT set胜clear，均由独立黄金验证。

首次工具调用漏加工具PATH，真实报iverilog不可用；修正PATH后8项ATDD通过。形式注入脚本最初查找`module Timer(`而实际为`module Timer (`，在执行证明前显式失败；修正定位，没有修改DUT或性质。

Chisel实际JVM编译先后失败于Scala Int上限（4278190080/4294967295）、Reset直接位运算、UInt1直接作Mux Bool。已全文读取并最小修改真实`crates/rhdl-firrtl/src/chisel.rs`：大无符号literal用BigInt（含Mem init），Reset位运算显式转UInt，非Bool Mux selector显式asBool；保留原有小值/Bool文本与公共API。没有修补生成Scala。新增跨Int/Long及u64最大literal/Mem初始化回归；Timer三seed实际JVM RTL覆盖真实失败路径。失败日志/Scala源码保留在原始归档。

## 真实验证

命令、UTC开始/结束、exit、环境见`128-2-build-commands.json`。最终每项exit0；日志同前缀。额外Chisel numeric回归独立json/log。例子原文仅依赖prelude，临时独立crate实际编译运行；生成C/Markdown与仓库文件逐字一致，API测试还固定独立手写关键地址并比较保存产物。C11静态断言检查所有offset/mask。

| 项目 | 实际结果与适用性 |
|---|---|
| `cargo test --locked -p bitloom --test fr197_timer --test fr197_timer_api -- --nocapture` | 8 passed，后端专用入口默认ignored不计通过；13端口、重复定义/冲突、宽向错误、真实双实例与层级明确拒绝 |
| direct RTL | 3seed，6578/6578/6579帧；每seed 468 raw matches、4 clear碰撞、16种WSTRB全达；提交=消费+取消；10组定向标签 |
| `fr197_timer -- --ignored --nocapture` | 单独1 passed；FIRRTL→固定firtool→Icarus及Chisel→Scala/JVM→固定firtool→Icarus各跑全部相同三seed/19735帧；仅Chisel端口clk/rst→clock/reset及io_映射 |
| `fr197_timer_formal -- --ignored --nocapture` | 2 passed，分别安全+cover、原始综合；不把默认ignored算通过 |
| prove | SMTBMC/Z3，depth16归纳成功（step14）；全32位状态、配置优先、事件、握手/快照/backpressure/reset一致性 |
| cover | 独立mode cover/depth24，8cover在4–6步全部到达：match、COMPARE0回绕、clear碰撞、软件抑制、CTRL保留字节不抑制、背压恢复、reset取消后恢复、连续periodic |
| 原始综合 | 无observer，Yosys hierarchy/proc/check/synth -flatten/check/stat，614cells，有DFF且无LATCH；不是PPA结论 |
| `bash scripts/chisel-numeric-check.sh` | 13 Rust矩阵入口实际通过；18个Chisel/JVM→RTL cases全部通过，含宽64、混宽、存储器、层级、复位及DAG；独立json/log记录exit0 |
| `cargo test --locked -p bitloom-firrtl` | 20 passed、0 failed，含literal回归；doc0 |
| `just semver-check` | 三个surface crate minor模式通过；工具skip检查不算额外产品PASS；实际新增API仍按minor文档登记 |
| native Interpreter/Compiled/generated | Timer standalone也含实例，全部按现有合同明确unsupported；拒绝消息实测。未另造无实例产品路径，不以此代替真实RTL |

三seed为`128219705511`、`deadbeef1282`、`73592401ffff`。分别提交2740/2713/2725，消费2735/2709/2719，reset取消5/4/6。回绕由软件设置近边界COUNT后自然运行，无缩位宽，不宣称经历2^32个周期。

形式仅假设首沿rst以及受阻请求稳定，不假设DUT输出、最终ready或任何公平性。参考状态为独立总线模型；DUT内部当前值对应关系是assert以增强归纳，不是assume。安全证明包含无限背压；cover证明存在轨迹，不作无条件活性承诺。工具身份：`128-2-build-sby-identity.json`按根预检重新核验16个入口/支持模块SHA，官方源tag对象bfc1c47eb786496fe794481ff88e75728f0529a6、commit daed0e1544fd96ee7dab843e5a891d92784c6230；不是仅版本字符串。

## 原始产物与维护边界

`128-2-build-raw.tar.gz`保存direct/API/formal/FIRRTL/Chisel原始输入、日志、RTL、波形、SMT/cover及例子产物，排除可重建JVM/Rust编译缓存。归档manifest与SHA见`128-2-build-manifest.json`；源码SHA可与后续review/automate修订区别，最终clean不会删除证据。历史失败日志另外保留`128-2-build-initial-*.log`。无板级CDC/电气/PPA结论；旧GPIO/UART等由主流程最终workspace覆盖。
