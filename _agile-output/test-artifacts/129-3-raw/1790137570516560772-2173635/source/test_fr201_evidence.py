"""Fast negative tests; run with python3 and python3 -O."""
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys
import tempfile
import shutil
import subprocess
import unittest
from fr201_evidence import BACKENDS, TOOLS, SEEDS, FEATURES, validate

spec = importlib.util.spec_from_file_location('runner', Path(__file__).with_name('129-3-build-runner.py'))
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)

class EvidenceTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.value = {'backends': {}, 'direct_csr_backends': {}, 'source_evidence': {},
            'semver': {'harness': 'PASS', 'registry': 'PASS'},
            'missing_tool_negative': {name: {'exit_code': 4, 'diagnostic': f'required tool is missing: {name}'} for name in TOOLS},
            'commands': [], 'compatibility_audit': {'status': 'PASS', 'scopes': [{}, {}]}}
        for section, topology in [('backends', 'axi'), ('direct_csr_backends', 'direct-csr')]:
            for name in BACKENDS:
                key = name if topology == 'axi' else 'direct-csr-' + name
                folder = self.root / key
                folder.mkdir()
                (folder / 'design.v').write_text('module dut; endmodule')
                (folder / 'tb.sv').write_text('independent oracle')
                (folder / 'wave.vcd').write_bytes(b'1' * 1025)
                row = {'topology': topology, 'exit_code': 0, 'transactions': 16000, 'assertions': 20,
                       'vcd_bytes': 1025, 'sha256': hashlib.sha256((folder / 'design.v').read_bytes()).hexdigest(),
                       'synthesis': {'check_assert': True, 'cells': 3}}
                (folder / 'run.log').write_text('PASS')
                row.update(random_seeds=SEEDS, coverage={name: True for name in FEATURES}, source_sha256={'tb.sv': hashlib.sha256((folder/'tb.sv').read_bytes()).hexdigest()}, commands=[{'exit_code': 0, 'timeout_seconds': 1, 'start_utc_unix_ms': 1, 'tool_sha256': 'a'*64, 'log': 'run.log'}], tools={'rustc': {'stdout': 'rustc 1.97.1 (fixture)', 'exit_code': 0, 'sha256': 'a'*64}, 'firtool': {'stdout': 'firtool-1.159.0', 'exit_code': 0, 'sha256': 'b'*64}})
                (folder / 'evidence.json').write_text(json.dumps(row))
                self.value[section][name] = row
                self.value['source_evidence'][key] = str((folder / 'evidence.json').relative_to(self.root))
        formal = {key: {'status': 'PASS', 'exit_code': 0, 'depth': depth} for key, depth in [('prove', 4), ('cover', 2), ('response_reset_bmc', 8), ('request_reset_bmc', 8), ('request_reset_cover', 8)]}
        formal['negative_control'] = {'status': 'EXPECTED_FAIL', 'exit_code': 1, 'counterexample_vcd': True}
        (self.root / 'formal/negative/engine_0').mkdir(parents=True)
        (self.root / 'formal/negative/engine_0/trace.vcd').write_text('counterexample')
        formal.update(source_sha256={'negative/engine_0/trace.vcd': hashlib.sha256(b'counterexample').hexdigest()}, commands=[{'exit_code': 0, 'timeout_seconds': 1, 'start_utc_unix_ms': 1, 'tool_sha256': 'a'*64, 'log': 'run.log'}], tools={'sby': {'exit_code': 0, 'sha256': 'a'*64}})
        (self.root/'formal/run.log').write_text('PASS')
        formal['negative_controls'] = {}
        for name, task in [('irq_wiring', 'negative'), ('request_hold', 'negative_request'), ('reset_cancel', 'negative_reset')]:
            trace = self.root/'formal'/task/'engine_0/trace.vcd'
            trace.parent.mkdir(parents=True, exist_ok=True)
            trace.write_text('counterexample')
            formal['negative_controls'][name] = {'status': 'EXPECTED_FAIL', 'exit_code': 1, 'counterexample_vcd': True, 'trace_sha256': hashlib.sha256(trace.read_bytes()).hexdigest()}
        formal['request_reset_cover']['witness_vcd'] = {}
        for number in range(5):
            name = f'cover{number}.vcd'
            trace = self.root/'formal'/name
            trace.write_text('cover witness')
            formal['request_reset_cover']['witness_vcd'][name] = hashlib.sha256(trace.read_bytes()).hexdigest()
        (self.root / 'formal/evidence.json').write_text(json.dumps(formal))
        self.value['formal'] = formal
        self.value['source_evidence']['formal'] = 'formal/evidence.json'
        self.value['archive_manifest'] = {str(p.relative_to(self.root)): hashlib.sha256(p.read_bytes()).hexdigest() for p in self.root.rglob('*') if p.is_file()}

    def test_valid_partial(self):
        self.assertTrue(validate(self.value, self.root, complete=False))

    def test_malformed_aggregates_fail(self):
        changes = [lambda v: v['backends']['direct'].update(transactions=0),
                   lambda v: v['direct_csr_backends'].pop('chisel'),
                   lambda v: v['formal']['prove'].update(status='UNKNOWN'),
                   lambda v: v['missing_tool_negative']['sby'].update(exit_code=1),
                   lambda v: v.update(archive_manifest={}),
                   lambda v: v['archive_manifest'].pop('formal/evidence.json'),
                   lambda v: v['formal'].pop('request_reset_bmc'),
                   lambda v: v['backends']['firrtl'].update(sha256='0'*64)]
        for change in changes:
            value = copy.deepcopy(self.value)
            change(value)
            with self.assertRaises((ValueError, KeyError)):
                validate(value, self.root, complete=False)

    def test_full_requires_replay(self):
        with self.assertRaises(ValueError):
            validate(self.value, self.root)

    def test_changed_archived_oracle_fails(self):
        (self.root / 'direct/tb.sv').write_text('different oracle')
        with self.assertRaises(ValueError):
            validate(self.value, self.root, complete=False)

    def test_preflight_override_and_individual_missing_tools(self):
        directory = self.root / 'bin'
        directory.mkdir()
        for name in TOOLS:
            (directory / ('z3' if name == 'solver' else name)).symlink_to(sys.executable)
        env = os.environ | {'PATH': str(directory), 'RHDL_FIRTOOL_PATH': str(directory), 'BITLOOM_SBY': str(directory/'sby')}
        paths = runner.preflight(env)
        self.assertEqual(set(paths), set(TOOLS))
        controls = runner.negatives(env, paths)
        self.assertEqual(len(controls), len(TOOLS))
        env['BITLOOM_SBY'] = str(directory/'missing-sby')
        with self.assertRaises(SystemExit) as error:
            runner.preflight(env)
        self.assertEqual(error.exception.code, 4)

    def test_latest_keeps_axi_and_newer_direct_csr_separate(self):
        original_archive, original_root = runner.ARCHIVE, runner.EVIDENCE_ROOT
        runner.ARCHIVE = self.root / 'archive'
        runner.EVIDENCE_ROOT = self.root
        self.addCleanup(setattr, runner, 'ARCHIVE', original_archive)
        self.addCleanup(setattr, runner, 'EVIDENCE_ROOT', original_root)
        artifacts = self.root / 'artifacts'
        for name, topology, timestamp in [('direct-1', 'axi', 100), ('direct-csr-direct-2', 'direct-csr', 200)]:
            folder = artifacts / name
            folder.mkdir(parents=True)
            evidence = folder / 'evidence.json'
            evidence.write_text(json.dumps({'backend': 'direct', 'topology': topology}))
            os.utime(evidence, (timestamp, timestamp))
        _, axi = runner.latest(artifacts, 'direct', 0)
        _, csr = runner.latest(artifacts, 'direct-csr-direct', 0)
        self.assertEqual(axi['topology'], 'axi')
        self.assertEqual(csr['topology'], 'direct-csr')

    def test_deadline_kills_process_group(self):
        original = runner.ARCHIVE
        runner.ARCHIVE = self.root / 'logs'
        self.addCleanup(setattr, runner, 'ARCHIVE', original)
        with self.assertRaises(SystemExit):
            runner.run([sys.executable, '-c', 'import time; time.sleep(60)'], 'timeout', os.environ.copy(), seconds=0.05)
        record = json.loads((runner.ARCHIVE/'timeout.command.json').read_text())
        self.assertEqual(record['exit_code'], 124)

class ReplayTests(unittest.TestCase):
    def test_clean_checkout_custom_destination_and_durable_child(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            area = root / '_agile-output/test-artifacts'
            area.mkdir(parents=True)
            script = Path(__file__).with_name('129-3-isolated-replay.sh')
            shutil.copy2(script, area / script.name)
            (area/'fr201_evidence.py').write_text('def validate(*args, **kwargs): return True\n')
            (area/'129-3-build-runner.py').write_text(
                'import argparse, json, os\nfrom pathlib import Path\n'
                'p=argparse.ArgumentParser(); p.add_argument("--isolated-child"); p.add_argument("--evidence"); a=p.parse_args()\n'
                'if Path(a.isolated_child).read_text().strip() != str(Path.cwd()): raise SystemExit(3)\n'
                'out=Path(a.evidence); out.parent.mkdir(parents=True,exist_ok=True); out.write_text(json.dumps({"child": True}))\n')
            default = area/'129-3-latest-results.json'
            default.write_text('{"untouched": true}')
            for args in [('init', '-q'), ('add', '.'), ('-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', 'commit', '-qm', 'fixture')]:
                subprocess.run(['git', *args], cwd=root, check=True, capture_output=True)
            custom = root/'custom.json'
            custom.write_text('{}')
            result = subprocess.run(['bash', str(area/script.name), str(custom)], cwd=root, capture_output=True, text=True, timeout=30)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            value = json.loads(custom.read_text())
            self.assertEqual(value['isolated_replay']['exit_code'], 0)
            self.assertTrue((root/value['isolated_replay']['evidence']).is_file())
            self.assertEqual(json.loads(default.read_text()), {'untouched': True})
            self.assertEqual(len(subprocess.check_output(['git', 'worktree', 'list'], cwd=root, text=True).splitlines()), 1)

if __name__ == '__main__':
    unittest.main()
