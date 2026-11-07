#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Execute bounded actual source/native shadow cases into a fresh external root."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import sys
import time as clock
from cases import SOURCE_SHA,build_cases,require
from compare import strict_loads,validate_source_batch,classify
from process import run_bounded,ProcessFailure

ROOT=Path(__file__).resolve().parent
REPO=ROOT.parents[2]
LOCK=REPO/'tests/oracles/openfilm-finite-v1/package-lock.json'
LOCK_SHA='6697b505f83230df597922faea37a88535458366ca554a7cb52ec3cdc3b9e535'
SOURCE_FILES={'packages/core/src/models.ts':'58dbe93f132577842b7971e0d8aad61aff1d78c6','packages/core/src/validation.ts':'ba60475333f60fd9f48c77df2e34f0e7cac8536f','packages/solver/src/index.ts':'c701b985495c8c76292880b5acbc9c062b03006c','packages/story/src/index.ts':'da4d47ce45772d402db57b11659cc71f1b71e946'}
PRODUCER_KEYS={'schema_version','source_head','batch_sha256','producer_sha256','node_version','node_sha256','tsx_version','tsx_loader_sha256','tsx_package_sha256','esbuild_version','esbuild_sha256','results'}

def digest(data):return hashlib.sha256(data).hexdigest()
def file_hash(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as stream:
        while chunk:=stream.read(65536):h.update(chunk)
    return h.hexdigest()
def compact(value):return json.dumps(value,ensure_ascii=False,separators=(',',':')).encode()
def blob(path):
    data=Path(path).read_bytes();return hashlib.sha1(f'blob {len(data)}\0'.encode()+data).hexdigest()

def run(args):
    started=clock.monotonic();deadline=started+180;active_deadline=deadline-5
    root=args.run_root.resolve();source_path=args.source.resolve();node=args.node.resolve();tsx=args.tsx.resolve();rust=args.rust_cli.resolve()
    root.mkdir(parents=True,exist_ok=False);(root/'logs').mkdir()
    report={'schema_version':1,'profile':'storytelling.planner-shadow.v1','scope':'selected-domain-live-planner-shadow','allPassed':False,'source':{},'target':{},'tools':{},'policy':{'seed':args.seed,'cases_per_domain':args.cases_per_domain,'generator_sha256':file_hash(ROOT/'cases.py'),'comparator_sha256':file_hash(ROOT/'compare.py'),'harness_sha256':file_hash(Path(__file__)),'process_sha256':file_hash(ROOT/'process.py'),'child_seconds':30,'global_seconds':180,'stdout_limit':4194304,'stderr_limit':65536,'source_postcheck_reserved_seconds':5,'cleared_child_overrides':['NODE_OPTIONS','ESBUILD_BINARY_PATH']},'summary':{'planned':0,'executed':0,'agreement':0,'intentional-delta':0,'unsupported':0,'unclassified':0},'commands':[],'cases':[],'artifacts':{},'notClaimed':['full multi-beat/coverage/media-balance/locks/echo/graph/preset solver parity','all locale/scorer parity or global optimality/feasibility','live Catalog single-writer integration','media/source filesystem or host verification/approval/execution','human editorial acceptance','release/admin/signature authority'],'executionError':None}
    git_path=shutil.which('git')
    env=os.environ.copy();env.pop('NODE_OPTIONS',None);env.pop('ESBUILD_BINARY_PATH',None);env['TSX_TSCONFIG_PATH']=str(source_path/'tsconfig.json')
    stage='preflight';source_before=None
    def command(argv,data=None,limit=4194304,post=False):
        index=len(report['commands']);prefix=f'logs/{index:03}'
        failure=None
        try:receipt=run_bounded([str(x) for x in argv],data,deadline if post else active_deadline,stdout_limit=limit,env=env)
        except ProcessFailure as exc:receipt=exc.receipt;failure=exc
        stdout=receipt.pop('stdout',b'');stderr=receipt.pop('stderr',b'')
        (root/(prefix+'-stdout')).write_bytes(stdout);(root/(prefix+'-stderr')).write_bytes(stderr)
        receipt.update({'stage':stage,'stdout_path':prefix+'-stdout','stderr_path':prefix+'-stderr','stdout_sha256':digest(stdout),'stderr_sha256':digest(stderr),'stdin_sha256':None if data is None else digest(data)})
        if failure:receipt['failure_code']=failure.code
        report['commands'].append(receipt)
        if failure:raise failure
        return receipt['exit'],stdout,stderr
    def checked(argv,data=None,limit=4194304,post=False):
        code,out,err=command(argv,data,limit,post)
        require(code==0 and not err,'unexpected exit/stderr: '+str(argv[-1]));return out
    def git(repo,*argv,post=False):return checked([git_path,'-C',repo,*argv],post=post)
    try:
        require(sys.platform=='linux' and platform.machine()=='x86_64','shadow requires Linux x86_64')
        require(git_path is not None,'git executable missing')
        for tool in (node,tsx,rust):require(tool.is_absolute() and tool.is_file(),'explicit tool missing: '+str(tool))
        require(source_path.is_dir(),'source checkout missing')
        cases=build_cases(args.seed,args.cases_per_domain);report['summary']['planned']=len(cases)
        cases_bytes=compact(cases)+b'\n';(root/'cases.json').write_bytes(cases_bytes)
        batch_bytes=compact({'schema_version':1,'cases':[{'id':c['id'],'source_input':c['source_input']} for c in cases]})+b'\n'
        require(len(batch_bytes)<=1048576,'source batch exceeds1MiB');(root/'source-batch.json').write_bytes(batch_bytes)
        source_head=git(source_path,'rev-parse','HEAD').decode().strip();status=git(source_path,'status','--porcelain')
        report['source']={'path':str(source_path),'head':source_head,'status_before_sha256':digest(status),'clean_before':not status,'files':[],'postcheck':False}
        source_before=(source_head,status)
        require(source_head==SOURCE_SHA and not status,'source revision/clean state mismatch')
        for name,wanted in SOURCE_FILES.items():
            actual=git(source_path,'rev-parse','HEAD:'+name).decode().strip();worktree=blob(source_path/name)
            report['source']['files'].append({'path':name,'expected_blob':wanted,'head_blob':actual,'worktree_blob':worktree})
            require(actual==wanted==worktree,'source blob changed: '+name)
        target_head=git(REPO,'rev-parse','HEAD').decode().strip();target_tree=git(REPO,'rev-parse','HEAD^{tree}').decode().strip();target_status=git(REPO,'status','--porcelain');target_diff=git(REPO,'diff','HEAD','--binary')
        report['target']={'repository':str(REPO),'head':target_head,'tree':target_tree,'working_tree_dirty':bool(target_status),'status_sha256':digest(target_status),'tracked_diff_sha256':digest(target_diff),'rust_cli':str(rust),'rust_cli_sha256':file_hash(rust)}
        node_version=checked([node,'--version'],limit=256).decode().strip();require(node_version=='v24.22.1','Node version mismatch')
        rust_version=checked([rust,'--version'],limit=256).decode().strip();require(rust_version.startswith('storytelling '),'native CLI version shape')
        require(file_hash(LOCK)==LOCK_SHA,'frozen test lock changed')
        prefix=tsx.parents[3];installed_lock=prefix/'package-lock.json';require(file_hash(installed_lock)==LOCK_SHA,'installed tools do not use frozen lock')
        tsx_package=tsx.parent.parent/'package.json';tsx_data=strict_loads(tsx_package.read_bytes());require(tsx_data['name']=='tsx' and tsx_data['version']=='4.23.15','tsx package identity')
        esbuild_package=tsx.parents[2]/'esbuild/package.json';esbuild_data=strict_loads(esbuild_package.read_bytes());require(esbuild_data['name']=='esbuild' and esbuild_data['version']=='0.28.2','esbuild package identity')
        esbuild=tsx.parents[2]/'@esbuild/linux-x64/bin/esbuild';require(esbuild.is_file(),'resolved esbuild binary missing')
        esbuild_version=checked([esbuild,'--version'],limit=256).decode().strip();require(esbuild_version=='0.28.2','esbuild binary version')
        report['tools']={'python':{'path':str(Path(sys.executable).resolve()),'version':platform.python_version(),'implementation':platform.python_implementation(),'sha256':file_hash(Path(sys.executable).resolve())},'node':{'path':str(node),'version':node_version,'sha256':file_hash(node)},'tsx':{'loader':str(tsx),'version':'4.23.15','loader_sha256':file_hash(tsx),'package_sha256':file_hash(tsx_package)},'esbuild':{'path':str(esbuild),'version':esbuild_version,'sha256':file_hash(esbuild),'package_sha256':file_hash(esbuild_package)},'rust':{'path':str(rust),'version':rust_version,'sha256':file_hash(rust)},'dependency_lock_sha256':file_hash(LOCK),'producer_sha256':file_hash(ROOT/'producer.mts')}
        stage='source-producer'
        raw=checked([node,'--import',tsx,ROOT/'producer.mts','--source',source_path],batch_bytes)
        require(raw.endswith(b'\n') and raw.count(b'\n')==1,'producer must emit one JSON line')
        producer=strict_loads(raw);require(type(producer) is dict and set(producer)==PRODUCER_KEYS,'producer exact report shape')
        expected={'schema_version':1,'source_head':source_head,'batch_sha256':digest(batch_bytes),'producer_sha256':report['tools']['producer_sha256'],'node_version':node_version,'node_sha256':report['tools']['node']['sha256'],'tsx_version':'4.23.15','tsx_loader_sha256':report['tools']['tsx']['loader_sha256'],'tsx_package_sha256':report['tools']['tsx']['package_sha256'],'esbuild_version':'0.28.2','esbuild_sha256':report['tools']['esbuild']['sha256']}
        require(type(producer['schema_version']) is int and all(producer[k]==v for k,v in expected.items()),'producer metadata/provenance mismatch')
        rows=validate_source_batch(producer,[c['id'] for c in cases])
        for case,source_row in zip(cases,rows):
            stage='native:'+case['id'];native_bytes=compact(case['native_input'])+b'\n';(root/(case['id']+'-native-input.json')).write_bytes(native_bytes)
            code,out,err=command([rust,'plan-story','-'],native_bytes);require(not err,'unexpected native stderr')
            require(out.endswith(b'\n') and out.count(b'\n')==1,'native must emit one JSON line')
            native={'exit':code,'response':strict_loads(out)};report['summary']['executed']+=1
            try:
                observed=classify(case,source_row,native)
                if native['response'].get('ok') is True:
                    stage='validator:'+case['id'];validation=checked([rust,'validate','-'],compact(native['response']['planning']['plan'])+b'\n')
                    require(validation.endswith(b'\n') and validation.count(b'\n')==1,'validator one-line response')
                    validated=strict_loads(validation);require(set(validated)=={'ok','validation'} and validated['ok'] is True,'actual nested plan rejected')
                    v=validated['validation'];require(set(v)=={'schema_version','plan_id','assets','shots','scope'} and type(v['schema_version']) is int and v['schema_version']==1 and type(v['assets']) is int and type(v['shots']) is int and v['plan_id']==case['native_input']['brief']['brief_id'] and v['assets']==len(native['response']['planning']['plan']['assets']) and v['shots']==len(native['response']['planning']['plan']['shots']) and v['scope']=='declared-metadata-only','actual validator report facts')
                report['summary'][observed['classification']]+=1
            except ValueError as exc:
                observed={'id':case['id'],'disposition':case['disposition'],'family':case['family'],'classification':'unclassified','passed':False,'source':{'outcome':'unvalidated','source_result_index':len(report['cases'])},'native':{'outcome':'unvalidated','exit':code},'differences':[str(exc)]};report['summary']['unclassified']+=1
                if report['executionError'] is None:report['executionError']={'stage':stage,'code':'semantic-mismatch','message':str(exc)}
            report['cases'].append(observed)
        report['allPassed']=report['summary']['executed']==len(cases) and not report['summary']['unclassified']
    except (ValueError,OSError,ProcessFailure,KeyError,TypeError) as exc:
        report['allPassed']=False
        report['executionError']={'stage':stage,'code':exc.code if isinstance(exc,ProcessFailure) else 'execution-failed','message':str(exc)}
    finally:
        stage='source-postcheck'
        if source_path.is_dir() and git_path:
            try:
                after_head=git(source_path,'rev-parse','HEAD',post=True).decode().strip();after_status=git(source_path,'status','--porcelain',post=True)
                report['source'].update({'head_after':after_head,'status_after_sha256':digest(after_status),'clean_after':not after_status,'postcheck_executed':True})
                require(source_before is not None and (after_head,after_status)==source_before and after_head==SOURCE_SHA and not after_status,'source changed or failed preflight')
                for name,wanted in SOURCE_FILES.items():require(blob(source_path/name)==wanted,'source worktree bytes changed after run')
                report['source']['postcheck']=True
            except (ValueError,OSError,ProcessFailure) as exc:
                report['allPassed']=False
                report['executionError']=report['executionError'] or {'stage':stage,'code':'source-postcheck-failed','message':str(exc)}
        else:report['allPassed']=False
        report['artifacts']={str(p.relative_to(root)):{'sha256':file_hash(p),'bytes':p.stat().st_size} for p in sorted(root.rglob('*')) if p.is_file() and p.name!='evidence.json'}
        (root/'evidence.json').write_bytes(json.dumps(report,ensure_ascii=False,indent=2).encode()+b'\n')
    print(json.dumps({'ok':report['allPassed'],'cases':report['summary'],'output':str(root/'evidence.json')}))
    return 0 if report['allPassed'] else 1

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    for name in ('source','node','tsx','rust-cli','run-root'):parser.add_argument('--'+name,type=Path,required=True)
    parser.add_argument('--seed',type=int,default=20261107);parser.add_argument('--cases-per-domain',type=int,default=24)
    args=parser.parse_args()
    if any(not getattr(args,k).is_absolute() for k in ('source','node','tsx','rust_cli','run_root')):parser.error('all supplied paths must be absolute')
    if not 0<=args.seed<=0xffffffff or not 1<=args.cases_per_domain<=64:parser.error('seed u32 and cases-per-domain1..64 required')
    root=args.run_root.resolve()
    if root.exists() or root.is_relative_to(REPO) or root.is_relative_to(args.source.resolve()):parser.error('run-root must be new and outside source/target checkouts')
    return run(args)

if __name__=='__main__':raise SystemExit(main())
