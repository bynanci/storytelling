#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Complete-baseline mutation tests; no source/native runtime dependency."""
import copy
import json
import unittest
from decimal import Decimal
from pathlib import Path
from cases import fixed_cases, make, asset
from cases import build_cases, validate_case
from compare import classify, strict_loads, validate_source_batch, numeric


def t(n, d=1):
    return {'numerator':str(n),'denominator':str(d)}


def baseline():
    native_input={'schema_version':1,'brief':{'schema_version':1,'brief_id':'required-test','intent':'Synthetic live shadow','assets':[{'id':'a'}],'manual_selected_asset_ids':['a'],'hard_constraints':[],'preferences':{'preferred_asset_ids':[],'target_duration':t(1)}},'candidates':[{'asset_id':'a','content_hash':'a'*64,'duration':t(1),'source_range':{'start':t(0),'end':t(1)},'score':3}],'max_duration':t(2),'max_shots':1}
    source_input={'story':{'id':'required-test','title':'Synthetic story','targetDuration':1,'maxDuration':2,'beats':[{'id':'beat','title':'Sequence','selectedAssetIds':['a'],'constraints':[]}]},'assets':[{'id':'a','name':'Synthetic a','uri':'synthetic://shadow/a','mediaType':'video','duration':1,'rating':3,'tags':[],'state':{},'metadata':{}}]}
    case={'id':'required-test','disposition':'agreement','family':'required-fractional','source_input':source_input,'native_input':native_input}
    comp={'storyId':'required-test','duration':1,'tracks':[{'type':'video','clips':[{'assetId':'a','beatId':'beat','sourceIn':0,'sourceOut':1,'timelineStart':0,'timelineDuration':1}]}]}
    source={'id':'required-test','input_unchanged':True,'ok':True,'composition':comp}
    audit={'schema_version':1,'brief_id':'required-test','intent':'Synthetic live shadow','scope':'declared-constraint-audit-only','executable':False,'selection_planned':False,'duration_analysis':'not-performed','consistent':True,'manual_selected_asset_ids':['a'],'required_assets':[{'asset_id':'a','origins':['$.manual_selected_asset_ids[0]']}],'excluded_assets':[],'precedence_edges':[],'preferences':native_input['brief']['preferences'],'conflicts':[],'warnings':[]}
    report={'schema_version':1,'profile':'storytelling.fixed-window-planner.v1','scope':'declared-fixed-window-plan-only','executable':False,'source_verified':False,'host_verified':False,'selection_planned':True,'duration_analysis':'exact-fixed-windows','content_clock_rate':'1','total_duration':t(1),'target_status':'met','brief_audit':audit,'plan':{'schema_version':1,'plan_id':'required-test','assets':[{'id':'a','content_hash':'a'*64,'duration':t(1)}],'shots':[{'id':'shot-000001','asset_id':'a','source_range':native_input['candidates'][0]['source_range']}]},'selections':[{'asset_id':'a','shot_id':'shot-000001','required':True,'declared_score':3,'timeline_start':t(0),'duration':t(1),'reason_paths':['$.brief.manual_selected_asset_ids[0]','$.candidates[0].source_range']}],'skipped_candidates':[]}
    native={'exit':0,'response':{'ok':True,'planning':report}}
    return case,source,native


class ShadowChecks(unittest.TestCase):
    def test_seeded_default_counts_order_and_bounds(self):
        cases=build_cases()
        self.assertEqual(len(cases),64)
        self.assertEqual(len({x['id'] for x in cases}),64)
        self.assertEqual(cases,build_cases())
        self.assertNotEqual(cases[:24],build_cases(123)[:24])
        self.assertEqual(len(build_cases(3,64)),144)
        for case in cases:validate_case(case)
        for value in (0,65):
            with self.assertRaisesRegex(ValueError,'cases-per-domain'):build_cases(1,value)
    def test_complete_actual_shape_baseline_agrees(self):
        c,s,n=baseline();r=classify(c,s,n)
        self.assertTrue(r['passed']);self.assertEqual(r['classification'],'agreement');self.assertEqual(r['differences'],[])
    def test_authority_windows_clock_reasons_and_placement_mutations_reject(self):
        for key in ['authority','source-window','clock','reasons','placement']:
            c,s,n=baseline();r=n['response']['planning']
            if key=='authority':r['host_verified']=True
            elif key=='source-window':r['plan']['shots'][0]['source_range']['end']=t(2)
            elif key=='clock':r['content_clock_rate']='2'
            elif key=='reasons':r['selections'][0]['reason_paths']=[]
            else:r['selections'][0]['timeline_start']=t(1)
            with self.subTest(key=key),self.assertRaises(ValueError):classify(c,s,n)
    def test_fractional_optional_budget_cannot_be_relabelled_as_shared_domain(self):
        c,s,n=baseline();c['family']='optional-ranking';c['native_input']['brief']['manual_selected_asset_ids']=[];c['native_input']['brief']['preferences']['target_duration']=t(3,2);c['source_input']['story']['beats'][0]['selectedAssetIds']=[];c['source_input']['story']['targetDuration']=1.5
        with self.assertRaisesRegex(ValueError,'optional'):validate_case(c)
    def test_arbitrary_two_errors_are_not_agreement(self):
        c,s,n=baseline();s={'id':c['id'],'input_unchanged':True,'ok':False,'error':{'name':'CompositionConstraintError','message':'arbitrary'}};n={'exit':1,'response':{'ok':False,'error':{'code':'planner.no_selection','path':'$.candidates','conflicts':[]}}}
        with self.assertRaises(ValueError):classify(c,s,n)
    def test_wrong_source_clip_order_and_extra_fields_reject(self):
        for key in ['id','extra']:
            c,s,n=baseline()
            if key=='id':s['composition']['tracks'][0]['clips'][0]['assetId']='other'
            else:s['composition']['surprise']=True
            with self.subTest(key=key),self.assertRaises(ValueError):classify(c,s,n)
    def test_duplicate_json_properties_reject(self):
        with self.assertRaisesRegex(ValueError,'duplicate'):strict_loads(b'{"a":1,"a":2}')
        self.assertEqual(strict_loads(b'{"a":1}'),{'a':1})
    def test_result_missing_duplicate_reordered_and_extra_shape_reject(self):
        c,s,n=baseline();doc={'results':[s]}
        self.assertEqual(validate_source_batch(doc,[c['id']]),[s])
        for rows in ([],[s,s],[dict(s,id='other')]):
            with self.subTest(rows=rows),self.assertRaises(ValueError):validate_source_batch({'results':rows},[c['id']])
    def test_probe_dispositions_and_expected_pairs_are_immutable(self):
        cases=build_cases()
        probes=[x for x in cases if x['disposition']!='agreement']
        self.assertEqual(len(probes),8)
        for original in probes:
            validate_case(original)
            changed=copy.deepcopy(original);changed['family']='arbitrary'
            with self.subTest(case=original['id']),self.assertRaises(ValueError):validate_case(changed)

    def test_complete_actual_probe_pairs_accept_and_reject_semantic_source_mutations(self):
        data=json.loads((Path(__file__).parent/'mutation-baselines.json').read_text())
        cases={c['id']:c for c in fixed_cases()}
        self.assertEqual(len(data['pairs']),8)
        for pair in data['pairs']:
            case=cases[pair['id']];result=classify(case,pair['source'],pair['native'])
            self.assertTrue(result['passed']);self.assertEqual(result['classification'],case['disposition'])
            source=copy.deepcopy(pair['source'])
            if not source['ok']:source['error']['message']+=' Changed.'
            elif pair['id']=='delta-empty':source['composition']={'storyId':case['id'],'duration':1,'tracks':[{'type':'video','clips':[{'assetId':'a','beatId':'beat','sourceIn':0,'sourceOut':1,'timelineStart':0,'timelineDuration':1}]}]}
            elif pair['id']=='unsupported-two-beats':
                clips=source['composition']['tracks'][0]['clips'];clips.reverse()
                for i,clip in enumerate(clips):clip['timelineStart']=i
            else:
                clip=source['composition']['tracks'][0]['clips'][0];clip['assetId']='b' if clip['assetId']=='a' else 'a'
            with self.subTest(case=case['id']),self.assertRaisesRegex(ValueError,'literal source outcome'):
                classify(case,source,pair['native'])
    def test_complete_unsupported_native_errors_cannot_be_replaced_by_other_errors(self):
        data=json.loads((Path(__file__).parent/'mutation-baselines.json').read_text());cases={c['id']:c for c in fixed_cases()}
        for pair in data['pairs']:
            if pair['native']['response']['ok']:continue
            native=copy.deepcopy(pair['native']);native['response']['error']['code']='planner.unrelated'
            with self.subTest(case=pair['id']),self.assertRaisesRegex(ValueError,'literal native outcome'):
                classify(cases[pair['id']],pair['source'],native)
    def test_changed_shared_source_schema_and_incomplete_case_reject_as_value_errors(self):
        for field in ('extra-story','missing-window','lock'):
            c,s,n=baseline()
            if field=='extra-story':c['source_input']['story']['unreviewed']=True
            elif field=='missing-window':del c['native_input']['candidates'][0]['source_range']['end']
            else:c['source_input']['assets'][0]['state']={'locked':True}
            with self.subTest(field=field),self.assertRaises(ValueError):validate_case(c)

    def test_required_membership_without_a_real_full_order_chain_rejects_before_comparison(self):
        case=make('required-membership-only',[asset('a'),asset('b')],['b'],2,2,[{'type':'must-include','asset_ids':['a']}])
        with self.assertRaisesRegex(ValueError,'complete chain'):validate_case(case)

    def test_optional_capture_timestamps_must_be_valid_calendar_dates(self):
        case=next(copy.deepcopy(c) for c in build_cases() if c['family']=='optional-ranking' and len(c['source_input']['assets'])>1)
        last=max(case['source_input']['assets'],key=lambda a:a['capturedAt']);last['capturedAt']='2020-01-99T00:00:00Z'
        with self.assertRaisesRegex(ValueError,'capture'):validate_case(case)

    def test_generated_cases_cannot_add_unregistered_error_mappings(self):
        c,s,n=baseline()
        c['expected_source']={'outcome':'error','error':{'name':'CompositionConstraintError','message':'Completely unregistered source failure'}}
        c['expected_native']={'outcome':'error','error':{'code':'planner.no_selection','path':'$','conflicts':[]}}
        s={'id':c['id'],'input_unchanged':True,'ok':False,'error':c['expected_source']['error']}
        n={'exit':1,'response':{'ok':False,'error':c['expected_native']['error']}}
        with self.assertRaisesRegex(ValueError,'unregistered'):classify(c,s,n)

    def test_generated_native_catalog_cannot_smuggle_unreviewed_fields(self):
        c,s,n=baseline();c['native_input']['brief']['assets'][0]['unreviewed']=True
        with self.assertRaisesRegex(ValueError,'catalog row'):classify(c,s,n)

    def test_native_rationals_require_ascii_decimal_wire_not_unicode_digits(self):
        c,s,n=baseline();n['response']['planning']['total_duration']={'numerator':'١','denominator':'١'}
        with self.assertRaisesRegex(ValueError,'canonical rational'):classify(c,s,n)

    def test_source_numeric_exponents_are_bounded_before_fraction_allocation(self):
        for value in (Decimal('1e-10000'),Decimal('1e10000')):
            with self.subTest(value=str(value)),self.assertRaisesRegex(ValueError,'numeric budget'):numeric(value)
        self.assertEqual(numeric(Decimal('0.25')).numerator,1)
        self.assertEqual(numeric(Decimal('0.25')).denominator,4)

if __name__=='__main__':unittest.main()
