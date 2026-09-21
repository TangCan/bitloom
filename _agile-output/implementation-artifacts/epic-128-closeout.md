# Epic128 / FR197 / M3 实施关闭

128.1–128.5七步全部完成，随Story128.5单独提交关闭。映射：

|故事|范围|提交|
|---|---|---|
|128.1|外设NFR14合同与维护门|1c4cf20c4f8e8fdcb038d57a7b3f03f9f80b5c00|
|128.2|Timer32 CSR/raw match|9d2aff720025d5b084ca746de4b1c9938757b350|
|128.3|五源事件IRQ/RAW与TEST分离|433771cdf62fba13870ed79b22880c23e5211e59|
|128.4|GPIO32 CSR/同步输入/上升事件|2f52d9969f60d08cc633e30e0c263f05854b39fe|
|128.5|双FIFO UART CSR/真实串行及IRQ组合/M3|包含本文件的Story128.5单提交；自提交哈希由后续执行记录登记|

各故事最终验收见`../test-artifacts/128-{1,2,3,4,5}-final-verification.md`。UART最后回归：482个结果块、1870 passed / 0 failed / 47 ignored；当前完整39命令源绑定实证、原始工件独立校验及清理后归档完整性见128.5最终验收。旧GPIO8/UART8/VIP/FL保持，新API逐符号登记FR142/minor。

本关闭只对应FR197/M3：Epic129系统/核心矩阵与Epic130外部试点仍开放，Phase24尚未完整交付。native/generated层级unsupported、有限证明和物理边界保持；FR189/Epic122 deferred、NFR91未清空。工具钉/版本不变，不push/publish。此前draft和故事中间时点记录不再代表当前交付状态。
