# Epic 60 Context: SymbiYosys/SMT 形式路径

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

验证工程师获得 F1-(ii) 级 **SymbiYosys（`sby`）绑定**的可复现产品路径（原 FR112 分支 A / **FR119**），超出 FR100 F1-(i) 与 FR112 分支 B。独立 SMT 求解器产品入口（绑定类 B）**未选**为对等完成面（SMT 仅可作为 sby 后端）。关闭后可按 FR123 宣称 SBY 加深面；Phase 12 FR100 与 Phase 13 FR112 分支 B 关闭证据仍有效。

## Stories

- Story 60.1: Epic 60 NFR14 风险记录
- Story 60.2: SBY/SMT 路径实现与验收（FR119）
- Story 60.3: FR119 收口与文档指针

## Requirements & Constraints

- **FR119：** 须交付 F1-(ii) 级 **选定 (A) SymbiYosys/`sby`** 可复现产品/文档化路径；禁止仅以 FR100 F1-(i)、FR112 分支 B MemRead≡tick、FR92 记分板 alone、docs-only、FR85 Verilator alone、或冒充 FR107 SystemC AT 关闭。
- **NFR14 / NFR49：** 无 Epic 60 专用风险记录则 60.2–60.3 不得 ready。
- **NFR48：** 不得改写 FR100 / FR112 分支 B「已关闭」为失败；既有形式等价与 MemRead≡tick 路径须仍可用。
- **NFR50：** 若触及 formal 路径相关 AD/脊柱说明，须先修订脊柱再标 story ready；遵守 `deferred-work.md` 诚实边界。
- **NFR51：** 未选分支 C（更多 IP 手写 FL）与独立 SMT (B) 产品支保持 deferred/未选，不得 silent 宣称。
- 公开品牌 **Bitloom**；设计 crate 只依赖 `bitloom-prelude`。
- 硬依赖 Epic 57（FR116）已关闭；≠ FR107 SystemC AT；≠ 改写 FR112 分支 B 关闭证据。

## Technical Decisions

- **选定绑定 (A) SymbiYosys（`sby`）**；写清工具检测/版本记录义务、assume/assert、夹具与 CI/本地可复现（默认 `just test` 不硬依赖 `sby`）。
- **相对 FR85：** fork 新入口（建议 `formal-sby-check`），不把 FR85 Verilator/`sby` 钩子 alone 写成 FR119。
- Formal/SBY **实现形状由本 epic + NFR14 钉死**；脊柱不预钉 crate 切分。
- 产品/文档化一等路径 + 至少一夹具证明（pass 与可读 fail）；工具缺失时失败可读，不得 silent 成功。
- Gate：Story 60.1 NFR14 未完成前，60.2–60.3 不得标 ready。

## UX & Interaction Patterns

- 无独立 UX 合同；验收以产品文档/夹具 + ATDD（或文档化手动清单）为准。

## Cross-Story Dependencies

- **60.1** 开门禁；钉死工具链与义务后 **60.2** 实现 + 夹具/ATDD；**60.3** 文档/deferred 收口（`docs/fr119-*` 或扩展 fr112/fr100）并勾选 Epic 60 / FR119。
- 不依赖 Epic 58–59 / 61–63；可与之并行但各需自有 NFR14。
