#!/usr/bin/env python3
"""Archive one complete, source-bound IRQ build run before cargo clean.

Required: --run /path/from/runner. Optional positional destination .tar.gz.
--include-history also preserves old failure evidence, which never satisfies the
selected run's acceptance checks. Custom runner output paths are supported.
Neither an existing archive nor its companion manifest is overwritten.
"""
from pathlib import Path
import argparse
import datetime
import hashlib
import importlib.util
import json
import tarfile

ROOT = Path(__file__).resolve().parents[2]
_spec = importlib.util.spec_from_file_location('irq_runner', Path(__file__).with_name('128-3-build-runner.py'))
_runner = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_runner)
GATES = {'atdd', 'formal', 'backends', 'pair-backends', 'firrtl-regression', 'numeric', 'semver',
         'example', 'example-artifacts', 'header-compile', 'header-execute'}


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def nonempty(path):
    require(path.is_file() and path.stat().st_size > 0, f'missing/empty evidence: {path}')


def validate_run(run, root):
    """Only this run's successful gates and associated raw artifacts certify it."""
    nonempty(run/'commands.json')
    commands = json.loads((run/'commands.json').read_text())
    start = json.loads((run/'source-fingerprint-start.json').read_text())
    end = json.loads((run/'source-fingerprint-end.json').read_text())
    require(commands.get('source_fingerprint_complete') is True and bool(start) and start == end,
            'incomplete or changed run source fingerprint')
    require(start == _runner.source_fingerprint(root), 'selected run does not match current related source')
    records = {}
    for record in commands.get('commands', []):
        label = record.get('label')
        if label in GATES:
            require(label not in records, f'duplicate selected-run gate: {label}')
            records[label] = record
    require(set(records) == GATES, f'missing selected-run gates: {sorted(GATES-set(records))}')
    for label, record in records.items():
        require(record.get('exit') == 0 and record.get('status') == 'pass', f'unsuccessful selected-run gate: {label}')
    for label, target in [('backends', 'p1_irq_firrtl_chisel_same_independent_vectors'),
                          ('pair-backends', 'p1_composition_firrtl_chisel_shared_renamed_and_actual_timer')]:
        require(records[label].get('exact_test') == target, f'unchecked exact target: {label}')
        _runner.exact_test_passed((run/f'{label}.log').read_text(), target)
    comparisons = json.loads((run/'example-artifacts.json').read_text())
    require({c.get('name') for c in comparisons} == {'irq-registers.h', 'irq-registers.md'} and
            all(c.get('equal') is True and c.get('expected_sha256') == c.get('actual_sha256') for c in comparisons),
            'software comparison did not pass')
    for name in ['src/main.rs', 'software/irq-registers.h', 'software/irq-registers.md']:
        nonempty(run/'example'/name)
    identity = json.loads((run/'sby-identity.json').read_text())
    require(len(identity.get('files', [])) == 16 and identity.get('runtime_command'), 'missing verified SBY runtime')
    roots = {}
    for label, record in records.items():
        for index, name in enumerate(record.get('artifact_roots', [])):
            path = Path(name)
            require(path.is_absolute() and path.is_dir(), f'missing associated artifact directory: {path}')
            roots[f'artifacts/{label}/{index}-{path.name}'] = path
    def associated(label, prefix):
        paths = [path for alias, path in roots.items() if alias.startswith(f'artifacts/{label}/') and path.name.startswith(prefix)]
        require(bool(paths), f'missing {label} artifact class: {prefix}')
        return paths
    for path in associated('atdd', 'direct-'):
        for name in ['design.v', 'tb.sv', 'trace.vcd', 'run.log']:
            nonempty(path/name)
    for graph in ['pair-true-false-', 'pair-false-false-', 'timer-irq-false-']:
        for path in associated('atdd', graph):
            for name in ['dut.v', 'tb.sv', 'trace.vcd']:
                nonempty(path/name)
    for label, prefixes in [('backends', ['backends-']), ('pair-backends', ['pair-true-true-', 'pair-false-true-', 'timer-irq-true-'])]:
        for prefix in prefixes:
            for path in associated(label, prefix):
                for name in ['design.fir', 'firrtl.vcd', 'src/main/scala/Design.scala', 'chisel/trace.vcd']:
                    nonempty(path/name)
    def status(path, expected):
        nonempty(path)
        require(path.read_text().split()[0] == expected, f'expected {expected}: {path}')
    for path in associated('formal', 'proof-'):
        nonempty(path/'formal.v')
        status(path/'irq_prove/status', 'PASS')
        status(path/'irq_cover/status', 'PASS')
    for path in associated('formal', 'mutations-'):
        status(path/'control/status', 'PASS')
        for fault in ['event-clear-priority', 'test-without-commit', 'raw-includes-software']:
            status(path/fault/'status', 'FAIL')
            nonempty(path/fault/'engine_0/trace.vcd')
    for path in associated('formal', 'synthesis-'):
        nonempty(path/'design.v')
        net = json.loads((path/'synthesis.json').read_text())
        require(bool(net.get('modules', {}).get('Irq', {}).get('cells')), 'missing original synthesis netlist')
    return roots


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--run', required=True, type=Path)
    parser.add_argument('--include-history', action='store_true')
    parser.add_argument('archive', nargs='?', type=Path)
    args = parser.parse_args()
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%fZ')
    archive = args.archive or ROOT/'_agile-output/test-artifacts'/f'128-3-build-raw-{stamp}.tar.gz'
    manifest_path = archive.with_suffix('').with_suffix('.manifest.json')
    for path in [archive, manifest_path]:
        require(not path.exists() and not path.is_symlink(), f'refusing to overwrite {path}')
    run = args.run.resolve()
    roots = validate_run(run, ROOT)
    roots['run'] = run
    if args.include_history:
        for name in ['fr197-irq', 'fr197-irq-api', 'fr197-irq-formal', 'fr197-irq-reruns', 'fr197-irq-bootstrap']:
            roots[f'history/{name}'] = ROOT/'target'/name
    files = {}
    excluded = []
    for alias, directory in sorted(roots.items()):
        for path in sorted(directory.rglob('*')):
            if not path.is_file():
                continue
            rel = path.relative_to(directory)
            name = f'{alias}/{rel.as_posix()}'
            if 'target' in rel.parts or path.name in ['simulation', 'sim', 'firrtl-sim', 'run', 'header-check']:
                excluded.append(name)
                continue
            require(name not in files, f'duplicate archive member: {name}')
            files[name] = path
    require(bool(files), 'empty selected-run evidence')
    members = {name: hashlib.sha256(path.read_bytes()).hexdigest() for name, path in files.items()}
    with tarfile.open(archive, 'x:gz', dereference=True) as output:
        for name, path in sorted(files.items()):
            output.add(path, arcname=name, recursive=False)
    with tarfile.open(archive, 'r:gz') as source:
        require(sorted(source.getnames()) == sorted(members), 'archive member mismatch')
        for member in source:
            require(hashlib.sha256(source.extractfile(member).read()).hexdigest() == members[member.name],
                    f'archive byte mismatch: {member.name}')
    manifest = dict(created_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                    selected_run=str(run), source_roots={alias: str(path) for alias, path in roots.items()},
                    archive=archive.name, archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),
                    verified_members=len(members), members=members, excluded=excluded)
    with manifest_path.open('x') as file:
        file.write(json.dumps(manifest, indent=2))
    print(json.dumps({k:v for k,v in manifest.items() if k not in ['members', 'excluded', 'source_roots']}, indent=2))


if __name__ == '__main__':
    main()
