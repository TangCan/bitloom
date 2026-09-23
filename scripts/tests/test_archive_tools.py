from pathlib import Path
import subprocess,tempfile,shutil,json,hashlib,os
source=Path(__file__).resolve().parents[2];scratch=Path(tempfile.mkdtemp(prefix='bitloom-archive-tools-'));checks=[]
def run(args,cwd,ok=True):
 p=subprocess.run(args,cwd=cwd,text=True,capture_output=True)
 assert (p.returncode==0)==ok,(args,p.stdout,p.stderr)
 return p
repo=scratch/'git';repo.mkdir();run(['git','init','-q'],repo);run(['git','config','user.name','Archive verification'],repo);run(['git','config','user.email','archive-test@example.invalid'],repo)
(repo/'scripts').mkdir();shutil.copyfile(source/'scripts/check-git-artifact-size.py',repo/'scripts/check-git-artifact-size.py');run(['git','add','.'],repo);run(['git','commit','-qm','baseline'],repo);base=run(['git','rev-parse','HEAD'],repo).stdout.strip()
path=repo/'_agile-output/test-artifacts';path.mkdir(parents=True);(path/'small.tar.gz.sha256').write_text('checksum text\n');run(['git','add','.'],repo)
run(['python3','scripts/check-git-artifact-size.py','--staged'],repo);checks.append('small checksum manifest allowed')
(path/'forbidden.tar.gz').write_bytes(b'tiny archive');run(['git','add','.'],repo);run(['python3','scripts/check-git-artifact-size.py'],repo,False);checks.append('small raw archive rejected')
run(['git','rm','--cached','_agile-output/test-artifacts/forbidden.tar.gz'],repo);(path/'forbidden.tar.gz').unlink();(repo/'large.bin').write_bytes(b'x'*(5*1024*1024+1));run(['git','add','.'],repo);run(['python3','scripts/check-git-artifact-size.py'],repo,False);checks.append('large staged blob rejected')
run(['git','commit','-qm','large blob test fixture'],repo);run(['git','rm','large.bin'],repo);run(['git','commit','-qm','remove large blob fixture'],repo);run(['python3','scripts/check-git-artifact-size.py','--base',base],repo,False);checks.append('large blob in deleted history rejected')
repo=scratch/'restore';(repo/'scripts').mkdir(parents=True);(repo/'docs/evidence').mkdir(parents=True);shutil.copyfile(source/'scripts/restore-evidence.py',repo/'scripts/restore-evidence.py');store=scratch/'store';(store/'objects').mkdir(parents=True)
payload=b'archived proof\n';oid=hashlib.sha1(f'blob {len(payload)}\0'.encode()+payload).hexdigest();blob=store/'objects'/oid;blob.write_bytes(payload);relative='_agile-output/test-artifacts/proof.tar.gz';target=repo/relative
row={'path':relative,'git_oid':oid,'bytes':len(payload),'sha256':hashlib.sha256(payload).hexdigest()};manifest=repo/'docs/evidence/archive-index.json';manifest.write_text(json.dumps({'archive_objects':[row]}))
cmd=['python3','scripts/restore-evidence.py','--store',str(store)]
run(cmd,repo);assert not target.exists();checks.append('verify mode does not restore files')
run(cmd+['--restore'],repo);assert target.read_bytes()==payload and (target.stat().st_mode&0o777)==0o644;checks.append('restore verifies content and permissions')
r=run(cmd+['--restore'],repo);assert json.loads(r.stdout)['restored']==0;checks.append('restore is idempotent')
target.write_bytes(b'keep existing');run(cmd+['--restore'],repo,False);assert target.read_bytes()==b'keep existing';checks.append('conflicting existing data preserved')
target.unlink();blob.write_bytes(b'bad source');run(cmd+['--restore'],repo,False);assert not target.exists();checks.append('corrupt source refused before writes')
blob.write_bytes(payload);outside=scratch/'outside';outside.mkdir();target.parent.rmdir();target.parent.symlink_to(outside,target_is_directory=True);run(cmd+['--restore'],repo,False);assert not list(outside.iterdir());checks.append('destination symlink refused')
row['path']='../../escape';manifest.write_text(json.dumps({'archive_objects':[row]}));run(cmd+['--restore'],repo,False);checks.append('escaping manifest path refused')
output={'passed':True,'checks':checks,'count':len(checks),'test_directory':str(scratch)}
print(json.dumps(output,ensure_ascii=False,indent=2));shutil.rmtree(scratch)
