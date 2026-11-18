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
