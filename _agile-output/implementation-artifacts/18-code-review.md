# Code review — Epic 18 (18.1–18.4)

**Verdict: approve**（文档 epic；18.3 有意延期）

## Findings

1. **(pass)** 18.1：`docs/tutorials/rv32-episode-ii/README.md` 含 MSRV 1.97.1、`cargo bitloom` / `bitloom-sim`、真独立 vs monorepo、章节表 + DoD 类型、链回 Episode I、Bitloom 免责。
2. **(pass)** 18.2：00–05 一步一变；测试名对齐 `rv32_core` / `rv32_pipe` 真实黄金；CSR 非必做；COMPLIANCE 措辞一致。
3. **(pass)** 18.3：**deferred** — Ch.06 stub + SUBSET/PIPE/COMPLIANCE/deferred-work 交叉引用；无半成品 CSR RTL；NFR32 明确。
4. **(pass)** 18.4：`99-episode-ii-outline` 指向 episode-ii；根 README Episode I/II 表；非目标含 cache/MMU/Linux/动态预测；VexRiscv 仅对照。
5. **(pass)** sprint-status：`epic-18` 与 `18-1`…`18-4` 均为 `done`。
6. **(info)** CSR 最小集规格仅文档化；日后实现需另开故事 + 绿测。
7. **(info)** 章节偏短，与 Episode I 风格一致，适合对照代码而非重写教材。

## Review-loop patches

无阻塞缺陷；未改 RTL。

**未 git commit**（按用户要求）。
