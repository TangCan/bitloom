# Digest: implementation-r1-1
## Findings
- claim: First cold sbt resolve+compile of Chisel deps dominates CI time; dependency caching via setup-java `cache: sbt` is the primary lever for warm runs.
  source: https://github.com/actions/setup-java
  publisher: GitHub
  pub_date: README
  accessed: 2026-08-21
  confidence: medium
  class: performance
- claim: Zinc/source hashing and JDK differences cause cache invalidation / full recompile; keep OS and JDK major stable across jobs that share caches.
  source: https://github.com/sbt/sbt/issues/7005
  publisher: sbt issue tracker
  pub_date: issue discussion
  accessed: 2026-08-21
  confidence: medium
  class: patterns
- claim: Anecdote: large monorepo compile+test ~25m → ~10m with sbt 2 remote cache — shows compile cost is real but remediable; not directly transferable to tiny FR28 fixture.
  source: https://medium.com/@idanbenzvi/one-line-in-sbt-2-how-remote-caching-cut-our-ci-in-half-90fcdb5a503d
  publisher: Medium / Idan Ben Zvi
  pub_date: article
  accessed: 2026-08-21
  confidence: low
  class: performance
## Leads
- Require non-zero exit on missing Java/sbt (invert current skip script)
- Cache key should include Chisel version pin file
## Not found
- Measured GHA minutes for Chisel 7.14 single-file Module compile (must measure in implement story)
