# Story128.5 create-story验证

日期：2026-09-21。基线`2f52d9969f60d08cc633e30e0c263f05854b39fe`。结果：故事上下文ready-for-dev；没有产品实现或测试PASS宣称。

## 技能与发现

按用户明确七步要求执行deprecated `bmad-create-story`：全文SKILL、resolver workflow、config、discover-inputs、template和checklist。resolver prepend/append/persistent_facts/on_complete均空；Chinese文档/交流，Richard，intermediate。用户持续授权覆盖例行确认，未要求重复许可。

SELECTIVE_LOAD完整Epic128–129、PRD addendum Phase24、架构AD-28/30/31和单HIR/依赖/工具约束；全文正式Phase24合同、Epic128 NFR14（含末尾同拍/连续输入细化）、前故事128.4。无适用UX输入。完整解析3540行sprint/709状态，核对所有development_status；基线446done/7backlog/2deferred。最近五个单故事提交GPIO/IRQ/Timer/NFR14/decoder用于连续性，旧状态按历史保存。

全文读旧uart.rs、param_sync_fifo.rs、CSR rtl及ip/mod.rs，主代理另独立核对csr/mod与上述基座。全文UPDATE审计为ip/mod.rs、public-api-1-0-surface、phase24-contract、ci.yml；sprint完整解析。旧UART为8bit分频且RX不满足新同步/中心采样/错误合同，选独立wrapper；现有ParamSyncFifo满空/同拍规则恰匹配，不需另造队列或改旧接口。主代理核对和算术记录有链接，算术非DUT验证。

实际读取SBY官方reference（https://yosyshq.readthedocs.io/projects/sby/en/latest/reference.html），只引用prove/cover和状态语义，不升级固定安装身份或声称工具钉为latest。未引入新库，无版本迁移需要。

## Checklist核对

|检查|结果|
|---|---|
|依赖|128.1/128.3/126.4 done，127.2基座done；128.2/4 done为最终M3前置，未提前关M3|
|合同|六地址/字段/owner、全部WSTRB、提交前满空、busy候选拒绝与零mask、32位DIV、四原始事件均明确|
|RX|e为沿前s2与history检测沿；H向下取整；DIV3/4/5固定拍表；相位/短start/坏stop/单stop背靠背帧/reset释放均列必达|
|同拍|成功配置先于空闲启动且分频锁存；满RX+pop+到达仍overflow；空RX+到达读错；满TX+取走仍写错；无环candidate→reject→commit→effective_config|
|兼容|旧UART8/VIP/FL、FIFO和所有已交付IP保持；native/generated层级unsupported、prelude-only、工具版本不变|
|证据|独立黄金、三实际RTL后端、有限formal/cover/原始综合、指定行为DUT负控制、seed/执行数/源绑定/原始归档齐全性防空过|
|关闭|七步和一故事一提交，M3按128.1–5真实证据映射；不吞129完整系统/130外部/FR189 deferred/NFR91|
|状态|本次仅128.5 ready-for-dev及last_updated，全部其它YAML键语义保持、注释结构保持|

fresh-context独立子代理`/root/uart1285_story/validate`按checklist全文重读故事/合同/NFR14/Epic128–129：0必须修正项、无已知规格实施阻塞；确认e/H、全宽、无环配置、FIFO碰撞和M3边界。API具体命名/签名/端口留ATDD显式冻结，是下一步骤任务而非缺合同。7项AC、5项任务全部待实现，未运行产品测试、未commit。
