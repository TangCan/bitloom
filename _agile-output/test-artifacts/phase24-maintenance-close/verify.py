# /// script
# dependencies = ["ruamel.yaml>=0.18"]
# ///
"""Read-only scoped validation against the maintenance baseline; run from repo root."""
import json
import re
import subprocess
from pathlib import Path
from ruamel.yaml import YAML

yaml = YAML(typ='safe')
root = Path('_agile-output/implementation-artifacts')
path = root / 'sprint-status.yaml'
baseline = 'f0b3551'
before = yaml.load(subprocess.check_output(['git', 'show', f'{baseline}:{path}'], text=True))
after = yaml.load(path.read_text())
expected = {f'epic-{n}-retrospective': 'done' for n in range(125, 131)}
old = before['development_status']; new = after['development_status']
assert {k: v for k, v in new.items() if k not in old} == expected
assert all(new[k] == v for k, v in old.items())
assert set(before) == set(after)
assert all(after[k] == v for k, v in before.items() if k not in ('development_status', 'action_items', 'last_updated'))
ids = {'phase24-retro-m3-doc-status', 'phase24-retro-m4-tracking-keys'}
assert len(before['action_items']) == len(after['action_items'])
seen = set()
for a, b in zip(before['action_items'], after['action_items']):
    if a['id'] in ids:
        assert a['status'] == 'open' and b['status'] == 'done'
        assert dict(b, status=a['status']) == a
        seen.add(a['id'])
    else:
        assert a == b
assert seen == ids
stories = {k: v for k, v in new.items() if re.match(r'^(125|126|127|128|129|130)-\d', k)}
assert len(stories) == 22 and set(stories.values()) == {'done'}
deferred = {k: v for k, v in new.items() if v == 'deferred'}
assert len(deferred) == 3 and 'epic-122' in deferred
artifact = Path('_agile-output/test-artifacts/phase24-maintenance-close')
a = json.loads((artifact / 'generic-validation-before.json').read_text())
b = json.loads((artifact / 'generic-validation-after.json').read_text())
assert a['problems'] == b['problems'] and len(b['problems']) == 3
retros = [root / f'epic-{n}-retro-2026-09-23.md' for n in range(125, 131)]
retros.append(root / 'phase24-retro-2026-09-23.md')
for p in retros:
    fm = yaml.load(p.read_text().split('---', 2)[1])
    assert fm['status'] == 'done' and fm['verdict'] == 'accepted'
files = retros + [Path('docs/ip/module-composition.md'), Path('docs/ip/csr.md'), artifact.parent / 'phase24-maintenance-close-verification.md']
links = 0
for p in files:
    for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)', p.read_text()):
        if '://' in target or target.startswith('#'):
            continue
        target = target.split('#')[0]
        assert (p.parent / target).exists(), (str(p), target)
        links += 1
subprocess.run(['git', 'diff', '--check'], check=True)
print(json.dumps({'valid': True, 'baseline': baseline, 'added_retrospectives': expected, 'phase24_done_stories': len(stories), 'updated_actions': sorted(ids), 'existing_data_preserved': True, 'deferred_preserved': deferred, 'generic_validator_expected_exceptions': b['problems'], 'local_links_checked': links, 'git_diff_check': 'pass'}, ensure_ascii=False, indent=2))
