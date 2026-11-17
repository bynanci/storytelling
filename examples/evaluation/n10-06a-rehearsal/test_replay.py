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
BINARY = None
if len(sys.argv) > 1 and not sys.argv[1].startswith('-'):
    BINARY = str(Path(sys.argv.pop(1)).resolve())
BASE = '06164f19a8b3614f438dd2f97ef810c94e64c3d3'


class RehearsalCaptureTests(unittest.TestCase):
    def invoke(self, binary, output, source=BASE):
        return subprocess.run([sys.executable, str(SCRIPT), '--cli', binary,
                               '--source-commit', source, '--output', str(output)],
                              capture_output=True, timeout=30)

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
