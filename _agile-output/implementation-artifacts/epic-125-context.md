# Epic 125 Context: M0 合同与 AXI 协议基础

## Goal

落实已批准Phase24规划，并修复旧Axi4LiteSlave已接受事务丢失，使FR192/FR193拥有native及真实RTL证据。仅M0获准执行。

## Stories

- 125.1：登记FR192–201/NFR93–99、6epic/22story、AD30/31、接口合同与NFR14。
- 125.2：生成当前RTL，独立期望复现AW/W分拍无B、并发读写丢R；记录工具探针与失败产物。
- 125.3：修复接收/响应状态及手写FL，兼容/native/RTL回归，重估并关闭M0。

## Requirements & Constraints

接口：`docs/ip/phase24-contract.md`；风险：`epic-125-nfr14.md`。FR192合同/M0闸门，FR193既有AXI正确性。NFR93历史保留、94每Epic风险/一故事一提交、95真实执行、96依赖/API/工具钉、97首版范围、98外部源证据、99收益实测。FR189/Epic122 deferred不变。

125.1只改合同/文档/状态检查；125.2只复现，不提前修复；125.3在红测证据后修复。设计仅prelude，不改旧端口/映射，不发布。研究native观察尚不代表RTL复现。

## Technical Decisions

旧bank同拍AW/W后1tick BVALID保持；AW/W分拍缓存，并发接受读写均响应，同址read-before-write；B/R受阻保持，reset清在途状态。手写FL同语义。未来CSR单执行请求容量不适用于旧bank。AD30/31限制未来同session/单freeze及构建层外部源绑定，此M0不实现它们。

真实RTL缺工具应失败，协议期望独立于实现；native/RTL差异先解释。红测隔离且125.3转为必须通过回归，不能缺陷观测通过冒称正确。

## Cross-Story Dependencies

125.1 → 125.2 → 125.3。主代理每故事单独提交。Epic125未done禁止126–130 ready/in-progress；之后其实现故事仍需相应.1 NFR14 done。此次后续只backlog，M0关闭后记录实测估算与是否继续，不自动执行全Phase24。检查入口：`python3 scripts/check_phase24_gate.py`。

## M0完成状态（2026-09-20）
125.1–125.3已完成；FR192/FR193关闭，真实红→绿及工作区证据见epic-125-closeout.md。以上Technical Decisions仍有效，125.2/125.3工作描述保留为故事上下文。后续126–130保持backlog。
