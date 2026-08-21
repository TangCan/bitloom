---
title: 'import CLI --also-chisel'
type: 'feature'
created: '2026-08-21'
status: 'done'
route: 'one-shot'
---

# import CLI --also-chisel

## Intent

**Problem:** `cargo bitloom import` 默认只写 `.v`（可选 `--also-fir`），产品入口未暴露 FR28 的 `emit_chisel`。

**Approach:** 在 `import` 上增加 `--also-chisel`（镜像 `--also-fir`），经现有 `rhdl_firrtl::emit_chisel` 可选写出 `.scala`；更新简短文档与 CLI smoke。

## Suggested Review Order

- 入口：可选标志与 `emit_chisel` 写出路径
  [`main.rs:75`](../../crates/bitloom/src/main.rs#L75)

- 实现：`also_chisel` 分支写 Artifact
  [`main.rs:675`](../../crates/bitloom/src/main.rs#L675)

- smoke：`--also-chisel` 产出 `.scala`
  [`import_cli.rs:69`](../../crates/bitloom/tests/import_cli.rs#L69)

- 文档：CLI 示例
  [`fr46-chisel-import.md:29`](../../docs/fr46-chisel-import.md#L29)
