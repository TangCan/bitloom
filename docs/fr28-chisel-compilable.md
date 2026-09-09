# FR28 — FrozenHir / FIRRTL → 可编译 Chisel Scala

**验收合同（AD-27 / FR88）：** 在钉死栈下可编译 + 公开端口名/宽/向与实例层次往返谓词。允许机械/生成风格；**不以**「结构化尽力失败」交差。

**公开诚实声明（FR88）：** `emit_chisel` / 往返验收 = **可编译 ≠ idiomatic**。合同只要求钉死栈下**可编译** + **端口/层次谓词**；**不**要求 idiomatic / 手写可维护 Scala。FIRRTL→idiomatic Scala 为 **FR93 永久非目标**（须新 PRD；见 README「永久非目标（FR93）」）。

**钉死版本对（AD-9 / NFR12）：** Chisel **7.14.0** ↔ firtool **1.155.0**。升钉须上游正式配对并更新脊柱 Stack / AD-9 / 本页运维清单；**禁止**私自升版交差。

```rust
let art = rhdl_firrtl::emit_chisel(&frozen)?;
// 或 .fir → import → emit_chisel
```

**CLI：** `cargo bitloom import --input design.fir --out-dir out --also-chisel` 在写 `.v` 的同时经 `emit_chisel` 写出 `.scala`。

## firtool / Chisel 钉死运维清单（FR88 / NFR3 / NFR12）

| 项 | 合同 |
|----|------|
| Chisel | **7.14.0**（JVM 真编译：`just chisel-fr28-jvm` / GHA `fr28-chisel-jvm`） |
| firtool | **1.155.0**（与 Chisel 正式配对；源码常量 `FIRTOOL_TARGET`） |
| 默认策略 | **不信任** `PATH` 上的裸 firtool；不得用 PATH 冒充钉死对 |
| 下载/校验/缓存 | `cargo run -p bitloom -- firtool info` · `firtool ensure`（资产 + `.sha256` → 本地缓存） |
| 覆盖入口 | `RHDL_FIRTOOL_PATH` = 含 `firtool` 二进制的目录 |
| 缓存根 | `RHDL_FIRTOOL_CACHE`（可选；覆盖默认缓存位置） |
| 多平台资产 | [`docs/nfr11-firtool-platforms.md`](nfr11-firtool-platforms.md)；不支持的 triple 必须设 `RHDL_FIRTOOL_PATH` |

README 交叉入口：[firtool（NFR3）](../README.md#firtoolnfr3) · 合同表「Chisel 可编译生成（FR28）」。

## 产出

- 成功：`.scala`（`class … extends Module`），含层次时 `Module(new Child)` 与按方向连线（跳过 `clk`/`rst`；Chisel `Module` 隐式 clock/reset）。
- **FR81 Path A（Epic 33）：** 文档化单时钟 `Mem` / `SyncReadMem`（含可选常量 `init`）→ 可编译 Scala；子集外 `MemDecl` 仍结构化失败 `rhdl::E0901`（不得删除诊断冒充全表面）。决策页：`fr81-mem-chisel-contract-decision-2026-09-09.md`。
- **Mem↔Chisel 边界：** FIRRTL/`firrtl.mem` 腿（AD-3 / AD-21）独立；本页约束 **`emit_chisel` Scala 腿**。双时钟裸 mem、超出 HIR `MemDecl` 的多口/掩码等仍在子集外 → E0901。

## CI / 本机（FR71 / NFR34）

- **默认 CI（Epic 25 / FR71）：** 除 Rust 谓词外，须有 **required** JVM job（见 Story 25.3）对黄金 `.scala` 跑真编译；失败则红。
- **Required 本机/脚本：** [`scripts/chisel-fr28-compile-required.sh`](../scripts/chisel-fr28-compile-required.sh)（或 `BITLOOM_REQUIRE_CHISEL_JVM=1` + [`chisel-fr28-compile.sh`](../scripts/chisel-fr28-compile.sh)）。缺 Java≥17 / sbt / 编译失败 → **非零退出**。ATDD：`just chisel-fr28-atdd` 或 `bash scripts/test-chisel-fr28-required.sh`。
- **黄金夹具（FR71 required）：** [`crates/rhdl-firrtl/testdata/fr28_golden_counter.scala`](../crates/rhdl-firrtl/testdata/fr28_golden_counter.scala)（**无 Mem**；不得因 FR81 变更变红或被 Mem 夹具替换）。
- **Mem Path A 合同夹具（FR81 / Story 33.4，可选 JVM）：** [`crates/rhdl-firrtl/testdata/fr81_path_a_sync_read_mem.scala`](../crates/rhdl-firrtl/testdata/fr81_path_a_sync_read_mem.scala) — 机械 `SyncReadMem` 读写烟测；同一 required 编译脚本、NFR12 钉死对。本机：`just chisel-fr81-mem-jvm`（**不**替代 `just chisel-fr28-jvm`）。
- **本机配方：** `just chisel-fr28-jvm` — 与 CI 同一 required 路径；**不**并入默认 `just test`（保持 Rust-only，降低贡献门槛）。维护者在合并涉及 `emit_chisel` / FR28 / FR81 的变更前应至少跑通一次（有 JDK17+sbt 时）；触及 Mem 降级时另跑 `just chisel-fr81-mem-jvm`。
- **逃生舱：** `BITLOOM_CHISEL_JVM_SKIP=1` 可跳过并 exit 0——**仅**本地逃生；**默认 CI 不得设置**（NFR34）。
- **可选 legacy：** 不设 require 时，缺工具链仍可 skip=0（非合同路径）。
- **默认 CI：** GitHub Actions required job **`fr28-chisel-jvm`**（与 Rust `test` 并行）：Temurin Java 17 + `cache: sbt` + `setup-sbt` → `scripts/chisel-fr28-compile-required.sh` 编译 **counter** 黄金夹具。失败则红；**不**设 `BITLOOM_CHISEL_JVM_SKIP`；**不** `continue-on-error`。当前 `timeout-minutes: 20`（首批冷/热样本前的保守上限；见下方墙钟记录）。Mem Path A 夹具由 Rust ATDD + 可选本机 `just chisel-fr81-mem-jvm` 覆盖，**不**削弱 FR71 required 面。

## 维护者合并前检查清单（FR28 / `emit_chisel`）

合并触及 `emit_chisel`、`scripts/chisel-fr28-*`、黄金 `.scala`、或 FR28/FR81 文档的 PR 前，维护者应勾选：

- [ ] `cargo test -p rhdl-firrtl -- chisel_fr28`（Rust 谓词）绿；触及 Mem 时另跑 `cargo test -p bitloom --test fr81_path_a_mem_chisel_emit` / `fr81_mem_chisel_atdd_fr71`
- [ ] 本机有 Java ≥ 17 + sbt 时：`just chisel-fr28-jvm` 绿（与 CI 同路径）；Mem 变更另跑 `just chisel-fr81-mem-jvm`
- [ ] 若本机无 JDK17+sbt：确认默认 CI 的 `fr28-chisel-jvm` job 将覆盖 **counter** 真编译；**不要**用 `BITLOOM_CHISEL_JVM_SKIP=1` 冒充通过
- [ ] 未把 FR28 改回「尽力失败 / skip=0 即合同」；未删除子集外 E0901 冒充全表面 Mem 支持

## GHA 墙钟记录（epic-25-retro-item-56）

| 样本 | 日期 | 冷/热 | 墙钟 | 备注 | `timeout-minutes` |
|------|------|-------|------|------|-------------------|
| （待首次 push 后填写） | — | cold | — | 首次依赖拉取可能分钟级；见调研 | 20（现行） |
| （待） | — | hot | — | sbt cache 命中后应收紧 timeout | 20 |

**流程：** push 后打开 Actions → `fr28-chisel-jvm` → 记录 Duration；冷热各至少一次后，若热跑 ≪ 20m，开 PR 下调 `timeout-minutes`（建议热跑×2 且 ≥10）。在实测前**不**盲目下调以免假红。

## 与 AD-3

FIRRTL 文本契约仍是 **FrozenHir ↔ FIRRTL 6.0.0**（`emit` / `import`）。Chisel Scala 是独立产品路径（AD-27），不替代文本契约。
