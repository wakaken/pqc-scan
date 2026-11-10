'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { prepareCodeScanning } = require('./prepare-code-scanning.cjs');
const root = path.resolve(__dirname, '..');
const read = name => JSON.parse(fs.readFileSync(path.join(root, name), 'utf8'));
const policy = () => read('docs/quality/SELF-SCAN-EXPECTATIONS.json');
const reviewed = () => read('docs/quality/pr14/REVIEWED-RESULTS.json');
const sourcePath = r => r.locations[0].physicalLocation.artifactLocation.uri;
const document = results => ({ version: '2.1.0', runs: [{
  tool: { driver: { name: 'pqc-scan' } }, results,
}] });

test('PR14 reviewed test inputs and classification data are omitted only from the upload copy', () => {
  const results = reviewed();
  assert.equal(results.length, 63);
  assert.equal(results.filter(r => sourcePath(r).includes('/tests/')).length, 15);
  assert.equal(results.filter(r => !sourcePath(r).includes('/tests/')).length, 48);
  const input = document(results), before = structuredClone(input);
  const { sarif, audit } = prepareCodeScanning(input, policy(), root);
  assert.deepEqual(input, before);
  assert.equal(sarif.runs[0].results.length, 0);
  assert.deepEqual([audit.rawResults, audit.expectedResults, audit.retainedResults], [63, 63, 0]);
  assert(audit.sourceChecks.every(s => s.status === 'verified'));
});

test('new error at the same reviewed production source position remains actionable', () => {
  const results = reviewed(), added = structuredClone(results.find(r => !sourcePath(r).includes('/tests/')));
  added.ruleId = 'UNREGISTERED_SELF_SCAN_CONTROL';
  added.level = 'error';
  added.message.text = 'Unreviewed production finding.';
  const { sarif, audit } = prepareCodeScanning(document([...results, added]), policy(), root);
  assert.deepEqual(sarif.runs[0].results, [added]);
  assert.deepEqual([audit.rawResults, audit.expectedResults, audit.retainedResults], [64, 63, 1]);
});

test('stale classification-source binding retains every finding from that file', () => {
  const results = reviewed(), p = policy();
  const filename = 'crates/pqc_scan_core/src/observation.rs';
  const entry = p.files.find(f => f.path === filename);
  assert(entry, 'classification source must be explicitly registered');
  entry.sha256 = '0'.repeat(64);
  const { sarif, audit } = prepareCodeScanning(document(results), p, root);
  const retained = results.filter(r => sourcePath(r) === filename);
  assert.deepEqual(sarif.runs[0].results, retained);
  assert.equal(audit.retainedResults, retained.length);
  assert.equal(audit.sourceChecks.find(s => s.path === filename).status, 'changed');
});
