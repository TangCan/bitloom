---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-21'
storyId: '128.5'
storyKey: '128-5-缓冲-uart-与-m3-关闭'
storyFile: '_agile-output/implementation-artifacts/128-5-缓冲-uart-与-m3-关闭.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-128-5-缓冲-uart-与-m3-关闭.md'
generatedTestFiles:
  - crates/bitloom/tests/fr197_uart_api.rs
  - crates/bitloom/tests/fr197_uart.rs
  - crates/bitloom/tests/fr197_uart_formal.rs
inputDocuments:
  - '_agile-output/implementation-artifacts/128-5-缓冲-uart-与-m3-关闭.md'
  - '_agile-output/implementation-artifacts/epic-128-nfr14.md'
  - 'docs/ip/phase24-contract.md'
  - '_bmad/tea/config.yaml'
  - '_agile-output/test-artifacts/128-5-atdd-knowledge-sha.json'
pact_mcp_reachable: false
pact_fallback_source: none
---

# Story 128.5 ATDD

Step01：Richard，中文；Rust/Cargo backend，既有集成测试与三RTL后端。故事完整且前置done，主代理核验create-story及sprint仅128.5状态改变。知识13篇先前完整加载，本次逐篇SHA验证相同；重新读取被工具截断不冒称新增全文加载。Playwright/Pact为不适用的JS/服务边界，无新依赖。单次工具列表probe空，无broker调用。用户连续授权覆盖例行确认。resolver全部hooks为空。

Step02：AI generation；无浏览器录制。

Step03：P0 CSR错误/唯一提交、全宽配置与同沿启动、独立串行采样/TX解码、FIFO边界、事件/reset；P1 API共享定义隔离、三后端组合、软件产物、旧API兼容、有限formal及故障负控制。AC1→API/组合，AC2→API黄金+RTL，AC3–6→独立RTL逐拍，AC7→后端/formal/证据/最终全量门。缺API编译RED与行为RED分列，不当作协议验证。

## 冻结API

- `bitloom_prelude::ip::UartCsr` unit struct；`registers() -> CsrBlock`；`define_module(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；Elaboratable top `UartCsr`，同体一次finish。
- 标准12 CSR端口：clk Clock/rst Reset；req_valid1/write1/addr16/wdata32/wstrb4/rsp_ready1输入；req_ready1/rsp_valid1/rdata32/error2输出。另rx UInt1输入、tx UInt1输出、raw_events UInt4输出，共15。
- 描述块名UartCsr；寄存器名 ctrl/baud_div/status/tx_data/rx_data/EVENT（大写避SV关键字），地址0/4/8/12/16/20；mask1/ffffffff/f/ff/ff/f；全部字段名bits、reset0。owner Leaf/Leaf/External/None/External/Leaf；access Rw/Rw/Ro/Wo/Ro/W1c；EVENT事件输入名event_bits；write_reject仅ctrl/baud_div/tx_data，read_reject仅rx_data。
- raw_events bit0 RX成功入队、bit1 TX取队首启动、bit2 overflow、bit3 framing，沿前组合事件，reset门控。两FIFO复用ParamSyncFifo<8,4>，不公开内部debug。
- TX取队首沿e输出start低，P拍后data0，其后每P拍下一bit；stop保持P拍才结束busy。后续取帧仅空闲，允许一拍额外idle，不把它变成RX发送器等待条件。RX e/H与配置/FIFO同拍严格按故事固定合同。

Step04执行能力：auto解析subagent（支持spawn，未提供专用agent-team运行器）；worker A API、worker B RTL/后端，双方只生成RED测试，Rust用ignore对应skip，缺API仍编译失败。无产品实现、无提交。

## Step04c汇总

双主worker与额外formal worker完成，14项Rust ignore RED测试（API9、RTL2、formal3），无TODO/恒真占位；本机适配跳过所有浏览器/HTTP/JS专用项。源与数量见128-5-atdd-summary.json。API worker提前执行compile-only RED（主代理初始调度指令错误，后续更正前已结束），真实exit101仅缺UartCsr；保留日志，不重跑或冒行为RED。

待build补齐：软件原文例与C/Markdown产品；真实loopback；共享/改名双实例及实际UART→IRQ三后端；全宽计时近终点证明；串行/FIFO/事件配置至少三类DUT变异；活动串行取消账本；SemVer/CI/证据runner。上述不是ATDD已执行覆盖或PASS，必须完成后才可关闭故事。形式现有仅公开总线安全子集、3短cover、原始综合及额外ready故障。

无mock服务、data-testid、浏览器session、Pact或合并JS fixtures。独立oracle用u64绝对deadline/VecDeque、确定seed和具名边界；三后端复用预期不替代实际各自运行。原始证据保留用于维护，不能自动删除。

## 实现清单与运行

按故事T1–T5执行。当前14项先保持ignore；build逐项移除ATDD ignore并先观察失败，再实现和复验；昂贵后端/formal保留专用ignore并实际--ignored。cargo test -p bitloom --test fr197_uart_api；--test fr197_uart；--test fr197_uart_formal -- --ignored --nocapture。无头浏览器调试N/A；RTL失败看保存TB/log/VCD。估算沿用故事4–7有效人日，不是代理墙钟承诺。提交推迟七步末尾。

Step05：按技能checklist核验Rust适用项，14项ignore且真实断言，元数据/故事回链齐全；API/RTL/formal三个目标实际compile RED均exit101/E0432缺UartCsr，无其他编译错误。日志与结果持久在test-artifacts。尚无行为PASS；进入bmad-build逐项实现并补齐列出的fixture。
