# FR34 — simulation coverage

After `tick`, `Sim::coverage_report()` emits a stable, parseable text. **Story 47.3 / FR105** bumped the header to **v2** while retaining toggle lines:

```
# bitloom-sim coverage v2
hit data_out
hit count
miss data_in
miss clk
```

A **hit** is a named port/reg whose sampled value toggled at least once; a **miss** was sampled but never toggled. Fixtures must show at least one of each (`parse_report`).

**FR34 toggle alone ≠ FR105.** Branch / condition extension (`branch_hit` / `branch_miss`) and Epic 47 closeout live in [`fr105-sim-coverage-ext.md`](fr105-sim-coverage-ext.md).
