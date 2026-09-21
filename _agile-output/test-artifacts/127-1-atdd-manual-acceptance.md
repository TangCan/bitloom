# Story127.1 ATDD 独立人工验收清单

范围：CSR/总线 NFR14 风险门禁；依据 [Story127.1](../implementation-artifacts/127-1-csr-总线-nfr14.md)、[正式合同](../../docs/ip/phase24-contract.md)、[NFR14模板](../implementation-artifacts/nfr14-risk-record-template.md)。无用户界面、浏览器交互或端到端 UI 旅程，E2E 数量为 0；不生成 Playwright、浏览器 fixture 或虚构产品红测。本清单覆盖 AC1/2/3/4/6；AC5 状态变异由同阶段独立门禁 worker 负责。

## 初始观测与状态语义

2026-09-20T12:55:50.083953+00:00 执行 `test -f _agile-output/implementation-artifacts/epic-127-nfr14.md`，实际 exit=1，stdout/stderr 均为空。原始记录：[127-1-atdd-gate-presence.log](127-1-atdd-gate-presence.log)。这只证明该时点目标文档缺失，构成验收尚未满足的真实证据；不表示 Rust、BFM、RTL 或 formal 失败。本 worker 未创建风险正文，也未运行任何工具版本/BFM/formal/synthesis 探针：它们当前均为**未测**，不是失败或通过。

以下项目初始均为未审阅。实施后由独立审阅者逐项填结论、证据位置和理由；无需把所有检查机械转为单元测试。标题或模板字符串存在、文件存在、脚本 exit=0 均不能证明正文风险分析正确。缺字段或含占位文本不通过，字段齐全但内容冲突也不通过。观察已存在的绿色门禁不伪装成红测。

## 审阅项目

| ID / AC / 优先级 | 审阅动作与通过条件 | 应留证据 / 当前结论 |
|---|---|---|
| M01 / AC1 / P0 | 核对 M0 125.3 与 M1 126.1–4 的关闭记录和当前 sprint；风险故事仅关闭127.1，127.2–4此时保持backlog，FR196/M2不冒称已交付。整体七步授权优先于旧范围注记；FR189/Epic122 deferred、NFR91、已有done保留，无push/publish。 | sprint具体键、M0/M1证据引用与审阅理由；未审阅 |
| M02 / AC2 / P0 | NFR14(a)上游约束、(b)工期带、(c)禁止静默降级、(d)负责人都有可执行内容。owner/依赖/逐项AC/失败动作/停止条件明确；估算127.1为0.5–1有效人日、Epic127基线12.5–19，与实际墙钟区分。记录维护回归面和资源假设。 | 阅读完整正文，对每一风险指出具体决策和责任；未审阅 |
| M03 / AC2 / P0 | 静态配置→RTL/地址文档/C头文件同源且确定；独立黄金值手写。非捕获builder回调适配有具体可行方案，完整参数身份不只使用摘要，不依赖全局可变表、捕获closure或第二HIR；不能只说“复用builder”。明确RO动态输入/WO副作用/W1C事件/动态拒绝的wrapper责任，产品实现分配到127.2及128，不冒称已实现。 | 现有define_module签名与参数重建方案逐项对应、未来验证责任；未审阅 |
| M04 / AC3 / P0 | 请求addr16/wdata32/wstrb4，响应rdata32/error2，全系统单已提交未消费请求；非reset沿req_valid&&req_ready唯一提交，读快照锁存，下一周期响应，受阻保持且不再提交；写/W1C/RX pop/TX push只执行一次。reset优先于提交。 | 正式合同逐项比对，127.2后续测试/责任定位；未审阅 |
| M05 / AC3 / P0 | WSTRB按little-endian逐字节合并；保留位读0写忽略。RO写/WO读SLVERR，失败读rdata=0，无副作用；零WSTRB合法写OKAY无作用，错误优先。W1C公式(old & ~clear)|event，硬件set优先。TX未选byte0不push，不能由满状态误判错误；动态UART配置检查针对合并值。 | 16种WSTRB、同拍事件、错误优先级的后续黄金序列分配；未审阅 |
| M06 / AC3 / P0 | error仅00/10/11；窗口外DECERR、窗口内未定义/未对齐/非法访问SLVERR。四窗基址0x0000/0100/0200/0300且各0x100，0x0400–ffff不命中；无高地址截断别名、无重叠/双选，无ID/version。 | 手写边界/洞/地址高位用例与127.4责任；未审阅 |
| M07 / AC2/3 / P0 | AW/W/AR独立单槽且写收齐才有仲裁资格；不完整写不堵读。外部握手由寄存态控制，B/R容量在CSR提交前预留。reset后同时合格读优先，成功提交翻转优先级，无响应空间者不参与且不堵另一类。 | AW早/W早/同时、B/R停顿、并发、轮转与容量边界的127.3/4验证分工；未审阅 |
| M08 / AC2/3 / P0 | BREADY/RREADY无限停顿仅承诺安全，活性必须附公平性/等待上界和深度；有限延迟leaf不以无限等待隐藏满/空错误。aresetn断言/释放均由外部控制器同步，bridge/leaf共同reset，清捕获及响应，既有UART物理发送不承诺回滚。 | reset各在途阶段取消矩阵、接受/提交/响应/取消分别计数；未审阅 |
| M09 / AC1/3 / P0 | 新CSR16-bit/错误语义与旧Axi4LiteSlave ADDR8/恒OKAY bank明确分离；旧端口/reset/read-before-write兼容性保持。设计只依赖prelude，唯一session/HIR；native/generated层级unsupported，AWPROT/ARPROT保留但忽略不称安全隔离。 | 现行旧bank接口核验与新旧回归责任；未审阅 |
| M10 / AC4 / P0 | 每个探针保存真实命令、路径、版本、日期、退出码和原始日志；至少Rust、Python及固定BFM包、iverilog/vvp、sby/yosys/z3发现入口。确认requirements固定版本、不升级产品pin；来源与固定版本API匹配，无法取得原文则诚实记录。 | 工具探针未运行，当前未测；不得填失败或PASS |
| M11 / AC4 / P0 | 重跑既有独立BFM stub探针并审阅五通道暂停/reset取消API；高层BFM会拆分/对齐，非法对齐与特殊WSTRB应原始通道驱动。原始驱动和BFM不并驱针脚。stub通过只证明工具可用，不代表新CSR RTL/formal通过。 | 实际probe命令/退出码/日志及API限制；当前未测 |
| M12 / AC2/4/6 / P0 | 缺工具、版本不兼容、probe失败、无法表达描述、合同冲突等写出具体修复/升级路径；未解决时后续保持backlog。缺证据标未测而不是skip成pass；已批准选择不再要求用户重新审批。 | 未解决项台账和有证据的关闭理由；未审阅 |
| M13 / AC6 / P0 | 七步真实完成且可追溯：create-story、ATDD、build、独立review、automate、clean/fmt/just test、单故事commit。实际结果区分workspace/native/RTL/formal/综合/PPA；默认ignored不当证明。风险正文有效且无未解决架构/工具阻塞才可放行后续。 | 每阶段日志与审阅结论，commit由主流程最终记录；未审阅 |
| M14 / AC6 / P0 | 无CSR/桥/译码/外设实现或未存在API登记；不关Epic127/FR196/M2，不把M0/M1历史证明当新CSR验证。未来127.2依赖127.1+126.2，127.3依赖127.2+126.3，127.4依赖127.3；状态脚本不声称已覆盖全部跨故事依赖。 | 最终diff、状态和依赖人工比对；未审阅 |

## 后续验收记录格式

每项填写 `ID → 通过 / 未通过 / 未测 → 证据路径与定位 → 实际内容判断 → 未解决问题/责任人`。所有P0项通过且无阻塞后，方可给出本风险故事人工验收通过。初始文件缺失证据保留，不随build产生正文而覆盖；重新验证记录追加至独立后续证据。此清单本身不是风险门禁正文，也不是风险故事完成证明。

## Build 自检记录（2026-09-21；不代替独立审阅）

下表由实施者逐条比对正文和原始证据；“自检通过”不是上表要求的独立验收结论。主代理/独立reviewer仍须裁定，M13尚未满足。

| ID | 自检结论与实质依据 | 待办/责任 |
|---|---|---|
|M01|自检通过：风险正文(a)列M0/M1关闭索引及真实127.1 in-progress、127.2–4 backlog；gate日志保护sprint hash，FR189/NFR91保留|独立review裁定|
|M02|自检通过：正文(a)–(d)、12.5–19基线与0.5–1本故事估算、维护组合面、owner及停止动作均有具体内容|独立review裁定|
|M03|自检通过：正文“非捕获builder”对照lib.rs:127签名，逐byte名称/完整字段参数重建，不依赖摘要或全局状态；矩阵明确动态端口与wrapper责任、三产物及独立oracle|127.2实现测试，当前仅风险方案|
|M04|自检通过：接口矩阵第二行明确addr16/data32/strb4/error2、唯一提交、快照、下一周期、受阻不重交/reset优先|独立review裁定|
|M05|自检通过：字节/动态错误两行覆盖16 mask、RO/WO、错误零数据、零WSTRB、W1C公式及TX byte0例外|功能验收尚未测|
|M06|自检通过：地址行列四窗首末、0400–ffff、无ID、error编码与高位别名黄金例|127.4验收尚未测|
|M07|自检通过：捕获/轮转两行分别写槽容量、完整写资格、响应预留、初始读优先及提交翻转，无槽不参选|127.3/4尚未测|
|M08|自检通过：无限背压与reset两行明确公平性/深度要求、双边同步、桥leaf共同reset及分阶段取消计数|后续证明/真实矩阵尚未测|
|M09|自检通过：核对axi.rs ADDR8/恒OKAY，正文旧bank兼容独立；设计依赖、唯一HIR、层级unsupported及PROT忽略明确|旧回归由主流程/后续故事维持|
|M10|自检通过：build-evidence.md、各原始日志及probes.json记录路径/版本/UTC/返回码；pip查询失败保留，metadata五包实测成功；sby仅发现不冒称proof|独立review裁定|
|M11|自检通过：BFM exit0、1 JUnit/0 fail/error/skip、312ns；归档stub/JUnit；源码对齐证据与raw驱动限制明确，不冒称CSR通过|独立review裁定|
|M12|自检通过：正文(c)停止矩阵与pip误用修复有明确证据；未解决工具/架构阻塞当前无，产品未测保持未测|独立review确认|
|M13|**待验**：build未执行独立review、automate、clean/fmt/just test、commit；不得提前签七步完成|主代理继续后续步骤|
|M14|自检通过：本轮新增风险/证据文档及清单，无产品/API修改；依赖矩阵逐项列127.1+126.2、127.2+126.3、127.3，脚本边界明确|主代理最后diff与状态核对|

证据入口：[风险正文](../implementation-artifacts/epic-127-nfr14.md)、[build实测汇总](127-1-build-evidence.md)。初始缺正文RED及未测历史原文保持，不覆盖原始观测。

## 独立审阅（2026-09-21）

主代理逐条实质判断见[127-1-manual-review.md](127-1-manual-review.md)：M01–M12、M14通过（M14提交前复核），M13等待第5–7步，不提前关闭。
