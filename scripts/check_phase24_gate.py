#!/usr/bin/env python3
"""Check Phase 24 inventory, M0/NFR14 gates and historical deferral.

Parses only the flat development_status mapping, using the standard library.
An optional status path allows checking prospective states without editing the tree.
"""
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
COUNTS = {125: 3, 126: 4, 127: 4, 128: 5, 129: 3, 130: 3}
ACTIVE = {'ready', 'ready-for-dev', 'in-progress', 'review', 'done'}
VALID = ACTIVE | {'backlog', 'blocked', 'deferred'}


def check(path):
    text = path.read_text()
    errors = []
    match = re.search(r'^development_status:\s*\n(.*?)(?=^\S|\Z)', text, re.M | re.S)
    if not match:
        return ['missing development_status mapping']
    states = {}
    for line in match[1].splitlines():
        if not line.strip() or line.lstrip().startswith('#'):
            continue
        row = re.fullmatch(r'  ([^:#]+):\s*([\w-]+)\s*(?:#.*)?', line)
        if not row:
            errors.append(f'unsupported development_status line: {line!r}')
            continue
        key, value = row.groups()
        if key in states:
            errors.append(f'duplicate status key {key}')
        states[key] = value
    by_story = {}
    for key, value in states.items():
        m = re.match(r'^(12[5-9]|130)-(\d+)(?:-|$)', key)
        if m:
            sid = tuple(map(int, m.groups()))
            if sid in by_story:
                errors.append(f'duplicate story {sid[0]}.{sid[1]}')
            by_story[sid] = value
    expected = {(epic, s) for epic, count in COUNTS.items() for s in range(1, count + 1)}
    if set(by_story) != expected:
        errors.append(f'expected exactly 22 stories: missing {sorted(expected-set(by_story))}; extra {sorted(set(by_story)-expected)}')
    for epic in COUNTS:
        key = f'epic-{epic}'
        if states.get(key) not in VALID:
            errors.append(f'missing/invalid {key}')
    for sid, value in by_story.items():
        if value not in VALID:
            errors.append(f'invalid state {sid}: {value}')
    closed = states.get('epic-125') == 'done'
    if closed and any(by_story.get((125, s)) != 'done' for s in range(1, 4)):
        errors.append('Epic 125 done requires all three M0 stories done')
    for epic in range(126, 131):
        if not closed and states.get(f'epic-{epic}') in ACTIVE:
            errors.append(f'Epic {epic} cannot advance before M0 closes')
        for s in range(1, COUNTS[epic] + 1):
            value = by_story.get((epic, s))
            if value in ACTIVE:
                if not closed:
                    errors.append(f'{epic}.{s} cannot advance before M0 closes')
                if s > 1 and by_story.get((epic, 1)) != 'done':
                    errors.append(f'{epic}.{s} requires {epic}.1 NFR14 done')
        if states.get(f'epic-{epic}') == 'done' and any(by_story.get((epic, s)) != 'done' for s in range(1, COUNTS[epic]+1)):
            errors.append(f'Epic {epic} done requires its stories done')
    if states.get('epic-122') != 'deferred':
        errors.append('Epic 122 must remain deferred')
    for n in (2, 3):
        historical = [v for k, v in states.items() if k.startswith(f'122-{n}-')]
        if historical != ['deferred']:
            errors.append(f'FR189 story 122.{n} must remain deferred')
    return errors


def main():
    path = Path(sys.argv[1]) if len(sys.argv) == 2 else ROOT / '_agile-output/implementation-artifacts/sprint-status.yaml'
    if len(sys.argv) > 2:
        print('usage: check_phase24_gate.py [sprint-status.yaml]', file=sys.stderr)
        return 2
    errors = check(path)
    if errors:
        print('\n'.join('FAIL: ' + item for item in errors), file=sys.stderr)
        return 1
    print('PASS: Phase 24 6 epics / 22 stories; M0 and NFR14 gates; FR189/Epic122 deferred')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
