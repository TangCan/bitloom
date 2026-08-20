# Deferred work

- source_spec: `_agile-output/implementation-artifacts/17-4-五级流水-转发-分支-flush.md`
  summary: LW/SW 在 EpisodeIIPipe 已接线但无 tick 黄金；load_q 时序与 load-use stall 留给 17.5
  evidence: 17.4 AC 明确不要求 load-use；PIPE.md 标明 LW 依赖未定义直至 17.5

- source_spec: `_agile-output/implementation-artifacts/17-4-五级流水-转发-分支-flush.md`
  summary: SW 到 LED MMIO 时仍可能写入 DMEM[0x100&0xf]（与 Episode I 相同）
  evidence: `assign_mem_write_en` 仅看 `ex_mem_is_sw`，未排除 `is_mmio`；预存于 rv32_core

- source_spec: `_agile-output/implementation-artifacts/17-4-五级流水-转发-分支-flush.md`
  summary: ADDI/BEQ/LW/SW 判定仍为 opcode-only（无 funct3），非子集编码可能被误认
  evidence: 与 `rv32_core` Episode I 译码合同一致；教学子集假定合法编码
