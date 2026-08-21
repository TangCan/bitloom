# Digest: architecture-r1-1
## Findings
- claim: sbt docs recommend splitting non-overlapping CI work into different jobs or matrix jobtypes rather than one mega-job.
  source: https://www.scala-sbt.org/release/docs/GitHub-Actions-with-sbt.html
  publisher: sbt Reference Manual
  pub_date: current
  accessed: 2026-08-21
  confidence: high
  class: patterns
- claim: Using `continue-on-error: true` on a job makes the job look failed in UI while needs.result may still be success — unsuitable for "required hard gate"; prefer required job without continue-on-error.
  source: https://www.kenmuse.com/blog/how-to-handle-step-and-job-errors-in-github-actions/
  publisher: Ken Muse
  pub_date: blog
  accessed: 2026-08-21
  confidence: medium
  class: patterns
- claim: Matrix with fail-fast: false lets other variants finish; use for optional OS matrices, not for soft-failing the required Chisel compile.
  source: https://www.scala-sbt.org/2.x/docs/en/recipes/github-actions-setup.html
  publisher: sbt Book
  pub_date: current
  accessed: 2026-08-21
  confidence: high
  class: patterns
## Leads
- Pattern A: parallel job `fr28-chisel-jvm` required + rust `test` job
- Pattern B: same job after rust tests (serializes wall clock)
## Not found
- Industry standard specifically named "predicate then JVM" for HDL emitters
