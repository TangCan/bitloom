# Code Review: Story 33.4 ATDD + FR71 回归（FR81 Path A close）

**Verdict:** Approve

**Scope:** Mem Path A JVM contract fixture + FR71 regression ATDD/docs + NFR14 Epic 33 close; sprint keys `33-4-…` / `epic-33`

## Findings

1. **无阻塞缺陷。** Path A emit ATDD 仍绿；新增 `fr81_mem_chisel_atdd_fr71` 锁定 FR71 counter 黄金/`just chisel-fr28-jvm`/GHA 合同、Mem 夹具、文档边界、NFR14 Epic 33 全勾。
2. **FR71 未削弱。** `fr28_golden_counter.scala` 仍无 Mem；GHA `fr28-chisel-jvm` 仍只编译 counter；Mem 夹具经可选 `just chisel-fr81-mem-jvm`，不替换 required 门禁。脚本 ATDD（`test-just-chisel-fr28-jvm.sh` / `test-gha-fr28-chisel-jvm.sh`）PASS。
3. **本机真 JVM：** 贡献者机为 Java 11 → `just chisel-fr28-jvm` 按合同非零失败（required）；真编译由 CI Temurin 17 覆盖。不挡合入。
4. **testarch-automate：** 无需额外套件——close-out ATDD 已覆盖 AC。

## AC Trace

| AC | Result |
| ---- | ------ |
| ATDD 覆盖决策路径 + FR71 等价回归（FR81） | pass（emit ATDD + `fr81_mem_chisel_atdd_fr71` + script ATDD） |
| FR71 / GHA `fr28-chisel-jvm` 合同仍绿 | pass（counter 未改；GHA/just 合同 ATDD） |
| 用户/维护者文档更新 Mem↔Chisel 边界 | pass（`fr28-chisel-compilable.md` / language-surface / README） |
| NFR14 Epic 33 关闭条件勾选 | pass（6/6 `[x]` + 证据行） |

## Decision

**Accept** — 标 `33-4-…: done`；`epic-33: done`；提交由父代理执行。
