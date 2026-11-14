"""TASK050 container-only release builds and sequential measurements."""
import json
import os
from pathlib import Path
import subprocess
import sys
import time
sys.path[:0]=['/source/tests/protocol','/source/tests/quality']
from container_entry import EVIDENCE,SCRATCH,write,boundary


def main():
    checks=boundary();write('checks.json',checks)
    cargo=SCRATCH/'cargo';cargo.mkdir()
    for name in ['registry','git']:
        cache=Path('/usr/local/cargo')/name
        if cache.exists():(cargo/name).symlink_to(cache,target_is_directory=True)
    commands=[]
    for mode,flags in [('default',[]),('all-features',['--all-features'])]:
        # Distinct target dirs, never overwrite a binary while measuring it.
        env=dict(os.environ,CARGO_TARGET_DIR=str(SCRATCH/('build-'+mode)),PQC050_CONTAINER='1',PQC050_CONSTRAINTS=json.dumps(checks))
        cmds=[('build',['cargo','build','--release','--locked','--offline','-p','pqc_scan_cli',*flags])]
        binary=Path(env['CARGO_TARGET_DIR'])/'release/pqc-scan'
        for repeat in [1,2]:
            cmds.append(('study-'+str(repeat),[sys.executable,'/source/tests/quality/run.py','--performance-study','--binary',str(binary),'--build-profile','release','--features-label',mode,'--out-dir',str(EVIDENCE/(mode+'-'+str(repeat)))]))
        for name,cmd in cmds:
            start=time.monotonic()
            with (EVIDENCE/(mode+'-'+name+'.log')).open('w') as log:code=subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=900).returncode
            commands.append(dict(name=mode+'-'+name,command=cmd,exit=code,seconds=time.monotonic()-start));write('VALIDATION.json',commands)
            if code:raise RuntimeError(mode+'-'+name)


if __name__=='__main__':main()
