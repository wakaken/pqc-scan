'use strict';
const test=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const {prepareCodeScanning}=require('./prepare-code-scanning.cjs');
const repositoryRoot=path.resolve(__dirname,'..');
const read=p=>JSON.parse(fs.readFileSync(path.join(repositoryRoot,p)));
const ledger=read('docs/quality/V02-SELF-SCAN-REVIEW.json');
const {reviewedResultTestSource}=require('./reviewed-result-test-source.cjs');
const fixture=reviewedResultTestSource(read('docs/quality/SELF-SCAN-EXPECTATIONS.json'));
const root=fixture.root;
const policy=()=>structuredClone(fixture.policy);
test.after(fixture.cleanup);
const input=()=>({version:'2.1.0',runs:[{tool:{driver:{name:'pqc-scan'}},results:structuredClone(ledger.results.map(x=>x.result))}]});
test('historical v0.2 finite result controls keep dependency observations on synthetic bound sources',()=>{
 const raw=input(),before=JSON.stringify(raw),out=prepareCodeScanning(raw,policy(),root);
 assert.equal(JSON.stringify(raw),before);
 assert.equal(out.audit.expectedResults,ledger.counts.expected);
 assert.equal(out.audit.retainedResults,ledger.counts.retained);
 assert.deepEqual(out.sarif.runs[0].results,ledger.results.filter(x=>x.disposition==='retained_inventory').map(x=>x.result));
 assert.ok(out.audit.retainedResults>0);
 assert.equal(out.audit.rawResults,ledger.counts.raw);
});
test('v0.2 review retains new results and excess identical observations',()=>{
 const row=ledger.results.find(x=>x.disposition==='reviewed_expected'&&x.classification==='inline_test');
 assert.ok(row);
 const raw=input(),added=structuredClone(row.result);added.message.text+=' Unreviewed observation';
 raw.runs[0].results.push(added);
 assert.ok(prepareCodeScanning(raw,policy(),root).sarif.runs[0].results.some(x=>JSON.stringify(x)===JSON.stringify(added)));
 const duplicate=input();duplicate.runs[0].results.push(structuredClone(row.result));
 assert.equal(prepareCodeScanning(duplicate,policy(),root).sarif.runs[0].results.filter(x=>JSON.stringify(x)===JSON.stringify(row.result)).length,2);
});
test('v0.2 changed source binding retains all observations from that source',()=>{
 const row=ledger.results.find(x=>x.disposition==='reviewed_expected'&&x.classification==='inline_test');
 const p=policy(),entry=p.files.find(x=>x.path===row.path);assert.ok(entry);entry.sha256='0'.repeat(64);
 const out=prepareCodeScanning(input(),p,root);
 assert.deepEqual(out.sarif.runs[0].results.filter(x=>x.locations[0].physicalLocation.artifactLocation.uri===row.path),ledger.results.filter(x=>x.path===row.path).map(x=>x.result));
});

test('historical v0.2 evidence never authorizes changed current sources',()=>{
 const currentPolicy=read('docs/quality/SELF-SCAN-EXPECTATIONS.json');
 const actual=prepareCodeScanning(input(),currentPolicy,repositoryRoot);
 for(const [p,h] of Object.entries(ledger.sourceBindings)) {
  const bytes=fs.readFileSync(path.join(repositoryRoot,p));
  if(crypto.createHash('sha256').update(bytes).digest('hex')===h) continue;
  const policyFile=currentPolicy.files.find(f=>f.path===p);
  if(!policyFile || policyFile.sha256===crypto.createHash('sha256').update(bytes).digest('hex')) continue;
  const fromFile=r=>r.locations[0].physicalLocation.artifactLocation.uri===p;
  assert.deepEqual(actual.sarif.runs[0].results.filter(fromFile),input().runs[0].results.filter(fromFile));
 }
});
