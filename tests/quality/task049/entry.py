"""TASK-049 measured only inside the existing finite offline container."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time
sys.path[:0]=['/source/tests/protocol','/source/tests/quality/task049']
from container_entry import EVIDENCE,SCRATCH,write,boundary
from assess import collect


def load_json(path):return json.loads(path.read_text())


def main():
    write('checks.json',boundary());(SCRATCH/'test-target').mkdir()
    cargo=SCRATCH/'cargo';cargo.mkdir()
    for name in ['registry','git']:
        cache=Path('/usr/local/cargo')/name
        if cache.exists():(cargo/name).symlink_to(cache,target_is_directory=True)
    env=dict(os.environ,PQC049_EVIDENCE=str(EVIDENCE/'raw'));commands=[];binaries={}
    def run(name,command,environment=env,timeout=900):
        start=time.monotonic()
        with (EVIDENCE/(name+'.log')).open('w') as log:code=subprocess.run(command,env=environment,stdout=log,stderr=subprocess.STDOUT,timeout=timeout).returncode
        commands.append({'name':name,'command':command,'exit':code,'seconds':time.monotonic()-start,'evidenceEnvironment':{'PQC049_EVIDENCE':environment['PQC049_EVIDENCE']},'log':name+'.log'});write('VALIDATION.json',commands)
        if code:raise RuntimeError(name)
    run('fmt',['cargo','fmt','--all','--check'])
    for mode,options in [('default',[]),('all-features',['--all-features'])]:
        for name,args in [('tests',['test','--workspace']),('clippy',['clippy','--workspace','--all-targets']),('build',['build','--workspace'])]:
            cmd=['cargo',*args,'--locked','--offline',*options]
            if name=='clippy':cmd+=['--','-D','warnings']
            run(mode+'-'+name,cmd)
        binary=SCRATCH/'target/debug/pqc-scan';binaries[mode]={'path':str(binary),'sha256':hashlib.sha256(binary.read_bytes()).hexdigest()};write('BINARIES.json',binaries)
    run('openssh-offers',['cargo','test','--locked','--offline','-p','pqc_scan_probe_ssh','--all-features','--test','openssh','--','--ignored','--nocapture'],dict(env,PQC_SCAN_TEST_SSH_HYBRID='sntrup761x25519-sha512@openssh.com'))
    run('openssh-kex',[sys.executable,'/source/tests/protocol/ssh-kex-product.py','--inside','--binary',str(SCRATCH/'target/debug/pqc-scan'),'--out-dir',str(EVIDENCE/'ssh-kex')],timeout=120)
    assert load_json(EVIDENCE/'ssh-kex/RESULTS.json')['binary_sha256']==binaries['all-features']['sha256']
    run('tools',['sh','-c','rustc -Vv; cargo -V; /usr/sbin/sshd -V 2>&1; /opt/openssh/sbin/sshd -V 2>&1; python3 -V'],timeout=30)
    # Build the library/test inventory using the same final feature configuration.
    run('test-binaries',['cargo','test','--workspace','--all-features','--locked','--offline','--no-run','--message-format=json'])
    artifacts={}
    for line in (EVIDENCE/'test-binaries.log').read_text().splitlines():
        if not line.startswith('{'):continue
        row=json.loads(line)
        if row.get('reason')=='compiler-artifact' and row.get('executable'):
            p=Path(row['executable']);artifacts[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
    write('TEST-BINARIES.json',artifacts)
    collect(EVIDENCE/'raw',EVIDENCE)


if __name__=='__main__':main()
