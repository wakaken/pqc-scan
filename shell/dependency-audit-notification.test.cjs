const { test } = require('node:test');
const assert = require('node:assert/strict');
const notify = require('../scripts/notify-dependency-audit.cjs');
function fixture(eventName = 'schedule', existing = []) {
  const calls = [];
  const github = { rest: { issues: {
    listForRepo: async () => {},
    create: async args => calls.push(['create', args]),
    createComment: async args => calls.push(['comment', args]),
  } }, paginate: async () => existing };
  const context = { eventName, ref: 'refs/heads/develop', runId: 123,
    repo: { owner: 'wakaken', repo: 'pqc-scan' },
    payload: { repository: { default_branch: 'develop', html_url: 'https://github.com/wakaken/pqc-scan' },
      untrusted: 'PRIVATE-REPORT-SECRET <script>unsafe</script>' } };
  return { github, context, calls };
}
test('default-branch failure creates an assigned generic notification with run link only', async () => {
  const f = fixture(); await notify(f);
  assert.equal(f.calls.length, 1); const [kind, value] = f.calls[0];
  assert.equal(kind, 'create'); assert.deepEqual(value.assignees, ['wakaken']);
  assert.match(value.body, /https:\/\/github.com\/wakaken\/pqc-scan\/actions\/runs\/123/);
  assert.doesNotMatch(value.body, /PRIVATE-REPORT-SECRET|<script>|RUSTSEC-/);
});
test('reuse only an actual bot notification Issue, not a PR or a user issue', async () => {
  const marker = '<!-- pqc-scan:dependency-audit -->';
  const rows = [
    { number: 1, body: marker, user: { login: 'someone' } },
    { number: 2, body: marker, user: { login: 'github-actions[bot]' }, pull_request: {} },
    { number: 3, body: marker, user: { login: 'github-actions[bot]' } },
  ];
  const f = fixture('push', rows); await notify(f);
  assert.equal(f.calls[0][0], 'comment'); assert.equal(f.calls[0][1].issue_number, 3);
});
test('fork and same-repository PR events never invoke write APIs', async () => {
  for (const name of ['pull_request', 'pull_request_target', 'workflow_run']) {
    const f = fixture(name); await assert.rejects(notify(f)); assert.deepEqual(f.calls, []);
  }
});
test('manual runs off default branch and invalid run IDs cannot notify', async () => {
  const f = fixture('workflow_dispatch'); f.context.ref = 'refs/heads/feature';
  await assert.rejects(notify(f)); assert.deepEqual(f.calls, []);
  const g = fixture(); g.context.runId = '123/../../other';
  await assert.rejects(notify(g)); assert.deepEqual(g.calls, []);
});
test('API failures surface instead of claiming notification delivery', async () => {
  const f = fixture(); f.github.rest.issues.create = async () => { throw new Error('API denied'); };
  await assert.rejects(notify(f), /API denied/);
});
