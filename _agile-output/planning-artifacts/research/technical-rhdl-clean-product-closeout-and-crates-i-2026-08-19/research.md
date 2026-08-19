---
title: 'technical research: rhdl clean product closeout and crates.io rhdl-rs release'
type: 'technical'
topic: 'rhdl clean product closeout and crates.io rhdl-rs release'
decision: 'How to achieve maturity-debt-free product closeout and publish rhdl-rs to crates.io'
source: 'native-run'
status: complete
preset: 'standard'
validation: 'normal'
created: '2026-08-19'
updated: '2026-08-19'
---

# technical research: 干净产品结项与 crates.io `rhdl-rs` 发布

**Decision this research serves:** 如何实现 (A) 无成熟度债务的干净产品结项，以及 (B) 产品发布 / crates.io `rhdl-rs`？

## Executive summary

证据表明：**sprint backlog 清空 ≠ 产品结项，也 ≠ crates.io 就绪。**[1][2]  
「干净结项」在 2025–2026 Rust 实践里，是一套**可验证的发布合同**：声明并 CI 验证的 MSRV、`0.x`/`1.0` 的 SemVer 政策、docs.rs 可构建文档、changelog+git tag、安全联系路径、诚实的 deferred/non-goals 表面。[3][4][5][6]  
「发布 `rhdl-rs`」则是**永久登记名** + 元数据 + `cargo publish` 验证；名字 FCFS，yank 不能抹掉代码；不要试图占用已被占用的近似名（如 `rhdl`），应用独立包名 `rhdl-rs`，二进制名可另设。[1][7][8]

**建议路径（高置信）：**  
1) 先关**成熟度门禁**（仍可 `0.x`）；2) `cargo publish --dry-run` 后**手动首次发布** `rhdl-rs`；3) 配置 **Trusted Publishing (OIDC)**，后续用 release-plz（或 cargo-release）+ 可选 cargo-dist；4) 启用 README/changelog 诚实声明 deferred 能力。[9][10][11]

**最大 caveat：** crates.io 政策页偶发抓取失败；发布前应人工复核 [crates.io/policies](https://crates.io/policies)。命名商标风险属法律层，不由 FCFS 技术规则解决。[12]

## Landscape & maturity

crates.io 对「第一次能传上去」的定义是：**永久版本 + 必填元数据 + 打包验证成功**，不是功能清单清空。[1]  
成熟「产品结项」被 Cargo 写成**自动化发布过程**（changelog + 打在发布提交上的 tag），与 backlog 正交。[2]  
`>=1.0.0` 是 **API 稳定门**（C-STABLE：公开依赖也须稳定），不是 sprint 结束条件。[3]  
MSRV 成熟度信号：写 `rust-version`、写支持政策、CI 验证；Cargo 假定改 `rust-version` 为 *minor* 不兼容。[4]  
`cargo-semver-checks` 是当前代 pre-publish 工具；docs.rs 是发布后文档成熟度信号；Crater 是编译器团队工具，不是普通 crate 作者门禁。[5][13][14]  
安全成熟度是维护过程（SECURITY.md、RustSec），不是 crates.io 内建门。[6]  
**Trusted Publishing（2025-07 起）**使无长期 token 的 CI 发布成为当前代能力；**首次仍需 API token**。[9]

**结论：** 无权威来源把「sprint 空」定义为发布就绪（查无）；[gaps] 干净结项 = 发布合同 + 验证面，而非 backlog 空。

## Architecture patterns（crates.io 发布）

| 架构 | crates.io | CLI 二进制 | 典型失败 |
|------|-----------|------------|----------|
| 裸 GHA `cargo publish` | 有 | 无 | 拓扑/索引延迟；无 dry-run；token 泄漏 |
| cargo-release (+ dist) | 有 | tag→dist | workspace 标签形状错；只发根 crate |
| release-plz (+ dist) | 有 | tag→dist | conventional-commit 噪音；token 权限导致 tag 工作流不触发 |
| 仅 cargo-dist | 无 | 有 | 忘记发 registry |

通行拆分：**registry 版本**（release-plz 或 cargo-release）与 **安装器用二进制**（cargo-dist）。[10][11]  
`[package].name = "rhdl-rs"` 是登记身份；`[[bin]] name` 可不同，但不构成 crates.io 名预留。[8]  
未成熟工具链应停在 **0.x**（Cargo 把最左非零分量当不兼容轴）；到 1.0 才谈完整 major 纪律。[15]  
改名只能新名再发；删除极窄（RFC 3660）；团队不再调解所有权转让（RFC 3646）；禁止长期占坑（RFC 3463）。[7][16][17]

## Implementation reality & ecosystem

**发布前清单（蒸馏）：**[1][18][19]
1. 确认 `rhdl-rs` 可用；禁止暗示/占用冲突名。  
2. `Cargo.toml`：description、license、repository、readme、keywords≤5、categories、authors、`rust-version`。  
3. `cargo package --list` / 体积；`cargo publish --dry-run`。  
4. `cargo test` / `cargo doc`；必要时 `[package.metadata.docs.rs]`。  
5. owners 计划；首次手动 publish → Trusted Publishing（可再开 trustpub-only）。[9][20]  
6. CHANGELOG + annotated tag；README 写清 status / non-goals / deferred。

**6–12 月运维负担：** 版本不可变 → SemVer 抖动；MSRV 抬升要再发版；RustSec/`cargo audit`；token 卫生或 trustpub-only；docs.rs 失败要重建或修后重发；yank 仅异常。[12][21]  
**诚实 deferred：** 用 `0.x`、README 警告/`[!WARNING]`、功能 feature 门控、changelog 与 roadmap 分离、C-RELNOTES 标破坏；勿用 categories/描述暗示未交付 API。[19][22]

2026-01  hardening：GitLab trusted publishing；可选 **Trusted Publishing Only**；阻止 `pull_request_target` / `workflow_run` 用于 trustpub。[20]

## Cross-dimension insights

- **A 与 B 顺序固定：** 先有「诚实成熟度合同」（仍可 0.x），再首次 publish；否则要么假完成（声称 1.0/功能齐），要么把永久登记建立在未声明债务上。[1][3][15]  
- **自动化是结项杠杆，不是发布前提：** Trusted Publishing + release-plz 解决重复发布卫生；首次仍手发。[9][10]  
- **命名政策与产品身份同向：** 技术上 `rhdl-rs` 与占用名并存合法；产品文档必须持续划清边界（政策禁止误导性占坑，但不替你做商标判断）。[7][12]

## Recommendations

| # | 建议 | 置信 | 下游消费 |
|---|------|------|----------|
| R1 | 定义「干净结项」= 发布合同清单（MSRV CI、SemVer/0.x 政策、SECURITY.md、docs.rs 绿、changelog+tag、README deferred），**明确不等于** backlog 空 | high | 下一 PRD / closeout 备注 |
| R2 | 首次仅 publish 需要上 registry 的 crate（至少 `rhdl-rs`）；库 crate 可 `publish = false` 直至 API 可承诺 | high | Cargo workspace 策略 |
| R3 | 版本停在 **0.x** 直至公开表面有意稳定；不要为结项冲 1.0 | high | versioning AD |
| R4 | 流水线：`dry-run` → 手动首发 → Trusted Publishing → release-plz（+ 可选 cargo-dist） | high | CI / ops |
| R5 | 用 README/changelog 诚实写 deferred（LSP、部分 CLI 动词等），避免 docs 描述 vapor API | medium | docs / FR 对齐 |
| R6 | 发布前人工复核 crates.io policies；评估商标风险（非技术门禁） | medium | 合规 |

## Open questions

1. 工作区多 crate 是否同一版本号锁步，还是仅 `rhdl-rs` 对外版本？（需产品决策）  
2. CLI 二进制对外安装名是否用 `cargo-rhdl` / `rhdl`？（本地工具冲突风险）  
3. 是否在首发后立即启用 **Trusted Publishing Only**？  
4. 库 crate（hir/sim/…）首发是否一并上架，还是长期 path-only？

## Source appendix

| n | Supports | Publisher | Pub | Accessed | Conf |
|---|----------|-----------|-----|----------|------|
| [1] | 发布永久/元数据/dry-run | [Cargo Book — Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html) | living | 2026-08-19 | high |
| [2] | 自动化发布=changelog+tag | 同上 | living | 2026-08-19 | high |
| [3] | C-STABLE 1.0 门 | [API Guidelines necessities](https://rust-lang.github.io/api-guidelines/necessities.html) | living | 2026-08-19 | high |
| [4] | rust-version / MSRV | [Cargo — rust-version](https://doc.rust-lang.org/cargo/reference/rust-version.html) | living | 2026-08-19 | high |
| [5] | cargo-semver-checks | [GitHub obi1kenobi/cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) | 0.50.0 ~2026-08-01 | 2026-08-19 | high |
| [6] | RustSec / security process | [crates.io/security](https://crates.io/security) · [rustsec contributing](https://rustsec.org/contributing.html) | living | 2026-08-19 | high |
| [7] | 名 FCFS / 占坑政策 | [RFC 3463](https://rust-lang.github.io/rfcs/3463-crates-io-policy-update.html) · [blog 2023-09-22](https://blog.rust-lang.org/2023/09/22/crates-io-usage-policy-rfc/) | 2023 | 2026-08-19 | high |
| [8] | package vs bin name | [Cargo manifest](https://doc.rust-lang.org/cargo/reference/manifest.html) · [targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) | living | 2026-08-19 | high |
| [9] | Trusted Publishing | [crates.io docs](https://crates.io/docs/trusted-publishing) · [blog 2025-07-11](https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/) | 2025-07 | 2026-08-19 | high |
| [10] | release-plz | [release-plz.dev](https://release-plz.dev/docs) | living | 2026-08-19 | high |
| [11] | cargo-dist + cargo-release | [cargo-dist book](https://axodotdev.github.io/cargo-dist/) | living | 2026-08-19 | high |
| [12] | crates.io policies（复核） | [crates.io/policies](https://crates.io/policies) | living | 2026-08-19 | medium |
| [13] | docs.rs | [docs.rs/about](https://docs.rs/about) | living | 2026-08-19 | high |
| [14] | Crater 非作者门禁 | [rustc-dev-guide crater](https://rustc-dev-guide.rust-lang.org/tests/crater.html) | living | 2026-08-19 | high |
| [15] | 0.x SemVer | [Cargo semver](https://doc.rust-lang.org/cargo/reference/semver.html) | living | 2026-08-19 | high |
| [16] | 删除限制 | [RFC 3660](https://rust-lang.github.io/rfcs/3660-crates-io-crate-deletions.html) | 2024 | 2026-08-19 | high |
| [17] | 无调解转让 | [RFC 3646](https://rust-lang.github.io/rfcs/3646-remove-crate-transfer-mediation-policy.html) | RFC | 2026-08-19 | high |
| [18] | Book 发布章 | [Rust Book ch14](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html) | living | 2026-08-19 | high |
| [19] | C-METADATA / C-RELNOTES | [API Guidelines documentation](https://rust-lang.github.io/api-guidelines/documentation.html) | living | 2026-08-19 | high |
| [20] | trustpub hardening 2026-01 | [blog 2026-01-21](https://blog.rust-lang.org/2026/01/21/crates-io-development-update/) | 2026-01-21 | 2026-08-19 | high |
| [21] | yank | [cargo yank](https://doc.rust-lang.org/cargo/commands/cargo-yank.html) | living | 2026-08-19 | high |
| [22] | README alerts | [blog 2025-07-11](https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/) | 2025-07 | 2026-08-19 | medium |

## Staleness map

| Class | Window | Example claims | Re-check by |
|-------|--------|----------------|-------------|
| version/compatibility | ≤1 mo | Trusted Publishing 功能面、cargo-semver-checks 版本 | 2026-09-19 |
| ecosystem signals | ≤6 mo | release-plz/dist 工作流惯例 | 2027-02-19 |
| landscape | ≤12 mo | 「sprint≠发布就绪」格局 | 2027-08-19 |
| patterns | ≤2 yr | 0.x vs 1.0、C-STABLE | 2028-08-19 |
| policy pages | ≤3 mo | crates.io/policies 正文 | 2026-11-19 |

**Earliest re-check:** 2026-09-19（Trusted Publishing / 工具版本）。用 Refresh 重跑。

## Digests

- `digests/landscape-maturity-r1-1.md` — [Landscape](e468a83c-5a63-45e1-af34-1622ebe483cc)  
- `digests/architecture-patterns-r1-1.md` — [Release patterns](150b13a2-20dd-4fa1-a72d-8d7f77fafd9e)  
- `digests/implementation-ecosystem-r1-1.md` — [Implement+ecosystem](ce50587b-e951-4024-8dec-1e552319b65d)  
- `digests/trusted-publishing-r1-1.md` — lead follow-up
