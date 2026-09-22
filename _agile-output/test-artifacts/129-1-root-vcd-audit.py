#!/usr/bin/env python3
"""独立检查实际适配路线的顶层输出波形，不依赖 runner PASS 字符串。"""
import argparse, hashlib, json
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('run',type=Path);p.add_argument('--report',type=Path,required=True);a=p.parse_args()
reports=[]
for backend,folder in [('direct','direct-run'),('firrtl','firrtl-run'),('chisel','chisel')]:
    path=a.run/'adapted'/folder/'trace.vcd';text=path.read_text();scope=[];ids={};values={};times={};now=0
    for line in text.splitlines():
        words=line.split()
        if line.startswith('$scope'):scope.append(words[2])
        elif line.startswith('$upscope'):scope.pop()
        elif line.startswith('$var') and scope==['tb']:ids[words[4]]=words[3]
        elif line.startswith('#'):
            times[now]=dict(values);now=int(line[1:])
        elif line.startswith('b'):
            value,key=line[1:].split();values[key]=value
        elif line and line[0] in '01xz':values[line[1:]]=line[0]
    times[now]=dict(values)
    rows=[]
    for when,o0,o1,clock,resetn in [(6,0x35,0xa7,1,1),(10,0x35,0xa7,0,0),(12,0,0,1,0),(14,0,0,0,1),(16,0x19,0x63,1,1)]:
        got={name:int(times[when][ids[name]],2) for name in ['out0','out1','clk','aresetn','enable','implicit_reset']}
        expected=dict(out0=o0,out1=o1,clk=clock,aresetn=resetn,enable=1,implicit_reset=0)
        if got!=expected:raise SystemExit(f'{backend} at {when}: {got} != {expected}')
        rows.append({'time':when,'values':got})
    reports.append({'backend':backend,'vcd':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'checked_samples':rows})
a.report.write_text(json.dumps({'scope':'适配边界三后端双子状态；typed Chisel未执行，不证明隐式连接行为','results':reports},indent=2,ensure_ascii=False)+'\n')
print('3 backend VCDs: 15 independent samples passed')
