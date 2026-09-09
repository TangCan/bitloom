# FR35 / FR50 / FR76 — Bitloom HLS product path (Bambu)

Bitloom **supports** HLS as a product path (概述 §1.3.8)：`#[hls]` / `cargo bitloom hls` 发射宿主 C，并调用钉死的外挂后端产出可综合 RTL。Bitloom **永不**实现树内调度器（AD-25）。

## 钉死后端

| 项 | 值 |
| --- | --- |
| 后端 | **PandA Bambu**（不选 Vitis/XLS） |
| 版本 | **2024.10** |
| AppImage | <https://release.bambuhls.eu/bambu-2024.10.AppImage> |
| 路径覆盖 | `BITLOOM_BAMBU_PATH`（兼容 `RHDL_BAMBU_PATH`） |

风险记录：[`nfr14-risk-hls.md`](../_agile-output/implementation-artifacts/nfr14-risk-hls.md)；Epic 29 HLS/IP 闭包：[`nfr14-risk-epic29-hls-ip-closures.md`](../_agile-output/implementation-artifacts/nfr14-risk-epic29-hls-ip-closures.md)。

## 默认文档路径

```bash
# 帮助
cargo run -p bitloom -- hls --help

# 仅发射 C（检查夹具；不算 RTL 成功）
cargo run -p bitloom -- hls --function add --out-dir target/bitloom-hls --emit-only

# FR76：溶解数据流变换后再发射（emit-only）
cargo run -p bitloom -- hls --function map_xor --dataflow xor_a5 --out-dir target/bitloom-hls --emit-only

# 产品路径：需要已安装的 Bambu 2024.10
export BITLOOM_BAMBU_PATH=/path/to/bambu   # 或把 bambu 放进 PATH
cargo run -p bitloom -- hls --function add --out-dir target/bitloom-hls
# 成功时打印 ok=<synthesizable .v/.sv>
```

语言表面：`#[rhdl::hls]` / `#[bitloom::hls]` 标记算法函数（宏不调度，仅保留标记）。

## FR76 — HLS 数据流闭包（D1 / Cap-R-62/71）

决策表 **D1**：HLS **自由**闭包仅允许在 **AD-25 外挂 HLS** 路径（与功能侧）；可综合 comb/seq 仍走 `SynthesizableClosure`（FR74），不得借 HLS 自由绕过。

**消歧（≠ FR47）：** 本路径的「数据流闭包」是 elaborate/emit-prep 期消解的用户 `Fn` → C。**不是** FR47「sim generators」（`generate_*_sim` / `gen-func` / `gen-cycle` 双视图 crate 生成）。Phase 7 英文偶用 *closure*（闭环）也无关。

| 项 | 值 |
| --- | --- |
| API | `bitloom::hls::dissolve_dataflow_transform` → `HlsDataflowOp` → C；`run_hls_dissolved` / CLI `--dataflow` |
| 约束类检查 | `check_hls_dataflow_closure(HlsFree, violations)`；捕获/错路径 token → 可读失败 |
| 消解时机 | **调度/降低前**（发射 C 时）；产物无 `Fn` / closure 残留（NFR36） |
| 透明抽检 | Story **29.4**：C / stub RTL 无闭包 IR；矩阵 `fr76_fr77_nfr36_transparency_matrix`（Cap-R-64 / NFR36） |
| 非目标 | 树内 scheduler（AD-25 / FR86）；把 `HlsFree` 用到可综合 comb/seq/IP |
| CLI aliases | `add` \| `identity` \| `add1` \| `xor_a5` |
| ATDD | `cargo test -p bitloom --test fr76_hls_dataflow_closure`（emit-only + bambu-ci-stub）；透明矩阵见上 |

最小库用法：

```ignore
use bitloom::hls::{dissolve_dataflow_transform, run_hls_dissolved, HlsDataflowOp};

let dissolved = dissolve_dataflow_transform("map_xor", &[], || {
    HlsDataflowOp::XorConst(0xa5) // closure dissolved here — never enters C/RTL
})?;
// emit-only intermediate, or invoke pinned Bambu / CI stub:
run_hls_dissolved(&dissolved, out_dir, /* emit_only */ true)?;
```

## 失败语义

- **后端缺失：** 非零退出 + `error:` 说明如何安装 2024.10 / 设置 `BITLOOM_BAMBU_PATH`（产品路径始终可用；缺工具是安装问题，不是功能关闭）。
- **后端非零退出或无 RTL 工件：** 非零退出；不得 silent 成功。
- **非法数据流闭包（捕获 / 错路径）：** dissolve 前失败；不得带着 `Fn` 进后端。

## 烟测 / CI

| 项 | 位置 |
| --- | --- |
| 发布/本地烟测 | `just hls-smoke` → [`scripts/hls-smoke.sh`](../scripts/hls-smoke.sh) |
| CI job | `.github/workflows/ci.yml` → job **`hls-smoke`**（失败不 ignore） |
| CI stub | [`scripts/fixtures/bambu-ci-stub.sh`](../scripts/fixtures/bambu-ci-stub.sh)（接线 + 可综合 `.v`；**非真实 HLS 质量**） |
| 真 Bambu | `BITLOOM_HLS_USE_REAL=1` + `BITLOOM_BAMBU_PATH` 或缓存 `${BITLOOM_HLS_CACHE:-~/.cache/bitloom-hls}/bambu-2024.10.AppImage` |
| FR76 ATDD | `cargo test -p bitloom --test fr76_hls_dataflow_closure` |
| FR76+FR77 透明矩阵（29.4） | `cargo test -p bitloom --test fr76_fr77_nfr36_transparency_matrix` |

**Locked:** CI / `just hls-smoke` 默认走 stub；stub 绿 ≠ 调度质量。真机入口仅为 `BITLOOM_HLS_USE_REAL=1`。

### FR88 / Epic 37 — Path B（本阶段选型）

Epic 37 **显式选择 Path B**：保持 **stub 默认**；**不**落地 CI optional/夜间真机 Bambu job。真机仍为显式环境变量入口（上表 `BITLOOM_HLS_USE_REAL=1` + `BITLOOM_BAMBU_PATH` / 缓存 AppImage）。不得把 stub 绿写成「HLS 质量已验」；若将来另开夜间真机 job，失败**不得** `continue-on-error` / ignore。树内 HLS 调度仍为非目标（AD-25 / FR86 / FR93）。deferred 收口见 `_agile-output/implementation-artifacts/deferred-work.md`（「本阶段选 B」）。第二算法烟测夹具仍可另开可选故事。

常驻覆盖：`cargo test -p bitloom --test hls_smoke`（缺后端可读失败）亦在主 `test` job 中运行。
