# 127.2 历史源码归档整理

独立review R9；2026-09-21。将13个历史源码/文档/CI副本（原review-fix-snapshot目录及review-fix-history单文件）归入[不可变归档](127-2-review-source-snapshots.tar.gz)。逐member SHA256核验成功后移除工作树普通副本，不丢原始字节。清单含archive与每个member摘要：[member index](127-2-review-source-snapshots-members.json)。

旧`127-2-review-fix-sha256.txt`原文不变，旧证据正文的snapshot链接属于当时路径；这些路径现在是tar member，不能直接把旧`sha256sum -c`失败误判为数据丢失。使用`python3 _agile-output/test-artifacts/127-2-verify-archived-sources.py`实际验证：13个归档member、26个原review证据摘要全部通过。验证器先校验archive，再逐member，再按原manifest校验全部证据；任何不匹配都失败，不用可被-O去掉的assert。

当前源码仍在crates/docs/scripts/.github，旧源码只在tar中供审计；归档不是活动测试或新实现。初始build-source-snapshot.tar.gz与原始工具tar不变。后续独立修补会有另一个新快照，不覆盖本轮历史。
