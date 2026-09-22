# Story129.1 真实层级同步复位探针

实施者Codex；最终运行2026-09-22T00:42:44.928859Z至00:43:15.552903Z附近，25条外部命令耗时合计30.770秒。确定性，seed=N/A；没有系统随机事务。本故事只确认复位路线，未交付FR198/FR201系统。

## 输入、重跑与源码边界

源码在`129-1-reset-probe/`：`design/`只依赖bitloom-prelude，两子寄存器同ElaborateSession、一次finish；`harness/`是隔离工具crate，依赖后端发射器，将direct/FIRRTL/Chisel输出保存到指定目录。没有修改workspace产品/API/pin。`boundary.v`是公开可读的自有无状态aresetn边界。runner只读核验它，三个后端使用相同字节；Chisel端口命名通过明确的无状态`CorePorts` shim适配。两层均参与真实综合。

重跑：先在PATH提供固定Rust/Cargo1.97.1、firtool1.159.0、Icarus/vvp、Java17、sbt、Yosys，并将`CHISEL_FIRTOOL_PATH`设为该firtool所在目录，再执行：

```sh
python3 _agile-output/test-artifacts/129-1-reset-probe/run.py --output "$PWD/129-1-reset-replay"
```

输出目录必须不存在；工具通过环境发现，不将本机/tmp路径写成用户依赖配方。Cargo产物隔离到输出目录。runner给每个子进程timeout、命令、实际路径、UTC、耗时、退出码和合并原始输出；pin不符硬失败，缺工具不skip。Cargo第一次可能获取依赖，净checkout系统配方仍由129.2交付，不以本探针冒充其验收。

## 实际结果与限制

|路线|direct|FIRRTL|Chisel|
|---|---|---|---|
|typed aresetn→UInt1→Eq/Reset wire→两child rst|编译+行为PASS|降低+编译+行为PASS|实际JVM编译后firtool lowering失败，未生成可运行RTL|
|通常clk/rst生成core + aresetn无状态边界|编译+行为PASS|降低+编译+行为PASS|JVM降低+编译+行为PASS|
|适配路线纯设计综合|PASS|PASS|PASS|

typed Chisel的实际诊断为`public module port must have concrete reset type`，指向`io.aresetn : Reset`。生成Scala也可见child由`Module(new Cell)`创建而core_reset没有显式连到child reset，但本次因更早的lowering失败，**不能把隐式reset连接问题称为已经行为验证的故障**。没有修改Scala强制类型或偷偷绑隐式reset来制造typed PASS，原始失败保留。

成功路线测试台公开输入aresetn；`ResetTop`实际反相驱动core的共同同步reset，Chisel shim显式连接`.reset(rst)`，不是测试台补额外reset。typed台的implicit_reset只用于初始状态，在aresetn测试前独立释放为0。每个成功仿真实际将两个状态写为0x35/0xa7，在低时钟改变aresetn并保持enable和非零新写数据：有效沿前检查状态不变；有效沿同时清零证明同步与reset胜写；释放后沿前仍零、有效沿恢复为0x19/0x63。VCD与`RESET PROBE PASS children=2 synchronous=1 priority=1 recovery=1 seed=N/A`均保存；watchdog防空跑。

综合是无TB/observer的生成core+端口shim+boundary，执行`hierarchy -check; synth; check -assert; stat`。它证明本有限图可综合、结构检查通过，不是全系统形式证明、PPA或板级同步可靠性。aresetn断言与释放仍由外部控制器同步到ACLK，反相不是同步器。

工具：Rust/Cargo1.97.1、firtool1.159.0、Icarus/vvp12.0、Java17.0.20、Yosys0.33；sbt1.10.11实际运行，`build.sbt`与实际SBT resolved报告及本地jar SHA确认Scala2.13.18/Chisel7.15.0及其compiler plugin。完整实际路径/版本原文归档。产品firtool是既定unpaired pin，不采信生成注释中“pair”的字样；实际typed诊断也如实报告Chisel发布针对1.158.0。

## 原始失败与归档

`129-1-reset-evidence.tar.gz`包含157个独立成员，SHA256：`21ca79e9ab0ce3af9ce77db8dc8ea4d911c348a96389bccee42c3fbb4314fc86`。逐文件SHA见`129-1-reset-archive-manifest.json`；已重新读取tar逐成员核验。归档保留生成源码、TB、VCD、命令、stdout/stderr合并日志、结果、Cargo锁、SBT声明与实际解析身份、探针源码快照。排除cargo-target/target缓存、Python缓存和可重新编译的simulation二进制。

- `129-1-reset-raw/`：首次manifest相对路径多一级，cargo exit101，保留原始错误；不是产品故障。
- `129-1-reset-raw-r2/`：typed Chisel lowering失败；适配三后端行为均PASS，但Chisel综合runner的glob误纳tb.sv，Yosys因`$fatal`报错。此失败保留，不称设计综合失败或通过。
- `129-1-reset-raw-r3/`：runner从Chisel生成filelist读取纯设计，适配三后端行为及综合全PASS；typed Chisel仍同一真实lowering失败。`source-identity.json`表明探针输入源起止SHA一致，snapshot与SHA随归档保存。`commands.json`为本次25条精确命令；不是把r2/r3测试并集当独立系统验收。

最终选择**适配路线**，与AD-30相符，产品表述必须为“生成core加公开无状态aresetn边界”，不称整个top纯HIR。129.2仍需用此路线执行真实桥/四窗/四外设图及全部复位取消账本，129.3复核三后端矩阵。typed不支持的当前限制保持，不在本风险故事扩后端能力。

## 当前最终复验（code-review，2026-09-22）

上文r3为历史原始证据。build内审后的r6归档`129-1-review-reset-evidence.tar.gz`保留r4解析错误、r5缓存失败及r6成功。当前code-review修补后最终运行在含空格路径，26条命令；typed direct/FIRRTL PASS，typed Chisel须exit1及确切abstract Reset诊断，适配三后端行为/综合均PASS。源码起止一致且包含工作区依赖/锁；harness/Cargo.lock已显式纳入版本控制。独立两个Rust crate已格式化。

当前归档`129-1-code-review-reset-evidence.tar.gz` SHA256 `671aabb8561062f0876ab60ed8c1754a2512dbb8e8152ee1e8a860906bc858e9`，74成员逐项见`129-1-code-review-reset-manifest.json`；记录含原始失败、实际命令路径、UTC、耗时、波形和源码。后续重放使用本文命令，输出支持空格。
