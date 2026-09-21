# Story128.5 主代理只读基座核对

基线2f52d9969f60d08cc633e30e0c263f05854b39fe，128.4七步done；本文件为可行性调查，不是新UART行为PASS。全文读旧uart.rs607行、param_sync_fifo.rs、csr/mod.rs、csr/rtl.rs；已读Epic128–129及完整epic128NFR14的UART精确合同。

1. ParamSyncFifo<8,4>具有共享session定义入口和独立Elaboratable；count为3bit，full输入拒绝不因同拍pop改变，empty无bypass；正常非满非空可同时push/pop，flush绑0、reset清count。可供TX/RX两个实例共享定义，状态不重复。
2. 旧UartTx/Rx为8bit分频、内部自行finish；旧Rx在raw rx下降后每period末采样，没有新合同要求的双同步、半bit确认/stop错误输出。应新增独立wrapper/串行状态实现，不原地加宽旧端口或修改旧FL/VIP语义。
3. CsrBlock叶只在非reset且无pending的submit提交；读拒绝不触发read_commit，合法零有效write_mask绕过write_reject且write_commit0。CTRL mask1、TX maskff故高字节WSTRB即使busy/full也不拒绝，不push。BAUD全部32位有效，任何非零WSTRB均有效。候选值独立于commit/reject，适合busy/invalid动态拒绝，不能反向形成组合环。
4. CTRL/BAUD可leaf RW唯一owner；成功配置候选→effective config→同拍空闲start及分频锁存。busy时BAUD有效写拒绝（包括同值），CTRL仅改变enable拒绝；零有效mask不干扰。TX_DATA WO候选低8经write_commit进TXFIFO；RX_DATA External RO来自队首，仅read_commit pop；提交前empty/full裁决，不以同沿到达/取走绕过。
5. 32bit倒计时装DIV，每次0推进后重装DIV，可表达DIV+1直到2^32而无需32bit计算溢出。H=floor((DIV+1)/2)可用(DIV>>1)+(DIV&1)，合法最小DIV3时H2；e检测同步下降后装H-1，e+H确认，随后每次装DIV形成周期P。该算术是实施提示，尚未跑产品。
6. 原始事件来自真实成功RX入队、TX取队首、overflow、framing，不来自sticky。与本地W1C/IRQ同沿采样，reset屏蔽；IRQ仅bit1/2/3，错误两类OR入bit3，不增加第6路。

ATDD仍需独立冻结公开类型/签名/端口/CSR字段和精确TX时序、RX采样拍表；端到端serial driver/decoder不能依赖DUTtimer当黄金。DIV3、奇数周期、phase0.1/0.5/0.9、单stop连续帧、配置/start竞争与full/empty同拍必须真实覆盖。最大DIV近终点或形式证据明确标注，不能声称模拟2^32周期。
