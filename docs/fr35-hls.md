# FR35 / FR50 / FR76 / FR95 / FR96 — Bitloom HLS product paths

Bitloom **supports** HLS as a product path（概述 §1.3.8）。公开品牌：**Bitloom**（`bitloom` / `cargo bitloom hls`）。

修订后 **AD-25**（Phase 12 / Path B）允许两条路径并存：

| 路径 | 角色 | 可否单独关闭 FR95 |
| --- | --- | --- |
| **树内** `schedule_in_tree` / `cargo bitloom hls --in-tree`（loop-unroll MVP） | **FR95 完成面** | **是**（须可检查 schedule IR / 可选 RTL stub；**不**调用 Bambu） |
| **外挂** 发射宿主 C → 钉死 **PandA Bambu**（stub / `BITLOOM_HLS_USE_REAL`） | **FR35 / FR76 / FR86** 可选/对照诚实路径 | **否**（不得单独满足 FR95） |

> **诚实边界：** 树内 MVP 是文档化子集（当前演示：**loop-unroll**；可选 pipeline II），产出可检查的 schedule IR 与诚实标注 `in-tree-mvp` 的 RTL stub——**不是**商业级完整 HLS 编译器。外挂 stub 绿 ≠ 调度质量，亦 ≠ FR95。**FR96** 在调度前将闭包 dissolve/inline 为数据流变换并进入树内 FR95 路径（≠ 仅外挂 FR76 dissolve）。**商业深度（流水阶段 / II）→ Phase 13 FR110** — [`fr110-hls-commercial-depth.md`](fr110-hls-commercial-depth.md)。**Handshake / 动态 DF 默认可综合 → Phase 14 FR121** — [`fr121-handshake-default.md`](fr121-handshake-default.md)。

## 钉死后端（外挂路径）

| 项 | 值 |
| --- | --- |
| 后端 | **PandA Bambu**（不选 Vitis/XLS） |
| 版本 | **2024.10** |
| AppImage | <https://release.bambuhls.eu/bambu-2024.10.AppImage> |
| 路径覆盖 | `BITLOOM_BAMBU_PATH`（兼容 `RHDL_BAMBU_PATH`） |

风险记录：[`nfr14-risk-hls.md`](../_agile-output/implementation-artifacts/nfr14-risk-hls.md)（Epic 24 外挂门禁）；Epic 41 树内：[`nfr14-risk-epic41-in-tree-hls.md`](../_agile-output/implementation-artifacts/nfr14-risk-epic41-in-tree-hls.md)。

## FR95 — 树内调度 MVP（完成面）

```bash
# 树内 loop-unroll：写出 {fn}.schedule.json + {fn}.v，永不 spawn bambu
# （CLI --in-tree 默认经 --dataflow dissolve → FR96 → FR95；常打印 fr96=true）
cargo run -p bitloom -- hls --in-tree --function map_add1 --dataflow add1 --unroll 4 \
  --out-dir target/bitloom-hls-in-tree
# 成功时打印 path=in-tree fr95=true fr96=true … 与 ok_schedule=… / ok_rtl=…
```

**CLI ↔ 库 API：** `cargo bitloom hls --in-tree` **总是**先 dissolve `--dataflow`（FR96），再进树内 schedule——因此成功路径几乎总带 `fr96=true`。若需要**裸 FR95**（无闭包 dissolve、仅 `HlsDataflowOp`），使用库 API `schedule_in_tree` / `run_hls_in_tree`（见下）；CLI 当前无「跳过 dissolve」开关。

库 API（裸 Op，无闭包 — FR95 only）：

```ignore
use bitloom::hls::{
    emit_in_tree_schedule, schedule_in_tree, HlsDataflowOp, InTreeScheduleKind,
};

let artifact = schedule_in_tree(
    "map_add1",
    HlsDataflowOp::AddConst(1),
    InTreeScheduleKind::LoopUnroll { trip_count: 4 },
)?;
emit_in_tree_schedule(&artifact, out_dir)?;
```

| 项 | 值 |
| --- | --- |
| 文档化子集 | **loop-unroll**（`--unroll N`）；可选 `InTreeScheduleKind::Pipeline` |
| 产物 | `{fn}.schedule.json`（含 `fr95` / `loop-unroll` / stages）+ `{fn}.v`（`in-tree-mvp`） |
| ATDD | `cargo test -p bitloom --test fr95_in_tree_hls_schedule` |
| 非目标（本 MVP） | 完整 allocation/binding（Handshake 默认可综合 → **FR121** / [`fr121-handshake-default.md`](fr121-handshake-default.md)） |

语言表面：`#[rhdl::hls]` / `#[bitloom::hls]` 标记算法函数（宏不调度；树内调度在库 API / CLI `--in-tree`）。

## FR96 — 调度前闭包数据流变换（完成面）

调度前将闭包作为**数据流变换单元** dissolve/inline，再进入 Story 41.2 的树内 `schedule_in_tree` 路径（修订 AD-25 + AD-18）。**仅**外挂 FR76 dissolve **不算**关闭 FR96。

```ignore
use bitloom::hls::{
    emit_in_tree_schedule, schedule_in_tree_from_transform, HlsDataflowOp, InTreeScheduleKind,
};

let artifact = schedule_in_tree_from_transform(
    "map_xor",
    &[], // capturing / wrong-path tokens → readable Err before schedule
    InTreeScheduleKind::LoopUnroll { trip_count: 3 },
    || HlsDataflowOp::XorConst(0xa5), // dissolved here — never enters schedule as Fn
)?;
emit_in_tree_schedule(&artifact, out_dir)?;
// schedule_ir cites fr96 + fr95; no Bambu
```

| 项 | 值 |
| --- | --- |
| API | `schedule_in_tree_from_transform` / `run_hls_in_tree_from_transform` |
| 约束 | `check_hls_dataflow_closure`；捕获 / 错路径 → 调度前可读失败（不得入 tick） |
| 产物 | 同 FR95 schedule IR，另含 `"fr96": true` / `dataflow_transform=dissolved` |
| ATDD | `cargo test -p bitloom --test fr96_hls_closure_dataflow_transform` |
| 消歧 | ≠ 仅 FR76 外挂 C dissolve；≠ FR74/75 synthesizable inline；≠ FR78 bridge host |

### 闭包合同交叉链（FR72–78 ↔ FR96）

| FR | 合同 | 文档锚 |
| --- | --- | --- |
| **FR72** | 受控闭包总合同 / AD-18（非捕获 elaborate-time `Fn`；捕获禁入 tick） | [`ARCHITECTURE-SPINE` AD-18](../_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md) |
| **FR73** | 生成器闭包（Mem init / factory） | [`docs/fr22-construct-bar.md`](fr22-construct-bar.md) |
| **FR74** / **FR75** | `SynthesizableClosure` comb/seq inline | [`docs/fr22-construct-bar.md`](fr22-construct-bar.md) |
| **FR76** | HLS 数据流闭包 dissolve → C / 外挂（本文件 FR76 节） | 下文 FR76 |
| **FR77** | IP 生成器闭包（`Crc8Lut`） | [`docs/ip/README.md`](ip/README.md) |
| **FR78** | 桥接适配器闭包模板 `start_wait_complete` | [`docs/fr78-bridge-adapter-closures.md`](fr78-bridge-adapter-closures.md) |
| **FR96** | **本节：** dissolve/inline → **树内** FR95 schedule | 本节 |

## 外挂默认文档路径（FR35）

```bash
# 帮助
cargo run -p bitloom -- hls --help

# 仅发射 C（检查夹具；不算 RTL 成功；不算 FR95）
cargo run -p bitloom -- hls --function add --out-dir target/bitloom-hls --emit-only

# FR76：溶解数据流变换后再发射（emit-only）
cargo run -p bitloom -- hls --function map_xor --dataflow xor_a5 --out-dir target/bitloom-hls --emit-only

# 产品外挂路径：需要已安装的 Bambu 2024.10（不得单独勾选 FR95）
export BITLOOM_BAMBU_PATH=/path/to/bambu   # 或把 bambu 放进 PATH
cargo run -p bitloom -- hls --function add --out-dir target/bitloom-hls
# 成功时打印 ok=<synthesizable .v/.sv>
```

## FR76 — HLS 数据流闭包（D1 / Cap-R-62/71）

决策表 **D1**：HLS **自由**闭包仅允许在 **AD-25 HLS** 路径（外挂与树内 FR95/FR96 调度前 dissolve）；可综合 comb/seq 仍走 `SynthesizableClosure`（FR74），不得借 HLS 自由绕过。

**消歧（≠ FR47）：** 本路径的「数据流闭包」是 elaborate/emit-prep 期消解的用户 `Fn` → C / schedule 描述符。**不是** FR47「sim generators」。Phase 7 英文偶用 *closure*（闭环）也无关。

| 项 | 值 |
| --- | --- |
| API | `bitloom::hls::dissolve_dataflow_transform` → `HlsDataflowOp` → C；`run_hls_dissolved` / CLI `--dataflow`；树内消费同一 `HlsDataflowOp`（FR96 经 `schedule_in_tree_from_transform`） |
| 约束类检查 | `check_hls_dataflow_closure(HlsFree, violations)`；捕获/错路径 token → 可读失败 |
| 消解时机 | **调度/降低前**；产物无 `Fn` / closure 残留（NFR36） |
| 透明抽检 | Story **29.4**：C / stub RTL 无闭包 IR；矩阵 `fr76_fr77_nfr36_transparency_matrix` |
| CLI aliases | `add` \| `identity` \| `add1` \| `xor_a5` |
| ATDD | `cargo test -p bitloom --test fr76_hls_dataflow_closure` |

最小库用法（外挂 dissolve）：

```ignore
use bitloom::hls::{dissolve_dataflow_transform, run_hls_dissolved, HlsDataflowOp};

let dissolved = dissolve_dataflow_transform("map_xor", &[], || {
    HlsDataflowOp::XorConst(0xa5) // closure dissolved here — never enters C/RTL
})?;
run_hls_dissolved(&dissolved, out_dir, /* emit_only */ true)?;
```

## 失败语义

- **树内非法参数（如 `trip_count=0`）：** 可读 `error:`；不得 silent 成功。
- **外挂后端缺失：** 非零退出 + `error:` 说明如何安装 2024.10 / 设置 `BITLOOM_BAMBU_PATH`。
- **外挂后端非零退出或无 RTL 工件：** 非零退出；不得 silent 成功。
- **非法数据流闭包（捕获 / 错路径）：** dissolve 前失败；不得带着 `Fn` 进后端或树内 schedule（FR76 / FR96）。

## 烟测 / CI

| 项 | 位置 |
| --- | --- |
| 发布/本地烟测 | `just hls-smoke` → [`scripts/hls-smoke.sh`](../scripts/hls-smoke.sh) |
| CI job | `.github/workflows/ci.yml` → job **`hls-smoke`**（失败不 ignore） |
| CI stub | [`scripts/fixtures/bambu-ci-stub.sh`](../scripts/fixtures/bambu-ci-stub.sh)（接线 + 可综合 `.v`；**非真实 HLS 质量**；**≠ FR95**） |
| 真 Bambu | `BITLOOM_HLS_USE_REAL=1` + `BITLOOM_BAMBU_PATH` 或缓存 AppImage |
| FR95 ATDD | `cargo test -p bitloom --test fr95_in_tree_hls_schedule` |
| FR96 ATDD | `cargo test -p bitloom --test fr96_hls_closure_dataflow_transform` |
| Epic 41 收口 | `cargo test -p bitloom --test fr95_fr96_epic41_closeout` |
| FR76 ATDD | `cargo test -p bitloom --test fr76_hls_dataflow_closure` |
| FR76+FR77 透明矩阵（29.4） | `cargo test -p bitloom --test fr76_fr77_nfr36_transparency_matrix` |

**Locked:** CI / `just hls-smoke` 默认走 stub；stub 绿 ≠ 调度质量，**不得**单独关闭 FR95。真机入口仅为 `BITLOOM_HLS_USE_REAL=1`。

### FR88 / Epic 37 — Path B（外挂诚实选型）

Epic 37 **显式选择 Path B**：保持 **stub 默认**；**不**落地 CI optional/夜间真机 Bambu job。真机仍为显式环境变量入口。不得把 stub 绿写成「HLS 质量已验」。**树内 FR95** 由 Epic 41 / `fr95_in_tree_hls_schedule` 验收，与 stub 无关；**FR96** 由 `fr96_hls_closure_dataflow_transform` 验收。**Epic 41 已关闭**（Story 41.4）：FR95/FR96 收口 ATDD 见 `fr95_fr96_epic41_closeout`；外挂路径仍诚实可选，**不得单独满足 FR95**。

常驻覆盖：`cargo test -p bitloom --test hls_smoke`（缺后端可读失败）亦在主 `test` job 中运行。
