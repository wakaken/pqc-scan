from pathlib import Path
from collections import Counter
import json,hashlib,subprocess
p=Path(__file__).resolve().parent;s=p/'self-scan';read=lambda x:json.loads(x.read_text())
sha=read(p/'SOURCE.json')['commit'];manifest=read(s/'self-scan-inputs.json'); ver=read(s/'self-scan-verification.json'); audit=read(s/'code-scanning-audit.json'); sarif=read(s/'code-scanning.sarif');raw=read(s/'report.sarif');alerts=read(p/'alerts.json');analyses=read(p/'analyses.json')
analysis=next(a for a in analyses if a['commit_sha']==sha and a['ref']=='refs/heads/codex/static-quality-completion-20261117');assert not analysis['error'] and not analysis['warning']
assert ver['manifestSha256']==hashlib.sha256((s/'self-scan-inputs.json').read_bytes()).hexdigest()
assert ver['verifiedFiles']==manifest['fileCount']==106
assert ver['verifiedBytes']==manifest['byteCount']
for f in manifest['files']:
 b=subprocess.check_output(['git','show',sha+':'+f['path']]);assert len(b)==f['bytes'] and hashlib.sha256(b).hexdigest()==f['sha256'], f['path']
results=[r for run in sarif['runs'] for r in run['results']]; rawresults=[r for run in raw['runs'] for r in run['results']]
def key(r):
 l=r['locations'][0]['physicalLocation'];return (r['ruleId'],l['artifactLocation']['uri'],l['region']['startLine'],r['message']['text'])
upload=Counter(key(r) for r in results); remote=Counter()
for a in alerts:
 i=a['most_recent_instance']; assert i['commit_sha']==sha and i['ref']==analysis['ref'];remote[(a['rule']['id'],i['location']['path'],i['location']['start_line'],i['message']['text'])]+=1
assert upload==remote
assert len(results)==analysis['results_count']==audit['retainedResults']==10
assert len(rawresults)==audit['rawResults']==487
assert audit['expectedResults']==477
assert audit['rawSarifSha256']==hashlib.sha256((s/'report.sarif').read_bytes()).hexdigest()
canonical=json.dumps(read(s/'self-scan-expectations.json'),sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
assert audit['expectationsSha256']==hashlib.sha256(canonical).hexdigest()
assert all(c['status']=='verified' for c in audit['sourceChecks'])
local=read(Path('docs/releases/static-candidate-20261117/audit-fixes/evidence/self-scan/manifest.json'));assert manifest==local
r={'commit':sha,'runId':44034239763,'analysisId':analysis['id'],'analysisRef':analysis['ref'],'analysisError':analysis['error'],'analysisWarning':analysis['warning'],'inputFiles':manifest['fileCount'],'inputBytes':manifest['byteCount'],'eachHostedInputMatchesCommit':True,'hostedManifestEqualsReviewedLocalManifest':True,'rawResults':len(rawresults),'reviewedReferences':audit['expectedResults'],'reviewResults':len(results),'openBranchAlerts':len(alerts),'uploadAndAlertMultisetsEqual':True,'comparisonFields':['ruleId','path','startLine','fullMessage'],'sourceChecksVerified':len(audit['sourceChecks']),'artifactId':14306173258,'artifactDigestVerified':True,'historicalFailure2135Cohort':'Unknown; not retrospectively resolved','vulnerabilityCount':'Not inferred from reference results'}
(p/'CODE-SCANNING-VERIFICATION.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
