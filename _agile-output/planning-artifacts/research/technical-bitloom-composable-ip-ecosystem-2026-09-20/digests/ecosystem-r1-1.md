# 外部 IP 生态与维护边界：第一轮证据摘要

访问日期：2026-09-20。未读取项目代码或规划文件；任务上下文只用于形成问题。只采用本轮读取的官方来源；搜索摘要不独立作为结论依据。本轮 4 次 web 调用，8 个实际打开的独立来源，达到来源 round cap。

## 来源表

| ID | URL | publisher | pub_date | accessed | 类型/时效 |
|---|---|---|---|---|---|
| S1 | https://fusesoc.readthedocs.io/en/latest/ref/capi2.html | FuseSoC | 未知 | 2026-09-20 | 官方当前参考；页面标识 2.4.8.dev1+g22f03f84c，非稳定版承诺 |
| S2 | https://fusesoc.readthedocs.io/en/latest/user/build_system/dependencies.html | FuseSoC | 未知 | 2026-09-20 | 官方当前依赖说明 |
| S3 | https://pulp-platform.github.io/bender/dependencies.html | PULP Platform / Bender | 未知 | 2026-09-20 | 官方当前依赖说明 |
| S4 | https://github.com/lowRISC/opentitan | lowRISC | 未知 | 2026-09-20 | 官方仓库当前 README 与文件目录 |
| S5 | https://raw.githubusercontent.com/lowRISC/opentitan/master/LICENSE | lowRISC（仓库载明文本） | 文件发布日期未知；文本版本 January 2004 | 2026-09-20 | 当前分支许可证文本 |
| S6 | https://github.com/fpganinja/taxi | FPGA Ninja | 未知 | 2026-09-20 | 官方仓库当前 README |
| S7 | https://raw.githubusercontent.com/fpganinja/taxi/master/LICENSE | FPGA Ninja（仓库载明 CERN 文本） | 文件发布日期未知；文本版权年 2020 | 2026-09-20 | 当前分支许可证文本 |
| S8 | https://github.com/lowRISC | lowRISC / GitHub 元数据 | 未知；仓库 Updated Sep 20, 2026 | 2026-09-20 | 官方组织页近期活动快照 |

日期未知不替换成抓取日期。当前页面支持访问时的文档状态，不能据此补造发布历史或已验证版本矩阵。许可证文本的年代不等同于项目不活跃。

## 可追溯事实

以下 claim 中 source ID 的 URL、publisher、pub_date、accessed 继承上表。confidence 是“对来源确实这么写的把握”，不是集成成功概率。

| Claim | 来源 | class | confidence | 支持摘意与边界 |
|---|---|---|---|---|
| E1：CAPI2 能描述源码、依赖、构建入口及生成器 | S1 | primary-spec | high | schema 有 filesets/depend、targets、parameters、provider、generate/generators。target 选用文件集、生成器和工具配置；依赖使用 default target。生成器是外部程序，可指定解释器及 none/input/generator 缓存方式。 |
| E2：源码来源与许可证可以作为元数据表达 | S1 | primary-spec | high | provider 包含 git/github/url/local 等及版本/缓存相关字段；license 可为 SPDX 字符串或自定义 name/text。该页 URL provider 的字段中未见内容 checksum 字段；不据此断言整个 FuseSoC 没有校验能力。 |
| E3：CAPI2 版本约束和文件编译序有明确规则 | S2 | primary-spec | high | 依赖写在 filesets.*.depend，可使用 =、比较、^、~；无版本接受任意版本。依赖文件先于使用者，但 depend 列表次序不构成依赖间编译顺序。 |
| E4：依赖 default target 不继承整个工具配置 | S2 | primary-spec | high | 继承 filesets/hooks/generate/parameters/vpi；不继承 tools/toplevel/default_tool。SemVer 为作者约定而非工具证明兼容；这不构成任何具体兼容/失败实验。 |
| E5：Bender 支持 Git 版本范围、revision 与本地路径 | S3 | primary-spec | high | Git SemVer 标签识别 vX.Y.Z；rev 可指 commit/branch/tag。update 解析依赖树并在 Bender.lock 写入精确 Git hash。target 条件影响源码列表，不排除依赖解析/lock。路径依赖不版本化。 |
| E6：OpenTitan 是硬件/软件/工具合并的 monorepo | S4 | primary-description | high | README 明确 monorepo；目录含 hw、sw、third_party、toolchain，以及 Bazel/锁文件等。适配所需具体 IP 闭包尚未解析；不能称为独立单文件 IP。 |
| E7：OpenTitan 顶层许可证及例外需分别记录 | S4,S5 | license-identification-only | high | README 限定 unless otherwise noted；LICENSE 标题原文为 “Apache License / Version 2.0, January 2004”。SPDX 记录候选 Apache-2.0 由仓库标识支持；未审计子目录、NOTICE 或第三方依赖，不作全依赖许可结论。 |
| E8：Taxi 提供多类 SystemVerilog 通信组件和测试入口 | S6 | upstream-self-report | high | README 列 AXI/AXI stream/Ethernet/PCIe 等，测试使用 Cocotb/Verilator，可经 pytest/tox/makefiles。上游自述项目在开发中，Corundum/Zircon 可能未适合生产。没有读到具体版本矩阵；性能数字不采信为独立验证结果。 |
| E9：Taxi 的许可证必须按当前项目记录 | S6,S7 | license-identification-only | high | LICENSE 标题原文为 “CERN Open Hardware Licence Version 2 - Strongly Reciprocal”；仓库标识 CERN-OHL-S-2.0，README 另述付费 commercial license 选项和组件例外。这里只记录上游文本，不解释其对 Bitloom 或全设计的法律效果。 |
| E10：OpenTitan 存在近期维护活动信号 | S8 | official-metadata-snapshot | medium | 读取组织页显示 OpenTitan Updated Sep 20, 2026。初次搜索缓存显示 Sep 19，说明只能当动态快照；不是发布版本日期、提交质量或 SLA 证据。 |

## 由证据提出的设计建议（不是当前实现事实）

| 交付方式 | 建议维护/发布责任 | 使用边界与支持声明 |
|---|---|---|
| 原生实现 | 独立版本的 Rust IP 包、原生参数/端口 API、测试与生成产物；维护者承担功能规格与实现验证 | 用于希望原生类型/生成器体验的精选组件。成本和质量收益未量化，不能宣称必然优于 RTL 封装。 |
| 第三方 RTL 封装 | wrapper 包与上游源码各自版本；维护参数映射、端口/时钟/复位约定、完整源列表、构建适配、上游测试与组合测试 | 可借 E1–E5 的 manifest/lock 模式；只有已运行的工具/参数组合可称“已验证”。原生仿真模型与 RTL 语义对应需单独证明。 |
| registry/manifest only | 发布索引条目、来源指针、锁定版本、工具要求、许可证证据及验证状态；不默认承担 RTL 功能正确性 | 适合先扩大可发现范围；“已收录”不得等同“已适配/受支持”。manifest 可引用已有 FuseSoC/Bender 闭包，避免人工复制依赖语义。 |

建议统一 release record：upstream URL + immutable commit；所有递归依赖/submodule 的 resolved commit；下载归档 SHA-256；wrapper/manifest 自身版本；许可证原文位置与 hash、文件级例外/NOTICE；生成器版本与参数、产物 hash；准确工具版本和测试证据链接。E2 的字段描述不足以保证所有这些要求，需要外部锁定/证据层；E5 是“版本意图与实际解析版本分离”的官方先例。

建议提供显式 fetch 与离线验证步骤：先获取完整源码/生成器依赖并校验内容，再在禁网环境构建。缓存键包含锁定依赖、工具链、参数与生成器输入。缓存存在不等于离线可重建；本轮没有跑断网实验，FuseSoC/Bender 原生 offline/cache 语义仍是缺证项。

建议上游升级通过候选分支 CI：比较 manifest/端口/参数/许可证变化；运行原上游测试、wrapper 编译、协议与复位/背压测试、代表参数组合；再升级锁定版本。矩阵需逐项记录模拟器、综合器、SystemVerilog 特性、Python/Cocotb、生成器、平台/器件版本及最后通过日期。当前没有证据可填任何 Bitloom × Taxi/OpenTitan 的绿色组合。

建议成熟度用 observed evidence 分级：catalogued → fetched-and-locked → compiled → behavior-tested → maintained。每一级附精确范围和日期。无人维护、上游终止、工具不再支持或重复回归时，可降级/弃用条目，冻结最后已验证版本、说明替代路线和结束支持日期；保留历史锁定记录，禁止静默换源。以上均为建议策略，不是已证明最佳实践。

## 两个接入可行性样例（仅候选）

1. **Taxi：先选单个 AXI-stream 组件作为 RTL 封装试点。** E8 支持存在该组件类别及测试栈；建议记录并封装精确接口/参数与源闭包后跑上游测试。许可证证据为 E9，不能把其他同作者项目的许可证沿用到 Taxi。实际模块名、参数边界、接口 flatten 要求、综合器支持、维护时间线尚未验证；不承诺可直接适配。
2. **OpenTitan：先 registry 记录，再选择单个 IP 提取闭包验证。** E6 支持 monorepo 属性，故建议保持 upstream 路径与版本，先确定生成步骤、primitive/总线/软件关联依赖，再讨论 wrapper。E10 只有最近更新时间信号；不能从项目品牌推出所有 IP 稳定或容易抽取。具体候选模块、构建目标、DV 依赖及跨工具通过状态仍缺证。

## 缺证、追查与停止理由

- **compatibility/performance/failure 一律未验证：** 本轮没有来自不同 publisher 的两来源确认，也没有独立执行证据。上游自述测试框架只能说明测试入口；没有证明与 Bitloom 兼容。
- **维护时间线未完成：** OpenTitan 只有 Sep 20 动态更新时间；Taxi README 声称持续开发，页面未提取到有日期的 release/commit 序列。应下一轮读 commits/releases 与近期 CI 结果；不使用 stars 或累计 commit 数推断质量。
- **版本时效：** 本轮当前页面符合近期检索，但 pub_date 未知；未建立一个月内验证的工具组合。技术模式有当前官方规范支持，未补造两年内发布日期。
- **进一步读取优先级：** FuseSoC library/provider/cache 与稳定版本 changelog；Bender offline/checkout 配置及发布说明；Taxi 实际一个模块、测试锁定版本和 workflow；OpenTitan 一个 IP 的 core/build、工具 pin、license/NOTICE 闭包。第一轮不继续扩展来源。
- **停止理由：round cap。** 8 个实际读取来源已覆盖 manifest/lock 模式、两个项目入口、许可证原文和一个近期活动信号；具体适配/性能/兼容与离线重建不足，明确保留为未验证，而非继续无限检索。
