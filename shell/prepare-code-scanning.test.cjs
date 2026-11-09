const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const { createHash } = require('node:crypto');

const SOURCE_SHA = '122decc454640def9d15b0a4fcd8727ba1ea61848f227158e7d4ecdfdbe3fcee';
// Independently computed with Python json.dumps(sort_keys=True, separators=(',', ':')).
const RESULT_SHA = 'b58582148fbaf6d26da96b04c8956657de8419e249c7dc5ae9e69b3259a963b5';
const script = path.join(__dirname, 'prepare-code-scanning.cjs');
const prepare = (...args) => require(script).prepareCodeScanning(...args);
const digest = value => require(script).resultDigest(value);

function result() {
  return {
    ruleId: 'TEST_ISSUE', level: 'error', message: { text: 'Synthetic test observation.' },
    locations: [{ physicalLocation: {
      artifactLocation: { uri: 'tests/fixture.txt' }, region: { startLine: 1, startColumn: 1 },
    } }], properties: { kind: 'synthetic' },
  };
}
function document(results = [result()]) {
  return { version: '2.1.0', $schema: 'https://json.schemastore.org/sarif-2.1.0.json',
    runs: [{ tool: { driver: { name: 'pqc-scan', rules: [{ id: 'TEST_ISSUE' }] } }, results }] };
}
function policy() {
  return { version: 1, files: [{ path: 'tests/fixture.txt', sha256: SOURCE_SHA,
    reason: 'Reviewed synthetic detector test input.',
    results: [{ ruleId: 'TEST_ISSUE', line: 1, column: 1, sha256: RESULT_SHA, count: 1 }] }] };
}
function fixture(t) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'pqc-ci-expectations-'));
  fs.mkdirSync(path.join(root, 'tests'));
  fs.writeFileSync(path.join(root, 'tests/fixture.txt'), 'reviewed synthetic input\n');
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));
  return root;
}

test('only the exact reviewed result is omitted; native input and run metadata stay intact', t => {
  const root = fixture(t), input = document(), before = structuredClone(input);
  input.runs[0].automationDetails = { id: 'self-scan' };
  before.runs[0].automationDetails = { id: 'self-scan' };
  const { sarif, audit } = prepare(input, policy(), root);
  assert.equal(sarif.runs[0].results.length, 0);
  assert.deepEqual(input, before);
  assert.deepEqual(sarif.runs[0].tool, input.runs[0].tool);
  assert.deepEqual(sarif.runs[0].automationDetails, { id: 'self-scan' });
  assert.deepEqual([audit.rawResults, audit.expectedResults, audit.retainedResults], [1, 1, 0]);
});

test('canonical digest is independent of object key ordering and includes nested properties', () => {
  assert.equal(digest(result()), RESULT_SHA);
  assert.equal(digest(Object.fromEntries(Object.entries(result()).reverse())), RESULT_SHA);
  const changed = result(); changed.properties.extra = 'new';
  assert.notEqual(digest(changed), RESULT_SHA);
});

test('a new result in the same fixture file remains actionable', t => {
  const root = fixture(t), added = result();
  added.locations[0].physicalLocation.region.startLine = 2;
  const { sarif, audit } = prepare(document([result(), added]), policy(), root);
  assert.deepEqual(sarif.runs[0].results, [added]);
  assert.deepEqual([audit.rawResults, audit.expectedResults, audit.retainedResults], [2, 1, 1]);
});

test('changed message, rule, level or properties are never covered by a previous allowance', t => {
  const root = fixture(t);
  for (const mutate of [
    r => { r.message.text = 'Unexpected new issue'; },
    r => { r.ruleId = 'NEW_ISSUE'; },
    r => { r.level = 'warning'; },
    r => { r.properties.extra = 'new context'; },
  ]) {
    const added = result(); mutate(added);
    assert.deepEqual(prepare(document([added]), policy(), root).sarif.runs[0].results, [added]);
  }
});

test('changing even the body of a reviewed source file keeps its result', t => {
  const root = fixture(t);
  fs.appendFileSync(path.join(root, 'tests/fixture.txt'), 'UNREVIEWED_BODY_CANARY\n');
  const input = document();
  const { sarif, audit } = prepare(input, policy(), root);
  assert.deepEqual(sarif, input);
  assert.equal(audit.expectedResults, 0);
  assert(!JSON.stringify(audit).includes('UNREVIEWED_BODY_CANARY'));
});

test('missing or symlinked sources do not borrow a reviewed source identity', t => {
  const root = fixture(t), source = path.join(root, 'tests/fixture.txt');
  fs.renameSync(source, path.join(root, 'real.txt'));
  assert.deepEqual(prepare(document(), policy(), root).sarif, document());
  fs.symlinkSync(path.join(root, 'real.txt'), source);
  assert.deepEqual(prepare(document(), policy(), root).sarif, document());
});

test('an excess duplicate keeps the entire group rather than consuming a prefix allowance', t => {
  const root = fixture(t), input = document([result(), result()]);
  assert.deepEqual(prepare(input, policy(), root).sarif, input);
  const approved = policy(); approved.files[0].results[0].count = 2;
  assert.equal(prepare(input, approved, root).sarif.runs[0].results.length, 0);
});

test('allowance counts cover the whole SARIF, including repeated runs', t => {
  const root = fixture(t), input = document();
  input.runs.push(structuredClone(input.runs[0]));
  const { sarif, audit } = prepare(input, policy(), root);
  assert.deepEqual(sarif, input);
  assert.deepEqual([audit.rawResults, audit.expectedResults, audit.retainedResults], [2, 0, 2]);
});

test('other tools and multi-location or related/code-flow results stay unchanged', t => {
  const root = fixture(t);
  for (const mutate of [
    doc => { doc.runs[0].tool.driver.name = 'another-tool'; },
    doc => { doc.runs[0].results[0].locations.push(structuredClone(result().locations[0])); },
    doc => { doc.runs[0].results[0].relatedLocations = [{ id: 1 }]; },
    doc => { doc.runs[0].results[0].codeFlows = [{ threadFlows: [] }]; },
  ]) {
    const input = document(); mutate(input);
    const approved = policy(); approved.files[0].results[0].sha256 = digest(input.runs[0].results[0]);
    assert.deepEqual(prepare(input, approved, root).sarif, input);
  }
});

test('indirect URI resolution is ineligible even with a matching result digest', t => {
  const root = fixture(t);
  for (const mutate of [
    doc => { doc.runs[0].results[0].locations[0].physicalLocation.artifactLocation.uriBaseId = 'OTHER'; },
    doc => { doc.runs[0].results[0].locations[0].physicalLocation.artifactLocation.index = 0; },
    doc => { doc.runs[0].originalUriBaseIds = { OTHER: { uri: 'file:///other/' } }; },
    doc => { doc.runs[0].artifacts = [{ location: { uri: 'other.txt' } }]; },
    doc => { doc.runs[0].invocations = [{ workingDirectory: { uri: 'file:///other/' } }]; },
  ]) {
    const input = document(); mutate(input);
    const approved = policy(); approved.files[0].results[0].sha256 = digest(input.runs[0].results[0]);
    assert.deepEqual(prepare(input, approved, root).sarif, input);
  }
});

test('unregistered product locations and missing regions are retained', t => {
  const root = fixture(t);
  for (const change of [
    r => { r.locations[0].physicalLocation.artifactLocation.uri = 'src/product.txt'; },
    r => { delete r.locations[0].physicalLocation.region; },
  ]) {
    const r = result(); change(r);
    const input = document([r]);
    assert.deepEqual(prepare(input, policy(), root).sarif, input);
  }
});

test('unsafe or nonliteral expectation paths and duplicate policies are rejected', t => {
  const root = fixture(t);
  for (const name of ['../fixture.txt', '/tmp/fixture.txt', 'tests\\fixture.txt', 'tests/*.txt', 'tests/%66ixture.txt']) {
    const approved = policy(); approved.files[0].path = name;
    assert.throws(() => prepare(document(), approved, root), /Invalid expectation policy/);
  }
  for (const mutate of [
    p => { p.files.push(structuredClone(p.files[0])); },
    p => { p.files[0].results.push(structuredClone(p.files[0].results[0])); },
    p => { p.files[0].results[0].count = 0; },
    p => { p.files[0].sha256 = '*'; },
    p => { p.files[0].sha256 = [SOURCE_SHA]; },
    p => { p.files[0].results[0].sha256 = [RESULT_SHA]; },
    p => { p.files[0].reason = ''; },
    p => { p.version = 99; },
  ]) {
    const approved = policy(); mutate(approved);
    assert.throws(() => prepare(document(), approved, root), /Invalid expectation policy/);
  }
});

test('empty or malformed SARIF cannot become a successful empty upload', t => {
  const root = fixture(t);
  for (const input of [null, {}, { version: '2.1.0', runs: [] },
    { version: '2.0.0', runs: document().runs }, { version: '2.1.0', runs: [{}] }]) {
    assert.throws(() => prepare(input, policy(), root), /Invalid SARIF/);
  }
  assert.deepEqual(prepare(document([]), policy(), root).sarif, document([]));
});

test('audit contains identities and reasons without copying message/source payloads', t => {
  const root = fixture(t), input = document();
  input.runs[0].results[0].message.text = 'SYNTHETIC_MESSAGE_CANARY';
  const approved = policy(); approved.files[0].results[0].sha256 = digest(input.runs[0].results[0]);
  const { audit } = prepare(input, approved, root);
  assert.equal(audit.expectedResults, 1);
  assert(!JSON.stringify(audit).includes('SYNTHETIC_MESSAGE_CANARY'));
});

function invoke(root, overrides = {}) {
  const options = { '--input': path.join(root, 'raw.sarif'), '--expectations': path.join(root, 'policy.json'),
    '--root': root, '--output': path.join(root, 'upload.sarif'), '--audit': path.join(root, 'audit.json'), ...overrides };
  const args = Object.entries(options).flat();
  return spawnSync(process.execPath, [script, ...args], { encoding: 'utf8' });
}
function cliInputs(root) {
  fs.writeFileSync(path.join(root, 'raw.sarif'), JSON.stringify(document()));
  fs.writeFileSync(path.join(root, 'policy.json'), JSON.stringify(policy()));
}

test('CLI writes a separate upload and audit, preserving the raw SARIF bytes', t => {
  const root = fixture(t); cliInputs(root);
  const before = fs.readFileSync(path.join(root, 'raw.sarif'));
  const run = invoke(root);
  assert.equal(run.status, 0, run.stderr);
  assert.deepEqual(fs.readFileSync(path.join(root, 'raw.sarif')), before);
  assert.equal(JSON.parse(fs.readFileSync(path.join(root, 'upload.sarif'))).runs[0].results.length, 0);
  assert.equal(JSON.parse(fs.readFileSync(path.join(root, 'audit.json'))).expectedResults, 1);
});

test('CLI refuses output aliases and reports malformed input without echoing a canary', t => {
  const root = fixture(t); cliInputs(root);
  const alias = invoke(root, { '--output': path.join(root, 'raw.sarif') });
  assert.notEqual(alias.status, 0);
  assert.equal(JSON.parse(fs.readFileSync(path.join(root, 'raw.sarif'))).runs[0].results.length, 1);
  fs.writeFileSync(path.join(root, 'raw.sarif'), '{INVALID_JSON_CANARY');
  const bad = invoke(root);
  assert.notEqual(bad.status, 0);
  assert.match(bad.stderr, /preparation failed/);
  assert(!bad.stderr.includes('INVALID_JSON_CANARY'));
  assert(!fs.existsSync(path.join(root, 'upload.sarif')));
});


test('CLI also protects raw input through hard-link and symlink output aliases', t => {
  const root = fixture(t); cliInputs(root);
  const raw = path.join(root, 'raw.sarif');
  for (const kind of ['hard', 'symbolic']) {
    const alias = path.join(root, kind + '.sarif');
    if (kind === 'hard') fs.linkSync(raw, alias); else fs.symlinkSync(raw, alias);
    const before = fs.readFileSync(raw);
    assert.notEqual(invoke(root, { '--output': alias }).status, 0);
    assert.deepEqual(fs.readFileSync(raw), before);
  }
});

test('actual scanner still detects identical fixture and product inputs; only the reviewed fixture is omitted', t => {
  const root = fixture(t), repository = path.resolve(__dirname, '..');
  const input = path.join(root, 'input'), output = path.join(root, 'output');
  fs.mkdirSync(input); fs.mkdirSync(output);
  const text = '-----BEGIN PRIVATE KEY-----\nSYNTHETIC_CI_CANARY\n-----END PRIVATE KEY-----\n';
  for (const name of ['fixture.txt', 'product.txt']) fs.writeFileSync(path.join(input, name), text);
  const scan = spawnSync('cargo', ['run', '--offline', '--manifest-path', path.join(repository, 'Cargo.toml'),
    '--bin', 'pqc-scan', '--', 'scan', input, '--format', 'sarif', '--out-dir', output,
    '--rules-dir', path.join(repository, 'rules')], { cwd: repository, encoding: 'utf8' });
  assert.equal(scan.status, 0, scan.stderr);
  const rawPath = path.join(output, 'report.sarif'), rawBytes = fs.readFileSync(rawPath);
  const raw = JSON.parse(rawBytes), results = raw.runs[0].results;
  assert.equal(results.length, 2);
  assert(results.every(r => r.ruleId === 'PRIVATE_KEY_PKCS8_HEADER' && r.level === 'error'));
  const expected = results.find(r => r.locations[0].physicalLocation.artifactLocation.uri === 'fixture.txt');
  assert(expected);
  const region = expected.locations[0].physicalLocation.region;
  const reviewed = { version: 1, files: [{ path: 'fixture.txt',
    sha256: createHash('sha256').update(fs.readFileSync(path.join(input, 'fixture.txt'))).digest('hex'),
    reason: 'Explicit synthetic end-to-end scanner fixture.', results: [{ ruleId: expected.ruleId,
      line: region.startLine, column: region.startColumn, sha256: digest(expected), count: 1 }] }] };
  const policyPath = path.join(output, 'policy.json'); fs.writeFileSync(policyPath, JSON.stringify(reviewed));
  const projection = invoke(root, { '--input': rawPath, '--expectations': policyPath, '--root': input });
  assert.equal(projection.status, 0, projection.stderr);
  assert.deepEqual(fs.readFileSync(rawPath), rawBytes);
  const retained = JSON.parse(fs.readFileSync(path.join(root, 'upload.sarif'))).runs[0].results;
  assert.equal(retained.length, 1);
  assert.equal(retained[0].locations[0].physicalLocation.artifactLocation.uri, 'product.txt');
  assert.equal(retained[0].level, 'error');
  assert(!rawBytes.includes(Buffer.from('SYNTHETIC_CI_CANARY')));
});
