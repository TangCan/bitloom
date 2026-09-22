# Epic 130 Context: 外部 IP 试点

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

以一个固定版本的小型外部 RTL 核验证 Bitloom 的可复现来源、现有外部模块绑定、真实 RTL 行为和长期支持责任。试点必须把上游源码身份、Bitloom wrapper、构建工具、许可证和支持状态分开记录；空黑盒、仅编译或仅收录目录都不能替代真实适配。

## Stories

- Story 130.1: 外部 IP 准入 NFR14
- Story 130.2: 来源清单、锁定与离线重放
- Story 130.3: 一个真实外部核适配及试点关闭

## Requirements & Constraints

- FR199 要求固定完整上游源码闭包、版本、内容校验、许可证材料、工具和维护人，并通过禁网重放。Manifest 表达依赖意图，lock 表达解析后的不可变身份。
- FR200 要求准确的 module binding、参数/端口/clock/reset 映射、关联 RTL 源和独立行为测试；空壳、零输出模型、compile-only 和上游自测均不足以达标。
- FR201 的核心和外部状态必须分开。外部能力按 `catalogued -> locked -> compiled -> behavior-tested -> maintained` 累积升级；`maintained` 还要求 required CI、长期 owner、冻结支持参数以及升级和弃用策略。
- 外部来源至少记录 upstream URL、tag 意图与对象、peeled/full commit、递归依赖 commit、顺序 filelist、include、宏、生成器输入及版本、逐内容 SHA256、许可证原文路径/hash、NOTICE、wrapper/tool 身份、已测参数、证据位置和最后通过时间。
- 离线验收先从空缓存联网获取完整闭包，再在隔离 checkout/cache 中禁止网络完成构建和测试。浮动引用、未声明文件、缓存外读取、网络访问或身份漂移必须硬失败。
- 设计 crate 仍仅依赖 `bitloom-prelude`；prelude 不获取第三方源码。无 native 行为模型时明确 `unsupported`，不能用零输出代替。
- 首版只接一个小型 FIFO/stream 核，不扩展到复杂外部核、未列协议、发布或商店上架。不改 MSRV、firtool/Chisel 产品钉、包版本或既有 API；新增公共表面必须另行登记。
- 首选方向是 PULP common_cells 稳定 `v1.40.0` FIFO，但其 stable tag、`fifo_v3` 路径/API、完整 commit、递归闭包和许可证必须重新核验。master `cc_fifo` 阅读不是 stable 证据；Taxi 不是自动替代。
- 没有 owner、来源或许可证无法固定、禁网重放失败、真实 RTL 行为失败、现有表示无法准确绑定，或维护资源不足时停止并保持未交付，禁止静默换源或降级。

## Technical Decisions

- 在现有外部模块表示上绑定 source manifest 和 lock 元数据；源码解析、获取和编排属于 CLI/构建层。不得把第三方源码嵌入 HIR 或创建第二套硬件 IR。
- 若现有 HIR 不能表达所需参数或端口，先提出最小扩展并补审，再继续适配。
- 上游核版本与 Bitloom wrapper 版本独立；原生核心、经验证外部适配和仅收录目录是不同支持类别。
- actual RTL 测试是 `behavior-tested` 的必要条件。独立 Bitloom 组合测试不得复用 DUT 的核心行为作为 oracle。
- 工具发现、网络 fetch 和外部文件参与编译分别只证明环境、可达性和绑定机制，不证明 FR199 锁定/离线成功或 FR200 行为。

## Cross-Story Dependencies

- 130.1 在 130.2/130.3 前完成 NFR14 风险门，只冻结接口、工具、责任、证据等级和停止条件，不下载或绑定候选核。
- 130.2 依赖 130.1，交付 FR199 的 manifest/lock、许可证和离线重放。
- 130.3 依赖 130.2 与 126.2，交付真实 wrapper、上游测试和独立组合 RTL 行为，并更新外部支持行。
- Epic129 已关闭 FR198 和 FR201 核心部分；该关闭不提升 Epic130 的外部状态。FR189/Epic122 deferred 与 NFR91 保持。
