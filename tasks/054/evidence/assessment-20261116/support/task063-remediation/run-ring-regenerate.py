from pathlib import Path
import subprocess,json,hashlib
r=Path('.').resolve();p=r/'target/quality-task-063/ring-regenerate';u=r/'target/quality-task-063/upstream';image='sha256:16a329d30bd8f2b8d12353b678777392203d3e3f567505709a8ca4a408f95bb1'
script='''
import pathlib,tarfile,hashlib,json,subprocess,os,shutil
s=pathlib.Path('/scratch');(s/'cargo').mkdir();(s/'cargo/registry').symlink_to('/usr/local/cargo/registry');(s/'home').mkdir();(s/'tmp').mkdir();shutil.copytree('/probe',s/'probe')
root=s/'ring';root.mkdir()
with tarfile.open('/upstream/ring-2723abbca9e83347d82b056d5b239c6604f786df.tar.gz','r:gz') as t:
 for m in t:
  if not m.isfile():continue
  name=pathlib.PurePosixPath(*pathlib.PurePosixPath(m.name).parts[1:]);assert '..' not in name.parts and not name.is_absolute()
  p=root/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(t.extractfile(m).read())
subprocess.run(['cargo','build','--offline','--manifest-path',str(s/'probe/Cargo.toml'),'--target-dir',str(s/'target')],check=True,stdout=__import__('sys').stderr,timeout=120)
subprocess.run([str(s/'target/debug/pqc_ring_regenerate'),str(root)],check=True,stdout=__import__('sys').stderr,timeout=120)
rows=[]
with tarfile.open('/upstream/ring-0.17.14.crate','r:gz') as t:
 for m in t:
  if not m.isfile():continue
  name='/'.join(pathlib.PurePosixPath(m.name).parts[1:])
  if not name.startswith('pregenerated/'):continue
  expected=t.extractfile(m).read();p=root/name;actual=p.read_bytes() if p.exists() else None
  rows.append({'path':name,'archive_sha256':hashlib.sha256(expected).hexdigest(),'regenerated_sha256':hashlib.sha256(actual).hexdigest() if actual is not None else None,'matches':actual==expected,'scope':'Windows object not assembled (NASM absent)' if name.endswith('.o') else 'Regenerated from recorded upstream source'})
print(json.dumps(rows,indent=2))
'''
cmd=['docker','run','--rm','--network','none','--read-only','--user','65534:65534','--cap-drop','ALL','--security-opt','no-new-privileges','--memory','1g','--memory-swap','1g','--cpus','2','--pids-limit','64','--tmpfs','/scratch:rw,exec,nosuid,nodev,size=1g,mode=1777','--ulimit','cpu=240:240','--ulimit','fsize=134217728:134217728','--mount',f'type=bind,src={p},dst=/probe,readonly','--mount',f'type=bind,src={u},dst=/upstream,readonly','--entrypoint','/usr/bin/env',image,'-i','PATH=/usr/local/cargo/bin:/usr/bin:/bin','HOME=/scratch/home','TMPDIR=/scratch/tmp','CARGO_HOME=/scratch/cargo','RUSTUP_HOME=/usr/local/rustup','CARGO_NET_OFFLINE=true','python3','-c',script]
with (p/'RESULTS.json').open('w') as out,(p/'RUN.log').open('w') as err:c=subprocess.run(cmd,stdout=out,stderr=err,timeout=300)
(p/'RUN.json').write_text(json.dumps({'image':image,'command':cmd,'exit':c.returncode,'driver_sha256':hashlib.sha256((p/'src/main.rs').read_bytes()).hexdigest()},indent=2)+'\n');print('regeneration exit',c.returncode)
if c.returncode==0:
 rows=json.loads((p/'RESULTS.json').read_text());print('matched',sum(x['matches'] for x in rows),'total',len(rows));print([(x['path'],x['scope']) for x in rows if not x['matches']][:6])
