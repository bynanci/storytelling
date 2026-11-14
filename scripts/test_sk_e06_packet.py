#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Tests for packet integrity; these never evaluate a model or participant."""
import copy
import hashlib
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

from sk_e06_packet import PacketError, load_packet, validate_packet

ROOT=Path(__file__).resolve().parents[1]
PACKET=ROOT/"docs/research/evaluations/sk-e06"

class PacketTests(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name)/"packet"
        shutil.copytree(PACKET,self.root)
        self.packet=load_packet(self.root)

    def rewrite(self,name,value):
        path=self.root/name
        path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n',encoding='utf-8', newline='\n')
        self.packet['artifacts_sha256'][name]=hashlib.sha256(path.read_bytes()).hexdigest()

    def first_case(self):
        name=self.packet['case_files'][0]
        return name,json.loads((self.root/name).read_text(encoding='utf-8'))

    def test_reference_packet_is_complete_without_observed_results(self):
        result=validate_packet(self.packet,self.root)
        self.assertEqual(result['case_count'],5)
        self.assertEqual(result['scope'],'reference_packet_integrity_only')
        self.assertIsNone(self.packet['efficacy_conclusion'])
        self.assertEqual(self.packet['observed_execution_records'],[])

    def test_rejects_relabeling_authored_examples_as_observed_execution(self):
        bad=copy.deepcopy(self.packet)
        bad['record_kind']='observed_model_run'
        with self.assertRaises(PacketError): validate_packet(bad,self.root)
        bad=copy.deepcopy(self.packet)
        bad['observed_execution_records']=[{'status':'passed'}]
        with self.assertRaises(PacketError): validate_packet(bad,self.root)

    def test_rejects_unversioned_or_unpinned_packet(self):
        bad=copy.deepcopy(self.packet)
        bad['schema_version']='future'
        with self.assertRaises(PacketError): validate_packet(bad,self.root)
        bad=copy.deepcopy(self.packet)
        bad['skill_snapshot']['commit']='unknown'
        with self.assertRaises(PacketError): validate_packet(bad,self.root)

    def test_rejects_missing_artifact_or_changed_exact_bytes(self):
        name=self.packet['case_files'][0]
        (self.root/name).write_bytes((self.root/name).read_bytes()+b' ')
        with self.assertRaises(PacketError): validate_packet(self.packet,self.root)
        (self.root/name).unlink()
        with self.assertRaises(PacketError): validate_packet(self.packet,self.root)

    def test_rejects_path_escape_before_reading_external_artifacts(self):
        bad=copy.deepcopy(self.packet)
        name=bad['case_files'][0]
        bad['case_files'][0]='../outside.json'
        bad['artifacts_sha256']['../outside.json']=bad['artifacts_sha256'].pop(name)
        with self.assertRaisesRegex(PacketError, "artifact path escapes"): validate_packet(bad,self.root)

    def test_rejects_duplicate_case_ids_or_missing_required_pattern(self):
        name,case=self.first_case()
        second=json.loads((self.root/self.packet['case_files'][1]).read_text(encoding='utf-8'))
        case['case_id']=second['case_id']
        self.rewrite(name,case)
        with self.assertRaises(PacketError): validate_packet(self.packet,self.root)

    def test_rejects_missing_locale_and_dangling_source_revision(self):
        name,case=self.first_case()
        case['locales'].pop('zh-TW')
        self.rewrite(name,case)
        with self.assertRaisesRegex(PacketError, "case locales"): validate_packet(self.packet,self.root)
        case=json.loads((PACKET/name).read_text(encoding='utf-8'))
        case['source_references'][0]='MISSING@99'
        self.rewrite(name,case)
        with self.assertRaisesRegex(PacketError, 'source reference/revision'): validate_packet(self.packet,self.root)

    def test_rejects_observation_field_and_broken_reference_literal_assertions(self):
        name,case=self.first_case()
        case['locales']['en']['observed_output']='fabricated capture'
        self.rewrite(name,case)
        with self.assertRaisesRegex(PacketError, 'case en'): validate_packet(self.packet,self.root)
        case=json.loads((PACKET/name).read_text(encoding='utf-8'))
        token=case['locales']['en']['reference_literal_checks']['must_contain'][0]
        case['locales']['en']['reference_output']=case['locales']['en']['reference_output'].replace(token,'REMOVED')
        self.rewrite(name,case)
        with self.assertRaisesRegex(PacketError, 'missing a required literal'): validate_packet(self.packet,self.root)

    def test_blank_form_cannot_prepopulate_results_or_omit_raw_capture_fields(self):
        name=self.packet['manual_form']
        form=json.loads((self.root/name).read_text(encoding='utf-8'))
        form['review']['instruction_conformance']='pass'
        self.rewrite(name,form)
        with self.assertRaisesRegex(PacketError, 'must remain blank'): validate_packet(self.packet,self.root)
        form=json.loads((PACKET/name).read_text(encoding='utf-8'))
        del form['raw_output']['verbatim']
        self.rewrite(name,form)
        with self.assertRaisesRegex(PacketError, 'manual form raw_output'): validate_packet(self.packet,self.root)

    def test_raw_duplicate_fields_and_nonfinite_values_reject(self):
        path=self.root/'packet-v1.json'
        path.write_bytes(b'{"packet_id":"one","packet_id":"two"}')
        with self.assertRaisesRegex(PacketError, 'duplicate field'):
            load_packet(self.root)
        path.write_bytes(b'{"metric":NaN}')
        with self.assertRaisesRegex(PacketError, 'non-finite constant'):
            load_packet(self.root)

    def test_cli_reports_integrity_only(self):
        p=subprocess.run([sys.executable,str(ROOT/'scripts/sk_e06_packet.py'),str(PACKET)],capture_output=True,text=True,check=True)
        result=json.loads(p.stdout)
        self.assertEqual(result['scope'],'reference_packet_integrity_only')
        self.assertEqual(result['case_count'],5)

if __name__=='__main__': unittest.main()
