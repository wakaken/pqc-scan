'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { createHash } = require('node:crypto');
const { prepareCodeScanning, renderSelfScanSummary, resultDigest } = require('./prepare-code-scanning.cjs');
const repositoryRoot = path.resolve(__dirname, '..');
const read = name => JSON.parse(fs.readFileSync(path.join(repositoryRoot, name), 'utf8'));
const ledger = read('docs/quality/self-scan/notification-review-20261116/REVIEWED-RESULTS.json');
// Historical full-repository results are tested on explicit synthetic sources.
// Current product bindings are verified separately; old line numbers cannot
// authorize omissions from the changed current source.
const { reviewedResultTestSource } = require('./reviewed-result-test-source.cjs');
const fixture = reviewedResultTestSource({ version: 1, files: ledger.files.map(file => ({
  path: file.path, sha256: file.sha256, reason: file.reason,
  results: [...new Map(file.results.map(result => {
    const region = result.locations[0].physicalLocation.region;
    const hash = resultDigest(result);
    return [hash, { ruleId: result.ruleId, line: region.startLine,
      column: region.startColumn ?? null, sha256: hash,
      count: file.results.filter(item => resultDigest(item) === hash).length }];
  })).values()],
})) });
const root = fixture.root;
const policy = () => structuredClone(fixture.policy);
test.after(fixture.cleanup);

test('current finite policy binds every actual source after reviewed source changes', () => {
  for (const file of read('docs/quality/self-scan/SELF-SCAN-EXPECTATIONS.json').files) {
    assert.equal(createHash('sha256').update(fs.readFileSync(path.join(repositoryRoot, file.path))).digest('hex'), file.sha256, file.path);
  }
});
const input = () => ({ version: '2.1.0', runs: [{ tool: { driver: { name: 'pqc-scan' } },
  results: structuredClone([...ledger.files.flatMap(file => file.results), ...ledger.remaining]) }] });
const filename = result => result.locations[0].physicalLocation.artifactLocation.uri;

test('historical finite review preserves dependency and unresolved observations on bound synthetic sources', () => {
  assert.deepEqual(ledger.counts, { raw: 5255, reviewed: 5240, requireReview: 15 });
  const current = policy();
  for (const file of ledger.files) {
    const binding = current.files.find(item => item.path === file.path);
    assert(binding, file.path);
    assert.equal(createHash('sha256').update(fs.readFileSync(path.join(root, file.path))).digest('hex'), binding.sha256, file.path);
    assert(file.reason.length > 20);
    assert(file.results.every(result => filename(result) === file.path));
  }
  const raw = input(), before = structuredClone(raw), prepared = prepareCodeScanning(raw, policy(), root);
  assert.deepEqual(raw, before);
  assert.deepEqual(prepared.sarif.runs[0].results, ledger.remaining);
  assert.deepEqual([prepared.audit.rawResults, prepared.audit.expectedResults, prepared.audit.retainedResults], [5255, 5240, 15]);
  assert(prepared.audit.sourceChecks.every(item => item.status === 'verified'));
  assert.equal(ledger.remaining.filter(result => !result.locations[0].physicalLocation.region).length, 4);
  assert.equal(ledger.remaining.filter(result => result.level === 'error').length, 0);
  const summary = renderSelfScanSummary(prepared);
  assert.match(summary, /Review required: 15 results/);
  assert.match(summary, /No source position: 4 results/);
  assert.match(summary, /Total: 5255 = 15 requiring review \+ 5240 reviewed references/);
});

test('new product findings and new payloads in reviewed files stay in the notification queue', () => {
  for (const relocate of [false, true]) {
    const raw = input(), added = structuredClone(ledger.files[0].results[0]);
    added.ruleId = 'UNREVIEWED_NOTIFICATION_CONTROL';
    added.message.text = 'New unreviewed observation';
    if (relocate) added.locations[0].physicalLocation.artifactLocation.uri = 'crates/runtime/src/app.rs';
    raw.runs[0].results.push(added);
    const output = prepareCodeScanning(raw, policy(), root);
    assert.deepEqual(output.sarif.runs[0].results, [...ledger.remaining, added]);
    assert.equal(output.audit.retainedResults, 16);
  }
  const raw = input(), duplicate = structuredClone(ledger.files[0].results[0]);
  raw.runs[0].results.push(duplicate);
  const output = prepareCodeScanning(raw, policy(), root);
  assert.equal(output.sarif.runs[0].results.filter(result => JSON.stringify(result) === JSON.stringify(duplicate)).length,
    raw.runs[0].results.filter(result => JSON.stringify(result) === JSON.stringify(duplicate)).length);
});

test('source drift fails closed for every reviewed context, without a directory-wide exception', () => {
  const raw = input();
  for (const prefix of ['crates/', 'tests/', 'examples/', 'tasks/', 'rules/', 'schemas/', 'shell/']) {
    const file = ledger.files.find(item => item.path.startsWith(prefix));
    assert(file, prefix);
    const changed = policy();
    changed.files.find(item => item.path === file.path).sha256 = '0'.repeat(64);
    const prepared = prepareCodeScanning(raw, changed, root);
    assert.deepEqual(prepared.sarif.runs[0].results.filter(result => filename(result) === file.path), file.results);
    assert.equal(prepared.audit.sourceChecks.find(item => item.path === file.path).status, 'changed');
  }
});

test('historical rule hashes remain recorded and changed source bindings fail closed', () => {
  const current = read('docs/quality/self-scan/SELF-SCAN-EXPECTATIONS.json');
  const historical = ledger.files.find(file => file.path === 'rules/default/crypto_api.yml');
  assert.equal(historical.sha256, '84f672787e883cbd2c2dd790ada8076c8a4c3f0628f3bc26aeb2b5bf4f7ea10a');
  assert.notEqual(current.files.find(file => file.path === historical.path).sha256, historical.sha256);
  const stale = policy();
  stale.files.find(file => file.path === historical.path).sha256 = historical.sha256;
  const prepared = prepareCodeScanning(input(), stale, root);
  assert.deepEqual(prepared.sarif.runs[0].results.filter(result => filename(result) === historical.path), historical.results);
  assert.equal(prepared.audit.retainedResults, ledger.counts.requireReview + historical.results.length);
  assert.equal(prepared.audit.sourceChecks.find(file => file.path === historical.path).status, 'changed');
});

test('historical static candidate references retain frozen source lines and payloads', () => {
  const review = read('docs/releases/static-candidate-20261117/evidence/POLICY-REBIND.json');
  const results = read('docs/releases/static-candidate-20261117/evidence/REVIEWED-RESULTS.json');
  assert.equal(results.length, 98);
  assert.equal(new Set(review.rows.map(row => row.path)).size, 4);
  const frozen = JSON.parse(require('node:zlib').gunzipSync(fs.readFileSync(path.join(repositoryRoot,
    'docs/releases/static-candidate-20261117/audit-fixes/STATIC-REFERENCE-SOURCES.json.gz'))));
  for (const source of Object.values(frozen.files)) assert.equal(createHash('sha256').update(source.text).digest('hex'), source.sha256);
  for (const row of review.rows) {
    const lines = frozen.files[row.path].text.split('\n');
    assert.equal(createHash('sha256').update(lines[row.newLine - 1]).digest('hex'), row.sourceLineSha256);
    assert(row.newLine > lines.findIndex(line => line.trim() === '#[cfg(test)]') + 1);
    assert.equal(results.filter(result => resultDigest(result) === row.newResultSha256).length, row.count);
  }
  const raw = { version: '2.1.0', runs: [{ tool: { driver: { name: 'pqc-scan' } }, results }] };
  const groups = [...new Set(review.rows.map(row => row.path))].map(name => ({
    path: name, sha256: frozen.files[name].sha256, reason: 'Historical reviewed inline test source references.',
    results: [...new Map(review.rows.filter(row => row.path === name).map(row => {
      const result = results.find(result => resultDigest(result) === row.newResultSha256);
      const region = result.locations[0].physicalLocation.region;
      return [row.newResultSha256, {ruleId:result.ruleId, line:region.startLine,
        column:region.startColumn ?? null, sha256:row.newResultSha256, count:row.count}];
    })).values()],
  }));
  const historical = reviewedResultTestSource({version:1, files:groups});
  try {
  for (const binding of historical.policy.files) {
    fs.writeFileSync(path.join(historical.root,binding.path), frozen.files[binding.path].text);
    binding.sha256=frozen.files[binding.path].sha256;
  }
  const prepared = prepareCodeScanning(raw, historical.policy, historical.root);
  assert.equal(prepared.audit.expectedResults, 98);
  assert.equal(prepared.audit.retainedResults, 0);
  const added = structuredClone(results[0]);
  added.message.text = 'New observation on an already reviewed test line';
  raw.runs[0].results.push(added);
  assert.deepEqual(prepareCodeScanning(raw, historical.policy, historical.root).sarif.runs[0].results, [added]);
  } finally { historical.cleanup(); }
});

test('historical product audit preserves its original source-bound review queue', () => {
  const bytes=fs.readFileSync(path.join(repositoryRoot,
    'docs/releases/static-candidate-20261117/audit-fixes/evidence/self-scan/report.sarif.gz'));
  const raw=JSON.parse(require('node:zlib').gunzipSync(bytes));
  const original=read('docs/quality/self-scan/html-report-20261118/HISTORICAL-PRODUCT-EXPECTATIONS.json');
  const sources=JSON.parse(require('node:zlib').gunzipSync(fs.readFileSync(path.join(repositoryRoot,
    'docs/quality/self-scan/html-report-20261118/HISTORICAL-PRODUCT-SOURCES.json.gz'))));
  const historical=reviewedResultTestSource(original);
  try {
  for(const file of historical.policy.files) {
    const source=sources.files[file.path];assert(source);
    assert.equal(createHash('sha256').update(source.text).digest('hex'),source.sha256);
    assert.equal(source.sha256,original.files.find(f=>f.path===file.path).sha256);
    fs.writeFileSync(path.join(historical.root,file.path),source.text);file.sha256=source.sha256;
  }
  const live=historical.policy;
  const prepared=prepareCodeScanning(raw,live,historical.root);
  assert.deepEqual([prepared.audit.rawResults,prepared.audit.expectedResults,prepared.audit.retainedResults],[487,477,10]);
  assert(prepared.audit.sourceChecks.every(row=>row.status==='verified'));
  const queue=prepared.sarif.runs.flatMap(run=>run.results);
  assert(queue.every(row=>['DEP_RUST_OPENSSL','DEP_PQC_CAPABLE_LIBRARY'].includes(row.ruleId)));
  const changed=structuredClone(raw), reference=changed.runs[0].results.find(row=>row.ruleId==='TLS_VERSION_1_0');
  reference.message.text='New unreviewed TLS report on an existing reference line';
  const checked=prepareCodeScanning(changed,live,historical.root);
  assert.equal(checked.audit.retainedResults,11);
  assert(checked.sarif.runs.flatMap(run=>run.results).some(row=>row.message.text===reference.message.text));
  } finally {historical.cleanup();}
});

test('current rebound references retain unchanged source lines and actual before/after result digests', () => {
  const directory='docs/quality/self-scan/html-report-20261118/';
  const review=read(directory+'POLICY-REBIND.json');
  const zipped=name=>JSON.parse(require('node:zlib').gunzipSync(fs.readFileSync(path.join(repositoryRoot,directory+name))));
  const sources=zipped('REBIND-SOURCES.json.gz');
  const before=zipped('BEFORE-CHANGED-FILES.sarif.gz').runs.flatMap(run=>run.results);
  const after=zipped('AFTER-CHANGED-FILES.sarif.gz').runs.flatMap(run=>run.results);
  const current=read('docs/quality/self-scan/SELF-SCAN-EXPECTATIONS.json');
  assert.equal(before.length,243);assert.equal(after.length,243);assert.equal(review.rows.length,203);
  for(const [name,source] of Object.entries(sources)) {
    assert.equal(createHash('sha256').update(source.beforeText).digest('hex'),source.beforeSha256);
    assert.equal(createHash('sha256').update(fs.readFileSync(path.join(repositoryRoot,name))).digest('hex'),source.afterSha256);
  }
  for(const row of review.rows) {
    const original=sources[row.path].beforeText.split('\n')[row.oldLine-1];
    const line=fs.readFileSync(path.join(repositoryRoot,row.path),'utf8').split('\n')[row.newLine-1];
    assert.equal(original,line);assert.equal(createHash('sha256').update(line).digest('hex'),row.sourceLineSha256);
    assert.equal(before.filter(result=>resultDigest(result)===row.oldResultSha256).length,row.count);
    assert.equal(after.filter(result=>resultDigest(result)===row.newResultSha256).length,row.count);
    const binding=current.files.find(file=>file.path===row.path).results.find(result=>result.sha256===row.newResultSha256);
    assert(binding);assert.equal(binding.ruleId,row.ruleId);assert.equal(binding.line,row.newLine);assert.equal(binding.count,row.count);
  }
  for(const removed of review.removedUnmatchedAllowances) {
    assert(!current.files.find(file=>file.path===removed.path).results.some(result=>result.sha256===removed.sha256));
  }
});
