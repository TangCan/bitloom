### Findings

1. **Claim:** Dominant teaching path for RV32 microarchitecture is the Harris/DDCA ladder—**single-cycle → multicycle (FSM) → classic 5-stage pipeline**—on a **limited RV32I subset**; bit-serial is a separate niche. WCAE’21 materials list Ch.7 as “Single-cycle, multicycle, & pipelined” with instruction subset `add, sub, and, or, slt, lw, sw, beq, jal, I-type ALU`, and labs 8–10 as Single-Cycle Processor → Multicycle Datapath → Multicycle Control (SystemVerilog).  
   **Source:** https://pages.hmc.edu/harris/research/WCAE_Paper8_2021_DDCA_RISCV_Harris_lightningslides.pdf  
   **Publisher:** Sarah L. Harris & David Harris (WCAE ’21)  
   **pub_date:** 2021 (slides: expected book pub Aug 15, 2021)  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** pattern | tutorial-structure | isa-subset

2. **Claim:** At teaching scale, single-cycle is the simplest but pays for longest path (`lw`), separate I/D memories, and multiple ALUs/adders; multicycle shortens cycle time and reuses hardware via FSM steps (tradeoff: sequencing overhead); pipeline adds temporal overlap via 5 stages (Fetch/Decode/Execute/Memory/Writeback).  
   **Source:** https://pages.hmc.edu/harris/class/e85/DDCArv_Ch7.pdf  
   **Publisher:** Sarah Harris & David Harris / Harvey Mudd E85 course notes (DDCA RISC-V Ch.7)  
   **pub_date:** materials © 2020 Elsevier (course PDF)  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** pattern | failure

3. **Claim:** Bit-serial (SERV-style) processes **one bit per cycle** for extreme area (~125 LUT / 164 FF Artix-7 minimal config excl. RF); base **RV32IZifencei**, optional C/M/**Zicsr**; listed applications include **Education** for teaching RISC-V/CPU design—not the standard “first datapath” course ladder.  
   **Source:** https://serv.readthedocs.io/en/latest/datasheet.html  
   **Publisher:** Olof Kindgren / SERV documentation  
   **pub_date:** ©2020 (docs site; content current as fetched)  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** pattern | isa-subset

4. **Claim:** Pipeline teaching failure modes repeatedly stressed: **RAW** resolved by forwarding when producer has reached EX+; **load-use** needs stall+forward (data not ready until MEM); **taken branch** under predict-not-taken costs **2 bubble cycles** (flush wrong-path fetches).  
   **Source:** https://cs224.cs.vassar.edu/labs/riscv_pipeline/  
   **Publisher:** Vassar CMPU-224 (Computer Organization) lab  
   **pub_date:** undated course page (content as of access; no explicit year on page)  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** failure

5. **Claim:** Implementation labs report concrete memory-path pain: `sw` needs both forwarded `rs2` data **and** immediate; `lw`→`sw` can still need a stall; waveform debugging of multi-instruction pipeline state is expected.  
   **Source:** https://www.rose-hulman.edu/Class/csse/csse232/Practical9/  
   **Publisher:** Rose-Hulman CSSE232 Computer Architecture I  
   **pub_date:** undated practical page  
   **accessed:** 2026-08-20  
   **confidence:** medium-high  
   **class:** failure

6. **Claim:** First teaching cores often ship **RV32I without CSR/system**—e.g. NJU Lab 11: all RV32I **except** ECALL/EBREAK, FENCE, and CSR access (**37** instructions), single-cycle FPGA build.  
   **Source:** https://nju-projectn.github.io/dlco-lecture-note/en/exp/11.html  
   **Publisher:** NJU School of Computer Science (DLCO course experiment notes)  
   **pub_date:** undated English lab page  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** isa-subset | tutorial-structure

7. **Claim:** Spec floor for privilege: **M-mode is the only mandatory** privilege level; simplest platforms may be **M-only** (no U/S protection). Privileged architecture **requires Zicsr** for CSR RMW; other privileged ops depend on feature set.  
   **Sources:** https://docs.riscv.org/reference/isa/v20260120/priv/priv-intro.html ; https://docs.riscv.org/reference/isa/v20260120/priv/priv-csrs.html  
   **Publisher:** RISC-V International (Ratified Specs Library)  
   **pub_date:** library revision labeled v20260120  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** isa-subset

8. **Claim:** Successful pipeline tutorial staging (after a prior single-cycle): (I) pipeline registers / left→right datapath copy with IF given; (II) forwarding; (III) branch flush/bubble; (IV) hazard detection for load-use etc.—explicit “do not modify module I/O” grading constraint.  
   **Source:** https://raw.githubusercontent.com/jlpteaching/dinocpu/main/assignments/assignment-3.md  
   **Publisher:** Jason Lowe-Power et al. / DINO CPU (UC Davis ECS 154B lineage)  
   **pub_date:** originally Winter 2019; modified Spring 2020 (header)  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** tutorial-structure | failure

9. **Claim (scale/complexity summary from above):** Single-cycle = semester/lab entry (full combinational path); multicycle = next lab after ALU/FSM practice; 5-stage = later assignment with hazards; SERV bit-serial = tiny-area / deep-embedded or “how small can ISA get,” optional Zicsr for traps/timer—not the default first teaching CPU.  
   **Sources:** WCAE slides + DDCArv Ch.7 + SERV datasheet (URLs above)  
   **Publisher:** as above  
   **pub_date:** 2020–2021 for primary teaching sources  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** pattern

### Leads worth chasing

- Harris companion **Labs zip** / HDL zip on https://pages.hmc.edu/harris/ddca/ddcarv.html for exact stepwise datapath additions (R-type → I-type → memory → branch).
- SERV internals “Instruction life cycle” (Fetch → Decode → Execute) for bit-serial staging contrast: https://serv.readthedocs.io/en/latest/
- Unprivileged Volume I RV32I base chapter for “40 instructions” vs course-cut subsets (Harris/NJU).
- CTU BIE-APS “picoRISC-V” subset lecture (lw/sw, few ALU, beq/jal/jalr) as alternate minimal teaching ISA: https://comparch.edu.cvut.cz/courses/fit/bi-aps/BIE-APS-Lecture03-SingleCycleCPU.pdf (2022).

### Looked for but not found

- Recent (≤2 yr) **industry/course post-mortems** specifically blaming incomplete ISA subsets on student surprise (beyond lab omission of CSR/ecall).
- Explicit primary-source comparison ranking bit-serial vs single-cycle as **teaching-first** default (SERV claims education use; courses still ladder Harris-style).
- MIT/Berkeley/CMU **public lab handouts** in this run that fully parallel Harris’s three-µarch sequence (search hit other schools; those three not retrieved as primary lab pages here).