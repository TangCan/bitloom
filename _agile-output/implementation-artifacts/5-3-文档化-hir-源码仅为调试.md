# Story 5.3: 文档化 HIR→源码仅为调试

Status: done

## Story

As a 工具链用户,
I want 文档明确 HIR→RHDL 再生不是产品互转,
so that 不会把它当成 FrozenHir↔FIRRTL 合同的一部分。

## Acceptance Criteria

**Given** 发布/用户文档与 CLI 帮助  
**When** 查阅源码再生或调试相关说明  
**Then** 标明 debug-only，且无发行测试宣称源码往返稳定（NFR10）

## Tasks

- [ ] README / docs 声明
- [ ] CLI `--help` 或子命令说明（若有 regen 入口则标注）
