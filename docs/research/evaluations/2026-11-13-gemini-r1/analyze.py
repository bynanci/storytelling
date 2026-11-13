"""Reproduce exploratory Gemini R1 statistics; no data or label repair."""
from pathlib import Path
import csv, json, hashlib, platform
import numpy as np
import scipy
from scipy import stats
from score_input import cases, DIMENSIONS

ROOT=Path(__file__).resolve().parent
LABELS=['事實準確','來源可追溯','不確定性界線','敘事連貫','清晰度','受眾適配','任務完成']
SEED=20261113
BOOT=100000
PERM=200000

def save_csv(name,rows,fields=None):
    with (ROOT/name).open('w',encoding='utf-8-sig',newline='') as f:
        w=csv.DictWriter(f,fieldnames=fields or list(rows[0]),lineterminator='\n');w.writeheader();w.writerows(rows)

def json_safe(v):
    if isinstance(v,np.generic): return v.item()
    raise TypeError(type(v).__name__)

def proportion(wins,n):
    if not n: return dict(wins=0,n=0,rate=None,exact_95_CI=None,p_two_sided=None)
    test=stats.binomtest(wins,n,.5)
    ci=test.proportion_ci(.95,method='exact')
    return dict(wins=int(wins),n=int(n),rate=wins/n,exact_95_CI=[ci.low,ci.high],p_two_sided=test.pvalue)

def bootstrap_ci(d):
    # Resample whole cases; never resample dimensions or repeated judge scores.
    rng=np.random.default_rng(SEED)
    vals=d[rng.integers(0,len(d),(BOOT,len(d)))].mean(axis=1)
    return np.quantile(vals,[.025,.975]).tolist()

def sign_flip_p(d):
    d=np.asarray(d,dtype=float)
    rng=np.random.default_rng(SEED+1)
    null=(rng.choice([-1,1],size=(PERM,len(d)))*d).mean(axis=1)
    return (1+int((np.abs(null)>=abs(d.mean())-1e-12).sum()))/(PERM+1)

def holm(p):
    p=np.asarray(p);order=np.argsort(p);out=np.empty(len(p));prev=0
    for i,j in enumerate(order):
        prev=max(prev,(len(p)-i)*p[j]);out[j]=min(1.,prev)
    return out.tolist()

def summary(a,b):
    # Positive differences favor b. Composite = exploratory equal-weight mean.
    d=b.mean(axis=1)-a.mean(axis=1)
    sd=d.std(ddof=1)
    return dict(n_cases=len(d),mean_a=float(a.mean()),mean_b=float(b.mean()),
                mean_difference_b_minus_a=float(d.mean()),median_difference=float(np.median(d)),
                bootstrap_case_95_CI=bootstrap_ci(d),cohen_dz=float(d.mean()/sd) if sd else None,
                sign_flip_mean_p_exploratory=sign_flip_p(d),
                composite_wins_b=int((d>1e-10).sum()),composite_wins_a=int((d< -1e-10).sum()),
                composite_ties=int((np.abs(d)<=1e-10).sum()))

def mcnemar(a,b):
    a=np.asarray(a,dtype=bool);b=np.asarray(b,dtype=bool)
    ao=int((a&~b).sum());bo=int((~a&b).sum())
    return dict(n=len(a),flagged_a=int(a.sum()),flagged_b=int(b.sum()),
                rate_a=float(a.mean()),rate_b=float(b.mean()),
                difference_b_minus_a=float(b.mean()-a.mean()),a_only=ao,b_only=bo,both=int((a&b).sum()),
                exact_mcnemar_p=proportion(bo,ao+bo)['p_two_sided'])

def main():
    cs=cases()
    assert len(cs)==40 and {c['case_number'] for c in cs}==set(range(1,41))
    for c in cs:
        assert len(c['ratings_X'])==len(c['ratings_Y'])==7
        assert all(1<=v<=5 for v in c['ratings_X']+c['ratings_Y'])
        assert c['preference'] in ['X','Y','tie']
    x=np.array([c['ratings_X'] for c in cs]);y=np.array([c['ratings_Y'] for c in cs])
    sx=np.array([c['declared_story_variant']=='X' for c in cs])
    story=np.where(sx[:,None],x,y);control=np.where(sx[:,None],y,x)
    rows=[];case_rows=[]
    for c in cs:
        for v in ['X','Y']:
            rows.append(dict(case_id=c['case_id'],original_case_id=c['original_case_id'],batch=c['batch'],
                             run_id=c['run_id'],variant=v,presentation_order=c['presentation_order'],
                             declared_condition='story' if v==c['declared_story_variant'] else 'control',
                             assignment_provenance=c['assignment_provenance'],
                             critical_error_present=c['critical_'+v],
                             **dict(zip(DIMENSIONS,c['ratings_'+v]))))
        story_pref='tie' if c['preference']=='tie' else ('story' if c['preference']==c['declared_story_variant'] else 'control')
        case_rows.append(dict(case_id=c['case_id'],topic=c['topic'],batch=c['batch'],
                              mean_X=float(np.mean(c['ratings_X'])),mean_Y=float(np.mean(c['ratings_Y'])),
                              mean_Y_minus_X=float(np.mean(c['ratings_Y'])-np.mean(c['ratings_X'])),
                              preference=c['preference'],declared_story_variant=c['declared_story_variant'],
                              declared_preference=story_pref,critical_X=c['critical_X'],critical_Y=c['critical_Y'],
                              assignment_provenance=c['assignment_provenance']))
    save_csv('ratings_numeric_R1.csv',rows)
    save_csv('case_pairs_R1.csv',case_rows)
    (ROOT/'ratings_normalized_R1.json').write_text(json.dumps(dict(
        scope='Numeric transcription only; not verbatim evidence/reasons JSON',
        dimensions=DIMENSIONS,cases=cs),ensure_ascii=False,indent=2),encoding='utf-8',newline='\n')
    dim_rows=[]
    for j,(dim,label) in enumerate(zip(DIMENSIONS,LABELS)):
        d=story[:,j]-control[:,j]
        xy=y[:,j]-x[:,j]
        test=proportion(int((d>0).sum()),int((d!=0).sum()))
        dim_rows.append(dict(dimension=dim,label=label,mean_X=float(x[:,j].mean()),mean_Y=float(y[:,j].mean()),
                             mean_Y_minus_X=float(xy.mean()),
                             mean_declared_story=float(story[:,j].mean()),mean_declared_control=float(control[:,j].mean()),
                             mean_story_minus_control=float(d.mean()),
                             bootstrap_case_95_CI=bootstrap_ci(d),story_higher=int((d>0).sum()),
                             control_higher=int((d<0).sum()),ties=int((d==0).sum()),
                             sign_test_p=test['p_two_sided']))
    adj=holm([d['sign_test_p'] for d in dim_rows])
    for d,p in zip(dim_rows,adj):d['sign_test_p_Holm_7_dimensions']=p
    save_csv('dimension_summary_R1.csv',[{**d,'bootstrap_case_95_CI':json.dumps(d['bootstrap_case_95_CI'])} for d in dim_rows])
    raw_pref={k:sum(c['preference']==k for c in cs) for k in ['X','Y','tie']}
    declared_pref={k:sum(c['declared_preference']==k for c in case_rows) for k in ['story','control','tie']}
    batches=[]
    for b in range(1,5):
        ix=np.array([c['batch']==b for c in cs]);sub=[r for r in case_rows if r['batch']==b]
        batches.append(dict(batch=b,**summary(x[ix],y[ix]),
                            preference={k:sum(c['preference']==k for c in sub) for k in ['X','Y','tie']},
                            declared_summary=summary(control[ix],story[ix]),
                            declared_preference={k:sum(c['declared_preference']==k for c in sub) for k in ['story','control','tie']}))
    cx=np.array([c['critical_X'] for c in cs]);cy=np.array([c['critical_Y'] for c in cs])
    sc=np.where(sx,cx,cy);cc=np.where(sx,cy,cx)
    result=dict(
        analysis_label='Exploratory, single-judge R1; declared labels unverified; no causal claim',
        date='2026-11-13 Asia/Taipei',n_cases=40,n_texts=80,n_ratings=560,
        composite_definition='Exploratory equal-weight average of seven 1–5 ordinal ratings; not a validated scale',
        raw_XY=summary(x,y),raw_preference=raw_pref,
        raw_preference_X_among_decisive=proportion(raw_pref['X'],raw_pref['X']+raw_pref['Y']),
        critical_raw=mcnemar(cx,cy),
        declared_group_all40=summary(control,story),declared_preference=declared_pref,
        declared_preference_story_among_decisive=proportion(declared_pref['story'],declared_pref['story']+declared_pref['control']),
        critical_declared=mcnemar(cc,sc),
        declared_group_C11_C40=summary(control[10:],story[10:]),
        dimensions=dim_rows,batches=batches,
        ceiling=dict(ratings_at_5=int((np.r_[x,y]==5).sum()),total=560,rate=float((np.r_[x,y]==5).mean()),
                     texts_all_7_dimensions_5_X=int((x==5).all(axis=1).sum()),
                     texts_all_7_dimensions_5_Y=int((y==5).all(axis=1).sum())),
        missing_metadata=['generator_model_version','skill_commit_hash','isolated_condition_execution_manifest',
                          'judge_model_version','judge_settings','judge_timestamp','evidence_scope_confirmation',
                          'predeclared_required_claims','generation_replicates','order_reversed_R2'],
        computation=dict(seed=SEED,bootstrap_resamples=BOOT,sign_flip_permutations=PERM,
                         bootstrap_unit='whole case',python=platform.python_version(),numpy=np.__version__,scipy=scipy.__version__,
                         input_sha256=hashlib.sha256((ROOT/'score_input.py').read_bytes()).hexdigest()))
    (ROOT/'statistics_R1.json').write_text(json.dumps(result,ensure_ascii=False,indent=2,default=json_safe),encoding='utf-8',newline='\n')
    print(json.dumps({k:v for k,v in result.items() if k not in ['dimensions','batches','missing_metadata']},ensure_ascii=False,indent=2,default=json_safe))
    print('DIMENSIONS',json.dumps(dim_rows,ensure_ascii=False,default=json_safe))

if __name__=='__main__':main()
