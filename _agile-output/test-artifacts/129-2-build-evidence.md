# Story 129.2 build evidence

`crates/bitloom/tests/fr198_peripheral_system.rs`在各自一次`ElaborateSession`中定义真实bridge、decoder、UART/GPIO/Timer/IRQ并仅一次freeze。`Fr198AxiCore`经真实bridge连接decoder；`Fr198DirectCore`直接公开decoder CSR总线且不实例化bridge。两个生成core都经过受控源码中的无状态`aresetn -> ~aresetn -> rst`边界，边界和core共同参与仿真及`check -assert`。

独立testbench只观察公开总线、UART线、GPIO pad和IRQ。最新隔离checkout运行中，AXI执行16102笔事务/62416项断言，direct执行16098笔事务/60406项断言；每图均包含冻结的16 seed x 1000完成事务随机预算。两者还定向覆盖16种WSTRB、窗内洞/未对齐/访问类型SLVERR、窗外及高位DECERR、响应背压、Timer one-shot/periodic/wrap、GPIO、UART TX/RX/framing/overflow、IRQ五位清除后静默及新事件、共同reset取消。runner要求随机预算、确切计数marker、非空VCD、零退出码和严格Yosys，无Icarus不得跳过。

本机Icarus 12.0位于临时维护目录，不是产品输入。隔离checkout的一命令配方明确披露该前提：

```text
PATH=/tmp/bitloom-maintenance-tools/bin:$PATH _agile-output/test-artifacts/129-2-isolated-replay.sh
```

该脚本把HEAD及当前Story diff复制到临时独立checkout，设置独立`CARGO_TARGET_DIR`与`BITLOOM_FR198_ARTIFACT_ROOT`，不读取主工作区target或既有生成RTL。实跑从空Cargo target编译后，两图均由临时checkout源码生成`design.v`、`tb.sv`、VCD、工具日志和`evidence.json`；manifest保存HEAD、工作树patch SHA、Rust/Cargo/Python/RTL工具身份及系统/外设源SHA。首次空缓存会访问crates.io；日志如实保存该准备成本。默认PATH缺Icarus的负控制实际非零失败，诊断为`required iverilog executable is missing`，符合缺工具硬失败合同。

该证据只关闭Story 129.2的direct RTL行为与原始结构门。native Interpreter/Compiled/GeneratedFunctional层级仍unsupported；FIRRTL/Chisel系统复核、有限formal、CI/兼容与贡献模板属于129.3，不在此冒充PASS。Epic130外部试点、Phase24及FR189/NFR91状态不变。
