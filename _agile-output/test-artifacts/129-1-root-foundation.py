#!/usr/bin/env python3
"""129.1 旧基座实测；工具由环境提供，不声称 FR198 或形式证明。"""
import datetime, hashlib, importlib.util, json, os, shutil, signal, subprocess, tarfile, time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'_agile-output/test-artifacts'
PREFIX='129-1-root-foundation'
RUN=Path(os.environ.get('BITLOOM_FOUNDATION_OUTPUT',ROOT/'target'/PREFIX/(datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))))
RUN.mkdir(parents=True,exist_ok=False)
spec=importlib.util.spec_from_file_location('identity',OUT/'128-5-build-runner.py'); helper=importlib.util.module_from_spec(spec);spec.loader.exec_module(helper)
env=dict(os.environ);env.update(PYTHONDONTWRITEBYTECODE='1',CARGO_PROFILE_TEST_OPT_LEVEL='1',BITLOOM_REQUIRE_RTL='1')
def fingerprints():
    values=helper.source_fingerprint(ROOT)
    values[str(Path(__file__).relative_to(ROOT))]=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    return values
shutil.copyfile(__file__,RUN/'runner-source.py')
data={'scope':'旧BFM stub/真实桥译码基座；不是FR198/形式prove','seed':'N/A (确定性)','run':str(RUN),'commands':[],'source_start':fingerprints(),'status':'running'}
def save(): (RUN/'results.json').write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n')
def run(label,cmd,cwd=ROOT,timeout=300):
    r={'label':label,'command':list(map(str,cmd)),'cwd':str(cwd),'utc':helper.utc(),'resolved_executable':shutil.which(str(cmd[0]),path=env['PATH'])};data['commands'].append(r);save();start=time.monotonic()
    try:
        p=subprocess.Popen(cmd,cwd=cwd,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
        try:
            stdout,stderr=p.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            os.killpg(p.pid,signal.SIGKILL)
            stdout,stderr=p.communicate()
            (RUN/(label+'.stdout')).write_bytes(stdout);(RUN/(label+'.stderr')).write_bytes(stderr)
            r.update(exit_code=p.returncode,timed_out=True);save()
            raise
        (RUN/(label+'.stdout')).write_bytes(stdout);(RUN/(label+'.stderr')).write_bytes(stderr)
        r.update(exit_code=p.returncode,seconds=time.monotonic()-start,end_utc=helper.utc());save()
        helper.require(p.returncode==0,f'{label} failed {p.returncode}')
        return stdout
    except Exception as e:
        r.update(error=str(e),seconds=time.monotonic()-start);save();raise
try:
    commands=[('rustc',['rustc','--version']),('cargo',['cargo','--version']),('python',[os.environ['BITLOOM_BFM_PYTHON'],'--version']),('iverilog',['iverilog','-V']),('vvp',['vvp','-V']),('firtool',[str(Path(env['RHDL_FIRTOOL_PATH'])/'firtool'),'--version']),('java',['java','-version']),('yosys',['yosys','-V']),('z3',['z3','--version'])]
    for label,cmd in commands: run(label,cmd)
    serial=[0]
    def capture(cmd):
        label=f'sby-source-{serial[0]:02}';serial[0]+=1;return run(label,cmd)
    identity=helper.sby_identity(env,capture)
    support=str(Path(identity['launcher']).parent.parent/'share/yosys/python3')
    bootstrap='import sys,runpy; sys.path.insert(0,'+repr(support)+'); sys.argv=sys.argv[1:]; runpy.run_path(sys.argv[0],run_name="__main__")'
    cmd=['python3','-I','-B','-c',bootstrap,identity['launcher'],'--version']
    identity['isolated_runtime']=cmd
    run('sby-isolated-version',cmd)
    (RUN/'sby-identity.json').write_text(json.dumps(identity,indent=2)+'\n')
    run('bfm',[env['BITLOOM_BFM_PYTHON'],str(ROOT/'scripts/phase24_axi_bfm_probe.py')])
    bfm=(RUN/'bfm.stdout').read_text()+(RUN/'bfm.stderr').read_text()
    helper.require('BFM_API_PROBE_PASS' in bfm and 'BFM probe passed;' in bfm,'BFM actual pass missing')
    target='p0_real_hierarchy_routes_errors_partial_write_and_common_reset'
    run('bridge-decoder',['cargo','test','-p','bitloom','--test','fr196_csr_decoder_integration',target,'--','--exact','--nocapture'],timeout=600)
    helper.exact_test_passed((RUN/'bridge-decoder.stdout').read_text()+(RUN/'bridge-decoder.stderr').read_text(),target)
    data['source_end']=fingerprints();helper.require(data['source_start']==data['source_end'],'source drift')
    data['status']='pass';data['tests']={'bfm_stub':1,'old_bridge_decoder_rtl':1};save()
finally:
    data['source_end']=fingerprints()
    data['artifact_scope']='shared target directories may include pre-existing artifacts; only current command logs and exact test result establish this run outcome'
    if data['status']=='running':data['status']='failed'
    save()
    archive=OUT/(PREFIX+'-'+RUN.name+'.tar.gz')
    with tarfile.open(archive,'w:gz') as t:
        t.add(RUN,arcname='run')
        for p in [ROOT/'target/phase24-axi-bfm-probe',ROOT/'target/fr196-decoder-integration']:
            if p.exists():t.add(p,arcname=p.relative_to(ROOT))
    data['archive']={'path':str(archive.relative_to(ROOT)),'sha256':hashlib.sha256(archive.read_bytes()).hexdigest()}
    (OUT/(PREFIX+'-'+RUN.name+'-results.json')).write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n')
print(json.dumps({'status':data['status'],'run':str(RUN),'archive':data['archive']},indent=2))
