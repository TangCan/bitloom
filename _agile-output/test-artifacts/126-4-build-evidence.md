# Story126.4 build 实测（2026-09-20）

实现代理按批准 spec 完成 ParamSyncFifo、文档、CI 与本阶段验证。未提交、未 cargo clean、未修改 story/sprint 状态；独立 code-review、automate 和最终 clean/fmt/just test 由主代理继续，当前不是 M1 关闭声明。

产品实现采用 count+DEPTH 个移位寄存器，pop 后移动、push 写入 count-pop 对应槽；count 宽度能表示 DEPTH（16 时5位）。WIDTH/DEPTH 校验先于 session 修改、宽度算术与分配；两个入口共用 define_body。旧 SyncFifo、native 层级拒绝、依赖与工具 pin 未修改。同步 reset 由 builder 优先施加，flush 清 count，取消后旧 payload 不再有效。

实际环境为 Rust/Cargo1.97.1、Icarus12.0、Yosys0.33 (2584903a060)、SBY yosys-0.47、Z3 4.8.12。原始版本探针见 `126-4-build-tools.log` 及工具目录内日志。运行 PATH 追加 `/tmp/bitloom-maintenance-tools/bin` 与 `/tmp/bitloom-1263-sby-installed/bin`，产品和 CI 不依赖该临时路径。Cargo 测试设 `CARGO_PROFILE_TEST_OPT_LEVEL=1`，保留断言与溢出检查。

| 实际命令 | 退出 / 结果 | 日志 |
|---|---|---|
| `cargo test -p bitloom --test fr195_param_sync_fifo -- --nocapture` | 0；31项通过，0 ignored；2.86s测试时间 | `126-4-build-api.log` |
| `cargo test -p bitloom --test fr195_param_sync_fifo_formal -- --ignored --nocapture` 首轮 | 101；4通过2失败；D2/3 basecase PASS，但 induction FAIL → UNKNOWN | `126-4-build-formal-first.log` |
| 同上，补纯assert不变量后 | 0；6项通过，0 ignored；9.54s测试时间 | `126-4-build-formal.log` |
| `cargo test -p bitloom --test fr82_fifo_uart_baseline --test fr103_ip_dual_model` | 0；4+8项通过 | `126-4-build-legacy.log` |
| `cargo test -p bitloom-prelude` | 0；17项通过，3个既有doc示例ignored | `126-4-build-prelude.log` |
| `python3 scripts/check_fr194_example.py` | 0；三个完整prelude-only程序编译执行 | `126-4-build-doc-example.log` |
| `rustfmt --edition 2024 crates/bitloom-prelude/src/ip/param_sync_fifo.rs crates/bitloom/tests/fr195_param_sync_fifo_formal.rs` | 0 | 仅本故事文件格式化；不是最终workspace fmt |
| `git diff --check` | 0 | 无空白错误 |

API/native/RTL矩阵实际执行24配置×3seed（0x12641950a551、0xdeadbeef8012、0x73592401ffff），每组420拍、最后20拍排空；每组在Interpreter、Compiled和Icarus中逐拍校验。日志逐配置记录accepted/delivered/cancelled、各占用命中、同时传输、满释放、在途reset/flush/重叠、最长背压与重复排空。DEPTH1同时传输为0，其余实际命中。三实例64×3/64×3/8×7层级RTL和旧SyncFifo已知RAM兼容也实际通过，不冒充native层级支持。

形式脚手架修复：最初纯端口队列不能约束归纳起点的未观察内部槽，导致D2/D3归纳UNKNOWN；没有报告证明通过。新增 `assert(count == ref_pending)` 与所有 live slot==独立端口ref_queue的纯断言，仍由真实归纳证明，无内部assume，无删除任何原合同断言。最终三个配置均basecase+temporal induction PASS；D1的4个cover均step4命中，D2的5个cover在step3/4/5/5/6，D3在step3/4/6/6/8。涵盖满→空、满reset恢复、满flush恢复、reset/flush重叠取消恢复及D>1同时进出。不添加ready公平性，不宣称无限活性。

综合仅输入未插入observer的原始design.v，Yosys `proc; check -assert; synth; check -assert`，再校验映射cells白名单和多驱动。1×1=7cells，8×3=66cells，64×16=2104cells；不是PPA或BRAM结果。严格CI复用既有安装脚本，新增FIFO专用ignored入口和formal/synthesis、RTL失败产物上传。

归档目录 `126-4-build-evidence/`：

- `fr195-fifo.tar.gz`：全部74个RTL运行目录，原始design.v、完整tb.sv、编译/执行/版本/命令日志；省略可重建的simulation可执行文件。
- `fr195-fifo-formal.tar.gz`：首轮失败与最终通过的全部目录，原始RTL、observer、注入后RTL、sby配置、工具日志、status、归纳失败轨迹和成功cover VCD/YW、模型及综合JSON。
- `synthesis-*-cells.json`：最终三组cell统计便于直接审阅。

归档均在target外，后续cargo clean不会删除。原始成功RTL进程3326052，首轮formal3327814，最终formal3336367；恢复后从各目录commands.log复现。只记录本地工具结果，不声称远端CI已跑。
