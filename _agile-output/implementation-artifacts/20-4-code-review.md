# Code Review: Story 20.4 反向导入 Chisel/.fir → Bitloom（FR46 腿 2）

**Reviewer:** adversarial pass（blind / edge / verification-gap）  
**Date:** 2026-08-21  
**Verdict:** accept after patches

## Findings triage

| Severity | Category | Finding | Disposition |
| --- | --- | --- | --- |
| medium | patch | 未知 `inst.port` 作 lhs 时可能 invent 带点号的 net | 改为跳过；不发明 net |
| low | defer | 未知 `parent <= inst.port`（rhs 有点但无实例）仍落入普通 assign | deferred：seal 会失败或语义外 |
| low | defer | dangling connect 经 emit 丢失，往返不覆盖 dangling | deferred（夹具无 dangling） |
| — | reject | 解析 Chisel Scala | 合同明确 `.fir` 边界 |
| — | reject | `import` CLI | 20.5 |

## AC checklist

| AC | Status |
| --- | --- |
| `.fir` → FrozenHir → emit/tick；对称端口/实例图谓词 | pass |
| 导出再导入或外部 `.fir` 再 emit 夹具 | pass（两者皆有） |
| 设计 crate 仅 `bitloom-prelude` | pass（工具在 `rhdl-firrtl`） |

## testarch-automate

- `fr46_export_reimport_ports_and_instance_graph`
- `fr46_external_fir_firtool_style_connects_then_emit`
- `fr46_import_counter_then_tick`

## Disposition

Patches 合入 → `just test` 绿 → mark done。
