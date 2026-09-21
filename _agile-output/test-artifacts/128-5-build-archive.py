#!/usr/bin/env python3
"""Archive one complete, source-bound UART build run before cargo clean.

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
_spec = importlib.util.spec_from_file_location('uart_runner', Path(__file__).with_name('128-5-build-runner.py'))
_runner = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_runner)
GATES = {'sby-runtime', 'atdd', 'formal', 'backends', 'pair-backends', 'firrtl-regression', 'numeric', 'semver',
         'example', 'example-artifacts', 'header-compile', 'header-execute', 'loopback-backends', 'edge-backends', 'legacy'}

# Frozen original three-seed oracle accounting, including every coverage counter.
ACCOUNTING_SHA256 = 'eaf20136a7e5667d834d4529f663ca5c472c3e8351d160a66084e03b467ed114'

EDGE_COUNTS = {'disabled_queue_access': 1, 'each_event_same_other_clear': 8, 'enabled_merged_div_legality': 6, 'final_busy_edge_reject': 2, 'nonzero_partial_div_merge': 16, 'reset_beats_stop_and_software': 1, 'reset_both_full_and_response': 1, 'reset_partial_synchronizer': 2, 'rx_only_or_dual_busy_configuration': 2, 'stalled_response_serial_and_inactive_noise': 1, 'tx_idle_configuration_collision': 5, 'tx_nonfull_simultaneous_push_pop': 1}

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
        if label != 'example-artifacts':
            require((run/f'{label}.log').is_file(), f'missing original gate log: {label}')
    for label, target in [('backends', 'p1_uart_firrtl_chisel_same_independent_serial_vectors'),
                          ('pair-backends', 'p1_uart_pair_irq_firrtl_chisel'), ('loopback-backends', 'p1_uart_actual_loopback_backends'), ('edge-backends', 'p1_uart_contract_edges_backends')]:
        require(records[label].get('exact_test') == target, f'unchecked exact target: {label}')
        _runner.exact_test_passed((run/f'{label}.log').read_text(), target)
    _runner.atdd_tests_passed((run/'atdd.log').read_text())
    _runner.formal_tests_passed((run/'formal.log').read_text())
    comparisons = json.loads((run/'example-artifacts.json').read_text())
    require({c.get('name') for c in comparisons} == {'uart-csr-registers.h', 'uart-csr-registers.md'} and
            all(c.get('equal') is True and c.get('expected_sha256') == c.get('actual_sha256') for c in comparisons),
            'software comparison did not pass')
    for name in ['src/main.rs', 'software/uart-csr-registers.h', 'software/uart-csr-registers.md']:
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
    def exactly(label, prefix):
        paths = associated(label, prefix)
        require(len(paths) == 1, f'duplicate class: {label}/{prefix}')
        return paths[0]
    def direct(path):
        for name in ['design.v', 'tb.sv']: nonempty(path/name)
        command(path/'compile.json'); command(path/'simulate.json')
        passed(path/'simulate.log', 'FR197 UART PASS'); vcd(path/'trace.vcd')
    def accounting(path):
        data = (path/'accounting.log').read_bytes()
        require(hashlib.sha256(data).hexdigest() == ACCOUNTING_SHA256,
                'fixed three seeds, directed coverage or cancellation ledger changed')
        require(len(re.findall(rb'^seed=', data, re.M)) == 3 and
                len(re.findall(rb'^active-ledger seed=', data, re.M)) == 3,
                'missing fixed-seed or active-frame ledger')
        for accepted, consumed, cancelled in re.findall(rb'accepted=(\d+) consumed=(\d+) cancelled=(\d+)', data):
            require(int(accepted) == int(consumed)+int(cancelled), 'response cancellation mismatch')
        for start, complete, cancelled, active in re.findall(rb'tx_start=(\d+) tx_complete=(\d+) tx_active_cancel=(\d+) tx_active=(\d+)', data):
            require(int(start) == int(complete)+int(cancelled)+int(active), 'serial activity cancellation mismatch')
    main = exactly('atdd', 'direct-'); direct(main); accounting(main)
    for prefix in ['pair-direct-false-', 'pair-direct-true-', 'loopback-false-', 'wide-', 'edges-false-']:
        direct(exactly('atdd', prefix))
    require(json.loads((exactly('atdd','edges-false-')/'coverage.json').read_text()) == EDGE_COUNTS, 'direct edge matrix coverage mismatch')
    reach = re.findall(r'UART IRQ reach dual_clear=(\d+) sticky_quiet_clear=(\d+) subsequent_event=(\d+) clears_with_event=(\d+)', (run/'atdd.log').read_text())
    require(len(reach) == 2 and all(int(n)>0 for row in reach for n in row), 'missing actual IRQ collision/retrigger reach')
    mutations=exactly('atdd', 'behavior-mutations-')
    for name, assertion in [('control', 'FR197 UART PASS'), ('serial-countdown', 'tx expected='),
                             ('fifo-push', 'after error expected=2 got=0'), ('raw-event', 'raw_events expected=')]:
        path=mutations/name
        for file in ['design.v', 'tb.sv']: nonempty(path/file)
        command(path/'compile.json'); command(path/'simulate.json', 0 if name=='control' else 1)
        require(json.loads((path/'simulate.json').read_text()).get('expected_assertion') == assertion,
                f'unchecked behavioral fault: {name}')
        passed(path/'simulate.log', assertion); vcd(path/'trace.vcd')
        if name!='control': require('FR197 UART PASS' not in (path/'simulate.log').read_text(), 'fault falsely passed')
    for label,prefixes in [('backends',['backends-']),('pair-backends',['pair-backends-false-','pair-backends-true-']),('loopback-backends',['loopback-true-']),('edge-backends',['edges-true-'])]:
        for prefix in prefixes:
            path=exactly(label,prefix)
            for name in ['design.fir','tb.sv','src/main/scala/Design.scala']:nonempty(path/name)
            for name in ['firrtl-lower.json','firrtl-compile.json','firrtl-simulate.json','sbt.json','chisel/compile.json','chisel/simulate.json']:command(path/name)
            passed(path/'firrtl-simulate.log','FR197 UART PASS');passed(path/'chisel/simulate.log','FR197 UART PASS')
            vcd(path/'firrtl.vcd');vcd(path/'chisel/trace.vcd')
            if label=='backends':accounting(path)
            if label=='edge-backends':require(json.loads((path/'coverage.json').read_text())==EDGE_COUNTS, 'backend edge matrix coverage mismatch')
    def status(path, expected):
        nonempty(path);require(path.read_text().split()[0]==expected, f'wrong formal status: {path}')
    path=exactly('formal','proof-');nonempty(path/'uart.v')
    for task in ['prove','cover']:
        status(path/f'uart_{task}/status','PASS');command(path/f'{task}-command.json')
    for name in ['logfile_basecase.txt','logfile_induction.txt']:
        passed(path/'uart_prove/engine_0'/name,'Status: passed')
    log=(path/'uart_cover/engine_0/logfile.txt').read_text()
    pending,witnessed=set(),set()
    for line in log.splitlines():
        match=re.search(r'Reached cover statement at (.+) in step \d+\.',line)
        if match:pending.add(match[1])
        match=re.search(r'Writing trace to VCD file: (engine_0/trace\d+\.vcd)$',line)
        if match:vcd(path/'uart_cover'/match[1]);witnessed.update(pending);pending.clear()
    require(len(witnessed)==3 and not pending and 'Status: passed' in log,'missing three witnessed covers')
    path=exactly('formal','mutation-')
    status(path/'control/status','PASS');command(path/'control-command.json')
    for name in ['logfile_basecase.txt','logfile_induction.txt']:passed(path/'control/engine_0'/name,'Status: passed')
    status(path/'ready-stuck-low/status','FAIL');command(path/'ready-stuck-low-command.json',2)
    log=(path/'ready-stuck-low/engine_0/logfile_basecase.txt').read_text()
    require(any('Assert failed' in line and line.rstrip().endswith(': uart_ready') for line in log.splitlines()),'wrong formal counterexample')
    vcd(path/'ready-stuck-low/engine_0/trace.vcd')
    path=exactly('formal','synthesis-');nonempty(path/'design.v');command(path/'synthesis-command.json')
    cells=json.loads((path/'synthesis.json').read_text()).get('modules',{}).get('UartCsr',{}).get('cells',{})
    require(cells and any('DFF' in c['type'] for c in cells.values()) and not any('LATCH' in c['type'] for c in cells.values()),'original UART synthesis missing registers or contains latch')
    return roots


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--run', required=True, type=Path)
    parser.add_argument('--include-history', action='store_true')
    parser.add_argument('archive', nargs='?', type=Path)
    args = parser.parse_args()
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%fZ')
    archive = args.archive or ROOT/'_agile-output/test-artifacts'/f'128-5-build-raw-{stamp}.tar.gz'
    manifest_path = archive.with_suffix('').with_suffix('.manifest.json')
    for path in [archive, manifest_path]:
        require(not path.exists() and not path.is_symlink(), f'refusing to overwrite {path}')
    run = args.run.resolve()
    roots = validate_run(run, ROOT)
    roots['run'] = run
    if args.include_history:
        for name in ['fr197-uart', 'fr197-uart-api', 'fr197-uart-formal', 'fr197-uart-reruns', 'fr197-uart-bootstrap']:
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
