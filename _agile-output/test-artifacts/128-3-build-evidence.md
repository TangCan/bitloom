# Story128.3 构建与真实验证

2026-09-21。实施范围为spec-128-3-irq.md固定七AC；本记录只报告build，独立review/automate、实际clean/fmt/完整workspace回归及一故事一提交仍由主流程完成。未clean、未commit、未push/publish、未改工具钉/包版本或sprint/总账；GPIO/UART、FR197/M3、Phase24整体未关闭，FR189 deferred/NFR91保持。

## 实现与唯一状态

`ip::Irq`/registers/define_module/Elaboratable逐项纳入FR142/minor。固定14端口与五源，四描述共享CsrBlock；PENDING/ENABLE仅CSR leaf持有，TEST无存储、RAW无candidate/write_mask。raw_events零扩展直接驱RAW；TEST candidate只经write_commit门控后与硬件事件OR，送leaf W1C事件输入；irq归约pending&enable。旧CSR与Timer产品代码未改。

文档/原文例/C头/地址表在`docs/ip/irq*`。例仅依赖prelude，真实编译、运行与elaborate；例内Timer→IRQ图与RTL集成夹具语义等价，但不是同一份图，不声称原文图本身已三后端执行。CI保留Timer门，追加普通workspace活动IRQ测试、prelude-only例/C头、JVM+FIRRTL全向量及三图组合、固定SBY身份核验/证明/cover/原始综合/负控制，覆盖新目录失败工件。

## 本轮命令与工具

可移植入口`128-3-build-runner.py`依调用者PATH、RHDL_FIRTOOL_PATH目录、BITLOOM_SBY_SOURCE；默认每次新目录，不覆写历史结果。每次保留commands.json中的argv/UTC/退出码/环境、source-sha256.json，工具子进程另存命令时间及状态。Rust1.97.1，Icarus12.0，Chisel7.15.0/Scala2.13.18/sbt1.10.11/Java17，firtool1.159.0；本机Yosys0.33（2584903a060）、Z3 4.8.12，按实际版本记录，不称工具钉升级。

SBY官方origin、tag对象bfc1c47eb786496fe794481ff88e75728f0529a6、剥离commit/HEAD daed0e1544fd96ee7dab843e5a891d92784c6230、tracked源码与安装launcher+支持模块16文件逐字节重建绑定。`python -O`正常身份检查仍成功；另复制安装目录后只改launcher，`python -O`确以installed/source mismatch拒绝（expected failure），真实安装未动。版本输出不是证明结果。

## 已执行结果

- 活动ATDD：9 PASS、2专用入口ignored另实际执行。最终活动记录`target/fr197-irq-reruns/20260921T094626.409300Z-4030408/atdd.log`，三个seed每个31,859/31,860/31,859帧，共95,578帧。独立逐源oracle穷举每seed1024源/mask组合、240 WSTRB/寄存器/数据组合、逐源碰撞/RAW/门控及3200随机cycle；提交/消费/取消分别12769/12762/7、12759/12751/8、12769/12762/7，账本守恒。
- direct实际RTL和同/异名双实例、真实Timer→IRQ组合通过；native Interpreter/Compiled与GeneratedFunctional对四层级入口按预期拒绝，属于unsupported验收，不计native行为PASS。
- FIRRTL→firtool与Chisel→JVM→firtool→Icarus各运行同一95,578帧黄金。合并seed仅每段首reset的沿前旧状态不比较；首reset沿后、其余全部帧/复位前后断言保持。三图组合在两个后端各实际执行，共六图/后端组合。
- 独立ghost safety prove PASS（depth16归纳），cover PASS（depth24、23/23 reached）。不假设最终rsp_ready；仅初始reset与受阻请求稳定。pending/enable/raw_value均assert对应；RAW软件污染即便单端口总线读不可同时观察，内部RAW assert仍能检出。
- 原始control prove PASS；clear优先、TEST未commit门控、RAW含TEST三种DUT定点变异均SBY FAIL/exit2，失败行含指定irq_pending/irq_raw，均有VCD；不是编译error/UNKNOWN/timeout冒充检出。
- 原始无observer RTL Yosys proc/check/synth/check通过，138 cells，含DFF、无latch；不宣称PPA/板级签核。证明和综合产物分目录。

## 真实失败与修复

保留全部首次失败：09:39 runner中的ATDD Timer夹具使用SV保留词`matches`，改为match_count；09:40 runner的Chisel实际JVM暴露既有Concat裸Cat缺import，最小修复发射器为`chisel3.util.Cat`，不改生成Scala/工具钉、不扩大多位Mux deferred；09:43 runner的新增emit回归夹具漏clk/rst，补齐既有freeze合同；09:46 runner中新增命令元数据误用synthesis.json覆盖原始netlist统计，改为*-command.json。该轮Yosys自身exit0但统计验收失败，未计综合PASS；之后完整formal重新PASS。

Emit回归覆盖机械、idiomatic与FR188风格面名字解析；字符串检查只作防回归，拼接位序/宽度行为由真实JVM IRQ向量验证。后续FIRRTL/numeric兼容门单列原始退出码，不能用emit成功替代行为。

## 最终构建门与归档

最终门执行及归档索引由下方完成记录补充；无原始证据的项不计PASS。后续主流程负责独立审查、automate及完整workspace回归。

最终稳定实现门 `target/fr197-irq-reruns/20260921T094746.224684Z-4041781` 全部exit0：formal 6.85s、全向量后端88.96s、三图后端49.56s、FIRRTL包21 PASS（1.40s）、既有numeric13 Rust PASS并实际18个Scala/JVM→RTL case（75.96s）、semver7.94s、原文例4.64s、C11头编译执行0.06s。活动ATDD采用前一轮09:46最终测试源码记录，9活动PASS；其2 ignored随后真实执行，不重复计数。专用IRQ共有5入口真实PASS（2后端+3formal/综合/负控制）。

最终formal目录：`target/fr197-irq-formal/{proof,synthesis,mutations}-4041934-*`。最终后端及三图目录由最终runner各log的artifacts行给出；工具metadata按实际cwd目录保存argv、UTC Unix毫秒/退出码。18case numeric隔离在最终runner的numeric/，不覆盖旧故事输出。`python -O`正常身份目录09:48；故障安装检出为`target/fr197-irq-reruns/identity-negative-20260921T094904.165186Z`。

所有runner过程目录依次为09:39:10（夹具SV语法失败）、09:40:39（ATDD/formal通过、Chisel裸Cat失败）、09:42:30（semver/例/C通过）、09:43:23（后端通过、新增emit夹具缺时钟失败）、09:46:26（最终活动ATDD通过、统计文件名冲突失败）、09:47:46（最终专用完整通过）、09:48:18（-O身份通过），日期均20260921且完整微秒/PID见归档member表。失败保留不覆写；同故事已解决，不列为未完成产品缺陷。

归档：[`128-3-build-raw-20260921T095151.183659Z.tar.gz`](128-3-build-raw-20260921T095151.183659Z.tar.gz)，SHA256 `1971244fb4145eb40b8e107bde5f2b96644d0456a0a9e3e02a60c9e576656f29`；[逐成员manifest](128-3-build-raw-20260921T095151.183659Z.manifest.json)记录1516成员，已逐member回读验SHA及集合/重复项。保留全部成功/失败原始RTL、testbench、VCD、proof/synthesis及命令日志/源码摘要；排除编译sim可执行与Cargo/sbt缓存，排除名单亦列manifest。最终实现文件摘要见[final-source](128-3-build-final-source.json)。clean之前证据已经脱离target保存，原始日志未为whitespace检查改写。

当前无未解决的本故事build缺陷。`git diff --check`通过；只定向rustfmt本故事源码/原文例，未执行cargo clean或最终全workspace。构建验证不代替主流程后续独立code-review/automate/clean→fmt→just test及单故事commit。


## 构建审查后的复现边界

经过身份核验的形式复现必须使用`python3 _agile-output/test-artifacts/128-3-build-runner.py --only formal`，调用者设置PATH和BITLOOM_SBY_SOURCE。直接cargo执行`fr197_irq_formal -- --ignored`不检查安装身份，因此仅为未核验的直接测试入口。runner强制SBY官方源码/安装绑定，产品Rust/Chisel/firtool钉保持；本机Yosys0.33/Z34.8.12只是记录值，runner不强制这两个host版本，不把记录当版本锁。

审查修复：归档先拒绝空输入或缺少direct/两后端/三图组合/formal/负控制/软件/身份/成功命令等完整构建证据类；runner在启动前持久记录尝试，异常仍保存状态/错误，软件产物比较独立列为gate。SBY全部安装查找路径拒绝未核验sby_*包或独立字节码；实际执行经`-I -B`隔离解释器、优先已核验support目录和新建cache prefix，不读取旧__pycache__，不修改真实安装。历史归档与原始日志保持原字节；历史身份正检结果不倒推已经具有此新增防影子能力。

定向修复回归：`128-3-build-harness-tests.py`在普通Python和`python -O`各6测试通过，包含空/缺类归档、原有效归档证据类、launch失败持久化、软件比较pass/fail、三查找路径package/legacy-pyc拒绝、实际可执行的unchecked-hash旧cache被隔离忽略、正常复制安装可执行。仅修改复制安装，真实安装未改。日志为`128-3-build-review-fixes-tests-2.log`与`128-3-build-review-fixes-tests-optimized.log`；首次mock误接git产生的2错误保留于`128-3-build-review-fixes-tests.log`，误生成的-z目录和本轮pycache已清除。未重复硬件/完整回归；主流程后续验证负责新runner下的正式formal门。


主代理修补后复验（2026-09-21）：新runner全部命令exit0，活动ATDD9项、formal3项（23 cover及3预期反例）、全向量与三组合后端、FIRRTL21项、numeric实际JVM、SemVer、原文例及C头通过。完整命令/源码摘要见128-3-root-review-recheck.json；新归档128-3-review-fix-raw.tar.gz，2228成员逐字节对照原target再次通过，SHA256 989b72dd2d932ebc0eccce8d69b151d5e71ef1c18a6dd9eb16de90145c419557。B1/B3/B4/B5/B6/B7及E1修补关闭；历史日志/归档不改。第六步clean/fmt/全workspace仍按七步顺序随后执行，不以本轮定向复验替代。


## 独立代码审查patch后的当前验证合同

B1/B2/B3/B4/B5：新归档必须显式`--run <一次完整默认runner的输出目录>`。只读取该run的gate结果（包含example-artifacts），核对该run的起止源码fingerprint与当前相关源码，再读取该run各gate记录的新artifact目录；prove/cover/control必须PASS，三mutant必须FAIL。自定义输出路径作为run/成员归档，关联原始工具输出作为artifacts/成员；`--include-history`仅附带旧记录，不参与当前成功判定。tar和manifest均禁止覆写。旧归档仍为原始历史证据，不被回写，也不能代替新合同的完整run。

B9/B10：两exact后端入口检查指定测试确实执行且1 passed/0 failed/0 ignored，cargo零匹配exit0仍失败。runner在开始/结束记录相关输入fingerprint，变化即拒绝；包含产品/测试源码、runner/archive/harness、工具钉、根Justfile、CI与实际原文/软件产物，排除持续新增日志、归档、JSON历史记录及状态文档。所有gate保存新artifact目录，防止从任意历史PASS拼凑当前结果。

B6：当前WSTRB矩阵每seed320组，TEST分别从硬件播种的pending0和pending01010开始，每组80向量（16WSTRB×5数据），独立oracle并读回；PENDING/ENABLE各80组。修补后的direct三seed实际通过32,579/32,580/32,579帧，总97,738帧，账本分别12929/12922/7、12919/12911/8、12929/12922/7。首次启动因本轮PATH未含已有iverilog而exit101，保留`128-3-code-review-patch-direct.log/.json`；补回调用者PATH后同入口通过，见`128-3-code-review-patch-direct-2.log/.json`。上述旧95,578帧/240组记录保留为修补前历史，不回写。

B7：formal CI增加harness普通与-O入口。定向harness12项在普通Python和-O均通过，包含历史PASS不能补缺、comparison必需、相关源码不匹配/运行中变化、formal ERROR/UNKNOWN/反向状态、所选raw缺失、外部run归档回读、tar/孤立manifest保护、零测试/ignored/错误target拒绝、SBY复制安装包/bytecode/cache隔离。新harness不依赖历史tar或target，CI可从干净checkout执行。原先12条裁定与B8驳回保持，不扩Concat值域。

本patch只跑修改文件关联的direct RTL与harness；未重跑全部后端/完整构建门。主流程须在最后源码稳定后运行默认runner一次，再执行`python3 _agile-output/test-artifacts/128-3-build-archive.py --run <该输出目录> [新tar.gz路径] [--include-history]`。不clean/commit、不改sprint/总账/spec，未改历史归档或raw日志。


修补完成复核：9组patch全部落实并经主代理审阅。direct扩展为97,738帧，普通/-O工具回归各12项通过；新默认完整runner 20260921T103117.009451Z-237066全命令PASS，活动9项、formal3项、完整两后端及三组合、FIRRTL21项、numeric/semver/example/header全部通过，两exact目标各1次真实PASS。起止和当前566源文件SHA一致。原始归档128-3-code-review-raw.tar.gz由root独立逐字节复核3681成员，SHA256 7ef194ce9e68a300cd4ad42475f26349a9eeae051a108adbe9761630247ded08；详见128-3-root-code-review-recheck.json。代码审查阶段完成，0 decision-needed、9 fixed、0 defer、1 rejected。用户七步顺序优先：Story/sprint在automate及clean/fmt/workspace/commit之前仍review，不提前done。


最终回归后指纹边界修复：cargo clean/fmt/just test全部exit0（1844通过/0失败/35忽略），格式化前后输入完全一致。唯一运行中变化是gitignore保护的crates/rhdl-formal/fixtures/fr119/fr119_pass/logfile.txt，由旧SBY测试生成，非产品源码。source_fingerprint错误包含该.txt，已排除SBY运行logfile.txt并扩充原有动态日志不干扰回归。仅runner/harness两Python文件更改；普通/-O各12项通过，详见128-3-final-source-audit.json与128-3-final-fingerprint-validation.json。原始回归JSON保留这次差异，未回写历史摘要为假一致；Rust产品/测试与CI均未在回归后更改。新完整runner再次绑定修正后的证据脚本及最终源码，旧最终归档保留其历史阶段。
