import os,json,time,datetime,subprocess,hashlib,importlib.util
from pathlib import Path
root=Path('/nvme_data2/richard/2026/rhdl');out=root/'_agile-output/test-artifacts';recordfile=out/'128-5-final-regression.json'
assert not recordfile.exists()
sp=importlib.util.spec_from_file_location('uart_runner',out/'128-5-build-runner.py');runner=importlib.util.module_from_spec(sp);sp.loader.exec_module(runner)
initial=runner.source_fingerprint(root)
env=dict(os.environ);env.update(PATH='/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:/tmp/bitloom-jvm-tools/bin:'+env['PATH'],CARGO_PROFILE_TEST_OPT_LEVEL='1',BITLOOM_REQUIRE_RTL='1',PYTHONDONTWRITEBYTECODE='1',RHDL_FIRTOOL_PATH='/home/richard/.cache/rhdl/firtool/1.159.0/bin',BITLOOM_SBY_SOURCE='/tmp/bitloom-1263-sby-src')
data={'commands':[],'source_start':initial,'status':'running'}
def save():recordfile.write_text(json.dumps(data,indent=2)+'\n')
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
save()
for label,cmd in [('clean',['cargo','clean']),('fmt',['cargo','fmt','--all']),('test',['just','test'])]:
 r={'label':label,'command':cmd,'start_utc':utc(),'status':'attempted'};data['commands'].append(r);save();t=time.monotonic()
 try:
  with (out/f'128-5-final-{label}.log').open('x') as log:code=subprocess.run(cmd,cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT).returncode
  r.update(exit_code=code,status='pass' if code==0 else 'fail')
 except Exception as e:r.update(status='launch-error',error=repr(e));raise
 finally:r.update(end_utc=utc(),seconds=time.monotonic()-t);save()
 if code:data['status']='fail';save();raise SystemExit(code)
data['source_end']=runner.source_fingerprint(root);data['source_unchanged']=data['source_end']==initial;data['status']='pass' if data['source_unchanged'] else 'source-drift';save();print(json.dumps({'status':data['status'],'source_unchanged':data['source_unchanged'],'commands':data['commands']},indent=2))
