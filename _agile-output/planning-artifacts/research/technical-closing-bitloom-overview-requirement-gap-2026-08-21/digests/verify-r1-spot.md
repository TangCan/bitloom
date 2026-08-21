# Spot verification (validation=normal)

| ref claim | check | outcome |
|---|---|---|
| FIRRTL→Scala Circuit parse unsupported; use firtool/CIRCT | Fetched https://github.com/chipsalliance/chisel/issues/4899 (seldridge 2025-04) | verified |
| Chisel pins firtool per release; emit .fir + firtool encouraged | Fetched https://www.chisel-lang.org/docs/appendix/versioning | verified |
| Spinal maintainer prefers out-of-tree fresh IP vs spinal.lib | Fetched https://github.com/SpinalHDL/SpinalHDL/pull/1010 (Dolu1990 2023-03-21; wontfix) | verified |
