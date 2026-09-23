---
title: 'Story 130.3：一个真实外部核适配及试点关闭'
type: 'feature'
created: '2026-09-23'
status: 'done'
route: 'dispatch'
review_loop_iteration: 1
baseline_commit: '4152610f29e8bb242bba789e3ba3f53376eb21fc'
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/130-3-一个真实外部核适配及试点关闭.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-130-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-130-nfr14.md'
  - '{project-root}/_agile-output/implementation-artifacts/spec-130-2-source-lock-offline-replay.md'
  - '{project-root}/ip/external/pulp-common-cells-fifo-v3.source.json'
  - '{project-root}/ip/external/pulp-common-cells-fifo-v3.source.lock.json'
  - '{project-root}/crates/bitloom/src/external_ip.rs'
  - '{project-root}/docs/ip/phase24-support-matrix.md'
---

<frozen-after-approval reason="已批准的 Story 130.3 意图；实现不得扩大到未声明协议、发布或其他外部核">

## Intent

**Problem:** Story 130.2 已交付固定来源闭包与离线重放，但 Bitloom 仍没有一个真实外部 RTL 核的可执行 module binding、参数/端口映射和独立行为证据，FR200 仍不可声称完成。

**Approach:** 只适配已锁定的 PULP `common_cells` stable `v1.40.0` `fifo_v3`。在现有 CLI/构建层读取 manifest/lock/cache，生成版本化且可审计的 wrapper binding 元数据；以真实锁定 FIFO RTL 参与综合/仿真，并用独立队列模型驱动组合测试，同时运行上游可获得的 FIFO 相关测试或明确记录工具/环境阻塞。只有 module binding、源关联、参数映射、reset/clock、独立行为和禁网重放全部通过，才把支持矩阵提升到 `behavior-tested`；否则保持未交付并记录失败。

**Always:** 保持第三方源码在缓存/外部 IP 层，不进入 HIR；设计 crate 只依赖 `bitloom-prelude`；wrapper 版本与 upstream 版本分开；所有参数、端口、clock/reset 和 source hash 必须来自 lock 并在运行时校验。

**Never:** 不用空壳、零输出、compile-only、目录登记或上游自测替代独立行为；不自动换 master/Taxi；不实现复杂协议或多核组合；不改工具 pin、Rust MSRV、firtool/Chisel 产品 pin、包版本或既有 API；不关闭 FR189/NFR91 或宣称整个 Phase24 完成。

## I/O & Edge-Case Matrix

| 场景 | 输入/状态 | 期望行为 | 失败条件 |
|---|---|---|---|
| Binding | canonical manifest/lock/cache + `fifo_v3` | 输出稳定 wrapper/binding 描述，module、source file、参数、端口宽度/顺序、clock/reset 与 lock 一致 | lock/cache/hash/shape 漂移、未声明源或参数时非零失败，不自动修复 |
| Reset/clock | 独立测试驱动 reset polarity/kind 与 clock | reset 后 FIFO 空，释放后按单时钟 push/pop | 极性错误、重复释放、跨时钟或未知 reset 时失败 |
| FIFO behavior | 独立入队模型、真实 RTL、有限随机/定向序列 | valid/ready、满/空、同时读写、深度/数据宽度参数和顺序与模型一致 | 使用 DUT 自身行为作 oracle、丢数、重复出数、越界或不一致 |
| Offline replay | copied lock/cache + isolated checkout + no network | wrapper 与行为测试只读取复制产物并完成真实 RTL parse/compile/sim | 访问主工作区、宿主缓存、网络或浮动来源时非零失败 |
| Support promotion | all prior evidence + owner/CI/support docs | 仅在证据完整时更新 `behavior-tested`；否则保持 `not delivered`/`locked` | 任一门禁未运行、ignored/UNKNOWN 当 PASS、FR201 核心证据越界 |

</frozen-after-approval>

## Code Map

- `crates/bitloom/src/external_ip.rs` -- 在现有 source lock/verify/replay 入口上增加私有 binding/behavior-test orchestration；复用既有 hash、路径和 fail-closed 约束。
- `crates/bitloom/src/main.rs` -- 如需新增命令，仅增加私有/稳定 CLI 路由和错误分类，不破坏 cargo-subcommand 参数归一化。
- `crates/bitloom/tests/fr200_external_ip_binding.rs` -- ATDD：binding schema、lock/source/parameter/port/reset 校验和负测。
- `crates/bitloom/tests/fr200_external_ip_behavior.rs` -- 独立模型驱动的 FIFO 行为测试；oracle 不得调用 DUT 内部状态或输出回灌。
- `scripts/phase24-external-ip-pilot.py` -- 清缓存获取产物的复制、隔离 replay、行为测试与证据汇总；不得复制 Rust 来源解析逻辑。
- `ip/external/pulp-common-cells-fifo-v3.binding.json` -- canonical wrapper/module binding 与经过验证的参数/端口映射。
- `docs/ip/phase24-support-matrix.md` -- 仅在真实证据通过后提升 External IP pilot 支持级别，保留 FR200/FR201 边界。
- `_agile-output/test-artifacts/130-3-*` -- ATDD、上游/独立行为、offline replay、负测和工具身份原始证据。

## Tasks & Acceptance

**执行任务：**

- [x] 读取并核验 lock 中 `fifo_v3` 的真实 module、参数、端口、include、宏、clock/reset 与 source hash；拒绝任何 manifest/lock/cache 漂移。
- [x] 实现 binding descriptor/wrapper 适配，明确 upstream identity、wrapper version、source file、参数专门化、端口方向/宽度/顺序和 reset/clock 映射。
- [x] 实现独立 FIFO reference model 和真实 RTL 组合测试，覆盖 reset、空/满、同时读写、参数宽度/深度、valid/ready 边界；oracle 不复用 DUT 行为。
- [x] 在复制 cache 的禁网隔离环境执行 binding、真实 RTL parse/compile/sim；运行上游可用测试，工具缺失或网络隔离失败必须保留非零证据并保持未交付。
- [x] 为每个 P0 负测加入普通/优化模式覆盖：source/hash/port/parameter/reset/cache/network/tool 漂移、未声明源和空壳 wrapper。
- [x] 仅在全部证据通过后更新 support matrix 与 Epic130 closeout；否则明确停止原因，不把 FR200/behavior-tested/maintained 写成完成。

**验收条件：**

- Given 130.2 lock/cache 和 126.2 组合基础，When binding 命令执行，Then 产生确定性 binding JSON，且所有 module/source/参数/端口/clock/reset 字段与 lock 和真实 `fifo_v3` 一致。
- Given 独立 reference model 和真实锁定 RTL，When 运行定向及有限随机序列，Then reset、空/满、同时读写、宽度/深度和握手行为一致；测试不读取 DUT 内部状态作为 oracle。
- Given copied cache、隔离 checkout 和禁网环境，When 执行 pilot replay，Then 实际消费锁定 FIFO RTL 完成 parse/compile/sim，并记录命令、工具、hash、退出码、耗时和来源身份。
- Given 任一 P0 mutation，When binding/replay/behavior gate 执行，Then 确定性非零失败，canonical manifest/lock/cache/binding 不被改写。
- Given 全部真实证据通过，When 审计支持矩阵，Then 只提升 External IP pilot 对应等级并关闭 130.3；FR189、NFR91、复杂外部核和整个 Phase24 仍按现状记录。

## Verification Plan

先运行 ATDD 红阶段，再实现后执行专用 binding/behavior 测试、脚本 replay 和负测；最后运行 `cargo clean && cargo fmt --all && just test`。联网、bubblewrap、Yosys 或上游测试不可用时必须保留原始失败证据，不能以 ignored、静态 JSON 或目录存在替代 PASS。

## Design Notes

130.2 的 `bitloom-yosys-sv-compat` 只证明锁定 FIFO 可被工具消费，不自动证明 wrapper 或行为；本故事必须新增独立 binding 和模型驱动行为证据。优先保持现有私有 CLI/脚本边界，避免公共 API 扩张；任何新增符号都按 FR142/SemVer 规则登记。

## Spec Change Log

## Review Triage Log

## Status

done；code-review、automation、独立前置提交与真实回归均已完成，证据由本 Story 单独提交归档。

### Review Findings（2026-09-23）

四层审查全部返回。下表保留每层发现的编号；同一根因合并修复，但每条发现均保留证据。上述问题均为已授权范围内的实现修复，不 defer 验收条件。

- [x] [Review][Patch][B1][high] 模拟器身份缺失：behavior 通过 PATH 调用 iverilog/vvp，tool_lock 未包含两者。
- [x] [Review][Patch][E3][high] 模拟器身份缺失：behavior 通过 PATH 调用 iverilog/vvp，tool_lock 未包含两者。
- [x] [Review][Patch][A2][high] 模拟器身份缺失：behavior 通过 PATH 调用 iverilog/vvp，tool_lock 未包含两者。
- [x] [Review][Patch][B2][high] 普通测试依赖临时缓存：新增非 ignored 测试默认依赖 /tmp/bitloom-130-2-transported-cache，CI 无准备入口。
- [x] [Review][Patch][E5][high] 普通测试依赖临时缓存：新增非 ignored 测试默认依赖 /tmp/bitloom-130-2-transported-cache，CI 无准备入口。
- [x] [Review][Patch][V3][high] 普通测试依赖临时缓存：新增非 ignored 测试默认依赖 /tmp/bitloom-130-2-transported-cache，CI 无准备入口。
- [x] [Review][Patch][B3][high] 隔离挂载泄露宿主文件：bwrap --ro-bind / / 保留整个宿主可读。
- [x] [Review][Patch][E6][high] 隔离挂载泄露宿主文件：bwrap --ro-bind / / 保留整个宿主可读。
- [x] [Review][Patch][A3][high] 隔离挂载泄露宿主文件：bwrap --ro-bind / / 保留整个宿主可读。
- [x] [Review][Patch][B4][medium] 离线测试未执行编排：ignored 测试仅直接调用 replay --compile，无复制、绑定或行为断言。
- [x] [Review][Patch][V2][medium] 离线测试未执行编排：ignored 测试仅直接调用 replay --compile，无复制、绑定或行为断言。
- [x] [Review][Patch][B5][medium] 依赖版本前缀误匹配：inline contains(version: 0.2.0) 接受 0.2.01，未限定 dependencies 段。
- [x] [Review][Patch][E1][medium] 依赖版本前缀误匹配：inline contains(version: 0.2.0) 接受 0.2.01，未限定 dependencies 段。
- [x] [Review][Patch][V4][medium] 依赖版本前缀误匹配：inline contains(version: 0.2.0) 接受 0.2.01，未限定 dependencies 段。
- [x] [Review][Patch][B6][medium] 负测可在错误层失败：只改 manifest 未同步摘要，且仅断言非零。
- [x] [Review][Patch][B7][medium] 模拟器无超时：iverilog/vvp 使用裸 output，不能终止挂起。
- [x] [Review][Patch][E4][medium] 模拟器无超时：iverilog/vvp 使用裸 output，不能终止挂起。
- [x] [Review][Patch][B8][medium] 缺少有数据时复位：TB 仅空队列启动复位。
- [x] [Review][Patch][E2][medium] 来源顺序影响绑定：URL/ref 取 sources[0]，commit 取 common_cells，允许合法重排后混合。
- [x] [Review][Patch][V1][medium] 绑定输出断言不完整：未校验全部参数、完整端口宽向与来源 hash。
- [x] [Review][Patch][A1][high] 未实现真实 Bitloom 组合绑定：binding 为 JSON；TB 直接实例化上游，不经过现有 HIR 组合/后端。
- [x] [Review][Patch][A4][high] P0 与上游验证缺失：完整负测及上游 FIFO 测试尚未运行。
- [x] [Review][Patch][A5][medium] 支持矩阵提前提升：NFR14 规定在 130.3 验收前保持外部行未交付。

Rejected：无。初轮审查时保持 in-progress；原阶段 final-verification 不证明关闭，当前最终结果见末尾交付记录。

## Implementation clarification（源合同落实）

依 NFR14 的单一冻结参数集：DATA_WIDTH=32、DEPTH=8、FALL_THROUGH=0；现有 HIR Instance 可记录数值参数但后端未发参数 override，因此必须核验真实源码默认值完全相等，不能声称任意参数支持。使用已有端口-only 外部声明与共享 session 实例，真实 Bitloom 发射父模块在 CLI 链接时去除经精确验证的 opaque stub，接入锁定上游源；不得改写父模块成手工 SV。新增 binding 命令必须输出并绑定该真实父模块的内容与 hash，behavior 必须消费同一路径。以上落实 AD-31，不扩 HIR/public API。

## 修复后独立复核（2026-09-23）

最终相对 FR199 独立快照复核未发现阻断项。23 条发现均有实现和实际证据对应：真实 HIR 父模块与空/零输出 mutant；完整绑定、来源和工具/helper 身份；只读复制 cache/host 路径与网络负测；精确 Bender 声明；有数据异步复位；有界工具进程；normal/-O 各 22 产品负测与独立消费者各 18 负测；上游六组各 100000 比较。复核时尚待 FR199 提交；现已由最终交付记录确认前置完成并提升支持矩阵。

完整 workspace 已退出 0（487 块，1893 passed / 0 failed / 55 ignored）；继承 FR199 的直接 compile-contract 单测随后单独复跑（11 passed / 1 ignored），fmt 检查最终通过。首次无效 panic scaffold 与一次末尾空行 fmt 失败都不反写为成功；详见最终验收记录。

## 最终交付

Story130.2 前置已在 4152610 关闭。最终 canonical lock 为49b22842deda1d09bb86baaa5c8c3474f013a52c7d587f624f827705b5141d6e，绑定实际 CLI 与 Icarus helper；来源210文件未变。最终 normal/-O canonical consumer 各11通过。六组上游默认100000次的并行调度运行通过，不把1000次诊断、900秒超时或补充单进程取消计PASS。支持等级及Epic关闭映射见epic-130-closeout.md；FR189/NFR91与整个Phase24的宣称边界保持。
