# Automation Summary — Story 37.3 / FR88 HLS Path B

**Mode:** Expand after implementation (docs honesty + deferred close)  
**Disposition:** Path B (explicit stub default; no nightly real Bambu)

## Guardrails added

| Area | Test binary | Notes |
|------|-------------|-------|
| Path B + stub≠质量 + 真机入口 | `fr88_hls_stub_path_b_honesty` | fr35 |
| 树内调度非目标 | same | AD-25 |
| deferred 本阶段选 B | same | item-54 closed |
| NFR14 Epic 37 关闭勾选 | same | all `[x]` + closed |
| README FR88 Path B | same | cross-link |
| CI 无 continue-on-error | same | hls-smoke slice |

## Risk residual

| Risk | Severity | Mitigation |
|------|----------|------------|
| Future silent nightly with continue-on-error | Med | ATDD + NFR14 forbid; reopen needs new story |
| Stub misread as HLS quality | Med | fr35 / README / deferred honesty locked |
| Tree-in scheduler creep | Low | AD-25 / FR93 / fr35 non-goal assert |

## Command

```bash
cargo test -p bitloom --test fr88_hls_stub_path_b_honesty
```

## Out of scope

- Path A nightly AppImage CI
- Epic 38
