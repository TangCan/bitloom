# Story129.1 主代理构建核验（2026-09-22）

根代理读取实际生成RTL、测试台、端口shim、适配边界及Chisel综合日志，并独立校验复位归档157成员与基座归档341成员的SHA。不是仅接受实施代理的PASS声明。

`129-1-root-vcd-audit.py`实际解析最终r3三个后端的VCD顶层输出，逐路核时间6/10/12/14/16：两个不同非零状态、低沿断言保持、有效沿同清零且优先非零写、释放沿前仍零、下一有效沿恢复。15个独立采样通过；JSON绑定每份VCD的SHA。原始路径移入归档后，以`129-1-reset-evidence.tar.gz`内同名`129-1-reset-raw-r3/…`成员复原到test-artifacts可重跑。时间单位是此逻辑探针任意仿真单位，不是板级时序承诺。

Chisel适配实际连接：ResetTop反相aresetn→CorePorts.rst→ResetCore.reset，两Cell共用该reset；这是随源码交付的实际电路边界，测试台没有另绑Chisel reset。原typed路线的Chisel失败位于firtool对公共抽象Reset端口的降低，不足以证明后续隐式连接的行为故障。原失败完整保存，选择适配路线未修改产品后端或缩小AD-30系统接口。

最终Chisel纯设计综合统计包含两Cell、16个同步使能触发器与1个反相器，进程0；未纳入testbench。三后端有限图通过不等于完整四外设系统已通过。实际完整系统、两拓扑和干净checkout配方仍属129.2，CI/核心关闭属129.3。

规格矩阵映射：同步复位行由三个真实TB及独立VCD审计覆盖；显式/隐式行由typed失败原文和适配路线实际状态核验覆盖（不宣称未执行的typed行为）；固定基座行由本次BFM1例与桥译码exact1例覆盖；状态门由84场景normal/-O及真实sprint哈希保护覆盖。18项人工清单的最终七步M18尚未关闭。

当前阶段仅build证据核验；独立内审、外部code-review、automate及最终clean/fmt/just test/提交完成前不标Story done。
