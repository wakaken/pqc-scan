from pathlib import Path
import sys,tempfile,subprocess,json,time,hashlib,os
sys.path.insert(0,'tests/protocol');import isolation
root=Path('.').resolve();out=root/'target/quality-task-063/native-r3';out.mkdir();sysroot=Path(subprocess.check_output(['rustc','--print','sysroot'],text=True).strip());python=Path(sys.executable).resolve();records=[]
with tempfile.TemporaryDirectory(prefix='pqc063-native-') as temp:
 scratch=Path(temp).resolve();env=isolation.environment(scratch,sysroot/'bin/rustc');env.update(CARGO_NET_OFFLINE='true',CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',GIT_CONFIG_GLOBAL='/dev/null',GIT_CONFIG_NOSYSTEM='1',PYTHONDONTWRITEBYTECODE='1')
 for name in ['registry','git']:
  cache=Path.home()/'.cargo'/name
  if cache.exists():(scratch/'cargo'/name).symlink_to(cache,target_is_directory=True)
 gitpath=subprocess.check_output(['xcrun','--find','git'],text=True).strip();env['PATH']=str(Path(gitpath).parent)+':'+env['PATH'];env['GIT_OPTIONAL_LOCKS']='0'
 clang=subprocess.check_output(['xcrun','--sdk','macosx','--find','clang'],text=True).strip();sdk=subprocess.check_output(['xcrun','--sdk','macosx','--show-sdk-path'],text=True).strip();ar=subprocess.check_output(['xcrun','--sdk','macosx','--find','ar'],text=True).strip();env.update(CC=clang,CXX=clang+'++',AR=ar,SDKROOT=sdk,CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER=clang,CLANG_MODULE_CACHE_PATH=str(scratch/'clang-cache'))
 policy=isolation.profile(root,scratch,read_paths=[sysroot,python.parents[1],Path.home()/'.cargo/registry',Path.home()/'.cargo/git','/private/var/db/xcode_select_link'])
 (out/'POLICY.sb').write_text(policy);(out/'ENVIRONMENT.json').write_text(json.dumps(env,indent=2)+'\n')
 for variant,flags in [('static',[]),('combined-probes',['--features','probe-tls,probe-ssh']),('all-probes',['--features','probe-tls,probe-ssh,probe-ssh-kex'])]:
  build=scratch/('build-'+variant);dest=scratch/('package-'+variant);install=out/(variant+'-install');binary=build/'release/pqc-scan'
  cmds=[('build',['cargo','build','--locked','--offline','--release','--package','pqc_scan_cli','--target-dir',str(build),*flags]),('package',[str(python),'scripts/package-release.py','--binary',str(binary),'--variant',variant,'--out-dir',str(dest)])]
  for name,args in cmds:
   with (out/(variant+'-'+name+'.log')).open('w') as log:
    start=time.monotonic();r=isolation.run(args,cwd=root,env=env,policy=policy,timeout=900,stdout=log,stderr=subprocess.STDOUT)
   records.append({'variant':variant,'name':name,'command':args,'exit':r.returncode,'seconds':time.monotonic()-start});(out/'COMMANDS.json').write_text(json.dumps(records,indent=2)+'\n');print(variant,name,'exit',r.returncode,flush=True)
   if r.returncode:raise SystemExit(r.returncode)
  archive,=dest.glob('*.tar.gz')
  # This trusted controller verifies/extracts to a fresh temporary prefix; each
  # installed product command is separately constrained offline by check-install.
  args=[str(python),'tests/distribution/check-install.py',str(archive),'--out-dir',str(install)]
  with (out/(variant+'-install.log')).open('w') as log:r=subprocess.run(args,cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=120)
  records.append({'variant':variant,'name':'install','command':args,'exit':r.returncode,'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'archive_sha256':hashlib.sha256(archive.read_bytes()).hexdigest()});(out/'COMMANDS.json').write_text(json.dumps(records,indent=2)+'\n');print(variant,'install exit',r.returncode,flush=True)
  if r.returncode:raise SystemExit(r.returncode)
  import shutil
  # Retain the tested local archive, never publish it. Build caches are disposable.
  target=out/variant;target.mkdir();shutil.copyfile(archive,target/archive.name)
(out/'COMPLETION.json').write_text(json.dumps({'exit':0,'scratch_removed':True,'limits':'macOS sandbox source/cache readonly, scratch-only writes for build/packaging, offline allowlisted environment and finite CPU/process/file/wall limits; no aggregate Linux cgroup memory/disk claim'},indent=2)+'\n')
