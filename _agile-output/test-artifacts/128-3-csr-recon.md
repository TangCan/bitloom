# Story128.3 CSR复用核对

2026-09-21；主代理独立读取 `ip/csr/{mod,rtl}.rs`、Timer与Epic128 NFR14，未改产品。

- PENDING 使用 Leaf W1C；自然事件在无总线提交或响应背压时仍更新，`(old & ~clear) | event` 已实现 set 优先。
- ENABLE 使用 Leaf RW；只有 `pending & enable` 影响输出，屏蔽不丢事件。
- TEST 使用 None WO：只有 read_commit/write_commit/candidate/write_mask，没有 value 端口。candidate 已按 WSTRB 与有效位掩码选择，再用成功 write_commit 门控，不能持续以 candidate 触发。
- RAW 使用 External RO：有 value 输入和两个 commit 输出，无 candidate/write_mask；接当前硬件事件零扩展，CSR 将其在读提交沿锁存。软件 TEST 不进入 RAW。
- 一个 CSR 提交口不能同拍既写 TEST 又写 PENDING。可达竞争是硬件事件与清除、硬件事件与 TEST；不能为了 cover 人造第二写入口。抽象 next-state 可说明 set 优先，但不可声称不可达总线场景实测。
- 定义体可组合复用，不新增无实例产品内核仅为跑 native；若保留 CSR 实例，native/generated 必须明确拒绝，完整行为以实际 RTL 验收。
- 端口接线依描述访问/owner 分支，不能复制 Timer 全部寄存器都有 candidate/value 的循环。
- 后续 UART/GPIO 未交付，五路单脉冲模拟属于 IRQ 输入验收。可用已交付真实 Timer 的 match_event→IRQ 验证 sticky EVENT 不自重触发；不把该夹具宣称完整子系统交付。

RAW可观察性注意：单口TEST成功写与RAW读不可同提交，所以若错误实现仅在TEST提交当拍将软件位OR到内部raw_value，总线读本身无法观测该瞬时错误。合同仍要求RAW只硬件，应以结构审核/独立形式raw_value对应assert核对；不能声称只靠一次TEST后RAW读即可检出所有可能的错误软件混入。pending/sticky错误来源则可用正常读直接检出。
