# Bitloom Episode II：流水线与 hazard 教程

公开产品名 **Bitloom**（`bitloom` / `cargo bitloom`）。与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。

前置：[Episode I](../rv32-episode-i/README.md)（`examples/rv32_core` 单周期子集）。本集核在 **`examples/rv32_pipe`**（`PIPE.md`）；立即数冻结仍见 **`examples/rv32_core`**（`SUBSET.md`）。

## 范围与非目标

| | |
|--|--|
| **范围** | 用户态立即数/符号扩展 → 经典 IF/ID/EX/MEM/WB → ALU 转发 → load-use 停顿 → predict-not-taken 分支 flush；可选 CSR 章（**`examples/rv32_priv`**，教学最小集） |
| **非目标** | cache / MMU / Linux Softcore；动态分支预测 / BTB；完整 Privileged 或 arch-test；PicoRV32 自定义 IRQ |
| **对照** | Harris DDCA Ch.7；FemtoRV Episode II；VexRiscv **仅对照**，不作第一路径 — 见 [femtorv-compare.md](./femtorv-compare.md) |

**NFR32：** CSR/trap 为可选里程碑。Epic 17（`rv32_pipe` 流水 DoD）**不**依赖本章 CSR 交付。

## 工具链

- **MSRV：** rustc **1.97.1** / edition 2024（`rust-toolchain.toml`）
- **构建：** `cargo bitloom`（包 `bitloom`，二进制 `cargo-bitloom`）
- **仿真：** `bitloom-sim` 仅 `[dev-dependencies]`；黄金在 `cargo test` 里 `tick`

### 路径 A：真独立（不必 clone monorepo）

```bash
rustup toolchain install 1.97.1
cargo install bitloom
cargo bitloom new my_pipe
# 设计只依赖 bitloom-prelude；仿真：
#   cargo add bitloom-sim --dev
```

对照上游 `examples/rv32_pipe` / `rv32_core` 抄写；**不得**把 clone 当成唯一入口。

### 路径 B：贡献者 monorepo

```bash
git clone https://github.com/TangCan/bitloom.git
cd bitloom
cargo test -p rv32_pipe
cargo test -p rv32_core
cargo run -p bitloom -- build --package rv32_pipe --manifest-dir . --out-dir /tmp/rv32-pipe-out
```

## 章节索引

| 章 | 主题 | 验收类型 | DoD（命令 / 测试名） |
|----|------|----------|----------------------|
| [00](./00-getting-started.md) | 工具链与包路径 | build / tick | `cargo test -p rv32_pipe`、`cargo bitloom build --package rv32_pipe` |
| [01](./01-isa-and-imm.md) | ISA / 立即数与符号扩展 | tick | `tick_addi_negative_imm_golden`（core）；`tick_addi_negative_imm_pipe_golden`（pipe） |
| [02](./02-five-stage.md) | 五级流水插入 | elaborate / tick | `elaborate_ok`；`tick_clean_path_addi_add_golden` |
| [03](./03-forwarding.md) | ALU RAW 转发 | tick | `tick_alu_alu_raw_forward_golden` |
| [04](./04-load-use.md) | Load-use 停顿 | tick | `tick_load_use_stall_atdd_golden` |
| [05](./05-branch-flush.md) | 分支 flush | tick | `tick_beq_taken_flush_wrong_path_not_committed` |
| [06](./06-csr-m-trap.md) | 可选 Zicsr + M-trap | tick（可选） | `cargo test -p rv32_priv`：`tick_mtvec_ecall_mret_golden`、`tick_csr_rmw_and_ie_serialize_golden` |
| [对照](./femtorv-compare.md) | Harris / FemtoRV / 可选 VCD | 阅读 | 对照表；不强制波形资产 |

每章只引入**一类**变化。主路径是 Bitloom `cargo` / `tick` / `build`，不以 Make/SBT 为必装。

## 合规措辞

与 [`examples/rv32_core/COMPLIANCE.md`](../../../examples/rv32_core/COMPLIANCE.md) 一致：定向黄金 →（可选、已延期）`riscv-tests` `rv32ui`；**不是**完整 DV；**不得**宣称 arch-test 等价。

## 延伸阅读

- Harris & Harris, *Digital Design and Computer Architecture: RISC-V Edition*, Ch.7
- FemtoRV [`FROM_BLINKER_TO_RISCV`](https://github.com/BrunoLevy/learn-fpga/blob/master/FemtoRV/TUTORIALS/FROM_BLINKER_TO_RISCV.md) 后半（流水 / hazard）
- 本章对照表：[femtorv-compare.md](./femtorv-compare.md)
- Episode I 大纲入口（已改指向本目录）：[`../rv32-episode-i/99-episode-ii-outline.md`](../rv32-episode-i/99-episode-ii-outline.md)
