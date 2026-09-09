# FR80 — Nested Bundle + derive（AD-20）

公开品牌 **Bitloom**；设计 crate 只依赖 **`bitloom-prelude`**。

## 相对 Epic 19 / FR51 的诚实度（NFR37）

| | Epic 19 / FR51 最小合同 | FR80 / Epic 32（本故事） |
| --- | --- | --- |
| 完成话术 | ground-leaf flatten + 宽/向 emit 前失败；nested **OUT OF SCOPE** | **至少一层**嵌套 Bundle → emit → tick；`#[derive(Bundle)]` |
| 深度关闭 | sprint 可 `done`（最小） | **不得**用历史 done 或仅删 OUT OF SCOPE 注释冒充 |

夹具：`examples/bundle_vec_skel`。  
Epic 32 收口跟练：[`docs/tutorials/nested-bundle.md`](tutorials/nested-bundle.md)。

## 合同内

- **一层嵌套：** 子 Bundle 作父成员；展平叶名 `{field}_{nested}_{leaf}`。
- **derive：** `#[derive(Bundle)]` 经 prelude 重导出（AD-6）。
- **宽/向：** 嵌套叶不匹配仍 emit 前失败（`rhdl::E0131` / `rhdl::E0112`）。
- **叶名碰撞：** `rhdl::E0152`。

## 合同外 / 限制

- **≥2 层嵌套：** 本 epic **默认非目标**。
- **`HwVec<Bundle,_>`：** 仍 OUT OF SCOPE。
- derive 拒绝：enum、tuple/unit、结构体泛型、`HwVec`/`Input`/`Output` 字段 → `rhdl::E0180`。

## 最小用法

```ignore
use bitloom_prelude::{Bool, Bundle, UInt};

#[derive(Bundle)]
struct Stream {
    data: UInt<8>,
    valid: Bool,
}

#[derive(Bundle)]
struct Packet {
    ready: Bool,
    stream: Stream,
}
```

## 验证配方

```bash
cargo test -p bitloom --test fr80_nested_bundle
cargo test -p bundle_vec_skel
```

贡献者全量：`just test`。
