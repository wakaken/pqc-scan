# Noise study

4 studies, each 24 successful scans: 3 scales x 2 thread counts x (1 first-measured + 3 warm). Raw samples and warm min/median/max are retained. First-measured is not true cold: no cache flush, input materialization already touches data. No privileged cache mutation. Linux quota vs native macOS is an environment comparison, not product speed ranking.

Historical different-day samples are retained with dates, hashes and explicit comparison rejection reasons in environment-comparison.json. There are no controlled same-condition multi-day samples, so no isolated daily-noise estimate or cold-cache performance claim. Existing historical input/config/source/environment differences prevent a retrospective regression verdict.
