from pathlib import Path
from urllib.request import urlopen
import json,hashlib,shutil
R=Path('/Users/waka/.codex/worktrees/pqc-language-research/pqc-scan');E=R/'tasks/087/evidence';C=Path('/Users/waka/.cache/pqc-scan-oss-validation-087');V=Path('/Users/waka/.cache/pqc-scan-oss-validation-085/report-view-20261125');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();receipt=json.loads((E/'OSS-RESCAN-09.json').read_text());controls=json.loads((E/'EXPORT-PRIVACY-CONTROLS-09.json').read_text());assert len(receipt['projects'])==4 and controls['allFormatsContainNoSyntheticSecrets'];rows=[]
for row in receipt['projects']:
 slug=row['slug'];files={}
 for name in ['report.html','report.rules.html']:
  src=Path(row['outDir'])/name;assert sha(src)==row['outputs'][name];dst=V/slug/name;shutil.copy2(src,dst);assert sha(dst)==sha(src);url=f'http://127.0.0.1:57385/{slug}/{name}?lang=ja';data=urlopen(url,timeout=5).read();assert hashlib.sha256(data).hexdigest()==sha(src);files[name]={'source':str(src),'display':str(dst),'sha256':sha(dst),'url':url}
 rows.append({'slug':slug,'files':files})
(E/'DISPLAYED-REPORTS-09.json').write_text(json.dumps({'sourceManifestSha256':receipt['sourceManifestSha256'],'operationSha256':sha(Path(__file__)),'note':'Only dedicated view copies refreshed; original085/086 artifacts untouched. Loopback-only server.','projects':rows},indent=2)+'\n');print('Four view copies refreshed; all served bytes verified')
