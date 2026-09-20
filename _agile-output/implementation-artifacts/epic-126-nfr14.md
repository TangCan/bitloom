# Epic 126 NFR14 — 组合与流基础风险门禁

记录日期：2026-09-20；执行者：Codex；项目 owner：Richard。关联 FR194/FR195、AD-30/31、NFR93–99。M0 已关闭，依赖证据见 [Epic125 关闭记录](epic-125-closeout.md)。当前完成风险记录和工具可用性探针，Story126.1 经主代理评审完成（done）；126.1结项时尚无126.2行为证据；后续结果见末节，不宣称 FR195 或 M1 交付。

## 当前授权、估算与依赖

用户在 M0 关闭后再次要求继续，本轮仅执行126.1→126.2。合同、架构和历史 M0 关闭文档中的“仅 M0”记录保留为当时授权历史；新增执行范围已同步到当前 spec、sprint-status、AGENTS.md 及上述文档的后续授权注记，行为接口仍以 [Phase24 合同](../../docs/ip/phase24-contract.md) 和 AD-30/31 为准。126.3/126.4 与 Epic127–130 均未获本轮自动执行授权。

| 故事 | 有效人日 | 执行安排与前置 |
|---|---:|---|
|126.1 组合基础风险门禁|0.5–1|当前授权；125.3/M0 done；Codex执行、Richard owner|
|126.2 共享模块定义与实例校验|3–4|当前授权；必须126.1 done 后启动；Codex执行、Richard owner|
|126.3 两槽 ready/valid 注册切片|2–3|原计划估算，未执行；依赖126.1 done|
|126.4 参数 FIFO 与 M1 关闭|3–5|原计划估算，未执行；依赖126.2与126.3 done|

126.3/126.4 数值沿用 `planning-artifacts/epics.md` Phase24 Inventory，合计 Epic126 为8.5–13人日；不是代理墙钟时间或交付日期承诺。每故事一个提交，由主代理提交。本故事不改产品代码，不重跑 M0 长矩阵，不调整 Rust/MSRV、firtool 等工具钉。历史关闭、FR189/Epic122 deferred、NFR91 均保持。

## 上游约束与风险处理

| 风险 | 处理、验收与失败动作 |
|---|---|
|独立 elaborate 冻结无法组合|复用既有 ElaborateSession/FrozenHir；define_module 与旧 elaborate 共用定义实现。所有模块在单 session 中定义，最终仅 finish/freeze 一次；禁止拼接冻结电路或另立 IR。设计只依赖 bitloom-prelude。|
|模块身份、复用及参数冲突|身份由明确模块名字、完整规范化参数和定义内容共同确定；同名、同参数且同定义内容才可复用。同配置不同名字可独立定义；不同配置同名字拒绝，同名同参数但定义内容不同也拒绝；重复参数键拒绝，不能通过覆盖键或仅比较指纹掩盖冲突。不同参数在 elaborate 时专门化为不同且受控的模块身份。Instance.params 不会被后端实现，不能作为 RTL 参数化支持承诺。身份冲突若无法解决，停止126.2，不带病冻结。|
|活动模块被覆盖、顶层误选|已有未 end_module 的活动定义不得被隐式覆盖。顶层名显式等于电路名；顶层可先定义并结束，再定义子模块，不依赖最后定义者推定 top。|
|局部名碰撞与图验证不完整|多个实例可有同名局部信号，但实例作用域必须隔离。freeze 前对全图检查重复模块/实例、缺模块、实例环（包括非 top 可达定义）、连接完整性、端口方向/宽度和多驱动；使用既有 Diagnostics，不只遍历第一个模块或以名称前缀伪装层级语义。|
|旧 API 或稳定性回归|保留旧 elaborate 的签名、公开端口和既有行为。126.2对新增 define_module、模块身份/注册接口及公开诊断等逐符号列出公共 API，更新 FR142 表面清单和 SemVer minor 说明；新增 API 不自动纳入稳定承诺。旧入口与新入口共用实现并跑兼容检查；旧 API 不能兼容则停止，不能擅改稳定合同。|
|native/generated 层级限制|继续明确拒绝 unsupported 多模块层级，不以零值、扁平名称或 emit 成功冒充模拟完成。126.2层级行为必须执行真实生成 RTL，并分别记录生成、编译与 vvp 执行结果。|
|复位语义漂移|默认单时钟、内部同步高有效 reset，所有 leaf 同域。后续 aresetn 需外部控制器同步断言和释放后转换；反相本身不实现同步化。126.2保留旧 IP reset 行为并验证。|
|RTL 工具缺失或结果分歧|工具缺失/不可执行严格失败，不允许 skip 为 pass。真实 RTL 执行或独立期望失败时保留命令、日志、种子/向量和波形，先定位差异；无法通过真实 RTL 则停止126.2，不以 native 或 emit 结果覆盖。|

126.2首批迁移 Gpio，让新 define_module 与旧 elaborate 共用同一定义体；以相同输入/同步 reset 向量证明旧入口端口与行为等价。独立参数夹具包含两个8位实例（共用一个8位定义）及一个16位实例（专门化定义），明确这不是将 Gpio 改成参数化产品。至少验证同参数双实例复用、异参数专门化、同名局部信号隔离、顶层先定义、活动定义不可覆盖，以及缺模块/环/重复定义/宽向错误/多驱动负向诊断；实际 RTL 检查输出和同步 reset，旧 elaborate 通过兼容回归。工具和范围阻塞未解决不得标 ready；风险文档本身不是功能交付证据。

## 后续规划风险登记（本轮不实施）

126.3两槽 RvRegSlice 必须保证输入 ready 与输出 valid/data 由寄存状态驱动、无输入到输出组合路径；受阻稳定、不丢不重，满后恢复允许一个等待拍。随机 RTL 流需独立参考及固定种子；小状态 formal 需实际 formal 工具和明确公平性假设，纯 Rust 枚举不算形式证明。后续工具安装入口采用 [scripts/ci-install-sby.sh](../../scripts/ci-install-sby.sh)：按现有 ci-sby-pins.env 安装 sby（脚本安装目标 `/usr/local/bin/sby`），apt 安装 yosys/z3；运行入口参考 [scripts/formal-sby-check.sh](../../scripts/formal-sby-check.sh)（`just formal-sby-check`），通过 PATH 的 `command -v sby`、`command -v yosys`、`command -v z3` 发现实际工具，记录 sby版本/帮助、`yosys -V`、`z3 -version`，使用 `smtbmc z3` 引擎。既有 FR119 fixture 通过不能替代126.3组件属性，后续须提供专用 harness。无丢失、无重复、受阻稳定属于安全属性；最终完成属于活性，须单独列出下游 ready 公平性或等待上界假设，reset/flush取消另计。此处只登记后续路径，本轮未安装/运行 formal，也不宣称通过；执行126.3前须重新探测并记录实际版本，缺工具不得跳过后宣称通过。

126.4 ParamSyncFifo 仅 WIDTH=1..64、DEPTH=1..16 的寄存器 FIFO，覆盖非二次幂；无空直通，满时即使同拍 pop 也不接受 push。reset > flush > 正常传输，清占用/有效状态，不承诺 payload 全部清零；非法参数显式失败。计划矩阵为宽1/8/32/64 × 深1/2/3/4/7/16及空/满、同拍、reset/flush、非法参数；旧 SyncFifo 8×4行为兼容。深 RAM、异步 FIFO、通用 FIRRTL 内存、native 层级仿真不在范围。综合/PPA、formal、native 和 RTL 证据分别记录，不互相替代。M1 关闭必须126.1–126.4全部 done并补齐风险关闭、需求映射与实际支持矩阵。

## 实测工具探针（2026-09-20）

| 命令 | 实际路径与结果 |
|---|---|
|`command -v rustc`；`rustc --version`；`rustup which rustc`|入口 `/home/richard/.cargo/bin/rustc`；实际 `/home/richard/.rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/bin/rustc`；`rustc 1.97.1 (8bab26f4f 2026-07-14)`|
|`/tmp/bitloom-maintenance-tools/bin/iverilog -V`|该入口为实际 wrapper 路径；Icarus Verilog 12.0 (stable)，命令退出0|
|`/tmp/bitloom-maintenance-tools/bin/vvp -V`|解析路径 `/tmp/bitloom-maintenance-tools/iverilog/usr/bin/vvp`；Icarus Verilog runtime 12.0 (stable)，命令退出0|

CI 安装入口为 [.github/workflows/ci.yml](../../.github/workflows/ci.yml) 的 `sudo apt-get update && sudo apt-get install -y iverilog`。126.2通过 `command -v iverilog` 和 `command -v vvp` 从 PATH 发现工具，再运行 `iverilog -V`、`vvp -V` 记录实际版本与路径；不固定临时 wrapper，也不假定任何环境必为12.0。2026-09-20复测时默认 PATH 均未找到工具，因此仅在本地探针 PATH 前加入 `/tmp/bitloom-maintenance-tools/bin` 后重测，二者实际均报告12.0 stable、退出0，与上表一致。CI或其他安装可直接使用其 PATH 中的工具；安装/配置后仍找不到工具须严格失败。本次只证明工具可调用；未编译执行组合 RTL，行为证据留待126.2。门禁状态变异验证见 [126.1 spec Verification](spec-126-1-composition-gate.md#verification)。

## Story126.2 验收结果

2026-09-20 Story126.2 / FR194 已完成：同一session模块定义复用、参数专门化与实例图/连接诊断；首批Gpio共用定义体，实际RTL验证通过。实现与证据见 docs/ip/module-composition.md 和 docs/ip/module-composition-evidence.md。126.1/126.2 done；126.3/126.4及Epic127–130保持backlog，Epic126/M1未关闭，FR195–201未交付；FR189 deferred/NFR91保持。

风险关闭：模块身份、生命周期、全图校验、连接与旧GPIO兼容均有对应回归；新API已显式登记FR142。保留实例/局部网名合法性、Instance.params覆盖两个既有缺口（deferred-work.md）。formal及流/FIFO仍待126.3/126.4，不把126.2测试当作后续验收。
