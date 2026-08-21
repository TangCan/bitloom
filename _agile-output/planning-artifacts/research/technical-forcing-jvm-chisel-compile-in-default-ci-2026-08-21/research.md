---
title: 'technical research: forcing JVM Chisel compile in default CI'
type: 'technical'
topic: 'forcing JVM Chisel compile in default CI'
decision: 'Make FR28 true JVM compile a hard default CI/just-test gate (no silent skip)'
source: 'run'
status: complete
preset: 'standard'
validation: 'normal'
created: '2026-08-21'
updated: '2026-08-21'
verified_claims: 4
unverified_claims: 1
---

# technical research: forcing JVM Chisel compile in default CI

**Decision this research serves:** Make FR28 true JVM compile a hard default CI/`just test` gate (fail red; no silent skip).

## Executive summary

**做：** 在 GitHub Actions 增加**独立 required job**（推荐名 `fr28-chisel-jvm`）：Temurin **Java 17** + `actions/setup-java` 的 `cache: sbt` + `sbt/setup-sbt@v1`，对 Bitloom 黄金夹具生成的 `.scala` 跑 `sbt -batch compile`；脚本在缺 JDK/sbt 时必须 **非零退出**（禁止今日的 skip=0）。Rust 谓词测试仍保留在现有 `test` job，与 JVM job **并行**。[1][2][4]

**驱动证据：** (1) sbt 官方 GHA 配方就是 JDK17 + sbt cache + setup-sbt [1]；(2) Chisel 官方建议 LTS Java 17+，且必须钉死匹配的 `chisel` + `chisel-plugin` [2][3]；(3) 硬门槛不要用 `continue-on-error` [4]。

**最大 caveat：** 公开数据**没有**「单文件 Chisel 7.14 Module 在 GHA 上冷/热编译墙钟」的基准——首次依赖拉取可能是分钟级，必须用缓存；实现故事要实测后再钉 `timeout-minutes` [5]。

## Landscape & maturity

主流做法已经固化：在 GHA 上编译 Scala/Chisel 用户工程，几乎都走 **setup-java（Temurin 17）→ 可选 sbt 依赖缓存 → setup-sbt → `sbt compile|test`** [1]。`actions/setup-java` 把 `cache: sbt` 做成一等能力，缓存键基于 `*.sbt` / `project/**` [6]。

上游 `chipsalliance/chisel` 自身 CI 同时存在 **sbt** 与 **Mill** 路径；对**消费方**（只编译生成的 `Module.scala`）仍以 sbt/`build.sbt` 模板最常见 [7][3]。Scala CLI 适合小片段，也会拉 JDK≥17，但对已有 `scripts/chisel-fr28-compile.sh`（sbt 取向）改动更小 [2]。

**对决策：** 不要发明新栈；对齐 sbt Book + Temurin 17。

## Integration & interoperability

Chisel 7.x 是 **Scala 库 + scalac 插件**：`build.sbt` 必须同时声明同版本 `org.chipsalliance` %% `chisel` 与 `% chisel-plugin`（`CrossVersion.full`）[3]。Scala/Java 兼容表显示 7.x 可到很高 Java，但文档明确 **推荐 LTS ≥17**；Scala CLI **要求** 17+ [2]。

与 Bitloom 钉死 **Chisel 7.14.0 ↔ firtool 1.155.0** 的关系：

| 步骤 | 工具 | 是否必须进「真编译」门 |
|------|------|------------------------|
| FrozenHir → `.scala` | Rust `emit_chisel` | 已有；CI 已测谓词 |
| `scalac`/Chisel 插件接受生成源 | JDK17 + sbt | **本决策新增** |
| 降到 Verilog 的 firtool | CIRCT/firtool | **不是**本门的必要条件（那是另一条后端路径）|

实现时：`build.sbt` 内 Chisel 版本字符串必须与产品钉死版本一致；`scalaVersion` 必须是发布该 `chisel-plugin` 所用的 2.13.x（`CrossVersion.full`）[3]。

## Architecture patterns in practice

**推荐拓扑：并行 required job**（Pattern A）

```
┌─ job: test (existing just test / cargo) ──┐
│  Rust 谓词 chisel_fr28_*                   │  → both required
└─ job: fr28-chisel-jvm ────────────────────┘
   JDK17 + sbt compile 黄金 .scala
```

理由：sbt 文档主张不重叠的工作拆 job/matrix [8]；硬门槛 job **禁止** `continue-on-error: true`（否则 PR 检查语义混乱）[4]。

**Pattern B（同 job 串行）：** `cargo test` 后再装 JDK/sbt——实现简单，但拉长关键路径墙钟，且把 JVM 冷启动绑在 Rust 超时上。仅适合极小仓库。

**`just test`：** 默认流水线若指「开发者本机」与「CI」两套：

- **CI：** 强制 JVM job（本决策）。
- **本机 `just test`：** 可保持 Rust-only，另加 `just chisel-fr28-jvm`；或用 `BITLOOM_REQUIRE_CHISEL_JVM=1` 在 `just test` 末尾强制。强制本机装 JDK 会抬高贡献门槛——建议 **CI 硬、本机可选默认 / 文档要求维护者跑一次**。

## Implementation reality

**脚本合同变更（相对今日 skip=0）：**

1. `scripts/chisel-fr28-compile.sh`：缺 Java≥17 或 sbt → **exit 1**（或独立 `chisel-fr28-compile-required.sh`）。
2. 工作流步骤：checkout →（可选）Rust emit 夹具 `.scala` → setup-java Temurin 17 `cache: sbt` → setup-sbt → 调用脚本 → **失败即红** [1]。
3. `JAVA_OPTS`/`JVM_OPTS` 可参考 sbt 文档矩阵示例（2G 堆）以防插件 OOM [8]。
4. 缓存：保持 **ubuntu-latest + Temurin 17** 稳定，减少 Zinc/JDK 漂移导致的全量重编 [9]。
5. **墙钟：** 无公开 FR28 单文件基准 [5]；故事验收应记录冷/热两次 GHA 时间并设 `timeout-minutes`（建议初值 15–20，实测后收紧）。

**常见翻车：**

- `chisel-plugin` 与 `scalaVersion` 不匹配（`CrossVersion.full`）[3]
- 生成 Scala 依赖隐式 clock/reset 约定与 emit 不一致（谓词测不到的 API 漂移）——正是本门要抓的
- 把 firtool 缺失当成 compile 失败——应把 firtool 步骤排除出本 job

## Cross-dimension insights

官方 CI 配方成熟 + Chisel 明确推荐 JDK17，使「强制 JVM」在工程上是**常规加 job**，不是研究前沿。真正的产品决策是 **成本 vs 合同字面**：谓词已覆盖结构合同；JVM 门抓住 **scalac/插件真实接受性**。并行 job 把成本从关键路径上拆开，是唯一同时满足「硬门槛」与「不拖死 Rust CI」的交叉结论。

## Contrary evidence

- Chisel 文档称 Java 8+ 仍可工作 [2]——不否定用 17 做 CI 钉死。
- 上游正更多用 Mill [7]——不影响消费方用最小 sbt 工程编译单文件。
- 远程/Zinc 缓存有失效案例 [9]——说明要钉 OS/JDK，不是说不要缓存。

## Recommendations

1. **Adopt Pattern A：** GHA required job `fr28-chisel-jvm` = Temurin 17 + `cache: sbt` + `setup-sbt` + 非 skip 脚本编译黄金夹具（confidence: high，基于 [1][4]）。→ 喂 architecture / CI story。
2. **改脚本语义：** 缺工具链失败；可选保留 `BITLOOM_CHISEL_JVM_SKIP=1` 仅给文档化逃生舱，**默认 CI 不设**（confidence: high，决策约束）。
3. **钉死 `build.sbt` 模板** 与产品 Chisel 7.14.0 / 匹配 scalaVersion（confidence: high，[3]）。
4. **`just test`：** 默认不加 JDK；`just ci` 或 GHA 调 `just chisel-fr28-jvm`（confidence: medium，贡献者体验权衡）。
5. **实现故事测墙钟** 后再写 timeout / 是否需要 coursier 预热（confidence: low on minutes，[5]）。

## Open questions

1. Bitloom 已钉死的 Chisel **7.14.0** 对应的确切 `scalaVersion`（查 Maven Central 该版本的 plugin artifact）——实现时一次确认。
2. 冷启动 GHA 分钟数（必须实测）。
3. 是否要把「elaborated Chisel → firtool」也并入同一 job（研究建议：**否**，分门禁）。

## Source appendix

| [n] | Supports | Publisher | Pub date | Accessed | Confidence |
|-----|----------|-----------|----------|----------|------------|
| [1] | Minimal GHA = Temurin 17 + cache sbt + setup-sbt | [sbt Book](https://www.scala-sbt.org/2.x/docs/en/recipes/github-actions-setup.html) | live docs | 2026-08-21 | high |
| [2] | Recommend Java 17+; Scala CLI needs 17+ | [chisel-lang installation](https://www.chisel-lang.org/docs/installation) | live docs | 2026-08-21 | high |
| [3] | chisel + chisel-plugin CrossVersion.full | [chipsalliance/chisel README](https://github.com/chipsalliance/chisel) | live README | 2026-08-21 | high |
| [4] | Avoid continue-on-error for hard gates | [Ken Muse GHA errors](https://www.kenmuse.com/blog/how-to-handle-step-and-job-errors-in-github-actions/) | blog | 2026-08-21 | medium |
| [5] | No public FR28 single-file wall-clock | (absence) | — | 2026-08-21 | — |
| [6] | setup-java cache:sbt | [actions/setup-java](https://github.com/actions/setup-java) | live README | 2026-08-21 | high |
| [7] | Chisel upstream sbt+Mill CI | [chisel test.yml snapshot](https://github.com/chipsalliance/chisel/blob/da986875765f02ed96547952da4b01d1b3f4eab2/.github/workflows/test.yml) | workflow | 2026-08-21 | medium |
| [8] | Split jobs / matrix; JVM heap env examples | [sbt GHA reference](https://www.scala-sbt.org/release/docs/GitHub-Actions-with-sbt.html) | live docs | 2026-08-21 | high |
| [9] | Cache invalidation across OS/JDK | [sbt#7005](https://github.com/sbt/sbt/issues/7005) | issue | 2026-08-21 | medium |

## Staleness map

| Claim class | Window | Recheck by |
|-------------|--------|------------|
| versions & compatibility (JDK/sbt/Chisel pins) | ≤ 1 mo | 2026-09-21 |
| ecosystem signals (setup-java / setup-sbt APIs) | ≤ 6 mo | 2027-02-21 |
| landscape (sbt vs Mill for consumers) | ≤ 12 mo | 2027-08-21 |
| patterns (job topology) | ≤ 2 yr | 2028-08-21 |

**Earliest re-check:** 2026-09-21（版本钉死与 setup-java v5 API）。
