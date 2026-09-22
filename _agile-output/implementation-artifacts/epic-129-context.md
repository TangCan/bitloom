# Epic 129 Context: 完整系统与核心证据

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

交付可复现的 UART/GPIO/Timer/IRQ 完整外设系统（FR198），以两种真实组合、后端执行矩阵及维护规范完成 FR201 核心部分。M0–M3 已关闭，当前整体七步执行授权有效；历史 M0-only 记录不再限制执行。Epic129 尚未交付，核心关闭不能代替 Epic130 外部试点或整个 Phase24 关闭。

## Stories

- Story 129.1: 组合系统与证据 NFR14
- Story 129.2: 外设子系统与使用配方
- Story 129.3: CI、兼容性、贡献模板与核心关闭

## Requirements & Constraints

完整 AXI-Lite 系统必须由独立主端配置 Timer、控制 GPIO32、完成真实 UART 串行收发并清中断。另一个实质不同拓扑采用直接 CSR 主端连接同四窗四外设；仅改名、旧局部夹具或空寄存器银行不算第二组合。两例实际运行，共享定义与事件接线。

设计仅依赖 bitloom-prelude。限定单时钟、地址16位、数据32位、WSTRB4、UART8N1、小寄存器 FIFO。四个256字节窗口依次为 UART 0x0000、GPIO 0x0100、Timer 0x0200、IRQ 0x0300；窗口外 DECERR，窗内洞、未对齐及非法访问 SLVERR，不截断高地址。GPIO32、IRQ5 优先于旧研究草案。

独立预期手写关键地址与副作用；验证 AW/W 独立、并发读写、响应背压、全部 WSTRB、错误及复位取消。接受、CSR提交、响应消费分开记账，受阻不重复副作用。硬件原始事件与本地粘滞 EVENT 分开：IRQ 清除后不因旧 EVENT 重触发，新事件仍可到达。安全性不依赖 ready 公平；完成上界另列假设。

干净独立 checkout、隔离产物目录中一条命令复现完整配方，明确工具前提，不依赖主工作区 target 或临时生成文件。实际记录上手步骤、接线/地址代码量、修改文件数及运行/CI成本，不承诺未经测量的性能或 PPA。

## Technical Decisions

AD-30：复用同一 ElaborateSession 定义模块，顶层一次 finish/freeze；保留唯一 FrozenHir，不拼接冻结图。全部内部模块共同同步高有效 reset；顶层 aresetn 断言和释放须先由外部控制器同步，内部反相不等于同步器。IRQ0–4 分别接 Timer、UART RX到达、TX取队首、overflow或framing、GPIO新沿的原始事件，无第六路。

aresetn 类型及 Chisel 隐式 reset 是待实际探针确认的风险，不能把静态审计写成已证实故障。最小父子状态图须在 direct/FIRRTL/Chisel 编译执行，区分显式与隐式 reset，验证共同清状态及恢复。若采用随例子交付的无状态 RTL 边界，应明确产物为生成核心加适配层，纳入源码/综合/执行证据，不宣称全部顶层由 HIR 生成。未解决不得放行后续功能故事。

三后端实际 RTL 结果分别记录；native 两引擎及 generated 层级明确 unsupported。Rust、形式 prove/cover、原始综合、SemVer 分列，emit 或工具发现不代替行为/证明。绑定命令、退出码、工具身份、种子、日志、波形及源码哈希；缺工具、零测试、UNKNOWN、超时不得计通过，clean 前归档。

沿用固定 Rust1.97.1、firtool1.159.0、Chisel7.15.0 与已锁 BFM 闭包；探针核真实工具身份并复跑 BFM stub、旧桥/译码基座，不把它们称新系统 PASS。新增公开表面逐符号登记 FR142/SemVer，旧 API 保持；不升工具钉、包版本或发布。AD-31 的外部源码锁及真实外部核适配属于 Epic130，核心矩阵与外部分栏。

## Cross-Story Dependencies

129.1 风险门前置为125.3/M0；必须具备上游限制、估算、owner、禁止降级项与停止条件。129.2 依赖129.1、127.4、128.2、128.4、128.5，集成也使用已交付128.3 IRQ；129.3依赖129.2并补齐核心关闭。贡献模板包含owner、独立测试、两种组合、兼容性、来源及证据。Richard维护，Codex实施/记录；每故事七步、一故事一提交。FR189 deferred/NFR91 与历史关闭保持，不push/publish。
