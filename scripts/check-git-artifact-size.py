#!/usr/bin/env python3
"""检查暂存文件或指定基线后的历史，拒绝大型blob与原始验收归档。"""
import argparse
import json
import re
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parent.parent

def git(*args, input=None):
    return subprocess.check_output(['git', *args], cwd=ROOT, input=input)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument('--base', help='检查该提交到HEAD的全部新增历史对象')
    mode.add_argument('--staged', action='store_true', help='检查暂存文件（默认）')
    args = parser.parse_args()
    records = []
    if args.base:
        base = git('rev-parse', '--verify', args.base + '^{commit}').decode().strip()
        for line in git('rev-list', '--objects', base + '..HEAD').splitlines():
            oid, _, path = line.partition(b' ')
            records.append((oid.decode(), path.decode(errors='replace')))
    else:
        changed = set(git('diff', '--cached', '--name-only', '--diff-filter=ACMR', '-z').split(b'\0'))
        for entry in git('ls-files', '--stage', '-z').split(b'\0'):
            if not entry:
                continue
            info, path = entry.split(b'\t', 1)
            _, oid, stage = info.split()
            if stage != b'0':
                raise ValueError('暂存区存在未解决冲突')
            if path in changed:
                records.append((oid.decode(), path.decode(errors='replace')))
    if not records:
        print(json.dumps({'passed': True, 'checked_blobs': 0}))
        return
    info = git('cat-file', '--batch-check=%(objecttype) %(objectsize)', input=('\n'.join(r[0] for r in records) + '\n').encode()).decode().splitlines()
    failures = []
    count = 0
    for (oid, path), line in zip(records, info):
        kind, size = line.split()
        if kind != 'blob':
            continue
        count += 1
        size = int(size)
        raw = path.startswith('_agile-output/test-artifacts/') and bool(re.search(r'(\.tar\.gz(?:\.part\d+)?|\.tgz|\.zip|\.tar\.zst)$', path))
        if size > 5 * 1024 * 1024 or raw:
            failures.append({'path': path, 'bytes': size, 'oid': oid, 'reason': 'raw evidence archive' if raw else 'blob exceeds 5 MiB'})
    print(json.dumps({'passed': not failures, 'checked_blobs': count, 'failures': failures}, ensure_ascii=False, indent=2))
    if failures:
        raise SystemExit(1)

if __name__ == '__main__':
    main()
