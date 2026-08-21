# Code Review — 25.3

**Outcome:** Approve

| Check | Result |
|-------|--------|
| Parallel required job | yes |
| Java 17 + sbt cache + setup-sbt | yes |
| Required script, no SKIP= | yes |
| no continue-on-error | yes |
| timeout 20 | yes |
| firtool not in job | yes |

**Note:** First CI cold compile wall-clock unknown; timeout 20m is intentional headroom.
