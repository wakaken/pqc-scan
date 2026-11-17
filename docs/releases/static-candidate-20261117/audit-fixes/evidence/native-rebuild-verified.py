from pathlib import Path
import json,sys,tempfile,subprocess,hashlib,shutil,datetime,time,os
sys.path.insert(0,'tests/protocol');import isolation,container_runner
root=Path('.').resolve();out=Path('/tmp/pqc-audit-fixes-20261117-9k0dmc45/native-rebuild-verified');out.mkdir()
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
sysroot=Path(subprocess.check_output(['rustc','--print','sysroot'],text=True).strip());python=Path(sys.executable).resolve()
compiler={str(p):sha(p) for p in [sysroot/'bin/rustc',sysroot/'bin/cargo',python]}
clang=subprocess.check_output(['xcrun','--sdk','macosx','--find','clang'],text=True).strip();ar=subprocess.check_output(['xcrun','--find','ar'],text=True).strip();sdk=subprocess.check_output(['xcrun','--sdk','macosx','--show-sdk-path'],text=True).strip()
epoch=subprocess.check_output(['git','show','-s','--format=%ct','HEAD'],text=True).strip();records=[];binaries={}
with tempfile.TemporaryDirectory(prefix='pqc060-rebuild-') as tmp:
 base=Path(tmp).resolve();a=base/'A';b=base/'B';a.mkdir();b.mkdir();bindings=container_runner.snapshot(root,a/'source');shutil.copytree(a/'source',b/'source')
 assert all(sha(b/'source'/f)==h for f,h in bindings.items())
 (out/'SNAPSHOT.json').write_text(json.dumps(bindings,indent=2)+'\n')
 selected={f:h for f,h in bindings.items() if f.startswith(('crates/','rules/')) or f in ['Cargo.toml','Cargo.lock','rust-toolchain.toml']};(out/'SOURCE.json').write_text(json.dumps(selected,indent=2)+'\n')
 rustflags=' '.join('--remap-path-prefix='+str(p)+'=/pqc-build' for p in [a,b]);cflags=' '.join('-ffile-prefix-map='+str(p)+'=/pqc-build -fdebug-prefix-map='+str(p)+'=/pqc-build' for p in [a,b])
 for name,area in [('A',a),('B',b)]:
  scratch=area/'work';scratch.mkdir();env=isolation.environment(scratch,sysroot/'bin/rustc');env.update(CARGO_NET_OFFLINE='true',CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',CARGO_PROFILE_RELEASE_DEBUG='0',RUSTFLAGS=rustflags,CFLAGS=cflags,CXXFLAGS=cflags,CC=clang,CXX=clang+'++',AR=ar,SDKROOT=sdk,CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER=clang,SOURCE_DATE_EPOCH=epoch,CLANG_MODULE_CACHE_PATH=str(scratch/'clang-cache'),GIT_CONFIG_GLOBAL='/dev/null',GIT_CONFIG_NOSYSTEM='1')
  for cache in ['registry','git']:
   src=Path.home()/'.cargo'/cache
   if src.exists():(scratch/'cargo'/cache).symlink_to(src,target_is_directory=True)
  policy=isolation.profile(area/'source',scratch,read_paths=[sysroot,python.parents[1],Path.home()/'.cargo/registry',Path.home()/'.cargo/git','/private/var/db/xcode_select_link'])
  (out/(name+'-POLICY.sb')).write_text(policy);(out/(name+'-ENVIRONMENT.json')).write_text(json.dumps(env,indent=2)+'\n')
  for variant,features in [('static',[])]:
   target=scratch/('target-'+variant);assert not target.exists();env['CARGO_TARGET_DIR']=str(target);cmd=['cargo','build','--release','--locked','--offline','-p','pqc_scan_cli',*features];at=datetime.datetime.now(datetime.timezone.utc).isoformat();start=time.monotonic()
   with (out/(name+'-'+variant+'.log')).open('w') as log:r=isolation.run(cmd,cwd=area/'source',env=env,policy=policy,timeout=1200,stdout=log,stderr=subprocess.STDOUT)
   record={'environment':name,'variant':variant,'command':cmd,'startedAt':at,'seconds':time.monotonic()-start,'exitCode':r.returncode,'targetInitiallyAbsent':True,'target':str(target)};records.append(record);(out/'COMMANDS.json').write_text(json.dumps(records,indent=2)+'\n');print(name,variant,'exit',r.returncode,flush=True);assert r.returncode==0
   binary=target/'release/pqc-scan';dest=out/(name+'-'+variant+'-pqc-scan');shutil.copyfile(binary,dest);dest.chmod(0o755)
   version=isolation.run([str(binary),'--version'],cwd=scratch,env=env,policy=policy,timeout=30,capture_output=True,text=True);assert version.returncode==0
   meta=subprocess.run(['otool','-l',str(dest)],capture_output=True,text=True);assert meta.returncode==0;(out/(name+'-'+variant+'-MACHO.txt')).write_text(meta.stdout)
   binaries[name,variant]={'path':str(dest),'sha256':sha(binary),'size':binary.stat().st_size,'version':version.stdout.strip()}
  assert all(sha(area/'source'/f)==h for f,h in bindings.items())
 assert all(sha(root/f)==h for f,h in selected.items());assert all(sha(Path(p))==h for p,h in compiler.items())
 rows=[]
 for variant in ['static']:
  left=binaries['A',variant];right=binaries['B',variant];x=Path(left['path']).read_bytes();y=Path(right['path']).read_bytes();equal=x==y;diff=[i for i,(u,v) in enumerate(zip(x,y)) if u!=v][:100]
  rows.append({'variant':variant,'A':left,'B':right,'rawBytesEqual':equal,'firstDifferenceOffsets':diff,'lengthDifference':len(x)-len(y),'interpretation':'Rawindependentlybuilt binaries match under recordednormalized conditions; no archive comparison used' if equal else 'Rawbinaries differ; inspect retainedMachO/offsets and explain differences before evaluationcompletion'})
 result={'status':'Measured; difference analysis and independentreviews pending','head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'independence':{'samePhysicalHost':True,'independentOSConfinedSourceHomeTmpCargoTargetRoots':True,'noSharedBuildArtifacts':True,'sharedReadonlyDependencyInputs':True,'sourceSnapshotsByteEqual':True,'scope':'Two clean local buildenvironments, not two hosts or every environment'},'conditions':{'target':'aarch64-apple-darwin','rustflags':rustflags,'cflags':cflags,'sourceDateEpoch':epoch,'compilerHashes':compiler,'sdk':sdk,'logicalRoot':'/pqc-build','source':'SOURCE.json'},'rows':rows}
 (out/'COMPARISON.json').write_text(json.dumps(result,indent=2)+'\n');print('rawequal',[(r['variant'],r['rawBytesEqual']) for r in rows],flush=True)
(out/'COMPLETION.json').write_text(json.dumps({'exitCode':0,'allScratchContextsRemoved':True,'scope':'Native readonlysource/cache,scratchonlywrites,noexternalnetwork,finiteperprocess/percommand limits; noaggregateLinuxcgroup guarantee.'},indent=2)+'\n')
