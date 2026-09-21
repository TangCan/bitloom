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
    spec = importlib.util.spec_from_file_location(name, HERE / f'128-4-build-{name}.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# Fixed original oracle accounting, independent of archive digest expectations.
DIRECT_ACCOUNTING_FIXTURES = {'128419705511': 'seed=128419705511 frames=18384 accepted=6801 consumed=6789 cancelled=12 rises=[889, 389, 538, 586, 479, 620, 748, 682, 468, 691, 661, 608, 775, 522, 531, 640, 550, 550, 410, 746, 399, 392, 629, 590, 601, 705, 574, 608, 646, 739, 609, 778] collisions=[16, 1, 4, 3, 7, 4, 9, 14, 10, 5, 4, 4, 5, 5, 4, 5, 16, 2, 3, 6, 5, 4, 6, 9, 13, 4, 3, 6, 3, 3, 4, 12] strobes=[[11, 10, 11, 7, 10, 7, 9, 10, 8, 8, 8, 6, 11, 9, 11, 214], [7, 11, 11, 7, 7, 10, 11, 11, 12, 9, 7, 9, 8, 7, 8, 602], [15, 18, 15, 12, 13, 15, 16, 18, 16, 15, 18, 17, 15, 12, 14, 15], [16, 16, 15, 17, 11, 13, 14, 13, 14, 16, 14, 13, 15, 16, 14, 15], [10, 13, 15, 8, 9, 12, 12, 18, 10, 8, 8, 15, 14, 12, 12, 107]] directed={"backpressure_natural_events": 1, "clear_full": 80, "clear_mixed": 80, "event_old_snapshot": 32, "every_pin_pipeline_initial_high_repeat_fall": 32, "full_strobe_matrix": 560, "held_write_consume_no_refill": 2, "idle_set_clear": 32, "in_actual_pins_all_directions": 16, "old_dir_input_to_output": 32, "old_dir_output_to_input": 32, "other_bit_clear_and_rise": 32, "permissions_aliases_with_rise": 126, "reset_beats_available_commit_and_rise": 5, "reset_cancels_response": 10, "same_bit_set_beats_clear": 32, "set_empty": 80, "set_mixed": 80, "w1c_strobes_with_new_rise": 16}\n', 'deadbeef1284': 'seed=deadbeef1284 frames=18383 accepted=6782 consumed=6770 cancelled=12 rises=[719, 162, 463, 547, 562, 453, 773, 473, 569, 550, 497, 543, 588, 564, 413, 492, 402, 420, 443, 555, 523, 416, 530, 436, 408, 548, 710, 686, 380, 600, 487, 922] collisions=[13, 1, 4, 5, 6, 3, 8, 11, 15, 3, 3, 4, 4, 5, 7, 6, 11, 2, 3, 2, 4, 3, 3, 1, 9, 3, 4, 7, 2, 3, 1, 13] strobes=[[13, 10, 8, 9, 9, 6, 16, 13, 10, 6, 6, 8, 9, 12, 9, 220], [10, 13, 9, 6, 5, 8, 10, 14, 9, 12, 8, 7, 11, 10, 6, 597], [16, 12, 16, 14, 18, 12, 14, 15, 13, 14, 18, 16, 13, 12, 14, 17], [13, 18, 15, 17, 18, 13, 17, 14, 13, 12, 16, 17, 16, 13, 16, 22], [14, 16, 7, 11, 13, 10, 14, 13, 11, 8, 8, 11, 13, 12, 12, 107]] directed={"backpressure_natural_events": 1, "clear_full": 80, "clear_mixed": 80, "event_old_snapshot": 32, "every_pin_pipeline_initial_high_repeat_fall": 32, "full_strobe_matrix": 560, "held_write_consume_no_refill": 2, "idle_set_clear": 32, "in_actual_pins_all_directions": 16, "old_dir_input_to_output": 32, "old_dir_output_to_input": 32, "other_bit_clear_and_rise": 32, "permissions_aliases_with_rise": 126, "reset_beats_available_commit_and_rise": 5, "reset_cancels_response": 10, "same_bit_set_beats_clear": 32, "set_empty": 80, "set_mixed": 80, "w1c_strobes_with_new_rise": 16}\n', '83592401ffff': 'seed=83592401ffff frames=18384 accepted=6798 consumed=6786 cancelled=12 rises=[867, 176, 456, 557, 493, 371, 585, 719, 430, 468, 516, 547, 587, 485, 656, 489, 423, 574, 669, 655, 671, 487, 552, 700, 675, 524, 587, 618, 412, 769, 698, 744] collisions=[13, 1, 4, 4, 5, 6, 2, 12, 12, 3, 3, 2, 3, 4, 4, 3, 9, 1, 7, 5, 5, 3, 2, 10, 12, 4, 4, 4, 4, 6, 5, 12] strobes=[[14, 11, 13, 10, 8, 8, 10, 12, 7, 9, 10, 9, 11, 10, 9, 212], [13, 8, 16, 7, 11, 10, 9, 8, 12, 8, 8, 13, 9, 14, 8, 600], [19, 16, 10, 13, 13, 13, 11, 18, 17, 12, 16, 14, 18, 16, 15, 19], [23, 14, 13, 13, 15, 13, 14, 14, 16, 14, 14, 18, 16, 14, 12, 16], [10, 10, 11, 10, 11, 11, 9, 13, 11, 9, 13, 11, 12, 13, 13, 109]] directed={"backpressure_natural_events": 1, "clear_full": 80, "clear_mixed": 80, "event_old_snapshot": 32, "every_pin_pipeline_initial_high_repeat_fall": 32, "full_strobe_matrix": 560, "held_write_consume_no_refill": 2, "idle_set_clear": 32, "in_actual_pins_all_directions": 16, "old_dir_input_to_output": 32, "old_dir_output_to_input": 32, "other_bit_clear_and_rise": 32, "permissions_aliases_with_rise": 126, "reset_beats_available_commit_and_rise": 5, "reset_cancels_response": 10, "same_bit_set_beats_clear": 32, "set_empty": 80, "set_mixed": 80, "w1c_strobes_with_new_rise": 16}\n'}

class HarnessReview(unittest.TestCase):
    def setUp(self):
        self.runner = load('runner')
        self.archive = load('archive')
        self.temp = tempfile.TemporaryDirectory(prefix='gpio-review-')
        self.addCleanup(self.temp.cleanup)
        self.work = Path(self.temp.name)

    def main(self, selection, out, extra_env=None):
        env = dict(os.environ, BITLOOM_GPIO_RERUN_DIR=str(out))
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
        def command(path, code=0):put(path,json.dumps({'exit_code':code}))
        def wave(path):put(path,'$enddefinitions $end\n#0')
        atdd=[]
        for target,(names,ignored) in self.runner.ATDD_TARGETS.items():
            atdd += [f'Running tests/{target}.rs (test-binary)',f'running {len(names)+1} tests']
            atdd += [f'test {name} ... ok' for name in names]
            atdd += [f'test {ignored} ... ignored, dedicated',
                     f'test result: ok. {len(names)} passed; 0 failed; 1 ignored;']
        put(run/'atdd.log','\n'.join(atdd)+'\n')
        for seed,accounting in DIRECT_ACCOUNTING_FIXTURES.items():
            path=artifact('atdd',f'direct-{seed}-unit',['design.v','tb.sv'])
            put(path/'accounting.log',accounting)
            wave(path/'trace.vcd')
            command(path/'compile.log.json');command(path/'run.log.json')
            put(path/'run.log','FR197 GPIO PASS')
        path=artifact('atdd','scoreboard-mutation-unit',[])
        for name in ['control','input-is-output']:
            put(path/name/'design.v');put(path/name/'tb.sv');wave(path/name/'trace.vcd')
            command(path/name/'compile.log.json')
            command(path/name/'run-command.json',0 if name=='control' else 1)
            put(path/name/'run.log','FR197 GPIO PASS' if name=='control' else
                'cycle=4 after rdata expected=80000000 got=00000000')
        for name in ['pair-true-false-unit','pair-false-false-unit','gpio-irq-false-unit']:
            path=artifact('atdd',name,['dut.v','tb.sv'])
            wave(path/'trace.vcd');command(path/'compile.json');command(path/'simulate.json')
            put(path/'simulate.log','PASS GPIO composition')
        for label,names in [('backends',['backends-unit']),('pair-backends',['pair-true-true-unit','pair-false-true-unit','gpio-irq-true-unit'])]:
            for name in names:
                path=artifact(label,name,['design.fir','src/main/scala/Design.scala'])
                wave(path/'firrtl.vcd');wave(path/'chisel/trace.vcd')
                for filename in ['firrtl-lower.json','firrtl-compile.json','firrtl-simulate.json',
                                 'sbt.json','chisel/compile.json','chisel/simulate.json']:
                    command(path/filename)
                marker='FR197 GPIO PASS' if label=='backends' else 'PASS GPIO composition'
                put(path/'firrtl-simulate.log',marker);put(path/'chisel/simulate.log',marker)
        proof=artifact('formal','proof-unit',['formal.v'])
        for task in ['prove','cover']:put(proof/f'gpio_{task}/status','PASS 0 1')
        for name in ['logfile_basecase.txt','logfile_induction.txt']:
            put(proof/'gpio_prove/engine_0'/name,'Status: passed')
        cover_log=[]
        for i in range(10):
            cover_log += [f'Reached cover statement at formal.v:{i} ($cover${i}) in step 4.',
                          f'Writing trace to VCD file: engine_0/trace{i}.vcd']
            put(proof/f'gpio_cover/engine_0/trace{i}.vcd','$enddefinitions $end\n#0')
        put(proof/'gpio_cover/engine_0/logfile.txt','\n'.join(cover_log+['Status: passed']))
        put(run/'formal.log','running 3 tests\n'+''.join(f'test {n} ... ok\n' for n in self.runner.FORMAL_TARGETS)
            +'test result: ok. 3 passed; 0 failed; 0 ignored;\n')
        mutations=artifact('formal','mutations-unit',[])
        put(mutations/'control/status','PASS 0 1')
        command(mutations/'control-command.json')
        for name in ['logfile_basecase.txt','logfile_induction.txt']:
            put(mutations/'control/engine_0'/name,'Status: passed')
        for fault,prop in [('event-clear-priority','gpio_events'),('input-is-output','gpio_input'),('set-ignores-strobes','gpio_out')]:
            put(mutations/fault/'status','FAIL 2 0')
            wave(mutations/fault/'engine_0/trace.vcd')
            command(mutations/f'{fault}-command.json',2)
            put(mutations/fault/'engine_0/logfile_basecase.txt',f'Assert failed in GpioCsr: {prop}')
        synth=artifact('formal','synthesis-unit',['design.v'])
        put(synth/'synthesis.json',json.dumps({'modules':{'GpioCsr':{'cells':{'ff':{}}}}}))
        for label,target in [('backends','p1_gpio_firrtl_chisel_same_independent_vectors'),('pair-backends','p1_composition_firrtl_chisel_shared_renamed_actual_gpio_irq')]:
            records[label]['exact_test']=target
            put(run/f'{label}.log',f'running 1 test\ntest {target} ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored;\n')
        put(run/'example/src/main.rs')
        comparisons=[]
        for name in ['gpio-csr-registers.h','gpio-csr-registers.md']:
            put(run/'example/software'/name)
            put(root/'docs/ip'/name)
            digest=hashlib.sha256(b'raw evidence').hexdigest()
            comparisons.append(dict(name=name,equal=True,expected_sha256=digest,actual_sha256=digest))
        put(run/'example-artifacts.json',json.dumps(comparisons))
        put(root/'scripts/ci-sby-pins.env',(ROOT/'scripts/ci-sby-pins.env').read_text())
        pins=dict(line.split('=',1) for line in (root/'scripts/ci-sby-pins.env').read_text().splitlines()
                  if line and not line.startswith('#'))
        put(run/'sby-runtime.log','SBY '+pins['SBY_GIT_REF']+'\n')
        put(run/'sby-identity.json',json.dumps({'files':[{}]*16,'runtime_command':['isolated-test-fixture'],
            'commit':self.runner.PIN_COMMIT,'tag_object':pins['SBY_GIT_SHA']}))
        fingerprint=self.runner.source_fingerprint(root)
        for name in ['start','end']:put(run/f'source-fingerprint-{name}.json',json.dumps(fingerprint))
        put(run/'commands.json',json.dumps(dict(commands=list(records.values()),source_fingerprint_complete=True)))
        return root,run,records

    def test_atdd_zero_and_mistaken_ignore_rejected(self):
        root,run,_=self.complete_run()
        valid=(run/'atdd.log').read_text()
        first=self.runner.ATDD_TARGETS['fr197_gpio'][0][0]
        for output in ['running 0 tests\ntest result: ok. 0 passed; 0 failed; 0 ignored;\n',
                       valid.replace(f'test {first} ... ok',f'test {first} ... ignored'),
                       valid.replace('2 passed; 0 failed; 1 ignored;','1 passed; 0 failed; 2 ignored;')]:
            with self.assertRaisesRegex(RuntimeError,'ATDD'):self.runner.atdd_tests_passed(output)
            (run/'atdd.log').write_text(output)
            with self.assertRaisesRegex(RuntimeError,'ATDD'):self.archive.validate_run(run,root)
        real_run=subprocess.run
        def zero(command,**kwargs):
            if command[0]!='cargo':return real_run(command,**kwargs)
            kwargs['stdout'].write('running 0 tests\ntest result: ok. 0 passed; 0 failed; 0 ignored;\n')
            return subprocess.CompletedProcess(command,0)
        out=self.work/'zero-atdd'
        with patch.object(self.runner.subprocess,'run',side_effect=zero),self.assertRaisesRegex(RuntimeError,'ATDD'):
            self.main('atdd',out)
        self.assertEqual(json.loads((out/'commands.json').read_text())['commands'][-1]['status'],'error')

    def test_archive_requires_all_seeds_coverage_and_scoreboard(self):
        root,run,records=self.complete_run()
        commands=run/'commands.json';original=commands.read_text()
        for prefix in ['direct-deadbeef1284-','scoreboard-mutation-']:
            changed=json.loads(original)
            for record in changed['commands']:
                if record['label']=='atdd':record['artifact_roots']=[p for p in record['artifact_roots'] if not Path(p).name.startswith(prefix)]
            commands.write_text(json.dumps(changed))
            with self.assertRaises(RuntimeError):self.archive.validate_run(run,root)
        commands.write_text(original)
        path=Path(records['atdd']['artifact_roots'][0])/'accounting.log'
        original=path.read_text()
        for altered in [original.replace('frames=18384','frames=0'),original.replace('"full_strobe_matrix": 560','"full_strobe_matrix": 0'),
                        original.replace('accepted=6801','accepted=6802')]:
            path.write_text(altered)
            with self.assertRaisesRegex(RuntimeError,'accounting/coverage'):self.archive.validate_run(run,root)
        path.write_text(original)
        mutation=next(Path(p) for p in records['atdd']['artifact_roots'] if Path(p).name.startswith('scoreboard-mutation-'))
        for filename,value in [('control/run-command.json','{"exit_code":1}'),('input-is-output/compile.log.json','{"exit_code":1}'),
                               ('input-is-output/run-command.json','{"exit_code":124}'),('input-is-output/run.log','FR197 GPIO PASS'),
                               ('input-is-output/trace.vcd','nonempty but not VCD')]:
            path=mutation/filename;old=path.read_text();path.write_text(value)
            with self.assertRaises(RuntimeError):self.archive.validate_run(run,root)
            path.write_text(old)

    def test_archive_rejects_false_composition_and_formal_failure_records(self):
        root,run,records=self.complete_run()
        direct=next(Path(p) for p in records['atdd']['artifact_roots'] if Path(p).name.startswith('gpio-irq-'))
        backend=Path(records['pair-backends']['artifact_roots'][0])
        formal=next(Path(p) for p in records['formal']['artifact_roots'] if Path(p).name.startswith('mutations-'))
        changes=[(direct/'simulate.log','no pass'),(direct/'simulate.json','{"exit_code":1}'),
                 (backend/'firrtl-simulate.log','no pass'),(backend/'chisel/simulate.json','{"exit_code":1}'),
                 (backend/'sbt.json','{"exit_code":124}'),(formal/'control-command.json','{"exit_code":2}')]
        for fault in ['event-clear-priority','input-is-output','set-ignores-strobes']:
            changes.extend([(formal/f'{fault}-command.json','{"exit_code":124}'),
                            (formal/fault/'engine_0/logfile_basecase.txt','Assert failed in GpioCsr: unrelated'),
                            (formal/fault/'engine_0/trace.vcd','not a VCD')])
        for path,value in changes:
            old=path.read_text();path.write_text(value)
            with self.subTest(path=path),self.assertRaises(RuntimeError):self.archive.validate_run(run,root)
            path.write_text(old)
        victim=backend/'chisel/simulate.json';victim.unlink()
        with self.assertRaisesRegex(RuntimeError,'missing/empty'):self.archive.validate_run(run,root)

    def test_archive_rechecks_software_bytes_identity_and_witnesses(self):
        root,run,records=self.complete_run()
        self.archive.validate_run(run,root)
        for name in ['gpio-csr-registers.h','gpio-csr-registers.md']:
            path=run/'example/software'/name
            original=path.read_bytes()
            path.write_bytes(original+b'changed after comparison')
            with self.assertRaisesRegex(RuntimeError,'software bytes/hash mismatch'):
                self.archive.validate_run(run,root)
            path.write_bytes(original)
        comparison=run/'example-artifacts.json'
        original=comparison.read_text()
        forged=json.loads(original)
        for item in forged:item.update(expected_sha256='0'*64,actual_sha256='0'*64)
        comparison.write_text(json.dumps(forged))
        with self.assertRaisesRegex(RuntimeError,'software bytes/hash mismatch'):self.archive.validate_run(run,root)
        comparison.write_text(original)
        identity=run/'sby-identity.json'
        original=identity.read_text()
        for key in ['commit','tag_object']:
            changed=json.loads(original);changed[key]='0'*40;identity.write_text(json.dumps(changed))
            with self.assertRaisesRegex(RuntimeError,'SBY pinned identity mismatch'):self.archive.validate_run(run,root)
        identity.write_text(original)
        commands=run/'commands.json';original=commands.read_text()
        changed=json.loads(original)
        for record in changed['commands']:
            if record['label']=='sby-runtime':record.update(exit=1,status='fail')
        commands.write_text(json.dumps(changed))
        with self.assertRaisesRegex(RuntimeError,'unsuccessful selected-run gate'):self.archive.validate_run(run,root)
        commands.write_text(original)
        proof=Path(records['formal']['artifact_roots'][0])
        for name in ['gpio_prove/engine_0/logfile_basecase.txt','gpio_prove/engine_0/logfile_induction.txt',
                     'gpio_cover/engine_0/trace9.vcd']:
            path=proof/name;original=path.read_bytes();path.unlink()
            with self.assertRaisesRegex(RuntimeError,'missing/empty evidence'):self.archive.validate_run(run,root)
            path.write_bytes(original)
        log=proof/'gpio_cover/engine_0/logfile.txt';original=log.read_text()
        log.write_text(original.replace('Reached cover statement at formal.v:9 ($cover$9) in step 4.',''))
        with self.assertRaisesRegex(RuntimeError,'ten witnessed cover'):self.archive.validate_run(run,root)
        log.write_text(original)
        self.archive.validate_run(run,root)

    def test_formal_zero_ignored_wrong_target_and_archive_rejected(self):
        root,run,_=self.complete_run()
        valid=(run/'formal.log').read_text()
        invalid=['running 0 tests\ntest result: ok. 0 passed; 0 failed; 0 ignored;\n',
                 valid.replace('3 passed; 0 failed; 0 ignored;','2 passed; 0 failed; 1 ignored;'),
                 valid.replace(self.runner.FORMAL_TARGETS[0],'unrelated')]
        for output in invalid:
            with self.assertRaisesRegex(RuntimeError,'all three targets'):self.runner.formal_tests_passed(output)
            (run/'formal.log').write_text(output)
            with self.assertRaisesRegex(RuntimeError,'all three targets'):self.archive.validate_run(run,root)
        real_run=subprocess.run
        def zero(command,**kwargs):
            if command[0]!='cargo':return real_run(command,**kwargs)
            kwargs['stdout'].write(invalid[0])
            return subprocess.CompletedProcess(command,0)
        out=self.work/'zero-formal'
        with patch.object(self.runner.subprocess,'run',side_effect=zero), self.assertRaisesRegex(RuntimeError,'all three targets'):
            self.main('formal',out)
        record=json.loads((out/'commands.json').read_text())['commands'][-1]
        self.assertEqual(record['label'],'formal')
        self.assertEqual(record['process_exit'],0)
        self.assertEqual(record['status'],'error')

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
        history=root/'target/fr197-gpio-reruns/old'
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
        paths=[proof/'gpio_prove/status',proof/'gpio_cover/status',mutations/'control/status']
        paths += [mutations/fault/'status' for fault in ['event-clear-priority','input-is-output','set-ignores-strobes']]
        for path in paths:
            original=path.read_text()
            for bad in ['ERROR','UNKNOWN','PASS' if original.startswith('FAIL') else 'FAIL']:
                path.write_text(bad)
                with self.subTest(path=path,bad=bad), self.assertRaises(RuntimeError):self.archive.validate_run(run,root)
            path.write_text(original)
        # An unrelated old PASS must never rescue a failed associated proof.
        old=self.work/'unrelated-proof';old.mkdir();(old/'status').write_text('PASS')
        (proof/'gpio_prove/status').write_text('ERROR')
        with self.assertRaises(RuntimeError):self.archive.validate_run(run,root)

    def test_missing_selected_artifact_cannot_use_history(self):
        root,run,records=self.complete_run()
        victim=Path(records['backends']['artifact_roots'][0])/'firrtl.vcd'
        victim.unlink()
        with self.assertRaisesRegex(RuntimeError,'missing/empty evidence'):self.archive.validate_run(run,root)

    def test_exact_zero_ignored_and_wrong_targets_fail(self):
        target='p1_gpio_firrtl_chisel_same_independent_vectors'
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
                    (logdir/'128-4-build-active.log').write_text('growing logs')
                    (logdir/'128-4-build-old-source.json').write_text('historical JSON changes')
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
                for name in ['gpio-csr-registers.h','gpio-csr-registers.md']:
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
