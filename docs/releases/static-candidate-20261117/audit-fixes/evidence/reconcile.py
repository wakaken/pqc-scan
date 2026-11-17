from pathlib import Path
import json,hashlib
root=Path('.');out=root/'docs/releases/static-candidate-20261117/audit-fixes';oldpath=root/'tasks/054/evidence/f054-fix-20261117/REQUIREMENT-ASSESSMENT.json';old=json.loads(oldpath.read_text())
phase={
'Q1':('Frozen corpus/truth, evaluator and labels unchanged. Fresh canonical runs and real default-detector boundary/control tests exercise the corrected matcher; exposed repair cases remain tuning, not new holdout.','COMPATIBILITY.json'),
'Q2':('Fresh default/all-feature inspection and budget controls plus complete 106-file product-input verification. Unknown broader population is not replaced with zero.','self-scan/verification.json'),
'Q3':('No risk/schema/scoring changes. Typed facts and prior exact-package fixes remain tested; TLS names remain references. Current real exports preserve corrected token positions and explicit legacy export losses.','exports-final'),
'Q4':('Final default/all-feature tests and original confined generic suite exercise limits/failures/privacy/read/network contracts. New ordinary coverage and retained auxiliary failure are distinct; prior bounded fuzz/mutation/formal studies retain their original scope.','suite-verified/HOST-RESULTS.json'),
'Q5':('Measurement/reducer/resource tooling, nominated performance corpus and thresholds unchanged. Reuse bounded original and audit performance method evidence; canonical repeated semantics and explicit TLS-only field differences are fresh. No claim of new whole-product throughput/cold/multi-host performance.','COMPATIBILITY.json'),
'Q6':('Process, support and authorization boundaries preserved. Fresh combined reviews, current hosted run, final-source package/install and independent static binary builds must supplement historical bounded operational studies.','NATIVE-COMMANDS.json')}
special={
'Q1-02':'All 123 IDs remain classified. The subsequent requested 98 rows have individual positive/negative nominal controls and stated oracle limits; no historical 93/5/25 labels are rewritten or independent accuracy claimed.',
'Q1-06':'The audit TLS counterexample is explicitly corrected, with LF/CRLF/Unicode/negative/custom/comment/cap and real export controls. Fixed corpus results do not imply absence of other counterexamples.',
'Q3-12':'F-02 regression now has 3 token locations, native byte ranges, distinct IDs, EN/JA equality, SARIF/HTML/Markdown position checks, and the intentional legacy CBOM same-line collapse. No output schema or renderer changes.',
'Q4-03':'Original bounded nine-domain study remains historical. The independent audit additional campaign remains Partial at its supervisor deadline; no product per-input hang was established and no campaign completion is invented.',
'Q4-13':'The 136 static spans retain individual assessment and 9 unobserved invariant directions, not covered. Fresh ordinary export is distinguished from failed auxiliary LLVM dump; 7 mismatch diagnostics identify the same locked external memchr definition. Raw src-path includes inline tests and is not production-only.',
'Q6-01':'F-01 generic suite now reaches all test executables, strict all-feature Clippy, preview and SSH KEX with original nonroot/readonly/network/kernel limits and cleanup. Hosted protocols remains explicitly skipped; it is not counted as executed.',
'Q6-02':'Use the same implementation/test reviewer identities for this combined follow-up and confirm final-source evidence; previous approvals alone cannot approve new changes.',
'Q6-03':'Cargo.lock/features/toolchain unchanged. Reuse dated 2026-11-17 RustSec snapshot/tool and dependency results only for their recorded DB/source scope; do not imply continuous monitoring.',
'Q6-04':'Static candidate dependency/notices mapping is checked freshly (139, no missing/review flags). Network four-condition notice/publication hold remains separate.',
'Q6-06':'Reuse dated observed settings and recommendations; no branch protection or permission setting changed or newly certified.',
'Q6-10':'The approved TLS-only count/location/evidence/ID differences are retained explicitly; full other finding fields match. Custom patterns/scopes/empty matches use existing generic semantics.',
'Q6-11':'Only the maintainer-selected static macOS arm64 support scope is evaluated for release. Linux isolated tests are supporting evidence, not a new official platform.',
'Q6-12':'Actual corrected Japanese browser report displays all 3 positions and highlights line2, with the dedicated loopback synthetic-report directory. Existing keyboard/narrow/offline checks remain scoped to unchanged renderer; paired guides updated consistently.',
'Q6-13':'Fresh same-source static A/B builds use independent confined source/home/target roots and readonly dependency cache. Compare actual raw binaries and Mach-O metadata, not archives; same physical host limitation retained.',
'Q6-14':'Bind final candidate binary/rules/notices/source to package checksums and provenance; recipient tamper/missing/mixed negative controls remain exercised. Internal integrity does not establish a production signing identity.',
'Q6-15':'Preserve the existing isolated ephemeral signing/tamper proof and unchanged implementation; actual release identity, tag, signing and publication remain separate authorization. No production credential is used.',
'Q6-16':'Retain every original ID/quote/owner and hash-linked historical evidence; add final-source manifest and hosted receipts. Historical failed/Partial/Unknown measurements stay unchanged.',
'Q6-17':'The old audit Q6-17 Unknown is not erased by test counts. Resolve both findings, verify this exact 73-row closeout and required reviews/hosted/package evidence before acceptance. Product acceptance/publication remain independent.',
'Q6-18':'This machine-readable matrix and human report identify source/method/environment/evidence/limits/prior comparison/priority and acceptance separately. No aggregate quality percentage is invented.'}
rows=[]
for i,r in enumerate(old['requirements']):
 group,ev=phase[r['id'].split('-')[0]]
 refs=[]
 for f in r.get('currentEvidence',[]):
  p=root/f
  refs.append({'path':f,'sha256':hashlib.sha256(p.read_bytes()).hexdigest() if p.is_file() else None})
 rows.append({'id':r['id'],'ownerTask':r['ownerTask'],'acceptanceQuote':r['acceptanceQuote'],'historicalRow':{'path':str(oldpath),'pointer':'/requirements/'+str(i)},'historicalQualityLabel':r['result'],'historicalCriterionResult':r['currentFulfilment']['result'],'historicalBasis':r['currentFulfilment']['decisionBasis'],'historicalEvidence':refs,'currentApplicability':special.get(r['id'],group),'currentEvidence':[ev],'localAssessment':'Prepared; final evidence/review checks pending','currentAcceptance':'Pending','remainingRequiredWork':'Final review/source binding and candidate hosted/package closeout; preserve row-specific historical scope and limits.'})
assert len(rows)==73 and len(set(r['id'] for r in rows))==73
j={'baseline':'a983be17297d3d87606d0b1531784a9a78c510c0','scope':'Original 73 v0.3 quality/evaluation requirements; static candidate macOS arm64 follow-up, not universal product quality.','previousMatrixSha256':hashlib.sha256(oldpath.read_bytes()).hexdigest(),'independentAuditSource':'ee6661ab016c6e20796f674c50f348f87d950ed4','independentAuditCounts':{'Fulfilled':72,'Unknown':1},'originalCriteriaPreserved':True,'truthAndNumericGatesChanged':False,'publicationAuthorized':False,'productAccepted':False,'candidateDecision':'In progress','requirements':rows}
(out/'REQUIREMENTS.json').write_text(json.dumps(j,ensure_ascii=False,indent=2)+'\n')
lines=['# Current candidate requirement reconciliation','','Every acceptance quote/ID/owner is retained exactly. Prior labels and criterion judgments are historical. The current column is a candidate closeout, not a product accuracy score. Network support and publication remain held.','','| ID | Current acceptance | Applicability / residual scope |','| --- | --- | --- |']
for r in rows:lines.append('| '+r['id']+' | '+r['currentAcceptance']+' | '+r['currentApplicability'].replace('|','\\|')+' |')
(out/'REQUIREMENTS.md').write_text('\n'.join(lines)+'\n')
print('73 original rows/quotes/owners retained')
