# Story129.2 ATDD 验收清单

状态：green；每项均由本故事真实VVP外部断言覆盖，Yosys仅作为独立结构门。

| ID | P0 场景 | Given / When / Then | 实际证据 |
|---|---|---|---|
| 129.2-RTL-001 | AXI 四窗层级 | Given 一次 session 的 bridge/decoder/四叶；When emit/编译/运行 reset 后读写；Then 四窗真实副作用和响应可观察 | AXI 101事务/403断言；AW早/W早/同拍和并发读写均实际执行 |
| 129.2-RTL-002 | 直接 CSR 复用 | Given 无桥直接 CSR 图；When 相同地址向量运行；Then 与 AXI 图结构不同且四叶行为一致 | direct 97事务/362断言；HIR图无bridge且共享五个真实定义 |
| 129.2-RTL-003 | 地址错误 | Given 0x0400、0x8104、窗口空洞和未对齐；When CSR/AXI 请求；Then DECERR/SLVERR 与 rdata=0 | 两台均断言洞、未对齐、RO/WO错访、窗外、高位不别名和失败读零 |
| 129.2-RTL-004 | reset/背压取消 | Given AW-only/W-only/AR-only、待响应、FIFO 和非零叶状态；When 同步 reset；Then 共同清除且不重放 | AXI覆盖独立捕获及B/R受阻；direct覆盖受阻CSR响应；两台均验证四叶/FIFO清零和无重放 |
| 129.2-RTL-005 | IRQ 五源 | Given Timer/RX/TX/错误/GPIO真实事件；When 清 pending 后无新事件、再产生新事件；Then 不重触发且新事件到达 | 两台均经Timer、UART RX/TX、framing+overflow、GPIO真实副作用逐位读pending并清除/重触发 |
| 129.2-RECIPE-001 | 干净配方 | Given 独立 checkout；When 一命令执行两个拓扑；Then 工具身份、SHA、日志、VCD和退出码完整，失败硬停 | `129-2-automate.py`核验JSON/VCD并归档；默认PATH缺Icarus负控制实际非零 |

ATDD静态门在普通及`python -O`均为GREEN，且不用可被优化删除的`assert`。行为PASS只来自实际VVP；清单存在本身不计PASS。
