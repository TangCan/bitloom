# Story129.1 fresh-context checklist review

只读审查对象：`/tmp/bitloom-1291-story-draft.md`。完整阅读create-story checklist；workflow.md不存在，实际工作流内嵌SKILL.md。核对Phase24 inventory/共用风险模板、完整Epic129/130、正式接口合同、NFR14模板、AD-1/6/7/18/28/30/31、gate源码、现有桥/译码/UART接口及研究§5/8。未执行测试，未修改仓库；128.5尚待主代理最终关闭，不预先背书M3。

结论：草稿总体合同正确，建议落仓前修正以下3项必要问题，另有2项小补充。风险记录/探针与129.2/129.3产品验收应维持明确分界，无需提前实现完整系统。

## 必要修正

1. **AC5 reset探针需要明确失败分支、实现责任与证据边界。**
   静态证据：`crates/rhdl-firrtl/src/chisel.rs:1127`的emit_instance_connects无条件跳过child `clk`/`rst`，`:1378`附近以`Module(new Child)`继承隐式clock/reset；因此“父UInt1反相→typed Reset wire→child rst”的HIR连接即使freeze通过，也不能推定Chisel执行符合该连接。草稿应把这一项记录为已发现的待实测风险，不是已证实的运行失败。
   建议AC5补：最小探针使父外部aresetn与Chisel隐式reset可区分，先将至少两个子状态置非零，验证同步断言优先于事务/状态更新、所有子状态同沿清除、同步释放后恢复；三后端分别真实执行。失败保留原始工件，禁止仅改测试驱动隐式reset来掩盖丢失连接。
   若HIR内转换不能三后端一致，129.1可实验一个明确公开的薄RTL顶层adapter：对已同步的aresetn只做逻辑反相，驱动同session内核的统一高有效rst；adapter必须属于未来一命令生成/编译源闭包、三个后端均执行它且顶层仍暴露aresetn，不得作为testbench私下补丁。否则记录最小后端修复的范围/owner/前置验证计划，且问题未解决时129.2不得ready。129.1不被迫交付129.2产品；风险故事可记录阻塞，不能用未执行的修复计划声称路径可用。

2. **将clean-checkout责任明确归129.2，129.3再回归。**
   epics.md:10090–10096明文要求129.2从干净checkout一条命令运行。草稿AC7以“129.3负责FR201核心”开头后包含干净checkout，可能让实现者推迟FR198的必要证据到129.3。建议AC4或AC7明确：“129.2交付并实测干净checkout一条命令的完整主端配方；129.3在核心CI/支持矩阵复核此入口，并完成两种不同组合的复用证据及矩阵。”129.1仅冻结这一责任表与可行性，不要求现在形成完整系统干净checkout PASS。

3. **所有窗口大小明确为0x100（256）字节。**
   AC3写“各100字节”存在十进制歧义，应改“各0x100（256）字节”，基址均加0x；正式合同及现有CsrDecoder源码均是256-byte windows。否则生成实现/文档容易只覆盖100字节而破坏窗口内SLVERR范围。

## 应补充但不需要扩范围

4. **风险验证计划的种子、覆盖预算与断言非空。**
   AC5列命令/UTC/版本/日志，但未明确随机种子；共用完成条件要求保存种子，研究§5建议固定16×1000作为可按CI实测调整的起始预算。补一句：确定性探针写seed=N/A，随机主端记录实际seed/事务数/必达场景计数；预算是计划，129.1不运行完整129.2随机系统压力。不要把16×1000改成未经实际预算确认的硬产品要求。

5. **指向真实接口文件以减少错找/照抄伪API。**
   保留现有“以源码为准”，补几个定位：`crates/bitloom-prelude/src/ip/axi_lite_csr.rs`（不是axi_lite_csr_bridge.rs）、`csr_decoder.rs`、`uart_csr.rs`及`docs/ip/{uart-csr,gpio-csr,irq,timer}.md`；后端目录仍`crates/rhdl-firrtl`，不能按包名猜`crates/bitloom-firrtl`。UART实际raw_events为4位，IRQ接线映射草稿正确。最终基线需等128.5提交后写实际SHA/关闭证据，不沿用当前128.4 HEAD。

## 已核实无需改动

- 五路IRQ（0Timer、1RX、2TX dequeue、3overflow|framing、4GPIO新沿），接raw脉冲而非sticky，符合正式合同，无第六路。
- reset输入必须由外部控制器同步断言和释放；反相不构成同步器。已发送物理bit不可回滚、内部共同reset取消状态的叙述正确。
- AXI完整系统与无桥direct CSR不同拓扑的第二组合是合理候选，可保留“129.2 ATDD冻结”；只改名不算复用，旧peer/局部UART→IRQ不能替代FR198完整系统。
- gate已覆盖M0/NFR14/epic关闭及FR189保留，不覆盖129.2依赖各叶故事或129.3依赖129.2；草稿人工逐边审计叙述准确。
- direct/FIRRTL/Chisel必须实际执行，native两引擎/generated层级unsupported；工具发现、emit、形式、安全性/活性、综合/PPA分开，正确。
- FR201核心可独立130.3关闭，但完整Phase24及FR201全部不能据此关闭；129.1不绑定或下载外部核，正确。
- 129.1正式前置仅125.3/M0；草稿等待M1–M3是本次串行执行的基线约束，建议称“本次创建基线”，不应写成修改正式依赖。

未发现需要重开产品选择或提前新增公开API的理由。当前不推荐任何新工具版本；仓库固定工具身份应由129.1真实本地探针验证，不能用本审查代替工具执行证据。
