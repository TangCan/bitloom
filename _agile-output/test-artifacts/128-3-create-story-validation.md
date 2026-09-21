# Story128.3 create-story 验证

日期2026-09-21；基线9d2aff720025d5b084ca746de4b1c9938757b350；结果：ready-for-dev（仅上下文完成）。

## 流程与输入

完整读取bmad-create-story SKILL.md、discover-inputs.md、template.md、checklist.md和_bmad/bmm/config.yaml。workflow resolver得到prepend/append/persistent_facts/on_complete空，语言Chinese、用户Richard。用户指定128.3且已授权七步连续执行，跳过自动挑选及重复确认；技能虽deprecated，因用户明确七步调用而执行。主代理并行只读CSR核对满足技能并行研究指令，未另起无用子任务。

SELECTIVE_LOAD读取完整Epic128及129依赖、Epic128 NFR14全文、128.2故事全文与Timer代码、正式Phase24合同全部段落、历史implementation-plan含§5、深层PRD addendum FR197/NFR93–99及架构相关AD。默认一层PRD/architecture位置不覆盖实际布局，使用项目明确深层来源；无适用UX需求。读取mod.rs、FR142清单、CI全文，CSR rtl.rs及CSR文档全文；默认不改Timer/CSR基座。最近五提交128.2/128.1/127.4/127.3/127.2已核对。

实际通过web读取SBY官方reference：https://yosyshq.readthedocs.io/projects/sby/en/latest/reference.html ，只引用prove/bmc/cover语义，不据latest文档升级固定版本。无新库/框架或需要升级的技术依赖；不主张工具钉是最新。

## Checklist裁决与修正

|检查|结果|
|---|---|
|前置/范围|128.1、127.2 done，Epic128 in-progress；历史风险记录in-progress快照不阻止当前执行；128.4/5与完整系统不提前关闭|
|五源与软件语义|固定IRQ5、mask0x1f、四地址、唯一owner、raw不含TEST、mask不清pending、重复事件不计数|
|CSR复用|记录WO无value、RO无candidate/mask，避免复制Timer全RW循环；TEST必须write_commit门控|
|碰撞可达性|硬件+clear、硬件+TEST各自验证；单端口TEST写与PENDING写不能同时提交，不用不可达cover|
|真实集成|Timer match接IRQ0，Timer本地EVENT未清时清IRQ不重触发，之后新match重新置位；不冒充GPIO/UART实现|
|RTL与native边界|三后端实际执行同黄金；无无实例内核则native/generated N/A并检查拒绝，不为测试扩产品路径|
|形式与综合|安装源码字节身份、独立ghost/assert、可达cover/负控制、无observer原始综合各自证据；不能用版本输出/ignored计PASS|
|背压/错误/reset|快照、事件在stall中继续采样、软件只一次、无refill、所有16WSTRB、保留位、非法地址/权限、reset取消齐全|
|复用及维护|同定义与重命名双实例、隔离/不对称背压；prelude-only原文例/C头、FR142/minor、CI持久门禁/失败工件|
|七步诚实|所有任务未勾选；当前无新IRQ执行结果；clean前归档，最终完整workspace而非default-members|

审查发现并在初稿内修正：不可能的TEST+clear双写cover，WO/RO虚构端口风险，RAW应为提交沿硬件快照，原始事件输入不应额外边沿检测，只有Timer已实现时不能宣称其它源算法交付。无需用户重新决定既定范围。

API建议交给ATDD固定：ip::Irq、raw_events UInt5、irq UInt1加既有12CSR端口；pending/enable/test/raw全小写、五个命名字段timer/uart_rx/uart_tx/uart_error/gpio（mask1/2/4/8/16）。具体公共符号/端口/字段在ATDD中固定后build遵守。

## 状态完整性

程序读取sprint完整文件并yaml.safe_load全部development_status，核对前置和原backlog。仅精确替换128-3-事件-irq为ready-for-dev及last_updated；完整YAML与预期深比较相等，其余状态/metadata保持，原文本注释和结构未重写。128.2 done、FR189 deferred与其它已done保留。当前只创建故事/此验证及状态更新，无产品/测试代码、无commit。
