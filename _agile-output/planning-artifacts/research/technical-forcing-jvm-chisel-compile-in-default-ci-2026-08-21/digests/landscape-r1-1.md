# Digest: landscape-r1-1
## Findings
- claim: Official sbt Book documents minimal GHA as Temurin Java 17 + `cache: sbt` + `sbt/setup-sbt@v1` then `sbt test`.
  source: https://www.scala-sbt.org/2.x/docs/en/recipes/github-actions-setup.html
  publisher: Scala Center / sbt docs
  pub_date: 2025-ish (live docs)
  accessed: 2026-08-21
  confidence: high
  class: version/compatibility
- claim: actions/setup-java supports `cache: sbt` keyed on `*.sbt` / project files; Temurin distribution is first-class.
  source: https://github.com/actions/setup-java
  publisher: GitHub
  pub_date: current README
  accessed: 2026-08-21
  confidence: high
  class: version/compatibility
- claim: chipsalliance/chisel CI historically used setup-java + `cache: sbt` + `sbt ++$scala test`; newer paths also use Mill; user projects still commonly sbt.
  source: https://github.com/chipsalliance/chisel/blob/da986875765f02ed96547952da4b01d1b3f4eab2/.github/workflows/test.yml
  publisher: chipsalliance
  pub_date: workflow snapshot
  accessed: 2026-08-21
  confidence: medium
  class: landscape
## Leads
- Pin scalaVersion carefully with chisel-plugin CrossVersion.full
- Optional scala-cli path for small snippets
## Not found
- Public wall-clock benchmarks specifically for "compile one generated Module.scala with Chisel 7.14" on GHA
