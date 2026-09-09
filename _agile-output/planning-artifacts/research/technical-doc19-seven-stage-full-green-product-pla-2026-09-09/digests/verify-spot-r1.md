# Digest: spot-verify load-bearing claims

agent: 92c8a70a-c883-40bd-adf0-2a38b4d4cb37
accessed: 2026-09-09

## Spot-check

| ID | Verdict | URL | Note |
|----|---------|-----|------|
| **A** | **verified** | https://github.com/chipsalliance/chisel/issues/4899 | Maintainers: FIRRTL text→Scala `Circuit` dropped with CIRCT (last OK ~Chisel 3.6 / FIRRTL 1.6); use `firtool` / CIRCT (`-parse-only` / `-import-firrtl`). |
| **B** | **verified** | https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Introduction/SpinalHDL.html | Docs: “SpinalHDL is not an HLS tool”; FAQ echoes non-HLS stance. |
| **C** | **verified** | https://docs.clash-lang.org/compiler-user-guide/general/faqs.html | FAQ: Clash-like interactive host sim rare among eDSLs (older Lava dual-embed); peers emit Verilog / lower to FIRRTL / interpret IR. |
| **D** | **verified** | https://panda.deib.polimi.it/?page_id=81 | PandA ships `bambu-*.AppImage`; docs center CLI (`bambu …`). No documented in-process HDL compiler API as primary surface. |
