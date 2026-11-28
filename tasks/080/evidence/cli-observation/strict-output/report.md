# PQC Migration Scan Report

Generated: 2026-11-24 16:29:58 +09:00 (UTC+09:00)

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
|observation-v1-21e006ea17788b9c7e067d5cbc95c9ec5ffb27dc712461cdcccebcc8e1123816|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4c2055a8d5235940a93b276f5bcee04ca31e8d9631821e4a57cc7a931e5da54a|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-ddbb793fe3569f4d807e668ea58ff4e1fd3945d6bf71670dd372e6dbd56f2214|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-20e6edf837fe5851c781f1f36c489b2afd59ba7f229c79fe85dfbad80f6c94f4|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-782c53a8b58fb0e2fe90a024c7f7f59a61f38fbaabe31bfce41efc63179d4c73|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b11983a645bffe116eacbb4513346d3c5096f3eef1c5f014fa43df85468ce965|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-73b9be9735ab502c07b903d5eded1884c822d42b467b4a455b448886447428de|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-d61af12d8069998baf4809c2b2c606e77210fab097501e4238ec88699e64f5c7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
