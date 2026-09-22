#!/usr/bin/env python3
"""三后端复位风险探针；PATH/CHISEL_FIRTOOL_PATH 提供固定工具，输出必须新目录。"""
import atexit, argparse, datetime, hashlib, json, os, pathlib, re, shutil, subprocess, time
P = pathlib.Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument('--output', required=True, type=pathlib.Path)
a = parser.parse_args(); out = a.output.resolve()
if out == P or P in out.parents:
    raise SystemExit('output must be outside probe source')
out.mkdir(parents=True, exist_ok=False)
env = os.environ.copy(); env.update(PYTHONDONTWRITEBYTECODE='1', CARGO_PROFILE_TEST_OPT_LEVEL='1', BITLOOM_REQUIRE_RTL='1')
records = []
def fingerprints():
    root = P.parents[2]
    paths = list(P.rglob('*')) + list((root/'crates').rglob('*'))
    paths += [root/'Cargo.toml', root/'Cargo.lock', root/'rust-toolchain.toml']
    return {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths if p.is_file() and 'target' not in p.parts and '__pycache__' not in p.parts and (P in p.parents or p.suffix in {'.rs','.toml','.lock'})}
source_before = fingerprints()
def finalize_failure():
    # Early tool/emission failures must retain inputs and completed command logs.
    if not (out/'source-identity.json').exists():
        after = fingerprints()
        (out/'source-identity.json').write_text(json.dumps(dict(before=source_before, after=after, unchanged=source_before==after), indent=2))
    if not (out/'results.json').exists():
        (out/'results.json').write_text(json.dumps({'status':'failed before backend results','commands':records}, indent=2))
    if not (out/'source').exists():
        shutil.copytree(P, out/'source', ignore=shutil.ignore_patterns('target','__pycache__'))
atexit.register(finalize_failure)
def run(label, cmd, cwd, timeout=120):
    start = time.monotonic(); utc = datetime.datetime.now(datetime.timezone.utc).isoformat()
    try:
        r = subprocess.run(['timeout', '--kill-after=5s', str(timeout), *cmd], cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        code, data = r.returncode, r.stdout
    except OSError as e: code, data = 127, str(e).encode()
    (out / (label+'.log')).write_bytes(data)
    records.append(dict(label=label, command=cmd, cwd=str(cwd), utc=utc, elapsed_seconds=time.monotonic()-start, exit_code=code, resolved_tool=shutil.which(cmd[0])))
    (out/'commands.json').write_text(json.dumps(records, indent=2))
    print(label, code, flush=True)
    return code == 0
for tool, flag in [('cargo','--version'),('rustc','--version'),('firtool','--version'),('iverilog','-V'),('vvp','-V'),('java','-version'),('yosys','-V')]:
    if not run('version-'+tool,[tool,flag],P): raise SystemExit('required tool unavailable')
for tool, expected in [('rustc', '1.97.1'), ('cargo','1.97.1'), ('firtool','1.159.0')]:
    match = re.search((r'CIRCT firtool-' if tool == 'firtool' else tool + r' ') + r'([0-9]+\.[0-9]+\.[0-9]+)(?![\d.])', (out/('version-'+tool+'.log')).read_text())
    if match is None or match.group(1) != expected:
        raise SystemExit('required pinned version mismatch: '+tool)
chisel_dir = pathlib.Path(env.get('CHISEL_FIRTOOL_PATH', pathlib.Path(shutil.which('firtool')).parent)).resolve()
chisel_tool = chisel_dir/'firtool'
if not run('version-chisel-firtool',[str(chisel_tool),'--version'],P):
    raise SystemExit('Chisel firtool unavailable')
match = re.search(r'CIRCT firtool-([0-9]+\.[0-9]+\.[0-9]+)(?![\d.])', (out/'version-chisel-firtool.log').read_text())
if match is None or match.group(1) != '1.159.0':
    raise SystemExit('Chisel firtool pin mismatch')
env['CHISEL_FIRTOOL_PATH'] = str(chisel_dir)
if not run('emit',['cargo','run','--locked','--manifest-path',str(P/'harness/Cargo.toml'),'--target-dir',str(out/'cargo-target'),'--',str(out)],P,300): raise SystemExit('emission failed')
# Same public wrapper for all backends. Chisel port spelling is normalized by a stateless shim.
wrapper = '''module ResetTop(input clk, aresetn, enable, input [7:0] data0,data1, output [7:0] out0,out1);
wire core_reset = ~aresetn;
CorePorts core(.clk(clk),.rst(core_reset),.enable(enable),.data0(data0),.data1(data1),.out0(out0),.out1(out1));
endmodule
'''
if (P/'boundary.v').read_text() != wrapper:
    raise SystemExit('boundary source differs from reviewed stateless adapter')
checks = '''
initial begin
$dumpfile("trace.vcd"); $dumpvars(0,tb);
#2; clk=1; #2; clk=0; // initialize via applicable reset
implicit_reset=0; aresetn=1; enable=1; data0=8'h35; data1=8'ha7;
#2; clk=1; #2;
if(out0!==8'h35 || out1!==8'ha7) $fatal(1,"nonzero setup failed");
clk=0; #2; aresetn=0; data0=8'hff; data1=8'hff; #2;
if(out0!==8'h35 || out1!==8'ha7) $fatal(1,"reset was asynchronous");
clk=1; #2;
if(out0!==0 || out1!==0) $fatal(1,"aresetn failed common synchronous clear / write priority");
clk=0; aresetn=1; data0=8'h19; data1=8'h63; #2;
if(out0!==0 || out1!==0) $fatal(1,"release changed state before edge");
clk=1; #2;
if(out0!==8'h19 || out1!==8'h63) $fatal(1,"release recovery failed");
$display("RESET PROBE PASS children=2 synchronous=1 priority=1 recovery=1 seed=N/A"); $finish;
end
initial begin #200; $fatal(1,"watchdog"); end
endmodule
'''
def yosys_quote(name):
    return '"' + name.replace('\\', '\\\\').replace('"', '\\"') + '"'

results = {}
for mode in ['typed','adapted']:
    d = out/mode
    (d/'project').mkdir()
    (d/'project/build.properties').write_text('sbt.version=1.10.11\n')
    (d/'build.sbt').write_text('scalaVersion := "2.13.18"\nlibraryDependencies += "org.chipsalliance" %% "chisel" % "7.15.0"\naddCompilerPlugin("org.chipsalliance" % "chisel-plugin" % "7.15.0" cross CrossVersion.full)\n')
    (d/'src/main/scala/Main.scala').write_text('object ProbeMain extends App { circt.stage.ChiselStage.emitSystemVerilogFile(new ResetCore, args=Array("--target-dir","chisel"), firtoolOpts=Array("--disable-all-randomization","--lowering-options=disallowLocalVariables")) }\n')
    lower = {'direct':True}
    lower['firrtl'] = run(mode+'-firrtl-lower',['firtool','design.fir','--verilog','--disable-all-randomization','--lowering-options=disallowLocalVariables','-o','firrtl.v'],d)
    lower['chisel'] = run(mode+'-chisel-lower',['sbt','-batch','runMain ProbeMain'],d,300)
    for backend in ['direct','firrtl','chisel']:
        key = mode+'-'+backend
        if not lower[backend]: results[key] = 'lower failed'; continue
        b = d/backend if backend=='chisel' else d/(backend+'-run'); b.mkdir(exist_ok=True)
        files = ['-f','filelist.f'] if backend=='chisel' else [str(d/(backend+'.v'))]
        ports = '.clock(clk),.reset(implicit_reset)' if backend=='chisel' else '.clk(clk)'
        prefix = 'io_' if backend=='chisel' else ''
        if mode=='typed':
            ports += ','+','.join('.'+prefix+n+'('+n+')' for n in ['aresetn','enable','data0','data1','out0','out1'])
            inst = 'ResetCore dut('+ports+');'
            extra=[]
        else:
            resetports = '.clock(clk),.reset(rst)' if backend=='chisel' else '.clk(clk),.rst(rst)'
            resetports += ','+','.join('.'+prefix+n+'('+n+')' for n in ['enable','data0','data1','out0','out1'])
            (b/'ports.v').write_text('module CorePorts(input clk,rst,enable,input [7:0] data0,data1,output [7:0] out0,out1); ResetCore core('+resetports+'); endmodule\n')
            shutil.copyfile(P/'boundary.v', b/'boundary.v')
            extra=['ports.v','boundary.v']; inst='ResetTop dut(.clk(clk),.aresetn(aresetn),.enable(enable),.data0(data0),.data1(data1),.out0(out0),.out1(out1));'
        (b/'tb.sv').write_text('module tb; reg clk=0,aresetn=0,implicit_reset=1,enable=0; reg[7:0] data0=0,data1=0; wire[7:0] out0,out1;\n'+inst+'\n'+checks)
        ok = run(key+'-compile',['iverilog','-g2012','-I','.','-s','tb','-o','simulation',*files,*extra,'tb.sv'],b)
        ok = ok and run(key+'-execute',['vvp','simulation'],b)
        if ok: ok = b'RESET PROBE PASS' in (out/(key+'-execute.log')).read_bytes()
        results[key] = 'PASS' if ok else 'FAIL'
        if mode=='adapted':
            designfiles = [str(d/'chisel'/name.strip()) for name in (d/'chisel/filelist.f').read_text().splitlines() if name.strip()] if backend=='chisel' else [str(d/(backend+'.v'))]
            synth = 'read_verilog -sv '+ ' '.join(yosys_quote(name) for name in designfiles+['ports.v','boundary.v'])+'; hierarchy -check -top ResetTop; synth -top ResetTop; check -assert; stat'
            results[key+'-synthesis'] = 'PASS' if run(key+'-synthesis',['yosys','-p',synth],b) else 'FAIL'
(out/'results.json').write_text(json.dumps(results,indent=2))
source_after = fingerprints()
(out/'source-identity.json').write_text(json.dumps(dict(before=source_before, after=source_after, unchanged=source_before==source_after), indent=2))
if source_before != source_after: raise SystemExit('probe source changed during execution')
# Preserve source bytes and SHA outside transient compilation products.
source = out/'source'; shutil.copytree(P, source, ignore=shutil.ignore_patterns('target','__pycache__'))
sha={str(p.relative_to(out)):hashlib.sha256(p.read_bytes()).hexdigest() for p in out.rglob('*') if p.is_file() and 'cargo-target' not in p.parts and 'target' not in p.parts}
(out/'sha256.json').write_text(json.dumps(sha,indent=2))
print(json.dumps(results,indent=2))
typed_chisel_ok = results.get('typed-chisel') == 'PASS'
if results.get('typed-chisel') == 'lower failed':
    record = next(r for r in records if r['label'] == 'typed-chisel-lower')
    diagnostic = (out/'typed-chisel-lower.log').read_text()
    typed_chisel_ok = record['exit_code'] == 1 and 'public module port must have concrete reset type' in diagnostic
if not typed_chisel_ok or any(results.get('typed-'+b) != 'PASS' for b in ['direct','firrtl']):
    raise SystemExit('typed backend result differs from accepted behavior or exact unsupported diagnostic')
if any(results.get('adapted-'+b)!='PASS' or results.get('adapted-'+b+'-synthesis')!='PASS' for b in ['direct','firrtl','chisel']): raise SystemExit(1)
