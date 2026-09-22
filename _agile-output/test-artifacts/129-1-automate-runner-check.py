#!/usr/bin/env python3
"""P1 runner subprocess failure contracts, never a product/backend PASS.

Run unchanged runner copies in isolated roots with actual failing executables.
Only timeout budgets are capped by the harness; Popen, wait, communicate and
killpg still operate on real OS processes. No clean/build/test is executed.
"""
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tarfile
import tempfile
import time

OUT = Path(__file__).resolve().parent
RUNNERS = ['129-1-final-regression.py', '129-1-root-foundation.py']
HARNESS = '''import runpy, subprocess, sys
original = subprocess.Popen
class BoundedPopen(original):
    def wait(self, timeout=None):
        return super().wait(timeout=min(timeout, 1.0) if timeout is not None else None)
    def communicate(self, input=None, timeout=None):
        return super().communicate(input=input, timeout=min(timeout, 1.0) if timeout is not None else None)
subprocess.Popen = BoundedPopen
sys.argv = sys.argv[1:]
runpy.run_path(sys.argv[0], run_name='__main__')
'''
FAIL = "import sys\nprint('runner-real-stdout', flush=True)\nprint('runner-real-stderr', file=sys.stderr, flush=True)\nsys.exit(23)\n"
TIMEOUT = '''import os, pathlib, subprocess, sys, time
child = subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(90)'])
pathlib.Path(os.environ['PID_RECORD']).write_text(str(os.getpid())+' '+str(child.pid))
print('runner-real-stdout', flush=True)
print('runner-real-stderr', file=sys.stderr, flush=True)
time.sleep(90)
'''


def require(value, message):
    if not value:
        raise AssertionError(message)


def live(pid):
    stat = Path(f'/proc/{pid}/stat')
    return stat.exists() and stat.read_text().split(') ', 1)[1].split()[0] != 'Z'


def check(runner, mode):
    with tempfile.TemporaryDirectory(prefix='bitloom-runner-contract-') as temporary:
        root = Path(temporary)
        out = root / '_agile-output/test-artifacts'
        out.mkdir(parents=True)
        for name in [runner, '128-5-build-runner.py']:
            (out / name).write_bytes((OUT / name).read_bytes())
        binaries = root / 'bin'
        binaries.mkdir()
        executable = binaries / ('cargo' if 'regression' in runner else 'rustc')
        if mode != 'missing':
            executable.write_text(f'#!{sys.executable}\n' + (TIMEOUT if mode == 'timeout' else FAIL))
            executable.chmod(0o755)
        env = dict(os.environ, PATH=str(binaries), PYTHONDONTWRITEBYTECODE='1',
                   BITLOOM_FOUNDATION_OUTPUT=str(root / 'foundation-attempt'),
                   BITLOOM_BFM_PYTHON=sys.executable, RHDL_FIRTOOL_PATH=str(binaries),
                   PID_RECORD=str(root / 'pids'))
        command = [sys.executable, '-B', '-c', HARNESS, str(out / runner)]
        if 'regression' in runner:
            command += ['--attempt', 'automate-boundary']
        started = time.monotonic()
        try:
            result = subprocess.run(command, env=env, capture_output=True, timeout=15)
            require(result.returncode != 0, 'failed command must propagate nonzero exit')
            reports = list(out.glob('*regression.json')) if 'regression' in runner else list(out.glob('*-results.json'))
            require(len(reports) == 1, 'exactly one failure report must survive')
            report = json.loads(reports[0].read_text())
            require(report['status'] == 'failed', 'report must reject pass')
            require(len(report['commands']) == 1, 'runner must stop after first failing command')
            record = report['commands'][0]
            require(report['source_start'] == report['source_end'], 'isolated runner must remain unchanged')
            if mode == 'missing':
                require('FileNotFoundError' in (report.get('error', '') + result.stderr.decode()), 'missing executable must be explicit')
            else:
                require(record['exit_code'] == (-signal.SIGKILL if mode == 'timeout' else 23), 'preserve actual exit code')
                logs = ''.join(p.read_text() for p in out.glob('*.log')) if 'regression' in runner else ''.join(p.read_text() for p in (root / 'foundation-attempt').glob('*.std*'))
                require('runner-real-stdout' in logs and 'runner-real-stderr' in logs, 'stdout/stderr failure evidence missing')
            if mode == 'timeout':
                require(record.get('timed_out') is True, 'timeout flag missing')
                pids = [int(p) for p in (root / 'pids').read_text().split()]
                # A reparented child can briefly be runnable before SIGKILL delivery.
                deadline = time.monotonic() + 2
                while any(live(pid) for pid in pids) and time.monotonic() < deadline:
                    time.sleep(0.01)
                require(not any(live(pid) for pid in pids), 'timeout left a live descendant')
            if 'foundation' in runner:
                archive = out / Path(report['archive']['path']).name
                require(hashlib.sha256(archive.read_bytes()).hexdigest() == report['archive']['sha256'], 'failure archive hash mismatch')
                with tarfile.open(archive) as saved:
                    archived = json.load(saved.extractfile('run/results.json'))
                    require(archived['status'] == 'failed', 'archive omitted failed status')
                    require(archived['commands'] == report['commands'], 'archive lost command evidence')
            else:
                before = {p.name: p.read_bytes() for p in out.iterdir() if p.is_file()}
                retry = subprocess.run(command, env=env, capture_output=True, timeout=15)
                require(retry.returncode != 0 and b'refusing to overwrite' in retry.stderr, 'same attempt must be rejected')
                require(before == {p.name: p.read_bytes() for p in out.iterdir() if p.is_file()}, 'retry changed original failure evidence')
            return {'runner': runner, 'scenario': mode, 'priority': 'P1', 'status': 'pass',
                    'seconds': round(time.monotonic() - started, 3), 'actual_runner_exit': result.returncode,
                    'retained_report': report, 'stderr': result.stderr.decode()}
        finally:
            if (root / 'pids').exists():
                for pid in map(int, (root / 'pids').read_text().split()):
                    if live(pid):
                        os.kill(pid, signal.SIGKILL)


def main():
    results = [check(runner, mode) for runner in RUNNERS for mode in ['nonzero', 'missing', 'timeout']]
    print(json.dumps({'status': 'pass', 'scope': 'runner failure boundaries only; no product acceptance',
                      'runner_sha256': {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in RUNNERS},
                      'test_count': len(results), 'results': results}, indent=2))


if __name__ == '__main__':
    main()
