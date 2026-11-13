"""Stable LLVM coverage of a declared replay subset; reuses TASK-044 accounting."""
import gzip,hashlib,json,os,resource,shutil,signal,subprocess,sys,time
from pathlib import Path
sys.path[:0]=['/source/tests/protocol','/source/tests/quality','/source/tests/quality/task047']
from container_entry import EVIDENCE,SCRATCH,boundary,write
from study import generate,digest
from task044.coverage import summarize

def main():
    write('checks.json',boundary())
    cargo=SCRATCH/'cargo';cargo.mkdir()
    for name in ['registry','git']:
        cache=Path('/usr/local/cargo')/name
        if cache.exists():(cargo/name).symlink_to(cache,target_is_directory=True)
    source=Path('/source/tests/quality/task047/coverage-inputs.json.gz')
    corpus=json.loads(gzip.decompress(source.read_bytes()))
    assert all(digest((Path('/source')/p).read_bytes())==h for p,h in corpus['ordinarySourceBindings'].items())
    write('corpus-binding.json',{'sha256':digest(source.read_bytes()),'sourceHashesMatchOrdinaryBuild':True,'scope':corpus['scope']})
    build_raw=SCRATCH/'build-raw';build_raw.mkdir()
    env=dict(os.environ,RUSTFLAGS='-C instrument-coverage',LLVM_PROFILE_FILE=str(build_raw/'%p-%m.profraw'))
    commands=[];artifacts=[]
    for i,selection in enumerate([['-p','pqc_scan_detectors','-p','pqc_scan_report','-p','pqc_scan_probe_ssh','--test','quality047'],['-p','pqc_scan_probe_tls','--lib']]):
        command=['cargo','test',*selection,'--all-features','--no-run','--locked','--offline','--message-format=json']
        with (EVIDENCE/f'build-{i}.jsonl').open('w') as out,(EVIDENCE/f'build-{i}.log').open('w') as err:
            start=time.monotonic();result=subprocess.run(command,env=env,stdout=out,stderr=err,timeout=700)
        commands.append({'command':command,'environment':env,'exit':result.returncode,'wallSeconds':time.monotonic()-start});write('commands.json',commands);assert result.returncode==0
        artifacts += [json.loads(s) for s in (EVIDENCE/f'build-{i}.jsonl').read_text().splitlines() if s.startswith('{')]
    binaries={}
    for row in artifacts:
        if row.get('reason')=='compiler-artifact' and row.get('executable'):
            for crate in ['pqc_scan_detectors','pqc_scan_report','pqc_scan_probe_ssh','pqc_scan_probe_tls']:
                if crate in row['package_id'] and ((row['target']['name']=='quality047') if crate!='pqc_scan_probe_tls' else row['target']['kind']==['lib']):binaries[crate]=row['executable']
    assert len(binaries)==4
    write('build-profiles.json',{p.name:digest(p.read_bytes()) for p in build_raw.glob('*')});shutil.rmtree(build_raw)
    sysroot=subprocess.check_output(['rustc','--print','sysroot'],text=True).strip();llvm=Path(sysroot)/'lib/rustlib/aarch64-unknown-linux-gnu/bin'
    (EVIDENCE/'llvm-version.log').write_text(subprocess.check_output([str(llvm/'llvm-cov'),'--version'],text=True))
    raw=SCRATCH/'raw';raw.mkdir();profile=SCRATCH/'fuzz.profdata';pending=SCRATCH/'pending.profdata';hashes={};observed=[]
    worker_env=dict(os.environ,LLVM_PROFILE_FILE=str(raw/'%p-%m.profraw'))
    def limits():
        resource.setrlimit(resource.RLIMIT_CORE,(0,0));resource.setrlimit(resource.RLIMIT_CPU,(2,3))
        # LLVM counters exceed the normal fuzz worker's2MiB file cap. This
        # measurement alone permits16MiB; outer aggregate limits stay unchanged.
        resource.setrlimit(resource.RLIMIT_FSIZE,(16*1024**2,16*1024**2))
    for campaign in corpus['campaigns']:
        label=campaign['target'];crate='pqc_scan_report' if label.startswith('output') else 'pqc_scan_probe_ssh' if label=='SSH' else 'pqc_scan_probe_tls' if label=='TLS' else 'pqc_scan_detectors'
        exact='runtime::quality047::quality047_worker' if label=='TLS' else 'quality047_worker';command=[binaries[crate],'--ignored','--exact',exact,'--nocapture']
        for row in campaign['rows']:
            name,data,_=generate(campaign['seeds'],row['seed'],row['index']);assert digest(data)==row['sha256']
            input_file=SCRATCH/'input.bin';input_file.write_bytes(data);worker_env.update(PQC_FUZZ_INPUT=str(input_file),PQC_FUZZ_NAME=name)
            start=time.monotonic()
            with (EVIDENCE/'worker.stdout').open('w') as out,(EVIDENCE/'worker.stderr').open('w') as err:
                child=subprocess.Popen(command,env=worker_env,stdout=out,stderr=err,start_new_session=True,preexec_fn=limits)
                killed=False
                try:code=child.wait(timeout=2)
                except subprocess.TimeoutExpired:os.killpg(child.pid,signal.SIGKILL);killed=True;child.wait();raise
                finally:
                    if not killed:
                        try:os.killpg(child.pid,signal.SIGKILL)
                        except ProcessLookupError:pass
            assert code==0,(label,row['index'],code)
            stdout=(EVIDENCE/'worker.stdout').read_text();stderr=(EVIDENCE/'worker.stderr').read_text()
            assert len(stdout.encode())<2*1024**2 and len(stderr.encode())<2*1024**2
            feedback=[json.loads(s[7:]) for s in stdout.splitlines() if s.startswith('PQC047:')];assert feedback==[row['result']['feedback']],(label,row['index'])
            files=sorted(raw.glob('*.profraw'));assert files;hashes.update({p.name:digest(p.read_bytes()) for p in files})
            merge=[str(llvm/'llvm-profdata'),'merge','-sparse',*([str(profile)] if profile.exists() else []),*[str(p) for p in files],'-o',str(pending)]
            subprocess.run(merge,check=True,stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,timeout=15);pending.replace(profile)
            for p in files:p.unlink()
            observed.append({'target':label,'index':row['index'],'seed':row['seed'],'inputSha256':row['sha256'],'ordinaryFeedbackMatches':True,'wallSeconds':time.monotonic()-start})
            write('profiled-inputs.json',observed)
    objects=[s for path in sorted(binaries.values()) for s in ['-object',path]]
    command=[str(llvm/'llvm-cov'),'export','-instr-profile='+str(profile),'-ignore-filename-regex=^(/scratch|/rustc|/usr)|/tests/','-skip-functions','-skip-expansions',*objects]
    with (EVIDENCE/'coverage.json').open('w') as out,(EVIDENCE/'export.stderr.log').open('w') as err:result=subprocess.run(command,stdout=out,stderr=err,timeout=120)
    assert result.returncode==0;summary=summarize(json.loads((EVIDENCE/'coverage.json').read_text()),branches=False)
    summary.update({'profiledInputs':len(observed),'scope':corpus['scope'],'denominator':'linked Rust source only; source-located cfg(test) blocks included; no dependency/FFI C/whole-product claim','branchCoverage':'Not Measured on stable; regions are not branches','binaries':{k:{'path':v,'sha256':digest(Path(v).read_bytes())} for k,v in binaries.items()},'rawProfileHashes':hashes,'profdataSha256':digest(profile.read_bytes()),'exportCommand':command,'exportExit':0,'workerLimits':{'wallSeconds':2,'cpuSeconds':2,'fileBytes':16*1024**2,'coreBytes':0},'ordinaryFuzzFileCapUnchanged':2*1024**2})
    summary['unmappedSourceFiles']=sorted(str(p.relative_to('/source')) for p in Path('/source/crates').glob('*/src/**/*.rs') if str(p.relative_to('/source')) not in summary['files'])
    write('summary.json',summary)
    for src,dst in [(EVIDENCE/'coverage.json',EVIDENCE/'coverage.json.gz'),(profile,EVIDENCE/'coverage.profdata.gz')]:
        with src.open('rb') as inp,gzip.open(dst,'wb') as out:shutil.copyfileobj(inp,out)
    (EVIDENCE/'coverage.json').unlink();write('COMPLETE.json',{'exit':0,'profiledInputs':len(observed),'sourceHashesMatchOrdinaryBuild':True})

if __name__=='__main__':main()
