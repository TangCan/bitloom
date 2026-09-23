#!/usr/bin/env python3
"""验证历史审计在干净历史中可重放，且仍拒绝范围内的越界改动。"""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / '_agile-output/test-artifacts'))
spec = importlib.util.spec_from_file_location('core_runner', ROOT / '_agile-output/test-artifacts/129-3-build-runner.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args], stderr=subprocess.PIPE, text=True).strip()


class HistoricalAudit(unittest.TestCase):
    def test_clean_clone_without_original_objects(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp) / 'clone'
            subprocess.run(['git', 'clone', '--quiet', '--no-local', '--single-branch', str(ROOT), str(repo)], check=True)
            for original in runner.AUDIT_SCOPES[0]:
                self.assertNotEqual(subprocess.run(['git', '-C', str(repo), 'cat-file', '-e', original], capture_output=True).returncode, 0)
            with patch.object(runner, 'ROOT', repo):
                result = runner.audit()
            self.assertEqual(result['status'], 'PASS')
            self.assertEqual([s['diff_sha256'] for s in result['scopes']], [
                '0ec4a697e0d53cfa5321a9e9fb75df1fdb99ee0407e9a465969b75da81db7e58',
                'db289e75cb482a510ce2e9408ecfb746a968c78e80161a8e0fc2de60141c0c9c',
            ])
            # 后续工作树版本改动不应污染明确的历史区间。
            (repo / 'Cargo.toml').write_text('[package]\nversion = "99.0.0"\n')
            with patch.object(runner, 'ROOT', repo):
                self.assertEqual(runner.audit(), result)

    def test_scoped_forbidden_changes_still_fail(self):
        for path, before, after, diagnostic in [
            ('Cargo.toml', 'version = "1.0.0"\n', 'version = "2.0.0"\n', 'package-version'),
            ('crates/example/src/lib.rs', '// 原始\n', 'pub fn added() {}\n', 'public Rust addition'),
            ('docs/public-api-1-0-surface.md', '旧表面\n', '新表面\n', 'public API surface'),
        ]:
            with self.subTest(path=path), tempfile.TemporaryDirectory() as tmp:
                repo = Path(tmp)
                git(repo, 'init', '-q')
                git(repo, 'config', 'user.name', 'Audit test')
                git(repo, 'config', 'user.email', 'audit@example.invalid')
                target = repo / path
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(before)
                git(repo, 'add', '.')
                git(repo, 'commit', '-qm', 'baseline')
                base = git(repo, 'rev-parse', 'HEAD')
                target.write_text(after)
                git(repo, 'commit', '-qam', 'mutation')
                end = git(repo, 'rev-parse', 'HEAD')
                mapping = {}
                for original_base, original_end in runner.AUDIT_SCOPES:
                    mapping[original_base] = base
                    mapping[original_end] = end
                manifest = repo / 'docs/evidence/commit-map.json'
                manifest.parent.mkdir(parents=True, exist_ok=True)
                manifest.write_text(json.dumps(mapping))
                with patch.object(runner, 'ROOT', repo), self.assertRaisesRegex(SystemExit, diagnostic):
                    runner.audit()


if __name__ == '__main__':
    unittest.main()
