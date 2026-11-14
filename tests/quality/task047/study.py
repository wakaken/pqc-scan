"""Bounded, replayable byte mutation. No coverage feedback or unsafe product settings."""
from collections import Counter
import hashlib
import json
import math
import os
from pathlib import Path
import random
import re
import shutil
import time
from bounded_process import execute
MAX_INPUT=1024*1024

def digest(data):return hashlib.sha256(data).hexdigest()

def generate(seeds,seed,index):
    rng=random.Random(seed+index*104729)
    selected=index if index<len(seeds) else rng.randrange(len(seeds))
    item=seeds[selected];data=bytes.fromhex(item['hex'])
    if index>=len(seeds):
        k=index-len(seeds)
        if k<6:data=[b'',b'\0',data[:1],data[:max(0,len(data)-1)],b'\xff'*65536,b'\0'*MAX_INPUT][k]
        else:
            for _ in range(1+rng.randrange(4)):
                pos=rng.randrange(len(data)+1);op=rng.randrange(5)
                if op==0:data=data[:pos]+bytes([rng.randrange(256)])+data[pos+1:]
                elif op==1:data=data[:pos]
                elif op==2:data=(data[:pos]+rng.choice([b'\0',b'\xff',b'{}',b'\xff\xff\xff\xff',b'-----BEGIN',b'RSA_unknown'])+data[pos:])[:MAX_INPUT]
                elif op==3:data=(data*(1+rng.randrange(8)))[:MAX_INPUT]
                else:data=data[:pos]+data[min(len(data),pos+1+rng.randrange(32)):]
    assert len(data)<=MAX_INPUT
    return item['name'],data,selected

def minimize(data,fails,*,max_calls=256,seconds=30):
    """Deletion-minimal result, not minimum over arbitrary byte replacements."""
    current=data;chunks=2;calls=0;start_time=time.monotonic()
    while current:
        width=max(1,(len(current)+chunks-1)//chunks);reduced=False
        for start in range(0,len(current),width):
            if calls>=max_calls or time.monotonic()-start_time>seconds:raise RuntimeError("reduction budget exhausted; original failure retained, minimality unverified")
            candidate=current[:start]+current[start+width:];calls+=1
            if fails(candidate):current=candidate;chunks=max(2,chunks-1);reduced=True;break
        if not reduced:
            if width==1:break
            chunks=min(len(current),chunks*2)
    return current,calls

def trial(command,name,data,work):
    work=Path(work);work.mkdir(exist_ok=True,parents=True);path=work/'input.bin';path.write_bytes(data)
    previous={k:os.environ.get(k) for k in ['PQC_FUZZ_INPUT','PQC_FUZZ_NAME']}
    try:
        os.environ.update(PQC_FUZZ_INPUT=str(path),PQC_FUZZ_NAME=name)
        result=execute(command,work/'logs',timeout=2,cpu_seconds=2)
    finally:
        for key,value in previous.items():
            if value is None:os.environ.pop(key,None)
            else:os.environ[key]=value
    stdout=(work/'logs/stdout.log').read_text(errors='replace');stderr=(work/'logs/stderr.log').read_text(errors='replace')
    result['stdoutSha256']=digest(stdout.encode());result['stderrSha256']=digest(stderr.encode())
    if result['status']=='completed':
        feedback=[line[len('PQC047:'):] for line in stdout.splitlines() if line.startswith('PQC047:')]
        try:
            if len(feedback)!=1:raise ValueError('one observation required')
            result['feedback']=json.loads(feedback[0])
        except (ValueError,TypeError):result['status']='invalid_feedback'
    signature=re.search(r'panicked at ([^\n]+)',stderr)
    result['failureSignature']=[result['status'],signature.group(1) if signature else str(result['returncode'])]
    return result

def run_target(label,command,seeds,out,work,*,seconds,seed,max_cases=40000):
    if not math.isfinite(seconds) or seconds<=0 or type(max_cases) is not int or max_cases<=0 or not seeds or any(len(bytes.fromhex(s['hex']))>MAX_INPUT for s in seeds):raise ValueError('finite positive campaign limits and bounded nonempty seeds required')
    out=Path(out);out.mkdir(parents=True,exist_ok=False);work=Path(work)
    (out/'seeds.json').write_text(json.dumps(seeds,separators=(',',':'))+'\n')
    start=time.monotonic();counts=Counter();paths=Counter();failures=[];index=0
    with (out/'executions.jsonl').open('w') as records:
        while time.monotonic()-start<seconds and index<max_cases:
            name,data,selected=generate(seeds,seed,index)
            result=trial(command,name,data,work);counts[result['status']]+=1
            feedback=result.get('feedback',{})
            if 'paths' in feedback:
                for row in feedback['paths']:paths[f"{row['detector']}:{row['state']}"]+=1
            else:
                for key,value in feedback.items():paths[f'{key}:{value}']+=1
            recipe={'index':index,'seed':seed,'selectedSeed':selected,'name':name,'sha256':digest(data),'bytes':len(data)}
            records.write(json.dumps(dict(recipe,result=result),separators=(',',':'))+'\n');records.flush()
            if result['status']!='completed':
                failure=out/'failure';failure.mkdir();shutil.copytree(work,failure/'original-run');(failure/'original.bin').write_bytes(data)
                signature=result['failureSignature'];shutil.rmtree(work)
                confirmation=trial(command,name,data,work);shutil.copytree(work,failure/'confirmation');shutil.rmtree(work)
                if confirmation['failureSignature']!=signature:raise RuntimeError('non-reproducible failure; evidence retained')
                def fails(candidate):
                    observed=trial(command,name,candidate,work);shutil.rmtree(work)
                    return observed['failureSignature']==signature
                minimal,calls=minimize(data,fails);(failure/'minimal.bin').write_bytes(minimal)
                final=trial(command,name,minimal,work);shutil.copytree(work,failure/'minimal-run');shutil.rmtree(work)
                assert final['failureSignature']==signature
                detail={'recipe':recipe,'signature':signature,'originalBytes':len(data),'minimalBytes':len(minimal),'minimalSha256':digest(minimal),'reductionCalls':calls,'minimality':'single-byte deletion minimal; not globally smallest'}
                (failure/'reproduction.json').write_text(json.dumps(detail,indent=2)+'\n');failures.append(detail)
                break
            shutil.rmtree(work);index+=1
    result={'target':label,'command':command,'seed':seed,'seedCount':len(seeds),'seedHashes':[digest(bytes.fromhex(s['hex'])) for s in seeds],'configuredSeconds':seconds,'wallSeconds':time.monotonic()-start,'inputLimitBytes':MAX_INPUT,'perInputWallSeconds':2,'perInputCpuSeconds':2,'maxCases':max_cases,'executed':sum(counts.values()),'statuses':dict(counts),'reachedStates':dict(paths),'failures':failures,'stopReason':'failure' if failures else 'case_cap' if index==max_cases else 'time_budget','engine':'deterministic byte mutation, not coverage-guided','reachability':'observed entrypoint outcome counts; no line/branch percentage claimed'}
    (out/'result.json').write_text(json.dumps(result,indent=2)+'\n');return result
