# Context-input screenshots — 2026-11-24

The illustrated scanning guides now use `web-04-purpose-clock-*`,
`web-05-validity-clock-*` and `web-08-save-status-*`, plus
`web-evaluation-time-*` for the fixed-time warning. These are fresh Chrome
captures of the same public Python example and six selected source records.
The default creation-time evaluation, explicit historical time and JSON saving
status are visible in the corresponding steps.
[CAPTURES-context-20261124.json](v0.4/CAPTURES-context-20261124.json) records
source assets, reviewed patch, binary, inputs, demonstration dates and image hashes.
The screenshots are unedited and use actual focus/scroll positions at the existing
browser viewport. Earlier images and their metadata below remain unchanged.

# Current v0.4 screenshots — 2026-11-23

The current README and illustrated user guides use the unedited captures under
`v0.4/`. [CAPTURES.json](v0.4/CAPTURES.json) binds each JPEG, dimensions, scanner
source/binary, inputs, dates and generation conditions. The browser viewport was
1280×720 CSS pixels. The folder picker is a full-page capture; purpose, validity,
location and saving steps use the actual scrolled viewport. Overview/plan page
clips include the documented content below the fold.
No image was resized, annotated or generated. Language/theme choices are actual
product controls.

- `scan-{light,dark}-{en,ja}.jpg`: a fresh scan of the same clean Keycloak revision
  used below, using v0.4.0 at d3b6d769. Human totals are2301 projected findings,
  with2283 review candidates and18 reference entries. Tests remain included.
  Old and new human totals use different product behavior and counting, so the
  difference is not an accuracy or risk-reduction measurement.
- `web-01` through `web-08`: actual local-web operations on copies of public
  `examples/go` and `examples/python`. The python walkthrough declares JWT/active
  for6 source records with explicit example provenance/expiry. It still produces
  investigations with undetermined urgency; adding input does not imply readiness.
- `migration-plan-{en,ja}.jpg`: the current binary's fixed-time Kubernetes example
  with an explicit JWT context and schedule. Its three phases are conditional;
  the dates and claims are demonstration inputs, not a universal migration date.

The captures contain no browser chrome, random session URLs, user home path or
private-key payload. Counts are snapshot-specific references, not confirmed
vulnerabilities or production-use counts. Earlier captures below remain unchanged
as historical evidence and are no longer the README previews.

## Historical captures — 2026-11-18

# Report screenshots

These are unedited Chrome page captures taken on 2026-11-18 at a 1484 × 1073
CSS-pixel viewport. English/Japanese and light/dark variants show the same
Keycloak scan with the file tree visible and the filter panel hidden. The default
risk-candidate view shows 4,058 of 4,072 findings; 14 are inventory findings.
The scan selected 10,452 files, fully analyzed 10,377, partially inspected 17,
and found no applicable inspection for 58. Three entries could not be enumerated.
Counts include test fixtures and do not represent confirmed vulnerabilities.

- Scanner source: `003cc0c3d9d8124f17de19b2fb8a6cad595b56f6` (report/server product code is unchanged at `11ff0b0c3e6f8c9a17148fe0551256f29dee74f0`).
- Keycloak source: `18559075f50c8934d315035aea0c9a96314d37a9`, clean checkout.
- Invocation: `pqc-scan serve --root /path/to/projects --rules-dir /path/to/pqc-scan/rules --port 8789 --threads 4 --no-open`; select the Keycloak folder.
- Time display: `Asia/Tokyo`. The reported generation time is 2026-11-18 10:35 JST.
- Generated HTML SHA-256: `32e4fea57805874b70100d48801f1750b7d1b8a712d9ba39126e2e9ad850db42`.
- No browser chrome, session URL, local absolute source path or private key contents are included in the captures.

- `report-dark-en.jpg`: SHA-256 `66456e32bb6b93d88569aceb19e2372e900c921d052ee9458c164a572ed9fdfd`.
- `report-dark-ja.jpg`: SHA-256 `ce78fd4b7a40fa50bec63018501dd09543767fb3a6eb6081e49a4ed2bfba8af7`.
- `report-light-en.jpg`: SHA-256 `665ac5b3572602dabf2ed11872a263a139071bc5a57b823b1ade1b722bc4d361`.
- `report-light-ja.jpg`: SHA-256 `493ad1439edb3d90cde1c97a655579e7522ae0bac7d6dc763596ac99349ca8dd`.

## README branding

`pqc-scan-banner.webp` is the original, unmodified banner restored from commit
`00854e1`. Its SHA-256 is
`353465643131e70951c87345f5729f6391726313c8b76ebad9d9b8f78dc40a73`.
Both README languages use this asset. The release badge follows the latest
published GitHub Release; it is not a hard-coded candidate version or publication
claim. The report captures above remain unchanged.
