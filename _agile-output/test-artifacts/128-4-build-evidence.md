# Story128.4 GPIO32 build evidence

2026-09-21。实现`ip::GpioCsr`固定16端口与六local寄存器，单session共享CsrBlock；唯一OUT、双同步级/history、沿前DIR、新沿raw与sticky分离。旧GPIO8/FL、CSR基座与所有后端未改。逐符号FR142/minor、软件原文例/同源C头与Markdown、CI及可移植证据工具已加入。Bitloom与samitbasu/rhdl无关。

选定一次完整run：`target/fr197-gpio-reruns/20260921T114105.437769Z-671840`，主命令日志`128-4-build-full-run.log`。每命令UTC/参数/退出码/耗时在run的commands.json；关联raw目录绑定到对应gate，不并集历史PASS。全部gate退出0，source_fingerprint_complete=true；起止源码逐项SHA一致，归档再核对当前源码。原始commands/fingerprint/SBY身份/软件比较JSON同时复制到本目录以便快速审阅。

|层次|已执行结果|
|---|---|
|direct GPIO RTL|三个seed共55151帧PASS；seed128419705511/deadbeef1284/83592401ffff，18384/18383/18384帧；accepted6801/6782/6798、consumed6789/6770/6786、cancelled各12|
|API/直接组合|9活动测试PASS：16端口/描述/同源软件黄金/诊断/native拒绝/共享及重命名双实例/真实GPIO→IRQ/旧GPIO两native与FL；专用JVM在普通目标ignored单列|
|FIRRTL与Chisel GPIO|同55151帧独立参考分别实际运行PASS；exact目标执行1次|
|FIRRTL与Chisel三组合|共享双实例、重命名双实例和GPIO→IRQ均实际RTL PASS；exact目标执行1次|
|formal safety|独立ghost状态，SBY prove basecase+induction PASS；无ready公平性假设，状态对应均assert|
|formal cover|10个cover全部到达PASS，原始轨迹保留；bit31、双向改DIR、同位/异位clear-rise、至少五拍stall新沿、reset取消、IN/事件快照和初始高|
|DUT变异负控制|同observer原始control prove PASS；IN误接OUT→gpio_input、clear胜set→gpio_events、SET忽略WSTRB→gpio_out均SBY FAIL(exit2)，各有指定断言与VCD；ERROR/UNKNOWN/timeout不计|
|原始综合|无observer RTL Yosys synth/check PASS，GpioCsr flattened 990 cells，有DFF、无LATCH，check -assert无多驱动|
|证据工具|普通Python与-O各12场景PASS；源码/选定run绑定、exact0匹配/ignored拒绝、launch失败、软件差异、归档保护、SBY包/字节码影子与隔离缓存|

工具实际：SBY yosys-0.47（官方tag对象bfc1c47eb786496fe794481ff88e75728f0529a6，commit daed0e1544fd96ee7dab843e5a891d92784c6230，安装16文件逐字节校验），隔离Python、全新缓存前缀与无字节码写入；Yosys0.33 git2584903a060、Z3 4.8.12按实录非新锁。产品Rust1.97.1、firtool1.159.0、Chisel7.15.0/Scala2.13.18/sbt1.10.11保持。runner依调用者PATH/RHDL_FIRTOOL_PATH/BITLOOM_SBY_SOURCE，不硬编码机器安装目录。

初次失败日志完整保留，说明见128-4-build-implementation-notes.md。缺软件产物及变异预期标签调整不能冒称产品原始行为失败。最终验收只取上述完整run。所有生成日志保持原字节，不为whitespace整理修改。

native Interpreter/Compiled及GeneratedFunctional对含CSR实例的完整GPIO/双实例/IRQ图均明确unsupported，测试核对拒绝诊断；没有独立无实例内核，因此不扩产品路径去造native PASS。旧GPIO8由已有native/手写FL回归另证。逻辑同步与综合不提供去抖、原子多位采样、MTBF、pad/电气/板级签核或PPA保证。

本次只实现FR197 GPIO子集。UART128.5、M3/FR197整体、Epic129–130与Phase24未关闭，FR189 deferred/NFR91保持。独立review/automate、最终cargo clean/fmt/just test与一故事一提交由主代理后续执行；不push/publish，未改工具钉或版本。CI入口已加入但没有声称远端CI实际运行。


## 完整run与补充回归结果

全run的atdd/formal/backends/pair-backends/firrtl-regression/numeric/semver/example/example-artifacts/header-compile/header-execute均PASS。FIRRTL库21 tests，numeric Rust matrix13 tests加18套实际Chisel RTL，均0失败；SemVer minor检查prelude/sim/firrtl三包，每包196 checks pass、58 skip（skip明确单列），未改版本。原文例实际编译/展开并生成两软件产物，与仓库逐字节一致；C11消费者编译及执行成功。CI YAML读取成功（17jobs），远端未运行。

补充旧GPIO/VIP/手写FL/SocPad/ChipPadRing/FR139分拆六目标：32 passed / 0 failed / 0 ignored，命令/UTC/耗时/退出在128-4-build-compatibility.json及两原始日志。此补充不代替完整run，也不改变相关源码；旧实现保持。rustc实际版本见compatibility-0.log。harness普通/-O各12通过的日志为128-4-build-harness{,-opt}.log。

## clean前持久归档

`128-4-build-raw.tar.gz`及`128-4-build-raw.manifest.json`已创建且1589成员逐字节SHA256校验成功；archive自身SHA256为`5ab1016a48551e6334ce1d5bf9c71a1c20bd4359e32d4e56d7f4fe2a4f16a9e2`。使用`--include-history`保留首次形式负控制标签失败与初次direct/API运行工件，历史只作诊断、不用于选定run验收。tar约31MiB，原始RTL/Scala/testbench/波形/证明/反例/综合/命令日志均保留；编译缓存和可执行仿真二进制按manifest排除。归档校验前当前相关源码等于完整run起止指纹，不覆盖既有tar/manifest。其余初次编译失败及补充兼容/harness日志已持久保存在本目录，clean不会删除。

## 构建内审后的第二次完整验证（历史阶段）

内审7项修补后完整run为`target/fr197-gpio-reruns/20260921T115845.483458Z-797187`，全部门禁通过。普通ATDD增加一个direct scoreboard负控制：原始控制PASS，IN=OUT变异在指定rdata断言失败，均有VCD；API组合增加mask保留pending和live raw期间reset。harness普通/-O各14通过。

新归档`128-4-build-review-raw.tar.gz`含678成员，SHA256 `528fd853bb7104065e77a8f48f0eb5e3e1392695b8b8004740484b0c81d39274`；主代理与原文件逐字节复核并验证566相关源码的起止/当时当前SHA一致，见`128-4-build-review-root-audit.json`。第一次归档保持原字节。此后进入独立code-review，若再修改源码，此阶段历史PASS不替代修补后的验证；最终验收另记。
