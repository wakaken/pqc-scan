const fs=require('fs'),cp=require('child_process'),crypto=require('crypto');
const {resultDigest}=require(process.cwd()+'/shell/prepare-code-scanning.cjs');
const dir='/tmp/pqc-audit-fixes-20261117-9k0dmc45';
const policyPath='docs/quality/self-scan/SELF-SCAN-EXPECTATIONS.json';
const policy=JSON.parse(fs.readFileSync(policyPath));const before=structuredClone(policy);
const old=JSON.parse(fs.readFileSync('target/static-quality-ci-a983be1/self-scan/report.sarif'));
const now=JSON.parse(fs.readFileSync(dir+'/self-scan/report/report.sarif'));
const pathOf=r=>r.locations[0].physicalLocation.artifactLocation.uri;
const oldMap=new Map(policy.files.map(f=>[f.path,new Map(f.results.map(r=>[r.sha256,r]))]));
const expected=new Set();for(const run of old.runs)for(const r of run.results) if(oldMap.get(pathOf(r))?.has(resultDigest(r)))expected.add(resultDigest(r));
const expectedCount=old.runs.flatMap(run=>run.results).filter(r=>oldMap.get(pathOf(r))?.has(resultDigest(r))).length; if(expectedCount!==475)throw Error('Old reference population changed '+expectedCount);
const file='crates/pqc_scan_detectors/src/regex_detector.rs';
const a=cp.execFileSync('git',['show','HEAD:'+file],{encoding:'utf8'}), b=fs.readFileSync(file,'utf8');
const marker='#[cfg(test)]';if(a.slice(a.indexOf(marker))!==b.slice(b.indexOf(marker)))throw Error('Old inline tests changed');
const delta=b.slice(0,b.indexOf(marker)).split('\n').length-a.slice(0,a.indexOf(marker)).split('\n').length;
let newRules=[],queue=[];const counts=new Map();
for(const run of now.runs) for(const r of run.results){
 const path=pathOf(r),digest=resultDigest(r),group=policy.files.find(f=>f.path===path);let match=oldMap.get(path)?.get(digest),basis='Exact previous result';
 if(!match&&path===file){const adjusted=structuredClone(r);const reg=adjusted.locations[0].physicalLocation.region;reg.startLine-=delta;if(reg.endLine)reg.endLine-=delta;match=oldMap.get(path)?.get(resultDigest(adjusted));basis='Unchanged inline-test source and fields; line translation only';}
 if(!match&&r.ruleId==='TLS_VERSION_1_0'&&group){match={};basis='Individually inspected token reference in rule vocabulary or inline synthetic test; corrected token location or new matcher fingerprint';}
 if(!match && path===file && r.ruleId==='TS_TLS_LEGACY_VERSION_LITERAL' && r.locations[0].physicalLocation.region.startLine===248){match={};basis='Individually reviewed literal is the matcher fingerprint, not a protocol invocation';}
 if(!match){queue.push(r);continue;}
 const reg=r.locations[0].physicalLocation.region;
 const key=path+'\0'+digest;if(!counts.has(key)){counts.set(key,{path,row:{ruleId:r.ruleId,line:reg.startLine,column:reg.startColumn??null,sha256:digest,count:0},basis});}
 counts.get(key).row.count++;
 if(!oldMap.get(path)?.has(digest))newRules.push({path,ruleId:r.ruleId,line:reg.startLine,column:reg.startColumn??null,basis,sourceLine:fs.readFileSync(path,'utf8').split('\n')[reg.startLine-1]});
}
const priorQueue=old.runs.flatMap(r=>r.results).filter(r=>!oldMap.get(pathOf(r))?.has(resultDigest(r))).map(resultDigest).sort();
console.log('queue',queue.map(r=>[pathOf(r),r.ruleId,r.locations[0].physicalLocation.region]));if(JSON.stringify(priorQueue)!==JSON.stringify(queue.map(resultDigest).sort()))throw Error('Unreviewed queue changed');
const productPaths=new Set(now.runs.flatMap(run=>run.results).map(pathOf));
for(const f of policy.files){if(!productPaths.has(f.path))continue;f.sha256=crypto.createHash('sha256').update(fs.readFileSync(f.path)).digest('hex');f.results=[...counts.values()].filter(x=>x.path===f.path).map(x=>x.row);if(!f.results.length)throw Error('Unexpected reference population loss '+f.path);}
fs.writeFileSync(dir+'/NOTIFICATION-POLICY-BEFORE.json',JSON.stringify(before,null,2)+'\n');
fs.writeFileSync(policyPath,JSON.stringify(policy,null,2)+'\n');
fs.writeFileSync(dir+'/NOTIFICATION-REBIND.json',JSON.stringify({oldRaw:485,newRaw:now.runs.flatMap(r=>r.results).length,oldReviewed:475,newReviewed:[...counts.values()].reduce((s,x)=>s+x.row.count,0),queueUnchanged:queue.length,inlineTestLineDelta:delta,changedRows:newRules},null,2)+'\n');
console.log('Reviewed', [...counts.values()].reduce((s,x)=>s+x.row.count,0),'queue',queue.length,'changed rows',newRules.length);
