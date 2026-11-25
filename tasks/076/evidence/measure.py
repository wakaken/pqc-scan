"""Source-bound baseline/final observer; never executes scanned project content."""
import argparse, hashlib, json, pathlib, platform, subprocess, time
ROOT = pathlib.Path(__file__).resolve().parents[3]
BASE = pathlib.Path('tasks/076/evidence')
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def main():
    ap=argparse.ArgumentParser(); ap.add_argument('phase',choices=['baseline','baseline-frozen','baseline-frozen-v2','baseline-adopted','baseline-review1','final']);args=ap.parse_args()
    dest=BASE/('measure-'+args.phase)
    if dest.exists(): raise SystemExit('Refuse to replace an existing observation phase')
    dest.mkdir()
    binary=pathlib.Path('target/debug/pqc-scan')
    result={'phase':args.phase,'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'host':platform.platform(),'machine':platform.machine(),'rust':subprocess.check_output(['rustc','-vV'],text=True),'binary_sha256':sha(binary),'profile':'dev','features':'workspace defaults','runs':[],'inputs':{}}
    for group,path in [('old-seven',BASE/'old-seven'),('new-source',BASE/'corpus'),('new-dependency',BASE/'dependency-corpus'),('saved',BASE/'saved.inventory.json')]:
        files=sorted(path.rglob('*')) if path.is_dir() else [path];files=[p for p in files if p.is_file()]
        result['inputs'][group]={'files':len(files),'bytes':sum(p.stat().st_size for p in files),'sha256':{str(p):sha(p) for p in files}}
        for i in range(4):
            out=dest/group/str(i);out.mkdir(parents=True)
            if group=='saved':argv=[str(binary),'assess',str(path),'--input-format','inventory-v1','--as-of','2026-11-24T00:00:00Z','--out-dir',str(out)]
            else:argv=[str(binary),'scan',str(path),'--threads','1','--format','all','--diagnostics','--inventory-json','--repository-scope','v05/frozen','--out-dir',str(out),'--rules-dir','rules']
            started=time.monotonic()
            with (out/'stdout.log').open('w') as stdout, (out/'resource.log').open('w') as stderr:
                p=subprocess.run(['/usr/bin/time','-l',*argv],stdout=stdout,stderr=stderr,cwd=ROOT,timeout=120)
            row={'group':group,'sample':i,'warmup':i==0,'argv':argv,'exit':p.returncode,'wall_seconds':time.monotonic()-started,'resource_log':str(out/'resource.log'),'outputs':{str(f.relative_to(out)):{'bytes':f.stat().st_size,'sha256':sha(f)} for f in sorted(out.rglob('*')) if f.is_file() and f.suffix!='.log'}}
            result['runs'].append(row);print(group,i,p.returncode,flush=True)
            (dest/'OBSERVATIONS.json').write_text(json.dumps(result,indent=2)+'\n')
            if p.returncode:raise SystemExit(f'Observation failed; retained {group}/{i}')
if __name__=='__main__': main()
