"""Sequential default/all-feature checks, preserving command and source evidence."""
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[3]


def main():
    out = Path(sys.argv[1]).resolve()
    out.mkdir(parents=True, exist_ok=True)
    (out/'logs').mkdir(exist_ok=True)
    env = dict(os.environ, PQC048_EVIDENCE=str(out/'raw'))
    files = subprocess.check_output(['git','ls-files'], cwd=ROOT, text=True).splitlines()
    files += subprocess.check_output(['git','ls-files','--others','--exclude-standard'], cwd=ROOT, text=True).splitlines()
    source = {p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in files if (ROOT/p).is_file()}
    (out/'SOURCE.json').write_text(json.dumps({'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'branch':subprocess.check_output(['git','branch','--show-current'],cwd=ROOT,text=True).strip(),'platform':platform.platform(),'files':source},indent=2)+'\n')
    commands = [('fmt',['cargo','fmt','--all','--check'])]
    for tag, features in [('default',[]),('all-features',['--all-features'])]:
        for action, args in [('tests',['test','--workspace']),('clippy',['clippy','--workspace','--all-targets']),('build',['build','--workspace'])]:
            command = ['cargo',*args,'--locked','--offline',*features]
            if action == 'clippy': command += ['--','-D','warnings']
            commands.append((tag+'-'+action,command))
    commands += [('python',[sys.executable,'-m','unittest','discover','-s','tests/quality','-p','test_*.py']),('compact-js',['node','--test','shell/compact-report.test.cjs']),('assessment',[sys.executable,'tests/quality/task048/contracts.py','--raw',str(out/'raw'),'--out',str(out)])]
    records=[]
    for tag,command in commands:
        start=time.monotonic()
        with (out/'logs'/f'{tag}.log').open('w') as log:
            try:
                code=subprocess.run(command,cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=1200).returncode
            except subprocess.TimeoutExpired:
                code='timeout'
        records.append({'name':tag,'command':command,'features':'all-features' if tag.startswith('all-features') else 'default','exit':code,'seconds':time.monotonic()-start,'log':'logs/'+tag+'.log','evidenceEnvironment':{'PQC048_EVIDENCE':env['PQC048_EVIDENCE']}})
        (out/'VALIDATION.json').write_text(json.dumps(records,indent=2)+'\n')
        print(tag,code,flush=True)
        if code != 0: return 1
    return 0


if __name__ == '__main__': sys.exit(main())
