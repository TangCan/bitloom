# Story 127.2: CSR 描述、RTL 与软件地址产物

Status: done

## Story

作为实现/集成维护者，我需要一份静态 Rust CSR 描述生成可组合的真实 RTL、地址文档和 C 头文件，使软件与硬件使用相同地址合同，并让后续外设通过单一状态所有权、动态状态与一次提交接口接入 FR196。

估算4–6有效人日。前置127.1、126.2均done；基线提交 `77ea01392e5be74b9f2622009ad146edb2468ea9`。本故事只交付FR196的描述/叶节点/软件产物子集；127.3桥、127.4静态译码与M2关闭、Epic128外设另行验收。当前全部剩余故事七步串行执行授权优先于历史M0-only注记，不需再次批准既定工作。

## Acceptance Criteria

1. **静态描述与错误诊断。Given** Rust静态寄存器描述，**When** 校验、elaborate或生成软件产物，**Then** 同一规范化描述包含寄存器/字段名称、local byte offset、mask、reset、RW/RO/WO/W1C访问、状态owner和事件连接身份；首版固定32位数据、16位地址、4位WSTRB、四字节对齐、reset0、每寄存器单访问类型。重复/重叠地址、超界、未对齐、重叠/零/越界字段mask、不一致reset、非零reset、不合法owner/事件组合、混合字段权限均以现有Diagnostics拒绝，不panic、截断、自动改名或静默降级。HDL/C标识符、保留字、生成端口/内部名字和大写C宏碰撞均检查。
2. **共享定义与完整身份。Given** 已验证配置，**When** 调用独立elaborate或同session define_module，**Then** 共用一个定义体、只由最外层finish一次；非捕获fn接受完整规范参数并重建配置，所有名称/字段/访问/owner/事件均参与身份。相同配置和名称可复用；只改名称、mask、事件或owner仍有不同参数身份，同名不同配置E0244且session最终finish失败。未知schema、重复键、缺索引、错误count/枚举/字节均拒绝。配置预校验失败在进入helper前不分配半模块，之后同session合法配置仍可工作；不得要求helper失败后的poisoned session恢复。
3. **唯一提交与响应。Given** clk、同步高有效rst及CSR通道，**When** 非reset上升沿req_valid&&req_ready，**Then** 恰好提交一次；该沿锁存rdata/error快照，下一周期rsp_valid可见。只容纳一笔已提交未消费请求；rsp受阻保持valid/rdata/error且不得新提交。访问副作用发生在提交沿，不能在响应/ready反复变化时重放。reset优先，取消待响应，所有拥有的CSR状态清0，恢复首拍可再接受；读错误rdata0，error只有00/10（叶节点不生成01；系统DECERR由127.4产生）。
4. **字段黄金行为。Given** 独立手写地址和期望，**When** 覆盖全部16种WSTRB、有效/保留位、权限/地址错误和事件碰撞，**Then** RW按小端逐字节合并并屏蔽保留位；RO写、WO读、local地址空洞/未对齐均SLVERR且无访问副作用。合法RW/W1C/WO零WSTRB返回OKAY无访问副作用，RO零WSTRB写仍错误。W1C `next=(old & ~clear)|event`，clear只含有效且选中字节的写1位，硬件set胜清除；失败访问或背压不得阻止独立硬件事件累积。读快照为提交前值，同拍新事件在之后读取可见。
5. **动态端口与外部owner。Given** 测试peer提供RO数据、W1C事件、读/写动态拒绝以及外部RW当前状态，**When** 请求提交，**Then** RO只锁存提交前快照，成功读/写脉冲仅一次，拒绝有限延迟SLVERR且无pop/push/写入。写拒绝基于候选合并值和提交前状态；有效写字节为空时不触发动态busy/full错误（仍先检查地址/权限）。低8位WO未选byte0即无push且OKAY。外部RW读回peer当前值，写输出基于该值合并的候选值和有效位mask，成功提交脉冲供peer在同沿更新；leaf没有该寄存器第二份存储。peer用自主递增/别名修改与部分软件写冲突证明单owner可用；不提前实现Timer/GPIO/UART产品。
6. **软件产物同源、确定且能使用。Given** 同一有效描述（含输入列表不同顺序），**When** 重复生成RTL、Markdown地址表和C头文件，**Then** 规范顺序、稳定内容且无时间/路径随机性。C产物明确是local offset而非全局物理地址，含字段mask/reset/访问及稳定命名空间/include guard，32位常量安全；手写黄金地址/常量校验，最小C消费者及两个不同block头文件共同include均以严格编译成功。不是只对比生成器自己生成的oracle。
7. **真实验证与可组合性。Given** native单模块和真实生成RTL，**When** 定向及随机协议合法刺激、backpressure/reset和peer组合，**Then** 独立scoreboard逐拍核验上述行为及接受/提交/响应/取消计数。至少一个真实同session顶层实例化CSR leaf与外部状态peer；native/generated遇层级仍明确unsupported。实际CSR RTL另跑安全proof（提交一次、响应保持、错误无访问副作用）与非空提交/错误/背压恢复cover；原始未加observer的RTL通过Yosys综合/check。安全不假设rsp_ready最终为1；cover不冒充proof，有限深度与归纳结论分别记录。工具缺失/超时必须失败，不skip为通过。
8. **可用入口、兼容与关闭诚实。Given** 中文使用文档与prelude-only可编译示例，**When** 完成review、automate、clean/fmt/just test和单故事提交，**Then** 新公开符号逐项登记FR142与SemVer minor说明，旧Axi4LiteSlave/SyncFifo/Gpio和M1接口保持；保存实际命令/版本/seed/结果及限制。仅127.2 done，Epic127仍in-progress，FR196/M2/Phase24未关闭；FR189/Epic122 deferred和NFR91保留，不push/publish。

## Tasks / Subtasks

- [x] T1 ATDD先行并确定API（AC1–8）。
  - [x] 建立清单、独立黄金配置/地址/期望、配置诊断与CSR时序红测；缺API编译红与实际行为红分别记录。
  - [x] 将下述建议API/端口表收敛为确切公开签名，再由所有测试和文档使用同一契约。
- [x] T2 静态描述/合法性/参数身份（AC1、2、6）。
  - [x] canonicalize、完整encode/decode与负例；校验先于session变更与产物输出。
  - [x] 共享无捕获定义体与独立入口，不修改通用builder契约。
- [x] T3 CSR leaf与动态peer（AC3–5）。
  - [x] 单响应寄存槽、提交快照、reset优先；RW/RO/WO/W1C与全部mask。
  - [x] 单owner、无环候选写值/有效mask、动态拒绝、读写提交pulse、事件输入；真实peer同沿验收。
- [x] T4 软件生成物和用户入口（AC6、8）。
  - [x] Markdown/C头文件确定性、手写黄金、两头共同编译；prelude-only完整示例。
  - [x] docs/ip索引、CSR文档、FR142逐符号追加；合同仅追加本故事真实交付状态。
- [x] T5 产品验证（AC7、8）。
  - [x] native定向/随机和真实RTL/层级peer；场景计数及种子，缺工具硬失败。
  - [x] CSR专用formal prove/cover、无observer RTL综合；旧bank和M1兼容回归。
- [x] T6 七步闭合（AC8）。
  - [x] 独立code-review修复后automate补有价值边界；最后clean、fmt、`just test`。
  - [x] 记录真实证据，更新本故事/sprint，主代理执行单故事commit；不提前开127.3实施。

## Dev Notes

### ATDD Artifacts / 确切 API（2026-09-21）

- Checklist：[127.2 ATDD](../test-artifacts/atdd-checklist-127-2-csr-描述-rtl-与软件地址产物.md)。
- 确切公开签名、字段、owner/event合法组合、裁剪端口、沿前pulse与软件宏命名：[API契约](../test-artifacts/127-2-atdd-api-contract.md)；取代本故事下文“建议”中的待定名字，不改变AC/范围。
- API/config/C11：`crates/bitloom/tests/fr196_csr_config.rs`；native/真实RTL/同session peer：`crates/bitloom/tests/fr196_csr.rs`；独立formal+原始综合：`crates/bitloom/tests/fr196_csr_formal.rs`。
- ATDD历史时点（当前build结果见文末）：当时仅红阶段；缺API编译错误不证明任何行为。专用formal采用既有CI `--ignored` 运行边界，必须真实执行且接入formal-sby job，ignored绝不记作PASS。私有codec畸形输入由build在产品模块内补单元测试。


### 可实现API与端口建议（ATDD阶段定名，实施不得弱化语义）

建议在 `bitloom_prelude::ip` 导出 `CsrAccess`、`CsrOwner`、`CsrField`、`CsrRegister`、`CsrBlock`。配置是host/elaboration数据，可用String/Vec或静态slice；没有tick运行时对象。`CsrBlock`提供validate、`define_module(&mut ElaborateSession, name)`、`elaborate(name)`及返回Result<String,Diagnostics>的地址Markdown/C头生成方法。配置依赖实例，不能硬塞进无参`Elaboratable::elaborate()`；使用显式配置入口即可。内部编码/解码保持私有并单元测试，不无端扩FR142。

每寄存器field各有名称、mask、访问声明，要求与寄存器访问相同；有效mask为字段并集。owner至少区分leaf与external（RW；外部W1C若暴露需完整同沿set优先测试）；RO/WO不创建CSR可写状态。描述显式标识W1C事件连接，生成接口可用规范索引或受校验名字，必须文档化如何从配置查到端口。非法组合要拒绝，不能忽略字段。首版reset全0，非零即诊断。

| 接口组 | 宽度/方向与责任 |
|---|---|
| 固定输入 | clk:Clock、rst:Reset；req_valid1、write1、addr16、wdata32、wstrb4、rsp_ready1 |
| 固定输出 | req_ready1、rsp_valid1、rdata32、error2 |
| 每寄存器当前值 | RO/external-owner RW由peer输入value32；leaf-owned RW/W1C可输出value32供组合使用，保留位清0 |
| 动态拒绝 | 按描述启用read_reject/write_reject各1输入；权限/地址错误优先，write_reject仅有效选位非空时生效 |
| 候选写 | candidate32与write_mask32输出，无论reject/commit如何都由当前值、wdata、wstrb、mask纯组合形成，供peer验证；避免reject→candidate→reject组合环 |
| 提交副作用 | read_commit/write_commit各1输出，只有地址命中、权限合法、动态允许、非reset且请求握手；write_commit额外要求有效选位非空。peer在同一个上升沿消费，不能先注册pulse导致晚一拍；oracle在tick/时钟沿之前采样组合pulse，不用tick后值冒充提交 |
| W1C事件 | event32输入（或显式映射后的mask输入）；leaf owner每拍累积，外部owner由peer唯一更新；所有事件与clear按有效mask过滤 |

具体端口按访问/owner裁剪或统一提供均可，但必须说明常量接线与未用输出，真实组合通过freeze驱动/方向/宽度检查。read_commit可用于未来RX pop，write_commit用于TX push或SET/CLEAR；本故事peer只证明接口行为。wdata全0但有效WSTRB不为0仍是有效RW/WO写，不能把“写值为0”误当无副作用；W1C clear实际可能为0。

**地址视图：** leaf收到完整16位local byte addr，不截断高位；描述offset范围0..0xfffc且4字节对齐。没有定义的local地址（含高位）和未对齐访问SLVERR。四个0x100全局窗口、窗口外DECERR、全局到local变换由127.4负责；127.2不发布四窗decoder。软件头明确`*_OFFSET`，由调用者提供base，不偷偷给出固定系统地址。

**单状态所有权：** Timer COUNT/CTRL与GPIO OUT后续需要自主更新/别名写，必须由wrapper保存；leaf从当前外部值合并并提供同沿成功pulse。测试peer的next-state用软件写优先，否则自主更新，证明合并基于提交前值且读回不会落后。GPIO真实SET/CLEAR、Timer match、UART FIFO/串行都留给128。

**响应容量：** 简单实现pending寄存槽，pending时req_ready=0，响应消费沿不必同时接新请求（首版无零气泡承诺）；下一空闲周期可再接。rst屏蔽访问pulse；reset沿表面valid/ready不记成功提交。读RO动态数据reset时必须视为0，peer同域reset；rsp停顿期间peer变化不能改变已锁存快照。

### 现有代码与修改位置

- `crates/bitloom-prelude/src/ip/mod.rs`（已完整读）：当前私有分协议module并pub use；添加csr module/re-export，保留旧所有路径。NEW优先`ip/csr.rs`（过大可拆`ip/csr/`），不把代码堆入lib.rs。GPIO既有路径为`ip/gpio/`目录。
- `crates/bitloom-builder/src/lib.rs::define_module`（READ ONLY）：完整键值排序、重复键E0242、同名配置/完整HIR含Span比较E0244、任何helper错误poison session。callback只声明ports/process，不能begin/end/nested define。复用`assign_slice/assign_concat/assign_zero_extend`及位运算生成byte mask，不新增HIR。
- `ip/param_sync_fifo.rs`（已完整读，READ ONLY）：示范预校验、稳定Span、共用body和一次finish。新CSR不能借const泛型捕获动态配置，采用下述编码方案。
- 新测试放 `crates/bitloom/tests/fr196_csr.rs`、`fr196_csr_formal.rs`（或等价拆分）；private codec tests可在csr模块。现有 `fr195_param_sync_fifo.rs`/`_formal.rs`提供真实工具超时/日志/port oracle模式，不能复用FIFO性质冒充CSR。
- UPDATE `docs/public-api-1-0-surface.md`、`docs/ip/README.md`（已读）：追加本次确切符号与使用链接，保留既有稳定表面及历史证据。NEW `docs/ip/csr.md`与示例检查入口；如需要更新Justfile/CI/其他代码，先完整读取再最小改动。
- 最终UPDATE本故事与sprint；goal由主代理维护。只允许本故事状态变迁，Epic127保持in-progress，不动127.3/4、128–130或deferred。

### 完整参数身份方案

校验后按offset、字段mask/name规范排序，序列化为现有Vec<(String,u32)>：schema_version、reg_count、各寄存器offset/mask/reset/access/owner/event、field_count及字段完整记录。字符串用name_len与逐字节name_byte索引键，event名称/映射也必须完整编码。拒绝未知/额外/缺失键、长度/count不一致、枚举非法和字节>255；禁止只存hash、全局碰撞表或捕获closure。反解还要走同一合法性检查，避免内部畸形参数越过校验。稳定Span确保重复定义同一HIR。测试输入顺序变化规范为同一配置；同名换任一语义字段必须拒绝，不同名可保留两个定义。

标识符空间需独立检查：输入名字合法不意味着生成的端口名/内部名/C宏仍唯一；例如大小写转换与寄存器/字段拼接可产生宏碰撞。采用block命名空间和显式分隔规则，include guard不使用实现保留的双下划线/下划线大写前缀。C头用`<stdint.h>`及UINT32_C等安全常量，`0x80000000`/`0xffffffff`必须编译验证。

### 验证矩阵与过程证据边界

1. 配置及codec：地址重复/最后合法0xfffc/越界/1、2、3偏移；字段交叠/非法mask/reset；混合权限；名字/保留字/大小写与拼接碰撞；owner/event变化身份；schema/count/byte缺失和异常；预校验无session副作用与helper失败poison区别。
2. 叶行为：所有访问类型×16WSTRB、0/全1/高位/部分mask、洞/未对齐/高地址无alias；长背压、req_valid持续、拒绝/输入状态在提交后变化、reset与请求/待响应碰撞、恢复新请求。独立黄金期望不得调用DUTdecode/merge helpers。
3. peer：RO快照+成功读pulse、WO低字节push计数、动态拒绝成功/失败、候选合并值校验、external RW自主更新/部分写、W1C set/clear碰撞且拒绝不抑制自然事件。响应停顿至少31拍；计数明确reset取消边界。
4. 软件：相同/重排配置三类产物逐字节一致；手写golden offset/mask/reset/access；`cc -std=c11 -Wall -Wextra -Werror`编译单头和双头消费者。无需在测试中执行MMIO读写。
5. 产品：native leaf与实际Icarus/vvp、真实层级peer；CSR正式sby proof/cover及原始RTL综合/check，记录工具pin/版本、深度、assumptions、seeds、命令、exit code、artifact。formal未执行不得以cargo ignored行或已有FIFO证明计pass。
6. 回归：旧M0 AXI及M1定向、公开示例/表面兼容、最终clean/fmt/`just test`全workspace。全部工具独立列结果，工作区通过不代表JVM/CIRCT/formal全绿。

127.1的117场景自动化是**状态/过程门禁**，固定BFM是工具API探针；两者不证明CSR产品。127.1真实回归1763 passed、0 failed、14 ignored是基线，不预填127.2结果。门禁并不核对全部跨故事依赖，127.2前置已人工核验。工具候选`/tmp/bitloom-maintenance-tools/bin`须重新探测，临时路径不是产品依赖；沿用`ci-sby-pins.env`和已有安装脚本，不升工具钉。缺工具先按授权修复再重跑，不缩AC换绿。

### 历史学习、架构与外部核验

最近5个提交依次是127.1风险门、126.4参数FIFO/M1、126.3注册切片、126.2共享定义、126.1组合风险门。127.1 review明确补入单owner、每寄存器单访问类型、C消费者和真实形式/结构责任；本故事全部承接。桥外部ready/valid组合依赖锥检查责任属127.3，127.2需避免自身candidate/reject环并通过综合check，不假装已完成桥结构验证。

遵守AD-1/6/7/13/18/30：设计仅依赖bitloom-prelude；唯一FrozenHir；静态配置只服务elaboration/软件产物；无运行时closure、dyn、第二IR。AD-31外部核与AD-9工具钉保持。Rust1.97.1/edition2024、firtool1.159.0、Chisel7.15.0不变，新增依赖默认不需要。Bitloom与samitbasu/rhdl无关。

2026-09-21已读取[官方Rust Reference函数指针正文](https://doc.rust-lang.org/reference/types/function-pointer.html)：函数项及非捕获非async闭包可强转fn pointer，与现有builder契约一致；本故事采用显式非捕获函数。此核验不构成升级Rust的理由，不宣称所用工具是当前最新。无其他新第三方库/API需求；W1C优先级、错误码和响应时序来自已批准本项目合同，不假借未读Arm/OpenTitan正文。

### References / 输入发现

SELECTIVE_LOAD：epics.md的Phase24共同要求及Epic127全部4故事；PRD addendum的Phase24 FR196/NFR93–99；架构脊柱AD-30/31及既有基础规则；完整正式CSR合同、127 context/NFR14、127.1最终记录、模块组合与现有IP代码。无适用UX输入，本故事是库/硬件及软件地址产物。

- [正式接口合同](../../docs/ip/phase24-contract.md)：CSR提交点、字段完整决定、reset、当前整体授权。
- [Epic127](../planning-artifacts/epics.md#epic-127-csr-与总线连接)；[PRD addendum](../planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md)。
- [风险正文](epic-127-nfr14.md)：完整参数、单owner、访问粒度、验收责任；[context](epic-127-context.md)；[127.1](127-1-csr-总线-nfr14.md)：最终关闭及review教训。
- [架构脊柱](../planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md)；[模块组合](../../docs/ip/module-composition.md)。
- [FR142表面](../../docs/public-api-1-0-surface.md)；[SemVer政策](../../docs/semver-1-0-policy.md)。

## Dev Agent Record

### Agent Model Used

Codex；create-story子代理csr_product_story，主代理并行只读核验builder/仿真API。

### Debug Log References

完整读取create-story SKILL、resolver workflow（prepend/append/persistent_facts为空）、config、discover-inputs、template、checklist；精确指定127.2，不选择deferred122。完整读取sprint文件并解析709个development_status条目；当前127.1/126.2 done，127.2 backlog，Epic127 in-progress。创建时只改story与sprint目标键/last_updated。

### Completion Notes List

- Ultimate context engine analysis completed - comprehensive developer guide created.
- checklist已核对并直接补齐owner、候选值无环、local地址、C产物编译、helper poison、真实formal/综合与过程门禁区别。
- 仅create-story完成，产品和ATDD尚未实施；无需就已授权合同重新请求选择或确认。下一步ATDD。

### File List

- `_agile-output/implementation-artifacts/127-2-csr-描述-rtl-与软件地址产物.md`（NEW）
- `_agile-output/implementation-artifacts/sprint-status.yaml`（UPDATE：仅127.2 ready-for-dev与last_updated）

### Build完成（2026-09-21）

实现与三路内审修复完成，见[spec](spec-127-2-csr-product.md)、[初始证据](../test-artifacts/127-2-build-evidence.md)、[审查修补证据](../test-artifacts/127-2-review-fix-evidence.md)。此记录更新前述ATDD/create阶段状态，独立code-review、automate、最终全量回归和提交仍未完成，story保持review。

### Review Findings

独立四层完整裁决见[review记录](../test-artifacts/127-2-independent-review.md)。

- [x] [Review][Patch] R2 补齐leaf/external/W1C读拒绝行为 — 见review记录。
- [x] [Review][Patch] R3 加强leaf动态拒绝formal — 见review记录。
- [x] [Review][Patch] R4 加入producer协议monitor — 见review记录。
- [x] [Review][Patch] R5 明确generated Rust standalone验证边界 — 见review记录。
- [x] [Review][Patch] R6 五种singleton原始综合 — 见review记录。
- [x] [Review][Patch] R7 codec显式枚举编码 — 见review记录。
- [x] [Review][Patch] R8 更新已完成任务与历史阶段标记 — 见review记录。
- [x] [Review][Patch] R9 归档历史源码，保留member摘要 — 见review记录。
- [x] [Review][Patch] R10 C消费者字节地址示例与编译 — 见review记录。
- [x] [Review][Patch] R11 多字段硬件独立黄金 — 见review记录。

Rejected：R1 low，同大写include guard的不同定义已由集成合同排除，保留标准重复include语义；补清楚大小写折叠限制。

独立code-review修补已完成，19功能/config、3codec、3真实formal/综合与文档例通过；证据见[独立修补](../test-artifacts/127-2-independent-fix-evidence.md)。下一步automate；不提前关闭AC8或提交。

### 最终关闭（2026-09-21）

全部七步随本故事提交闭合，完整回归1787 passed/0 failed/18 ignored，专用CSR真实formal/综合4项另跑通过。前述create/ATDD/build阶段的“尚未完成”均为历史，以[最终验证](../test-artifacts/127-2-final-verification.md)为当前状态。仅127.2 done；127.3/127.4 backlog、Epic127 in-progress，FR196/M2未整体关闭。

### 最终 File List

- `crates/bitloom-prelude/src/ip/csr/{mod,codec,rtl}.rs`、`ip/mod.rs`。
- `crates/bitloom/tests/fr196_csr{,_config,_formal}.rs`。
- `docs/ip/csr.md`、`docs/ip/README.md`、`docs/ip/phase24-contract.md`、`docs/public-api-1-0-surface.md`。
- `scripts/check_fr196_example.py`、`.github/workflows/ci.yml`。
- 本story、`spec-127-2-csr-product.md`、`epic-127-nfr14.md`、sprint与goal台账。
- `test-artifacts/127-2-*`（日志/命令/摘要/不可变源码与真实工具归档/验证器）、对应ATDD清单、`automation-127-2.md`；完整归属以本提交git列表核验。
