# 128.4 主代理只读CSR/旧GPIO核对

基线433771cdf62fba13870ed79b22880c23e5211e59。全文读取phase24-contract.md、gpio/base.rs、irq.rs、csr/rtl.rs。旧Gpio为8位组合pad_in、dir输入、masked OUT；保留其旧API和FL，不将其默改成正式32位wrapper。

CSR External RW的out_value由wrapper唯一OUT状态驱动，leaf产生按WSTRB合并的candidate与成功有效write_commit。SET/CLEAR采用WO/None，其candidate是已mask的写1位；没有value端口。仅三个write_commit之一为真时更新OUT；单请求接口三写不可同沿发生。DIR可Leaf RW，RISE_EVENT可Leaf W1C并复用set胜clear，IN为External RO。不得为合并OUT/SET/CLEAR再建第二份OUT owner，不需修改CSR既有语义。

建议ATDD固定沿前状态模型：edge时sync1'=pad_in，sync2'=sync1，history'=sync2；当沿rise=sync2 & ~history & ~DIR，IN请求锁存沿前sync2。raw GPIO IRQ归约rise，供本地W1C和IRQ同沿采样；DIR写同沿用旧DIR判定事件，reset清全部同步/历史状态。此为实现边界建议，最终由ATDD明确端口与逐拍黄金，不能以自然语言“两拍”代替采样沿。

必测bit31与四字节独立WSTRB；SET从0/混合状态、CLEAR从全1/混合状态，避免幂等掩盖故障。reset后初始高、持续高、下降沿、重复上升沿、改方向同沿、背压时自然事件、W1C碰撞及IRQ清除不重触发。真实同步链不证明亚稳态MTBF或滤毛刺；不扩pad/电气范围。

128.3最终已独立提交。其最新完整证据指纹562文件，737原始成员逐字节匹配；历史566计数包括当时生成日志/文件集，勿硬编码用于新故事。新runner应排除所有明确生成工件并对选定run的开始/结束/当前相关源码一致性验收。
