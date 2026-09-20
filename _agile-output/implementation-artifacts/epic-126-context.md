# Epic 126 Context: 组合与流基础

<!-- 由规划材料编译。可直接编辑；规划变更后可用 compile-epic-context 重新生成。 -->

## Goal

在同一 elaboration session 内提供可复用模块定义（FR194），并交付经独立验证的小型 ready/valid 基础组件（FR195），为后续 CSR 和外设组合打基础，不引入另一套电路表示。M0 已关闭；用户随后要求继续，授权下一段模块组合工作，即 Story126.1 和126.2，不代表自动执行本阶段其余工作。本文覆盖整个 Epic 的上下文，不宣称 M1 或 Phase24 已交付。

## Stories

- Story 126.1: 组合基础 NFR14
- Story 126.2: 共享模块定义与实例校验
- Story 126.3: 两槽 ready/valid 注册切片
- Story 126.4: 参数 FIFO 与 M1 关闭

## Requirements & Constraints

功能实现前先完成本 Epic 风险闸门：明确上游限制、支持接口与参数、工具探针、负责人、估算、验收条件、依赖、失败处理和停止条件。架构或工具阻塞未解决时不得标记 ready。每故事一个提交；风险文档不能代替功能故事的可运行证据。

保留既有 elaborate API、公开端口和行为。设计仅依赖 `bitloom-prelude`。新增公开符号须显式登记 FR142 表面清单及 SemVer minor 说明；实现 API 不自动扩大稳定承诺。不调整既有 MSRV 和工具钉。历史关闭保持有效，FR189 仍未交付，NFR91 未清空。

范围限单时钟组合与小型寄存器流组件，不包含原生层级仿真、通用 FIRRTL 内存、异步或深 RAM FIFO，也不包含后续完整系统或外部核交付。emit、真实 RTL 执行、Rust 执行、综合和形式证明须分别记录；工具缺失不得静默当作通过。保存命令、工具版本、种子和结果。性能与 PPA 宣称必须有实测。

ready/valid 在上升沿两者均有效时传输一次；受阻时保持 valid 和 payload。reset 丢弃在途数据，须明确计数边界。以独立参考期望及属性验证流行为，适用时包含真实 RTL 随机流和小状态形式检查。

## Technical Decisions

复用既有 `ElaborateSession`、`FrozenHir`；模块定义 helper 由新组合路径和旧 elaborate 入口共用。旧入口创建 session、调用定义 helper，再 finish 一次。禁止拼接冻结电路或另立 IR。顶层与全部子模块属于同一 session，最终仅 finish/freeze 一次。

不得隐式覆盖尚未结束的模块定义。顶层可以先定义并结束，再定义子模块；顶层名显式等于电路名。相同参数可复用同一定义，不同参数须有不同且受控的标识。参数指纹冲突、重复模块名、缺模块、实例环、端口宽度或方向错误，均须在 freeze 前通过既有 Diagnostics 报告。多个实例的局部名称须隔离，包括同名局部信号。层级修改限新组合路径必需部分。

层级行为以真实生成 RTL 验收。native/generated 仿真对不支持的多模块系统须明确拒绝；添加名称前缀不能代替层级语义。内部 reset 为同一时钟域内的同步高有效复位。

`RvRegSlice` 为两槽；输出 valid/data 和输入 ready 均由寄存状态驱动，无输入到输出组合路径。稳态可每拍传输一笔，满后恢复允许一个等待拍，不承诺所有情况下零气泡。

`ParamSyncFifo` 支持 WIDTH 1–64、DEPTH 1–16，包含非二次幂深度；采用寄存器，不承诺块 RAM 推断。无空直通，入队数据最早下一周期可出队。满时 input_ready 为0，即使同拍 pop 也不接受 push；空时 output_valid 为0；非满非空时支持同拍 push/pop。优先级为 reset、flush、正常传输；清占用与有效状态，不承诺清零全部 payload 存储，无效 payload 不作为功能结果。越界参数明确报错，不裁剪，也不在 panic 后继续生成。若内部共用实现，仍须保留旧 `SyncFifo` API 及8×4的延迟、full/empty 和输出行为。

## Cross-Story Dependencies

Story126.1 依赖已完成的 Story125.3/M0。Story126.2 和126.3 各依赖126.1；Story126.4 同时依赖两者。只有全部故事完成，且风险关闭记录、需求映射和实际支持矩阵一致，才可关闭 M1。FIFO 验证覆盖宽度1/8/32/64与深度1/2/3/4/7/16，以及空/满、同拍传输、reset/flush、非法参数和旧行为兼容。

这些基础供后续 CSR/桥及外设系统使用，外部适配也依赖可用的模块组合。上述下游交付及 Story126.3–126.4 均不在本轮执行126.1–126.2的授权范围内。

## 当前整体执行授权（2026-09-20）

用户明确要求按create-story→ATDD→build→code-review→automate→clean/fmt/regression→commit七步连续处理sprint-status全部未完成Story。此授权取代旧的M0-only或仅126.1/126.2执行安排；各故事仍须满足自身依赖、NFR14及真实验证，一故事一提交。已有done保留；deferred不得假交付。122.2/122.3在2026-09-20实时核验后仍因没有已发布firtool>1.159.0而阻塞；继续其他17个可执行故事，待前置满足后再恢复FR189。整体目标尚未完成，不推送或发布。

## 126.3 实现连续性（2026-09-20）

RvRegSlice<WIDTH=32>已实现，宽度1..64，两个payload寄存器+2-bit占用；input_ready/output_valid/output_data只依赖寄存态，空无直通，满时不接收，即使同时pop。reset>flush>正常传输，取消沿的表面握手不记账。新helper登记WIDTH，旧SyncFifo未改。注册切片原始设计、SBY observer安全归纳和四条cover证据见test-artifacts/automation-126-3.md；这是FR195子集，非FIFO或M1关闭。

126.4应复用共享session定义机制、独立端口队列和真实RTL工具入口；参数WIDTH1..64、DEPTH1..16（含非二次幂），矩阵宽1/8/32/64×深1/2/3/4/7/16。native层级仍unsupported。顶层只能一个Reset输入，双实例取消隔离用共享rst+独立flush；不要再次生成多个Reset端口。当前用户授权七步连续执行全部未完成故事，每故事一提交，提交前clean全回归。

126.3最终收尾：七步完成，本故事提交；clean+fmt+just test退出0（1732通过、8忽略），专用formal实际通过。126.3 done；126.4仍待实现，Epic126/M1未关闭。证据索引test-artifacts/126-3-final-verification.md。

## Story126.4 / M1 最终关闭（2026-09-20）

2026-09-20 Story126.4七步完成，随本故事单独提交关闭Epic126/M1：126.1–126.4均done，FR194模块组合与FR195两槽注册切片/参数FIFO交付。clean + fmt + just test退出0，460个实际结果块汇总1763 passed、0 failed、14 ignored；8个专用形式测试另有真实运行证据，6个既有doc-test保持ignored。最终证据 `_agile-output/test-artifacts/126-4-final-verification.md`，关闭映射 `_agile-output/implementation-artifacts/epic-126-closeout.md`。Epic127–130 / FR196–201尚待各自七步与NFR14，整个Phase24未完成；FR189/Epic122 deferred及NFR91保持，不push、不publish。

风险关闭：非法参数在硬件定义前诊断、全参数身份、非二次幂寄存器容量、取消控制×占用、在途共享reset隔离恢复、旧SyncFifo RAM/延迟、真实RTL/归纳证明/综合及文档例均有本故事证据。支持宽1..64深1..16；行为24配置×3seed，形式W1×D1/2/3，综合1×1/8×3/64×16。无PPA、BRAM、异步/深RAM FIFO或native层级支持宣称；126.2既有名称/Instance.params遗留仍在deferred-work，不宣称账本清空。
