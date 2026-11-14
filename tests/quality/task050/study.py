"""Finite opt-in performance/reproducibility study using run.py's real CLI timer.

No package installation, remote endpoints, corpus execution or product changes.
"""
import copy
import datetime
import json
import os
from pathlib import Path
import re
import shutil
import statistics
import tarfile
import tempfile
import time

from evaluate import bytes_digest, canonical, digest, normalized_report
from run import REPO, distributions, environment, measure, write_json


FORMATS = ['report.json','report.sarif','report.html','report.md','cbom.json','dependency-sbom.json',
           'diagnostics.json','scan.inventory.v1.json','scan.assessment.v1.json','cbom.cdx-1.7.json','dependency-sbom.cdx-1.7.json']
# Explicit collections only. Arrays such as source lines/parameters retain order.
COLLECTIONS = {'findings','cbom','dependency_sbom','results','rules','components','assets',
               'observations','assessments','asset_policies','legacy_associations','diagnostics'}
ENV_KEYS = ['python','cpuCount','wallTimer','cachePolicy','binarySha256','harnessDigest','platform','machine','hardware','rust','buildProfile','features','runtimeSourceDigest','constraints']


def inventory(root):
    bindings={str(p.relative_to(root)):dict(sha256=bytes_digest(p),bytes=p.stat().st_size)
              for p in sorted(root.rglob('*')) if p.is_file()}
    return dict(files=len(bindings),bytes=sum(v['bytes'] for v in bindings.values()),sha256=digest(bindings),bindings=bindings)


def verify_input(root, expected):
    if inventory(root)!=expected:raise ValueError('measured input changed')


def throughput(sample, info):
    if sample['exitCode'] or sample['timedOut']:return None
    return dict(filesPerSecond=info['files']/sample['wallSeconds'],bytesPerSecond=info['bytes']/sample['wallSeconds'])


def fingerprints(name, value):
    if name.endswith('.json') or name.endswith('.sarif'):
        value=normalized_report(value) if name=='report.json' else copy.deepcopy(value)
        if isinstance(value,dict):
            if name=='diagnostics.json':value.pop('generated_at',None)
            if name=='scan.inventory.v1.json':value.get('run',{}).pop('generated_at',None)
        order=digest(value)
        def sort_collections(node, key=None):
            if isinstance(node,dict):return {k:sort_collections(v,k) for k,v in node.items()}
            if isinstance(node,list):
                rows=[sort_collections(v) for v in node]
                return sorted(rows,key=canonical) if key in COLLECTIONS or (key == '__root__' and name in {'cbom.json','dependency-sbom.json'}) else rows
            return node
        return dict(order=order,semantic=digest(sort_collections(value,'__root__')))
    # Remove only the renderer's generated-time line; no generic timestamp/ID scrub.
    value=re.sub(r'(?m)(Generated: )\d{4}-\d\d-\d\d \d\d:\d\d:\d\d(?:\.\d+)? UTC',r'\1<VOLATILE>',value)
    return dict(order=digest(value),semantic=digest(value))


def comparison(before, after):
    reasons=[k for k in ENV_KEYS if before['environment'].get(k)!=after['environment'].get(k)]
    for key in ['input','configuration']:
        if before.get(key)!=after.get(key):reasons.append(key)
    ratios={};flags=[]
    if not reasons:
        for key in ['wallSeconds','peakRssBytes']:
            a=before['warmMax'].get(key);b=after['warmMax'].get(key)
            ratios[key]=b/a if a and b is not None else None
            if ratios[key] is not None and ratios[key]>2:flags.append(key)
    return dict(comparable=not reasons,reasons=reasons,ratios=ratios,investigate=flags,
                policy='strictly >2x prior warm maximum requires investigation, not automatic regression; no threshold revision')


def sources():
    # First-party product implementation, not test/fuzz/dependency vendor code.
    paths=sorted(p for p in (REPO/'crates').glob('*/src/**/*.rs') if p.is_file())
    paths += [REPO/'Cargo.toml',REPO/'Cargo.lock']
    return paths


def materialize(root, copies):
    root.mkdir()
    for n in range(copies):
        for path in sources():
            dest=root/f'project-{n:02}'/path.relative_to(REPO)
            dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(path,dest)
    return inventory(root)


def inspect_outputs(directory):
    missing=[name for name in FORMATS if not (directory/name).is_file()]
    if missing:raise ValueError('missing output '+str(missing))
    result={}
    for name in FORMATS:
        p=directory/name;value=json.loads(p.read_text()) if name.endswith(('.json','.sarif')) else p.read_text()
        result[name]=dict(sha256=bytes_digest(p),bytes=p.stat().st_size,**fingerprints(name,value))
    report=json.loads((directory/'report.json').read_text())
    inv=json.loads((directory/'scan.inventory.v1.json').read_text())
    return result,facts_from_documents(report,inv),report


def facts_from_documents(report, inv):
    return dict(findingCount=len(report['findings']),findingIds=[x['finding_id'] for x in report['findings']],
                assetIds=[x['id'] for x in inv['assets']],occurrenceKeys=[x['occurrence_key'] for x in inv['observations']],
                observationIds=[x['id'] for x in inv['observations']],runId=inv['run']['id'])


def failure_fixture(root):
    (root/'Usage.java').write_text('import java.security.Signature;\nclass Usage { void check() throws Exception { Signature.getInstance("SHA256withRSA"); } }\n')
    (root/'repeated.js').write_text('const data = \"' + 'RS256 '*10000 + '\";\n')
    (root/'oversized.py').write_bytes(b'#'+b'x'*(2*1024**2))


def study(args):
    binary=Path(args.binary).resolve(strict=True);out=Path(args.out_dir).resolve()
    # Native runs keep the old target-only boundary; the container has its own bounded evidence root.
    allowed=REPO/'target'
    if os.environ.get('PQC050_CONTAINER')=='1' and REPO==Path('/source'):allowed=Path('/scratch/evidence')
    if not out.is_relative_to(allowed) or out==allowed:raise ValueError('output outside evidence boundary')
    if out.exists() and any(out.iterdir()):raise ValueError('refusing to overwrite evidence')
    out.mkdir(parents=True,exist_ok=True)
    env=environment(binary);env.update(buildProfile=args.build_profile,features=args.features_label,
        constraints=json.loads(os.environ['PQC050_CONSTRAINTS']) if 'PQC050_CONSTRAINTS' in os.environ else None,
        provider='no TLS/SSH provider exercised by static scan',loadBefore=list(os.getloadavg()) if hasattr(os,'getloadavg') else None)
    write_json(out/'environment.json',env)
    results=[];failures=[]
    with tempfile.TemporaryDirectory(prefix='pqc050-') as tmp:
        tmp=Path(tmp)
        for scale,copies in [('small',1),('medium',4),('large',8)]:
            root=tmp/scale;info=materialize(root,copies)
            write_json(out/(scale+'-input.json'),info)
            for threads in [1,2]:
                for n in range(4):
                    name=f'{scale}-t{threads}-{n}';directory=out/name
                    cmd=[str(binary),'scan',str(root),'--format','all','--diagnostics','--cyclonedx-17','--inventory-json','--policy-profile','migration-review-v1','--repository-scope','quality050','--threads',str(threads),'--rules-dir',str(REPO/'rules'),'--out-dir',str(directory)]
                    sample=measure(cmd,directory,args.timeout)
                    sample.update(name=name,scale=scale,threads=threads,cacheClass='first-measured' if n==0 else 'warm',index=n,clockUtc=datetime.datetime.now(datetime.timezone.utc).isoformat(),input={k:info[k] for k in ['files','bytes','sha256']})
                    sample['throughput']=throughput(sample,info)
                    if sample['throughput'] is None:
                        write_json(directory/'measurement.json',sample);raise RuntimeError('matrix CLI failed: '+name)
                    outputs,facts,report=inspect_outputs(directory)
                    facts={key:dict(count=len(value),uniqueCount=len(set(canonical(x) for x in value)),multisetDigest=digest(sorted(value,key=canonical)),orderDigest=digest(value)) if isinstance(value,list) else value for key,value in facts.items()}
                    sample.update(outputs=outputs,facts=facts,scannedFiles=report['summary']['scanned_files'])
                    if sample['scannedFiles']!=info['files']:raise ValueError('actual scan input count mismatch')
                    verify_input(root,info)
                    sample['inputVerifiedAfter']=True
                    write_json(directory/'measurement.json',sample);results.append(sample)
                    # All source output bytes retained compressed; avoid extraction file/size limits.
                    write_json(out/(name+'.json'),sample)
                    # Keep one complete specimen per study; every other output has raw byte,
                    # semantic and order hashes measured before deletion. This is explicit
                    # retention sampling, not a claim that all original bytes are archived.
                    if scale=='small' and threads==1 and n==0:
                        with tarfile.open(out/(name+'.tar.gz'),'w:gz') as archive:
                            for p in sorted(directory.iterdir()):archive.add(p,arcname=p.name,recursive=False)
                    shutil.rmtree(directory)
                    print(name,round(sample['wallSeconds'],3),flush=True)
        # Real product failures/partial results, kept distinct from success throughput.
        root=tmp/'failures';root.mkdir();failure_fixture(root)
        for name,extra,timeout in [('threshold',['--fail-on','high'],args.timeout),('incomplete',['--require-complete'],args.timeout),('invalid-rules',['--rules-dir',str(tmp/'absent-rules')],args.timeout),('deadline',[],0.001)]:
            directory=out/('failure-'+name)
            command=[str(binary),'scan',str(root),'--format','all','--diagnostics','--threads','1','--rules-dir',str(REPO/'rules'),'--out-dir',str(directory)]
            if name=='invalid-rules':command=command[:-2];command[command.index('--rules-dir')+1]=str(tmp/'absent-rules');command+=['--out-dir',str(directory)]
            else:command+=extra
            sample=measure(command,directory,timeout);sample.update(name=name,expectedFailure=True,throughput=throughput(sample,inventory(root)),retainedOutputs={p.name:dict(bytes=p.stat().st_size,sha256=bytes_digest(p)) for p in directory.iterdir() if p.is_file()})
            if sample['exitCode']==0 and not sample['timedOut']:raise ValueError('failure fixture unexpectedly succeeded: '+name)
            if name in {'threshold','incomplete'}:
                r=json.loads((directory/'report.json').read_text());d=json.loads((directory/'diagnostics.json').read_text())
                if not r['findings'] or not d['coverage']['excluded_enumerated_files']:raise ValueError('failed to retain findings/limit')
                sample.update(retainedFindingCount=len(r['findings']),coverage=d['coverage'])
            write_json(directory/'measurement.json',sample);failures.append(sample)
            with tarfile.open(out/('failure-'+name+'.tar.gz'),'w:gz') as archive:
                for p in sorted(directory.iterdir()):archive.add(p,arcname=p.name,recursive=False)
            shutil.rmtree(directory)
    write_json(out/'samples.json',results);write_json(out/'failures.json',failures)
    summarize(out,env,results,failures)
    return results


def validate_study(rows):
    expected={(scale,t,n) for scale in ['small','medium','large'] for t in [1,2] for n in range(4)}
    actual={(r['scale'],r['threads'],r['index']) for r in rows}
    if len(rows)!=24 or actual!=expected:raise ValueError('incomplete/duplicate matrix')
    for row in rows:
        if row['exitCode']!=0 or row['timedOut'] or row['wallSeconds']<=0:raise ValueError('failed sample in success matrix')
        if row['scannedFiles']!=row['input']['files'] or not row['inputVerifiedAfter']:raise ValueError('input accounting mismatch')
        if row['throughput']!=throughput(row,row['input']):raise ValueError('invalid throughput denominator')
        if set(row['outputs'])!=set(FORMATS):raise ValueError('missing output format')


def summarize(out,env,results,failures):
    validate_study(results)
    groups=[]
    for scale in ['small','medium','large']:
        for threads in [1,2]:
            rows=[r for r in results if r['scale']==scale and r['threads']==threads];warm=rows[1:]
            groups.append(dict(scale=scale,configuration={'threads':threads,'format':'all+inventory+assessment+cyclonedx17','samples':4},input=rows[0]['input'],environment=env,
                firstMeasured=rows[0]['name'],warmDistribution=distributions(warm),warmMax={k:max(r[k] for r in warm) if all(r[k] is not None for r in warm) else None for k in ['wallSeconds','peakRssBytes']}))
    write_json(out/'resources.json',dict(environment=env,groups=groups,samplesFile='samples.json'))
    write_json(out/'throughput.json',dict(denominator='all materialized regular files and actual bytes, including excluded files if any; scanned count checked separately',wall='launch + scanning + all output + process exit/reap; excludes build and postprocessing',samples=[{k:r[k] for k in ['name','input','scannedFiles','wallSeconds','throughput']} for r in results]))
    write_json(out/'scaling.json',dict(license='MIT, first-party crates/*/src/**/*.rs and Cargo manifests/lock from recorded source; no fixture code execution; copies are not independent real projects',groups=groups))
    semantic=[];ordering=[]
    for row in results:
        base=next(r for r in results if r['scale']==row['scale'] and r['threads']==1 and r['index']==0)
        semantic.append(dict(name=row['name'],baseline=base['name'],formats={n:row['outputs'][n]['semantic']==base['outputs'][n]['semantic'] for n in FORMATS},
            findingIdMultisetStable=row['facts']['findingIds']['multisetDigest']==base['facts']['findingIds']['multisetDigest'],assetIdMultisetStable=row['facts']['assetIds']['multisetDigest']==base['facts']['assetIds']['multisetDigest'],occurrenceKeyMultisetStable=row['facts']['occurrenceKeys']['multisetDigest']==base['facts']['occurrenceKeys']['multisetDigest'],runIdStable=row['facts']['runId']==base['facts']['runId'],observationIdsStable=row['facts']['observationIds']==base['facts']['observationIds']))
        ordering.append(dict(name=row['name'],baseline=base['name'],formats={n:row['outputs'][n]['order']==base['outputs'][n]['order'] for n in FORMATS},note='JSON known collection order compared separately; HTML/Markdown conservative text equality, no independent unordered semantic claim'))
    write_json(out/'semantic-reproducibility.json',dict(normalization='only report/diagnostics generated_at, inventory.run.generated_at, renderer Generated line, existing scoped certificate assessment clock; all IDs and duplicates retained',rows=semantic,runScopedIdentity='run and observation IDs deliberately retained; differences are measurements, not product-stability claims'))
    write_json(out/'ordering.json',dict(rows=ordering,stabilityCondition='only measured input/binary/config/environment and repeated samples; unchanged meaning does not imply unchanged order'))
    write_json(out/'failure-resources.json',dict(excludedFromSuccessDistributions=True,samples=failures))
    (out/'noise-study.md').write_text('# Noise and cache\n\nEach scale/thread group has 1 first-measured + 3 warm samples; min/median/max and all values in resources.json. No filesystem cache flush, no true cold claim. Materialization reads input; first-measured may already be cached. 5 ms polling bounds timer resolution, particularly significant on fast runs. Wall includes launch and output. Sequential measurement avoids intentional competing benchmarks; host load is not controlled.\n\nCurrent date: '+env['clockUtc']+'\n')
    write_json(out/'environment-comparison.json',dict(environment=env,withinRun=[comparison(g,g) for g in groups],note='cross-run comparisons added by compare.py; hardware, OS, toolchain, features, source, input, constraints and configuration must match; static scan does not exercise TLS provider'))
    (out/'regression-review.md').write_text('# Regression policy\n\nPreserve TASK-023-ACCEPTANCE.md: unchanged eligible truth TP must not decrease and FP/FN must not increase. Same host/profile/config/input wall or RSS >2x previous warm maximum requires investigation, not automatic numeric CI failure. Threshold revision needs measured noise and maintainer approval. No new threshold or product optimization. Current matrix is new input, not directly comparable to historical corpus. See comparison evidence for preceding same-condition runs.\n')
