# architecture-r1-1.md
Access: 2026-08-20
## Claims
- IF/ID/EX/MEM/WB classic stages — KFUPM — high — pattern
- ALU RAW → forward EX/MEM & MEM/WB — MIT 6.823 — high — pattern
- Load-use → one-cycle stall then forward — MIT 6.823 — high — pattern
- Teaching control hazard: predict-not-taken + flush — CTU BE35APO — high — pattern
- Machine CSRs include mstatus/mie/mtvec/mscratch/mepc/mcause/mtval/mip — RISC-V priv-csrs v20250508 — high — version
- Trap: mepc/mcause/mstatus then mtvec; mret restore — privileged PDF — high — version
- Teaching min CSRs: mstatus mtvec mepc mcause mscratch ± mie/mip — Ecrionix/mriscv — medium — pattern
- Sign bit always inst[31]; all imm sign-extended — EECS-2016-118 — high — version
- B-imm: {31,7,30:25,11:8,0} — StackExchange quoting ISA — high — version
- Common bug: extra <<1 or contiguous B-imm — SE — high — failure
- Interrupt skid after CSR enable without flush — CVA6 #3175 — high — failure
## Sources
MIT pset; KFUPM; CTU wiki; priv-csrs; privileged PDF; EECS-2016-118; CVA6 3175; SE B-imm
