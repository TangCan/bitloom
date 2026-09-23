#!/usr/bin/env python3
"""校验仓库外原始证据，按需恢复到被Git忽略的原路径。默认只校验。"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import tempfile

ROOT = Path(__file__).resolve().parent.parent

def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        while chunk := stream.read(1024 * 1024):
            h.update(chunk)
    return h.hexdigest()

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--store', type=Path, required=True, help='包含objects目录的外部证据库')
    parser.add_argument('--restore', action='store_true', help='校验后恢复缺失文件，不覆盖已有不同文件')
    args = parser.parse_args()
    manifest = json.loads((ROOT / 'docs/evidence/archive-index.json').read_text())
    records = []
    for row in manifest['archive_objects']:
        relative = Path(row['path'])
        if relative.is_absolute() or '..' in relative.parts or relative.parts[:2] != ('_agile-output', 'test-artifacts'):
            raise ValueError('非法证据路径')
        source = args.store / 'objects' / row['git_oid']
        target = ROOT / relative
        if source.is_symlink() or not source.is_file() or source.stat().st_size != row['bytes'] or digest(source) != row['sha256']:
            raise ValueError(f'源证据缺失或校验失败：{relative}')
        for part in [target, *target.parents]:
            if part == ROOT:
                break
            if part.is_symlink():
                raise ValueError(f'目标路径包含符号链接：{relative}')
        if target.exists() and (not target.is_file() or digest(target) != row['sha256']):
            raise ValueError(f'目标已存在不同内容，拒绝覆盖：{relative}')
        records.append((row, source, target))
    restored = 0
    if args.restore:
        for row, source, target in records:
            if target.exists():
                continue
            target.parent.mkdir(parents=True, exist_ok=True)
            fd, temporary = tempfile.mkstemp(prefix='.restore-evidence-', dir=target.parent)
            try:
                with os.fdopen(fd, 'wb') as output, source.open('rb') as data:
                    shutil.copyfileobj(data, output)
                    output.flush()
                    os.fchmod(output.fileno(), 0o644)
                    os.fsync(output.fileno())
                if digest(Path(temporary)) != row['sha256']:
                    raise ValueError(f'复制校验失败：{row["path"]}')
                # hard-link原子创建；并发出现目标时拒绝覆盖。
                os.link(temporary, target)
                restored += 1
            finally:
                Path(temporary).unlink(missing_ok=True)
    print(json.dumps({'verified': len(records), 'restored': restored, 'mode': 'restore' if args.restore else 'verify'}, ensure_ascii=False))

if __name__ == '__main__':
    main()
