# Code Review: Story 24.2 钉死后端的默认 HLS 路径（FR35）

**Verdict:** accept

## Findings

1. **移除「永久 unsupported」门控** — 不再依赖 `RHDL_HLS_ENABLE`；缺后端失败可读，符合 AD-25/FR35。
2. **版本钉死** — `HLS_BACKEND_VERSION = 2024.10` 与 NFR14 一致；错误消息含 AppImage URL。
3. **无树内调度** — 仅 emit C + 外挂进程；ATDD 断言 stub 注释。
4. **`--emit-only`** — 明确不计入 RTL 成功；避免无后端环境无法检查夹具。

## Disposition

- accept；CI/烟测夹具留给 24.3。
