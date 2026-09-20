# 本轮项目事实审计（不作为行业证据）

版本：`29c52dfbfa403e0ae9930d1061e99cfe73bd41fc`；读取/执行日期：2026-09-20；来源发布者：本地 Bitloom 工作区；结论只适用于该快照。路径均相对项目根。

| ID | 本轮直接检查 | 来源与含义 |
|---|---|---|
| L1 | 设计正式依赖只能为 bitloom-prelude；prelude 不依赖后端 | `AGENTS.md`、`_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` AD-6；新IP先沿用prelude内部模块，不要求用户依赖CLI或新IP crate。 |
| L2 | 已导出 FIFO/UART/SPI/I2C/AXI/GPIO/CRC/黑盒 | `crates/bitloom-prelude/src/ip/mod.rs`、`docs/ip/README.md`；公开名称不等于完整协议合规，本轮不沿用“near-VIP”等标签作为新证明。 |
| L3 | SyncFifo 固定宽8、深4；同步单时钟；内存读寄存独立于rd_en | `crates/bitloom-prelude/src/ip/sync_fifo.rs`；参数化需新合同，不能只替换常量而忽略输出有效时机及存储器推断。 |
| L4 | AXI写提交要求同拍aw_fire与w_fire；同时写时屏蔽do_read但ARREADY仍可高 | `crates/bitloom-prelude/src/ip/axi.rs`，约228–255行；两种行为由下述执行探针确认。 |
| L5 | native/generated仿真显式拒绝多模块；已有实际层级RTL只证明指定夹具 | `crates/bitloom-sim/src/engine.rs` validate_single_module、`crates/bitloom/tests/simulator_bit_vectors.rs`；新组合系统先走真实外部RTL仿真，不把新增通用层级模拟器作为首版默认前置。 |
| L6 | 后端内存支持不齐 | `docs/backend-boundary-evidence-2026-09-20.md`及`crates/rhdl-firrtl/src/lib.rs`；FIRRTL文本内存未形成行为证明；direct/Chisel已有指定非碰撞夹具。新IP能力矩阵逐后端登记。 |
| L7 | IP::elaborate创建独立session并finish；add_instance只记录引用 | `crates/bitloom-prelude/src/ip/{uart,axi,sync_fifo}.rs`、`crates/bitloom-builder/src/lib.rs` add_instance；缺可直接把这些独立IP组合的统一模块定义接口，计划采用共享define_module入口、finish一次，不拼接冻结HIR或重建第二IR。 |
| L8 | ExtBlackBox目前是端口壳及空vendor stub，不是外部IP管理器 | `crates/bitloom-prelude/src/ip/blackbox.rs`；第三方适配需要真实module binding、源清单、参数映射和行为联验。 |
| L9 | ip_box分别测试各IP，未构建AXI到外设系统 | `examples/ip_box/src/lib.rs`；新组合示例是新增交付。该例子dev依赖比AD-6更宽；新例子正式依赖仍只prelude，后端测试放CLI integration。 |
| L10 | 公共API扩充有SemVer和表面登记要求 | `docs/public-api-1-0-surface.md`、`scripts/semver-check.sh`；新API草案须通过合同/兼容检查后才成为承诺。 |
| L11 | 一故事一提交，正式编号由新合同分配 | `_agile-output/implementation-artifacts/process-one-story-one-commit.md`；计划使用临时IP-* ID，不伪造已批准FR/Epic。FR189/NFR91状态不变。 |
| L12 | 性能结论仅为特定仿真工作量 | `docs/benchmarks/backend-boundary-profile-2026-09-20.md`；不能推成RTL PPA不足，也不能据此推断IP需求或经济收益。 |

## 实际最小探针

源码：[axi-observation.rs](axi-observation.rs)；输出：[axi-observation.log](axi-observation.log)。独立临时Cargo包，path依赖当前prelude、dev依赖sim，offline执行；未改产品源代码。两项测试是**观测当前缺陷的探针**，通过表示复现该行为，不是协议合格测试。

1. 复位后 AW 单独valid，settle观察AWREADY=1；下一拍撤AW、W单独valid，观察WREADY=1；之后5个观测拍无B响应。源码没有保存独立AW/W状态，不能把这解释为任意正常响应延时。
2. 同拍 AW/W/AR 均valid，观察三路ready均1；随后5个观测拍无R响应。源码do_write屏蔽do_read且未缓存AR，存在已接受请求丢失路径。

本轮只在native Sim执行。实施第一故事必须把上述序列转成**期望响应的红测试**并运行真实生成RTL；不得声称本轮已经验证外部RTL修复或形式证明。

复跑方式：把源码包装在`#[cfg(test)] mod audit { ... }`中；临时Cargo包edition=2024，[dependencies] bitloom-prelude路径指向本项目，[dev-dependencies] bitloom-sim同理；`cargo test --manifest-path <临时包>/Cargo.toml --target-dir target/ip-research-probe --offline -- --nocapture`。

## 文档卫生（计划任务，不在本轮顺手修改）

`docs/ip/README.md`仍写set_inputs后必须手动settle；与本轮前已提交的tick组合预稳定语义不一致。后续IP文档刷新应改为：仅在tick前观察组合输出时需要settle。保留历史合同记录，同时补当前能力限制。
