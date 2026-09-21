#!/usr/bin/env python3
"""Focused review regressions; requires the existing pinned SBY install/source.

Run with Python and Python -O. Only copied installations are modified.
"""
import contextlib
import importlib.util
import hashlib
import io
import json
import os
from pathlib import Path
import py_compile
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def load(name):
    spec = importlib.util.spec_from_file_location(name, HERE / f'128-3-build-{name}.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class HarnessReview(unittest.TestCase):
    def setUp(self):
        self.runner = load('runner')
        self.archive = load('archive')
        self.temp = tempfile.TemporaryDirectory(prefix='irq-review-')
        self.addCleanup(self.temp.cleanup)
        self.work = Path(self.temp.name)

    def main(self, selection, out, extra_env=None):
        env = dict(os.environ, BITLOOM_IRQ_RERUN_DIR=str(out))
        env.update(extra_env or {})
        with patch.dict(os.environ, env, clear=True), patch.object(sys, 'argv', ['runner', '--only', selection]), contextlib.redirect_stdout(io.StringIO()):
            self.runner.main()

    def complete_run(self):
        root=self.work/'repo'
        root.mkdir()
        (root/'Cargo.toml').write_text('[workspace]\n')
        run=self.work/'custom-output'
        run.mkdir()
        def put(path, text='raw evidence'):
            path.parent.mkdir(parents=True,exist_ok=True)
            path.write_text(text)
        records={name:dict(label=name,exit=0,status='pass',artifact_roots=[]) for name in self.archive.GATES}
        def artifact(label, name, files):
            path=self.work/'artifacts'/name
            for file in files:put(path/file)
            records[label]['artifact_roots'].append(str(path))
            return path
        artifact('atdd','direct-seed',['design.v','tb.sv','trace.vcd','run.log'])
        for name in ['pair-true-false-unit','pair-false-false-unit','timer-irq-false-unit']:
            artifact('atdd',name,['dut.v','tb.sv','trace.vcd'])
        for label,names in [('backends',['backends-unit']),('pair-backends',['pair-true-true-unit','pair-false-true-unit','timer-irq-true-unit'])]:
            for name in names:artifact(label,name,['design.fir','firrtl.vcd','src/main/scala/Design.scala','chisel/trace.vcd'])
        proof=artifact('formal','proof-unit',['formal.v'])
        for task in ['prove','cover']:put(proof/f'irq_{task}/status','PASS 0 1')
        mutations=artifact('formal','mutations-unit',[])
        put(mutations/'control/status','PASS 0 1')
        for fault in ['event-clear-priority','test-without-commit','raw-includes-software']:
            put(mutations/fault/'status','FAIL 2 0')
            put(mutations/fault/'engine_0/trace.vcd')
        synth=artifact('formal','synthesis-unit',['design.v'])
        put(synth/'synthesis.json',json.dumps({'modules':{'Irq':{'cells':{'ff':{}}}}}))
        for label,target in [('backends','p1_irq_firrtl_chisel_same_independent_vectors'),('pair-backends','p1_composition_firrtl_chisel_shared_renamed_and_actual_timer')]:
            records[label]['exact_test']=target
            put(run/f'{label}.log',f'running 1 test\ntest {target} ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored;\n')
        put(run/'example/src/main.rs')
        comparisons=[]
        for name in ['irq-registers.h','irq-registers.md']:
            put(run/'example/software'/name)
            digest=hashlib.sha256(b'raw evidence').hexdigest()
            comparisons.append(dict(name=name,equal=True,expected_sha256=digest,actual_sha256=digest))
        put(run/'example-artifacts.json',json.dumps(comparisons))
        put(run/'sby-identity.json',json.dumps({'files':[{}]*16,'runtime_command':['isolated-test-fixture']}))
        fingerprint=self.runner.source_fingerprint(root)
        for name in ['start','end']:put(run/f'source-fingerprint-{name}.json',json.dumps(fingerprint))
        put(run/'commands.json',json.dumps(dict(commands=list(records.values()),source_fingerprint_complete=True)))
        return root,run,records

    def test_absent_and_empty_archive_inputs_fail_without_output(self):
        for populated in [False,True]:
            run=self.work/str(populated)
            if populated:run.mkdir()
            destination=self.work/f'{populated}.tar.gz'
            with patch.object(sys,'argv',['archive','--run',str(run),str(destination)]):
                with self.assertRaisesRegex(RuntimeError,'missing/empty evidence'):
                    self.archive.main()
            self.assertFalse(destination.exists())

    def test_custom_selected_run_archive_roundtrip_and_no_overwrite(self):
        root,run,_=self.complete_run()
        output=self.work/'capture.tar.gz'
        manifest=output.with_suffix('').with_suffix('.manifest.json')
        with patch.object(self.archive,'ROOT',root), patch.object(sys,'argv',['archive','--run',str(run),str(output)]), contextlib.redirect_stdout(io.StringIO()):
            # A lone manifest also forbids creating/replacing its paired tar.
            manifest.write_text('keep me')
            with self.assertRaisesRegex(RuntimeError,'refusing to overwrite'):self.archive.main()
            self.assertFalse(output.exists())
            self.assertEqual(manifest.read_text(),'keep me')
            manifest.unlink()
            self.archive.main()
            original=output.read_bytes()
            data=json.loads(manifest.read_text())
            self.assertEqual(data['selected_run'],str(run))
            self.assertIn('run/example-artifacts.json',data['members'])
            self.assertTrue(any(name.startswith('artifacts/formal/') for name in data['members']))
            with self.assertRaisesRegex(RuntimeError,'refusing to overwrite'):self.archive.main()
            self.assertEqual(output.read_bytes(),original)

    def test_no_historical_gate_union_and_comparison_required(self):
        root,run,records=self.complete_run()
        history=root/'target/fr197-irq-reruns/old'
        history.mkdir(parents=True)
        (history/'commands.json').write_text((run/'commands.json').read_text())
        for label in ['atdd','formal','example-artifacts']:
            for missing in [False,True]:
                with self.subTest(label=label,missing=missing):
                    changed=json.loads((history/'commands.json').read_text())
                    if missing:changed['commands']=[r for r in changed['commands'] if r['label']!=label]
                    else:
                        for r in changed['commands']:
                            if r['label']==label:r.update(exit=1,status='fail')
                    (run/'commands.json').write_text(json.dumps(changed))
                    with self.assertRaises(RuntimeError):self.archive.validate_run(run,root)

    def test_source_binding_and_associated_formal_statuses(self):
        root,run,records=self.complete_run()
        self.archive.validate_run(run,root)
        (root/'Cargo.toml').write_text('changed product source')
        with self.assertRaisesRegex(RuntimeError,'current related source'):self.archive.validate_run(run,root)
        (root/'Cargo.toml').write_text('[workspace]\n')
        proof=Path(records['formal']['artifact_roots'][0])
        mutations=Path(records['formal']['artifact_roots'][1])
        paths=[proof/'irq_prove/status',proof/'irq_cover/status',mutations/'control/status']
        paths += [mutations/fault/'status' for fault in ['event-clear-priority','test-without-commit','raw-includes-software']]
        for path in paths:
            original=path.read_text()
            for bad in ['ERROR','UNKNOWN','PASS' if original.startswith('FAIL') else 'FAIL']:
                path.write_text(bad)
                with self.subTest(path=path,bad=bad), self.assertRaises(RuntimeError):self.archive.validate_run(run,root)
            path.write_text(original)
        # An unrelated old PASS must never rescue a failed associated proof.
        old=self.work/'unrelated-proof';old.mkdir();(old/'status').write_text('PASS')
        (proof/'irq_prove/status').write_text('ERROR')
        with self.assertRaises(RuntimeError):self.archive.validate_run(run,root)

    def test_missing_selected_artifact_cannot_use_history(self):
        root,run,records=self.complete_run()
        victim=Path(records['backends']['artifact_roots'][0])/'firrtl.vcd'
        victim.unlink()
        with self.assertRaisesRegex(RuntimeError,'missing/empty evidence'):self.archive.validate_run(run,root)

    def test_exact_zero_ignored_and_wrong_targets_fail(self):
        target='p1_irq_firrtl_chisel_same_independent_vectors'
        for output in ['running 0 tests\ntest result: ok. 0 passed; 0 failed; 0 ignored;\n',
                       f'running 1 test\ntest {target} ... ignored\ntest result: ok. 0 passed; 0 failed; 1 ignored;\n',
                       'running 1 test\ntest unrelated ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored;\n']:
            with self.assertRaises(RuntimeError):self.runner.exact_test_passed(output,target)
        out=self.work/'zero-matched'
        real_run=subprocess.run
        def zero(command,**kwargs):
            if command[0]=='git':return real_run(command,**kwargs)
            kwargs['stdout'].write('running 0 tests\ntest result: ok. 0 passed; 0 failed; 0 ignored;\n')
            return subprocess.CompletedProcess(command,0)
        with patch.object(self.runner.subprocess,'run',side_effect=zero), self.assertRaisesRegex(RuntimeError,'did not execute'):
            self.main('backends',out)
        result=json.loads((out/'commands.json').read_text())['commands'][-1]
        self.assertEqual(result['process_exit'],0)
        self.assertEqual(result['status'],'error')
        self.assertIsNone(result['exit'])

    def test_during_run_source_change_rejected_but_logs_do_not_change_fingerprint(self):
        for mutate in [False,True]:
            root=self.work/f'repo-{mutate}';root.mkdir()
            (root/'Cargo.toml').write_text('[workspace]\n')
            out=self.work/f'run-{mutate}'
            def tool(command,**kwargs):
                if command[0]!='git':
                    logdir=root/'_agile-output/test-artifacts';logdir.mkdir(parents=True)
                    (logdir/'128-3-build-active.log').write_text('growing logs')
                    (logdir/'128-3-build-old-source.json').write_text('historical JSON changes')
                    formal_log=root/'crates/rhdl-formal/fixtures/fr119/fr119_pass/logfile.txt'
                    formal_log.parent.mkdir(parents=True,exist_ok=True)
                    formal_log.write_text('generated SBY runtime log, not a source input')
                    if mutate:(root/'Cargo.toml').write_text('changed during tool execution')
                return subprocess.CompletedProcess(command,0)
            with patch.object(self.runner,'ROOT',root), patch.object(self.runner.subprocess,'run',side_effect=tool):
                if mutate:
                    with self.assertRaisesRegex(RuntimeError,'source changed during run'):self.main('semver',out)
                else:self.main('semver',out)
            result=json.loads((out/'commands.json').read_text())
            self.assertEqual(result['source_fingerprint_complete'],not mutate)
            self.assertTrue((out/'source-fingerprint-end.json').exists())

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

    def test_software_comparison_is_explicit_pass_or_fail_gate(self):
        for mismatch in [False,True]:
            out=self.work/str(mismatch)
            real_run = subprocess.run
            def cargo(command, **kwargs):
                if command[0] == 'git': return real_run(command, **kwargs)
                software=Path(command[-1]);software.mkdir()
                for name in ['irq-registers.h','irq-registers.md']:
                    content=(ROOT/'docs/ip'/name).read_bytes()
                    if mismatch and name.endswith('.h'): content += b'controlled mismatch'
                    (software/name).write_bytes(content)
                return subprocess.CompletedProcess(command,0)
            with patch.object(self.runner.subprocess,'run',side_effect=cargo):
                if mismatch:
                    with self.assertRaises(SystemExit) as error:self.main('example',out)
                    self.assertEqual(error.exception.code,1)
                else:self.main('example',out)
            record=json.loads((out/'commands.json').read_text())['commands'][-1]
            self.assertEqual(record['label'],'example-artifacts')
            self.assertEqual(record['status'],'fail' if mismatch else 'pass')
            self.assertEqual(record['exit'],int(mismatch))

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


if __name__=='__main__':
    unittest.main(verbosity=2)
