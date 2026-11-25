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
|observation-v1-dfa6d8294e97dd9041f126fb9211ad4de8dcabf1032d832042b22187f15572ca|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-4852f8bf0084f287c923b0c7ed8049b19499d136d33eefbbb0d8c2a17a225604|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-681981a63ef326196a3e8594b1a98098a4d8b9b2316371d6f7e73082c20da05d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-412e76af1937e7add5bca7bf187b0bb0272db70f4c14dff21f0d1af6bff89947|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-b6d4205b9b595d66ea81cb2123a2d1224e845cc62fef8088ef8aed1ae234424d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-12635aadb6628706b4c50e2f9c832928303100f387afaa92960038f9d1ba7da6|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-179bc3f3596d1b1f119a3ffab65f423a77d4f431c3df99dbad2fb369a9af9fbf|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-6f9e8a0b06d613265802922d6d0cd8a130d7ffc5c943aaaf7efecb0e4c81af63|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
