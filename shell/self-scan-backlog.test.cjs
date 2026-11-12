'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { prepareCodeScanning } = require('./prepare-code-scanning.cjs');
const repositoryRoot = path.resolve(__dirname, '..');
const read = p => JSON.parse(fs.readFileSync(path.join(repositoryRoot, p), 'utf8'));
const { reviewedResultTestSource } = require('./reviewed-result-test-source.cjs');
const fixture = reviewedResultTestSource(read('docs/quality/SELF-SCAN-EXPECTATIONS.json'));
const root = fixture.root;
const policy = () => structuredClone(fixture.policy);
test.after(fixture.cleanup);
const records = () => read('docs/quality/self-scan-backlog/REVIEWED-RESULTS.json');
const document = results => ({ version: '2.1.0', runs: [{ tool: { driver: { name: 'pqc-scan' } }, results }] });

test('reviewed rule definitions, guidance, identifiers and OID labels are upload-only exceptions', () => {
  const results = records(), input = document(results), before = structuredClone(input);
  assert.equal(results.length, 130);
  assert.equal(new Set(results.map(r => r.locations[0].physicalLocation.artifactLocation.uri)).size, 11);
  const { sarif, audit } = prepareCodeScanning(input, policy(), root);
  assert.deepEqual(input, before);
  assert.equal(sarif.runs[0].results.length, 0);
  assert.deepEqual([audit.rawResults, audit.expectedResults, audit.retainedResults], [130, 130, 0]);
  assert(audit.sourceChecks.every(s => s.status === 'verified'));
});

test('an unreviewed result in a registered rule-definition file remains visible', () => {
  const results = records();
  const added = structuredClone(results.find(r => r.locations[0].physicalLocation.artifactLocation.uri.startsWith('rules/')));
  added.message.text = 'New unreviewed result in the same rule source.';
  const { sarif, audit } = prepareCodeScanning(document([...results, added]), policy(), root);
  assert.deepEqual(sarif.runs[0].results, [added]);
  assert.equal(audit.retainedResults, 1);
});

test('artifact-only test findings remain visible without a reviewed positive line', () => {
  const unknown = { ruleId: 'UNREVIEWED_CONTROL', level: 'error', message: { text: 'Unknown source location.' },
    locations: [{ physicalLocation: { artifactLocation: { uri: 'tests/quality/corpus/holdout/sbom.json' } } }] };
  const { sarif, audit } = prepareCodeScanning(document([...records(), unknown]), policy(), root);
  assert.deepEqual(sarif.runs[0].results, [unknown]);
  assert.deepEqual([audit.rawResults, audit.expectedResults, audit.retainedResults], [131, 130, 1]);
});
