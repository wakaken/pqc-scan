"""Frozen protocol, distinct comparable strata; never runs scanned project contents."""
import argparse,datetime,hashlib,json,os,pathlib,platform,shutil,subprocess,time
ROOT=pathlib.Path.cwd();BASE=ROOT/'tasks/076/evidence';OUT=ROOT/'tasks/083/evidence'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 ap=argparse.ArgumentParser();ap.add_argument('phase',choices=['baseline-comparable','final']);a=ap.parse_args()
 dest=OUT/('measure-'+a.phase)
 if dest.exists():raise SystemExit('refuse to replace observed phase')
 origin=ROOT/('target/v05-comparable-baseline-build/debug/pqc-scan' if a.phase=='baseline-comparable' else 'target/debug/pqc-scan');assert origin.is_file()
 binary=ROOT/'target/v05-measure-bin/pqc-scan';binary.parent.mkdir(exist_ok=True);shutil.copy2(origin,binary)
 dest.mkdir();source=ROOT/('target/v05-comparable-baseline-source' if a.phase=='baseline-comparable' else '.')
 tracked=subprocess.check_output(['git','ls-files'],text=True).splitlines()
 relevant=[p for p in tracked if p.startswith(('crates/','rules/','third_party/')) or p in ['Cargo.toml','Cargo.lock','rust-toolchain.toml']]
 if a.phase=='final':
  relevant=sorted(set(relevant)|set(json.loads((ROOT/'tasks/082/evidence/IMPLEMENTATION-SOURCE.json').read_text())['files']))
 source_hashes={p:sha(source/p)for p in sorted(relevant)if (source/p).is_file()}
 rules=binary.parent/'rules'
 if rules.exists():shutil.rmtree(rules)
 shutil.copytree(source/'rules',rules)
 result={'protocol':'tasks/076/MEASUREMENT-PROTOCOL.md','protocol_sha256':sha(ROOT/'tasks/076/MEASUREMENT-PROTOCOL.md'),'phase':a.phase,'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'source_hashes':source_hashes,'host':platform.platform(),'machine':platform.machine(),'rust':subprocess.check_output(['rustc','-vV'],text=True),'profile':'dev','features':'workspace defaults','CARGO_INCREMENTAL':'0','binary_sha256':sha(binary),'binary_origin':str(origin.relative_to(ROOT)),'compilation_excluded':True,'runs':[],'inputs':{}}
 for group,path in [('old-seven',BASE/'old-seven'),('new-source',BASE/'corpus'),('new-dependency',BASE/'dependency-corpus'),('saved',BASE/'saved.inventory.json')]:
  files=sorted(p for p in path.rglob('*')if p.is_file())if path.is_dir()else[path];result['inputs'][group]={'files':len(files),'bytes':sum(p.stat().st_size for p in files),'sha256':{str(p.relative_to(ROOT)):sha(p)for p in files}}
  for i in range(4):
   out=dest/group/str(i);out.mkdir(parents=True)
   if group=='saved':argv=[str(binary),'assess',str(path.relative_to(ROOT)),'--input-format','inventory-v1','--as-of','2026-11-24T00:00:00Z','--out-dir',str(out.relative_to(ROOT))]
   else:argv=[str(binary),'scan',str(path.relative_to(ROOT)),'--threads','1','--format','all','--diagnostics','--inventory-json','--repository-scope','v05/frozen','--out-dir',str(out.relative_to(ROOT)),'--rules-dir',str(rules)]
   started=time.monotonic();instant=datetime.datetime.now(datetime.timezone.utc).isoformat()
   with (out/'stdout.log').open('w')as stdout,(out/'resource.log').open('w')as stderr:p=subprocess.run(['/usr/bin/time','-l',*argv],stdout=stdout,stderr=stderr,cwd=ROOT,timeout=120)
   row={'group':group,'sample':i,'warmup':i==0,'started_utc':instant,'argv':argv,'exit':p.returncode,'wall_seconds':time.monotonic()-started,'resource_log':str((out/'resource.log').relative_to(ROOT)),'outputs':{str(f.relative_to(out)):{'bytes':f.stat().st_size,'sha256':sha(f)}for f in sorted(out.rglob('*'))if f.is_file()and f.suffix!='.log'}};result['runs'].append(row);(dest/'OBSERVATIONS.json').write_text(json.dumps(result,indent=2)+'\n');print(group,i,p.returncode,flush=True)
   if p.returncode:raise SystemExit('failed observation retained')
if __name__=='__main__':main()
