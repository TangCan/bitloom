# Epic130 外部 IP 试点 NFR14 风险记录

日期：2026-09-22。范围：FR199 / FR200 / FR201 外部部分，NFR14 / NFR94 / NFR98，AD-28 / AD-31。状态：Story130.1 build 已形成，待独立 review、automate、完整回归和单 Story 提交后 accepted。本记录不是来源锁定、离线重放、真实外部核行为或外部支持等级 PASS。

长期维护与许可证升级 owner：Richard。当次实施及证据整理：Codex。七步状态和单 Story 提交：主代理。没有 owner、来源/许可证不可判定或真实行为失败时保持未交付。

## (a) 上游约束

权威依次为用户全部未完成 Story 七步授权、正式 Phase24 合同、FR199–201/NFR98、AD-31、Epic130 context 和 Story130.1。研究只提供候选与初始方案，不是 stable-tag 验收。Bitloom 与 `samitbasu/rhdl` 无关；设计 crate 只依赖 `bitloom-prelude`。

Epic126–129 已关闭，FR198 与 FR201 核心部分已交付。它们不提升外部支持行。当前只解除 130.1 风险门；130.2/130.3、FR199/FR200、FR201 外部部分、Epic130 和整个 Phase24 均未交付。FR189/Epic122 deferred、NFR91、已有 done 保持；不 push、不 publish。

| 后续入口 | 必须先满足 | 责任与失败动作 |
|---|---|---|
| 130.1 | 125.3/M0 | 已满足；只交风险门 |
| 130.2 / FR199 | 130.1 done | 来源/lock/license/offline 任一合同不可执行则保持 backlog |
| 130.3 / FR200 | 130.2、126.2 done | 真实 wrapper/RTL/oracle 任一失败则不适配、不提升支持级别 |
| Epic130 / Phase24 | 130.1–130.3 全 done，核心与外部分列 | 不用核心关闭吞并外部行 |

### 候选与稳定版本边界

PULP `common_cells` 稳定 `v1.40.0` FIFO 只是首选方向。既有研究实际读过 master 的 `cc_fifo`，不能据此断言 stable tag 的路径、接口、依赖或许可证。130.2 必须从上游重新解析：

- tag 名称与 tag object；annotated/lightweight 类型及 peeled full commit；
- stable 内容中的实际 `fifo_v3` 路径、顶层 module、参数、平坦端口、clock/reset；
- 递归依赖和生成器输入的完整 commit；
- 有序 filelist、include 路径、宏、编译顺序及逐文件/归档 SHA256；
- 该仓库实际许可证原文、NOTICE 和再分发责任。

任何一项不符合合同时记录拒绝并停止。不得自动切换 Taxi、浮动 master、复制研究中的源码或裁掉依赖；Taxi 如需使用必须重新走完整准入。

### 架构与工具边界

现有 `ExtBlackBox` 只声明 `clk/rst/data_in/data_out`，`vendor_blackbox_v()` 是无行为 stub。它只用于证明外部 Verilog 文件能参与 compile/sim/synthesis，不能证明 source lock、离线闭包或行为。

AD-31 要求在现有外部模块表示上绑定来源清单和 lock 元数据。源码解析、获取和编排属于 CLI/构建层；prelude 不联网、不拉第三方源码，第三方源码不嵌入 HIR。若当前 HIR 无法准确表达候选的参数/端口，先提出最小扩展并补审，不创建第二套 IR。

本 Epic 不修改 Rust 1.97.1、edition 2024、firtool 1.159.0、Chisel 7.15.0、Yosys 产品声明、包版本或发布状态。本机工具版本是执行证据，不自动成为产品 pin。

## (b) 粗工期带、支持参数与维护成本

130.1 为 0.5–1 有效人日，130.2 为 2–3，130.3 为 3–5；Epic130 合计 5.5–9。若管理采用 25% 集成预留，另列 6.875–11.25，有效人日而非代理墙钟、日历承诺或发布日期。置信度中等：stable 闭包、许可证和离线缓存成本尚需 130.2 实测，wrapper 与 oracle 成本需 130.3 实测。

首个试点限一个小型同步 FIFO/stream 核及一个冻结参数集。不得扩展到 Ethernet/DDR/PCIe、任意外部目录、未列协议、多时钟、原生层级仿真、PPA/时序/物理签核或发布。130.3 才冻结真实 module、参数、平坦端口、clock/reset、ordered filelist 和 wrapper 版本。

维护叠加包括：上游 tag/递归依赖与许可证漂移、wrapper 端口兼容、生成器与工具版本、离线缓存格式、required CI 时间、证据重放、弃用窗口及 owner 交接。先记录实际步骤和成本，不承诺未经测量的采用率、节省、PPA、升级频率或收益百分比。

## FR199 manifest / lock / license / offline 合同

Manifest 表达依赖意图，lock 表达一次解析后的不可变身份。仅有 lock 文件不自动证明本地路径、下载资源、生成器和工具已完整锁定。

| 类别 | 130.2 必填 |
|---|---|
| 上游意图 | URL、tag/version 意图、来源类型、候选理由 |
| 不可变身份 | tag object、peeled/full commit、每个递归依赖 full commit |
| 内容闭包 | ordered filelist、include、宏、编译顺序、每个源码/归档 SHA256、未声明文件拒绝规则 |
| 生成与工具 | 生成器输入及版本、命令、工具锁、缓存布局、wrapper 版本 |
| 绑定 | 顶层 module、参数、端口宽向、clock/reset 映射、已测参数 |
| 许可证 | 原文路径及 SHA256、NOTICE、依赖逐项许可证、再分发/归属义务 |
| 维护 | owner、证据路径、最后通过 UTC、升级与弃用策略 |

离线重放分两个独立阶段：

1. 在空缓存中允许网络，解析并获取声明的完整闭包，保存不可变身份和内容校验。
2. 使用隔离 checkout 和仅含第一阶段产物的缓存，禁止网络，重新完成生成、构建、上游测试与 Bitloom 测试。

浮动引用、hash/license/NOTICE 漂移、未声明文件、缓存外读取、网络访问、缺失递归依赖或生成器输入均硬失败。网络 fetch 成功只证明当时可达；禁网命名空间存在只证明隔离机制可用。130.2 必须用真实候选完整重放后才可交付 FR199。

## FR200 真实适配与支持等级合同

130.3 必须把真实 upstream module、冻结参数、端口/clock/reset、source association 和 Bitloom wrapper 逐项绑定。验收同时运行上游测试和独立 Bitloom 组合 RTL 测试；独立 oracle 不复用 DUT 行为。无 native 模型明确 `unsupported`；如另加 Rust 模型，须单独验证它与锁定 RTL 等价。

支持等级累积升级：

| 等级 | 必要证据 |
|---|---|
| catalogued | 候选、用途、owner 和上游入口已记录 |
| locked | manifest/lock、完整闭包、校验与许可证通过 |
| compiled | 锁定来源由记录的真实工具成功编译 |
| behavior-tested | actual RTL 由独立 oracle 执行并通过；compile-only 不足 |
| maintained | required CI、长期 owner、冻结支持参数、升级/弃用策略与最近通过证据 |

零输出模型、空 blackbox、compile-only、上游自测、网络可达、工具版本、目录条目或 FR201 核心矩阵都不得升级 `behavior-tested`。外部支持矩阵在 130.3 验收前保持全 `no` / `not delivered`。

## (c) 禁止静默降级与停止条件

不得用 master 替 stable、短 SHA 替完整对象、仅 URL/tag 名替内容锁、单一归档 hash 替递归闭包、同作者其他仓库许可证替实际许可证；不得忽略 NOTICE、生成器、本地路径、缓存外读取或编译顺序。

不得因获取工具缺失、网络失败、禁网失败、hash/license 漂移、compile/sim/synth 失败、测试为零、超时或未知而 skip/PASS。不得用空壳输出零、仅 lint/compile、上游测试或核心证据冒充独立行为。不得悄悄换源、缩参数、改端口/reset、裁依赖、扩公共 API、改工具 pin 或创建第二 IR。

| 风险 / owner | 控制与失败动作 | 停止条件 |
|---|---|---|
| stable 身份/闭包 / Codex 实施 | 解析 tag object/peeled commit/递归闭包并逐内容校验 | 任何身份或闭包无法固定，130.2 不 ready |
| license/NOTICE / Richard 决策、Codex 取证 | 逐依赖保存原文/hash/义务，未知升级人工决策 | 许可证或再分发责任不清，拒绝候选 |
| 离线边界 / Codex | 空缓存在线获取后以禁网隔离重放，负测网络/未声明文件 | 网络仍可达、缓存泄漏或重放失败，FR199 不交付 |
| 表示/绑定 / Richard 架构、Codex 实施 | 精确参数/端口/reset；不足则最小扩展补审 | 现有表示无法准确绑定且扩展未批准，130.3 停止 |
| 真实行为 / Codex | 上游测试加独立组合 RTL oracle，保存原始失败 | RTL 与 oracle 分歧或无 actual RTL，FR200 不交付 |
| 维护能力 / Richard | required CI、owner、冻结支持范围和弃用策略 | 无长期 owner 或 CI 资源，不称 maintained |
| 范围漂移 / 主代理 | diff 审计产品源码/API/pin/version/矩阵 | 需要未批准扩面则重开规格，不在 130.1 偷做 |

停止时保留最后可复现身份、失败日志和候选状态，130.2 保持 backlog；不得用“实验性”标签掩盖本应硬失败的准入条件。

## (d) 负责人

Richard 负责长期维护、许可证/再分发和架构升级决策；Codex 负责当次 manifest/lock、工具运行、wrapper、测试和证据；主代理负责七步独立审查、完整回归、状态和单 Story 提交。依赖责任不能转给未命名的社区或并行代理。

130.2 的交付 owner 是来源/许可证/离线闭包；130.3 的交付 owner 是真实绑定/行为/支持升级。任一故事只能在自身实际证据完整后改变状态。

## Story130.1 实际探针与证据边界

`130-1-build-probe.py` 在普通与优化 Python 下各执行一次，结构化结果位于 `130-1-build-probe-{normal,opt}/results.json`。实际检查 Git、SHA256、递归子模块枚举、HEAD archive、`bwrap --unshare-net` 仅 loopback、Icarus/vvp、Yosys、Rust `fr82_blackbox_boundary_documented`，并用 `vendor_ext_ip.v` 真实 compile/sim/synthesis。源码、命令、UTC、耗时、退出码、工具路径/hash 和日志均归档。

本机 `unshare -n` 因权限退出 1，原始环境探测未被称为成功；可用的 bwrap 隔离实际验证仅 loopback。Icarus 来自既有 `/tmp/bitloom-maintenance-tools/bin`，Yosys 为本机执行版本；这些路径是本轮证据，不写入产品合同。

空模块 simulation 只输出 `behavior=absent` 机制标记，没有数据行为断言。它证明外部文件被 elaboration/compile/sim/synth 入口消费，不证明任何上游核、许可证、离线重放或 FR200 行为。外部矩阵因此保持未交付。

ATDD presence 在正文创建前准确 RED；正文创建后应 GREEN。普通和优化 Python 状态门各 84 场景验证当前、NFR14 前置、M0、提前 Epic 关闭及合法未来状态，临时副本不改变真实 sprint。130.3 依赖 130.2 与 126.2 由人工审计，因为通用脚本只自动检查 `.1`。

本记录有效性还须完成 `130-1-atdd-manual-acceptance.md` M01–M21、独立 code review、automate、clean/fmt/`just test`。M22 与最终状态只由主代理在七步齐备后关闭。
