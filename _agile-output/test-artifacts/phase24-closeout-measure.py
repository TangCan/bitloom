#!/usr/bin/env python3
"""Reproduce scope/maintenance footprint metrics, without inferring productivity."""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parents[2]
def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

def main():
    boundary = '29c52dfbfa403e0ae9930d1061e99cfe73bd41fc'
    delivered = '4bdd680e1a6db7e9ca77e6d4fcd7caa4b825d2b1'
    commits = git('rev-list', '--first-parent', '--reverse', boundary+'..'+delivered).splitlines()
    ids = [f'{e}.{s}' for e,n in [(125,3),(126,4),(127,4),(128,5),(129,3),(130,3)] for s in range(1,n+1)]
    if len(commits) != len(ids):
        raise SystemExit('story/commit count mismatch')
    stories=[]
    for story,commit in zip(ids,commits):
        names=git('diff-tree','--no-commit-id','--name-only','-r',commit).splitlines()
        stories.append({'story':story,'commit':commit,'subject':git('show','-s','--format=%s',commit).strip(),
                        'changed_files':len(names),'product_rust_files':sum('/src/' in p and p.endswith('.rs') for p in names)})
    path='crates/bitloom/tests/fr198_peripheral_system.rs'
    text=(ROOT/path).read_text()
    # Physical lines, including comments and blanks; source ranges explicit.
    start=text.index('fn standalone('); end=text.index('\nfn emit_compile(')
    system=text[text.index('pub(crate) fn system('):end]
    metrics={'path':path,'sha256':hashlib.sha256(text.encode()).hexdigest(),
             'shared_composition_lines':len(text[start:end].splitlines()),
             'system_function_lines':len(system.splitlines()),
             'topologies':['axi','direct-csr'],
             'shared_leaf_types':['UartCsr','GpioCsr','Timer','Irq'],
             'shared_decoder':'CsrDecoder','axi_only_bridge':'AxiLiteCsrBridge',
             'method':'physical source lines from fn standalone through before fn emit_compile; not RTL LOC or human effort'}
    tb={}
    for name in ('axi_tb.sv','direct_tb.sv'):
        p=ROOT/'crates/bitloom/tests/fr198_peripheral_system'/name
        lines=p.read_text().splitlines()
        tb[name]={'physical_lines':len(lines),'address_literal_lines':sum("16'h" in l for l in lines),
                  'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
    result={'delivery_head':delivered,'measurement_head':git('rev-parse','HEAD').strip(),
            'scope':'counts only; no before/after baseline or human-time measurement',
            'stories':stories,'composition':metrics,'independent_bench_maintenance':tb}
    dest=ROOT/'_agile-output/test-artifacts/phase24-closeout-costs.json'
    dest.write_text(json.dumps(result,indent=2,ensure_ascii=False)+'\n')
    print(dest.relative_to(ROOT))
    print(json.dumps(metrics,ensure_ascii=False))

if __name__=='__main__': main()
