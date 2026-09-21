#!/usr/bin/env python3
"""UART evidence-validator regressions. Synthetic logs never certify product behavior.
Run normally and with -O; pinned SBY source/install required for isolation checks.
"""
import contextlib
import hashlib
import importlib.util
import io
import json
import os
import shutil
import subprocess
import py_compile
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
def load(file):
    spec=importlib.util.spec_from_file_location(file,HERE/file)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);return module
# Preserve the known-accounting bytes independently from the validator digest.
ACCOUNTING = 'seed=128519705511 frames=24139 accepted=2038 consumed=2029 cancelled=9 tx_push=70 tx_start=42 tx_cancel=24 rx_push=74 rx_pop=68 rx_cancel=6 events=[74, 42, 2, 18] samples=988 directed={"back_to_back": 6, "event_set_clear_collision": 3, "framing": 9, "full_width_bounded_launch": 3, "other_strobes": 48, "permissions": 44, "reset_cancel": 7, "reset_release": 2, "rx_config_start_collision": 5, "rx_prestate_collision": 3, "serial_phase_byte": 36, "short_start": 9, "tx_decode_busy_configuration": 4, "tx_full_pop_collision": 1, "wide_strobes": 176}\nactive-ledger seed=128519705511 tx_start=42 tx_complete=29 tx_active_cancel=12 tx_active=1 rx_active_cancel=8\nseed=deadbeef1285 frames=24136 accepted=2066 consumed=2057 cancelled=9 tx_push=63 tx_start=34 tx_cancel=25 rx_push=69 rx_pop=65 rx_cancel=4 events=[69, 34, 2, 13] samples=876 directed={"back_to_back": 6, "event_set_clear_collision": 3, "framing": 9, "full_width_bounded_launch": 3, "other_strobes": 48, "permissions": 44, "reset_cancel": 7, "reset_release": 2, "rx_config_start_collision": 5, "rx_prestate_collision": 3, "serial_phase_byte": 36, "short_start": 9, "tx_decode_busy_configuration": 4, "tx_full_pop_collision": 1, "wide_strobes": 176}\nactive-ledger seed=deadbeef1285 tx_start=34 tx_complete=21 tx_active_cancel=12 tx_active=1 rx_active_cancel=8\nseed=83592501ffff frames=24138 accepted=2051 consumed=2043 cancelled=8 tx_push=66 tx_start=40 tx_cancel=22 rx_push=77 rx_pop=72 rx_cancel=5 events=[77, 40, 2, 14] samples=984 directed={"back_to_back": 6, "event_set_clear_collision": 3, "framing": 9, "full_width_bounded_launch": 3, "other_strobes": 48, "permissions": 44, "reset_cancel": 7, "reset_release": 2, "rx_config_start_collision": 5, "rx_prestate_collision": 3, "serial_phase_byte": 36, "short_start": 9, "tx_decode_busy_configuration": 4, "tx_full_pop_collision": 1, "wide_strobes": 176}\nactive-ledger seed=83592501ffff tx_start=40 tx_complete=28 tx_active_cancel=11 tx_active=1 rx_active_cancel=8\n'

class Harness(unittest.TestCase):
    def setUp(self):
        self.runner=load('128-5-build-runner.py');self.archive=load('128-5-build-archive.py')
        self.temp=tempfile.TemporaryDirectory(prefix='uart-evidence-harness-');self.addCleanup(self.temp.cleanup);self.work=Path(self.temp.name)
    def main(self,selection,out,extra_env=None):
        env=dict(os.environ,BITLOOM_UART_RERUN_DIR=str(out));env.update(extra_env or {})
        with patch.dict(os.environ,env,clear=True),patch.object(sys,'argv',['runner','--only',selection]),contextlib.redirect_stdout(io.StringIO()):self.runner.main()
    def install_copy(self, label):
        original=Path(shutil.which('sby')).resolve()
        prefix=self.work/label
        (prefix/'bin').mkdir(parents=True)
        shutil.copy2(original,prefix/'bin/sby')
        shutil.copytree(original.parent.parent/'share/yosys/python3',prefix/'share/yosys/python3')
        return prefix
    def test_package_and_legacy_bytecode_shadow_rejected(self):
        for location in ['bin','bin/share/python3','share/yosys/python3']:
            for kind in ['package','pyc']:
                with self.subTest(location=location,kind=kind):
                    prefix=self.install_copy(location.replace('/','-')+kind)
                    folder=prefix/location;folder.mkdir(parents=True,exist_ok=True)
                    if kind=='package':
                        (folder/'sby_core').mkdir()
                        (folder/'sby_core/__init__.py').write_text("raise RuntimeError('shadow')\n")
                    else:(folder/'sby_core.pyc').write_bytes(b'unchecked legacy bytecode')
                    env=dict(os.environ,PATH=str(prefix/'bin')+os.pathsep+os.environ['PATH'])
                    with self.assertRaisesRegex(RuntimeError,'shadow SBY import candidate'):
                        self.runner.sby_identity(env)
    def test_normal_and_unchecked_hash_cache_use_verified_sources(self):
        for poison in [False,True]:
            prefix=self.install_copy('cached' if poison else 'normal')
            support=prefix/'share/yosys/python3'
            if poison:
                source=self.work/'poison.py';source.write_text("raise RuntimeError('UNVERIFIED_CACHE_EXECUTED')\n")
                cache=support/'__pycache__';cache.mkdir(exist_ok=True)
                for optimize in [0,1,2]:
                    suffix='' if optimize==0 else f'.opt-{optimize}'
                    cfile=cache/f'sby_core.{sys.implementation.cache_tag}{suffix}.pyc'
                    py_compile.compile(str(source),cfile=str(cfile),doraise=True,optimize=optimize,
                                       invalidation_mode=py_compile.PycInvalidationMode.UNCHECKED_HASH)
            out=self.work/('cached-run' if poison else 'normal-run')
            self.main('identity',out,{'PATH':str(prefix/'bin')+os.pathsep+os.environ['PATH']})
            identity=json.loads((out/'sby-identity.json').read_text())
            self.assertEqual(len(identity['files']),16)
            self.assertIn('SBY yosys-0.47',(out/'sby-runtime.log').read_text())
            self.assertEqual(list((out/'sby-runtime/cache').rglob('*.pyc')),[])
            self.assertIn('-I',identity['runtime_command'])
            # Prove the poisoned cache really is executable without isolation.
            if poison:
                process=subprocess.run([sys.executable,str(prefix/'bin/sby'),'--version'],capture_output=True,text=True)
                self.assertNotEqual(process.returncode,0)
                self.assertIn('UNVERIFIED_CACHE_EXECUTED',process.stderr)
    def test_launch_failure_preserves_attempt(self):
        out=self.work/'launch'
        real_run = subprocess.run
        def launch(command, **kwargs):
            if command[0] == 'git': return real_run(command, **kwargs)
            raise FileNotFoundError('controlled missing executable')
        with patch.object(self.runner.subprocess, 'run', side_effect=launch):
            with self.assertRaises(FileNotFoundError):
                self.main('semver',out)
        record=json.loads((out/'commands.json').read_text())['commands'][-1]
        self.assertEqual(record['command'],['just','semver-check'])
        self.assertEqual(record['status'],'launch-error')
        self.assertIsNone(record['exit'])
        self.assertIn('FileNotFoundError',record['error'])
        self.assertIn('start_utc',record)
        self.assertIn('end_utc',record)
    def test_fingerprint_includes_justfile_and_tool_pins(self):
        root=self.work/'fingerprint';root.mkdir()
        (root/'Cargo.toml').write_text('[workspace]\n')
        before=self.runner.source_fingerprint(root)
        for name in ['Justfile','rust-toolchain.toml','scripts/ci-sby-pins.env']:
            path=root/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_text('pin or command definition')
            after=self.runner.source_fingerprint(root)
            self.assertNotEqual(before,after)
            self.assertIn(name,after)
            before=after
    def complete_run(self):
        root=self.work/'repo';run=self.work/'run';root.mkdir();run.mkdir()
        def put(path,text='raw evidence'):
            path.parent.mkdir(parents=True,exist_ok=True);path.write_text(text)
        def command(path,code=0,expected=None):put(path,json.dumps(dict(exit_code=code,expected_assertion=expected)))
        def wave(path):put(path,'$enddefinitions $end\n#0')
        records={n:dict(label=n,exit=0,status='pass',artifact_roots=[]) for n in self.archive.GATES}
        for label in ['semver','legacy','numeric','firrtl-regression','example','header-compile','header-execute']:
            put(run/f'{label}.log','')  # Successful commands may emit no output.
        def artifact(label,name):
            path=self.work/'artifacts'/name;path.mkdir(parents=True);records[label]['artifact_roots'].append(str(path));return path
        def direct(name):
            path=artifact('atdd',name)
            for file in ['design.v','tb.sv']:put(path/file)
            command(path/'compile.json');command(path/'simulate.json');wave(path/'trace.vcd');put(path/'simulate.log','FR197 UART PASS');return path
        atdd=[]
        for target,(names,ignored) in self.runner.ATDD_TARGETS.items():
            atdd += [f'Running tests/{target}.rs (test-binary)',f'running {len(names)+len(ignored)} tests']
            atdd += [f'test {name} ... ok' for name in names]+[f'test {name} ... ignored, dedicated' for name in ignored]
            atdd += [f'test result: ok. {len(names)} passed; 0 failed; {len(ignored)} ignored;']
        atdd += ['UART IRQ reach dual_clear=3 sticky_quiet_clear=1 subsequent_event=1 clears_with_event=3']*2
        put(run/'atdd.log','\n'.join(atdd)+'\n')
        put(direct('direct-unit')/'accounting.log',ACCOUNTING)
        for name in ['pair-direct-false-unit','pair-direct-true-unit','loopback-false-unit','wide-unit']:direct(name)
        put(direct('edges-false-unit')/'coverage.json',json.dumps({'disabled_queue_access': 1, 'each_event_same_other_clear': 8, 'enabled_merged_div_legality': 6, 'final_busy_edge_reject': 2, 'nonzero_partial_div_merge': 16, 'reset_beats_stop_and_software': 1, 'reset_both_full_and_response': 1, 'reset_partial_synchronizer': 2, 'rx_only_or_dual_busy_configuration': 2, 'stalled_response_serial_and_inactive_noise': 1, 'tx_idle_configuration_collision': 5, 'tx_nonfull_simultaneous_push_pop': 1}))
        path=artifact('atdd','behavior-mutations-unit')
        for name,assertion in [('control','FR197 UART PASS'),('serial-countdown','tx expected='),('fifo-push','after error expected=2 got=0'),('raw-event','raw_events expected=')]:
            for file in ['design.v','tb.sv']:put(path/name/file)
            command(path/name/'compile.json');command(path/name/'simulate.json',0 if name=='control'else 1,assertion);wave(path/name/'trace.vcd');put(path/name/'simulate.log',assertion)
        for label,prefixes in [('backends',['backends-unit']),('pair-backends',['pair-backends-false-unit','pair-backends-true-unit']),('loopback-backends',['loopback-true-unit']),('edge-backends',['edges-true-unit'])]:
            for prefix in prefixes:
                path=artifact(label,prefix)
                for name in ['design.fir','tb.sv','src/main/scala/Design.scala']:put(path/name)
                for name in ['firrtl-lower.json','firrtl-compile.json','firrtl-simulate.json','sbt.json','chisel/compile.json','chisel/simulate.json']:command(path/name)
                for name in ['firrtl-simulate.log','chisel/simulate.log']:put(path/name,'FR197 UART PASS')
                wave(path/'firrtl.vcd');wave(path/'chisel/trace.vcd')
                if label=='backends':put(path/'accounting.log',ACCOUNTING)
                if label=='edge-backends':put(path/'coverage.json',json.dumps({'disabled_queue_access': 1, 'each_event_same_other_clear': 8, 'enabled_merged_div_legality': 6, 'final_busy_edge_reject': 2, 'nonzero_partial_div_merge': 16, 'reset_beats_stop_and_software': 1, 'reset_both_full_and_response': 1, 'reset_partial_synchronizer': 2, 'rx_only_or_dual_busy_configuration': 2, 'stalled_response_serial_and_inactive_noise': 1, 'tx_idle_configuration_collision': 5, 'tx_nonfull_simultaneous_push_pop': 1}))
        for label,target in [('backends','p1_uart_firrtl_chisel_same_independent_serial_vectors'),('pair-backends','p1_uart_pair_irq_firrtl_chisel'),('loopback-backends','p1_uart_actual_loopback_backends'),('edge-backends','p1_uart_contract_edges_backends')]:
            records[label]['exact_test']=target;put(run/f'{label}.log',f'running 1 test\ntest {target} ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored;\n')
        put(run/'formal.log','running 3 tests\n'+''.join(f'test {n} ... ok\n'for n in self.runner.FORMAL_TARGETS)+'test result: ok. 3 passed; 0 failed; 0 ignored;\n')
        path=artifact('formal','proof-unit');put(path/'uart.v')
        for task in ['prove','cover']:put(path/f'uart_{task}/status','PASS');command(path/f'{task}-command.json')
        for name in ['logfile_basecase.txt','logfile_induction.txt']:put(path/'uart_prove/engine_0'/name,'Status: passed')
        lines=[]
        for i in range(3):
            lines += [f'Reached cover statement at uart.v:{i} in step 4.',f'Writing trace to VCD file: engine_0/trace{i}.vcd'];wave(path/f'uart_cover/engine_0/trace{i}.vcd')
        put(path/'uart_cover/engine_0/logfile.txt','\n'.join(lines+['Status: passed']))
        path=artifact('formal','mutation-unit');put(path/'control/status','PASS');command(path/'control-command.json')
        for name in ['logfile_basecase.txt','logfile_induction.txt']:put(path/'control/engine_0'/name,'Status: passed')
        put(path/'ready-stuck-low/status','FAIL');command(path/'ready-stuck-low-command.json',2);put(path/'ready-stuck-low/engine_0/logfile_basecase.txt','Assert failed in UartCsr: uart_ready');wave(path/'ready-stuck-low/engine_0/trace.vcd')
        path=artifact('formal','synthesis-unit');put(path/'design.v');command(path/'synthesis-command.json');put(path/'synthesis.json',json.dumps({'modules':{'UartCsr':{'cells':{'ff':{'type':'$_DFF_P_'}}}}}))
        put(root/'Cargo.toml','[workspace]\n');put(root/'scripts/ci-sby-pins.env',(ROOT/'scripts/ci-sby-pins.env').read_text())
        put(run/'example/src/main.rs');comparisons=[]
        for name in ['uart-csr-registers.h','uart-csr-registers.md']:
            put(root/'docs/ip'/name);put(run/'example/software'/name);digest=hashlib.sha256(b'raw evidence').hexdigest();comparisons.append(dict(name=name,equal=True,expected_sha256=digest,actual_sha256=digest))
        put(run/'example-artifacts.json',json.dumps(comparisons))
        pins=dict(line.split('=',1)for line in (root/'scripts/ci-sby-pins.env').read_text().splitlines()if line and not line.startswith('#'))
        put(run/'sby-runtime.log','SBY '+pins['SBY_GIT_REF']);put(run/'sby-identity.json',json.dumps(dict(files=[{}]*16,runtime_command=['fixture'],commit=self.runner.PIN_COMMIT,tag_object=pins['SBY_GIT_SHA'])))
        fp=self.runner.source_fingerprint(root)
        for name in ['start','end']:put(run/f'source-fingerprint-{name}.json',json.dumps(fp))
        put(run/'commands.json',json.dumps(dict(commands=list(records.values()),source_fingerprint_complete=True)))
        self.archive.validate_run(run,root)
        return root,run,records
    def test_missing_original_gate_logs_rejected(self):
        root,run,_=self.complete_run()
        for label in sorted(self.archive.GATES-{'example-artifacts'}):
            path=run/f'{label}.log';original=path.read_bytes();path.unlink()
            with self.subTest(gate=label),self.assertRaisesRegex(RuntimeError,'missing original gate log'):
                self.archive.validate_run(run,root)
            path.write_bytes(original)
        self.archive.validate_run(run,root)
    def test_missing_each_gate_and_class_rejected(self):
        root,run,records=self.complete_run();path=run/'commands.json';old=path.read_text()
        for label in self.archive.GATES:
            data=json.loads(old);data['commands']=[r for r in data['commands']if r['label']!=label];path.write_text(json.dumps(data))
            with self.subTest(gate=label),self.assertRaises(RuntimeError):self.archive.validate_run(run,root)
        for label,r in records.items():
            for victim in r['artifact_roots']:
                data=json.loads(old)
                for record in data['commands']:
                    if record['label']==label:record['artifact_roots'].remove(victim)
                path.write_text(json.dumps(data))
                with self.subTest(artifact=victim),self.assertRaises(RuntimeError):self.archive.validate_run(run,root)
        path.write_text(old)
    def test_missing_wrong_exit_marker_wave_cover_and_seed_fail(self):
        root,run,records=self.complete_run()
        mutations=[]
        for path in (self.work/'artifacts').rglob('*'):
            if path.is_file() and path.name.endswith('.json') and path.name!='synthesis.json':mutations.append((path,'{"exit_code":124}'))
            if path.is_file() and path.suffix=='.vcd':mutations.append((path,'bad waveform'))
            if path.is_file() and path.name in ['simulate.log','firrtl-simulate.log','logfile_basecase.txt','logfile_induction.txt']:mutations.append((path,'unrelated assertion'))
            if path.is_file() and path.name=='accounting.log':
                mutations.extend([(path,path.read_text().replace('seed=deadbeef1285','seed=bad')),(path,path.read_text().replace('tx_active_cancel=12','tx_active_cancel=0'))])
        mutations += [(run/'atdd.log',(run/'atdd.log').read_text().replace('dual_clear=3','dual_clear=0'))]
        cover=self.work/'artifacts/proof-unit/uart_cover/engine_0/logfile.txt';mutations.append((cover,cover.read_text().replace('Reached cover statement at uart.v:2 in step 4.','')))
        for path,value in mutations:
            old=path.read_text();path.write_text(value)
            with self.subTest(path=path),self.assertRaises(RuntimeError):self.archive.validate_run(run,root)
            path.write_text(old)
    def test_exact_test_skip_zero_and_formal_wrong_name_rejected(self):
        root,run,_=self.complete_run()
        for file in ['atdd.log','formal.log','backends.log']:
            path=run/file;old=path.read_text()
            for text in ['running 0 tests\ntest result: ok. 0 passed;',old.replace(' ... ok',' ... ignored',1),old.replace('p0_uart','unrelated',1) if file!='backends.log'else old.replace('p1_uart','unrelated',1)]:
                path.write_text(text)
                with self.subTest(file=file),self.assertRaises(RuntimeError):self.archive.validate_run(run,root)
            path.write_text(old)
    def test_source_drift_and_software_bytes_rejected(self):
        root,run,_=self.complete_run()
        for path in [root/'Cargo.toml',run/'example/software/uart-csr-registers.h',run/'example/software/uart-csr-registers.md']:
            old=path.read_bytes();path.write_bytes(old+b'changed')
            with self.assertRaises(RuntimeError):self.archive.validate_run(run,root)
            path.write_bytes(old)
    def test_archive_roundtrip_and_no_overwrite(self):
        root,run,_=self.complete_run();out=self.work/'out.tar.gz'
        with patch.object(self.archive,'ROOT',root),patch.object(sys,'argv',['archive','--run',str(run),str(out)]),contextlib.redirect_stdout(io.StringIO()):self.archive.main()
        self.assertTrue(out.is_file());self.assertTrue(self.work.joinpath('out.manifest.json').is_file())
        with patch.object(self.archive,'ROOT',root),patch.object(sys,'argv',['archive','--run',str(run),str(out)]),self.assertRaises(RuntimeError):self.archive.main()

if __name__=='__main__':unittest.main(verbosity=2)
