#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Actual failed runs retain honest evidence and still check the source checkout."""
import hashlib
import json
import os
from pathlib import Path
import sys
import tempfile
import time
import unittest
from process import run_bounded

ROOT=Path(__file__).resolve().parent

class FailedEvidenceChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.paths={key:Path(os.environ['PLANNER_SHADOW_'+key]) for key in ('SOURCE','NODE','TSX')}
        if not all(p.is_absolute() and p.exists() for p in cls.paths.values()):raise ValueError('explicit shadow test tool paths required')
    def invoke(self,cli,root):
        argv=[sys.executable,str(ROOT/'run.py'),'--source',str(self.paths['SOURCE']),'--node',str(self.paths['NODE']),'--tsx',str(self.paths['TSX']),'--rust-cli',str(cli),'--run-root',str(root),'--cases-per-domain','1']
        receipt=run_bounded(argv,None,time.monotonic()+20,timeout=20)
        self.assertEqual(receipt['exit'],1);self.assertEqual(receipt['stderr'],b'')
        summary=json.loads(receipt['stdout']);self.assertFalse(summary['ok'])
        evidence=json.loads((root/'evidence.json').read_text());self.assertFalse(evidence['allPassed']);self.assertIsNotNone(evidence['executionError'])
        self.assertTrue(evidence['source']['postcheck']);self.assertTrue(evidence['source']['clean_after'])
        for name,info in evidence['artifacts'].items():
            data=(root/name).read_bytes();self.assertEqual(len(data),info['bytes']);self.assertEqual(hashlib.sha256(data).hexdigest(),info['sha256'])
        return evidence
    def test_wrong_native_version_preserves_unexecuted_unknown_tools_and_failed_evidence(self):
        with tempfile.TemporaryDirectory(prefix='shadow-negative-') as parent:
            evidence=self.invoke(Path(sys.executable).resolve(),Path(parent)/'run')
            self.assertEqual(evidence['summary']['executed'],0);self.assertEqual(evidence['tools'],{})
            self.assertEqual(evidence['cases'],[]);self.assertEqual(evidence['executionError']['stage'],'preflight')
            self.assertIn('native CLI version',evidence['executionError']['message'])
    def test_arbitrary_native_errors_never_become_successful_shadow(self):
        with tempfile.TemporaryDirectory(prefix='shadow-negative-') as parent:
            cli=Path(parent)/'known-failed-native'
            cli.write_text('#!'+sys.executable+'\nimport sys\nif sys.argv[1:]==["--version"]:\n print("storytelling 0.1.0-dev.0")\nelse:\n print(\'{"ok":false,"error":{"code":"planner.no_selection","path":"$.candidates","conflicts":[]}}\')\n sys.exit(1)\n');cli.chmod(0o755)
            evidence=self.invoke(cli,Path(parent)/'run')
            self.assertEqual(evidence['summary']['executed'],18);self.assertGreater(evidence['summary']['unclassified'],0)
            self.assertEqual(evidence['executionError']['code'],'semantic-mismatch')
            self.assertTrue(any(not row['passed'] and row['classification']=='unclassified' for row in evidence['cases']))

if __name__=='__main__':unittest.main()
