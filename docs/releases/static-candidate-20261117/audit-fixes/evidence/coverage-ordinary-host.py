from pathlib import Path
import importlib.util,subprocess,json,tempfile,time,hashlib,traceback,uuid,shutil,tomllib,tarfile
root=Path.cwd();spec=importlib.util.spec_from_file_location('runner',root/'tests/protocol/container_runner.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
out=Path('/tmp/pqc-audit-fixes-20261117-9k0dmc45/coverage-ordinary');out.mkdir(exist_ok=False)
image='sha256:e9de9dffc2a84fc5c7ab438445c74f7466542f3149b54626b706e2646efbe527';name='pqc052-coverage-'+uuid.uuid4().hex
record={'image':image,'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'sourceBindings':{},'environment':m.ENV,'limits':{'network':'none','readOnlyRoot':True,'memoryBytes':6*1024**3,'swapBytes':0,'scratchBytes':4*1024**3,'cpus':2,'pids':128,'wallSeconds':1000,'cpuSeconds':900,'fileBytes':1024**3},'exit':None,'containerRemoved':False}
try:
 with tempfile.TemporaryDirectory(prefix='pqc052-build-input-') as td:
  td=Path(td);source=td/'source';record['sourceBindings']=m.snapshot(root,source);(source/'target').symlink_to('/scratch/test-target',target_is_directory=True)
  for p in [source,*[p for p in source.rglob('*') if p.is_dir()]]:p.chmod(0o755)
  cache=td/'cache';registry=cache/'registry';registry.mkdir(parents=True)
  cache_records=[];host_registry=Path('/Users/waka/.cargo/registry');index_name='index.crates.io-1949cf8c6b5b557f'
  for package in tomllib.loads((root/'Cargo.lock').read_text())['package']:
   if not package.get('source','').startswith('registry+'):continue
   n=package['name'];v=package['version'];prefix=('1/'+n if len(n)==1 else '2/'+n if len(n)==2 else '3/'+n[0]+'/'+n if len(n)==3 else n[:2]+'/'+n[2:4]+'/'+n)
   for relative in [f'cache/{index_name}/{n}-{v}.crate',f'index/{index_name}/.cache/{prefix}']:
    src=host_registry/relative;dst=registry/relative;assert src.is_file() and not src.is_symlink();dst.parent.mkdir(parents=True,exist_ok=True)
    if not dst.exists():shutil.copyfile(src,dst)
    sha=m.digest(dst)
    if relative.endswith('.crate'):assert sha==package['checksum'],n
    cache_records.append({'path':relative,'sha256':sha})
  unpacked=0;unpacked_files=0
  for package in tomllib.loads((root/'Cargo.lock').read_text())['package']:
   if not package.get('source','').startswith('registry+'):continue
   prefix=package['name']+'-'+package['version'];archive=registry/'cache'/index_name/(prefix+'.crate');dest=registry/'src'/index_name;dest.mkdir(parents=True,exist_ok=True)
   with tarfile.open(archive,'r:gz') as tf:
    for member in tf:
     path=Path(member.name);assert not path.is_absolute() and '..' not in path.parts and path.parts[0]==prefix
     target=dest/path
     if member.isdir():target.mkdir(parents=True,exist_ok=True);continue
     assert member.isfile() and not member.issym() and not member.islnk() and member.size<=32*1024**2
     unpacked+=member.size;unpacked_files+=1;assert unpacked<=1024**3 and unpacked_files<=100000
     target.parent.mkdir(parents=True,exist_ok=True)
     with tf.extractfile(member) as src,target.open('xb') as dst:shutil.copyfileobj(src,dst)
   (dest/prefix/'.cargo-ok').write_text('{"v":1}')
  record['readonlyCacheExtractedBytes']=unpacked;record['readonlyCacheExtractedFiles']=unpacked_files
  (registry/'index'/index_name/'config.json').write_text('{"dl":"https://static.crates.io/crates","api":"https://crates.io"}')
  record['existingCacheInputs']=cache_records
  probe=td/'probe.py';shutil.copyfile('/tmp/pqc-audit-fixes-20261117-9k0dmc45/coverage-ordinary-entry.py',probe);probe.chmod(0o444);record['probeSha256']=m.digest(probe)
  args=['docker','create','--name',name,'--network','none','--read-only','--user','65534:65534','--cap-drop','ALL','--security-opt','no-new-privileges','--memory',str(6*1024**3),'--memory-swap',str(6*1024**3),'--cpus','2','--pids-limit','128','--log-driver','none','--shm-size','1m','--ulimit','cpu=900:900','--ulimit','fsize=1073741824:1073741824','--ulimit','nofile=1024:1024','--ulimit','core=0:0','--tmpfs','/scratch:rw,exec,nosuid,nodev,size=4294967296,mode=1777','--mount',f'type=bind,src={source},dst=/source,readonly','--mount',f'type=bind,src={registry},dst=/usr/local/cargo/registry,readonly','--mount',f'type=bind,src={probe},dst=/probe.py,readonly','--workdir','/source','--entrypoint','/usr/bin/env',image,'-i',*[k+'='+v for k,v in m.ENV.items()],'/usr/bin/python3','/probe.py']
  record['command']=args;subprocess.run(args,check=True,stdout=subprocess.DEVNULL,timeout=30)
  inspect=json.loads(subprocess.check_output(['docker','inspect',name],timeout=10))[0];hc=inspect['HostConfig']
  assert hc['NetworkMode']=='none' and hc['ReadonlyRootfs'] and hc['Memory']==hc['MemorySwap']==6*1024**3 and hc['PidsLimit']==128 and all(not p['RW'] for p in inspect['Mounts'])
  record['verifiedContainerControls']={k:hc[k] for k in ['NetworkMode','ReadonlyRootfs','Memory','MemorySwap','PidsLimit','NanoCpus','Tmpfs','SecurityOpt','CapDrop']}
  subprocess.run(['docker','start',name],check=True,stdout=subprocess.DEVNULL,timeout=30);start=time.monotonic()
  while time.monotonic()-start<1000:
   c=subprocess.run(['docker','exec',name,'/bin/cat','/scratch/status.json'],capture_output=True,text=True,timeout=5)
   if c.returncode==0:record['exit']=json.loads(c.stdout)['exit'];break
   state=json.loads(subprocess.check_output(['docker','inspect',name],timeout=10))[0]['State']
   if not state['Running']:raise RuntimeError('container stopped before result: '+str(state['ExitCode']))
   time.sleep(2)
  else:raise RuntimeError('wall deadline exceeded')
  record['wallSeconds']=time.monotonic()-start
  data=m.bounded_output(['docker','exec',name,'/bin/tar','-C','/scratch/evidence','-cf','-','.'],limit=64*1024**2,timeout=30);m.extract_artifacts(data,out)
  record['sourceBeforeAfterMatch']=all(hashlib.sha256(m.source_bytes(root,n)).hexdigest()==v for n,v in record['sourceBindings'].items())
except BaseException:record['error']=traceback.format_exc()
finally:
 c=subprocess.run(['docker','rm','-f',name],capture_output=True,text=True,timeout=30);record['containerRemoved']=c.returncode==0
 shutil.copyfile('/tmp/pqc-audit-fixes-20261117-9k0dmc45/coverage-ordinary-entry.py',out/'probe.py.txt');(out/'HOST-RESULTS.json').write_text(json.dumps(record,indent=2)+'\n');shutil.copyfile('/tmp/pqc-audit-fixes-20261117-9k0dmc45/coverage-ordinary-host.py',out/'controller.py.txt');print(json.dumps({k:record.get(k) for k in ['exit','wallSeconds','containerRemoved','error']}))
