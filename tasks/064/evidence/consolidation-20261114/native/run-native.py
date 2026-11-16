from pathlib import Path
import sys,tempfile,subprocess,json,time,hashlib,tarfile,shutil
sys.path.insert(0,'tests/protocol');import isolation
root=Path('.').resolve();out=root/'target/quality-task-064/native-r2';out.mkdir(exist_ok=False)
sha=lambda b:hashlib.sha256(b).hexdigest()
sysroot=Path(subprocess.check_output(['rustc','--print','sysroot'],text=True).strip());python=Path(sys.executable).resolve();records=[]
old=root/'tasks/063/evidence/remediation-20261114/native'
archives={}
for item in json.loads((old/'LOCAL-ARCHIVES.json').read_text()):
 p=Path(item['local_path']);assert sha(p.read_bytes())==item['sha256'];archives[p.parent.name]=p
bindings=[]
for variant,previous in [('static','static'),('network','all-probes')]:
 provenance=json.loads((old/f'package-records/{previous}/PROVENANCE.json').read_text())
 inputs={k:v for k,v in provenance['source_inputs'].items() if k.startswith(('crates/','rules/')) or k in ['Cargo.toml','Cargo.lock','rust-toolchain.toml']}
 current=set(subprocess.check_output(['git','ls-files','--cached','--others','--exclude-standard','crates','rules','Cargo.toml','Cargo.lock','rust-toolchain.toml'],text=True).splitlines())
 full=json.loads((root/'tasks/063/evidence/remediation-20261114/validation-all-features/RUN.json').read_text())['source_bindings']
 for name in current-set(inputs):
  inputs[name]=full[name]
 assert current==set(inputs),(current-set(inputs),set(inputs)-current)
 assert all(sha((root/k).read_bytes())==v for k,v in inputs.items())
 bindings.append({'variant':variant,'reused_from':previous,'source_inputs':inputs,'binary_sha256':provenance['binary_sha256'],'source_inputs_identical':True,'prior_build_evidence':'tasks/063/evidence/remediation-20261114/native/COMMANDS.json','rust_suites':'tasks/063/evidence/remediation-20261114/validation-default-combined/RUN.json and validation-all-features/RUN.json'})
(out/'BINARY-REUSE.json').write_text(json.dumps(bindings,indent=2)+'\n')
with tempfile.TemporaryDirectory(prefix='pqc064-native-') as temp:
 scratch=Path(temp).resolve();env=isolation.environment(scratch,sysroot/'bin/rustc');env.update(CARGO_NET_OFFLINE='true',GIT_CONFIG_GLOBAL='/dev/null',GIT_CONFIG_NOSYSTEM='1',PYTHONDONTWRITEBYTECODE='1',GIT_OPTIONAL_LOCKS='0')
 for name in ['registry','git']:
  cache=Path.home()/'.cargo'/name
  if cache.exists():(scratch/'cargo'/name).symlink_to(cache,target_is_directory=True)
 gitpath=subprocess.check_output(['xcrun','--find','git'],text=True).strip();env['PATH']=str(Path(gitpath).parent)+':'+env['PATH']
 policy=isolation.profile(root,scratch,read_paths=[sysroot,python.parents[1],Path.home()/'.cargo/registry',Path.home()/'.cargo/git','/private/var/db/xcode_select_link'])
 (out/'POLICY.sb').write_text(policy)
 for binding in bindings:
  variant=binding['variant'];previous=binding['reused_from'];binary=scratch/f'{variant}-pqc-scan'
  with tarfile.open(archives[previous]) as tf:
   member,=[m for m in tf.getmembers() if m.name.endswith('/bin/pqc-scan')];assert member.isfile();data=tf.extractfile(member).read();assert sha(data)==binding['binary_sha256'];binary.write_bytes(data);binary.chmod(0o755)
  results=[]
  for attempt in ['first','second']:
   dest=scratch/f'{variant}-{attempt}';args=[str(python),'scripts/package-release.py','--binary',str(binary),'--variant',variant,'--out-dir',str(dest)]
   with (out/f'{variant}-{attempt}.log').open('w') as log:
    start=time.monotonic();r=isolation.run(args,cwd=root,env=env,policy=policy,timeout=180,stdout=log,stderr=subprocess.STDOUT)
   records.append({'variant':variant,'name':attempt,'command':args,'exit':r.returncode,'seconds':time.monotonic()-start});(out/'COMMANDS.json').write_text(json.dumps(records,indent=2)+'\n');print(variant,attempt,r.returncode,flush=True)
   if r.returncode:raise SystemExit(r.returncode)
   archive,=dest.glob('*.tar.gz');results.append(archive)
  assert results[0].read_bytes()==results[1].read_bytes()
  retained=out/variant;retained.mkdir();archive=retained/results[0].name;shutil.copyfile(results[0],archive);shutil.copyfile(Path(str(results[0])+'.sha256'),Path(str(archive)+'.sha256'))
  for label,archive_to_check in [('current',archive),('historical',archives[previous])]:
   args=[str(python),'tests/distribution/check-install.py',str(archive_to_check),'--out-dir',str(out/f'{variant}-{label}-install')]
   with (out/f'{variant}-{label}-install.log').open('w') as log:r=subprocess.run(args,cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=120)
   records.append({'variant':variant,'name':label+'-install','command':args,'exit':r.returncode,'archive_sha256':sha(archive_to_check.read_bytes()),'repeatability':label=='current'});(out/'COMMANDS.json').write_text(json.dumps(records,indent=2)+'\n');print(variant,label+'-install',r.returncode,flush=True)
   if r.returncode:raise SystemExit(r.returncode)
 # The removed combined distribution is also still a valid historical input.
 args=[str(python),'tests/distribution/check-install.py',str(archives['combined-probes']),'--out-dir',str(out/'combined-historical-install')]
 with (out/'combined-historical-install.log').open('w') as log:r=subprocess.run(args,cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=120)
 records.append({'name':'combined-historical-install','command':args,'exit':r.returncode});(out/'COMMANDS.json').write_text(json.dumps(records,indent=2)+'\n');print('combined-historical-install',r.returncode,flush=True)
 if r.returncode:raise SystemExit(r.returncode)
(out/'COMPLETION.json').write_text(json.dumps({'exit':0,'scratch_removed':True,'limits':'offline macOS sandbox source/cache read-only; scratch-only package writes; per-process CPU/process/file and wall limits; no aggregate cgroup claim; installed product commands individually sandboxed by check-install'},indent=2)+'\n')
