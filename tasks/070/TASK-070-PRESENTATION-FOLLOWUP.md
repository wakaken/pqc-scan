# Human presentation follow-up

Status: done; both original reviewer identities accepted final source, all checks passed. The maintainer found the Markdown preview incomprehensible:
fields and raw candidate JSON did not explain the report, and requested styling
like the existing scan HTML plus a clear assess-generated HTML entry. Reopen070
X03; earlier passing technical/review evidence remains historical and does not
prove the new UX. The maintainer request explicitly selects HTML under D4.

The report supports an organization's migration response using saved observations
and scoped declarations; it does not rescan or verify deployment. Make that
purpose clear in the title/short introduction and lead with actual decisions.

## Agreed outcome and implementation

- `assess` emits migration.assessment.html plus existing JSON/Markdown, using
  --lang en|ja. HTML is a self-contained local report, reusable CSS and the existing
  language/theme controller; no new application, server integration or auto-open.
- Main view: response summary counts, a target index and readable decision cards
  with system/location, urgency, action, reason, blockers/missing information and
  next checks. Expandable evidence/claims/candidates use human tables and labels.
  Raw JSON dumps are removed from human output; the exact JSON remains linked.
- Both human formats share localized terms, typed claim-value rendering and stable
  references. Preserve independent axes, unknown/conflicting/stale labels, declared
  readiness, separate roles and all machine data/IDs. Never evaluate policy here.
- Stream bounded HTML including both languages before any writes, preflight all
  three output aliases, reuse safe_output and preserve individual-file atomicity.
  Legacy scan HTML/rules/serve remain unchanged. HTML storage addition is explicitly
  authorized by this request; no JSON/context/rule/crypto/model changes are selected.

Alternatives: a cosmetic stylesheet over the old JSON-heavy page would preserve
its comprehension problem; retaining a QA-only preview would not deliver the
requested command output. Use a real HTML writer and readable Markdown instead.

## Validation and completion

Check paired semantics, escaping/canaries/no remote resources, links, all three
aliases, library JSON-only compatibility and actual CLI output/default language.
Run required default/full/affected-feature checks. In the actual browser inspect
mixed-system and blocked/unknown/stale/partial roles, first screen, narrow layout,
keyboard, evidence disclosure, language/theme state and loaded-offline behavior.
Use a dedicated loopback directory containing only final product outputs, without
Markdown-to-HTML QA conversion. Reuse the original two reviewer identities.
Record final source, commands, screenshots, review/fix evidence and direct output
instructions. 071 remains blocked on actual resources; no useful-study, score,
release or public-hosting claim follows from this UI repair.
