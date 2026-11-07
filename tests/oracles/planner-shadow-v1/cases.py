# SPDX-License-Identifier: Apache-2.0
"""Deterministic synthetic domains; no policy label can excuse an arbitrary pair."""
from copy import deepcopy
from datetime import datetime
from fractions import Fraction
import hashlib
import json

SOURCE_SHA='7add4762fe8174d76f43b863375ea39b73121e76'

def require(condition,message):
    if not condition: raise ValueError(message)

def time(value):
    value=Fraction(value)
    return {'numerator':str(value.numerator),'denominator':str(value.denominator)}

def exact(value):
    require(isinstance(value,dict) and set(value)=={'numerator','denominator'},'rational shape')
    for key in value:
        require(type(value[key]) is str and value[key].isascii() and value[key].isdigit() and (value[key]=='0' or not value[key].startswith('0')),'canonical rational')
    n,d=int(value['numerator']),int(value['denominator']);require(d>0,'rational denominator')
    f=Fraction(n,d);require(f.numerator==n and f.denominator==d,'reduced rational');return f

def seconds(value):
    value=Fraction(value)
    return int(value) if value.denominator==1 else float(value)

def asset(identifier,duration=1,rating=3,kind='video',captured=None):
    row={'id':identifier,'name':f'Synthetic {identifier}','uri':f'synthetic://shadow/{identifier}','mediaType':kind,'duration':seconds(duration),'rating':rating,'tags':[],'state':{},'metadata':{}}
    if captured is not None: row['capturedAt']=captured
    return row

def make(identifier,assets,manual,target,maximum,constraints=None,preferred=None,windows=None,scores=None):
    constraints=deepcopy(constraints or [])
    source_constraints=[{'type':c['type'],'assetIds':list(c['asset_ids'])} for c in constraints]
    preferences={'preferred_asset_ids':list(preferred or [])}
    story={'id':identifier,'title':'Synthetic story','maxDuration':seconds(maximum),'beats':[{'id':'beat','title':'Sequence','selectedAssetIds':list(manual),'constraints':source_constraints}]}
    if target is not None:preferences['target_duration']=time(target);story['targetDuration']=seconds(target)
    candidates=[]
    for a in assets:
        start,end=(windows or {}).get(a['id'],(0,Fraction(str(a['duration']))))
        candidates.append({'asset_id':a['id'],'content_hash':hashlib.sha256(('shadow/'+a['id']).encode()).hexdigest(),'duration':time(Fraction(str(a['duration']))),'source_range':{'start':time(start),'end':time(end)},'score':(scores or {}).get(a['id'],a['rating'])})
    native={'schema_version':1,'brief':{'schema_version':1,'brief_id':identifier,'intent':'Synthetic live shadow','assets':[{'id':a['id']} for a in assets],'manual_selected_asset_ids':list(manual),'hard_constraints':constraints,'preferences':preferences},'candidates':candidates,'max_duration':time(maximum),'max_shots':max(1,len(assets))}
    return {'id':identifier,'disposition':'agreement','family':'required-fractional','source_input':{'story':story,'assets':deepcopy(assets)},'native_input':native}

def projection(ids,lengths,starts=None):
    total=Fraction(0);clips=[]
    for index,(identifier,length) in enumerate(zip(ids,lengths)):
        start=Fraction((starts or [0]*len(ids))[index]);length=Fraction(length)
        clips.append({'asset_id':identifier,'source_start':time(start),'source_end':time(start+length),'timeline_start':time(total),'duration':time(length)});total+=length
    return {'outcome':'ok','total_duration':time(total),'clips':clips}

def source_error(message):return {'outcome':'error','error':{'name':'CompositionConstraintError','message':message}}
def native_error(code,path='$',conflicts=None):return {'outcome':'error','error':{'code':code,'path':path,'conflicts':conflicts or []}}

def fixed_cases():
    result=[]
    def add(c,family,disposition='agreement',source=None,native=None):
        c['family']=family;c['disposition']=disposition
        if source is not None:c['expected_source']=source;c['expected_native']=native
        result.append(c);return c
    add(make('required-over-target',[asset('a',Fraction(1,2)),asset('b',Fraction(1,2))],['b','a'],Fraction(1,2),2),'required-over-target')
    add(make('explicit-exclusion',[asset('b',1,5),asset('a',Fraction(1,2),1)],['a'],Fraction(1,2),1,[{'type':'must-exclude','asset_ids':['b']}]),'explicit-exclusion')
    c=make('missing-required',[asset('a')],['missing'],1,1)
    add(c,'missing-reference',source=source_error('Beat "Sequence" references missing asset missing. Relink the asset or remove the constraint.'),native=native_error('planner.brief_conflict','$.brief',[{'kind':'unknown-asset','path':'$.brief.manual_selected_asset_ids[0]','asset_ids':['missing']}]))
    c=make('required-excluded',[asset('a')],['a'],1,1,[{'type':'must-exclude','asset_ids':['a']}])
    add(c,'required-excluded',source=source_error('Beat "Sequence" both includes and excludes Synthetic a. Remove one of these constraints.'),native=native_error('planner.brief_conflict','$.brief',[{'kind':'required-excluded','path':'$.brief.manual_selected_asset_ids[0]','asset_ids':['a']}]))
    c=make('order-cycle',[asset('a'),asset('b')],['a','b'],2,2,[{'type':'asset-order','asset_ids':['b','a']}])
    add(c,'order-cycle',source=source_error('Beat "Sequence" has conflicting asset orders or chronology. Remove or reconcile the ordering constraints.'),native=native_error('planner.brief_conflict','$.brief',[{'kind':'order-cycle','path':'$.brief.hard_constraints[0].asset_ids[1]','asset_ids':['a','b']}]))
    c=make('required-maximum',[asset('a',Fraction(3,4)),asset('b',Fraction(3,4))],['a','b'],1,1)
    add(c,'required-maximum',source=source_error('Required assets and beat minima need 1.50s, above the story maximum of 1.00s. Increase the maximum, reduce beat minima, or remove selections/locks.'),native=native_error('planner.required_duration_exceeded','$.max_duration'))
    add(make('shortfall',[asset('a',Fraction(1,2))],['a'],1,1),'shortfall')
    add(make('required-inclusion',[asset('a',Fraction(1,2))],[],Fraction(1,2),1,[{'type':'must-include','asset_ids':['a']}]),'required-inclusion')
    c=make('delta-fixed-window',[asset('a',4,5,'image'),asset('b',1,0)],[],1,1,scores={'a':100,'b':0})
    add(c,'fixed-window-no-shortening','intentional-delta',projection(['a'],[1]),projection(['b'],[1]))
    c=make('delta-preferred-id',[asset('a',4,0,'image'),asset('b',4,5,'image')],[],4,4,preferred=['a'],scores={'a':0,'b':100})
    add(c,'preferred-id-ranking','intentional-delta',projection(['b'],[4]),projection(['a'],[4]))
    c=make('delta-empty',[asset('a')],[],1,1,[{'type':'must-exclude','asset_ids':['a']}])
    add(c,'native-empty-refusal','intentional-delta',projection([],[]),native_error('planner.no_selection','$.candidates'))
    c=make('delta-target-above-max',[asset('a',Fraction(1,2))],[],3,1)
    add(c,'soft-target-above-maximum','intentional-delta',source_error('Story target exceeds maximum. Lower the target or increase the maximum.'),projection(['a'],[Fraction(1,2)]))
    c=make('unsupported-two-beats',[asset('a'),asset('b')],['a'],2,2)
    c['source_input']['story']['beats']=[{'id':'first','title':'First','selectedAssetIds':['a'],'constraints':[]},{'id':'second','title':'Second','selectedAssetIds':['b'],'constraints':[]}];c['native_input']['beats']=deepcopy(c['source_input']['story']['beats'])
    add(c,'multi-beat-distinct','unsupported',projection(['a','b'],[1,1]),native_error('input.invalid_json'))
    c=make('unsupported-repeated-beats',[asset('a')],['a'],2,2)
    c['source_input']['story']['beats']=[{'id':'first','title':'First','selectedAssetIds':['a'],'constraints':[]},{'id':'second','title':'Second','selectedAssetIds':['a'],'constraints':[]}];c['native_input']['beats']=deepcopy(c['source_input']['story']['beats'])
    add(c,'multi-beat-repeated','unsupported',source_error('Asset "Synthetic a" is required by more than one beat. Keep it in one beat; repeated assets are disabled.'),native_error('input.invalid_json'))
    for identifier,balance,wanted in [('unsupported-video-balance',{'minVideos':1,'maxImages':0},'b'),('unsupported-image-balance',{'minImages':1,'maxVideos':0},'a')]:
        c=make(identifier,[asset('a',1,3,'image'),asset('b')],[],1,1)
        c['source_input']['story']['beats'][0]['constraints']=[{'type':'media-balance',**balance}]
        c['native_input']['brief']['hard_constraints']=[{'type':'media-balance',**balance}]
        add(c,'media-balance-'+wanted,'unsupported',projection([wanted],[1]),native_error('input.invalid_json','$.brief'))
    return result

class Generator:
    def __init__(self,seed):self.state=(seed ^ 0x9e3779b9)&0xffffffff
    def integer(self,maximum):self.state=(1664525*self.state+1013904223)&0xffffffff;return self.state%maximum
    def shuffle(self,rows):
        rows=list(rows)
        for index in range(len(rows)-1,0,-1):
            other=self.integer(index+1);rows[index],rows[other]=rows[other],rows[index]
        return rows

def build_cases(seed=20261107,per_domain=24):
    require(type(seed) is int and 0<=seed<=0xffffffff,'seed must be u32')
    require(type(per_domain) is int and 1<=per_domain<=64,'cases-per-domain must be1..64')
    rng=Generator(seed);result=[]
    for index in range(per_domain):
        count=1+rng.integer(5);assets=[asset(chr(97+i),Fraction(1+rng.integer(4),4),rng.integer(6)) for i in range(count)]
        total=sum(Fraction(str(a['duration'])) for a in assets);maximum=total+Fraction(rng.integer(4),4);target=Fraction(1+rng.integer(int(maximum*4)),4)
        manual=rng.shuffle([a['id'] for a in assets]);constraints=[]
        if count>1 and rng.integer(2):constraints=[{'type':'asset-order','asset_ids':manual}];manual=[]
        c=make(f'required-{index+1:04}',rng.shuffle(assets),manual,target,maximum,constraints);result.append(c)
    for index in range(per_domain):
        count=1+rng.integer(6);assets=[asset(chr(97+i),1,rng.integer(6)) for i in range(count)]
        rank=sorted(assets,key=lambda a:(-a['rating'],a['id']))
        for position,a in enumerate(rank):a['capturedAt']=f'2020-01-{position+1:02}T00:00:00Z'
        k=1+rng.integer(count);c=make(f'optional-{index+1:04}',rng.shuffle(assets),[],k,k);c['family']='optional-ranking';result.append(c)
    result+=fixed_cases()
    for case in result:validate_case(case)
    return result

def _canonical(value):return json.dumps(value,sort_keys=True,separators=(',',':'))

def validate_case(case):
    require(type(case) is dict,'case shape')
    require(type(case.get('id')) is str and 1<=len(case['id'])<=128 and case['id'].isascii(),'case identifier')
    require(set(case) in ({'id','disposition','family','source_input','native_input'},{'id','disposition','family','source_input','native_input','expected_source','expected_native'}),'case fields')
    fixed={x['id']:x for x in fixed_cases()}
    if case['id'] in fixed:
        require(_canonical(case)==_canonical(fixed[case['id']]),'fixed probe disposition/precondition/expected pair changed');return
    require(set(case)=={'id','disposition','family','source_input','native_input'},'unregistered expected error/outcome mapping')
    require(case['disposition']=='agreement' and case['family'] in ('required-fractional','optional-ranking'),'unknown disposition/family')
    source=case['source_input'];native=case['native_input']
    require(type(source) is dict and set(source)=={'story','assets'},'shared source envelope')
    require(type(native) is dict and set(native)=={'schema_version','brief','candidates','max_duration','max_shots'} and type(native['schema_version']) is int and native['schema_version']==1,'shared native envelope')
    story=source['story'];assets=source['assets'];brief=native['brief'];candidates=native['candidates']
    require(type(story) is dict and set(story) in ({'id','title','maxDuration','beats'},{'id','title','targetDuration','maxDuration','beats'}),'shared source story fields')
    require(type(assets) is list and type(candidates) is list and 1<=len(assets)<=1024,'shared catalog bounds')
    require(type(brief) is dict and set(brief)=={'schema_version','brief_id','intent','assets','manual_selected_asset_ids','hard_constraints','preferences'} and type(brief['schema_version']) is int and brief['schema_version']==1,'shared brief fields')
    require(type(brief['assets']) is list and all(type(a) is dict and set(a)=={'id'} and type(a['id']) is str for a in brief['assets']),'shared catalog row shape')
    require(type(brief['preferences']) is dict and set(brief['preferences']) in ({'preferred_asset_ids'},{'preferred_asset_ids','target_duration'}),'shared preferences fields')
    require(type(story['beats']) is list and len(story['beats'])==1,'shared one-beat domain')
    require(type(story['beats'][0]) is dict and set(story['beats'][0])=={'id','title','selectedAssetIds','constraints'},'shared source beat fields')
    require(type(brief['manual_selected_asset_ids']) is list and len(set(brief['manual_selected_asset_ids']))==len(brief['manual_selected_asset_ids']),'shared manual shape')
    require(type(brief['hard_constraints']) is list and all(type(c) is dict and set(c)=={'type','asset_ids'} and c['type'] in ('must-include','asset-order') and type(c['asset_ids']) is list and len(c['asset_ids'])==len(set(c['asset_ids'])) for c in brief['hard_constraints']),'shared constraint shape')
    require(all(type(a) is dict and {'id','name','uri','mediaType','duration','rating','tags','state','metadata'}<=set(a) and type(a['id']) is str for a in assets),'shared source asset fields')
    require(all(type(c) is dict and set(c)=={'asset_id','content_hash','duration','source_range','score'} and type(c['source_range']) is dict and set(c['source_range'])=={'start','end'} and type(c['score']) is int and -(1<<31)<=c['score']<(1<<31) and type(c['content_hash']) is str and len(c['content_hash'])==64 and all(x in '0123456789abcdef' for x in c['content_hash']) for c in candidates),'shared candidate shape')
    require(type(native['max_shots']) is int and native['max_shots']<=1024,'shared shot budget')
    require(set(source)=={'story','assets'} and len(story['beats'])==1,'shared one-beat domain')
    require(story['id']==brief['brief_id']==case['id'],'shared story identity')
    ids=[a['id'] for a in assets];require(ids and len(ids)==len(set(ids)) and all(i.isascii() and i.islower() and i.isalpha() for i in ids),'shared lowercase ASCII IDs')
    require([c['asset_id'] for c in candidates]==ids==[a['id'] for a in brief['assets']],'shared catalog order')
    require(native['max_shots']>=len(ids) and brief['preferences']['preferred_asset_ids']==[],'shared nonbinding shot limit/preferences')
    require(exact(native['max_duration'])==Fraction(str(story['maxDuration']))>0,'shared maximum')
    target=brief['preferences'].get('target_duration');source_target=story.get('targetDuration')
    require((target is None)==(source_target is None),'shared target presence')
    if target is not None:require(exact(target)==Fraction(str(source_target))<=exact(native['max_duration']),'shared target must fit maximum')
    beat=story['beats'][0];require(beat['selectedAssetIds']==brief['manual_selected_asset_ids'],'shared manual order')
    constraints=brief['hard_constraints'];require(beat['constraints']==[{'type':c['type'],'assetIds':c['asset_ids']} for c in constraints],'shared constraint mapping')
    for a,c in zip(assets,candidates):
        require(a['mediaType']=='video' and a['state']=={} and a['tags']==[] and a['metadata']=={},'shared unadorned video facts')
        require(type(a['rating']) is int and 0<=a['rating']<=5 and c['score']==a['rating'],'shared monotone rating declaration')
        require(exact(c['duration'])==Fraction(str(a['duration'])) and exact(c['source_range']['start'])==0 and exact(c['source_range']['end'])==exact(c['duration']),'shared full zero-origin windows')
    if case['family']=='required-fractional':
        require(all(Fraction(str(a['duration'])) in (Fraction(1,4),Fraction(1,2),Fraction(3,4),Fraction(1)) for a in assets),'required dyadic capacities')
        require(all(set(a)=={'id','name','uri','mediaType','duration','rating','tags','state','metadata'} for a in assets),'required source scoring facts')
        required=list(brief['manual_selected_asset_ids']);orders=[list(required)] if required else []
        for c in constraints:
            require(c['type'] in ('must-include','asset-order'),'required shared constraints')
            if c['type']=='asset-order':orders.append(c['asset_ids'])
            required.extend(c['asset_ids'])
        require(set(required)==set(ids) and any(len(o)==len(ids) and set(o)==set(ids) for o in orders),'required domain needs complete chain')
    else:
        maximum=exact(native['max_duration'])
        require(target is not None and exact(target)==maximum and maximum.denominator==1 and maximum<=len(ids),'optional positive integer target=max K')
        require(not brief['manual_selected_asset_ids'] and not constraints,'optional has no required constraints')
        require(all(a['duration']==1 and set(a)=={'id','name','uri','mediaType','duration','rating','tags','state','metadata','capturedAt'} for a in assets),'optional one-second source facts')
        ranked=sorted(assets,key=lambda a:(-a['rating'],a['id']));captures=[a['capturedAt'] for a in ranked]
        require(len(set(captures))==len(ids) and captures==sorted(captures) and all(len(x)==20 and x.startswith('2020-01-') and x.endswith('T00:00:00Z') for x in captures),'optional capture/rank alignment')
        for value in captures:
            try:datetime.strptime(value,'%Y-%m-%dT%H:%M:%SZ')
            except (TypeError,ValueError) as exc:raise ValueError('optional invalid capture date') from exc
