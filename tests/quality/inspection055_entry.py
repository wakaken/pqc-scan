"""TASK-055 verification, executed only by the existing offline container runner."""
import os
from pathlib import Path
import subprocess
import sys
import time
sys.path[:0]=['/source/tests/protocol','/source/tests/quality']
from container_entry import boundary,write,SCRATCH,EVIDENCE
from inspection055 import collect


def main():
    write('checks.json',boundary())
    (SCRATCH/'test-target').mkdir()
    cargo=SCRATCH/'cargo';cargo.mkdir()
    for name in ['registry','git']:
        cache=Path('/usr/local/cargo')/name
        if cache.exists():(cargo/name).symlink_to(cache,target_is_directory=True)
    commands=[]
    for mode in ['default','all-features']:
        env=dict(os.environ,PQC_QUALITY_EVIDENCE_DIR=str(EVIDENCE/mode/'raw'))
        options=[] if mode=='default' else ['--all-features']
        for name,args in [
            ('tests',['cargo','test','--workspace','--locked','--offline',*options]),
            ('clippy',['cargo','clippy','--workspace','--all-targets','--locked','--offline',*options,'--','-D','warnings']),
            ('build',['cargo','build','--workspace','--locked','--offline',*options])]:
            start=time.monotonic();log=f'{mode}-{name}.log'
            with (EVIDENCE/log).open('w') as stream:
                result=subprocess.run(args,env=env,stdout=stream,stderr=subprocess.STDOUT,timeout=900)
            commands.append({'command':args,'environment':env,'log':log,'exit':result.returncode,'wallSeconds':time.monotonic()-start})
            write('commands.json',commands)
            if result.returncode:raise RuntimeError(log)
        collect(EVIDENCE/mode/'raw',EVIDENCE/mode/'assessment')
    with (EVIDENCE/'fmt.log').open('w') as stream:
        result=subprocess.run(['cargo','fmt','--all','--check'],stdout=stream,stderr=subprocess.STDOUT,timeout=60)
    if result.returncode:raise RuntimeError('formatting failed')


if __name__=='__main__':main()
