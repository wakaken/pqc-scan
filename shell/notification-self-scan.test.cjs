'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { createHash } = require('node:crypto');
const { prepareCodeScanning, renderSelfScanSummary } = require('./prepare-code-scanning.cjs');
const root = path.resolve(__dirname, '..');
const read = name => JSON.parse(fs.readFileSync(path.join(root, name), 'utf8'));
const ledger = read('docs/quality/self-scan/notification-review-20261116/REVIEWED-RESULTS.json');
const policy = () => read('docs/quality/self-scan/SELF-SCAN-EXPECTATIONS.json');
const input = () => ({ version: '2.1.0', runs: [{ tool: { driver: { name: 'pqc-scan' } },
  results: structuredClone([...ledger.files.flatMap(file => file.results), ...ledger.remaining]) }] });
const filename = result => result.locations[0].physicalLocation.artifactLocation.uri;

test('current finite review binds every actual source and preserves dependency and unresolved observations', () => {
  assert.deepEqual(ledger.counts, { raw: 5255, reviewed: 5240, requireReview: 15 });
  for (const file of ledger.files) {
    assert.equal(createHash('sha256').update(fs.readFileSync(path.join(root, file.path))).digest('hex'), file.sha256, file.path);
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
