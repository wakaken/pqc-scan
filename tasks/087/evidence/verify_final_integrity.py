from pathlib import Path
from urllib.request import urlopen
import json,hashlib,subprocess,datetime
R=Path('/Users/waka/.codex/worktrees/pqc-language-research/pqc-scan');E=R/'tasks/087/evidence';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();read=lambda p:json.loads(p.read_text());M=E/'IMPLEMENTATION-SOURCE-09.json';m=read(M);source=sha(M)
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()==m['head']
for group in ['files','runtimeSourceBindings']:
 for n,h in m[group].items():assert sha(R/n)==h,n
checks=read(E/'CHECKS-09.json');assert checks['sourceManifestSha256']==source and len(checks['checks'])==9
for c in checks['checks']:assert c['exitCode']==0 and sha(R/c['log'])==c['logSha256'],c['name']
oss=read(E/'OSS-RESCAN-09.json');assert oss['sourceManifestSha256']==source and len(oss['projects'])==4
binary=Path('/Users/waka/.cache/pqc-scan-oss-validation-087/pqc-scan-source09');assert sha(binary)==oss['binarySha256'];assert sha(R/'target/release/pqc-scan')==sha(binary)
for row in oss['projects']:
 assert row['coverageEquality'] and row['scopeEquality'] and row['findingIdRuleLocationSeverityRiskEquality']
 for name,h in row['outputs'].items():assert sha(Path(row['outDir'])/name)==h,name
control=read(E/'EXPORT-PRIVACY-CONTROLS-09.json');assert control['sourceManifestSha256']==source and control['binarySha256']==sha(binary)
assert sha(E/'export_privacy_controls.py')==control['operationSha256']
for name,h in control['outputs'].items():
 p=Path('/Users/waka/.cache/pqc-scan-oss-validation-087/export-control-output')/name;assert sha(p)==h;assert b'SYNTH087_' not in p.read_bytes()
for name,h in control['inputHashes'].items():assert sha(Path('/Users/waka/.cache/pqc-scan-oss-validation-087/export-control-input')/name)==h
views=read(E/'DISPLAYED-REPORTS-09.json');assert views['sourceManifestSha256']==source
for row in views['projects']:
 for f in row['files'].values():assert sha(Path(f['source']))==sha(Path(f['display']))==f['sha256']==hashlib.sha256(urlopen(f['url'],timeout=5).read()).hexdigest()
browser=read(E/'BROWSER-09.json');assert browser['sourceManifestSha256']==source and browser['keyboard']=={'closed':False,'opened':True}
for n,h in browser['artifactHashes'].items():assert sha(E/n)==h,n
for v in browser['layouts'].values():assert v['documentWidth']==v['clientWidth']
history=R/'tasks/086';prior=read(history/'evidence/FINAL-INTEGRITY-05.json')
assert sha(history/'evidence/IMPLEMENTATION-SOURCE-05.json')==prior['sourceManifestSha256']
for n,h in prior['records'].items():assert sha(history/n)==h,n
assert sha(Path('/Users/waka/.cache/pqc-scan-oss-validation-086/pqc-scan-source05'))=='4bde10003df04732b889ea73838501f1097dcadb7dd4c5e75f990ad4d273e0d3'
p=subprocess.run(['git','diff','--check'],cwd=R,capture_output=True,text=True);assert p.returncode==0,p.stdout+p.stderr
out=dict(verifiedAtUtc=datetime.datetime.now(datetime.timezone.utc).isoformat(),sourceManifestSha256=source,head=m['head'],ownedFilesVerified=len(m['files']),runtimeBindingsVerified=len(m['runtimeSourceBindings']),all9ChecksExitZero=True,allLogHashesVerified=True,fourRealReportOutputAndDisplayedHttpHashesVerified=True,syntheticInputOutputAndOperationHashesVerified=True,browserArtifactHashesVerified=True,prior086TaskRecordsManifestBinaryPreserved=True,gitDiffCheckExitCode=p.returncode,noNewCommitOrPush=True)
(E/'ARTIFACT-INTEGRITY-09.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
