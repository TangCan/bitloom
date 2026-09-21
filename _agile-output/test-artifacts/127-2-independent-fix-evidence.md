# Story127.2 independent code-review patches

2026-09-21。对应[独立审查裁决](127-2-independent-review.md)中交给实现代理的8项代码/文档任务。本轮只做CSR相关定向验证，无旧M0/M1、workspace、clean、commit；未改story/sprint/spec/goal。旧build/review-fix日志、快照、清单均保持原样；root另行压缩历史源码，见`127-2-source-archive-compaction.md`。

## 逐项变化与实证

1. **多字段硬件黄金。** `p0_multifield_union_native_and_real_rtl` 用三个独立描述字段mask `1/f0/80000000`；手写的候选/并集mask/状态/readback分别覆盖enable写、high-byte写、mode覆盖、完全保留byte无pulse、全mask屏蔽保留位。黄金不遍历配置生成；两个native引擎和真实RTL全部PASS。若实现只用第一个字段，其高位/模式写期望会失败。
2. **read_reject完整合法形状。** `p0_read_rejection_for_leaf_external_and_w1c_native_and_real_rtl` 对leaf RW、external RW、W1C均开启read_reject。拒绝读SLVERR/rdata0/read_commit0；自然W1C事件在拒绝和背压持续；释放拒绝后同地址成功、read_commit1，提交前snapshot在之后event、peer值、reject变化中保持。两native+真实RTL PASS。
3. **真实formal增强。** Probe的leaf RW/W1C启用write_reject和read_reject，external RW也启用read_reject；独立observer按各自有效mask显式判拒绝/zero-mask、读快照与独立事件。保留全部旧安全assert、所有assume和5个cover，未假设DUT输出正确。实际prove基例depth16完成，step14归纳成功；cover depth24五项全部达到（成功/错误/W1C碰撞step2，背压恢复/取消后fresh step4）。无rsp_ready公平性或无条件活性宣称。
4. **独立producer稳定monitor。** 原随机/peer真实RTL与新增directed真实RTL均注入沿前monitor；两个native runner也使用独立monitor。只读实际driven inputs+实际DUT req_ready，不读expected/reference state；非reset未接受请求必须维持valid与write/addr/wdata/wstrb直到接受或reset。自动识别无前缀、`a_`、`b_`通道；两个实例各自监控。原所有fixture合法通过；额外负例对三个前缀分别制造撤回/改地址并确认monitor拒绝，reset可取消。负例的6条catch_unwind panic输出是预期成功测试，不是被忽略的协议失败。
5. **singleton原始综合。** 专用formal target新增一个测试枚举全部五个access/owner：leaf RW、external RO、WO None、leaf W1C、external RW。每个原始未插observer的RTL均跑Yosys hierarchy/proc/check/synth/check和netlist DFF/no-LATCH检查。结构断言为三个响应寄存器，只有leaf RW/W1C额外一个状态寄存器；external/WO无重复leaf状态。新增该结构断言后仅重跑singleton，5种再次PASS。它们仅是综合/结构验收，不声称singleton也套Probe安全证明。
6. **稳定codec映射。** `access_code`/`owner_code`显式match固定schema v1码，不依赖Rust enum顺序；decode映射不变。3个private codec测试及9配置测试PASS（包含所有合法owner组合共享定义与重建）。无新增公开API。
7. **文档边界。** 明确Generated Rust standalone本故事未验证，不能用native或层级拒绝替代；明确`Probe`/`PROBE`大写归一化同guard禁止不同内容，guard不会检测这种复用。固定宏/guard拼写与重复include行为保持，无identity API。
8. **C消费者。** 文档短例用`uintptr_t base + OFFSET`，再转volatile uint32_t指针，避免uint32_t指针缩放。`p1_documented_c_byte_offset_consumer_compiles_without_mmio_execution`提取原文C代码，生成与Rust例对应的Example头，严格C11 `-pedantic-errors -Wall -Wextra -Werror -c`成功。只编译对象，未执行MMIO；调用者负责有效映射与对齐base。

## 定向运行记录

完整命令/实际exit在[commands](127-2-independent-fix-commands.json)。环境与前轮固定工具相同：PATH前置`/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin`；cargo测试`CARGO_PROFILE_TEST_OPT_LEVEL=1`。本轮工具原始日志再次记录Icarus/vvp12.0、Yosys0.33、SBY yosys-0.47、Z3 4.8.12；Rust/Cargo1.97.1沿用。无pin升级。

| 定向入口 | 结果 | 日志 |
|---|---|---|
| `cargo test -p bitloom --test fr196_csr --test fr196_csr_config -- --nocapture` | exit0；10行为+9配置 PASS，0 failed/ignored | [functional](127-2-independent-fix-functional.log) |
| `cargo test -p bitloom --test fr196_csr_formal -- --ignored --nocapture` | exit0；3实际测试PASS（proof/cover、Probe原始综合、5singleton原始综合） | [formal](127-2-independent-fix-formal.log) |
| `cargo test -p bitloom --test fr196_csr_formal p0_each_supported_singleton_original_rtl_synthesis -- --exact --ignored --nocapture` | exit0；结构断言补入后1测试/5综合PASS | [singleton-final](127-2-independent-fix-singleton-final.log) |
| `cargo test -p bitloom-prelude csr` | exit0；3 codec PASS | [codec](127-2-independent-fix-codec.log) |
| `python3 scripts/check_fr196_example.py` | exit0；Rust文档原文prelude-only编译运行PASS | [example](127-2-independent-fix-example.log) |
| scoped rustfmt检查4个编辑Rust文件、`git diff --check` | exit0 | shell返回已核验 |

本轮没有新增失败执行；预期的非法配置/C宏冲突/producer违约/层级unsupported均由独立负例断言，不记作产品失败。原seed/31拍背压/16WSTRB计数保持，实际日志保留。

## 不可变归档

- [本轮源码tar](127-2-independent-fix-source-snapshot.tar.gz)：14个源/文档/脚本/CI成员，49,319 bytes，SHA256 `89817195d15d803d7f5b661b7393ef238b66c2e4c964737328caada1467313a6`。普通工作树没有重建旧`.rs`副本目录。成员原仓库路径与摘要见[member清单](127-2-independent-fix-source-members.sha256)，逐字节回读核验见[source verification](127-2-independent-fix-source-verification.log)。该清单用于tar成员，不承诺未来live-source不变。
- [本轮真实工具tar](127-2-independent-fix-tool-artifacts.tar.gz)：2,188,347 bytes，SHA256 `bd0c8955e496821aab832977eb117f3e7d9c414e6dde0ad85b90b1237d0fc8f4`。CSR PID1060736、C PID1062713、formal PID1060973及最终singleton PID1066670。包含原始RTL/完整testbench（含独立monitor）、文档C消费者/生成头/命令/exit、formal配置/observer/日志/status/cover traces、六类原始综合netlist；省略可重建Icarus executable/.o和重复formal model/src中间文件。
- [本轮总SHA256清单](127-2-independent-fix-sha256.txt)只核验新日志/报告/tar，不覆盖或改写旧清单；[校验结果](127-2-independent-fix-sha256-verification.log)全部OK。

限制：真实formal针对增强Probe，不推广到任意配置；singleton只综合；generated standalone未验；没有证明PPA/板级或无条件活性。后续automate/最终clean/fmt/workspace由主流程继续，FR196/M2、Phase24整体尚未关闭，FR189 deferred/NFR91保持。
