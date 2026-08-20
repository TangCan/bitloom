---
title: 'technical research: cargo bitloom standalone usage after install'
type: 'technical'
topic: 'cargo bitloom standalone usage after install'
decision: 'Can users use Bitloom fully standalone after cargo install bitloom; what step-by-step tutorial is feasible today — and HOW to achieve true independence'
source: 'deep-recon-run+deepen'
status: complete
preset: 'standard'
validation: 'normal'
created: '2026-08-20'
updated: '2026-08-20'
claims_verified: 10
claims_unverified: 1
claims_overturned: 0
deepen: 'true-standalone-path-2026-08-20'
---

# technical research: cargo bitloom standalone usage after install

**Decision this research serves:** 用户在 `cargo install bitloom` 之后，能否完全脱离本仓库独立使用？今天能诚实写出怎样的 step-by-step 教程？**深化：** 若产品要求「真独立」，应选哪条实现路线？

## Executive summary

**今日结论：不能**（原裁决不变）。crates.io 的 `bitloom` 只交付 CLI；`build` path 绑 monorepo；`rhdl-prelude` 未发布。[1][2][7][11][14][15]

**真独立结论（深化）：推荐路线 A — 发布 `bitloom-*` 库族 + 改 CLI host shim 为 crates.io 依赖 + `cargo bitloom new`。** 与 Spade 多 crate 前缀、`wasm-bindgen` / `wasm-bindgen-cli` 拆分一致；保留 AD-14 进程内 elaborate+emit，遵守 AD-6「设计不依赖 CLI」。[21][22][23][24]

最小发布面（Verilog MVP）：`bitloom-hir` → `bitloom-builder` / `bitloom-macro` → `bitloom-prelude`（用户依赖）+ `bitloom-vlog`（CLI host）+ 已有 `bitloom` CLI。[25]  
`bitloom-*` 与 `rhdl-*` 名在 crates.io **均空闲**（2026-08-20）；应发 **`bitloom-*`** 并修订 AD-2（勿发 `rhdl-prelude`，以免抵消 Bitloom 品牌锁定）。[26][27]

**拒绝：** 让设计 crate 依赖 CLI 库（违 AD-2/6）；把 FrozenHir 序列化当阶段一 CLI 协议（违 AD-14）。[23][24]

**过渡（可选，非终态）：** `new` 脚手架暂用 git 依赖 monorepo 包 — 用户可不手工 clone，但不是 crates.io 真独立。[28]

最大 caveat 仍在：搜索混淆 `bitbloom` / 其它 `rhdl*`。[4][5]

---

## 1. Landscape — `cargo install` 工具的成熟分发形态

Cargo 明确：`cargo install` 只安装带可执行目标的包到 install root 的 `bin`；不是给用户装「可依赖的库」。[6] 自定义子命令靠 `cargo-<name>` 出现在 `$PATH`（默认优先 `$CARGO_HOME/bin`）。[8]

成熟对照：**mdBook** — `cargo install mdbook` 后，对**用户自己的书稿**执行 `mdbook init|build|serve`，不要求克隆 mdBook 源码仓库。[9][10] 这是「装 CLI → 操作用户资产」的主导模式。同族还有 **cargo-nextest**、**Spade Swim**、**sus_compiler**：文档均为 install（或 `--git` install）后直接作用于用户工程，而不是要求 clone 工具 monorepo。[16][17][18]

即使是「插件型」CLI，也常另需工具链碎片（如 **cargo-expand** 的 nightly/rustfmt；**wasm-pack** 仍要 rustc）。[19][20] 「可 install」≠「零依赖」；但业界默认仍是：**用户项目 + 已发布库/自包含二进制**，而非挂工具源码树。

Bitloom 今日表面像 mdBook（可 install 的 cargo 子命令），但 **build 语义仍绑定工具链 monorepo**，更接近「半发布的内部驱动器」，而非完整用户工作流闭环。

---

## 2. Integration — crates.io 上 `bitloom` 实际交付了什么

API（2026-08-20）：`bitloom` 最新 **0.1.1**（另有 0.1.0）；`has_lib: false`；`bin_names: ["cargo-bitloom"]`；`rust_version: 1.97.1`；仓库 `TangCan/bitloom`；依赖仅 **clap、sha2**。[1][2]

挂载 README 的快速开始仍是 workspace 内：

```text
cargo run -p bitloom -- build --package counter_ports --out-dir /tmp/rhdl-out --manifest-dir .
```

并要求设计 crate 依赖 `rhdl-prelude`。[3] 但 crates.io 上 **`rhdl-prelude` 不存在**，用户无法 `cargo add rhdl-prelude`。[11]

`docs.rs/crate/bitloom/0.1.1` 页面几乎无可读库文档（bin-only 符合预期）。[12]

---

## 3. Implementation reality — 为何脱离仓库必然失败

本地（与发布面一致）`crates/bitloom`：`publish = true`，bin=`cargo-bitloom`，deps=clap+sha2。[7] 同仓库其余 `rhdl-*` crate 均为 **`publish = false`**。[13]

`build` 实现：在 `--manifest-dir` 下写 `target/rhdl-host/<pkg>/`，path 依赖 `examples|crates|<pkg>` 与 `crates/rhdl-vlog`、`crates/rhdl-hir`，再 `cargo +1.97.1 run` 调用 `rhdl_elaborate()` → `rhdl_vlog::emit`。[14]

**实证（2026-08-20）：** 空临时目录调用已安装的 `/…/.cargo/bin/cargo-bitloom`：`--help` 正常；`build --package counter_ports --manifest-dir .` 报错缺少 `examples/counter_ports/Cargo.toml`。[15]

---

## Cross-dimension insights

| 组合 | 含义 |
|------|------|
| Landscape「装完操作用户项目」× Impl「build 绑 monorepo」 | 产品叙事与实现错位：用户以为 install 即闭环，实际只装了遥控器 |
| Integration「README 要 rhdl-prelude」× crates.io「prelude 未发布」 | 公开文档步骤在 crates.io-only 世界不可执行 |
| CLI 可装 × 库全 `publish=false` | 刻意的 0.x 发布边界，不是意外漏发一个 crate |
| 品牌锁定 Bitloom × AD-2 仍写 `rhdl-prelude` | 真独立时必须改 AD：对外依赖名应跟品牌，否则装完仍像「另一个 RHDL」 |
| Spade/`wasm-bindgen` 多 crate × Bitloom 单 CLI | 对标已有；缺的是发布图与 shim，不是新架构范式 |

---

## 4. Deepen — 真独立实现路线（2026-08-20）

### 4.1 Publish graph & naming

| 角色 | 今日内部名 | 建议发布名 | MVP？ |
|------|------------|------------|-------|
| 用户唯一依赖 | `rhdl-prelude` | **`bitloom-prelude`** | 必 |
| builder / macro / hir | `rhdl-*` | **`bitloom-builder` / `bitloom-macro` / `bitloom-hir`** | 必（prelude 传递依赖） |
| Verilog emit | `rhdl-vlog` | **`bitloom-vlog`** | 必（CLI host） |
| CLI | `bitloom` | `bitloom`（已发） | 已有 |
| tick / VCD | `rhdl-sim` | `bitloom-sim` | 第二波（dev-dep） |
| 其它后端/可选 | firrtl, hls, … | `bitloom-*` 按需 | 非 MVP |

crates.io 探测（UA，2026-08-20）：上表 `bitloom-*` 与对应 `rhdl-*` **均 404**。[26]  
命名研究 + AD-2：**不要**为省事发布 `rhdl-prelude`（名称空闲但抵消 Bitloom 锁定、加重与 samitbasu/rhdl 混淆）。[27][21]

### 4.2 架构模式选型

| 选项 | 裁决 | 理由 |
|------|------|------|
| **A. `bitloom-*` 库 + AD-14 host 改用 crates.io 依赖** | **采用** | 对齐 Spade 前缀族、`wasm-bindgen`↔`wasm-bindgen-cli`；设计仍只依赖 prelude[22][23][24] |
| B. 仅 git 依赖脚手架 | 过渡 | 快，但非 crates.io 真独立[28] |
| C. 设计依赖 CLI 库 | 拒绝 | 违 AD-2/6[24] |
| D. FrozenHir 文件协议替代 host | 拒绝（阶段一） | 违 AD-14[23] |

Spade 文档：`cargo install --git … swim` 后对用户工程工作，不要求 clone 编译器树。[17][28]  
`spade-lang` 依赖一串版本对齐的 `spade-*`。[22]  
补充：Swim 仍是 **per-project git/compiler pin**（`swim.lock`），**不是** crates.io 真独立范本；真独立应对标 **wasm-bindgen ↔ wasm-bindgen-cli** 的 registry 拆分，脚手架可学 **wasm-pack new / cargo-generate**；SUS 的 XDG 捆绑数据仅作 firtool/资产类参考。[29][30]

**Umbrella 澄清：** 用户 `cargo add` 的应是 **`bitloom-prelude`**（可对内 re-export），**不是** CLI 包 `bitloom`——避免违 AD-6「设计依赖 CLI」。[24]

### 4.3 实现变更清单（DoD）

当前 DAG：`design → prelude → {builder,macro,hir}`；host → `{design, vlog→hir}`；除 CLI 外皆 `publish=false`。[25][13]

必改：

1. 包改名/发布 `bitloom-{hir,builder,macro,prelude,vlog}`，协调同版本。  
2. 修订 AD-2/6：设计依赖 **`bitloom-prelude`**。  
3. `build_host_cargo`：path → crates.io 版本钉死（与 CLI 同版）。  
4. `--package` 用用户 workspace 的 `cargo metadata` 解析，不再假定 `examples/<name>`。  
5. `cargo bitloom new`。  
6. ATDD：下方验收命令在无 clone 时必须绿。

**验收（真独立定义）：**

```bash
cargo install bitloom@<ver>
cd $(mktemp -d)
cargo bitloom new demo
cargo bitloom build --package demo --out-dir out --manifest-dir .
# out 下存在生成的 .v
```

### 4.4 终态 step-by-step 教程（目标，非今日）

1. rustup：**1.97.1**（MSRV）。  
2. `cargo install bitloom`。  
3. `cargo bitloom new my_mod && cd my_mod`（或在 workspace 内 new）。  
4. 编辑 `#[module]`；保持只依赖 `bitloom-prelude`。  
5. `cargo bitloom build --package my_mod --out-dir ./out --manifest-dir .`。  
6. （可选）`cargo add bitloom-sim --dev` + `tick` 测试。  
7. 工具链开发者仍 clone `TangCan/bitloom`；终端用户不必。

---

## Contrary evidence

- 用户确已成功 `cargo install bitloom` 并看到子命令列表 —— 这只证明 **CLI 分发成功**，不推翻 build 对仓库的依赖。[15]
- `firtool ensure` 等命令可能在无仓库时仍有价值 —— 支持「部分独立」，不支持「完整 RTL 工作流独立」。

---

## Recommendations（服务决策 / 下游）

1. **文档立即诚实化（高置信）** — README：写清 install ≠ 独立工作流；快速开始区分「用户（待真独立）」与「贡献者 clone」。绑定：README。依据 [3][14][15]。
2. **开 Epic：真独立 Publish Graph（高置信）** — 路线 A：`bitloom-*` MVP 五件套 + CLI shim + `new` + ATDD 验收。绑定：epics / AD-2 修订 / architecture spine。依据 [21]–[26]。
3. **AD-2 修订草案（高置信）** — 设计 crate 唯一依赖改为 `bitloom-prelude`；继续禁止 `rhdl`/`rhdl-bits`/`rhdl-rs`。绑定：ARCHITECTURE-SPINE。依据 [21][27]。
4. **可选过渡 git scaffold（中置信）** — 在库未上 crates.io 前，`new` 可写 git 依赖；文档标明过渡。绑定：CLI story。依据 [28]。
5. **搜索消歧（中置信）** — 继续声明与 `bitbloom`、samitbasu/rhdl 无关。依据 [4][5]。

---

## 诚实 step-by-step 教程（今日可行）

### 路径 A — 推荐：以仓库为工作区（可用已 install 的 CLI）

1. 安装工具链：**rustc 1.97.1**（可用仓库 `rust-toolchain.toml`）。
2. （可选）`cargo install bitloom` — 得到 `cargo bitloom`；或在仓库内 `cargo run -p bitloom -- …`。
3. Clone：`git clone https://github.com/TangCan/bitloom.git && cd bitloom`。
4. 出 Verilog：  
   `cargo bitloom build --package counter_ports --out-dir /tmp/bitloom-out --manifest-dir .`
5. 查看 `/tmp/bitloom-out` 下的 `.v`。
6. 仿真/测试：`cargo test -p counter_ports` 或 `just test`。
7. 自有设计：在 `examples/` 新建 crate，依赖 path 的 `rhdl-prelude`，导出 `rhdl_elaborate()`，再对 `--package <name>` 执行 build。

### 路径 B — 仅 install、不 clone：**不可**完成 build

今日只能：`cargo bitloom --help`、`firtool info|ensure`、`sim-engines`（及视环境而定的 `hls`）。**不能**写出用户设计的 Verilog。

### 路径 C — 真独立目标（深化推荐；待实现）

见 §4.4。产品应把路径 C 当作 onboarding 主路径，路径 A 降为贡献者文档。

---

## Open questions

1. ~~最小发布集与命名？~~ → **已答：** MVP = `bitloom-{hir,builder,macro,prelude,vlog}` + CLI；见 §4.1。
2. `build` 在库未发布前是否应检测非 monorepo 并给出明确错误？（仍开放；建议做）
3. docs.rs：CLI crate 是否用 crate-level 长文承载独立教程？（仍开放）
4. 包目录是否同步从 `crates/rhdl-*` 重命名为 `crates/bitloom-*`，或仅改 `package.name`？（实现偏好，不影响裁决）
5. 多 crate 同版本发布：release-plz 工作区策略与 Trusted Publishing 是否一次覆盖全部新包？（工程开放）

---

## Source appendix

| # | Supports | Publisher | Pub date | Accessed | Confidence |
|---|----------|-----------|----------|----------|------------|
| [1] | bitloom 0.1.1 bin-only, repo, MSRV | [crates.io API bitloom](https://crates.io/api/v1/crates/bitloom) | 2026-08-19 | 2026-08-20 | high |
| [2] | deps clap+sha2 only | [deps 0.1.1](https://crates.io/api/v1/crates/bitloom/0.1.1/dependencies) | 2026-08-19 | 2026-08-20 | high |
| [3] | README monorepo quick start + prelude | [readme 0.1.1](https://crates.io/api/v1/crates/bitloom/0.1.1/readme) | 2026-08-19 | 2026-08-20 | high |
| [4] | search confusion with bitbloom | WebSearch “crates.io bitloom” | 2026-08-20 | 2026-08-20 | medium |
| [5] | other rhdl* on crates.io | [crates.io ?q=rhdl](https://crates.io/api/v1/crates?q=rhdl) | 2026-08-20 | 2026-08-20 | high |
| [6] | cargo install = binaries only | [Cargo Book install](https://doc.rust-lang.org/cargo/commands/cargo-install.html) | living | 2026-08-20 | high |
| [7] | local publish=true bin surface | file:crates/bitloom/Cargo.toml | 2026-08-20 | 2026-08-20 | high |
| [8] | cargo-* subcommand discovery | [Cargo external tools](https://doc.rust-lang.org/cargo/reference/external-tools.html) | living | 2026-08-20 | high |
| [9] | mdbook cargo install | [mdBook install](https://rust-lang.github.io/mdBook/guide/installation.html) | living | 2026-08-20 | high |
| [10] | mdbook CLI on user books | [mdBook CLI](https://rust-lang.github.io/mdBook/cli/index.html) | living | 2026-08-20 | high |
| [11] | rhdl-prelude missing on crates.io | crates.io API error for rhdl-prelude | 2026-08-20 | 2026-08-20 | high |
| [12] | docs.rs sparse for bitloom | [docs.rs crate bitloom 0.1.1](https://docs.rs/crate/bitloom/0.1.1) | 2026-08-19 | 2026-08-20 | medium |
| [13] | libraries publish=false | workspace crates/*/Cargo.toml | 2026-08-20 | 2026-08-20 | high |
| [14] | path shim to hir/vlog/design | file:crates/bitloom/src/main.rs | 2026-08-20 | 2026-08-20 | high |
| [15] | empty-dir build failure | empirical `/tmp` run with cargo-bitloom | 2026-08-20 | 2026-08-20 | high |
| [16] | nextest install-then-use any project | [nexte.st from-source](https://nexte.st/docs/installation/from-source/) | living | 2026-08-20 | high |
| [17] | Spade Swim install without cloning Spade tree | [Spade install guide](https://docs.spade-lang.org/guide_installation.html) | living | 2026-08-20 | high |
| [18] | sus_compiler install then compile user files | [crates.io sus_compiler](https://crates.io/crates/sus_compiler) | ~2026-08-18 | 2026-08-20 | high |
| [19] | cargo-expand install + optional toolchain pieces | [crates.io cargo-expand](https://crates.io/crates/cargo-expand) | 2026-08-19 | 2026-08-20 | high |
| [20] | wasm-pack still needs rustc after CLI install | [wasm-pack prerequisites](https://rustwasm.github.io/docs/wasm-pack/prerequisites/index.html) | living | 2026-08-20 | high |
| [21] | AD-2 Bitloom publish identity / forbid rhdl names | file:ARCHITECTURE-SPINE.md AD-2 | 2026-08-19 | 2026-08-20 | high |
| [22] | spade-lang depends on spade-* crate family | [spade-lang 0.16.0 deps](https://crates.io/api/v1/crates/spade-lang/0.16.0/dependencies) | 0.16.0 | 2026-08-20 | high |
| [23] | AD-14 host shim elaborate+emit in-process | file:ARCHITECTURE-SPINE.md AD-14 | adopted | 2026-08-20 | high |
| [24] | AD-6 design only prelude; not CLI | file:ARCHITECTURE-SPINE.md AD-6 | adopted | 2026-08-20 | high |
| [25] | prelude/vlog/hir Cargo.toml dependency DAG | file:crates/rhdl-{prelude,vlog,hir,builder,macro}/Cargo.toml | 2026-08-20 | 2026-08-20 | high |
| [26] | bitloom-* and rhdl-* names 404 on crates.io | crates.io API probes | 2026-08-20 | 2026-08-20 | high |
| [27] | Bitloom brand lock; avoid rhdl* public surface | [naming research](../technical-rhdl-rename-alternatives-product-naming-2026-08-19/research.md) | 2026-08-19 | 2026-08-20 | high |
| [28] | Spade install via git swim (user project, not clone compiler) | [Spade install](https://docs.spade-lang.org/guide_installation.html) | living | 2026-08-20 | high |
| [29] | wasm-bindgen lib vs wasm-bindgen-cli split | crates.io API wasm-bindgen / wasm-bindgen-cli | 0.2.127 | 2026-08-20 | high |
| [30] | wasm-pack new uses cargo-generate templates | [wasm-pack new](https://wasm-bindgen.github.io/wasm-pack/book/commands/new.html) | living | 2026-08-20 | high |

---

## Staleness map

| Claim class | Window | Re-check by | Notes |
|-------------|--------|-------------|-------|
| version / name availability ([1][2][11][26]) | ≤ 1 mo | **2026-09-01** | 若有人抢注 `bitloom-*` 需立刻改方案 |
| process / tutorial ([3][14][15]) | ≤ 1–3 mo | 2026-09-20 | 真独立落地后整节改写 |
| landscape / peer patterns ([6][8][9][22][29]) | ≤ 12 mo | 2027-08-20 | 惯例较稳 |
| policy AD text ([21][23][24]) | on spine change | — | AD 修订即刷新 |

**Earliest refresh:** 2026-09-01（名称占用）。
