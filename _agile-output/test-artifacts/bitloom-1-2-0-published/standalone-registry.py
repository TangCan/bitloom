"""从crates.io安装后的包验证仓库外使用；不使用patch或path覆盖。"""
from pathlib import Path
import os,sys,subprocess,json,tomllib,hashlib,tempfile
root=Path.cwd(); s=Path(sys.argv[1]).resolve(); evidence=Path('/tmp/bitloom-1-2-0-publish')
env=dict(os.environ,CARGO_HOME=str(s/'cargo-home'),CARGO_TARGET_DIR=str(s/'consumer-target'),CARGO_NET_OFFLINE='false')
env.pop('CARGO_MANIFEST_DIR',None)
bin=s/'install/bin/cargo-bitloom'; work=Path(tempfile.mkdtemp(prefix='consumer-',dir=s))
results=[]
def run(args,cwd=work):
 p=subprocess.run([str(x) for x in args],cwd=cwd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=120)
 with (evidence/'standalone.log').open('a') as f:f.write('$ '+' '.join(map(str,args))+'\n'+p.stdout+p.stderr+'\nexit='+str(p.returncode)+'\n')
 assert p.returncode==0,(args,p.stdout[-4000:],p.stderr[-4000:]); results.append({'command':list(map(str,args)),'exit':0});return p.stdout
assert 'bitloom v1.2.0' in run(['cargo','install','--list','--root',s/'install'])
run([bin,'--help'])
# LSP没有CLI help/version；协议握手另行验证。
run([bin,'new','candidate_blink','--path',work])
design=work/'candidate_blink'
run([bin,'build','--package','candidate_blink','--manifest-dir',design,'--out-dir',work/'verilog'])
assert any((work/'verilog').glob('*.v'))
manifest_evidence={}
for mode in ('func','cycle'):
 out=work/mode
 run([bin,'gen-'+mode,'--package','candidate_blink','--manifest-dir',design,'--out-dir',out])
 manifest=(out/'Cargo.toml').read_text();deps=tomllib.loads(manifest)['dependencies']
 assert deps and all(v=='1.2.0' for v in deps.values()),deps
 (evidence/f'generated-{mode}-Cargo.toml.txt').write_text(manifest)
 run(['cargo','run','--manifest-path',out/'Cargo.toml'])
 manifest_evidence[mode]=deps
# 直接编译运行已发布表面的Phase24 CSR示例，设计仅依赖prelude。
csr=work/'csr-example';(csr/'src').mkdir(parents=True)
(csr/'Cargo.toml').write_text('[package]\nname="candidate_csr"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[dependencies]\nbitloom-prelude="=1.2.0"\n')
source=(root/'docs/ip/csr.md').read_text().split('```rust\n',1)[1].split('```',1)[0]
(csr/'src/main.rs').write_text(source)
run(['cargo','run','--manifest-path',csr/'Cargo.toml'])
# 真实消费依赖必须来自crates.io，不得使用候选目录或源仓库path。
meta=json.loads(run(['cargo','metadata','--format-version','1','--manifest-path',design/'Cargo.toml']))
family=[p for p in meta['packages'] if p['name']=='bitloom' or p['name'].startswith('bitloom-')]
assert all(p['version']=='1.2.0' and p['source']=='registry+https://github.com/rust-lang/crates.io-index' for p in family)
(evidence/'standalone-summary.json').write_text(json.dumps({'passed':True,'sandbox':str(s),'generated_registry_dependencies':manifest_evidence,'consumer_packages':[{k:p[k] for k in ('name','version','source','manifest_path')} for p in family],'commands':results},ensure_ascii=False,indent=2)+'\n')
print('standalone candidate checks passed')
