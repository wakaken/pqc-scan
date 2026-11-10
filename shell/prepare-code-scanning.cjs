#!/usr/bin/env node
'use strict';

// Repository CI projection only. The scanner and its raw SARIF remain unchanged.
const fs = require('node:fs');
const path = require('node:path');
const { createHash, randomUUID } = require('node:crypto');

const object = value => value !== null && typeof value === 'object' && !Array.isArray(value);
const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');
const digestPattern = /^[0-9a-f]{64}$/;
const positiveInteger = n => Number.isSafeInteger(n) && n > 0;
const literalPath = name => typeof name === 'string'
  && /^[A-Za-z0-9_.-]+(?:\/[A-Za-z0-9_.-]+)*$/.test(name)
  && name.split('/').every(part => part !== '.' && part !== '..');

function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(',')}]`;
  if (object(value)) return `{${Object.keys(value).sort()
    .map(key => `${JSON.stringify(key)}:${canonical(value[key])}`).join(',')}}`;
  if (value === null || ['string', 'boolean'].includes(typeof value)
    || (typeof value === 'number' && Number.isFinite(value))) return JSON.stringify(value);
  throw new Error('Invalid JSON value');
}
const resultDigest = result => sha256(canonical(result));

function exactKeys(value, allowed) {
  return object(value) && Object.keys(value).length === allowed.length
    && allowed.every(key => Object.hasOwn(value, key));
}
function validatePolicy(policy) {
  const invalid = () => { throw new Error('Invalid expectation policy'); };
  if (!exactKeys(policy, ['version', 'files']) || policy.version !== 1 || !Array.isArray(policy.files)) invalid();
  const paths = new Set();
  for (const file of policy.files) {
    if (!exactKeys(file, ['path', 'sha256', 'reason', 'results']) || !literalPath(file.path)
      || paths.has(file.path) || typeof file.sha256 !== 'string' || !digestPattern.test(file.sha256)
      || typeof file.reason !== 'string' || !file.reason.trim() || file.reason.length > 500
      || !Array.isArray(file.results) || !file.results.length) invalid();
    paths.add(file.path);
    const hashes = new Set();
    for (const result of file.results) {
      if (!exactKeys(result, ['ruleId', 'line', 'column', 'sha256', 'count'])
        || typeof result.ruleId !== 'string' || !result.ruleId
        || !positiveInteger(result.line) || !(result.column === null || positiveInteger(result.column))
        || !positiveInteger(result.count) || typeof result.sha256 !== 'string' || !digestPattern.test(result.sha256)
        || hashes.has(result.sha256)) invalid();
      hashes.add(result.sha256);
    }
  }
}
function validateSarif(sarif) {
  if (!object(sarif) || sarif.version !== '2.1.0' || !Array.isArray(sarif.runs) || !sarif.runs.length
    || sarif.runs.some(run => !object(run) || typeof run.tool?.driver?.name !== 'string'
      || !Array.isArray(run.results) || run.results.some(result => !object(result)))) {
    throw new Error('Invalid SARIF');
  }
}

function sourceStatus(root, file) {
  try {
    const absolute = path.join(root, file.path);
    // Reject file and parent symlink aliases, including aliases within the root.
    if (fs.realpathSync(absolute) !== absolute || !fs.lstatSync(absolute).isFile()) return 'unavailable';
    return sha256(fs.readFileSync(absolute)) === file.sha256 ? 'verified' : 'changed';
  } catch {
    return 'unavailable';
  }
}
function identity(run, result) {
  if (run.tool.driver.name !== 'pqc-scan'
    || ['artifacts', 'originalUriBaseIds', 'invocations'].some(key => Object.hasOwn(run, key))
    || !Array.isArray(result.locations) || result.locations.length !== 1
    || ['relatedLocations', 'codeFlows', 'stacks'].some(key => Object.hasOwn(result, key))) return null;
  const location = result.locations[0];
  const physical = location?.physicalLocation;
  const artifact = physical?.artifactLocation;
  const region = physical?.region;
  if (!exactKeys(artifact, ['uri']) || !literalPath(artifact.uri)
    || !positiveInteger(region?.startLine)
    || (Object.hasOwn(region, 'startColumn') && !positiveInteger(region.startColumn))
    || typeof result.ruleId !== 'string' || !result.ruleId
    || (Array.isArray(location.logicalLocations) && location.logicalLocations.length)) return null;
  const hash = resultDigest(result);
  return { key: `${artifact.uri}\0${hash}`, path: artifact.uri, hash,
    ruleId: result.ruleId, line: region.startLine, column: Object.hasOwn(region, 'startColumn') ? region.startColumn : null };
}

function prepareCodeScanning(sarif, policy, root) {
  validateSarif(sarif);
  validatePolicy(policy);
  const realRoot = fs.realpathSync(root);
  const allowances = new Map();
  const sourceChecks = [];
  for (const file of policy.files) {
    const status = sourceStatus(realRoot, file);
    sourceChecks.push({ path: file.path, expectedSha256: file.sha256, status });
    if (status !== 'verified') continue;
    for (const expected of file.results) {
      allowances.set(`${file.path}\0${expected.sha256}`, { ...expected, reason: file.reason });
    }
  }
  // Count before removal, globally across runs: allowances are never reset per run.
  const counts = new Map();
  const identities = sarif.runs.map(run => run.results.map(result => {
    const id = identity(run, result);
    if (id) counts.set(id.key, (counts.get(id.key) || 0) + 1);
    return id;
  }));
  const output = structuredClone(sarif);
  const audit = { schemaVersion: 1, expectationsSha256: sha256(canonical(policy)),
    rawResults: 0, expectedResults: 0, retainedResults: 0,
    sourceChecks, omissions: [] };
  const recorded = new Set();
  for (const [runIndex, run] of output.runs.entries()) {
    audit.rawResults += run.results.length;
    run.results = run.results.filter((result, resultIndex) => {
      const id = identities[runIndex][resultIndex];
      const expected = id && allowances.get(id.key);
      if (!expected || expected.ruleId !== id.ruleId || expected.line !== id.line
        || expected.column !== id.column || counts.get(id.key) !== expected.count) return true;
      audit.expectedResults++;
      if (!recorded.has(id.key)) {
        recorded.add(id.key);
        audit.omissions.push({ path: id.path, ruleId: id.ruleId, line: id.line, column: id.column,
          resultSha256: id.hash, count: expected.count, reason: expected.reason });
      }
      return false;
    });
  }
  audit.retainedResults = audit.rawResults - audit.expectedResults;
  return { sarif: output, audit };
}

// Resolve missing destinations through existing ancestors, so path/symlink and
// existing hard-link aliases cannot overwrite input, policy or another output.
function destinationIdentity(filename) {
  const absolute = path.resolve(filename);
  let existing = absolute;
  const missing = [];
  while (!fs.existsSync(existing)) {
    missing.unshift(path.basename(existing));
    const parent = path.dirname(existing);
    if (parent === existing) throw new Error('Invalid destination');
    existing = parent;
  }
  const canonicalPath = path.join(fs.realpathSync(existing), ...missing);
  const stat = missing.length ? null : fs.statSync(existing);
  return { path: canonicalPath, inode: stat && `${stat.dev}:${stat.ino}` };
}
function assertDistinct(filenames) {
  const identities = filenames.map(destinationIdentity);
  for (let i = 0; i < identities.length; i++) {
    for (let j = i + 1; j < identities.length; j++) {
      if (identities[i].path === identities[j].path
        || (identities[i].inode && identities[i].inode === identities[j].inode)) {
        throw new Error('Input and output paths must be distinct');
      }
    }
  }
}
function writeJson(filename, value) {
  fs.mkdirSync(path.dirname(path.resolve(filename)), { recursive: true });
  const temporary = `${filename}.tmp-${process.pid}-${randomUUID()}`;
  try {
    fs.writeFileSync(temporary, `${JSON.stringify(value, null, 2)}\n`, { flag: 'wx' });
    fs.renameSync(temporary, filename);
  } finally {
    fs.rmSync(temporary, { force: true });
  }
}
function main(args) {
  const keys = ['--input', '--expectations', '--root', '--output', '--audit'];
  const options = new Map();
  for (let i = 0; i < args.length; i += 2) {
    if (!keys.includes(args[i]) || options.has(args[i]) || !args[i + 1]
      || args[i + 1].startsWith('--')) throw new Error('Invalid arguments');
    options.set(args[i], args[i + 1]);
  }
  if (options.size !== keys.length) throw new Error('Invalid arguments');
  assertDistinct(['--input', '--expectations', '--output', '--audit'].map(key => options.get(key)));
  const input = fs.readFileSync(options.get('--input'));
  const policy = JSON.parse(fs.readFileSync(options.get('--expectations'), 'utf8'));
  const { sarif, audit } = prepareCodeScanning(JSON.parse(input), policy, options.get('--root'));
  audit.rawSarifSha256 = sha256(input);
  writeJson(options.get('--output'), sarif);
  writeJson(options.get('--audit'), audit);
  process.stdout.write(`Self-scan: ${audit.rawResults} raw, ${audit.expectedResults} reviewed expected results, ${audit.retainedResults} retained.\n`);
}

module.exports = { prepareCodeScanning, resultDigest };
if (require.main === module) {
  try { main(process.argv.slice(2)); }
  catch {
    process.stderr.write('Code Scanning preparation failed; check input and expectation policy.\n');
    process.exitCode = 1;
  }
}
