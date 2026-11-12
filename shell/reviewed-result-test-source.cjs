'use strict';
// Unit-test sources are synthetic. This never changes production allowances or
// asserts that historical SARIF locations describe the current source tree.
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { createHash } = require('node:crypto');
function reviewedResultTestSource(originalPolicy) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'pqc-reviewed-source-'));
  const policy = structuredClone(originalPolicy);
  for (const file of policy.files) {
    const absolute = path.resolve(root, file.path);
    if (!absolute.startsWith(root + path.sep)) throw new Error('Invalid synthetic fixture path');
    fs.mkdirSync(path.dirname(absolute), { recursive: true });
    const bytes = Buffer.from(`Synthetic source-hash fixture for ${file.path}\n`);
    fs.writeFileSync(absolute, bytes);
    file.sha256 = createHash('sha256').update(bytes).digest('hex');
  }
  return { root, policy, cleanup: () => fs.rmSync(root, { recursive: true, force: true }) };
}
module.exports = { reviewedResultTestSource };
