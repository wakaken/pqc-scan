'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { prepareCodeScanning } = require('./prepare-code-scanning.cjs');
const root = path.resolve(__dirname, '..');
const read = file => JSON.parse(fs.readFileSync(path.join(root, file), 'utf8'));
const review = read('docs/records/document-relocation-20261114/SELF-SCAN-REVIEW.json');
const policy = () => read('docs/quality/self-scan/SELF-SCAN-EXPECTATIONS.json');
const results = () => structuredClone(review.files.flatMap(f => f.results));
const document = rows => ({ version: '2.1.0', runs: [{ tool: { driver: { name: 'pqc-scan' } }, results: rows }] });
const sourcePath = r => r.locations[0].physicalLocation.artifactLocation.uri;

test('relocated evidence review matches actual source bytes and omits only its exact upload observations', () => {
  assert.equal(review.files.length, 37);
  for (const file of review.files) {
    const bytes = fs.readFileSync(path.join(root, file.path));
    assert.equal(crypto.createHash('sha256').update(bytes).digest('hex'), file.sha256, file.path);
    assert(file.reason.length > 20);
    assert(file.results.every(r => sourcePath(r) === file.path));
  }
  const raw = document(results()), before = structuredClone(raw);
  const { sarif, audit } = prepareCodeScanning(raw, policy(), root);
  assert.deepEqual(raw, before, 'raw evidence must remain unchanged');
  assert.deepEqual([audit.rawResults, audit.expectedResults, audit.retainedResults], [926, 926, 0]);
  assert.deepEqual(sarif.runs[0].results, []);
});

test('new payloads, additional occurrences and findings outside reviewed evidence remain visible', () => {
  const known = results()[0], changed = structuredClone(known), outside = structuredClone(known);
  changed.message.text += ' Unreviewed observation.';
  outside.locations[0].physicalLocation.artifactLocation.uri = 'crates/pqc_scan_core/src/lib.rs';
  for (const added of [changed, outside]) {
    const { sarif, audit } = prepareCodeScanning(document([...results(), added]), policy(), root);
    assert.deepEqual(sarif.runs[0].results, [added]);
    assert.equal(audit.retainedResults, 1);
  }
  const { sarif } = prepareCodeScanning(document([...results(), known]), policy(), root);
  assert.equal(sarif.runs[0].results.filter(r => JSON.stringify(r) === JSON.stringify(known)).length, 2);
});

test('changed whole-file bindings retain every observation from the changed evidence', () => {
  for (const file of review.files) {
    const p = policy(), entry = p.files.find(f => f.path === file.path);
    assert(entry, file.path);
    entry.sha256 = '0'.repeat(64);
    const { sarif, audit } = prepareCodeScanning(document(results()), p, root);
    assert.deepEqual(sarif.runs[0].results, file.results, file.path);
    assert.equal(audit.sourceChecks.find(f => f.path === file.path).status, 'changed');
  }
});
