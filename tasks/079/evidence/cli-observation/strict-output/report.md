# PQC Migration Scan Report

Generated: 2026-11-24 15:17:52 +09:00 (UTC+09:00)

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
|observation-v1-054d23a7ad859591a283b4910ed94c6adbb548cad51735ebf3c8bb07eb92b348|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6cf4294c976f03d2f809fab2238be0c6623408c9c8a20faf620d0a0639671f56|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d524ccb86d28b255ac3085d1ee56a65717f0985b5ac92881bd57592d7e1b89d6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-0969829cfe2c4e8fb7326e8cec9d46c3bf1a2d0bbeda8b34891685c09d8cbac5|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-200b27d3874d7d2a6c757066c5df6e203eadab371f5a41ed79766879b042b9ea|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4a6f6938deded3ca623bb5627a813750e8c5727aad5da0d0e05a6024817dc99c|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-1d0f294b8f2a5eb01c9e70e4f6f4788be8720ca10c167ba5971d251644d93a9a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6598c5e3c83823e03c0cd250b318e8dbfd20699e75cfb20ed88e460649cdfbe0|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
