# Story129.1 独立人工验收清单（ATDD）

本清单是风险门内容审查，不是浏览器E2E、FR198系统行为测试或NFR14正文。所有项目初始 **unreviewed**，实际证据均待build；未来计划的完整性与未来产品运行成功分别判定。审阅者须逐读原始证据，填写具体位置、结论、日期和unknown；不能用标题、关键词、文件存在或历史绿色代替内容判断。

输入：Story129.1八项AC、正式phase24-contract、NFR14模板及当前关闭/源码证据。审阅目标为 `_agile-output/implementation-artifacts/epic-129-nfr14.md`。存在性脚本仅检查这个正文路径，缺失exit1；即使存在exit0也不证明质量、FR198行为或FR201完成。ATDD worker未运行脚本或任何工具/产品探针。

Rust/Cargo/Python后端，无浏览器、HTTP或移动边界，浏览器E2E与provider states=N/A；Playwright/Pact flags true不构成引入JS的理由。沿用父流程一次Pact探测false/fallback none，本worker未重试。没有生成产品API测试、fixture或文档关键词断言。

|ID / AC / 优先级|独立验收动作及通过条件|状态|实际证据 / 日期 / 审阅人|unknown或失败处理|
|---|---|---|---|---|
|M01 / AC1 / P0|核对125.3/M0正式前置及M1/M2/M3关闭提交、当前sprint；只解除129.1风险门，129.2/129.3仍backlog，Epic129 in-progress，FR198和核心FR201未交付，FR189 deferred/NFR91与历史done保留。|build内容核验通过（计划或探针边界见证据）|风险正文(a)与root-dependency-audit.json；M0–M3已done，下游仍backlog；仅风险范围有效；2026-09-22 UTC / Codex实施代理|状态冲突先查证，不改历史交付事实。|
|M02 / AC2 / P0|正文(a)上游约束、(b)工期带、(c)禁止静默降级、(d)负责人均有效且相互一致；129.1为0.5–1、129.2/129.3各3–5，共6.5–11有效人日；25%预留若采用另列8.125–13.75，不是日期/墙钟承诺。|build内容核验通过（计划或探针边界见证据）|风险正文(b)：0.5–1+3–5+3–5=6.5–11，25%另列8.125–13.75；有效估算非墙钟承诺；2026-09-22 UTC / Codex实施代理|字段缺失或套用旧研究6–10阻止关闭。|
|M03 / AC2 / P0|Richard维护owner、Codex实施证据、主代理七步提交责任明确；每项风险都有owner、失败动作、停止条件，维护叠加覆盖旧API、共享图、三后端、工具身份和CI成本；不将并行代理当免费维护人力。|build内容核验通过（计划或探针边界见证据）|正文(d)及风险/停止表：Richard/Codex/主代理职责与维护叠加齐全；2026-09-22 UTC / Codex实施代理|缺owner/行动方案不可交付风险门。|
|M04 / AC3 / P0|核源码可行性：一个ElaborateSession一次finish，真实桥→四窗→UART/GPIO/Timer/IRQ；addr16/data32/WSTRB4、单ACLK；0000/0100/0200/0300各256字节，洞/未对齐/访问错误SLVERR、0400–ffff DECERR且不截高位。prelude-only，不重写外设/拼FrozenHir/新增native层级。|build内容核验通过（计划或探针边界见证据）|正文两种正式拓扑、源码define_module/端口复核及system-recon；风险计划有效，尚非真实系统PASS；2026-09-22 UTC / Codex实施代理|不兼容连接或新架构范围先记录解决，不缩小合同。|
|M05 / AC3,4 / P0|IRQ严格五路：Timer、RX成功到达、TX取队首、overflow或framing、GPIO新沿；原始脉冲接IRQ，不用sticky EVENT，无第六路。验收计划能区分本地EVENT未清时IRQ清除不重触发和后续新事件到达。|build内容核验通过（计划或探针边界见证据）|正文IRQ五位映射/clear静默再事件序列；root-foundation-verification源码独立审计；未来行为待129.2；2026-09-22 UTC / Codex实施代理|用定向时序和外部观察判据，不能只列信号名。|
|M06 / AC4 / P0|固定两种不同拓扑：完整AXI桥图与直接CSR图，共享四窗/四真实外设和事件helper；两图均计划真实行为测试，不能只改名，旧127.4 peer或128.5 UART→IRQ不能替代FR198。|build内容核验通过（计划或探针边界见证据）|正文AXI桥图与无桥直接CSR图固定，共享四窗四外设；未来两图行为待129.2；2026-09-22 UTC / Codex实施代理|图未明确或仅旧样例需补充计划，不能宣称系统通过。|
|M07 / AC4 / P0|独立主端手写关键地址/副作用，配置Timer、GPIO32、UART真实线端收发及清IRQ；AW/W独立、读写并发、各通道背压、全部WSTRB和访问错误/未对齐覆盖，特殊请求用原始通道，不能与BFM同针脚双驱动。|build内容核验通过（计划或探针边界见证据）|正文独立验收：手写黄金、独立线端、raw/BFM互斥、16种WSTRB；计划有效未执行未来系统；2026-09-22 UTC / Codex实施代理|scoreboard不得复用DUT状态转移或生成地址自证。|
|M08 / AC4 / P0|复位与事务账本区分桥捕获、CSR唯一提交、响应消费和取消；reset共同清bridge/decoder/leaf/FIFO/FSM，已发送物理bit不回滚，BFM None为取消非OKAY；安全性不依赖ready公平，完成上界另列假设。随机预算16固定seed×1000完成事务及必达定向计数，调整须实测理由。|build内容核验通过（计划或探针边界见证据）|正文账本与复位取消位置、16seed×1000完成预算及必达计数；计划有效，未来实际seed/覆盖待129.2；2026-09-22 UTC / Codex实施代理|缺取消/守恒或概率当保证需补计划。|
|M09 / AC5 / P0|真实工具记录命令/UTC/耗时/退出码/解析路径/版本/日志；Rust/Cargo/Python3.12、锁定BFM闭包、iverilog/vvp、firtool、Java/Scala/Chisel、Yosys/sby/z3身份可追溯，SBY tag/commit/安装文件/隔离Python闭包核验；不改工具钉。|build内容核验通过（计划或探针边界见证据）|root-foundation-verification/results/manifest及reset-evidence；35基座命令成功、16个SBY安装文件核验；实际JVM依赖见reset tar的jvm-dependency-identity.json；2026-09-22 UTC / Codex实施代理|缺工具或错误退出不能skip为PASS；保留失败原件。|
|M10 / AC5 / P0|实际运行固定BFM stub和一个既有真实桥/译码层级RTL入口，精确测试数、版本与原始产物可核；确定性seed=N/A，随机记录实际seed/事务数/必达计数。只说明旧基座可用，不是FR198或形式证明。|build内容核验通过（计划或探针边界见证据）|root-foundation-verification：BFM1例，旧decoder exact1例，31提交/31响应；只验旧基座，seed=N/A；2026-09-22 UTC / Codex实施代理|历史PASS、仅版本发现或compile-only不足。|
|M11 / AC3,5 / P0|最小真实父子复位图在direct/FIRRTL/Chisel逐路实际编译执行：先把至少两个子状态置非零，aresetn与Chisel隐式reset独立区分；验证同步断言优先、同沿共同清零、释放后恢复。保留生成源码/TB/命令/波形/失败，不允许TB绑定额外reset掩盖生成连接。|build内容核验通过（计划或探针边界见证据）|reset-evidence及tar r3：适配三后端实际双子非零/沿前保持/同步优先/共同清零/恢复；typed Chisel lower失败保留，不能声称其行为连接已验证；2026-09-22 UTC / Codex实施代理|检查HIR Reset类型和Chisel显式child reset路径；UInt1 XOR或emit成功不能作结论。|
|M12 / AC5,8 / P0|真实探针决定合法复位路线：typed HIR路线须实证；如失败，保留失败并按AD-30核无状态最薄RTL适配，源码随例子交付，三后端共用且实际跑/综合/源绑定，准确称生成core+aresetn适配；不冒称全top HIR或外部IP试点。|build内容核验通过（计划或探针边界见证据）|reset-evidence最终路线：公开无状态boundary+core+端口shim，三后端同源执行和纯设计综合通过；AD-30与外部同步前提保持；2026-09-22 UTC / Codex实施代理|路线未解不得下游ready；不可静默修后端或缩为仅高有效rst顶层。外部控制器须同步aresetn断言及释放，反相不是同步器。|
|M13 / AC6 / P0|实际gate当前合法状态通过；临时状态覆盖129.1未done时129.2/129.3各active状态拒绝、M0未关拒绝、Epic129提前done拒绝、合法未来状态通过。检查确切诊断/退出和真实sprint前后不变。|build内容核验通过（计划或探针边界见证据）|129-1-build-gate.log与-opt.log各84例，精确诊断/当前状态通过，真实sprint SHA c4c1e1cbce6f6fe515959d1c8816d9c4e261f03bb74673c93fcc21f72d7c38fa前后相同；2026-09-22 UTC / Codex实施代理|worker A过程结果单列；不能把既有GREEN伪称产品RED。|
|M14 / AC6 / P0|人工独立核全部边：129.1需125.3；129.2需129.1/127.4/128.2/128.4/128.5；129.3需129.2；M4需129.1–3全done。gate只覆盖部分关系，不谎称自动脚本覆盖全部边。|build内容核验通过（计划或探针边界见证据）|正文逐边表与root-dependency-audit.json；129.2风险门未done、129.3前置backlog；gate并非全依赖检查；2026-09-22 UTC / Codex实施代理|逐边引用sprint/合同证据，未满足不得ready。|
|M15 / AC7 / P1|129.3核心矩阵明确direct/FIRRTL/Chisel各自编译执行相同系统预期；native两引擎/generated层级unsupported。有限formal prove、cover、原始综合、旧API兼容及SemVer分别验收，不将综合当PPA、工具发现当prove。|build内容核验通过（计划或探针边界见证据）|正文核心矩阵：三后端/unsupported/prove/cover/原始综合/兼容分行；当前仅探针，不声称系统矩阵完成；2026-09-22 UTC / Codex实施代理|每条证据必须绑定源码/工具身份，不取历史PASS并集。|
|M16 / AC7 / P1|责任分配：129.2交付干净checkout一条命令使用配方并真实隔离复现；129.3复核核心CI/矩阵。入口列外部工具，不依赖主target或/tmp生成文件；上手步骤/耗时、手工接线/地址代码量、修改文件数、CI耗时实际测量，贡献模板列owner/独立测试/两组合/支持矩阵/兼容/来源许可/证据。|build内容核验通过（计划或探针边界见证据）|正文核心矩阵责任：129.2净checkout一命令与成本实测，129.3重放/贡献模板/CI复核；未来验收待实施；2026-09-22 UTC / Codex实施代理|不可将计划写成完成实测或编造性能收益。|
|M17 / AC7 / P0|核心与外部分界：Epic130外部试点独立，外部行不能凭核心绿PASS；核心FR201关闭不依赖130.3，但完整Phase24须外部验收，不吞NFR91或FR189 deferred。|build内容核验通过（计划或探针边界见证据）|正文范围/核心矩阵：Epic130独立外部行、核心不依赖130.3但Phase24必须外部验收，FR189/NFR91保持；2026-09-22 UTC / Codex实施代理|矩阵与关闭宣称有越界则修正文案/状态。|
|M18 / AC8 / P0|正文有效、探针与gate实际证据、独立review、automate、clean→fmt→just test和单故事commit齐备才129.1 done；只解自身NFR14，不新增API/工具pin/包版本，不push/publish。ignored、失败、未执行单列。|unreviewed|待父流程七步完成|有未解工具/架构阻塞保留，不提前标通过。|

验收记录格式：ID、正文位置、实际命令/原始日志位置、通过或失败理由、剩余unknown、审阅人及UTC。对未来系统计划只能标“风险计划有效”，不能标FR198/FR201行为PASS；最终主代理复核另附，不覆盖ATDD初始记录。

## Build核验补记（2026-09-22 UTC）

初始清单的unreviewed描述保留为ATDD历史；以上M01–17为实施代理逐项内容核验，最终主代理独立审阅另附。M18仍unreviewed：独立review、automate、clean→fmt→just test及单故事commit由主代理执行后才闭合。未交付FR198/FR201或未来系统；typed lowering与r2综合runner失败均在reset归档保留。

正文存在性：129-1-build-presence.log exit0，只证文件存在；normal与-O两次过程gate exit0各84例，未改真实sprint。工具/基座/复位证据分别记录，不合并为系统PASS。
