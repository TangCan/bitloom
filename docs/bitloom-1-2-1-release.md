# Bitloom 1.2.1 发布记录

状态（2026-10-13）：**十个 Bitloom 包均已发布到 crates.io，版本 1.2.1**。上传源码提交为 `7fecc30`（`chore(release): prepare Bitloom 1.2.1`）。`v1.2.1` annotated tag 指向该提交；发布记录提交 `c96e767` 已推送到 `main`。 [GitHub Release](https://github.com/TangCan/bitloom/releases/tag/v1.2.1) 已发布。

本补丁版优化 `bitloom-sim` 覆盖率采样和 Mux 分支记录热点，保持模拟结果、周期及覆盖率语义不变。公开 API SemVer 检查没有要求版本升级，补丁号适用。

## 已发布包

下列校验和取自 crates.io sparse index 的 `cksum` 字段；索引均确认 1.2.1 可用。

| 包 | 版本 | SHA-256 |
|---|---:|---|
| `bitloom-macro` | 1.2.1 | `f6a3a0d36a9cfed69f4e070f64c787b0362634a5a490e90f27e39db65ee7ec74` |
| `bitloom-hir` | 1.2.1 | `94a812b6909bdb62e264399f38008252455d8fa7e891b76062471ebe6d9f10de` |
| `bitloom-builder` | 1.2.1 | `24b397e0494e8f4f0c81fe75d3b6320e017b4819c8c2747ec82cb47111d4161c` |
| `bitloom-vlog` | 1.2.1 | `edb1498cc5d58002b7555a86cc019206d338106da04892f2cc92f15c215c2e13` |
| `bitloom-sim` | 1.2.1 | `8dabfcafe51f86a0462c39a3dfd5d55864e052cb93993641fa07783280ed5bf3` |
| `bitloom-prelude` | 1.2.1 | `df37f48d5e5adac987c1ca2c94b933a5bd910ded3b23ca4e4d50abb0d2103054` |
| `bitloom-firrtl` | 1.2.1 | `d09ad18b3c5e47f201f1c56bfca246f5ab7be8cb38c30ef8b08c7f74b22716c4` |
| `bitloom-viz` | 1.2.1 | `414921fe37da7ad810a605e7608050351f86e6c1c71d4a1298a019294c6d8d8f` |
| `bitloom` | 1.2.1 | `712a6f994ce5467caf0b796b93ed64514b72647f5c8598537deb801f573e7597` |
| `bitloom-lsp` | 1.2.1 | `574efa48961a9c235fd643f51fb6de02a66e7fa56c0f0b901369b0cc0959c286` |

## 验收

- `cargo publish --workspace --dry-run --allow-dirty`：十包预演成功；实际上传时又按依赖顺序逐包运行 `cargo publish -p <package> --dry-run --locked`，随后正式上传并等待 crates.io 确认可用。
- `just semver-check`：`bitloom-prelude`、`bitloom-sim`、`bitloom-firrtl` 各 196 项通过，均无 SemVer API 更新要求。
- GitHub Actions [CI run 38385362859](https://github.com/TangCan/bitloom/actions/runs/38385362859)：success。
- 使用隔离 `CARGO_HOME` 从 crates.io 安装 `bitloom 1.2.1` 与 `bitloom-lsp 1.2.1`；`cargo install --list` 确认版本，CLI `--help` 正常，LSP `initialize` 返回 `serverInfo.version = 1.2.1`。
- 已安装 CLI 在仓库外执行 `new`、`build`、`gen-func`、`gen-cycle` 成功；两个生成的仿真 crate 均从 crates.io 解析依赖并成功 `cargo run`。

功能仿真生成器当前声明的 `bitloom-hir` 最低版本仍为 1.2.0；隔离注册表验收解析到已发布的 1.2.1 并成功运行。功能生成 crate 的编译输出有既存 `unused_mut` 警告，不影响生成与运行。

发布候选与发布记录均已推送；`v1.2.1` 标签及 GitHub Release 已创建。不得据此改变 FR189/Epic122 deferred 或 NFR91 的状态。
