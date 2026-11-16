from pathlib import Path
import subprocess,tempfile,hashlib,json
root=Path('.').resolve();p=root/'target/quality-task-063/yaml-probe';image=(root/'target/quality-task-063/preparation/yaml-image-id').read_text().strip()
with tempfile.TemporaryDirectory(prefix='pqc-yaml-corpus-') as tmp:
 corpus=Path(tmp);sources={}
 paths=[]
 for folder in ['rules','tests']:
  for ext in ['*.yml','*.yaml']:paths+=list((root/folder).rglob(ext))
 paths += [root/'tasks'/f'{i:03d}'/'task.yaml' for i in range(1,64) if (root/'tasks'/f'{i:03d}'/'task.yaml').exists()]
 for i,f in enumerate(sorted({f for f in paths if f.is_file() and not f.is_symlink()})):
  name=f'{i:04d}.yaml';data=f.read_bytes();assert len(data)<=2*1024**2;(corpus/name).write_bytes(data);sources[name]={'path':f.relative_to(root).as_posix(),'sha256':hashlib.sha256(data).hexdigest()}
 script='mkdir -p /scratch/home /scratch/tmp /scratch/cargo; ln -s /usr/local/cargo/registry /scratch/cargo/registry; cargo build --manifest-path /probe/Cargo.toml --target-dir /scratch/target --locked --offline 1>&2 && /scratch/target/debug/pqc_yaml_probe /corpus/*.yaml'
 cmd=['docker','run','--rm','--network','none','--read-only','--user','65534:65534','--cap-drop','ALL','--security-opt','no-new-privileges','--memory','1g','--memory-swap','1g','--cpus','2','--pids-limit','64','--tmpfs','/scratch:rw,exec,nosuid,nodev,size=1g,mode=1777','--ulimit','cpu=180:180','--ulimit','fsize=134217728:134217728','--ulimit','nofile=256:256','--mount',f'type=bind,src={p},dst=/probe,readonly','--mount',f'type=bind,src={corpus},dst=/corpus,readonly','--entrypoint','/usr/bin/env',image,'-i','PATH=/usr/local/cargo/bin:/usr/bin:/bin','HOME=/scratch/home','TMPDIR=/scratch/tmp','CARGO_HOME=/scratch/cargo','RUSTUP_HOME=/usr/local/rustup','CARGO_NET_OFFLINE=true','/bin/sh','-c',script]
 with (p/'RESULTS-r2.json').open('w') as out,(p/'build-r2.log').open('w') as err:r=subprocess.run(cmd,stdout=out,stderr=err,timeout=240)
 (p/'RUN-r2.json').write_text(json.dumps({'image':image,'command':cmd,'exit':r.returncode,'corpus':sources,'source':{f.name:hashlib.sha256(f.read_bytes()).hexdigest() for f in [p/'Cargo.toml',p/'Cargo.lock',p/'src/main.rs']}},indent=2)+'\n')
 print('probe exit',r.returncode)
 if r.returncode==0:
  records=json.loads((p/'RESULTS-r2.json').read_text());diff=[x for x in records if not x['same_value_or_error']];print('cases',len(records),'differences',len(diff));print(json.dumps(diff[:8],ensure_ascii=False,indent=2))
