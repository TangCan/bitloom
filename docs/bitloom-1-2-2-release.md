# Bitloom 1.2.2 发布记录

状态（2026-10-14）：**十个 Bitloom 包均已发布到 crates.io，版本 1.2.2**。release-plz 发布提交为 [`03b8dac`](https://github.com/TangCan/bitloom/commit/03b8dac629c59661c9f1de583b81a9a058f173b5)；workspace 发布顺序修正合并至 `main` 的提交为 [`476ac44`](https://github.com/TangCan/bitloom/commit/476ac4460f320ef098a4ad6c908df146fc4920fb)。各包的 crate 专属标签与 GitHub Release 均已创建。

本补丁版优化 `bitloom-sim` 的热点求值路径，保持模拟行为不变。其余 crate 随共享版本与依赖锁步发布。

## 已发布包

校验和取自 crates.io API 的 `version.checksum` 字段。

| 包 | 版本 | SHA-256 |
|---|---:|---|
| `bitloom-hir` | 1.2.2 | `442095a2348fbb85003e8ecf9a658bc5e14b65ae67cc0df35e36dcdbc81f969e` |
| `bitloom-builder` | 1.2.2 | `3f6d8eceddc9a2f62774381bf387b023702c28825b90fda74a0d205a93fa412b` |
| `bitloom-macro` | 1.2.2 | `2e47fec378165822cd7191bc41188eceedb4081c772e0d9fdbdd5f1ce95c4f11` |
| `bitloom-vlog` | 1.2.2 | `63fbf9c4b602fe31b30498f238b0b728b128abf1c0ca763d906f1235ab56f227` |
| `bitloom-sim` | 1.2.2 | `e2537c07a2807046e585bac460068699e021eefe7106bdfb8e7646c109da7bb3` |
| `bitloom-prelude` | 1.2.2 | `16505759ac35f373fe9e8a43c4e24854f9fed37871a1ce53d8b6025c877d77c5` |
| `bitloom-firrtl` | 1.2.2 | `6a439e328a3314c4fea2f0ade7ed23dae0ecfe7d7a41951a4368746ed2721747` |
| `bitloom-viz` | 1.2.2 | `e87c3a8fef544088743a6e7900733e829db352a45ce4381696a841c34801138d` |
| `bitloom-lsp` | 1.2.2 | `29889defa1fc700c366962d3691b93cadf608310aadbcd0c177bcbfac40e4980` |
| `bitloom` | 1.2.2 | `70edfb0e63c496c9be9d0294f221a624bb58891a2964d5fb9f323e1d4ed6bb72` |

## 验收

- crates.io API 确认上述十个包的 `1.2.2` 版本存在；校验和已记录于上表。
- GitHub Actions [发布工作流 run 38504659979](https://github.com/TangCan/bitloom/actions/runs/38504659979) 成功，创建全部 crate 标签与 GitHub Release。
- 合并后的主分支 CI [run 38504649407](https://github.com/TangCan/bitloom/actions/runs/38504649407) 成功。
- 发布完成后再次运行 [release-plz `release-pr` run 38506427862](https://github.com/TangCan/bitloom/actions/runs/38506427862)：十个包均显示 `already up to date`，输出 `prs: []`。

