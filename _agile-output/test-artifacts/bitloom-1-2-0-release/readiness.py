"""验证1.2.0候选清单与归档，不访问网络或发布。"""
from pathlib import Path
import json,hashlib,subprocess,tarfile,tomllib
root=Path.cwd();e=root/'_agile-output/test-artifacts/bitloom-1-2-0-release'
names={'bitloom','bitloom-macro','bitloom-hir','bitloom-builder','bitloom-vlog','bitloom-sim','bitloom-prelude','bitloom-firrtl','bitloom-viz','bitloom-lsp'}
m=json.loads(subprocess.check_output(['cargo','metadata','--offline','--no-deps','--format-version','1'],text=True))
packages=[p for p in m['packages'] if p['name'] in names]
assert len(packages)==10
assert all(p['version']=='1.2.0' and p['publish']!=[] for p in packages)
assert all(p['publish']==[] for p in m['packages'] if p['name'].startswith('rhdl-'))
for p in packages:
 for d in p['dependencies']:
  if d['name'] in names:assert d['req']=='^1.2.0',(p['name'],d)
hashes=json.loads((e/'package-sha256.json').read_text())
for name in names:
 archive=root/'target/package'/f'{name}-1.2.0.crate'
 assert hashlib.sha256(archive.read_bytes()).hexdigest()==hashes[archive.name]
 with tarfile.open(archive) as t:
  p=tomllib.loads(t.extractfile(f'{name}-1.2.0/Cargo.toml').read().decode())
  assert p['package']['version']=='1.2.0'
  for section in ('dependencies','dev-dependencies','build-dependencies'):
   for dep,info in p.get(section,{}).items():
    if dep in names:assert info['version']=='1.2.0' and 'path' not in info,(name,dep,info)
lock=e/'workspace-Cargo.lock.txt';assert root.joinpath('Cargo.lock').read_bytes()==lock.read_bytes()
assert json.loads((e/'standalone-summary.json').read_text())['passed']
assert json.loads((e/'lsp-smoke.json').read_text())['passed']
subprocess.run(['cargo','fmt','--all','--check'],check=True)
subprocess.run(['git','diff','--check'],check=True)
subprocess.run(['git','diff','--cached','--check'],check=True)
print(json.dumps({'passed':True,'candidate_version':'1.2.0','packages':sorted(names),'internal_dependency_minimum':'1.2.0','workspace_lock_sha256':hashlib.sha256(lock.read_bytes()).hexdigest(),'format_and_diff':'pass','archive_hashes_verified':len(hashes),'published':False},ensure_ascii=False,indent=2))
