#!/usr/bin/env python3
"""[P0] Story128.1 AC5: process CLI safety mutation acceptance.

Runs real gate copies, never an independent reimplementation of gate rules.
Each mutant removes exactly one diagnostic branch; positive control and
isolated negative input distinguish safety regression from crashes. No product,
HTTP, Pact, or future peripheral acceptance is claimed. Standard library only.
"""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
SPRINT = ROOT / '_agile-output/implementation-artifacts/sprint-status.yaml'
GATE = ROOT / 'scripts/check_phase24_gate.py'
SUCCESS = 'PASS: Phase 24 6 epics / 22 stories; M0 and NFR14 gates; FR189/Epic122 deferred\n'


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def set_state(text, prefix, value):
    suffix = r'[^:\n]*' if prefix.endswith('-') else ''
    pattern = rf'^(  {re.escape(prefix)}{suffix}:[ \t]*)[\w-]+([ \t]*(?:#.*)?)$'
    result, count = re.subn(pattern, lambda m: m[1] + value + m[2], text, flags=re.M)
    require(count == 1, f'fixture requires one {prefix}, found {count}')
    return result


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    originals = {SPRINT: SPRINT.read_bytes(), GATE: GATE.read_bytes()}
    source = originals[GATE].decode('utf-8')
    base = originals[SPRINT].decode('utf-8')
    # Isolate targets from evolving real sprint progress. Preserve all keys and
    # historical content while making a legal, explicit M0-only closed fixture.
    for epic, count in ((125, 3), (126, 4), (127, 4), (128, 5), (129, 3), (130, 3)):
        state = 'done' if epic == 125 else 'backlog'
        base = set_state(base, f'epic-{epic}', state)
        for story in range(1, count + 1):
            base = set_state(base, f'{epic}-{story}-', state)
    for prefix in ('epic-122', '122-2-', '122-3-'):
        base = set_state(base, prefix, 'deferred')

    m0_open = set_state(base, 'epic-125', 'in-progress')
    cases = [
        ('M0 epic advance', set_state(m0_open, 'epic-128', 'in-progress'),
         'Epic 128 cannot advance before M0 closes',
         "errors.append(f'Epic {epic} cannot advance before M0 closes')"),
        ('M0 story advance', set_state(m0_open, '128-1-', 'ready-for-dev'),
         '128.1 cannot advance before M0 closes',
         "errors.append(f'{epic}.{s} cannot advance before M0 closes')"),
        ('M0 inconsistent closure', set_state(base, '125-3-', 'review'),
         'Epic 125 done requires all three M0 stories done',
         "errors.append('Epic 125 done requires all three M0 stories done')"),
        ('NFR14 prerequisite', set_state(base, '128-2-', 'ready-for-dev'),
         '128.2 requires 128.1 NFR14 done',
         "errors.append(f'{epic}.{s} requires {epic}.1 NFR14 done')"),
        ('Epic premature closure', set_state(base, 'epic-128', 'done'),
         'Epic 128 done requires its stories done',
         "errors.append(f'Epic {epic} done requires its stories done')"),
        ('Historical epic deferred', set_state(base, 'epic-122', 'done'),
         'Epic 122 must remain deferred',
         "errors.append('Epic 122 must remain deferred')"),
        ('Historical story 122.2 deferred', set_state(base, '122-2-', 'done'),
         'FR189 story 122.2 must remain deferred',
         "errors.append(f'FR189 story 122.{n} must remain deferred')"),
        ('Historical story 122.3 deferred', set_state(base, '122-3-', 'done'),
         'FR189 story 122.3 must remain deferred',
         "errors.append(f'FR189 story 122.{n} must remain deferred')"),
    ]
    observations = []
    directory = None
    try:
        with tempfile.TemporaryDirectory(prefix='bitloom-1281-mutation-') as tmp:
            directory = Path(tmp)
            original_copy = directory / 'original.py'
            original_copy.write_bytes(originals[GATE])
            fixture = directory / 'sprint.yaml'

            def observe(name, executable, content, expected):
                fixture.write_text(content, encoding='utf-8')
                result = subprocess.run(
                    [sys.executable, '-B', str(executable), str(fixture)],
                    capture_output=True, text=True, timeout=30, check=False,
                )
                actual = (result.returncode, result.stdout, result.stderr)
                require(actual == expected, f'{name}: expected {expected!r}, actual {actual!r}')
                record = {'name': name, 'priority': 'P0', 'exit_code': result.returncode,
                          'stdout': result.stdout, 'stderr': result.stderr,
                          'input_sha256': sha(content.encode('utf-8')),
                          'gate_sha256': sha(executable.read_bytes())}
                observations.append(record)
                print(json.dumps(record, ensure_ascii=False))
                return actual

            success = (0, SUCCESS, '')
            observe('current real sprint / original control', original_copy,
                    originals[SPRINT].decode('utf-8'), success)
            for index, (name, negative, diagnostic, removal) in enumerate(cases):
                require(source.count(removal) == 1, f'{name}: mutation anchor not unique')
                mutant_source = source.replace(removal, 'pass  # safety mutation', 1)
                mutant = directory / f'mutant-{index}.py'
                mutant.write_text(mutant_source, encoding='utf-8')
                failure = (1, '', f'FAIL: {diagnostic}\n')
                observe(name + ' / original positive', original_copy, base, success)
                observe(name + ' / original exact negative', original_copy, negative, failure)
                observe(name + ' / mutant positive', mutant, base, success)
                mutant_result = observe(name + ' / mutant unsafe acceptance', mutant, negative, success)
                # Kill means the acceptance oracle rejects this precise missing
                # safety diagnostic. A syntax error/nonzero unrelated exception
                # cannot reach here: mutant must accept the isolated bad state.
                require(mutant_result != failure, f'{name}: mutation survived safety oracle')
                print(json.dumps({'mutation_killed': name, 'priority': 'P0',
                                  'required_safety_diagnostic': failure[2],
                                  'removed_statement': removal,
                                  'reason': 'isolated forbidden state accepted with exact success output'},
                                 ensure_ascii=False))
            require(len(observations) == 33, f'expected 33 CLI cases, got {len(observations)}')
    finally:
        for path, original in originals.items():
            current = path.read_bytes()
            require(current == original, f'real source changed: {path}')
            require(sha(current) == sha(original), f'real source hash changed: {path}')
            print(json.dumps({'preserved': str(path.relative_to(ROOT)), 'sha256': sha(current)}))
        if directory is not None:
            require(not directory.exists(), 'temporary mutation directory was not cleaned')
    print('RESULT: 33 P0 CLI cases passed; 8 targeted safety mutations killed; '
          'real sprint/source bytes and SHA256 preserved; temporary copies removed.')


if __name__ == '__main__':
    main()
