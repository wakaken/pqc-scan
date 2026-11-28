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
|observation-v1-c7a1c856226fcf76fedde1e6c721fa5fe287b73267fed6c69035baa08f00e8bc|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-aac8898ffcd5509624842b6a2d8ee9055cf150878e461e9ea79824e4f547d7d7|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-82fdf9242effd6332008149123c35a53ef40ae329a655a0ee8e3b2a177e7b980|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-190ef90ee23ffd2b140058fc17809bdd1e11e7ebbcaebb3554b426b3a38ea6d2|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bd2a33974bd654fbf26230056fdc64b3eb3ba5dc9c9e5fb4c12bd396aa86f64d|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-edb5f7219d9b66a0d07b3ed7704d77d886b3c4d766e4371181846dcc3e1bf566|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-554ae4f56f4a00edbf2d92d18e4cc2d2ac70a5dd715dbe1959e20632b7c64d29|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
|observation-v1-bd62be7fc0535e7097d274a0e143c397f3ff948b79481b5ee0d6b682429c4935|Assurance: Syntactic pattern (syntactic); Usage: Referenced (referenced)|
