import json,subprocess,tempfile,hashlib,datetime,tarfile
from pathlib import Path
root=Path.cwd();src=root/'target/fr197-uart/direct-1415809-1789998869715577288';out=Path(tempfile.mkdtemp(prefix='bitloom-1285-rx-capture-'));source=(src/'design.v').read_text();needle='assign _uart_n107 = {sync2, _uart_n106};';assert source.count(needle)==1
results=[]
for name,stage in [('control','sync2'),('one_cycle_newer','sync1'),('one_cycle_older','history')]:
 d=out/name;d.mkdir();(d/'design.v').write_text(source.replace(needle,needle.replace('{sync2,','{'+stage+',')));(d/'tb.sv').write_bytes((src/'tb.sv').read_bytes())
 for label,cmd in [('compile',['iverilog','-g2012','-s','tb','-o','sim','design.v','tb.sv']),('simulate',['vvp','sim'])]:
  command=['timeout','--kill-after=5s','180s',*cmd];start=datetime.datetime.now(datetime.timezone.utc).isoformat()
  with (d/(label+'.log')).open('w') as log:r=subprocess.run(command,cwd=d,stdout=log,stderr=subprocess.STDOUT)
  record={'command':command,'exit_code':r.returncode,'start_utc':start,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()};(d/(label+'.json')).write_text(json.dumps(record,indent=2))
  expected=0 if label=='compile' or name=='control' else 1;assert r.returncode==expected,(name,label,r.returncode)
 log=(d/'simulate.log').read_text();assert ('FR197 UART PASS' in log) if name=='control' else ('rdata expected=' in log and 'FR197 UART PASS' not in log)
 wave=(d/'trace.vcd').read_text();assert '$enddefinitions' in wave and '#0' in wave
 results.append({'case':name,'capture':stage,'exit':0 if name=='control' else 1,'last_lines':log.splitlines()[-4:]})
files={str(p.relative_to(out)):hashlib.sha256(p.read_bytes()).hexdigest() for p in out.rglob('*') if p.is_file() and p.name!='sim'}
archive=root/'_agile-output/test-artifacts/128-5-code-review-rx-capture-raw.tar.gz'
with tarfile.open(archive,'x:gz') as t:
 for name in sorted(files):t.add(out/name,arcname=name,recursive=False)
with tarfile.open(archive) as t:
 for m in t:assert hashlib.sha256(t.extractfile(m).read()).hexdigest()==files[m.name]
report={'scope':'Review reproduction only: unchanged existing three-seed TB, emitted DUT capture source changed by one synchronizer stage; no repository source edit','source_directory':str(src),'work_directory':str(out),'results':results,'archive':archive.name,'archive_sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),'members':files,'status':'pass'}
(root/'_agile-output/test-artifacts/128-5-code-review-rx-capture-audit.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
