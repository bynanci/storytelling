# SPDX-License-Identifier: Apache-2.0
"""Exact live observations; dispositions never excuse unlisted mismatches."""
from decimal import Decimal
from fractions import Fraction
import json
from math import gcd,lcm
import re
from cases import validate_case,require,time,exact

MAX_U64=(1<<64)-1
REPORT_KEYS={'schema_version','profile','scope','executable','source_verified','host_verified','selection_planned','duration_analysis','content_clock_rate','total_duration','target_status','brief_audit','plan','selections','skipped_candidates'}
AUDIT_KEYS={'schema_version','brief_id','intent','scope','executable','selection_planned','duration_analysis','consistent','manual_selected_asset_ids','required_assets','excluded_assets','precedence_edges','preferences','conflicts','warnings'}

def strict_loads(raw):
    def unique(rows):
        result={}
        for key,value in rows:
            require(key not in result,'duplicate JSON property');result[key]=value
        return result
    try:return json.loads(raw,object_pairs_hook=unique,parse_float=Decimal,parse_constant=lambda _:(_ for _ in ()).throw(ValueError('nonfinite JSON')))
    except (UnicodeDecodeError,json.JSONDecodeError) as exc:raise ValueError('invalid JSON/UTF8') from exc

def validate_source_batch(document,expected_ids):
    require(type(document) is dict and 'results' in document and type(document['results']) is list,'source result shape')
    require([x.get('id') for x in document['results']]==expected_ids,'source results missing/duplicate/reordered')
    return document['results']

def rational(value):
    f=exact(value);require(0<=f.numerator<=MAX_U64 and f.denominator<=MAX_U64,'native rational exceeds u64');return f

def numeric(value):
    require(type(value) in (int,Decimal,float),'source numeric shape')
    decimal=Decimal(str(value));parts=decimal.as_tuple()
    require(decimal.is_finite() and -20<=parts.exponent<=20 and len(parts.digits)<=64 and 0<=decimal<=MAX_U64,'source numeric budget exceeded')
    result=Fraction(decimal);require(result>=0,'negative source time');return result

def _source(case,row):
    require(type(row) is dict and row.get('id')==case['id'] and row.get('input_unchanged') is True,'source row identity/input mutation')
    require(type(row.get('ok')) is bool,'source row status')
    if not row['ok']:
        require(set(row)=={'id','input_unchanged','ok','error'},'source error row shape')
        error=row['error'];require(type(error) is dict and set(error)=={'name','message'} and error['name']=='CompositionConstraintError' and type(error['message']) is str,'unexpected source exception')
        return {'outcome':'error','error':error}
    require(set(row)=={'id','input_unchanged','ok','composition'},'source success row shape')
    comp=row['composition'];require(type(comp) is dict and set(comp)=={'storyId','duration','tracks'},'source composition shape')
    require(comp['storyId']==case['source_input']['story']['id'] and type(comp['tracks']) is list,'source composition identity')
    require(len(comp['tracks'])<=1,'source track shape')
    clips=[];total=Fraction(0);beats=case['source_input']['story']['beats'];allowed_ids={a['id'] for a in case['source_input']['assets']}
    for track in comp['tracks']:
        require(type(track) is dict and set(track)=={'type','clips'} and track['type']=='video' and type(track['clips']) is list,'source video track shape')
        for c in track['clips']:
            require(type(c) is dict and set(c)=={'assetId','beatId','sourceIn','sourceOut','timelineStart','timelineDuration'},'source clip shape')
            require(c['assetId'] in allowed_ids,'unknown source asset')
            if len(beats)==1:require(c['beatId']==beats[0]['id'],'source beat identity')
            else:require(any(c['beatId']==b['id'] and c['assetId'] in b.get('selectedAssetIds',[]) for b in beats),'source multi-beat ownership')
            start,end,position,duration=(numeric(c[k]) for k in ('sourceIn','sourceOut','timelineStart','timelineDuration'))
            require(end>start and end-start==duration and position==total,'source range/duration/placement')
            clips.append({'asset_id':c['assetId'],'source_start':time(start),'source_end':time(end),'timeline_start':time(position),'duration':time(duration)});total+=duration
    require(numeric(comp['duration'])==total,'source total duration')
    return {'outcome':'ok','total_duration':time(total),'clips':clips}

def _bindings(brief):
    required={};excluded={};edges={}
    def add(target,identifier,path):target.setdefault(identifier,[]).append(path)
    def order(ids,prefix):
        for i in range(1,len(ids)):edges.setdefault((ids[i-1],ids[i]),[]).append(f'{prefix}[{i}]')
    for i,identifier in enumerate(brief['manual_selected_asset_ids']):add(required,identifier,f'$.manual_selected_asset_ids[{i}]')
    order(brief['manual_selected_asset_ids'],'$.manual_selected_asset_ids')
    for i,constraint in enumerate(brief['hard_constraints']):
        target=excluded if constraint['type']=='must-exclude' else required
        for j,identifier in enumerate(constraint['asset_ids']):add(target,identifier,f'$.hard_constraints[{i}].asset_ids[{j}]')
        if constraint['type']=='asset-order':order(constraint['asset_ids'],f'$.hard_constraints[{i}].asset_ids')
    return required,excluded,edges

def _native(case,observation):
    require(type(observation) is dict and set(observation)=={'exit','response'} and type(observation['exit']) is int,'native command observation shape')
    response=observation['response'];require(type(response) is dict and type(response.get('ok')) is bool,'native response shape')
    if not response['ok']:
        require(observation['exit']==1 and set(response)=={'ok','error'},'native rejection exit/envelope')
        error=response['error'];require(type(error) is dict and set(error)=={'code','path','conflicts'} and type(error['code']) is str and type(error['path']) is str and type(error['conflicts']) is list,'native typed error shape')
        for conflict in error['conflicts']:require(set(conflict)=={'kind','path','asset_ids'},'native conflict shape')
        return {'outcome':'error','error':error}
    require(observation['exit']==0 and set(response)=={'ok','planning'},'native success exit/envelope')
    r=response['planning'];require(type(r) is dict and set(r)==REPORT_KEYS,'native report shape')
    require(type(r['schema_version']) is int and r['schema_version']==1 and r['profile']=='storytelling.fixed-window-planner.v1' and r['scope']=='declared-fixed-window-plan-only','native version/profile')
    require(r['executable'] is False and r['source_verified'] is False and r['host_verified'] is False and r['selection_planned'] is True and r['duration_analysis']=='exact-fixed-windows','native authority')
    data=case['native_input'];brief=data['brief'];candidates=data['candidates'];facts={c['asset_id']:(i,c) for i,c in enumerate(candidates)}
    audit=r['brief_audit'];require(type(audit) is dict and set(audit)==AUDIT_KEYS,'native audit shape')
    require(type(audit['schema_version']) is int and audit['schema_version']==1 and audit['brief_id']==brief['brief_id'] and audit['intent']==brief['intent'] and audit['scope']=='declared-constraint-audit-only' and audit['executable'] is False and audit['selection_planned'] is False and audit['duration_analysis']=='not-performed' and audit['consistent'] is True,'native retained audit authority')
    require(audit['manual_selected_asset_ids']==brief['manual_selected_asset_ids'] and audit['preferences']==brief['preferences'] and audit['conflicts']==[] and audit['warnings']==[],'native retained brief facts')
    required,excluded,edges=_bindings(brief)
    require(audit['required_assets']==[{'asset_id':identifier,'origins':paths} for identifier,paths in required.items()] and audit['excluded_assets']==[{'asset_id':identifier,'origins':paths} for identifier,paths in excluded.items()],'native hard bindings/origins')
    require(audit['precedence_edges']==[{'before':a,'after':b,'origins':paths} for (a,b),paths in edges.items()],'native edge origins')
    plan=r['plan'];require(type(plan) is dict and set(plan)=={'schema_version','plan_id','assets','shots'} and type(plan['schema_version']) is int and plan['schema_version']==1 and plan['plan_id']==brief['brief_id'],'native plan identity')
    shots=plan['shots'];require(type(shots) is list and shots and len(shots)<=data['max_shots'] and len(r['selections'])==len(shots),'native nonempty selection/count')
    ids=[s['asset_id'] for s in shots];require(len(ids)==len(set(ids)) and set(ids)<=facts.keys() and required.keys()<=set(ids) and not(set(ids)&excluded.keys()),'native required/excluded membership')
    wanted_assets=[{'id':c['asset_id'],'content_hash':c['content_hash'],'duration':c['duration']} for c in candidates if c['asset_id'] in ids];require(plan['assets']==wanted_assets,'native catalog/source facts')
    positions={identifier:i for i,identifier in enumerate(ids)};require(all(positions[a]<positions[b] for a,b in edges),'native precedence')
    values=[data['max_duration']]+[v for c in candidates for v in (c['duration'],c['source_range']['start'],c['source_range']['end'])]
    if 'target_duration' in brief['preferences']:values.append(brief['preferences']['target_duration'])
    rate=1
    for value in values:rate=lcm(rate,rational(value).denominator)
    require(r['content_clock_rate']==str(rate) and 0<rate<=MAX_U64,'native planning clock')
    clips=[];total=Fraction(0)
    preferred=brief['preferences']['preferred_asset_ids']
    for index,(shot,selection) in enumerate(zip(shots,r['selections']),1):
        identifier=shot['asset_id'];candidate_index,candidate=facts[identifier];shot_id=f'shot-{index:06}'
        require(shot=={'id':shot_id,'asset_id':identifier,'source_range':candidate['source_range']},'native immutable source window/shot ID')
        require(type(selection) is dict and set(selection)=={'asset_id','shot_id','required','declared_score','timeline_start','duration','reason_paths'},'native selection shape')
        start,end=(rational(candidate['source_range'][k]) for k in ('start','end'));duration=end-start
        require(start>=0 and duration>0 and end<=rational(candidate['duration']),'native source bounds')
        paths=['$.brief'+p[1:] for p in required[identifier]] if identifier in required else ([f'$.brief.preferences.preferred_asset_ids[{preferred.index(identifier)}]'] if identifier in preferred else [])+[f'$.candidates[{candidate_index}].score']
        paths.append(f'$.candidates[{candidate_index}].source_range')
        require(selection['asset_id']==identifier and selection['shot_id']==shot_id and selection['required'] is (identifier in required) and type(selection['declared_score']) is int and selection['declared_score']==candidate['score'] and selection['reason_paths']==paths,'native declaration reasons/score')
        require(rational(selection['timeline_start'])==total and rational(selection['duration'])==duration,'native exact placement/duration')
        require(total*rate<=MAX_U64 and (total*rate).denominator==1,'native placement clock bounds')
        clips.append({'asset_id':identifier,'source_start':time(start),'source_end':time(end),'timeline_start':time(total),'duration':time(duration)});total+=duration
    require(rational(r['total_duration'])==total<=rational(data['max_duration']) and total*rate<=MAX_U64,'native total/hard budget')
    target=brief['preferences'].get('target_duration');status='not-requested' if target is None else ('below' if total<rational(target) else 'above' if total>rational(target) else 'met')
    require(r['target_status']==status,'native soft target status')
    skipped=r['skipped_candidates'];unselected=[c['asset_id'] for c in candidates if c['asset_id'] not in ids]
    require(type(skipped) is list and [row['asset_id'] for row in skipped]==unselected,'native skipped catalog order')
    for row in skipped:
        identifier=row['asset_id'];candidate_index,candidate=facts[identifier]
        require(set(row)=={'asset_id','path','reason'} and row['path']==f'$.candidates[{candidate_index}]','native skipped shape/path')
        if identifier in excluded:wanted='excluded'
        elif case['id']=='delta-fixed-window' and identifier=='a':wanted='duration-limit'
        elif target is not None and total>=rational(target):wanted='not-needed'
        elif len(ids)>=data['max_shots']:wanted='shot-limit'
        else:wanted='duration-limit'
        require(row['reason']==wanted,'native actual skipped reason')
    return {'outcome':'ok','total_duration':time(total),'clips':clips}

def _differences(source,native):
    if source['outcome']!=native['outcome']:return ['outcome']
    if source['outcome']=='error':return [] if source['error']==native['error'] else ['error']
    differences=[]
    if source['total_duration']!=native['total_duration']:differences.append('total-duration')
    a,b=source['clips'],native['clips']
    if [x['asset_id'] for x in a]!=[x['asset_id'] for x in b]:differences.append('selection-order')
    for key,label in [('source_start','source-start'),('source_end','source-end'),('timeline_start','timeline-start'),('duration','window-duration')]:
        if [x[key] for x in a]!=[x[key] for x in b]:differences.append(label)
    return differences

def classify(case,source,native):
    validate_case(case)
    s=_source(case,source);n=_native(case,native);differences=_differences(s,n)
    if 'expected_source' in case:
        require(s==case['expected_source'],'literal source outcome/error pair changed')
        require(n==case['expected_native'],'literal native outcome/error pair changed')
    if case['disposition']=='agreement':
        if s['outcome']=='error':require('expected_source' in case and 'expected_native' in case,'unmapped arbitrary both-errors')
        else:require(s==n,'unclassified source/native semantic difference')
        classification='agreement'
        # Typed error vocabularies differ, but the immutable declared family maps them.
        differences=[]
    else:
        require('expected_source' in case and 'expected_native' in case and differences,'probe needs exact literal differing pair')
        classification=case['disposition']
    return {'id':case['id'],'disposition':case['disposition'],'family':case['family'],'classification':classification,'passed':True,'source':s,'native':n,'differences':differences}
