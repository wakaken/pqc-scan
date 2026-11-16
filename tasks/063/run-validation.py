#!/usr/bin/env python3
"""Task-local offline validation launcher; reuses existing confinement primitives."""
import argparse,hashlib,importlib.util,json,subprocess,tempfile,time,uuid
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
spec=importlib.util.spec_from_file_location('confined',ROOT/'tests/protocol/container_runner.py');c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c)
ENTRY='''
import hashlib,json,os,subprocess,sys,time,traceback
from pathlib import Path
sys.path.insert(0,'/source/tests/protocol')
from container_entry import boundary
s=Path('/scratch');e=s/'evidence'
for p in [e,s/'home',s/'tmp',s/'test-target',s/'cargo']:p.mkdir(exist_ok=True)
for name in ['registry','git']:
 p=Path('/usr/local/cargo')/name
 if p.exists():(s/'cargo'/name).symlink_to(p)
checks=boundary();(e/'boundary.json').write_text(json.dumps(checks,indent=2)+'\\n')
records=[];status=0
try:
 for mode,flags in [('default',[]),('combined-probes',['--features','pqc_scan_cli/probe-tls,pqc_scan_cli/probe-ssh']),('all-features',['--all-features'])]:
  env=dict(os.environ,CARGO_TARGET_DIR='/scratch/build-'+mode)
  for name,args in [('build',['cargo','build','--workspace','--locked','--offline',*flags]),('test',['cargo','test','--workspace','--locked','--offline',*flags]),('clippy',['cargo','clippy','--workspace','--all-targets','--locked','--offline',*flags,'--','-D','warnings'])]:
   with (e/(mode+'-'+name+'.log')).open('w') as log:
    started=time.monotonic();r=subprocess.run(args,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=750)
   records.append({'mode':mode,'command':args,'exit':r.returncode,'seconds':time.monotonic()-started});(e/'COMMANDS.json').write_text(json.dumps(records,indent=2)+'\\n')
   if r.returncode:raise RuntimeError(mode+'-'+name)
 with (e/'distribution.log').open('w') as log:r=subprocess.run(['python3','-m','unittest','discover','-s','tests/distribution','-v'],stdout=log,stderr=subprocess.STDOUT,timeout=60)
 records.append({'command':['python3','-m','unittest','discover','-s','tests/distribution','-v'],'exit':r.returncode})
 if r.returncode:raise RuntimeError('distribution')
 with (e/'ssh-kex.log').open('w') as log:r=subprocess.run(['python3','tests/protocol/ssh-kex-product.py','--inside','--binary','/scratch/build-all-features/debug/pqc-scan','--out-dir',str(e/'ssh-kex')],stdout=log,stderr=subprocess.STDOUT,timeout=120)
 records.append({'command':['python3','tests/protocol/ssh-kex-product.py','--inside','--binary','/scratch/build-all-features/debug/pqc-scan','--out-dir',str(e/'ssh-kex')],'exit':r.returncode})
 if r.returncode:raise RuntimeError('ssh-kex')
except BaseException:
 status=1;(e/'FAILURE.txt').write_text(traceback.format_exc())
(e/'COMMANDS.json').write_text(json.dumps(records,indent=2)+'\\n')
(e/'BINARIES.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for mode in ['default','all-features'] for p in (s/('build-'+mode)/'debug').glob('pqc-scan')},indent=2)+'\\n')
(s/'status.json').write_text(json.dumps({'exit':status}))
while True:time.sleep(60)
'''
def main():
 p=argparse.ArgumentParser();p.add_argument('--image',required=True);p.add_argument('--out-dir',type=Path,required=True);p.add_argument('--all-features-only',action='store_true');a=p.parse_args()
 entry=ENTRY
 if a.all_features_only:entry=entry.replace("[('default',[]),('combined-probes',['--features','pqc_scan_cli/probe-tls,pqc_scan_cli/probe-ssh']),('all-features',['--all-features'])]","[('all-features',['--all-features'])]")
 import re
 if not re.fullmatch(r'sha256:[0-9a-f]{64}',a.image):p.error('immutable image required')
 out=a.out_dir.resolve();out.mkdir(parents=True,exist_ok=False);name='pqc063-'+uuid.uuid4().hex;record={'image':a.image,'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'exit':None}
 try:
  with tempfile.TemporaryDirectory(prefix='pqc063-source-') as t:
   source=Path(t)/'source';bindings=c.snapshot(ROOT,source)
   extra=['SECURITY.md','.github/workflows/release.yml']
   for folder in ['scripts','third_party/notices','docs/user']:
    extra += [p.relative_to(ROOT).as_posix() for p in (ROOT/folder).rglob('*') if p.is_file() and '__pycache__' not in p.parts]
   for n in extra:
    b=c.source_bytes(ROOT,n);d=source/n;d.parent.mkdir(parents=True,exist_ok=True);d.write_bytes(b);d.chmod(0o444);bindings[n]=hashlib.sha256(b).hexdigest()
   (source/'target').symlink_to('/scratch/test-target',target_is_directory=True);(source/'validate063.py').write_text(entry)
   record.update(source_bindings=bindings,entry_sha256=hashlib.sha256(entry.encode()).hexdigest(),entry=entry)
   env=dict(c.ENV,CARGO_TARGET_DIR='/scratch/build-default')
   cmd=['docker','create','--name',name,'--network','none','--read-only','--user','65534:65534','--log-driver','none','--cap-drop','ALL','--security-opt','no-new-privileges','--memory','6g','--memory-swap','6g','--cpus','2','--pids-limit','128','--shm-size','1m','--ulimit','cpu=1500:1500','--ulimit','fsize=1073741824:1073741824','--ulimit','nofile=1024:1024','--tmpfs','/scratch:rw,exec,nosuid,nodev,size=4g,mode=1777','--mount',f'type=bind,src={source},dst=/source,readonly','--workdir','/source','--entrypoint','/usr/bin/env',a.image,'-i',*[f'{k}={v}' for k,v in env.items()],'python3','/source/validate063.py']
   record['command']=cmd;subprocess.run(cmd,check=True,capture_output=True,timeout=30);subprocess.run(['docker','start',name],check=True,capture_output=True,timeout=30)
   end=time.monotonic()+1800
   while True:
    if time.monotonic()>end:raise TimeoutError('validation deadline')
    try:status=json.loads(c.bounded_output(['docker','exec',name,'cat','/scratch/status.json'],limit=4096,timeout=5));break
    except RuntimeError:
     state=json.loads(subprocess.check_output(['docker','inspect',name],timeout=10))[0]['State']
     if not state['Running']:raise RuntimeError('container stopped before export')
     time.sleep(1)
   record['exit']=status['exit'];data=c.bounded_output(['docker','exec',name,'tar','-C','/scratch/evidence','-cf','-','.'],limit=32*1024**2,timeout=30);c.extract_artifacts(data,out/'artifacts',max_bytes=32*1024**2)
   record['source_unchanged']=all(c.digest(ROOT/n)==h for n,h in bindings.items())
   if not record['source_unchanged']:raise RuntimeError('source changed')
 except BaseException as error:
  record['error']=type(error).__name__;record['exit']=1;raise
 finally:
  r=subprocess.run(['docker','rm','--force',name],capture_output=True,timeout=30);record['container_removed']=r.returncode==0;(out/'RUN.json').write_text(json.dumps(record,indent=2)+'\n')
 print(json.dumps({'exit':record['exit'],'container_removed':record['container_removed']}));return record['exit'] if record['container_removed'] else 1
if __name__=='__main__':raise SystemExit(main())
