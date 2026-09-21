#!/usr/bin/env python3
"""Archive one complete, source-bound GPIO build run before cargo clean.

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
import re
import tarfile

ROOT = Path(__file__).resolve().parents[2]
_spec = importlib.util.spec_from_file_location('gpio_runner', Path(__file__).with_name('128-4-build-runner.py'))
_runner = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_runner)
GATES = {'sby-runtime', 'atdd', 'formal', 'backends', 'pair-backends', 'firrtl-regression', 'numeric', 'semver',
         'example', 'example-artifacts', 'header-compile', 'header-execute'}

# Frozen original three-seed oracle accounting, including every coverage counter.
DIRECT_ACCOUNTING_SHA256 = {'128419705511': '9fbc415b845db49a98aebd70f897f46f6c9ffbf1ce9ee53029ef7747b9d3364e', 'deadbeef1284': '98f437c9ae855296dc638fbddf4ef70a2d101f7c475c0c1dee8f03919533672c', '83592401ffff': 'fafc668fa72a9b4900302003488069472caa11aa1125a2627058a5c6e90c90d2'}

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
    for label, target in [('backends', 'p1_gpio_firrtl_chisel_same_independent_vectors'),
                          ('pair-backends', 'p1_composition_firrtl_chisel_shared_renamed_actual_gpio_irq')]:
        require(records[label].get('exact_test') == target, f'unchecked exact target: {label}')
        _runner.exact_test_passed((run/f'{label}.log').read_text(), target)
    _runner.atdd_tests_passed((run/'atdd.log').read_text())
    _runner.formal_tests_passed((run/'formal.log').read_text())
    comparisons = json.loads((run/'example-artifacts.json').read_text())
    require({c.get('name') for c in comparisons} == {'gpio-csr-registers.h', 'gpio-csr-registers.md'} and
            all(c.get('equal') is True and c.get('expected_sha256') == c.get('actual_sha256') for c in comparisons),
            'software comparison did not pass')
    for name in ['src/main.rs', 'software/gpio-csr-registers.h', 'software/gpio-csr-registers.md']:
        nonempty(run/'example'/name)
    for comparison in comparisons:
        name = comparison['name']
        golden = (root/'docs/ip'/name).read_bytes()
        generated = (run/'example/software'/name).read_bytes()
        require(generated == golden
                and hashlib.sha256(golden).hexdigest() == comparison['expected_sha256']
                and hashlib.sha256(generated).hexdigest() == comparison['actual_sha256'],
                f'software bytes/hash mismatch: {name}')
    identity = json.loads((run/'sby-identity.json').read_text())
    require(len(identity.get('files', [])) == 16 and identity.get('runtime_command'), 'missing verified SBY runtime')
    pins = dict(line.split('=', 1) for line in
                (root/'scripts/ci-sby-pins.env').read_text().splitlines()
                if line and not line.startswith('#'))
    require(identity.get('commit') == _runner.PIN_COMMIT
            and identity.get('tag_object') == pins['SBY_GIT_SHA'], 'SBY pinned identity mismatch')
    require((run/'sby-runtime.log').read_text().strip() == 'SBY ' + pins['SBY_GIT_REF'],
            'SBY runtime release mismatch')
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
    def command(path, expected=0):
        nonempty(path)
        require(json.loads(path.read_text()).get('exit_code') == expected,
                f'wrong command exit: {path}')
    def passed(path, marker):
        nonempty(path)
        require(marker in path.read_text(), f'missing actual PASS marker: {path}')
    def vcd(path):
        nonempty(path)
        text = path.read_text()
        require('$enddefinitions' in text and '#0' in text, f'invalid VCD: {path}')
    direct = associated('atdd', 'direct-')
    seeds = [path.name.split('-')[1] for path in direct]
    require(len(seeds) == 3 and set(seeds) == set(DIRECT_ACCOUNTING_SHA256),
            'missing or duplicate fixed direct seeds')
    for path, seed in zip(direct, seeds):
        for name in ['design.v', 'tb.sv']:
            nonempty(path/name)
        vcd(path/'trace.vcd')
        accounting = (path/'accounting.log').read_bytes()
        require(hashlib.sha256(accounting).hexdigest() == DIRECT_ACCOUNTING_SHA256[seed],
                f'fixed-seed accounting/coverage mismatch: {seed}')
        counts = re.search(rb'accepted=(\d+) consumed=(\d+) cancelled=(\d+)', accounting)
        require(counts is not None and int(counts[1]) == int(counts[2])+int(counts[3]),
                'direct response ledger mismatch')
        command(path/'compile.log.json')
        command(path/'run.log.json')
        passed(path/'run.log', 'FR197 GPIO PASS')
    for path in associated('atdd', 'scoreboard-mutation-'):
        for name in ['control', 'input-is-output']:
            case = path/name
            nonempty(case/'design.v'); nonempty(case/'tb.sv')
            command(case/'compile.log.json')
            command(case/'run-command.json', 0 if name == 'control' else 1)
            vcd(case/'trace.vcd')
            output = (case/'run.log').read_text()
            if name == 'control':
                require('FR197 GPIO PASS' in output, 'scoreboard control did not pass')
            else:
                require('cycle=4 after rdata expected=80000000 got=00000000' in output
                        and 'FR197 GPIO PASS' not in output, 'wrong scoreboard behavioral failure')
    for graph in ['pair-true-false-', 'pair-false-false-', 'gpio-irq-false-']:
        for path in associated('atdd', graph):
            for name in ['dut.v', 'tb.sv']:
                nonempty(path/name)
            vcd(path/'trace.vcd')
            command(path/'compile.json'); command(path/'simulate.json')
            passed(path/'simulate.log', 'PASS GPIO composition')
    for label, prefixes in [('backends', ['backends-']), ('pair-backends', ['pair-true-true-', 'pair-false-true-', 'gpio-irq-true-'])]:
        for prefix in prefixes:
            for path in associated(label, prefix):
                for name in ['design.fir', 'src/main/scala/Design.scala']:
                    nonempty(path/name)
                vcd(path/'firrtl.vcd'); vcd(path/'chisel/trace.vcd')
                for name in ['firrtl-lower.json', 'firrtl-compile.json', 'firrtl-simulate.json',
                             'sbt.json', 'chisel/compile.json', 'chisel/simulate.json']:
                    command(path/name)
                marker = 'FR197 GPIO PASS' if label == 'backends' else 'PASS GPIO composition'
                passed(path/'firrtl-simulate.log', marker)
                passed(path/'chisel/simulate.log', marker)
    def status(path, expected):
        nonempty(path)
        require(path.read_text().split()[0] == expected, f'expected {expected}: {path}')
    for path in associated('formal', 'proof-'):
        nonempty(path/'formal.v')
        status(path/'gpio_prove/status', 'PASS')
        status(path/'gpio_cover/status', 'PASS')
        for name in ['logfile_basecase.txt', 'logfile_induction.txt']:
            proof_log = path/'gpio_prove/engine_0'/name
            nonempty(proof_log)
            require('Status: passed' in proof_log.read_text(), f'proof log did not pass: {proof_log}')
        cover_log = path/'gpio_cover/engine_0/logfile.txt'
        nonempty(cover_log)
        pending, witnessed = set(), set()
        for line in cover_log.read_text().splitlines():
            match = re.search(r'Reached cover statement at (.+) in step \d+\.', line)
            if match:
                pending.add(match[1])
            trace = re.search(r'Writing trace to VCD file: (engine_0/trace\d+\.vcd)$', line)
            if trace:
                witness = path/'gpio_cover'/trace[1]
                nonempty(witness)
                require('$enddefinitions' in witness.read_text() and '#0' in witness.read_text(),
                        f'invalid cover witness: {witness}')
                witnessed.update(pending)
                pending.clear()
        require(len(witnessed) == 10 and not pending and 'Status: passed' in cover_log.read_text(),
                'missing ten witnessed cover statements')
    for path in associated('formal', 'mutations-'):
        status(path/'control/status', 'PASS')
        command(path/'control-command.json')
        for name in ['logfile_basecase.txt', 'logfile_induction.txt']:
            passed(path/'control/engine_0'/name, 'Status: passed')
        for fault, property_name in [('event-clear-priority', 'gpio_events'), ('input-is-output', 'gpio_input'),
                                     ('set-ignores-strobes', 'gpio_out')]:
            status(path/fault/'status', 'FAIL')
            command(path/f'{fault}-command.json', 2)
            log = (path/fault/'engine_0/logfile_basecase.txt').read_text()
            require(any('Assert failed' in line and line.rstrip().endswith(': '+property_name)
                        for line in log.splitlines()), f'wrong formal failure property: {fault}')
            vcd(path/fault/'engine_0/trace.vcd')
    for path in associated('formal', 'synthesis-'):
        nonempty(path/'design.v')
        net = json.loads((path/'synthesis.json').read_text())
        require(bool(net.get('modules', {}).get('GpioCsr', {}).get('cells')), 'missing original synthesis netlist')
    return roots


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--run', required=True, type=Path)
    parser.add_argument('--include-history', action='store_true')
    parser.add_argument('archive', nargs='?', type=Path)
    args = parser.parse_args()
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%fZ')
    archive = args.archive or ROOT/'_agile-output/test-artifacts'/f'128-4-build-raw-{stamp}.tar.gz'
    manifest_path = archive.with_suffix('').with_suffix('.manifest.json')
    for path in [archive, manifest_path]:
        require(not path.exists() and not path.is_symlink(), f'refusing to overwrite {path}')
    run = args.run.resolve()
    roots = validate_run(run, ROOT)
    roots['run'] = run
    if args.include_history:
        for name in ['fr197-gpio', 'fr197-gpio-api', 'fr197-gpio-formal', 'fr197-gpio-reruns', 'fr197-gpio-bootstrap']:
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
