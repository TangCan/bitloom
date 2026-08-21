# Digest: integration-r1-1
## Findings
- claim: Chisel docs: install JDK; Chisel works on Java 8+, recommend LTS 17+; Scala CLI requires Java 17+.
  source: https://www.chisel-lang.org/docs/installation
  publisher: chisel-lang.org
  pub_date: current docs
  accessed: 2026-08-21
  confidence: high
  class: version/compatibility
- claim: Chisel README example build.sbt needs both `chisel` library and `chisel-plugin` with matching version + CrossVersion.full; scalaVersion e.g. 2.13.16 for 7.2.0 examples.
  source: https://github.com/chipsalliance/chisel
  publisher: chipsalliance
  pub_date: README current
  accessed: 2026-08-21
  confidence: high
  class: version/compatibility
- claim: Compatibility table lists max Java per Chisel/Scala line (7.x up to Java 26 with Scala 2.13.18) — pin LTS 17 to stay inside supported envelope.
  source: https://www.chisel-lang.org/docs/installation
  publisher: chisel-lang.org
  pub_date: current docs
  accessed: 2026-08-21
  confidence: high
  class: version/compatibility
## Leads
- Align Bitloom pin Chisel 7.14.0 with exact scalaVersion that publishes that plugin
- firtool/CIRCT install is separate from scalac compile of Module.scala
## Not found
- Official Bitloom-specific docs (N/A — out of research firewall)
