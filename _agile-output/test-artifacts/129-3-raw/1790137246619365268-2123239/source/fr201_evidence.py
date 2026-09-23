"""Independent, optimization-safe FR201 evidence consumer."""
import hashlib
import json
from pathlib import Path

BACKENDS = ('direct', 'firrtl', 'chisel')
TOOLS = ('cargo', 'iverilog', 'vvp', 'yosys', 'firtool', 'java', 'sbt', 'sby', 'solver', 'jq')

def require(ok, message):
    if not ok:
        raise ValueError(message)

SEEDS = '10203040 89abcdef 13579bdf 2468ace1 deadbeef c001d00d 31415926 27182818 0badf00d 55aaaa55 76543210 fedcba98 11223344 a5a55a5a 7f4a7c15 6d2b79f5'.split()
FEATURES = 'slverr decerr wstrb backpressure reset_cancel timer gpio uart_tx uart_rx irq_0_4'.split()

def provenance(row, directory, negative=False):
    hashes = row.get('source_sha256', {})
    require(bool(hashes), 'missing source hashes')
    for name, digest in hashes.items():
        path = directory / name
        require(path.resolve().is_relative_to(directory.resolve()), 'source path escaped')
        require(hashlib.sha256(path.read_bytes()).hexdigest() == digest, f'source hash: {name}')
    require(bool(row.get('commands')), 'missing command provenance')
    for command in row['commands']:
        require(command.get('exit_code') is not None, 'missing exit code')
        if not negative:
            require(command['exit_code'] == 0, 'failed backend command')
        require(command.get('timeout_seconds', 0) > 0 and command.get('start_utc_unix_ms', 0) > 0, 'command timing')
        require(len(command.get('tool_sha256', '')) == 64, 'command tool hash')
        require((directory / command['log']).is_file(), 'missing raw command log')
    require(bool(row.get('tools')), 'missing tool identities')
    for name, tool in row['tools'].items():
        require(tool.get('exit_code') == 0 and len(tool.get('sha256', '')) == 64, f'tool identity: {name}')


def validate(value, root, complete=True):
    root = Path(root).resolve()
    required_paths = set()
    for section, topology in [('backends', 'axi'), ('direct_csr_backends', 'direct-csr')]:
        require(set(value.get(section, {})) == set(BACKENDS), f'{section}: missing backend')
        for backend in BACKENDS:
            row = value[section][backend]
            require(row.get('exit_code') == 0, f'{section}/{backend}: exit')
            require(row.get('topology') == topology, f'{section}/{backend}: topology')
            require(row.get('random_seeds') == SEEDS, 'frozen random seeds')
            require(all(row.get('coverage', {}).get(name) is True for name in FEATURES), 'behavior coverage')
            require(row.get('transactions') == 16000, f'{section}/{backend}: transaction budget')
            require(isinstance(row.get('assertions'), int) and row['assertions'] > 0, 'assertions')
            require(row.get('vcd_bytes', 0) > 1024, 'empty waveform')
            key = backend if topology == 'axi' else f'direct-csr-{backend}'
            evidence = root / value['source_evidence'][key]
            require(evidence.resolve().is_relative_to(root), 'evidence escaped root')
            require(json.loads(evidence.read_text()) == row, 'aggregate differs from producer')
            provenance(row, evidence.parent)
            require(row.get('tools', {}).get('rustc', {}).get('stdout', '').startswith('rustc 1.97.1 '), 'Rust pin')
            if backend != 'direct':
                require('firtool-1.159.0' in row['tools'].get('firtool', {}).get('stdout', ''), 'firtool pin')
            required_paths.update(str(p.relative_to(root)) for p in evidence.parent.rglob('*') if p.is_file())
            design = evidence.parent / 'design.v'
            require(hashlib.sha256(design.read_bytes()).hexdigest() == row.get('sha256'), 'RTL hash mismatch')
            require((evidence.parent / 'tb.sv').stat().st_size > 0, 'missing oracle')
            waves = list(evidence.parent.glob('*.vcd'))
            require(any(p.stat().st_size == row['vcd_bytes'] for p in waves), 'waveform size mismatch')
            require(row.get('synthesis', {}).get('check_assert') is True, 'synthesis check')
            require(row['synthesis'].get('cells', 0) > 0, 'synthesis cells')
    formal = value['formal']
    for key, depth in [('prove', 4), ('cover', 2), ('response_reset_bmc', 8), ('request_reset_bmc', 8), ('request_reset_cover', 8)]:
        require(formal.get(key, {}).get('status') == 'PASS', f'formal {key}')
        require(formal[key].get('exit_code') == 0 and formal[key].get('depth') == depth, f'formal {key} result')
    negative = formal.get('negative_control', {})
    require(negative.get('status') == 'EXPECTED_FAIL' and negative.get('exit_code') not in (None, 0, 124, 137), 'formal negative')
    require(negative.get('counterexample_vcd') is True, 'formal counterexample')
    formal_path = root / value['source_evidence']['formal']
    require(formal_path.resolve().is_relative_to(root), 'formal path escaped')
    provenance(formal, formal_path.parent, negative=True)
    witnesses = formal['request_reset_cover'].get('witness_vcd', {})
    require(len(witnesses) >= 5, 'missing request/reset cover witnesses')
    for name, digest in witnesses.items():
        witness = formal_path.parent / name
        require(witness.resolve().is_relative_to(formal_path.parent.resolve()), 'cover path escaped')
        require(witness.stat().st_size > 0 and hashlib.sha256(witness.read_bytes()).hexdigest() == digest, 'cover witness hash')
    required_paths.update(str(p.relative_to(root)) for p in formal_path.parent.rglob('*') if p.is_file())
    for name, task in [('irq_wiring', 'negative'), ('request_hold', 'negative_request'), ('reset_cancel', 'negative_reset')]:
        control = formal.get('negative_controls', {}).get(name, {})
        require(control.get('status') == 'EXPECTED_FAIL' and control.get('exit_code') not in (None, 0, 124, 137), f'negative control: {name}')
        trace = formal_path.parent / task / 'engine_0/trace.vcd'
        require(trace.stat().st_size > 0 and control.get('counterexample_vcd') is True, f'negative trace: {name}')
        if name != 'irq_wiring':
            require(hashlib.sha256(trace.read_bytes()).hexdigest() == control.get('trace_sha256'), 'negative trace hash')
    require(json.loads(formal_path.read_text()) == formal, 'formal aggregate mismatch')
    require((formal_path.parent / 'negative/engine_0/trace.vcd').stat().st_size > 0, 'missing counterexample')
    require(value.get('semver', {}).get('harness') == 'PASS' and value['semver'].get('registry') == 'PASS', 'semver')
    for tool in TOOLS:
        row = value.get('missing_tool_negative', {}).get(tool, {})
        require(row.get('exit_code') == 4 and row.get('diagnostic') == f'required tool is missing: {tool}', f'missing tool {tool}')
    for command in value['commands']:
        require(command['exit_code'] == 0 and command['end_utc_unix_ms'] >= command['start_utc_unix_ms'], 'command outcome')
        require((root / command['log']).is_file(), 'missing command log')
        required_paths.add(command['log'])
    audit = value.get('compatibility_audit', {})
    require(audit.get('status') == 'PASS' and len(audit.get('scopes', [])) == 2, 'scoped compatibility audit')
    manifest = value.get('archive_manifest', {})
    require(bool(manifest), 'missing archive manifest')
    require(required_paths.issubset(manifest), 'archive manifest omits required evidence')
    for path, expected in manifest.items():
        target = root / path
        require(target.resolve().is_relative_to(root), 'archive path escaped root')
        require(hashlib.sha256(target.read_bytes()).hexdigest() == expected, f'archive hash mismatch: {path}')
    if complete:
        replay = value.get('isolated_replay', {})
        require(replay.get('exit_code') == 0, 'isolated replay missing')
        require(replay.get('used_main_target') is False and replay.get('used_hidden_tmp_rtl') is False, 'isolation')
        child = root / replay['evidence']
        require(child.resolve().is_relative_to(root), 'child evidence path escaped')
        validate(json.loads(child.read_text()), root, complete=False)
    return True
