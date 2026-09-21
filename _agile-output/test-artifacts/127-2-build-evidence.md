> **Historical build snapshot:** 本文下方记录初次build。旧`127-2-build-sha256.txt`保持原样，是当时快照，不再指向修订后的live源码。旧源文件保存在`127-2-build-source-snapshot.tar.gz`（archive member路径对应旧清单）；原本文/清单/校验结果在`127-2-build-history/`。复查旧摘要时使用这些快照与未变更旧日志，不能直接对当前源码执行旧清单。后续修复、持久新源码与新摘要见[review-fix证据](127-2-review-fix-evidence.md)。

# Story127.2 build evidence

2026-09-21；baseline `77ea01392e5be74b9f2622009ad146edb2468ea9`。本记录仅build结果，独立review、automate、最终clean/fmt/workspace回归与commit由主代理继续。未改故事/sprint/goal状态，无push/publish。仅FR196描述/CSR叶子集；桥、四窗decoder、外设、M2/Phase24整体未交付，FR189 deferred及NFR91保留。

## 产品变化

`bitloom-prelude::ip::{CsrAccess,CsrOwner,CsrField,CsrRegister,CsrBlock}`，五个配置/生成方法；共享单session定义体，完整无hash参数codec；单响应槽、沿前提交pulse、读快照、动态reject、所有WSTRB、external唯一owner与W1C set优先。私有名字`_csr_`隔离公共identifier空间。中文文档/完整prelude-only例、FR142逐符号与minor登记，CI专用FR196 formal及失败归档入口。未改旧IP/builder/工具钉/版本。

## 实跑环境与命令

环境：`PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH`，cargo tests均`CARGO_PROFILE_TEST_OPT_LEVEL=1`。版本/每条exit见[environment](127-2-build-environment.log)：Rust/Cargo1.97.1、Icarus/vvp12.0、GCC13.3.0、Yosys0.33（环境bootstrap实际版本，非AD-9产品钉变更）、SBY yosys-0.47、Z3 4.8.12。沿用已有ci-sby-pins/install策略，无升级。

| 命令 | 实际结果 | 原始日志 |
|---|---|---|
| `cargo test -p bitloom --test fr196_csr_config --test fr196_csr -- --nocapture` | exit0，6配置+4行为 PASS；真实Icarus/peer/C11均执行 | [functional](127-2-build-functional.log) |
| `cargo test -p bitloom --test fr196_csr_config -- --nocapture` | exit0，6 PASS；追加关键字负例后重跑 | [config-final](127-2-build-config-final.log) |
| `cargo test -p bitloom-prelude csr` | exit0，3 private codec PASS | [codec](127-2-build-codec.log) |
| `cargo test -p bitloom --test fr196_csr_formal -- --ignored --nocapture` | exit0，2实际工具测试 PASS，非ignored计数 | [formal](127-2-build-formal.log) |
| `python3 scripts/check_fr196_example.py` | exit0；原文提取、prelude-only独立crate编译执行 | [example](127-2-build-example.log) |
| `cargo test -p bitloom --test fr193_axi_protocol --test fr98_axi_near_vip --test fr194_module_composition --test fr195_rv_reg_slice --test fr195_param_sync_fifo -- --nocapture` | exit0，62 PASS（12 AXI M0 + 5 composition + 31 FIFO + 9 slice + 5旧AXI） | [regression](127-2-build-regression.log) |
| scoped `rustfmt --edition 2024 --check`（CSR3源文件/config/formal）及 `git diff --check` | exit0；不是最终workspace检查 | 执行工具返回exit0 |

## 行为与软件黄金

原始[functional日志](127-2-build-functional.log)含沿前/沿后独立oracle、native Interpreter/Compiled和真实RTL。三个叶seed `127219605511`、`deadbeef1272`、`73592401ffff` 分别2310/2311/2311帧；peer seed `12720a11ce55` 2311帧。全部16WSTRB，max_stall31；计数如下（accepted=consumed+cancelled，最终无pending）：

| 场景 | accepted | consumed | cancelled | successful reads | effective writes | errors | zero-mask |
|---|---:|---:|---:|---:|---:|---:|---:|
| leaf 127219605511 |1015|1013|2|298|249|374|94|
| leaf deadbeef1272 |1013|1012|1|313|253|360|87|
| leaf 73592401ffff |1014|1012|2|301|225|390|98|
| peer 12720a11ce55 |1016|1013|3|283|256|381|96|

peer实际3模块同session，reject17、partial成功83、high候选允许9，独立递增/部分写优先。native/generated层级明确拒绝测试中的panic由catch_unwind预期捕获，测试PASS，非错误被忽略。external保留位独立测试PASS。

C11单头/重复include/双namespace头用独立手写offset/mask/access常量，`cc -std=c11 -pedantic-errors -Wall -Wextra -Werror -c`真实成功。原始C源码、头与编译日志在压缩归档，软件不是从生成结果反推oracle。

codec3测试含完整往返/键顺序、每个键逐一删除与重复、额外键、未知schema、count/length溢出、缺索引、非法enum/flag/byte/UTF-8及重建配置失败，使用现有Diagnostics且无公共测试接口。`endcase`遗漏已补，增加endfunction/always_ff/restrict负例；[关键字审计](127-2-build-keyword-audit.log)与现有builder完整HDL表无遗漏，并核验[Yosys官方lexer](https://raw.githubusercontent.com/YosysHQ/yosys/main/frontends/verilog/verilog_lexer.l)字面规则，无工具升级。

## Formal与原始综合

归档 `fr196-csr-formal/proof-878268` 保存csr.sby、observer、原始design.v、插桩design_formal.v、每个命令/版本/exit、SBY完整日志/model/traces/status。`prove: depth16` 基例全部通过，temporal induction在step14成功，SBY明确 `successful proof by k-induction`；是所列假设下安全归纳证明，非仅16拍BMC。没有添加/削弱任何安全assert或假设DUT正确。

假设只有首沿reset，以及req_valid被阻时请求payload/valid保持（reset豁免）；不假设rsp_ready最终为1，不证明无条件活性。端口独立reference检查pending、响应保持、状态、读写commit、候选值/mask，错误无访问副作用。`cover: depth24` 四项均达到：成功提交/错误提交/W1C clear-set冲突在step2，先停顿后响应恢复在step4。cover不是安全proof。

`fr196-csr-formal/synthesis-878268` 直接使用未插observer的design.v：`read_verilog; hierarchy -check; proc; check -assert; synth; check -assert; stat; write_json` 全部exit0；netlist实际有DFF、无LATCH。无PPA/BRAM/板级结论。

## 保留的失败与修复

1. [首次functional](127-2-build-functional-first-failure.log) exit101，2 PASS/2 FAIL。漏加现有工具PATH，`iverilog` probe返回127 `No such file or directory`；不是DUT错，也未skip。原始 `fr196-csr/*-852373` 诊断在归档。使用既定工具PATH后10测试通过。
2. [首次formal](127-2-build-formal-first-failure.log) exit101，synthesis PASS，prove exit16。ATDD observer的`ref_bytes`拼接缺外层右花括号，Yosys报 `design_formal.v:386 syntax error, unexpected ';'`。只将末尾 `{8{wstrb[0]}};` 修为 `{8{wstrb[0]}}};`；地址/值/模型/性质/假设不变。原始失败 `proof-854089` 完整留存，随后prove/cover/synthesis全部PASS。这是测试语法修复，不冒称红测发现产品行为bug。

## 持久证据

[工具归档](127-2-build-tool-artifacts.tar.gz) 保留target外源RTL、完整刺激testbench、C源/头、命令、日志、退出码、formal模型/trace/status及首轮失败，省略可重建的Icarus执行文件/目标对象。归档SHA256为 `086136a54e9f867de5bc9dc3c503e0f1801136710b2cfe1ba59eec7e26ebd1bb`。其余日志与本次产品源摘要见`127-2-build-sha256.txt`；最终clean不会删这些证据。

限制：真实验证是上述静态Probe配置与列出范围，不表示任意硬件/协议全状态都获证；独立review/automate/最终全workspace回归仍待主流程。没有恢复FR189或关闭NFR91。
