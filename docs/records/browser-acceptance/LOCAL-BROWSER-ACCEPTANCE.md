# Local report browser acceptance

Use a loopback HTTP server for generated reports. The maintainer explicitly
selected this method on2026-11-11 after file-scheme navigation was rejected.
HTTP navigation and captures were permitted by the browser tool; the unsupported
file-scheme policy was not changed. This replaces the previous unestablished
browser gate with new observed evidence, not a retroactive claim about old runs.

Stage only the intended synthetic report HTML files in `target/browser-reports`,
then run:

```sh
python3 -m http.server 8765 --bind 127.0.0.1 --directory target/browser-reports
```

Do not serve the checkout/home directory or private product scans. All current
HTML styles/scripts are inline. Keep the generated input bytes/hashes alongside
the captures so later builds are not mistaken for the pages actually observed.

## Static English/Japanese report observations

Current captures and structured actions are under
`target/continuation-20261111/browser/`. Browser: ChatGPT in-app browser; version
not exposed by the supported read-only API. Default DOM viewport1304×1248; requested
narrow390×844 produced an observed document width354. Both languages had no
horizontal document overflow in the measured narrow expanded-finding state.

Observed: coverage precedes findings; no advisory-policy/observation-ID table;
search JWT_RS256 yields1 group/22 locations; Tab moves from search to severity;
Enter opens a rule and a location. High-severity filtering then yields0 matches
and a visible empty-state explanation. Reset and next-page controls work; inventory
mode gives8 groups/15 findings; vulnerable-risk filtering gives38 groups/173
findings. All counters remain relative to83 groups/294 total findings.

Both language documents were loaded, the dedicated HTTP server was stopped, and
search/native disclosure controls continued working without reloading. The server
was then restarted for the user. This is loaded-document offline behavior, not an
OS-wide network-disconnect test. No external script/stylesheet/image dependencies
were present in the observed DOM. Captures record actual pixels, not mocked DOMs.

## Endpoint reports

`tests/protocol/report-previews.py` creates successful verified fresh TLS1.3 and
unauthenticated SSH-advertisement examples, plus denied/no-traffic examples, in
both languages. Run under the same loopback-only sandbox as protocol tests. Test
PKI keys exist only in a removed OS temporary directory, never the served/output
root. SSH records client packet types[20] and zero subsequent bytes. Final browser
acceptance must bind the actual generated successful and denied pages and exercise
closed technical details/keyboard/narrow layout; generation alone is not acceptance.

## Completed endpoint observations

Eight successful/denied TLS/SSH pages in both languages were observed over the
loopback server. Initial technical sections were closed; Tab focused their native
summary and Enter opened them. Requested1280×1000 desktop sizing produced an
observed document width1163, while requested390×844 narrow sizing produced354.
No document-level horizontal overflow appeared with the complete technical table
open. The actual TLS fixture selected TLS1.3 / TLS13_AES_256_GCM_SHA384 /
X25519MLKEM768, ordinarily validated against the ephemeral local CA. SSH remained
explicitly unauthenticated advertisements, with the fixture's packet record[20].
The four successful pages were then loaded, the HTTP server stopped, and technical
details continued to open in both languages; the dedicated server was restarted.

Bindings: `target/continuation-20261111/browser/runtime-page-bindings.json`;
observations/text: `runtime-checks.json`; screenshots: `tls-ja-desktop.jpg`,
`ssh-ja-desktop.jpg`, `ja-desktop.jpg`, `en-desktop.jpg` and narrow captures in
that directory. These observations qualify this in-app browser and measured
viewports, not every physical mobile device/browser. Browser version could not
be obtained from the supported API and remains explicitly unknown.
