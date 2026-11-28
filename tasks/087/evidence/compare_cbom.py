from pathlib import Path
import json,hashlib,gzip
R=Path('/Users/waka/.codex/worktrees/pqc-language-research/pqc-scan');E=R/'tasks/087/evidence';B=R/'tasks/085/evidence';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();read=lambda p:json.loads(p.read_text());receipt=read(E/'OSS-RESCAN-09.json');projects=read(B/'FINAL-COHORT-SOURCE06.json')['projects'];rows=[]
for row in receipt['projects']:
 cohort=next(p for p in projects if p['repository']==row['repository']);before=read(Path(cohort['profiles']['default'][0]));art=before['artifacts']['report.json'];p=Path(art['compressedPath']);assert sha(p)==art['compressedSha256'];old=json.loads(gzip.decompress(p.read_bytes()));new=read(Path(row['outDir'])/'report.json');a,b=old['cbom'],new['cbom'];assert len(a)==len(b);diffs=[]
 for i,(x,y)in enumerate(zip(a,b)):
  keys=set(x)|set(y);changed={k:{'before':x.get(k),'after':y.get(k)}for k in keys if x.get(k)!=y.get(k)}
  if changed:diffs.append({'index':i,'location':y.get('location'),'changed':changed})
 rows.append({'repository':row['repository'],'rowsBefore':len(a),'rowsAfter':len(b),'allRowsEqual':a==b,'differences':diffs})
(E/'CBOM-COMPATIBILITY-09.json').write_text(json.dumps({'sourceManifestSha256':receipt['sourceManifestSha256'],'operationSha256':sha(Path(__file__)),'scope':'Exhaustive row comparison for four displayed pinned scopes; differences require main inspection. Recovery of prior public lexical values is not new typed crypto evidence.','projects':rows},indent=2)+'\n');print(json.dumps(rows,indent=2))
