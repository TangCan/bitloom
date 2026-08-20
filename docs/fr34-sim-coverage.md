# FR34 — simulation coverage

After `tick`, `Sim::coverage_report()` emits a stable, parseable text:

```
# bitloom-sim coverage v1
hit data_out
hit count
miss data_in
miss clk
```

A **hit** is a named port/reg whose sampled value toggled at least once; a **miss** was sampled but never toggled. Fixtures must show at least one of each (`parse_report`).
