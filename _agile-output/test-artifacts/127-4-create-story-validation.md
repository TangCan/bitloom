# Story127.4 create-story 验证记录

2026-09-21；基线 `91828619f66f787c63483dda4b0ee558bb7501e8`。这是七步第1步，仅创建故事，不是ATDD或产品验证；M2仍开放。

## 技能与配置执行

用户明确指定deprecated `bmad-create-story`，故按该技能执行而非替换为build。技能正文、discover-inputs、template、checklist已读；正常官方实施入口现为bmad-build，本次显式调用仍有效。resolver实际执行：

```text
uv run _bmad/scripts/resolve_customization.py --skill .agents/skills/bmad-create-story --project-root . --key workflow
```

退出0：activation_steps_prepend=[]、activation_steps_append=[]、persistent_facts=[]、on_complete空。配置rhdl/Richard/Chinese/intermediate、planning与implementation目录解析完成。无额外激活步骤。用户持续七步授权覆盖checklist常规改进选择提示，必要改进直接应用，无重复确认。

## discover-inputs与分析

| 输入 | 加载与分析 |
|---|---|
| epics_content | 1文件epics.md，SELECTIVE_LOAD完整Epic127四故事及Phase24依赖；127.4前置127.3已done |
| prd_content | 1适用深层分片prds/prd-rhdl-2026-08-19/addendum.md，Phase24 FR192–201/NFR93–99；浅层模式不能直接匹配实际深度，按已知目录定位 |
| architecture_content | 1深层脊柱ARCHITECTURE-SPINE.md，扫描AD并提取AD1/4/6/7/18/30/31；无新IR、单session、真实RTL、工具钉 |
| ux_content | 无适用UX文档；库/硬件与文档示例，不引入网页交互 |
| 补充合同 | AGENTS、phase24-contract、epic127 context/NFR14、CSR文档和127.2产品规格、127.3故事/最终验收/桥文档 |
| 实际源码/更新面 | CSR mod/rtl、桥axi_lite_csr、ip/mod、README、FR142清单、CI及桥文档示例脚本；UPDATE现状/增量/保留边界已写故事 |
| Git情报 | 最近5提交127.3/127.2/127.1/126.4/126.3；前故事1801/0/21与专用证明只列历史，不当当前通过 |
| sprint | 整份文件读取并遍历故事状态；创建前441done/12backlog/2deferred，127.1/2/3均done，127.4backlog |

独立子代理audit_contract只读审计合同：确认local地址转换、owner锁定、未命中单响应、共同reset、四叶真实组合与Epic128范围隔离；主代理独立核对同沿提交与固定四窗API。对“可配窗口”的审计建议不扩产品范围，采用固定ZST；固定map需静态不重叠/越界验证，不虚构不存在的用户配置诊断。

## checklist结果与修补

| 防错项 | 本故事落实 |
|---|---|
| 复用/防重复 | 复用AxiLiteCsrBridge+CsrBlock，decoder为新独立模块，禁止重写旧bank/CSR/桥语义 |
| 地址与错误 | 完整16位先命中再减base，低两位保留；窗外DECERR、窗内leaf SLVERR；端点/high alias与全地址静态oracle |
| 提交/归属 | 命中上游与叶同沿握手，不加分离提交队列；owner锁到消费，miss pending独立且可背压 |
| reset | req_ready/down_req_valid/down_rsp_ready在rst时0；up_rsp_valid不组合rst门控，reset沿后清；取消优先，共同reset |
| 独立验证 | raw AXI、真实四CsrBlock组合、独立oracle与producer monitor负例、prove/cover及observer突变、无observer综合与桥结构边界 |
| 软件/兼容 | 局部offset语义不变，四窗base独立黄金/C消费者；旧bank、M0/M1、CSR/bridge兼容重新验收 |
| 交付诚实 | 8条BDD AC和6项未完成任务；ready-for-dev；Epic128测试peer不称真实外设；七步后才M2关闭 |
| 信息密度 | API/状态/边界/测试/源码表集中，历史只留可执行教训；支持层级限制/FR142/minor与工具锁清楚 |

独立checklist复核未发现阻塞项，提出两项验收歧义已修清：65536地址逐个比较实际DUT而非仅自测golden；背压保持明确reset例外。主代理完整读取后确认合同并要求准确端口固化，已去除允许ATDD自行改名的措辞。

未发现需要改变正式合同的实质冲突。所有常规明确化已写入故事；ATDD负责确切端口/API冻结和实际红测，不在create阶段伪造测试通过。

## 本故事官方资料核验

2026-09-21实际打开[SBY官方reference](https://yosyshq.readthedocs.io/projects/sby/en/latest/reference.html)与[BFM上游README](https://github.com/alexforencich/cocotbext-axi)，提取prove/cover/depth区别及raw时序需要；实际HTTP读取[PyPI cocotb JSON](https://pypi.org/pypi/cocotb/json)得2.1.0、[cocotbext-axi JSON](https://pypi.org/pypi/cocotbext-axi/json)得0.1.28。只作当日技术核验，产品仍用已探测2.0.1/0.1.28，不改Rust/firtool/Chisel pin，不重新探测FR189。

## 状态与写入范围

仅本故事、此记录、sprint中127.4 backlog→ready-for-dev和last_updated。Epic127保持in-progress，其余故事状态、全部done与deferred保持；目标文件由主代理管理，本创建代理没有修改。没有实现、ATDD、clean、commit、push或publish。

完成前再次解析workflow.on_complete，空值无需终端动作；创建产物交给主代理核对后继续ATDD。
