---
title: '18.3 可选 Zicsr + M-mode trap（教学最小集）'
type: 'feature'
created: '2026-08-20'
status: 'done'
baseline_commit: '1804929c18414816fbff120f6367828b4beb85a1'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/18-2-递进教程正文-isa-流水-hazard.md'
  - '{project-root}/examples/rv32_pipe/PIPE.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR65 可选特权教学需要有交付物；完整 CSR RTL 过大易半成品。

**Approach:** **延期 stub**：教程 Ch.06 + SUBSET/PIPE/COMPLIANCE 延期注记；记录最小集规格但不实现 RTL。Epic 17 仍 done（NFR32）。

## Boundaries & Constraints

**Always:**
- 延期不得回溯 Epic 17 / 18.2
- 禁止 PicoRV32 自定义 IRQ 模板措辞
- 默认目标「能教 / 能跑 mret」——未实现时明确延期

**Never:**
- 半成品 CSR 路径冒充绿测
- 宣称 Privileged / arch-test 合规

</frozen-after-approval>

## Tasks & Acceptance

**Execution:**
- [x] `06-csr-m-trap-deferred.md` — stub：最小集规格 + 延期理由 + NFR32
- [x] `PIPE.md` / `SUBSET.md` / `COMPLIANCE.md` — 可选延期交叉引用
- [x] `deferred-work.md` — 追加 CSR RTL 延期条目
- [x] **未**实现 CSR RTL（有意）

**Acceptance Criteria:**
- Given 18.2 流水章存在，when 交付可选路径，then 延期 stub 清晰且不阻塞 Epic 17
- And 禁止 PicoRV32 IRQ 模板；无合规夸大

## Dev Agent Record

### Completion Notes

- **18.3 disposition: deferred**（stub chapter，无 CSR RTL）
- **未 git commit**

### File List

- `docs/tutorials/rv32-episode-ii/06-csr-m-trap-deferred.md`
- `examples/rv32_pipe/PIPE.md`
- `examples/rv32_core/SUBSET.md`
- `examples/rv32_core/COMPLIANCE.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/18-3-可选-zicsr-m-mode-trap-教学最小集.md`
