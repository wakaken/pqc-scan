# Final product HTML browser verification

Source: ROLE-FIX-SOURCE.json (SHA-256
87be3b7b42902aba3445698501fd9a9eb5c22cf0676948ed603e9d76649ccef0).
The actual CLI generated28 paired report sets from14 synthetic cases (84 files),
recorded in GENERATED-FINAL.json. These are QA fixtures, not independent study data
or practitioner observations. No Markdown conversion is used for this acceptance.

The real in-app browser opened only product reports under the dedicated directory
/tmp/pqc-scan070-human-reports-q8d6yvn4 at127.0.0.1:58674. Repository/context/input
files were not served. Final URL:
http://127.0.0.1:58674/blocked/ja/migration.assessment.html.

BROWSER-OBSERVATIONS.json and browser/ contain28 final live-page observations,
accessibility snapshots and screenshots after the neutral role-wording fix. All
fit1280x900, with no broken internal anchors, remote assets, visible JSON dumps or
secret canary. Cases include urgent-ready/blocked, unknown, plan, conflicting/stale
claims, scoped exclusion, partial/failed TLS, multiple systems/roles, AES, hash and
escaped input. Failed verification never establishes successful key negotiation.

Actual controls: native summary Enter opens candidates; Tab/Enter opens evidence;
a source fragment link opens its enclosing details and scrolls to the edition.
English/Japanese switching preserves disclosure state. Dark/light styles apply to
cards. Narrow390x844 English/Japanese layouts have scrollWidth390 and wrap readably;
complete decisions naturally require vertical scrolling. Interaction screenshots
precede the neutral role-copy fix; layout/controller source is unchanged. Final
browser/ snapshots bind current copy. A role-based automation selector initially
failed to locate native summary; observed CSS #candidate-1 > summary plus native
Enter verified the control. This was a QA selector issue, not a product failure.

For loaded-offline acceptance the listener was stopped, connection refusal was
independently observed (connect_ex61), and the loaded page was not reloaded. Index
and candidate keyboard navigation, language/theme and disclosure still worked
(OFFLINE-SERVER.json, OFFLINE-BROWSER.json, offline-controls.jpg). The listener was
restarted for local review. A QA-only noscript-control.html removed the sole static
script from product HTML: initial Japanese content and native candidate expansion
remained readable, unavailable settings hidden. It is not an extra product output.

The viewport was reset; the final Japanese blocked report was left open and marked
as deliverable. BROWSER-FINAL.json records the resulting1090x1395 geometry;
final-report.jpg is the final screenshot. Earlier observations and
browser-before-role-refresh remain historical, not current-copy acceptance.
