# pqc-scan project constraints

Respond in clear, concise and polite Japanese.

## Development harness

Use the installed `pstack:poteto-mode` as the primary development harness.
Its skills, playbooks, principles and configured provider dispatch own the
general workflow, delegation, design exploration and review orchestration.
This file replaces the root `AGENTS.md` during the trial; do not import it.
The legacy review prompts, task lifecycle and restart protocol are retained
for history and rollback, not as active workflow instructions. References to
them in older tasks, guides or skills do not reactivate them. Do not automatically
compose another general workflow (including GSD or Superpowers) with pstack.

## Review model selection

Use these explicit project assignments for all future reviews:

| Review scope | Assigned model |
| --- | --- |
| Decision-trail and work-record audits, including `pstack:show-me-your-work` | `codex:gpt-6.1-terra@high` |
| Implementation correctness, test adequacy, architecture, design, security and other important technical reviews | `codex:gpt-6.1-sol@high` |

These assignments take precedence over skill defaults and any conflicting
requirement to choose a different model family. Preserve reviewer independence:
a change's author cannot approve their own work; use a separate review agent.
If an assigned model is unavailable, report that review as unavailable rather
than substituting another model. State each review's scope and assigned model
explicitly. Preserve historical reviews and their recorded reviewer identities.

## Product and source of truth

`pqc-scan` is a Rust CLI static analyzer of post-quantum cryptography migration
risks. Use the affected product specifications and the task's original acceptance
criteria; do not reinterpret incomplete work as complete.
Consult [architecture](docs/architecture.md), [best practices](docs/BEST-PRACTICES.md),
[patterns](docs/PATTERNS.md) and [known mistakes](docs/MISTAKES.md) for affected
components. End-user behavior is documented in `README.md`, `README.ja.md` and
`docs/user/`. Rules live in `rules/default/*.yml`; follow the affected entries and
[rule validation requirements](docs/rules-reference.md#validation-and-quality-review).

## Security and compatibility

- Validate inputs. Never expose secrets, private keys or sensitive cryptographic
  data in logs, diagnostics, snippets or reports; never weaken cryptographic checks.
- Keep scanning static: do not execute scanned files or resolve their dependencies.
  Preserve bounded reads, resource limits, path/symlink boundaries and redaction.
- Preserve CLI/API behavior, external schemas and formats, finding identity,
  ordering and reference integrity. An absent detection is not proof of safety;
  keep unknown, unsupported, failed, partial and complete inspection distinct.
- Obtain human review before high-risk architecture changes, cryptographic logic
  changes without a clear specification, potentially incompatible rule changes
  or storage-format changes. Pstack's design agreement does not waive these gates.

## Quality requirements

- Use `rust-toolchain.toml`. Apply the change-specific commands and evidence
  requirements in [verification](docs/agent/VERIFICATION.md), including optional
  features and relevant positive, negative and compatibility cases.
- For executable changes, require independent assessment of implementation
  correctness and test adequacy through pstack's review mechanisms. A change's
  author cannot independently approve that change. No fixed two-stage schedule,
  reviewer identity budget or legacy review template is imposed.
- Validate the final integrated source, not only isolated candidates. Record
  source revision and dirty changes, commands, configuration, exit results and
  evidence. Missing, failed or unavailable required validation keeps work unfinished.
- Preserve tests, quality thresholds, formal-verification assets, benchmarks,
  source-bound measurements and historical evidence. Do not overwrite prior runs.
  Follow [report requirements](docs/agent/REPORTS.md) for report content and browser
  acceptance; DOM tests do not replace required rendered observations.
- Isolate concurrent build/test outputs, especially Cargo feature matrices;
  follow the shared-target lesson in `docs/MISTAKES.md`.

## Documentation and existing records

Keep English/Japanese end-user documentation factually synchronized. Write
developer documentation in English, including task prose, without translating
historical records. Keep this override, active quality references and links aligned.
Existing `tasks/<ID>/task.yaml` files retain their original requirements and state;
`tasks/README.md` describes their storage format. Preserve plans, reviews and
`evidence/` in place. Do not rewrite history or automatically select old queue work.
New process artifacts and task decomposition follow pstack; no additional local
workflow or task record is required solely by this migration.
