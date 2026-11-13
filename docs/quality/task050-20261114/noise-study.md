# Noise study

6 studies, each 24 successful scans: 3 scales x 2 thread counts x (1 first-measured + 3 warm). Raw samples and warm min/median/max are retained. First-measured is not true cold: no cache flush, input materialization already touches data. No privileged cache mutation. Linux quota vs native macOS is an environment comparison, not product speed ranking.

Historical different-day samples are retained with dates, hashes and explicit comparison rejection reasons in environment-comparison.json. There are no controlled same-condition multi-day samples, so no isolated daily-noise estimate or cold-cache performance claim. Existing historical input/config/source/environment differences prevent a retrospective regression verdict.

## Measured large-workload noise

Wall seconds; each warm distribution has n=3. Other scale/CPU/RSS samples remain in resources.json.

|Study|Threads|First measured|Warm min|Warm median|Warm max|
|---|---:|---:|---:|---:|---:|
|target/quality-task-050/host-default-final-2|1|4.648068|4.241929|4.242163|4.329014|
|target/quality-task-050/host-default-final-2|2|2.846668|2.836993|2.849999|2.884985|
|target/quality-task-050/host-default-final-3|1|4.418299|4.245891|4.255643|4.269933|
|target/quality-task-050/host-default-final-3|2|2.833126|2.861319|2.898550|3.123549|
|target/quality-task-050/confined-final/default-1|1|4.189470|4.166709|4.196311|4.262829|
|target/quality-task-050/confined-final/default-1|2|2.900346|2.917162|2.972783|3.083023|
|target/quality-task-050/confined-final/default-2|1|4.177333|4.171496|4.180308|4.189348|
|target/quality-task-050/confined-final/default-2|2|2.941220|2.927517|2.989453|2.993429|
|target/quality-task-050/confined-final/all-features-1|1|4.238736|4.194926|4.219062|4.321360|
|target/quality-task-050/confined-final/all-features-1|2|2.959777|2.932822|2.981711|2.989972|
|target/quality-task-050/confined-final/all-features-2|1|4.199443|4.206289|4.234490|4.257107|
|target/quality-task-050/confined-final/all-features-2|2|2.943190|2.916033|2.949363|2.954776|
