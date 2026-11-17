#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Real-entry tests for synthetic capture; never participant evidence."""
import base64
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]
SCRIPT = Path(__file__).with_name('replay.py')
MUTANT = Path(__file__).with_name('mutant_cli.py')
BINARY = None
if __name__ == '__main__' and len(sys.argv) > 1 and not sys.argv[1].startswith('-'):
    BINARY = str(Path(sys.argv.pop(1)).resolve())
BASE = '06164f19a8b3614f438dd2f97ef810c94e64c3d3'
VALID_INPUTS = {
    'decimal-25': {'unit': 'seconds', 'start': '2.5', 'end': '4.5'},
    'decimal-01': {'unit': 'seconds', 'start': '0', 'end': '0.1'},
    'frames-30': {'unit': 'frames', 'start_frame': '75', 'end_frame': '135',
                  'fps': {'numerator': '30', 'denominator': '1'}, 'timing': 'constant'},
    'frames-ntsc': {'unit': 'frames', 'start_frame': '0', 'end_frame': '150',
                    'fps': {'numerator': '30000', 'denominator': '1001'}, 'timing': 'constant'},
    'declaration-complete': {'unit': 'seconds', 'start': '2.5', 'end': '4.5'},
}


class RehearsalCaptureTests(unittest.TestCase):
    def invoke(self, binary, output, source=BASE):
        return subprocess.run([sys.executable, str(SCRIPT), '--cli', binary,
                               '--source-commit', source, '--output', str(output)],
                              capture_output=True, timeout=30)

    def assert_mutant_rejected(self, case_id, fault):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            config = {'source_range': VALID_INPUTS[case_id],
                      'second_status': 'mapped' if case_id == 'declaration-complete' else 'unresolved',
                      'fault': fault}
            config_path = directory / 'fault.json'
            config_path.write_text(json.dumps(config), encoding='utf-8')
            output = directory / 'receipt.json'
            result = subprocess.run([sys.executable, str(MUTANT), str(config_path), str(SCRIPT),
                                     '--cli', BINARY, '--source-commit', BASE, '--output', str(output)],
                                    capture_output=True, timeout=30)
            self.assertEqual(result.returncode, 1, result.stdout.decode(errors='replace'))
            report = json.loads(output.read_bytes())
            self.assertEqual(report['status'], 'failed')
            self.assertEqual(report['failure'], 'wrong_canonical_declarations')
            self.assertEqual(report['cases'][-1]['case_id'], case_id)
            self.assertTrue(report['cases'][-1]['calls'][0]['stdout_base64'])

    @unittest.skipUnless(BINARY, 'explicit feature-enabled native binary required')
    def test_consistently_wrong_endpoints_fail_capture(self):
        for case_id in VALID_INPUTS:
            for mapping_index in range(2 if case_id == 'declaration-complete' else 1):
                for endpoint in ('start', 'end'):
                    with self.subTest(case=case_id, mapping=mapping_index, endpoint=endpoint):
                        self.assert_mutant_rejected(case_id, {'kind': 'range',
                                                            'mapping_index': mapping_index,
                                                            'endpoint': endpoint})

    @unittest.skipUnless(BINARY, 'explicit feature-enabled native binary required')
    def test_consistently_changed_unit_refs_fail_capture(self):
        for kind in ('mapping_refs', 'selected_refs'):
            with self.subTest(kind=kind):
                self.assert_mutant_rejected('decimal-25', {'kind': kind})

    @unittest.skipUnless(BINARY, 'explicit feature-enabled native binary required')
    def test_consistently_changed_unresolved_reason_fails_capture(self):
        for case_id in VALID_INPUTS:
            if case_id != 'declaration-complete':
                with self.subTest(case=case_id):
                    self.assert_mutant_rejected(case_id, {'kind': 'unresolved_reason'})

    @unittest.skipUnless(BINARY, 'explicit feature-enabled native binary required')
    def test_real_entry_captures_exact_cases_without_human_results(self):
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp) / 'receipt.json'
            result = self.invoke(BINARY, output)
            self.assertEqual(result.returncode, 0, result.stderr.decode(errors='replace'))
            report = json.loads(output.read_bytes())
            self.assertEqual(report['status'], 'passed')
            self.assertEqual(report['evidence_kind'], 'synthetic_machine_replay')
            self.assertEqual(report['human_participants'], 0)
            self.assertIsNone(report['human_completion_rate'])
            self.assertIsNone(report['author_elapsed_seconds'])
            self.assertIsNone(report['maintainer_acceptance'])
            self.assertEqual(report['host_observations'], 0)
            self.assertEqual(report['case_count'], 8)
            self.assertEqual(report['cli_calls'], 37)
            self.assertEqual(len(report['cases']), 8)
            by_id = {row['case_id']: row for row in report['cases']}
            self.assertEqual(by_id['decimal-25']['preview']['coverage'],
                             by_id['frames-30']['preview']['coverage'])
            expected_ranges = {
                'decimal-25': ({'numerator': '5', 'denominator': '2'}, {'numerator': '9', 'denominator': '2'}),
                'decimal-01': ({'numerator': '0', 'denominator': '1'}, {'numerator': '1', 'denominator': '10'}),
                'frames-30': ({'numerator': '5', 'denominator': '2'}, {'numerator': '9', 'denominator': '2'}),
                'frames-ntsc': ({'numerator': '0', 'denominator': '1'}, {'numerator': '1001', 'denominator': '200'}),
                'declaration-complete': ({'numerator': '5', 'denominator': '2'}, {'numerator': '9', 'denominator': '2'}),
            }
            for case_id, (start, end) in expected_ranges.items():
                for mapping in by_id[case_id]['preview']['coverage']['mappings']:
                    if mapping['status'] == 'mapped':
                        self.assertEqual(mapping['source_range'], {'start': start, 'end': end})
            self.assertEqual(by_id['decimal-01']['preview']['coverage']['mappings'][0]['source_range']['end'],
                             {'numerator': '1', 'denominator': '10'})
            self.assertEqual(by_id['frames-ntsc']['preview']['coverage']['mappings'][0]['source_range']['end'],
                             {'numerator': '1001', 'denominator': '200'})
            for case_id in ['decimal-25', 'decimal-01', 'frames-30', 'frames-ntsc']:
                row = by_id[case_id]['preview']
                self.assertEqual(row['scope'], 'declarations_only')
                self.assertFalse(row['validation']['ready_for_planning'])
                self.assertEqual(row['coverage']['mappings'][1]['unit_ref'], {'unit_id': 'S03', 'revision': 1})
                self.assertEqual(row['coverage']['mappings'][1]['status'], 'unresolved')
            self.assertTrue(by_id['declaration-complete']['preview']['validation']['ready_for_planning'])
            self.assertEqual(by_id['declaration-complete']['preview']['scope'], 'declarations_only')
            self.assertEqual(by_id['decimal-comma']['preview']['error']['code'], 'draft_time.invalid_decimal')
            self.assertEqual(by_id['beyond-duration']['preview']['error']['code'], 'video_coverage.invalid_range')
            self.assertEqual(by_id['vfr-frames']['preview']['error']['code'], 'draft_time.frame_timing_not_supported')
            for row in report['cases']:
                self.assertTrue(row['input_unchanged'])
                self.assertTrue(row['deterministic_file_stdin'])
                first = row['calls'][0]
                raw = base64.b64decode(first['stdout_base64'])
                self.assertEqual(hashlib.sha256(raw).hexdigest(), first['stdout_sha256'])
                self.assertEqual(json.loads(raw), row['preview'])
            self.assertTrue(report['binary_unchanged'])
            self.assertTrue(report['source_fixtures_unchanged'])

    def test_existing_receipt_is_never_overwritten(self):
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp) / 'existing.json'
            output.write_bytes(b'original observation')
            result = self.invoke(sys.executable, output)
            self.assertEqual(result.returncode, 2)
            self.assertEqual(output.read_bytes(), b'original observation')
            self.assertIn(b'already exists', result.stderr)

    def test_wrong_executable_preserves_failed_machine_evidence(self):
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp) / 'failed.json'
            result = self.invoke(sys.executable, output)
            self.assertEqual(result.returncode, 1)
            report = json.loads(output.read_bytes())
            self.assertEqual(report['status'], 'failed')
            self.assertEqual(report['human_participants'], 0)
            self.assertEqual(len(report['cases']), 1)
            self.assertEqual(report['case_count'], 1)
            self.assertEqual(report['planned_case_count'], 8)
            self.assertNotEqual(report['cases'][0]['calls'][0]['exit_code'], 0)
            self.assertTrue(report['cases'][0]['calls'][0]['stderr_base64'])

    def test_invalid_source_identity_fails_before_creating_receipt(self):
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp) / 'invalid.json'
            result = self.invoke(sys.executable, output, 'not-a-commit')
            self.assertEqual(result.returncode, 2)
            self.assertFalse(output.exists())


if __name__ == '__main__':
    unittest.main()
