# Digest: Implementation reality — round 1

agent: lead-inline
accessed: 2026-09-09

## Findings

1. SpinalHDL publicly rejects HLS as a product goal (“What SpinalHDL is not … not an HLS tool”); IDE “done” is host Scala IDE features. | https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Introduction/SpinalHDL.html | SpinalHDL | master docs | 2026-09-09 | high | primary_doc
2. Production SV LSP (HRT slang-server) requires **shallow compilation** for keystroke path; full design analysis is optional/slower — implies multi-year / team investment even for SV, and **eDSL-netlist LSP is a worse ROI** than host-language LSP. | https://www.hudsonrivertrading.com/hrtbeat/designing-a-systemverilog-language-server/ | Hudson River Trading | HRTBeat (open-source announcement ~2026-03 per LinkedIn) | 2026-09-09 | high | primary_post
3. PandA Bambu ships as **CLI/AppImage/Docker**, not an in-process API inside foreign HDLs — wrapping is the sustainable integration shape. | https://docs.bambuhls.eu/da/df6/install_docker.html ; https://github.com/ferrandi/PandA-bambu/blob/main/README.md | PoliMi PandA | 2024.02 docs / live README | 2026-09-09 | high | primary_doc
4. Failure mode for “full green” roadmaps: treating **IP catalog + first-party LSP + auto TLM≡CA + idiomatic Chisel round-trip + in-process HLS** as one milestone — peers never ship that bundle; each item is multi-year or permanently out-of-core. | synthesis of landscape+integration+architecture digests | — | — | 2026-09-09 | medium | synthesis
5. Practical sequencing peers imply: (1) lock IR/file interchange + emit quality, (2) thin generators/stdlib + out-of-tree deep IP, (3) host LSP / shallow HDL LSP later, (4) shared-stimulus dual paths not dual products, (5) HLS as external wrap forever. | synthesis | — | — | 2026-09-09 | medium-high | decision_relevant

## Leads
- slang-server shallow compilation feature page for productization recipe
- Spinal lib vs VexRiscv out-of-tree as effort contrast (not quantified this round)

## Looked for but not found
- Public person-year estimates for “complete IP catalog” or “eDSL netlist LSP” across peers
