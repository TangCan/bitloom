# FR35 / FR50 — Bitloom HLS product path (Bambu)

Bitloom **supports** HLS as a product path (概述 §1.3.8)：`#[hls]` / `cargo bitloom hls` 发射宿主 C，并调用钉死的外挂后端产出可综合 RTL。Bitloom **永不**实现树内调度器（AD-25）。

## 钉死后端

| 项 | 值 |
| --- | --- |
| 后端 | **PandA Bambu**（不选 Vitis/XLS） |
| 版本 | **2024.10** |
| AppImage | <https://release.bambuhls.eu/bambu-2024.10.AppImage> |
| 路径覆盖 | `BITLOOM_BAMBU_PATH`（兼容 `RHDL_BAMBU_PATH`） |

风险记录：[`nfr14-risk-hls.md`](../_agile-output/implementation-artifacts/nfr14-risk-hls.md)。

## 默认文档路径

```bash
# 帮助
cargo run -p bitloom -- hls --help

# 仅发射 C（检查夹具；不算 RTL 成功）
cargo run -p bitloom -- hls --function add --out-dir target/bitloom-hls --emit-only

# 产品路径：需要已安装的 Bambu 2024.10
export BITLOOM_BAMBU_PATH=/path/to/bambu   # 或把 bambu 放进 PATH
cargo run -p bitloom -- hls --function add --out-dir target/bitloom-hls
# 成功时打印 ok=<synthesizable .v/.sv>
```

语言表面：`#[rhdl::hls]` / `#[bitloom::hls]` 标记算法函数（宏不调度，仅保留标记）。

## 失败语义

- **后端缺失：** 非零退出 + `error:` 说明如何安装 2024.10 / 设置 `BITLOOM_BAMBU_PATH`（**不是**「永久 unsupported」）。
- **后端非零退出或无 RTL 工件：** 非零退出；不得 silent 成功。

## 烟测 / CI

见 Story 24.3：`scripts/hls-smoke.sh` 与 CI `hls-smoke` job（夹具 + 版本/缓存策略）。
