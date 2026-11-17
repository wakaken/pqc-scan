from pathlib import Path
import subprocess,json,time,hashlib
p=Path('/tmp/pqc-audit-fixes-20261117-9k0dmc45');old=json.loads(Path('/private/tmp/pqc-quality-completion-20261117-ufeh_mbz/ACCURACY-COMMANDS.json').read_text());records=[]
for i,row in enumerate(old):
 cmd=[v.replace('/private/tmp/pqc-quality-completion-20261117-ufeh_mbz/pqc-scan-static',str(p/'pqc-scan-static')).replace('target/static-quality-completion-20261117','target/audit-fixes-20261117') for v in row['argv']]
 log=p/('accuracy-'+str(i)+'.log');start=time.monotonic()
 with log.open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,timeout=300)
 records.append(dict(argv=cmd,exit=r.returncode,seconds=time.monotonic()-start,log=str(log),binarySha256=hashlib.sha256((p/'pqc-scan-static').read_bytes()).hexdigest()));(p/'ACCURACY-COMMANDS.json').write_text(json.dumps(records,indent=2)+'\n');print(i,r.returncode,flush=True)
 if r.returncode:raise SystemExit(r.returncode)
