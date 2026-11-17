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

test('static candidate references map unchanged test lines and retain every unreviewed payload', () => {
  const review = read('docs/releases/static-candidate-20261117/evidence/POLICY-REBIND.json');
  const results = read('docs/releases/static-candidate-20261117/evidence/REVIEWED-RESULTS.json');
  assert.equal(results.length, 98);
  assert.equal(new Set(review.rows.map(row => row.path)).size, 4);
  for (const row of review.rows) {
    const lines = fs.readFileSync(path.join(repositoryRoot, row.path), 'utf8').split('\n');
    assert.equal(createHash('sha256').update(lines[row.newLine - 1]).digest('hex'), row.sourceLineSha256);
    assert(row.newLine > lines.findIndex(line => line.trim() === '#[cfg(test)]') + 1);
    assert.equal(results.filter(result => resultDigest(result) === row.newResultSha256).length, row.count);
  }
  const raw = { version: '2.1.0', runs: [{ tool: { driver: { name: 'pqc-scan' } }, results }] };
  const live = read('docs/quality/self-scan/SELF-SCAN-EXPECTATIONS.json');
  const prepared = prepareCodeScanning(raw, live, repositoryRoot);
  assert.equal(prepared.audit.expectedResults, 98);
  assert.equal(prepared.audit.retainedResults, 0);
  const added = structuredClone(results[0]);
  added.message.text = 'New observation on an already reviewed test line';
  raw.runs[0].results.push(added);
  assert.deepEqual(prepareCodeScanning(raw, live, repositoryRoot).sarif.runs[0].results, [added]);
});
