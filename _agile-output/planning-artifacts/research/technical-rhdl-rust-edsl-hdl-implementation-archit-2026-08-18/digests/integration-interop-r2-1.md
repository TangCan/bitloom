# Digest: integration-interop r2-1

## Findings

**Chisel v7.14.0 names firtool 1.155.0 as the associated version**, including implicit-truncation warnings in that firtool and a CIRCT bump series ending at firtool-1.155.0 (PR #5460).
**source** https://github.com/chipsalliance/chisel/releases/tag/v7.14.0
**publisher** chipsalliance / jackkoenig
**pub_date** 2026-08-13
**accessed** 2026-08-18
**confidence** high
**class** version/compat

**The v7.14.0 tag pins CIRCT at `"version": "firtool-1.155.0"` in `etc/circt.json`.**
**source** https://raw.githubusercontent.com/chipsalliance/chisel/v7.14.0/etc/circt.json
**publisher** chipsalliance/chisel
**pub_date** 2026-08-13 (tag v7.14.0)
**accessed** 2026-08-18
**confidence** high
**class** version/compat

**The versioning page’s `org.chipsalliance::llvm-firtool % "1.153.0"` snippet is an example of how to override the managed firtool in sbt/Mill, not a 7.14.0 pairing.** It states each Chisel release is tested against a specific firtool in generated tables; a given Chisel is not guaranteed with other firtool versions. Users may emit `.fir` and invoke an installed firtool separately, or set `CHISEL_FIRTOOL_PATH` to a directory containing a `firtool` binary from CIRCT pre-built artifacts.
**source** https://www.chisel-lang.org/docs/appendix/versioning and https://raw.githubusercontent.com/chipsalliance/chisel/v7.14.0/docs/src/appendix/versioning.md
**publisher** chisel-lang.org / chipsalliance/chisel
**pub_date** v7.14.0 docs (2026-08-13); live page fetched 2026-08-18
**accessed** 2026-08-18
**confidence** high
**class** version/compat

**This-run fetch of the live versioning HTML did not emit Chisel 7.x table rows** (tabs/JS; source tables are SBT-generated via `FirtoolVersionsTable.releaseTable`). The rendered page showed the Chisel 5.x table (5.3.0→1.43.0 … 5.0.0→1.40.0). **Do not treat the 1.153.0 example as the 7.14.0 pairing; the pairing is named on the v7.14.0 release and in `etc/circt.json`.**
**source** https://www.chisel-lang.org/docs/appendix/versioning
**publisher** chisel-lang.org
**pub_date** fetched live 2026-08-18
**accessed** 2026-08-18
**confidence** high
**class** failure

**ScalaDoc for chisel 7.14.0 exposes `chisel3.BuildInfo.firtoolVersion: Option[String]` but does not print the Option’s contents.** Installation docs say to query it (example uses Chisel 7.2.0, not 7.14.0) and to override with `CHISEL_FIRTOOL_PATH`.
**source** https://www.chisel-lang.org/api/latest/chisel3/BuildInfo$.html ; https://www.chisel-lang.org/docs/installation
**publisher** chisel-lang.org
**pub_date** labeled chisel 7.14.0 (API); live install page 2026-08-18
**accessed** 2026-08-18
**confidence** medium
**class** version/compat

**firtool-resolver is a Scala library and CLI that downloads LLVM Firtool as a Maven artifact.** LLVM Firtool Native publishes those binaries as Maven artifacts under FNDDS 1.0.0: `groupId`/`artifactId`/`version`, optional extra jars classified by platform (`linux|windows|macos` × `x64|aarch64`), with `FNDDS.version` and `project.version` files under a base directory and platform files under an artifact directory. That layout is how a non-Scala consumer can pin: fetch the matching Maven jar (or CIRCT tarball) for OS/arch rather than inventing a Scala-only pairing.
**source** https://raw.githubusercontent.com/chipsalliance/firtool-resolver/main/README.md
**publisher** chipsalliance/firtool-resolver
**pub_date** README on main (no dated release on the file); FNDDS 1.0.0
**accessed** 2026-08-18
**confidence** high
**class** integration

**`org.chipsalliance:llvm-firtool:1.155.0` is published on Maven Central (2026-08-11) as “Package of native firtool binary”**, with classifier jars `linux-x64`, `macos-aarch64`, `macos-x64`, `windows-x64`, plus an unclassified fat `llvm-firtool-1.155.0.jar`.
**source** https://repo1.maven.org/maven2/org/chipsalliance/llvm-firtool/1.155.0/ and `llvm-firtool-1.155.0.pom`
**publisher** Maven Central / org.chipsalliance
**pub_date** 2026-08-11
**accessed** 2026-08-18
**confidence** high
**class** integration

**CIRCT tag `firtool-1.155.0` exists with platform tarballs** (`firrtl-bin-linux-x64.tar.gz`, macos-arm64/x64, windows-x64, plus circt-full-* archives), matching the versioning page’s “install from pre-built artifacts” path for a non-Scala pin.
**source** https://github.com/llvm/circt/releases/tag/firtool-1.155.0
**publisher** llvm/circt / seldridge
**pub_date** 2026-08-11
**accessed** 2026-08-18
**confidence** high
**class** integration

**chipsalliance/chisel#4899 is still open.** Maintainer (seldridge, 2025-04-22): FIRRTL text parse-back into a Chisel/FIRRTL `Circuit` was dropped with the CIRCT migration; last Scala parser was Chisel 3.6 / FIRRTL 1.6; since Chisel 5, parse FIRRTL with `firtool -parse-only` / `circt-opt -import-firrtl`, or serialize objects (e.g. upickle) instead of FIRRTL text, or implement transforms as firtool pass plugins. No later comments on the fetched page.
**source** https://github.com/chipsalliance/chisel/issues/4899
**publisher** chipsalliance/chisel
**pub_date** created 2025-04-20; last update 2025-04-22; state open as fetched
**accessed** 2026-08-18
**confidence** high
**class** failure

**FIRRTL ABI (firrtl-spec v6.0.0 `abi.md`), one paragraph:** A second frontend must treat the circuit as a boundary contract, not an internal IR: the circuit itself has no ABI; **public modules keep their FIRRTL names as Verilog modules in `{module}.sv` plus `filelist_{module}.f`**, compiled as if never instantiated; **extmodules** use `defname` or the FIRRTL name and the same port-lowering ABIs, with implementation files supplied by the user. **Private modules have no ABI**; compilers **must mangle** remaining private-module names (scheme implementation-defined) to avoid collisions. Ports of public/ext modules follow **ABIv1** (aggregates scalarized to ground/netlist types; integer ports as packed vectors; ref ports become `` `define ref_{module}_{portname} `` macros in `ref_{module}.sv`) or **ABIv2** (vectors → packed vectors; passive bundles → packed structs; refs still split as if scalarized). Layers use named **bind** files `layers-{module}-{root}[-{nested}].sv` or **inline** `` `ifdef `` macros `layer${root}[$nested]`; bind module/port names are implementation-defined. Types on ABI-visible elements: `logic` packed vectors for integers, packed structs/vectors for aggregates, specified enum lowering; **properties have no ABI**.
**source** https://raw.githubusercontent.com/chipsalliance/firrtl-spec/v6.0.0/abi.md (release v6.0.0)
**publisher** chipsalliance/firrtl-spec
**pub_date** 2026-05-12
**accessed** 2026-08-18
**confidence** high
**class** pattern

## Leads worth chasing

- Chisel PR #5465 (“website docs about using specific versions of firtool”) cited on the v7.14.0 release; not fetched this run.
- Run the documented `scala-cli … BuildInfo.firtoolVersion` one-liner against `org.chipsalliance::chisel:7.14.0` to read the Option the Scaladoc omits.
- firtool-resolver CLI flags/invocation: README asserts a CLI exists but does not document a non-Scala command line this fetch.
- Unpack path inside `llvm-firtool-1.155.0-linux-x64.jar` per FNDDS `artifactDirectory` (not listed in the Maven directory HTML).
- CIRCT `firrtl-bin-*.tar.gz` vs fat `llvm-firtool-1.155.0.jar` as the Phase-1 pin artifact.

## Looked for and did not find

- A printed `BuildInfo.firtoolVersion` value of `1.155.0` (or any string) on the 7.14.0 Scaladoc page.
- Chisel 7.14.0 ↔ firtool rows in the fetched live versioning HTML (7.x tab did not render; source is SBT-generated).
- A firtool-resolver README recipe for pinning without Scala/JVM (only FNDDS Maven layout + “Scala library and CLI”).
- Maven Central `maven-metadata.xml` for `org.chipsalliance/llvm-firtool` (HTTP 500 this run).
- Any reopen/close activity on #4899 after 2025-04-22; no Chisel FIRRTL text parser restoration.
- An ABI requirement to preserve private-module names or internal lowering (explicitly undefined / implementation-defined).
