# Phase 24 / Story 125.2：AXI-Lite BFM 工具 API 探针

2026-09-20：使用独立手写单寄存器 RTL stub 实测通过：**1 PASS / 0 FAIL / 0 SKIP，312 ns**。探针是 `scripts/phase24_axi_bfm_probe.py`，完整运行依赖版本在 `scripts/phase24-axi-bfm-requirements.txt`。这证明所选工具接口可用，**不证明 Bitloom `Axi4LiteSlave` 产品 RTL 已通过**，也不构成协议完备性证明。

## 固定环境与复现

本次：Python **3.12.3**、Icarus Verilog **12.0**、cocotb **2.0.1**、cocotbext-axi **0.1.28**、cocotb-bus **0.3.0**、find-libpython **0.5.1**、scapy **2.7.0**。requirements 固定全部五项运行时依赖；没有引入设计 crate 或 CLI 的产品依赖。首次安装需要包索引访问；此文件不是 wheel 哈希锁或禁网供给链证明。

```bash
uv venv --python /usr/bin/python3 /tmp/bitloom-phase24-bfm-py312
uv pip sync --python /tmp/bitloom-phase24-bfm-py312/bin/python \
  scripts/phase24-axi-bfm-requirements.txt
PATH=/tmp/bitloom-maintenance-tools/bin:$PATH \
  /tmp/bitloom-phase24-bfm-py312/bin/python scripts/phase24_axi_bfm_probe.py
```

`/usr/bin/python3` 在本机为 3.12.3；其他机器应明确选用受支持的 Python 3.12 环境。上述 Icarus 路径是本机已有的 12.0 安装包装器，其他机器安装该版本并提供 `iverilog`、`vvp` 至 PATH 即可。runner 缺 Icarus 会失败，脚本逐项断言 Python 包版本及 JUnit 报告恰有一个测试、没有失败或跳过。产物位于 `target/phase24-axi-bfm-probe/`，包括 `stub.sv`、仿真可执行文件及 `results.xml`。

默认 `uv venv` 在本机选择 Python 3.14.6，cocotb 2.0.1 安装明确拒绝该版本（最高支持 3.13）。实际探针改用 Python 3.12.3，没有启用绕过版本保护的环境变量。cocotbext-axi 0.1.28 在 cocotb 2.0.1 下输出上游弃用警告，但此探针无运行失败。

## 已核验接口和行为

实际从缺省 `AWPROT/ARPROT` 端口的 8 位地址、32 位数据 stub 构造：

```python
master = AxiLiteMaster(AxiLiteBus.from_prefix(dut, "s_axi"), dut.clk, dut.rst)
```

- `master.write_if.aw_channel.set_pause_generator(...)`
- `master.write_if.w_channel.set_pause_generator(...)`
- `master.write_if.b_channel.set_pause_generator(...)`
- `master.read_if.ar_channel.set_pause_generator(...)`
- `master.read_if.r_channel.set_pause_generator(...)`

五通道都使用重复 pause 序列，完成 `write(0, b"IP24")`、`read(0, 4)` 并验证 `AxiResp.OKAY` 与完整数据。暂停生成器可用 `set_pause_generator(None)` 停止；随后显式设置 `pause`。

复位接口：连接高有效 `dut.rst`；同时确认 write/read 子接口及五个 channel 的 `assert_reset` 可调用。安装源码 `cocotbext/axi/reset.py` 中签名为 `assert_reset(val=None)`；本探针实际使用外部 reset 信号，而非声称已经逐项执行该本地方法。时钟启动前先断言复位，防止未初始化 READY 的 X 值被 Python 转为布尔值。

探针另将 B 接收通道暂停，启动一笔写并验证它仍未完成；拉高 reset 后，该 `write()` 返回 **None**，不等于 OKAY。解除 reset 后读取返回零，证明工具在本次中止后可以继续工作。这个用例测的是 BFM 中止语义；半笔 AW/W 等产品复位矩阵仍须独立覆盖。

## 使用边界与依据

高层 BFM 会对调用的传输做分割和对齐，不能以 `write()`/`read()` 覆盖替代原始 WSTRB=0、特殊地址、半笔事务复位和所有交错时序。产品缺陷红测及最终回归可使用独立原始 RTL 通道驱动，避免高层转换隐藏刺激。本 stub 只存一个字，忽略地址；它不是可复用 IP 或参考协议实现。

接口选择依据：[cocotbext-axi 官方 PyPI 文档](https://pypi.org/project/cocotbext-axi/) 描述 AxiLiteMaster、read/write、reset 极性和传输拆分；[cocotb 2.0.1 Library Reference](https://docs.cocotb.org/en/v2.0.1/library_reference.html) 提供 Clock、Timer、ClockCycles、with_timeout 等调度 API。实际 pause 属性路径及 reset 的 None 返回还通过本次固定版本安装源码与仿真交叉核验；不将 PyPI 页面当前内容当成整个历史版本的兼容保证。
