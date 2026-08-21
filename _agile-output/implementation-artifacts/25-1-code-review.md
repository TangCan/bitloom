# Code Review — 25.1 必失败 FR28 JVM 编译脚本

**Date:** 2026-08-21  
**Outcome:** Approve with notes  
**Baseline:** story implementation in working tree

## Findings

| Severity | Disposition | Finding | Action |
|----------|-------------|---------|--------|
| low | patch | ATDD 曾对 `VAR=1 cmd` 经 `"$@"` 展开误成命令名 → 127 | 已改为 `env VAR=1` |
| low | accept | 本机无 Java≥17+sbt，黄金真编译未在本机绿 | 文档/ATDD SKIP；25.3 CI 安装工具链后强制 |
| info | accept | optional 路径仍可 skip=0 | 符合 AC；CI 用 required |

## AC Trace

1. 缺工具链非零 — ATDD `no-java` / `java-too-old` PASS  
2. build.sbt pin 7.14.0 + plugin — 脚本内模板  
3. SKIP 逃生舱 — ATDD + docs  
4. 黄金夹具 — `fr28_golden_counter.scala`（真编译 deferred to CI）  
5. 未改 GHA / just test — 确认  

## Verdict

**Approve** — ready for automate + regression + commit.
