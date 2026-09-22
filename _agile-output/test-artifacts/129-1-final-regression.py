#!/usr/bin/env python3
"""Story129.1 最终顺序 clean/fmt/完整回归；需要调用者配置已核工具环境。"""
import argparse, datetime, hashlib, importlib.util, json, os, signal, subprocess, time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'_agile-output/test-artifacts'
parser=argparse.ArgumentParser()
parser.add_argument('--attempt',default='final')
args=parser.parse_args()
if not args.attempt.replace('-','').isalnum():raise SystemExit('attempt must be alphanumeric or hyphen')
PREFIX='129-1-'+args.attempt
REPORT=OUT/(PREFIX+'-regression.json')
if REPORT.exists():raise SystemExit('refusing to overwrite final evidence')
spec=importlib.util.spec_from_file_location('fingerprint',OUT/'128-5-build-runner.py')
helper=importlib.util.module_from_spec(spec);spec.loader.exec_module(helper)
def fingerprints():
    values=helper.source_fingerprint(ROOT)
    for p in OUT.glob('129-1*'):
        candidates=p.rglob('*') if p.is_dir() else [p]
        for f in candidates:
            if f.is_file() and f.suffix in {'.py','.rs','.toml','.lock','.sv','.v','.scala'} and not {'target','cargo-target','__pycache__'}.intersection(f.parts):
                values[str(f.relative_to(ROOT))]=hashlib.sha256(f.read_bytes()).hexdigest()
    return dict(sorted(values.items()))
def archives():return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(OUT.glob('129-1*.tar.gz'))}
env=dict(os.environ);env.update(CARGO_PROFILE_TEST_OPT_LEVEL='1',BITLOOM_REQUIRE_RTL='1',PYTHONDONTWRITEBYTECODE='1')
data={'source_start':fingerprints(),'archives_start':archives(),'commands':[],'status':'running','environment':{k:env[k] for k in ['PATH','RHDL_FIRTOOL_PATH','BITLOOM_SBY_SOURCE','CARGO_PROFILE_TEST_OPT_LEVEL','BITLOOM_REQUIRE_RTL'] if k in env}}
def save():REPORT.write_text(json.dumps(data,indent=2)+'\n')
save()
try:
    for label,cmd,timeout in [('clean',['cargo','clean'],600),('fmt',['cargo','fmt','--all'],180),('probe-fmt-design',['cargo','fmt','--manifest-path',str(OUT/'129-1-reset-probe/design/Cargo.toml'),'--','--check'],180),('probe-fmt-harness',['cargo','fmt','--manifest-path',str(OUT/'129-1-reset-probe/harness/Cargo.toml'),'--','--check'],180),('test',['just','test'],3600)]:
        r={'label':label,'command':cmd,'start_utc':helper.utc(),'status':'attempted'};data['commands'].append(r);save();start=time.monotonic()
        try:
            with (OUT/f'{PREFIX}-{label}.log').open('x') as log:
                proc=subprocess.Popen(cmd,cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
                try:
                    code=proc.wait(timeout=timeout)
                except subprocess.TimeoutExpired:
                    os.killpg(proc.pid,signal.SIGKILL);proc.wait()
                    r.update(exit_code=proc.returncode,timed_out=True,status='fail')
                    raise
            r.update(exit_code=code,status='pass' if code==0 else 'fail')
            helper.require(code==0,f'{label} exit {code}')
        finally:r.update(end_utc=helper.utc(),seconds=time.monotonic()-start);save()
    data['source_end']=fingerprints();data['archives_end']=archives()
    helper.require(data['source_start']==data['source_end'],'source drift after clean/fmt/test')
    helper.require(data['archives_start']==data['archives_end'],'archived evidence drift')
    data['status']='pass'
except Exception as e:
    data.update(status='failed',error=repr(e));raise
finally:
    data['source_end']=fingerprints();data['archives_end']=archives();save()
print(json.dumps({'status':data['status'],'commands':data['commands']},indent=2))
