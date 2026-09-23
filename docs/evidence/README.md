# 原始验收证据归档（2026-09-23）

推送已中止；本次只整理本地尚未推送的历史，不更改已发布的 Bitloom 1.2.0 包。

68 个原始归档移出 Git 历史，校验清单、报告和代码保留。36 个原提交全部保留，其中 23 个 SHA 因树或父提交改变而改变。逐提交核对：除归档删除外，文件内容和模式一致，见 [验证记录](rewrite-verification.json)。原始 SHA 到整理后 SHA 的映射见 [commit-map.json](commit-map.json)。

## 存储与恢复

本机完整备份目录：

`/nvme_data2/richard/2026/rhdl-evidence/archive-cleanup-20260923-k8emktf7`

该目录保存独立对象、原始历史 bundle 及校验清单；这是本地备份，尚未配置异地存储。请同时保管目录及其 `objects/`。初始备份包含 70 个对象，其中两份很小的 `.tar.gz.sha256` 文本仍留在 Git；实际迁出清单为 [archive-index.json](archive-index.json) 中的 68 个对象。

在仓库根目录执行（省略 `--restore` 时只校验）：

```sh
python3 scripts/restore-evidence.py --store /nvme_data2/richard/2026/rhdl-evidence/archive-cleanup-20260923-k8emktf7 --restore
```

工具逐项验证大小与 SHA256，将原始字节恢复到忽略的原路径，不解压、不覆盖不同内容。历史报告中的归档相对链接在恢复后可用。新克隆需要另行取得外部证据库，Git 不再包含这些原始归档。

## 发布溯源与原历史

crates.io 1.2.0 的真实上传源码提交仍为 `e33d12fd747e576a542e24f18be1e3cfea4760d7`；整理后的对应提交是 `2cda375fbd5b8f1f33c6c3eb6abd93b73eda9e83`。历史发布记录和已上传包的 VCS 元数据保持原值，映射不代表重新发布。

原本地 HEAD 为 `956851d487a68cf6585113e8ec067cb5dedcf0ef`，由本地分支 `backup/pre-archive-cleanup-20260923-k8emktf7` 和 `unpublished-history.bundle` 保留。bundle 需要仓库已有基线 `f37a9ea4cd41ac73649a1e9ea15f6e34240f059e`，SHA256 为 `ef3b600f36954c7b5631dcd100d0bf95f2a05558e42ddb3f7a19d6f93bbdae3f`。可用 `git bundle verify <路径>/unpublished-history.bundle` 检查；恢复历史时在另一个有该基线的仓库中 fetch bundle 所列分支。

整理后主线仍继承远端基线，后续可正常快进推送，无需强推。不要推送备份分支，也不要使用 `--all` 或 `--mirror`，否则会重新引入归档历史。本次不执行推送。

## 防止再次膨胀

`.gitignore` 仅忽略验收目录的原始归档，保留校验文本与报告。安装本地提交检查：

```sh
git config core.hooksPath .githooks
python3 scripts/check-git-artifact-size.py --base f37a9ea4cd41ac73649a1e9ea15f6e34240f059e
python3 scripts/tests/test_archive_tools.py
```

pre-commit 拒绝暂存的原始证据归档及超过 5 MiB 的单个 blob；历史检查也检查先提交后删除的大对象。钩子需每个克隆单独配置，不是服务端强制策略。

工具验证覆盖正常恢复、重复恢复、冲突保护、损坏源、符号链接、路径越界、校验清单放行与大对象拒绝，见 [测试结果](tool-verification.json)。产品代码未变，本次未重跑 Rust 全工作区测试。
