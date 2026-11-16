'use strict';

// Public notifications intentionally contain no advisory or private report data.
module.exports = async function notify({ github, context }) {
  const allowed = new Set(['push', 'schedule', 'workflow_dispatch']);
  const defaultBranch = context.payload.repository.default_branch;
  if (!allowed.has(context.eventName) || context.ref !== `refs/heads/${defaultBranch}` ||
      !Number.isSafeInteger(context.runId) || context.runId <= 0) {
    throw new Error('Dependency notifications require a trusted default-branch run');
  }
  const { owner, repo } = context.repo;
  const marker = '<!-- pqc-scan:dependency-audit -->';
  const body = `${marker}\nThe dependency vulnerability check failed or could not complete.\n` +
    `Review the run and retained evidence: https://github.com/${owner}/${repo}/actions/runs/${context.runId}\n\n` +
    'Confirm applicability, prepare any necessary fix, and rerun the check. ' +
    'Do not post private vulnerability details here. Close this issue after verification.';
  const issues = await github.paginate(github.rest.issues.listForRepo,
    { owner, repo, state: 'open', creator: 'github-actions[bot]', assignee: owner, per_page: 100 });
  const existing = issues.find(issue => !issue.pull_request &&
    issue.user?.login === 'github-actions[bot]' && issue.body?.startsWith(marker));
  if (existing) {
    await github.rest.issues.createComment({ owner, repo, issue_number: existing.number, body });
  } else {
    await github.rest.issues.create({ owner, repo, title: 'Dependency vulnerability check requires attention',
      assignees: [owner], body });
  }
};
