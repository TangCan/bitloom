---
route: oneshot
status: done
story_key: ''
---

# 意图

将本仓库已有的两轮功能模拟覆盖率热点优化适配到 `bitloom-sim`：预先建立信号覆盖采样槽，避免每个 tick 临时创建名称列表；预先建立 Mux 分支标签，避免每次求值格式化标签。保持公开 Coverage API、报告内容/排序、首采样与未执行 Mux 语义不变。优化范围为 FrozenHir Sim tick 路径。

# 验收标准

- 每 tick 覆盖采样不再分配信号名称列表，基于稳定槽位记录已采样信号。
- Mux 标签仅在 Sim 构造阶段生成；执行路径以预登记槽位记录命中。
- 已有公开接口与覆盖报告语义保持兼容。
- 新增回归测试覆盖信号采样、Mux 分支以及报告产物等价行为。
- 运行 bitloom-sim 与仓库规定的验证。

## Implementation Notes

- 上游没有此前 fork 的 `signal_values`；采用独立 coverage value vector，并在端口/寄存器写入处同步，tick 采样不再查找名称。
- 此上游 Compiled 与 Interpreter 共用 `Sim::eval`，Mux 以构造期缓存标签表接入，AST 不变。
- 信号槽记录先前值，并只在首次采样和首次命中时插入有序集合，避免稳态 tick 重复克隆名称。
- 两种 TickEngine 均使用同一覆盖槽同步路径；增加相应覆盖回归测试。

## Review Triage Log

- `patch`（中）：最初缺少 Compiled 覆盖路径回归；将槽更新测试扩展到 Interpreter 与 Compiled，专项测试通过。
- `patch`（低）：最初缺少 Sim 层未执行 Mux 的覆盖语义测试；新增构造后未 tick 的测试，确认仍为空且不报告 branch miss。
- `false`：named `Coverage::sample` 与 indexed slot API 的历史状态可能不同；slot 方法为 `pub(crate)`，Sim 暴露的是只读 `coverage()`，外部用户不能混用；公开 named API 保留独立历史用于兼容。
- `false`：保留 `prev` map 属重复状态；该 map 仍服务公开的 `Coverage::sample` 路径，Sim 内部改用 slots，不重复记账。
