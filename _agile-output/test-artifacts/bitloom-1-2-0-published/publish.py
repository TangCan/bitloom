from pathlib import Path
import subprocess,json,datetime,hashlib,sys
out=Path('/tmp/bitloom-1-2-0-publish')
packages=['bitloom-macro','bitloom-hir','bitloom-builder','bitloom-vlog','bitloom-sim','bitloom-prelude','bitloom-firrtl','bitloom-viz','bitloom','bitloom-lsp']
assert not subprocess.check_output(['git','status','--porcelain'],text=True).strip()
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
assert head=='e33d12fd747e576a542e24f18be1e3cfea4760d7'
journal={'source_commit':head,'version':'1.2.0','started_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'packages':[]}
def save(): (out/'journal.json').write_text(json.dumps(journal,indent=2)+'\n')
for name in packages:
 entry={'name':name};journal['packages'].append(entry);save()
 for phase,flags in [('dry-run',['--dry-run']),('publish',[])]:
  print(name,phase,flush=True)
  with (out/f'{name}-{phase}.log').open('w') as log:
   p=subprocess.run(['cargo','publish','-p',name,'--locked',*flags],stdout=log,stderr=subprocess.STDOUT)
  entry[phase+'_exit']=p.returncode;save()
  if p.returncode:
   print('STOP:',name,phase,'exit',p.returncode,flush=True);sys.exit(p.returncode)
  if phase=='publish':
   archive=Path('target/package')/f'{name}-1.2.0.crate'
   entry['archive_sha256']=hashlib.sha256(archive.read_bytes()).hexdigest()
   entry['completed_at']=datetime.datetime.now(datetime.timezone.utc).isoformat();save()
 print(name,'published and index-ready',flush=True)
journal['complete']=True;save();print('ALL TEN PUBLISHED',flush=True)
