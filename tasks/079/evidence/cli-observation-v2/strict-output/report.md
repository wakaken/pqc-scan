# PQC Migration Scan Report

Generated: 2026-11-24 14:41:49 +09:00 (UTC+09:00)

Findings to review (items): 0 / Informational findings (items): 8 / Files inspected: 7

|Severity|Count (all findings)|
|---|---:|
|info|8|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[FOUNDATION_CONTROL](#rule-0) Rule FOUNDATION_CONTROL matched a quantum-relevant crypto usage|info|quantum-uncertain|8|7|

<a id="rule-0"></a>

## FOUNDATION_CONTROL

Rule FOUNDATION_CONTROL matched a quantum-relevant crypto usage

Check: Review

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[a.c:1:5](#finding-0)|CONTROL|Review / [Source](#source-0)|
|<a id="finding-1"></a>[a.cs:1:30](#finding-1)|CONTROL|Review / [Source](#source-1)|
|<a id="finding-2"></a>[a.h:1:5](#finding-2)|CONTROL|Review / [Source](#source-2)|
|<a id="finding-3"></a>[a.phtml:1:11](#finding-3)|CONTROL|Review / [Source](#source-3)|
|<a id="finding-4"></a>[a.phtml:1:23](#finding-4)|CONTROL|Review / [Source](#source-4)|
|<a id="finding-5"></a>[b.blade.php:1:17](#finding-5)|CONTROL|Review / [Source](#source-5)|
|<a id="finding-6"></a>[encoded.php:1:18](#finding-6)|CONTROL|Review / [Source](#source-6)|
|<a id="finding-7"></a>[upper.C:1:21](#finding-7)|CONTROL|Review / [Source](#source-7)|

### a.c:1:5 — Matched rules

- `FOUNDATION_CONTROL` (info): Rule FOUNDATION_CONTROL matched a quantum-relevant crypto usage — Review

<a id="source-0"></a>

### a.c:1:5

    > 1 int CONTROL;

### a.cs:1:30 — Matched rules

- `FOUNDATION_CONTROL` (info): Rule FOUNDATION_CONTROL matched a quantum-relevant crypto usage — Review

<a id="source-1"></a>

### a.cs:1:30

    > 1 namespace App; class C { int CONTROL; }

### a.h:1:5 — Matched rules

- `FOUNDATION_CONTROL` (info): Rule FOUNDATION_CONTROL matched a quantum-relevant crypto usage — Review

<a id="source-2"></a>

### a.h:1:5

    > 1 int CONTROL;

### a.phtml:1:11 — Matched rules

- `FOUNDATION_CONTROL` (info): Rule FOUNDATION_CONTROL matched a quantum-relevant crypto usage — Review

<a id="source-3"></a>

### a.phtml:1:11

    > 1 日本語<?php $CONTROL=1; $CONTROL=2; ?>

### a.phtml:1:23 — Matched rules

- `FOUNDATION_CONTROL` (info): Rule FOUNDATION_CONTROL matched a quantum-relevant crypto usage — Review

<a id="source-4"></a>

### a.phtml:1:23

    > 1 日本語<?php $CONTROL=1; $CONTROL=2; ?>

### b.blade.php:1:17 — Matched rules

- `FOUNDATION_CONTROL` (info): Rule FOUNDATION_CONTROL matched a quantum-relevant crypto usage — Review

<a id="source-5"></a>

### b.blade.php:1:17

    > 1 @if($ok) <?php $CONTROL=1; ?> @endif

### encoded.php:1:18 — Matched rules

- `FOUNDATION_CONTROL` (info): Rule FOUNDATION_CONTROL matched a quantum-relevant crypto usage — Review

<a id="source-6"></a>

### encoded.php:1:18

    > 1 <?php /* 日本語 */ $CONTROL=1;

### upper.C:1:21 — Matched rules

- `FOUNDATION_CONTROL` (info): Rule FOUNDATION_CONTROL matched a quantum-relevant crypto usage — Review

<a id="source-7"></a>

### upper.C:1:21

    > 1 namespace App { int CONTROL; }

## Action details

Each location links only to its applicable action. Identical payloads are printed once.


## Inventory exports

- [CBOM](cbom.json): 7
- [Dependency SBOM](dependency-sbom.json): 0


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Some inspection could not complete (partial)|
|Selected files|7|
|Read successfully|7|
|Read failed|0|
|Analyzed|6|
|Partial|1|
|Analysis failed|0|
|Unsupported|0|
|Not applicable|0|
|Unknown|0|
|Enumerated exclusions|0|
|Enumeration failures|0|
|Unenumerated population|Unknown|

## Inspection diagnostics

Counts and up to three example paths per reason; full records remain in diagnostics JSON.

|Item|Value|
|---|---|
|Only part of inspection completed (detector_partial): Unrendered template: static source inspected; generated values and branches remain unknown.|1 records; b.blade.php|

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-892111702f84fe77160d151f0309d35beb31232fdae8ba7493c364ae1d546cf4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-60ac4f7fe6816ad07b542d76eff231a6cec61d846e3478c54d7b54e1da718849|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-014c23f6c424b67e7de58def99465b7377bae2876671583d2b521f7bb1e4b613|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-85797a748f1632c49dde4c3f8aa09a36282eb50fe93e12ad092772779408f906|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-710e25dd4748c2560c81dcc907ab75c0cce7a8398aff9b03bd375f952db83131|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6a51579c594e114ba1bb140e44c12880c044d34c9085bc5c74fc4802f9a229cd|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-573bd2c5b9f616e8d74a9af9b416c2e34bcb00e7ab9b80ca5c8ecb4e2748884c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0e87538a9ed9b7a1aea5f24b69095922cf59c4e8d89e4606eb797529adec600e|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
