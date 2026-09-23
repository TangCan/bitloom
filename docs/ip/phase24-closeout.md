# Phase24 范围结项验收

状态：已按批准范围完成结项（2026-09-23）。范围为2026-09-20批准的FR192–FR201 / NFR93–NFR99，六个Epic、22个故事。2026-09-23用户批准补审、总验收和状态同步；本次是后续维护验收提交，不改写22个单故事提交。

## 功能验收映射

|需求|交付范围|可审计证据|
|---|---|---|
|FR192|正式接口、复位/地址/兼容合同、M0与NFR14门|[合同](phase24-contract.md)、[M0关闭](../../_agile-output/implementation-artifacts/epic-125-closeout.md)|
|FR193|原AXI独立AW/W捕获、读写并发与手写FL修复|[实际红→绿、固定随机和双模型](phase24-axi-green-evidence.md)|
|FR194|共享ElaborateSession模块定义/参数身份/实例校验，单次freeze|[模块组合实测](module-composition-evidence.md)、[M1关闭](../../_agile-output/implementation-artifacts/epic-126-closeout.md)|
|FR195|两槽RvRegSlice与WIDTH1..64/DEPTH1..16寄存器FIFO|[注册切片](rv-reg-slice.md)、[FIFO边界与实测配置](param-sync-fifo.md)、[126.4最终验收](../../_agile-output/test-artifacts/126-4-final-verification.md)|
|FR196|静态CSR、同源Markdown/C头、AXI-Lite桥、四窗译码|[M2逐项映射](../../_agile-output/implementation-artifacts/epic-127-closeout.md)、[127.4验收](../../_agile-output/test-artifacts/127-4-final-verification.md)|
|FR197|Timer32、IRQ5、GPIO32、缓冲UART8N1及CSR|[M3关闭](../../_agile-output/implementation-artifacts/epic-128-closeout.md)、[128.5源绑定验收](../../_agile-output/test-artifacts/128-5-final-verification.md)|
|FR198|真实AXI-Lite与direct CSR两种四外设组合；单时钟/同步共同复位|[核心历史关闭](../../_agile-output/implementation-artifacts/epic-129-closeout.md)、[本次补审](../../_agile-output/test-artifacts/phase24-closeout-review.md)及[本次补验记录](../../_agile-output/test-artifacts/phase24-closeout-final-verification.md)|
|FR199|三仓库210文件完整commit/内容锁、许可、维护人、只读禁网隔离重放|[130.2验收](../../_agile-output/test-artifacts/130-2-final-verification.md)、[外部试点](external-fifo-pilot.md)|
|FR200|真实Bitloom HIR父模块绑定fifo_v3；固定32-bit/depth8/fall-through0，独立oracle与上游套件|[130.3验收](../../_agile-output/test-artifacts/130-3-final-verification.md)、[M5关闭](../../_agile-output/implementation-artifacts/epic-130-closeout.md)|
|FR201|核心/外部支持分栏、FR142显式登记、兼容门禁、owner与贡献流程|[支持矩阵](phase24-support-matrix.md)、[贡献模板](contribution-template.md)、[公开API清单](../public-api-1-0-surface.md)|

所有历史最终验收只证明当时对应源/工具快照；补审暴露的问题如实追加修复与新证据，不将历史done当作正确性证明，也不倒填旧日志。

## 非功能合同验收

|需求|结项判断和证据|
|---|---|
|NFR93|历史关闭保留；[本次20项审查/12组处置](../../_agile-output/test-artifacts/phase24-closeout-review.md)公开记录；FR189继续未交付，NFR91保留。|
|NFR94|125–130各自.1 NFR14文件与后续提交顺序可追溯；[完整22故事提交/维护文件数](../../_agile-output/test-artifacts/phase24-closeout-costs.json)以完整git标识登记，结项修补单独维护提交。|
|NFR95|Rust回归、实际RTL、原始综合、有限形式分别列证；ignored不计PASS；补验验证新鲜证据、工具缺失与坏证据硬失败。|
|NFR96|设计依赖prelude；旧API与FL保持，新增表面已按FR142/minor登记；产品工具钉、MSRV、包版本不变。补审只修验收链；SemVer验证工具固定已有0.50.0，属于CI复现配置。|
|NFR97|单时钟，内部同步reset；32位数据/16位地址、小寄存器FIFO；native/generated层级仍unsupported，不交付通用FIRRTL内存或复杂外部核。|
|NFR98|[外部试点文档](external-fifo-pilot.md)和130.2/130.3锁定源/许可/工具证据；获取属于CLI，prelude不拉源码，无模型明确拒绝。|
|NFR99|见下节真实复用、维护表面积、操作步骤与运行成本；不把估算人日、代理墙钟或综合cells换算成生产率/加速/PPA收益。|

## NFR99：实测结果与解释边界

两种实质不同的拓扑共享UART/GPIO/Timer/IRQ四种叶模块及同一CsrDecoder定义；AXI版本增加AxiLiteCsrBridge，direct CSR版本绕过桥。它们共用`system(axi)`生成函数，复用由结构测试和实际独立行为测试核验，不只凭文字计数。

[可复现测量脚本](../../_agile-output/test-artifacts/phase24-closeout-measure.py)记录源码哈希、物理行数、22个提交的修改文件数；行数含空行和注释，仅表示维护表面积，不表示节省的人日。独立SV oracle中硬编码地址是保持预期独立性的维护成本，不以DUT地址自动生成代替。

|已测项目|结果|解释|
|---|---|---|
|共享组合实现|264物理行，其中`system(axi)`148行|含注释/空行；两种拓扑共用，不是节省代码量的对照实验|
|复用对象|四种叶模块 + 一个decoder；AXI另加bridge|两个真实系统均执行独立预期|
|独立oracle维护|AXI307行、direct CSR270行；含16位地址字面的行分别48/42|不同总线驱动保持独立；不自动从DUT生成预期|
|交付维护表面积|22个单故事提交，逐提交文件数与完整hash见JSON|包括代码、测试、文档和原始证据；不等于人工工时|
|本次工具补齐|apt update约4.88秒、JDK17安装约17.50秒、SBY克隆约2.09秒、安装约0.03秒、sbt launcher下载约2.96秒|本机已有Rust/Cargo/Maven缓存和Icarus/Yosys/Z3/firtool；不是全新机器总安装时间|

本地上手流程按实际操作分四步：取得checkout；安装/确认Rust、Icarus、Yosys/Z3/SBY、JDK17/sbt及SemVer检查工具；使用CLI确保既定firtool并设置路径；执行`just fr198-fr201-core-check`。默认入口自行完成独立checkout、空Cargo target重放和证据审核。首次需要Cargo/Maven及工具下载网络；有缓存并不等于离线保证。工具准备原始命令与耗时见[准备记录](../../_agile-output/test-artifacts/phase24-closeout-tools/provision.json)。这是维护者复现测量，不是新用户可用性试验。

[补验实测](../../_agile-output/test-artifacts/phase24-closeout-final-verification.md)：完整主门禁加隔离重放275.947秒，其中独立checkout/空Cargo target子流程153.715秒；两处使用已有registry/Maven缓存。主运行AXI三后端46.376秒、CSR三后端42.409秒、formal22.561秒、真实SemVer6.931秒；隔离相应为60.425、40.466、25.385、16.900秒。不可将后端单步时长与总耗时混用。远端GitHub Actions本轮未触发，因此没有远端CI实际排队/运行/计费数据；报告的是同一required命令的本地执行成本，不能称远端CI实测。核心CI预算35分钟，外部试点75分钟是配置上限，不是实测时长。

维护责任由Richard承担：模块/API变化需要两组合三后端行为、有限formal/原始综合、兼容检查；外部源升级还需闭包/许可/工具重锁、独立oracle、上游套件及只读禁网重放。代价被列明，尚无长期故障率、平均修复时间、人工工时或对照项目基线，故不作定量生产率提升承诺。

## 保留事项与恢复条件

- **FR189 / Epic122 / Story122.2、122.3：deferred，未交付。** 本轮不重新断言上游最新版本。恢复前须重新核验已发布firtool严格大于1.159.0，按既有NFR91/AD-9/NFR14合同确认升级范围、工具身份、配对/非配对诚实声明和实际验收；owner Richard。当前授权不产生自动追随浮动版本或发布行为。
- **NFR91与既有明确遗留：保留。** 实例名/参数覆盖等先前已登记事项见[遗留账本](../../_agile-output/implementation-artifacts/deferred-work.md)，不把406项历史retro action done当作全账本清空。
- 本阶段不含native/generated层级仿真、异步/深RAM FIFO、多时钟/多主/复杂协议/任意外部核、完整AXI/UART形式证明、PPA/时序/物理签核；需要新增范围合同。
- 结项是获批范围交付验收；远端CI/分支保护运行、push和publish未执行，也不是本次关闭的必要交付项。

## 最终验证

四层补审及追加复核无未解决的范围内阻断问题。核心两拓扑六路线与隔离重放实际通过；最终工作区回归487个结果块、1894 passed / 0 failed / 56 ignored，ignored不计PASS。执行配置、原始日志和有限验证边界见[本次完整验收记录](../../_agile-output/test-artifacts/phase24-closeout-final-verification.md)。
