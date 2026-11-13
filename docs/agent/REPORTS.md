# Generated Reports and Browser Verification

These are mandatory details of [AGENTS.md](../../AGENTS.md). Read before changing
report content, presentation or interaction, or performing report browser
acceptance checks. Also read the affected report/user/design documentation.

# User-Facing Report Content

- Design generated HTML/Markdown reports for people investigating their project:
  prioritize what was found, where it was found and the next useful check.
- Keep the opening focused on results. Do not place internal aggregation rules,
  implementation terminology, generic guarantee disclaimers or obvious interface
  instructions ahead of the findings.
- Explain incomplete inspection in the inspection overview using concrete states,
  affected files and reasons. Do not repeat a generic warning above that overview.
- Keep technical definitions, counting conventions and detailed limitations in
  documentation, diagnostics or machine-readable exports as appropriate.
- Review changed report copy in the rendered report, including its first screen.
  Prefer short, natural wording that helps the reader make a decision.

# Local Report Browser Verification

When report presentation, browser interaction or report acceptance requires
browser checks:

- Use a temporary HTTP server bound to `127.0.0.1`.
- Do not repeatedly attempt unsupported `file://` navigation.
- Serve a dedicated directory containing only the intended generated reports.
- Never serve the repository root, home directory or private production inputs.
- Record actual browser and viewport observations and captures.
- Verify relevant layout, keyboard behavior and loaded-offline behavior.
- Source or DOM unit tests do not substitute for required browser observations.
- Preserve the user's visible report tab and report the loopback URL.
- Keep or stop the dedicated server according to the requested viewing workflow.
- If required browser checks are unavailable, record them as unexecuted and
  keep the affected acceptance criteria unfinished.
