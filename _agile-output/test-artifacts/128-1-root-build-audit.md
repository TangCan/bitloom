# Story128.1 主代理基座核验

2026-09-21，主代理直接阅读正式合同全文、CSR rtl.rs候选/拒绝/提交更新段及fr196_csr.rs的peer_body/hierarchy/实际测试入口。

- External RW当前值来自外设输入；候选先按mask合并，reject不依赖commit，成功且非空mask才产生write_commit。W1C自然事件不依赖提交或响应槽。此结构能支持外设唯一状态owner，但不证明尚未实现的外设算法。
- peer_body在同一上升沿以commit选择candidate，否则自增；拒绝来自candidate高位和寄存count低位。最终三模块由同一个session.finish形成；预先独立elaborate仅查询端口，不拼接FrozenHir。
- 直接核对实际日志：固定BFM 1 PASS/0 FAIL/0 SKIP；CSR peer 1通过，2311帧、17拒绝、83部分写、9合法高位路径；gate35场景通过。原始pip查询exit1保留，metadata查询成功且BFM逐包断言固定版本。
- 独立程序逐个计算128-1-build-sha256.json列出的8个SHA，全部一致。打开13成员的归档，实际XML恰好1用例且无failure/error/skipped，RTL run.log含FR196 PASS。

本文件是主代理复核记录，不是额外测试用例数，不把同一探针重复计数；不代替后续独立code-review、automate和完整回归。以上核验时实施代理仍在整理人工映射，最终状态由七步证据决定。
