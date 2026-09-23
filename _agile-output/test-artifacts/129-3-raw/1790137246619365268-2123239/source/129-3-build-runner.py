#!/usr/bin/env python3
"""Execute, archive and independently validate the FR201 core matrix."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import tempfile
import time
from fr201_evidence import BACKENDS, TOOLS, validate

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_EVIDENCE = ROOT / '_agile-output/test-artifacts/129-3-latest-results.json'
EVIDENCE_ROOT = Path(os.environ.get('BITLOOM_FR201_EVIDENCE_ROOT', ROOT)).resolve()
ARCHIVE = EVIDENCE_ROOT / '_agile-output/test-artifacts/129-3-raw' / f'{time.time_ns()}-{os.getpid()}'

def executable(name, env):
    if name == 'firtool':
        candidate = str(Path(env.get('RHDL_FIRTOOL_PATH', '/nonexistent')) / 'firtool')
    elif name == 'sby' and env.get('BITLOOM_SBY'):
        candidate = env['BITLOOM_SBY']
    else:
        candidate = 'z3' if name == 'solver' else name
    return shutil.which(candidate, path=env.get('PATH'))

def preflight(env):
    paths = {}
    for tool in TOOLS:
        path = executable(tool, env)
        if not path:
            print(f'required tool is missing: {tool}', file=sys.stderr)
            raise SystemExit(4)
        paths[tool] = str(Path(path).resolve())
    return paths

def run(command, label, env, seconds=1200):
    ARCHIVE.mkdir(parents=True, exist_ok=True)
    log = ARCHIVE / f'{label}.log'
    started = time.time_ns() // 1_000_000
    with log.open('wb') as stream:
        process = subprocess.Popen(command, cwd=ROOT, env=env, stdout=stream, stderr=subprocess.STDOUT, start_new_session=True)
        previous_term = signal.getsignal(signal.SIGTERM)
        def terminate(signum, frame):
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            process.wait()
            raise SystemExit(128 + signum)
        signal.signal(signal.SIGTERM, terminate)
        try:
            code = process.wait(timeout=seconds)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                pass
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            process.wait()
            code = 124
        finally:
            signal.signal(signal.SIGTERM, previous_term)
    row = dict(command=command, exit_code=code, start_utc_unix_ms=started,
               end_utc_unix_ms=time.time_ns() // 1_000_000, timeout_seconds=seconds,
               log=os.path.relpath(log, EVIDENCE_ROOT))
    (ARCHIVE / f'{label}.command.json').write_text(json.dumps(row, indent=2))
    if code:
        raise SystemExit(f'{label} failed ({code}); see {log}')
    return row

def latest(root, prefix, since):
    candidates = []
    for path in root.glob(f'{prefix}-*/evidence.json'):
        if path.stat().st_mtime_ns // 1_000_000 >= since:
            row = json.loads(path.read_text())
            if row.get('backend') == prefix or prefix == 'run' or (prefix.startswith('direct-csr-') and row.get('topology') == 'direct-csr' and row.get('backend') == prefix.removeprefix('direct-csr-')):
                candidates.append(path)
    if not candidates:
        raise SystemExit(f'missing fresh {prefix} evidence below {root}')
    path = max(candidates, key=lambda p: p.stat().st_mtime_ns)
    if not path.resolve().is_relative_to(root.resolve()):
        raise SystemExit('evidence escaped artifact root')
    destination = ARCHIVE / prefix
    shutil.copytree(path.parent, destination, ignore=shutil.ignore_patterns('target', '.bloop', '.bsp', 'simulation', '*.o', '*.class', '*.jar'))
    saved = destination / 'evidence.json'
    return os.path.relpath(saved, EVIDENCE_ROOT), json.loads(saved.read_text())

def negatives(env, paths):
    result = {}
    for missing in TOOLS:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            for name, path in paths.items():
                if name != missing:
                    (directory / ('z3' if name == 'solver' else name)).symlink_to(path)
            test_env = env | {'PATH': temporary, 'RHDL_FIRTOOL_PATH': temporary, 'BITLOOM_SBY': str(directory / 'sby')}
            process = subprocess.run([sys.executable, __file__, '--preflight-only'], env=test_env, cwd=ROOT, capture_output=True, text=True, timeout=30)
            expected = f'required tool is missing: {missing}'
            if process.returncode != 4 or process.stderr.strip() != expected:
                raise SystemExit(f'missing-tool control failed: {missing}: {process.returncode}: {process.stderr}')
            result[missing] = {'exit_code': process.returncode, 'diagnostic': process.stderr.strip()}
    return result

def audit():
    scopes = []
    for base, end in [('218f0620e85ab438a3ef1351e253277552c8c410', 'baf07e739af2b36f4e0befddda9826964d16118e'), ('4bdd680', None)]:
        args = ['git', 'diff', base] + ([end] if end else [])
        diff = subprocess.check_output(args + ['--', 'Cargo.toml', 'crates', 'docs/public-api-1-0-surface.md'], cwd=ROOT, text=True)
        names = subprocess.check_output(args + ['--name-only'], cwd=ROOT, text=True).splitlines()
        current = ''
        for line in diff.splitlines():
            if line.startswith('+++ b/'):
                current = line[6:]
            if line.startswith(('+', '-')) and not line.startswith(('+++', '---')):
                if current.endswith('Cargo.toml') and re.match(r'[+-]\s*version\s*=', line):
                    raise SystemExit('scoped package-version change: ' + current)
                if '/src/' in current and re.match(r'\+\s*pub(?:\([^)]*\))?\s+', line):
                    raise SystemExit('public Rust addition requires explicit scoped review: ' + current)
                if current == 'docs/public-api-1-0-surface.md':
                    raise SystemExit('public API surface changed in reviewed scope')
        base_sha = subprocess.check_output(['git', 'rev-parse', base], cwd=ROOT, text=True).strip()
        scopes.append({'base': base_sha, 'end': end or 'working-tree', 'changed_paths': names, 'diff_sha256': hashlib.sha256(diff.encode()).hexdigest()})
    return {'status': 'PASS', 'method': 'scoped source/public-surface and package-version diff audit; 130.2/130.3 excluded', 'scopes': scopes}

def manifest():
    return {os.path.relpath(p, EVIDENCE_ROOT): hashlib.sha256(p.read_bytes()).hexdigest() for p in ARCHIVE.rglob('*') if p.is_file()}

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--preflight-only', action='store_true')
    parser.add_argument('--skip-isolated', action='store_true')
    parser.add_argument('--isolated-child', type=Path)
    parser.add_argument('--evidence', type=Path, default=DEFAULT_EVIDENCE)
    args = parser.parse_args()
    env = os.environ.copy()
    paths = preflight(env)
    if args.preflight_only:
        return 0
    child = args.isolated_child is not None
    if child and (not args.isolated_child.is_file() or args.isolated_child.read_text().strip() != str(ROOT) or not env.get('CARGO_TARGET_DIR') or Path(env['CARGO_TARGET_DIR']).resolve().is_relative_to(EVIDENCE_ROOT / 'target')):
        raise SystemExit('invalid isolated child invocation')
    ARCHIVE.mkdir(parents=True, exist_ok=True)
    if DEFAULT_EVIDENCE.exists():
        historical = EVIDENCE_ROOT / '_agile-output/test-artifacts/129-3-historical-results.json'
        if not historical.exists():
            shutil.copy2(DEFAULT_EVIDENCE, historical)
    env.pop('BITLOOM_FR201_BACKENDS', None)
    env.update(CARGO_PROFILE_TEST_OPT_LEVEL='1', YOSYS_HISTORY_FILE='/dev/null', BITLOOM_SBY=paths['sby'])
    commands = []
    for label, test in [('backend-matrix', 'fr201_backend_matrix::fr201_axi_complete_system_three_backend_matrix'), ('direct-csr-matrix', 'fr201_backend_matrix::fr201_direct_csr_complete_system_three_backend_matrix'), ('formal', 'fr201_core_formal::fr201_complete_system_formal_and_negative_control')]:
        commands.append(run(['cargo', 'test', '-p', 'bitloom', '--test', 'fr198_peripheral_system', test, '--', '--ignored', '--exact', '--nocapture'], label, env))
    for label, script in [('semver-harness', 'scripts/test-semver-check.sh'), ('semver-registry', 'scripts/semver-check.sh')]:
        commands.append(run(['bash', script], label, env))
    for label, flags in [('validator-tests', []), ('validator-tests-optimized', ['-O'])]:
        commands.append(run([sys.executable, *flags, str(ROOT / '_agile-output/test-artifacts/test_fr201_evidence.py')], label, env, seconds=60))
    backend_root = Path(env.get('BITLOOM_FR201_ARTIFACT_ROOT', ROOT / 'target/fr201-core'))
    formal_root = Path(env.get('BITLOOM_FR201_FORMAL_ROOT', ROOT / 'target/fr201-core-formal'))
    sources, backends, csr = {}, {}, {}
    for section, prefix, index in [(backends, '', 0), (csr, 'direct-csr-', 1)]:
        for backend in BACKENDS:
            key = prefix + backend
            sources[key], section[backend] = latest(backend_root, key, commands[index]['start_utc_unix_ms'])
    sources['formal'], formal = latest(formal_root, 'run', commands[2]['start_utc_unix_ms'])
    source_dir = ARCHIVE / 'source'
    source_dir.mkdir()
    (source_dir / 'git-head.txt').write_bytes(subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT))
    (source_dir / 'git-working-tree.patch').write_bytes(subprocess.check_output(['git', 'diff', '--binary', 'HEAD'], cwd=ROOT))
    run(['rustc', '--version'], 'rustc-identity', env, seconds=30)
    if not (ARCHIVE / 'rustc-identity.log').read_text().startswith('rustc 1.97.1 '):
        raise SystemExit('required Rust version is 1.97.1')
    for path in [ROOT / 'Cargo.lock', ROOT / 'rust-toolchain.toml', *ROOT.glob('crates/bitloom/tests/fr198_peripheral_system*'), *ROOT.glob('_agile-output/test-artifacts/*129-3*.py'), ROOT / '_agile-output/test-artifacts/fr201_evidence.py']:
        if path.is_dir():
            shutil.copytree(path, source_dir / path.name)
        else:
            shutil.copy2(path, source_dir / path.name)
    result = {'story': '129.3', 'schema': 2, 'status': 'PARTIAL', 'backends': backends, 'direct_csr_backends': csr,
              'formal': formal, 'synthesis': {name: row['synthesis'] for name, row in backends.items()},
              'semver': {'harness': 'PASS', 'registry': 'PASS', 'packages': ['bitloom-prelude', 'bitloom-sim', 'bitloom-firrtl']},
              'compatibility_audit': audit(), 'missing_tool_negative': negatives(env, paths),
              'tool_identity': {name: {'path': path, 'sha256': hashlib.sha256(Path(path).read_bytes()).hexdigest()} for name, path in paths.items()},
              'commands': commands, 'source_evidence': sources, 'isolated_replay': {'exit_code': None}, 'archive_manifest': manifest()}
    validate(result, EVIDENCE_ROOT, complete=False)
    args.evidence = args.evidence.resolve()
    args.evidence.parent.mkdir(parents=True, exist_ok=True)
    args.evidence.write_text(json.dumps(result, indent=2) + '\n')
    if not child and not args.skip_isolated:
        replay_command = run(['bash', str(ROOT / '_agile-output/test-artifacts/129-3-isolated-replay.sh'), str(args.evidence)], 'isolated-replay', env, seconds=2400)
        result = json.loads(args.evidence.read_text())
        result['commands'].append(replay_command)
        result['archive_manifest'] = manifest()
        validate(result, EVIDENCE_ROOT, complete=True)
        result['status'] = 'PASS'
        args.evidence.write_text(json.dumps(result, indent=2) + '\n')
    print(f'Story129.3 core runner {result["status"]} evidence={args.evidence}')
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
