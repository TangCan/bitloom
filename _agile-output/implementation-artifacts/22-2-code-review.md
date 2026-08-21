# Code Review: Story 22.2 树内 FIFO + UART + 黑盒（FR37）

**Reviewer:** adversarial (agent)  
**Disposition:** accept

## Findings

1. **UART 非全栈：** 文档化为 holding-register stub；符合「非空壳但可最小」。非缺陷。
2. **黑盒 body 为空：** 测试断言 `body.is_empty()`；vendor `.v` 旁路保留。通过。
3. **设计依赖：** IP 在 `bitloom-prelude`；`ip_box` 仅依赖 prelude。通过。

## AC Trace

| AC | Result |
| --- | --- |
| FIFO/UART/黑盒 elaborate→emit→tick | pass |
| 设计侧仅 bitloom-prelude | pass |
| 各至少一 smoke | pass |
