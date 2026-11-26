# Rendered HTML report

## Sub-features

Inspection overview, finding details and source evidence, candidate/inventory/all views, search and filters, reset, file tree, rule explanations, language selection, pagination, keyboard focus, offline reading, and mobile layout.

## How to get to it (user POV)

Generate `--format all`, then open `report.html`. The Smoke run produces English and Japanese directories. Serve only one generated reports directory through a temporary HTTP server bound to `127.0.0.1`. For example, after setting `VERIFY_REPORT_DIR` to the absolute `reports/en` path, run `python3 -m http.server 8765 --bind 127.0.0.1 --directory "$VERIFY_REPORT_DIR"`. Retain the process handle and open `http://127.0.0.1:8765/report.html`. Choose another loopback port if occupied; never stop its owner.

## Driving it with the browser

Use actual browser automation and visible interactions. Select the active visible `[data-compact-report]`; both language variants exist, so do not blindly use the first global match. Set `[data-mode]` to `all` before assertions that include inventory items. Open the search conditions `<details>` when needed.

1. Capture the first screen and inspection overview. Open a visible `[data-finding-id] > summary` and confirm `app.rs:1`, the `RS256` evidence, and suggested follow-up are visible.
2. Fill the controls' `input[data-search]` with `app.rs` and observe matching rows and `[data-report-status]`. Search for an impossible synthetic term and observe `[data-empty]`. Click `[data-reset]` and confirm rows return.
3. Exercise `[data-severity]`, `[data-risk]`, `[data-rule-filter]`, `[data-file-filter]`, and `[data-show-tests]` with a fixture containing their required populations. Select a file through `[data-tree-file]`, then clear it. The tiny Smoke fixture does not prove test-code filtering or pagination.
4. Follow `[data-rule-help]`. Confirm `[data-rule-dialog]` opens with the matched rule explanation. Use `[data-close-rule-dialog]` and Escape, and inspect focus return. Open `report.rules.html` and follow its return link.
5. Use the rendered language control or the Japanese report. Compare observed file locations, IDs, and counts. Exercise keyboard Tab, Enter, and Escape. Record visible focus and any trap.
6. Observe desktop and mobile viewports. Capture overflow, reading order, detail expansion, and controls. Block external network requests or disconnect networking while keeping loopback available, reload, and repeat a search/detail journey. Record the observed request behavior and successful local rendering.

Capture action and resulting state, screenshots, browser/version, viewport, and URL in a browser receipt under the run. Run `node --test shell/compact-report.test.cjs` as an additional automated behavior check. It does not replace these rendered observations.

## Gotchas

Do not serve the repository, home directory, or private inputs. Do not repeatedly try unsupported `file://` navigation. Required unavailable browser checks remain unexecuted. Use fixtures with more than 20 groups or occurrences before claiming pagination. Preserve the visible report tab and report its loopback URL. Stop only the server you started, according to the requested viewing workflow. Keep screenshots and logs after teardown.
