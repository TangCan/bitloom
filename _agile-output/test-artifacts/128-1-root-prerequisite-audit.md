# Story128.1 前置及复用边界核验

主代理在create-story并行阶段只读核验；基线`b926629e93b0ad5e09d795b59ec2f1bdb47010b6`。工作区启动时干净。127.4已单独提交，最终470结果块、1823 passed/0 failed/25 ignored，M2关闭；这些是历史前置证据，不是128.1新实测。

实际sprint的M0/125.3、M1/126.2/126.4、M2/127.2/127.4均done。128.1原始前置是M0，后续功能依赖按epics各条分别检查，不把已闭合M2推断为FR197已交付。`check_phase24_gate.py`全文已读，只检查M0/.1/epic关闭及FR189 deferred，不检查所有跨故事边；未来128.4须128.3+126.2，128.5须128.3+126.4，M3须128.1–5全部done。

已核对正式合同1–146行、AD30/31、Epic128全部5故事与现有源码：

- `ip/uart.rs`旧UartTx使用8-bit baud_div、无共享define_module；不能代替FR197的32-bit BAUD_DIV/双8×4FIFO。新API必须在功能故事逐符号登记，保留旧入口行为。最大BAUD_DIV+1须能表达2^32个时钟，软件/硬件计数算法不能无意截断成0。
- `ip/gpio/base.rs`全文：旧Gpio为8位组合pad读/写与输出寄存，共享helper已交付；无双级输入同步或逐位上升沿事件。新GPIO32必须保持新接口宽度，并保留旧GPIO8回归。
- `ip/csr/rtl.rs`关键提交逻辑提供read_commit/write_commit、candidate/write_mask、动态reject以及Leaf W1C set优先；自然事件独立于请求/待响应。Timer/GPIO/UART外部寄存状态必须唯一owner，消费响应不能重复push/pop/写入。
- 新IRQ严格5位原始事件脉冲；不能把本地粘滞EVENT电平接入而在清IRQ后立即重触发。RAW不含软件TEST，mask不丢pending。
- UART/GPIO同步路径是逻辑边界，不能宣称CDC物理/板级签核；无新增外部IRQ针脚。CSR读快照、零有效WSTRB、动态拒绝/SLVERR与共同同步reset沿用完整合同。

独立工具发现/行为探针、NFR14正文有效性及七步最终关闭属于后续ATDD/build/review，本文不替代这些证据。未来故事仍未实施，FR189/Epic122 deferred与NFR91保持。
