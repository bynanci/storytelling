# SPDX-License-Identifier: MIT
"""Synthetic recorder tests. No test creates a human study observation."""
import hashlib
import io
import importlib.util
import json
import os
import sqlite3
import subprocess
import sys
import tempfile
import concurrent.futures
import threading
import time
from contextlib import redirect_stdout, redirect_stderr
import unittest
from pathlib import Path
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
SCRIPT = HERE / 'record.py'
ROOT = HERE.parents[2]
SOURCE = 'ce73836caa86502933f538821fdbf5dac92c1224'
MATERIALS = '8e2c08094f6bce70a7507e11ed84265bd98e3820'


class RecorderTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.round = self.base / 'round'
        self.cli = Path(sys.executable).resolve()
        self.receipt = self.base / 'receipt.json'
        # Deliberately synthetic build declaration; no source attestation implied.
        self.receipt.write_text(json.dumps({
            'protocol': 'n07-native-notes-v3-proposed', 'human_evidence': False,
            'source_commit': SOURCE, 'materials_commit': MATERIALS,
            'binary_sha256': hashlib.sha256(self.cli.read_bytes()).hexdigest(),
            'lock_sha256': '0' * 64, 'raw_report_sha256': '0' * 64,
            'engineering_results_sha256': '0' * 64, 'rustc': 'synthetic',
            'cargo': 'synthetic'}))

    def module(self):
        spec = importlib.util.spec_from_file_location('recorder_test_subject', SCRIPT)
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)
        return mod

    def command(self, *args, input=None):
        return subprocess.run([sys.executable, str(SCRIPT), *map(str, args)],
                              input=input, capture_output=True, text=True, encoding='utf-8', timeout=20)

    def initialize(self):
        m = self.module()
        m.initialize(self.round, self.cli, self.receipt, 'rehearsal', None)
        return m

    def enroll(self, m, code='P01', slot=1, exposure=False, replacement=None):
        p = self.base / (code + '.json')
        p.write_text(json.dumps({'schema_version': 1, 'screened_at': '2026-11-12T00:00:00+00:00',
                                'prior_exposure': exposure, 'cli_json_experience': 'synthetic',
                                'facilitator_code': 'F01'}))
        m.enroll(self.round, code, slot, p, replacement)

    def start(self, m, code='P01', condition=None):
        state = m.inspect(self.round)
        if condition is None:
            slot = state['participants'][code]['slot']
            condition = state['config']['allocation'][slot - 1][0]
        return m.start(self.round, code, condition)

    def test_cli_rehearsal_init_and_nonoverwrite(self):
        args = ('init', '--round', self.round, '--cli', self.cli,
                '--receipt', self.receipt, '--mode', 'rehearsal')
        first = self.command(*args)
        self.assertEqual(first.returncode, 0, first.stderr)
        before = (self.round / 'records.sqlite3').read_bytes()
        second = self.command(*args)
        self.assertNotEqual(second.returncode, 0)
        self.assertEqual((self.round / 'records.sqlite3').read_bytes(), before)
        value = json.loads(self.command('status', '--round', self.round).stdout)
        self.assertEqual(value['config']['allocation'].count('AB'), 3)
        self.assertEqual(value['config']['allocation'].count('BA'), 2)
        self.assertEqual(value['participants'], {})
        self.assertEqual(value['recommendation'], '待真人證據')

    def test_trial_requires_real_agreement_record(self):
        m = self.module()
        with self.assertRaises(ValueError):
            m.initialize(self.round, self.cli, self.receipt, 'trial', None)
        self.assertFalse(self.round.exists())

    def test_agreement_binds_exact_recorder_and_preserves_declaration_not_authentication(self):
        m = self.module()
        a = {'schema_version': 1, 'protocol_sha256': m.PROTOCOL_SHA,
             'protocol_commit': 'fe2b145250b42564bf5b4d781ced5a95e8a8f9bf',
             'recorder_sha256': '0' * 64, 'adopted': True,
             'recorded_at': '2026-11-11T00:00:00+00:00',
             'maintainer_label': 'synthetic declaration, no actual approval', 'round_id': 'TEST'}
        file = self.base / 'agreement.json'; file.write_text(json.dumps(a))
        with self.assertRaises(ValueError):
            m.initialize(self.round, self.cli, self.receipt, 'trial', file)
        a['recorder_sha256'] = hashlib.sha256(SCRIPT.read_bytes()).hexdigest()
        file.write_text(json.dumps(a))
        m.initialize(self.round, self.cli, self.receipt, 'trial', file)
        self.assertFalse(m.inspect(self.round)['config']['agreement_authenticated'])

    def test_receipt_duplicate_unknown_and_mismatched_binary_rejected(self):
        m = self.module()
        original = self.receipt.read_text()
        for text in ['{"protocol":1,"protocol":2}',
                     original[:-1] + ',"extra":true}',
                     original.replace(hashlib.sha256(self.cli.read_bytes()).hexdigest(), '1' * 64)]:
            self.receipt.write_text(text)
            with self.assertRaises(ValueError):
                m.initialize(self.round, self.cli, self.receipt, 'rehearsal', None)
            self.assertFalse(self.round.exists())

    def test_exposure_preserved_and_start_refused(self):
        m = self.initialize()
        self.enroll(m, exposure=True)
        with self.assertRaises(ValueError):
            self.start(m)
        self.assertTrue(m.inspect(self.round)['participants']['P01']['prior_exposure'])

    def test_concurrent_start_and_duplicate_arm_refused(self):
        m = self.initialize()
        self.enroll(m)
        sid = self.start(m)
        with self.assertRaises(ValueError):
            self.start(m)
        m.end(self.round, sid, 'submitted', 10, [], None)
        with self.assertRaises(ValueError):
            self.start(m)

    def test_deadline_cannot_be_reported_as_submission(self):
        m = self.initialize()
        self.enroll(m)
        sid = self.start(m)
        m.end(self.round, sid, 'submitted', 480.01, [], None)
        record = m.inspect(self.round)['sessions'][sid]
        self.assertEqual(record['status'], 'timeout')
        self.assertEqual(record['analysis_seconds'], 480)
        self.assertFalse(record['completed'])
        self.assertEqual(record['elapsed_seconds'], 480.01)

    def test_recovery_is_unknown_and_late_end_cannot_overwrite(self):
        m = self.initialize()
        self.enroll(m)
        sid = self.start(m)
        m.recover(self.round, sid, 'synthetic process interruption')
        with self.assertRaises(ValueError):
            m.end(self.round, sid, 'submitted', 20, [], None)
        record = m.inspect(self.round)['sessions'][sid]
        self.assertEqual(record['status'], 'interrupted')
        self.assertIsNone(record['analysis_seconds'])
        self.assertIsNone(record['completed'])

    def test_replacement_retains_prior_start(self):
        m = self.initialize()
        self.enroll(m)
        sid = self.start(m)
        m.end(self.round, sid, 'withdrawn', 12, [], 'synthetic withdrawal')
        self.enroll(m, code='P02', replacement='P01')
        state = m.inspect(self.round)
        self.assertEqual(len(state['sessions']), 1)
        self.assertEqual(state['participants']['P02']['replacement_for'], 'P01')
        self.assertEqual(state['sessions'][sid]['status'], 'withdrawn')

    def test_verbatim_attachment_export_and_overwrite_refusal(self):
        m = self.initialize()
        self.enroll(m)
        sid = self.start(m)
        answer = self.base / 'answers.txt'
        raw = '意外回答：B 比 A 早。\r\n  不確定「敲門」。\n'.encode()
        answer.write_bytes(raw)
        m.attach(self.round, sid, 'feedback', answer)
        answer.write_text('later edit')
        out = self.base / 'export'
        m.export(self.round, out)
        self.assertEqual((out / 'blobs' / hashlib.sha256(raw).hexdigest()).read_bytes(), raw)
        with self.assertRaises(FileExistsError):
            m.export(self.round, out)

    def test_disallowed_command_and_notes_arm_never_launch(self):
        m = self.initialize()
        self.enroll(m)
        sid = self.start(m)
        with patch.object(m.subprocess, 'Popen') as launch:
            with self.assertRaises(ValueError):
                m.invoke(self.round, sid, ['process', '--scenario'], 30)
            if m.inspect(self.round)['sessions'][sid]['condition'] == 'A':
                with self.assertRaises(ValueError):
                    m.invoke(self.round, sid, ['render-narrative', str(self.receipt)], 30)
            launch.assert_not_called()

    def test_bounded_file_read_and_strict_json(self):
        m = self.initialize()
        f = self.base / 'large.txt'
        f.write_bytes(b'x' * (m.MAX_INPUT + 1))
        with self.assertRaises(ValueError):
            m.read_bytes(f)
        bad = self.base / 'bad.json'
        bad.write_text('{"value":NaN}')
        with self.assertRaises(ValueError):
            m.read_json(bad)
        for raw in [b'{"value":1e9999}', '{"value":1}'.encode('utf-16')]:
            bad.write_bytes(raw)
            with self.assertRaises(ValueError):
                m.read_json(bad)

    def test_recorded_hash_detects_damaged_blob(self):
        m = self.initialize()
        self.enroll(m)
        sid = self.start(m)
        answer = self.base / 'answer.txt'; answer.write_bytes(b'original')
        m.attach(self.round, sid, 'feedback', answer)
        db = sqlite3.connect(self.round / 'records.sqlite3')
        try:
            db.execute('UPDATE blobs SET bytes=?', (b'corrupted',))
            db.commit()
        finally:
            db.close()
        with self.assertRaises(ValueError):
            m.export(self.round, self.base / 'export')

    def test_two_simultaneous_starts_have_only_one_winner(self):
        m = self.initialize()
        self.enroll(m)
        gate = threading.Barrier(2)
        def attempt():
            gate.wait()
            try:
                return self.start(m)
            except ValueError:
                return None
        with concurrent.futures.ThreadPoolExecutor(2) as executor:
            results = list(executor.map(lambda _: attempt(), range(2)))
        self.assertEqual(sum(v is not None for v in results), 1)
        self.assertEqual(len(m.inspect(self.round)['sessions']), 1)

    def test_capture_preserves_binary_streams_and_timeout_partial(self):
        m = self.module()
        out, err = self.base / 'stdout', self.base / 'stderr'
        result = m.capture([sys.executable, '-c',
                            'import sys;sys.stdout.buffer.write(bytes([255,0,13,10]));sys.stderr.write("error\\n")'],
                           out, err, 10)
        self.assertEqual(out.read_bytes(), bytes([255, 0, 13, 10]))
        self.assertEqual(err.read_bytes(), b'error\n' if os.name != 'nt' else b'error\r\n')
        self.assertEqual(result['exit_code'], 0)
        out2, err2 = self.base / 'partial', self.base / 'partial-err'
        result = m.capture([sys.executable, '-c',
                            'import sys,time;sys.stdout.write("partial");sys.stdout.flush();time.sleep(10)'],
                           out2, err2, 2)
        self.assertTrue(result['timed_out'])
        self.assertEqual(out2.read_bytes(), b'partial')

    def test_output_limit_is_explicit_and_bounded(self):
        m = self.module()
        out, err = self.base / 'stdout', self.base / 'stderr'
        with patch.object(m, 'MAX_OUTPUT', 128):
            result = m.capture([sys.executable, '-c', 'print("x" * 10000)'], out, err, 10)
        self.assertTrue(result['output_limited'])
        self.assertEqual(len(out.read_bytes()), 128)

    def native_start(self, m):
        slot = m.inspect(self.round)['config']['allocation'].index('BA') + 1
        self.enroll(m, slot=slot)
        return self.start(m, condition='B')

    def test_intent_is_durable_and_late_capture_does_not_end_new_session(self):
        m = self.initialize()
        sid = self.native_start(m)
        def fake_capture(argv, out, err, timeout):
            pending = m.inspect(self.round)
            self.assertEqual(len(pending['calls']), 1)
            self.assertEqual(next(iter(pending['calls'].values()))['status'], 'unknown')
            Path(out).write_bytes(b'raw'); Path(err).write_bytes(b'')
            m.recover(self.round, sid, 'synthetic interleaving')
            current_slot = pending['participants']['P01']['slot']
            self.enroll(m, code='P02', slot=2 if current_slot == 1 else 1)
            self.start(m, code='P02')
            return {'exit_code': 0, 'timed_out': False, 'launch_error': None,
                    'output_limited': False, 'capture_error': None, 'process_ms': 1}
        with patch.object(m, 'capture', side_effect=fake_capture):
            result = m.invoke(self.round, sid, ['render-narrative', str(self.receipt)], 10)
        self.assertTrue(result['late_after_session_end'])
        self.assertEqual(m.inspect(self.round)['sessions'][sid]['status'], 'interrupted')
        newer = [s for s in m.inspect(self.round)['sessions'].values() if s['participant'] == 'P02']
        self.assertEqual(newer[0]['status'], 'active')

    def test_crash_preserves_unknown_call_and_exported_partial_without_retry(self):
        m = self.initialize()
        sid = self.native_start(m)
        def interrupted(argv, out, err, timeout):
            Path(out).write_bytes(b'partial-original')
            raise RuntimeError('synthetic process crash')
        with patch.object(m, 'capture', side_effect=interrupted) as capture:
            with self.assertRaises(RuntimeError):
                m.invoke(self.round, sid, ['render-narrative', str(self.receipt)], 10)
            m.recover(self.round, sid, 'synthetic crash observed')
            m.export(self.round, self.base / 'export')
            capture.assert_called_once()
        report = json.loads((self.base / 'export/report.json').read_text())
        self.assertEqual(next(iter(report['calls'].values()))['status'], 'unknown')
        partial = report['raw_partial_streams'][0]
        self.assertFalse(partial['complete'])
        self.assertEqual((self.base / 'export/blobs' / partial['sha256']).read_bytes(), b'partial-original')

    def test_eof_records_interruption_not_completion(self):
        m = self.initialize()
        self.enroll(m)
        condition = m.inspect(self.round)['participants']['P01']['assigned_order'][0]
        result = self.command('session', '--round', self.round, '--participant', 'P01',
                              '--condition', condition, input='')
        self.assertEqual(result.returncode, 1)
        session = next(iter(m.inspect(self.round)['sessions'].values()))
        self.assertEqual(session['status'], 'interrupted')
        self.assertIsNone(session['analysis_seconds'])

    def test_workspace_copies_matched_words_without_key_and_never_overwrites(self):
        m = self.initialize()
        self.enroll(m)
        a, b = self.base / 'notes', self.base / 'native'
        m.workspace(self.round, 'P01', 'A', a)
        m.workspace(self.round, 'P01', 'B', b)
        self.assertEqual((a / 'shared/narrative-drafts.md').read_bytes(),
                         (b / 'shared/narrative-drafts.md').read_bytes())
        self.assertFalse((a / 'cli').exists())
        self.assertFalse((b / 'facilitator').exists())
        self.assertTrue((b / 'cli/working-document.json').exists())
        with self.assertRaises(FileExistsError):
            m.workspace(self.round, 'P01', 'A', a)

    def test_live_timer_expires_while_waiting_for_input(self):
        m = self.initialize()
        self.enroll(m)
        condition = m.inspect(self.round)['participants']['P01']['assigned_order'][0]
        class IdleInput:
            def readline(self, limit):
                time.sleep(0.3)
                return ''
        with patch.object(m, 'LIMIT', 0.05), patch.object(m.sys, 'stdin', IdleInput()), \
                redirect_stdout(io.StringIO()), redirect_stderr(io.StringIO()):
            result = m.session_loop(self.round, 'P01', condition)
        self.assertEqual(result, 1)
        session = next(iter(m.inspect(self.round)['sessions'].values()))
        self.assertEqual(session['status'], 'timeout')
        self.assertFalse(session['completed'])

    def test_real_cli_entry_preserves_raw_native_report(self):
        native = ROOT / 'target' / 'debug' / ('storytelling.exe' if os.name == 'nt' else 'storytelling')
        if not native.exists():
            self.skipTest('build the workspace before native integration replay')
        receipt = json.loads(self.receipt.read_text())
        receipt['binary_sha256'] = hashlib.sha256(native.read_bytes()).hexdigest()
        self.receipt.write_text(json.dumps(receipt))
        # Synthetic rehearsal receipt intentionally does not claim this test build is ce73836.
        result = self.command('init', '--round', self.round, '--cli', native,
                              '--receipt', self.receipt, '--mode', 'rehearsal')
        self.assertEqual(result.returncode, 0, result.stderr)
        m = self.module()
        slot = m.inspect(self.round)['config']['allocation'].index('BA') + 1
        self.enroll(m, slot=slot)
        copy = ROOT / 'examples/evaluation/n07-pilot/participant-kit/cli/copy-template.json'
        ledger = ROOT / 'examples/evaluation/n07-pilot/participant-kit/cli/evidence-ledger.json'
        before = copy.read_bytes()
        commands = 'audit ' + json.dumps([str(copy), str(ledger)]) + '\nsubmit ' + json.dumps([str(copy)]) + '\n'
        result = self.command('session', '--round', self.round, '--participant', 'P01',
                              '--condition', 'B', input=commands)
        self.assertEqual(result.returncode, 0, result.stderr)
        state = m.inspect(self.round)
        call = next(iter(state['calls'].values()))
        raw = (self.round / call['stdout_file']).read_bytes()
        report = json.loads(raw)
        claim = next(c for c in report['commercial_copy']['claims'] if c['id'] == 'unsupported-program-claim')
        self.assertEqual(claim['reference_status'], 'missing')
        self.assertEqual(claim['evidence_refs'], [])
        self.assertEqual(call['stdout_sha256'], hashlib.sha256(raw).hexdigest())
        self.assertEqual(copy.read_bytes(), before)
        self.assertIsNone(state['comparable_pairs'])


if __name__ == '__main__':
    unittest.main()
