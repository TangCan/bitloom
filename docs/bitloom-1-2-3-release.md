# Bitloom 1.2.3 发布记录

状态（2026-10-14）：**十个 Bitloom 包均已发布至 crates.io，版本 1.2.3**。发布由 Trusted Publishing / OIDC 工作流完成。

## 发布证据

- 发布 PR：[PR #5](https://github.com/TangCan/bitloom/pull/5)，合并提交 [`bd23270`](https://github.com/TangCan/bitloom/commit/bd23270d2bbdc89a1009ffa051dc00794455e9d9)。
- release-plz 版本源码标签指向提交 [`2ead5c9`](https://github.com/TangCan/bitloom/commit/2ead5c96af1b6d8e828b29b709398f979b5c4209)，该提交包含在 main 合并历史中。
- 合并后 main CI：[run 38535300377](https://github.com/TangCan/bitloom/actions/runs/38535300377) 成功，含 FR200 外部 IP 试点。
- 发布工作流：[run 38541036520](https://github.com/TangCan/bitloom/actions/runs/38541036520) 成功；日志确认十个包均已发布并创建 crate 专属 tag 与 GitHub Release。
- crates.io 消费验证：`cargo info --registry crates-io bitloom@1.2.3` 成功下载 `bitloom` 1.2.3。

## 已发布包与校验和

SHA-256 对应 crates.io 下载的 `.crate` 归档。

| 包 | 版本 | SHA-256 |
|---|---:|---|
| `bitloom-hir` | 1.2.3 | `7c98655aa736331d4edd3144306ea8aa8cc261a4dd0840b1d0fe9112e028417f` |
| `bitloom-builder` | 1.2.3 | `a15f4ac4ad3a74a05ef539329ce1546f50cddd1e821700b3ce51da1c61168cfa` |
| `bitloom-macro` | 1.2.3 | `5cb72416e4bd845bf2d529b1c2e19522c207ad81e5e6b0da01bc9bfe9610d848` |
| `bitloom-vlog` | 1.2.3 | `682dd5cbea638cfce9b409c3daf7f73b1dca4bb044db932f1a50018cad7a5a7b` |
| `bitloom-sim` | 1.2.3 | `88a2cf72fa49c24353434ac16de0a1d3645891a65066c3504946229095b7f73c` |
| `bitloom-prelude` | 1.2.3 | `056ad0fd9e9042bc938887edb7a6af0ed742d1fd8c68557f43910c853f95a25c` |
| `bitloom-firrtl` | 1.2.3 | `4c6cb41febb1c96f95520fd4aa4244f5e497b22409189fae363cd2abf46df137` |
| `bitloom-viz` | 1.2.3 | `c0b320c7f6aaaad48b2150f4067eea73fc0e8ee954b4bfced2f7c14d834d5eb0` |
| `bitloom-lsp` | 1.2.3 | `5fa09d19f54aefe07f8306f4d29ddb47f67ae35f7b06aa2a847b5fb07b024114` |
| `bitloom` | 1.2.3 | `3deb6ec8b030ec97ae13cc995a8b5d46ada5da1d857875b2a4ad1aa16bdfcaaf` |

## 发布后消费验证

- 在发布提交 `bd23270` 上运行 `cargo test -p bitloom --test standalone_empty_dir -- --exact empty_dir_new_then_build_emits_verilog --nocapture`：通过。测试中的 standalone host crate 从 crates.io 解析已发布的 `bitloom-hir` / `bitloom-vlog`，构建成功并生成非空 Verilog；没有走“候选版本尚未发布”的延后分支。
- 在干净安装根目录运行 `cargo install bitloom --version 1.2.3 --locked --root /tmp/bitloom-install-123`：成功，安装 `cargo-bitloom`。
- 使用安装后的 CLI 执行 `new` 和 `build`，从 crates.io 下载 1.2.3 host 依赖，成功 elaboration 并生成 `BitloomReleaseSmoke.v`。
