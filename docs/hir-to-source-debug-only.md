# Debug-only: HIR → RHDL source

**NFR10 / AD-3：** 从 `FrozenHir` 再生成 RHDL 源码（若实现）**仅用于调试**。

- **不是**产品互转格式。
- **产品互转契约**仍是 FrozenHir ↔ `FIRRTL version 6.0.0` 文本。
- 发行版测试**不得**宣称 RHDL 源码往返稳定。

See also: `README.md` identity section; architecture spine AD-3 / AD-26.
