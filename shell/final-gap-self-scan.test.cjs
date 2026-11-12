'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { prepareCodeScanning } = require('./prepare-code-scanning.cjs');
const repositoryRoot = path.resolve(__dirname, '..');
const read = name => JSON.parse(fs.readFileSync(path.join(repositoryRoot, name)));
const ledger = read('docs/quality/FINAL-GAPS-SELF-SCAN-REVIEW.json');
const { reviewedResultTestSource } = require('./reviewed-result-test-source.cjs');
const fixture = reviewedResultTestSource(read('docs/quality/SELF-SCAN-EXPECTATIONS.json'));
const { root, policy } = fixture;
test.after(fixture.cleanup);
const input = () => ({version:'2.1.0', runs:[{tool:{driver:{name:'pqc-scan'}},
  results:structuredClone(ledger.results.map(r => r.result))}]});

test('reviewed final-gap results omit only the finite 81 matches and retain inventory', () => {
  const raw = input(), before = JSON.stringify(raw);
  const result = prepareCodeScanning(raw, policy, root);
  assert.equal(JSON.stringify(raw), before);
  assert.equal(result.audit.expectedResults, 81);
  assert.deepEqual(result.sarif.runs[0].results,
    ledger.results.filter(r => r.disposition === 'retained_inventory').map(r => r.result));
  assert.equal(result.audit.retainedResults, 28);
});

test('a new error at a reviewed diagnostic location remains actionable', () => {
  const raw = input();
  const row = ledger.results.find(r => r.disposition === 'reviewed_expected'
    && r.path.endsWith('/src/runtime.rs'));
  const unexpected = structuredClone(row.result);
  unexpected.message.text += ' New unreviewed observation';
  unexpected.level = 'error';
  raw.runs[0].results.push(unexpected);
  const result = prepareCodeScanning(raw, policy, root);
  assert.ok(result.sarif.runs[0].results.some(r => JSON.stringify(r) === JSON.stringify(unexpected)));
  assert.equal(result.audit.retainedResults, 29);
});

test('stale source binding and excess identical occurrences invalidate finite allowances', () => {
  const row = ledger.results.find(r => r.disposition === 'reviewed_expected'
    && r.path.endsWith('/src/runtime.rs'));
  const stale = structuredClone(policy);
  stale.files.find(f => f.path === row.path).sha256 = '0'.repeat(64);
  const result = prepareCodeScanning(input(), stale, root);
  assert.ok(result.sarif.runs[0].results.some(r => JSON.stringify(r) === JSON.stringify(row.result)));
  const doubled = input();
  doubled.runs[0].results.push(structuredClone(row.result));
  const excess = prepareCodeScanning(doubled, policy, root);
  assert.equal(excess.sarif.runs[0].results.filter(r => JSON.stringify(r) === JSON.stringify(row.result)).length, 2);
});
