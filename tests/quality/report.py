#!/usr/bin/env python3
"""Summarize existing TASK-023 evidence offline; never execute corpus code."""
import argparse
import collections
import hashlib
import json
import pathlib
import sys

from evaluate import (bytes_digest, check_coverage, digest, load_truth, order_digest,
                      safe_relative, score, semantic_digest, validate_corpus)
from run import distributions

REPO = pathlib.Path(__file__).resolve().parents[2]
COUNTS = ('tp', 'fp', 'fn', 'tn', 'positive', 'negative', 'ambiguous',
          'unsupported', 'excluded_predictions')


def ratio(numerator, denominator):
    return numerator / denominator if denominator else None


def metrics(group):
    """FP is a prediction count; FPR uses distinct negative truth units."""
    m = {k: group[k] for k in COUNTS}
    m.update(eligible=m['positive'] + m['negative'],
             precision_denominator=m['tp'] + m['fp'],
             recall_denominator=m['positive'],
             f1_denominator=2*m['tp'] + m['fp'] + m['fn'],
             false_positive_units=m['negative'] - m['tn'],
             fpr_denominator=m['negative'], fnr_denominator=m['positive'])
    m.update(precision=ratio(m['tp'], m['precision_denominator']),
             recall=ratio(m['tp'], m['recall_denominator']),
             f1=ratio(2*m['tp'], m['f1_denominator']),
             fpr=ratio(m['false_positive_units'], m['fpr_denominator']),
             fnr=ratio(m['fn'], m['fnr_denominator']))
    return m


def compare_accuracy(current, previous):
    same = (all(current[k] == previous[k] for k in ('corpusSha256', 'truthSha256'))
            and current['corpusInput']['sha256'] == previous['corpusInput']['sha256']
            and current['accuracy']['groups'].keys() == previous['accuracy']['groups'].keys())
    if not same:
        return dict(status='Not Applicable', reason='Different truth, corpus, input or evaluated groups')
    deltas = {key: {k: value[k] - previous['accuracy']['groups'][key][k] for k in ('tp', 'fp', 'fn')}
              for key, value in current['accuracy']['groups'].items()}
    bad = [key for key, d in deltas.items() if d['tp'] < 0 or d['fp'] > 0 or d['fn'] > 0]
    return dict(status='Failed' if bad else 'Passed', deltas=deltas, regressedGroups=bad,
                policy='TASK-023 approved split/family nonregression; descriptive report, no numeric CI enforcement')


def stratify(data, labels, findings, file_values=None, rule_values=None):
    """Partition inputs then call the authoritative matcher; no new matching logic."""
    values = sorted(set((file_values or rule_values).values()))
    rows = {}
    for value in values:
        files = [f for f in data['files'] if file_values is None or file_values[f['path']] == value]
        paths = {f['path'] for f in files}
        families = {family: [r for r in rules if rule_values is None or rule_values[r] == value]
                    for family, rules in data['families'].items()}
        families = {family: rules for family, rules in families.items() if rules}
        rules = {r for ids in families.values() for r in ids}
        units = [u for u in data['units'] if u['path'] in paths and u['ruleId'] in rules]
        selected = dict(data, files=files, families=families, units=units)
        result = score(selected, {u['id']: labels[u['id']] for u in units},
                       [f for f in findings if f['location']['file'] in paths and f['rule_id'] in rules])
        totals = {}
        for key, group in result['groups'].items():
            split = key.split('/')[0]
            counts = totals.setdefault(split, collections.Counter())
            counts.update({k: group[k] for k in COUNTS})
        for split, group in totals.items():
            rows[f'{split}/{value}'] = metrics(group)
    return rows


def load_json(path):
    return json.loads(path.read_text())


def runtime_bindings(repo):
    """Mirror the existing runner's declared source population, including test code."""
    files = [repo/name for name in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml')]
    for root, patterns in [('crates', ('*.rs', '*.js', '*.css', 'Cargo.toml')), ('rules', ('*.yml',))]:
        for pattern in patterns:
            files.extend((repo/root).rglob(pattern))
    return {str(f.relative_to(repo)): bytes_digest(f) for f in sorted(set(files))}


def corpus_input(data, repo, copies):
    bindings = {}
    for index in range(copies):
        for file in data['files']:
            name = file['path'] if copies == 1 else f"copy-{index:03}/" + file['path']
            bindings[name] = dict(sha256=file['sha256'], bytes=(repo/file['source']).stat().st_size)
    return dict(files=len(bindings), bytes=sum(f['bytes'] for f in bindings.values()),
                sha256=digest(bindings), bindings=bindings)


def benchmark_input(name, data, repo):
    copies = {'small': 1, 'medium': 8, 'large': 32}
    if name in copies:
        count = copies[name]
        return corpus_input(data, repo, count), f'{count} deterministic complete corpus copies; not accuracy truth'
    if name != 'pathological':
        raise ValueError('Unknown benchmark recipe')
    content = ('const data = "' + 'RS256 ' * 10000 + '";\n').encode()
    binding = {'repeated.js': dict(sha256=hashlib.sha256(content).hexdigest(), bytes=len(content))}
    return (dict(files=1, bytes=len(content), sha256=digest(binding)),
            'one JS string with 10000 repeated RSA JWT tokens, intentionally bounded/truncated')


def supported_scope(path, repo, directory, data, truth, findings, language_by_file, kinds):
    """Apply separately reviewed applicability; retain original object truth/results."""
    from taxonomy import aggregate, utc_time
    path = path if path.is_absolute() else repo/path
    if path.resolve() != path or not path.is_relative_to(repo/'tests/quality'):
        raise ValueError('Applicability must be under tests/quality')
    manifest = load_json(path)
    def bound(root, name, expected):
        p = root/safe_relative(name)
        if p.resolve() != p or bytes_digest(p) != expected:
            raise ValueError('Applicability source/review binding changed')
        return p
    if (manifest.get('version') != 1 or manifest.get('scope') != 'documented-public-input-subset'
            or manifest.get('corpusSha256') != bytes_digest(directory/'corpus.json')
            or manifest.get('truthSha256') != bytes_digest(directory/'truth.json')
            or manifest.get('postObservation') is not True or manifest.get('sourceBased') is not True
            or not manifest.get('contractBindings') or len(manifest.get('reviewSources', {})) != 2):
        raise ValueError('Missing bound applicability assessment')
    frozen_at = utc_time(manifest.get('timestampUtc'))
    bound(path.parent, 'PROTOCOL.md', manifest['protocolSha256'])
    for name, expected in manifest['contractBindings'].items():
        bound(repo, name, expected)
    original = {r['id']: r for r in truth['labels']}
    decisions = manifest['dispositions']
    ids = [d['id'] for d in decisions]
    if (not ids or len(set(ids)) != len(ids) or not set(ids).issubset(original)
            or any(d.get('status') != 'unsupported' or not d.get('reason', '').strip() for d in decisions)):
        raise ValueError('Invalid unsupported applicability decisions')
    reviewers = set()
    for name, expected in manifest['reviewSources'].items():
        review = load_json(bound(path.parent, name, expected))
        for key in ['corpusSha256', 'truthSha256', 'protocolSha256', 'contractBindings']:
            if review.get(key) != manifest[key]:
                raise ValueError('Applicability review disagrees with source bindings')
        if (review.get('sourceBased') is not True or review.get('postObservation') is not True
                or not isinstance(review.get('historicalMeasurementsSeen'), bool)
                or review.get('otherApplicabilityReviewSeen') is not False
                or review.get('remainingUnitsReviewed') is not True or review.get('unresolved') != []
                or utc_time(review.get('timestampUtc')) > frozen_at
                or {d['id']: d['status'] for d in review['dispositions']} != {d['id']: d['status'] for d in decisions}
                or len(review['dispositions']) != len(ids)
                or any(not d.get('reason', '').strip() for d in review['dispositions'])
                or not review.get('reviewer', '').strip()):
            raise ValueError('Incomplete or inconsistent applicability review')
        reviewers.add(review['reviewer'])
    if len(reviewers) != 2:
        raise ValueError('Two distinct applicability reviewers required')
    scoped_truth = dict(truth, labels=[dict(row, label='unsupported') if row['id'] in ids else dict(row)
                                      for row in truth['labels']])
    labels = {r['id']: r['label'] for r in scoped_truth['labels']}
    scored = score(data, labels, findings)
    result = dict(status='Verified', scope=manifest['scope'], postObservation=True,
        interpretation='Supported-input applicability correction, not detector improvement or a new independent evaluation',
        applicabilitySha256=bytes_digest(path), sourceBindings=manifest,
        excludedUnits=[dict(d, originalLabel=original[d['id']]['label']) for d in decisions],
        bySplitFamily={k: metrics(v) for k, v in scored['groups'].items()},
        byRule=stratify(data, labels, findings, rule_values={r: r for r in kinds}),
        byDetector=stratify(data, labels, findings, rule_values=kinds),
        byLanguage=stratify(data, labels, findings, file_values=language_by_file),
        byInputFormat=stratify(data, labels, findings, file_values={f['path']: f['inputFormat'] for f in data['files']}),
        errorDetails=[d for d in scored['details'] if d['disposition'] in ('fp', 'fn', 'excluded_prediction')])
    for field, key in [('algorithms', 'byAlgorithm'), ('algorithmParameters', 'byAlgorithmParameter'),
                       ('usage', 'byUsage'), ('artifact', 'byArtifact'), ('caseKind', 'byCaseKind')]:
        result[key] = aggregate(data, scoped_truth, scored, field, metrics)
    return result


def build_assessment(baseline, repo=REPO, reference=None, corpus_dir=None, applicability=None):
    from taxonomy import select_corpus
    corpus_directory = select_corpus(repo, corpus_dir)
    data, labels = load_truth(corpus_directory)
    validate_corpus(data, repo)
    summary = load_json(baseline / 'summary.json')
    if (summary['corpusSha256'] != bytes_digest(corpus_directory/'corpus.json')
            or summary['truthSha256'] != bytes_digest(corpus_directory/'truth.json')
            or summary['corpusVersion'] != data['corpusVersion']):
        raise ValueError('Baseline does not bind the current truth/corpus')
    if summary.get('corpusDirectory', 'tests/quality') != str(corpus_directory.relative_to(repo)):
        raise ValueError('Baseline corpus directory differs from selected corpus')
    environment = summary['environment']
    expected_runtime = runtime_bindings(repo)
    if (environment.get('runtimeSourceBindings') != expected_runtime
            or environment.get('runtimeSourceDigest') != digest(expected_runtime)):
        raise ValueError('Measured runtime source population/digest differs from checkout')
    harness = environment.get('harnessBindings', {})
    if environment.get('harnessDigest') != digest(harness):
        raise ValueError('Measurement harness digest differs from bindings')
    required_harness = ['tests/quality/evaluate.py', 'tests/quality/run.py']
    if data.get('annotationVersion'):
        required_harness += ['tests/quality/taxonomy.py']
    for name in required_harness:
        if harness.get(name) != bytes_digest(repo/name):
            raise ValueError('Missing or changed measurement evaluator/runner binding')
    if summary['corpusInput'] != corpus_input(data, repo, 1):
        raise ValueError('Measured input binding differs from corpus')
    repeats = summary['repeatability']
    if repeats['repeats'] < 2 or repeats['repeats'] != len(repeats['samples']):
        raise ValueError('Missing repeated measurements')
    reports, diagnostic_digests, order_digests, bindings = [], [], [], {}
    for index, sample in enumerate(repeats['samples']):
        directory = baseline / f'accuracy-{index}'
        raw = load_json(directory/'report.json')
        diagnostics = load_json(directory/'diagnostics.json')
        actual_sample = load_json(directory/'measurement.json')
        for path in (directory/'report.json', directory/'diagnostics.json', directory/'measurement.json'):
            bindings[str(path.relative_to(baseline))] = bytes_digest(path)
        if sample != actual_sample or sample['exitCode'] != 0 or sample['timedOut']:
            raise ValueError('Failed, timed-out or inconsistent sample')
        coverage = check_coverage(diagnostics, len(data['files']))
        diagnostic = digest({k: v for k, v in diagnostics.items() if k != 'generated_at'})
        scored = score(data, labels, raw['findings'])
        if (sample['semanticDigest'] != semantic_digest(raw) or sample['orderDigest'] != order_digest(raw)
                or sample['diagnosticDigest'] != diagnostic or sample['coverage'] != coverage
                or sample['findingCount'] != len(raw['findings']) or scored != summary['accuracy']):
            raise ValueError('Raw evidence does not match measured summary')
        reports.append(raw)
        diagnostic_digests.append(diagnostic)
        order_digests.append(order_digest(raw))
    stable = dict(semanticStable=len({semantic_digest(r) for r in reports}) == 1,
                  diagnosticStable=len(set(diagnostic_digests)) == 1,
                  rawOrderStable=len(set(order_digests)) == 1, scoreStable=True)
    if any(repeats[k] != v for k, v in stable.items()) or not all(stable[k] for k in stable if k != 'rawOrderStable'):
        raise ValueError('Repeatability claim differs from raw measurements')
    benchmarks = {}
    for name, benchmark in summary['benchmarks'].items():
        safe_relative(name)
        expected_input, expected_recipe = benchmark_input(name, data, repo)
        if benchmark['input'] != expected_input or benchmark['recipe'] != expected_recipe:
            raise ValueError('Benchmark input/recipe differs from frozen corpus workload')
        samples = benchmark['samples']
        if len(samples) < 4 or benchmark['firstMeasured'] != samples[0]:
            raise ValueError('Missing first/warm performance measurements')
        if benchmark['warmDistribution'] != distributions(samples[1:]):
            raise ValueError('Performance distribution differs from samples')
        for index, sample in enumerate(samples):
            path = baseline/f'{name}-{index}'/'measurement.json'
            if (sample['exitCode'] != 0 or sample['timedOut']
                    or {k: v for k, v in sample.items() if k != 'cacheClass'} != load_json(path)):
                raise ValueError('Failed or inconsistent performance sample')
            bindings[str(path.relative_to(baseline))] = bytes_digest(path)
        # Full per-copy bindings remain in the hashed original summary.
        benchmarks[name] = {k: v for k, v in benchmark.items() if k != 'input'}
        benchmarks[name]['input'] = {k: v for k, v in benchmark['input'].items() if k != 'bindings'}
    metadata_path = repo/'docs/quality/RULE-QUALITY-METADATA.json'
    metadata = load_json(metadata_path)
    for source in metadata['source_files']:
        if bytes_digest(repo/safe_relative(source['path'])) != source['sha256']:
            raise ValueError('Rule quality companion is stale')
    nominated = {r for rules in data['families'].values() for r in rules}
    kinds = {r['id']: r['rule_snapshot']['kind'] for r in metadata['rules'] if r['id'] in nominated}
    if kinds.keys() != nominated:
        raise ValueError('Missing nominated rule provenance')
    languages = {'.java': 'Java', '.go': 'Go', '.js': 'JavaScript', '.ts': 'TypeScript',
                 '.tsx': 'TSX', '.py': 'Python', '.rs': 'Rust', '.rb': 'Ruby', '.swift': 'Unsupported Swift'}
    language_by_file = {f['path']: languages.get(pathlib.Path(f['path']).suffix, 'Not Applicable') for f in data['files']}
    family_by_file = {}
    if data.get('annotationVersion'):
        family_by_file = {f['path']: f['artifact'] for f in data['files']}
    for unit in data['units']:
        if data.get('annotationVersion'):
            continue
        family = next(f for f, rules in data['families'].items() if unit['ruleId'] in rules)
        artifact = {'source_api': 'source', 'draft': 'source', 'config': 'configuration',
                    'certificate': 'certificate', 'key': 'key_marker', 'dependency': 'dependency'}.get(family, family)
        if artifact == 'dependency':
            artifact = 'sbom' if 'sbom' in pathlib.Path(unit['path']).name.lower() else 'dependency_manifest'
        previous = family_by_file.setdefault(unit['path'], artifact)
        if previous != artifact:
            raise ValueError('Mixed artifact file needs explicit taxonomy')
    if family_by_file.keys() != {f['path'] for f in data['files']}:
        raise ValueError('Missing artifact taxonomy')
    findings = reports[0]['findings']
    groups = {k: metrics(v) for k, v in summary['accuracy']['groups'].items()}
    result = dict(model='PQS-QAM/1', measuredHead=summary['environment']['head']['stdout'],
                  environment=summary['environment'], corpusVersion=data['corpusVersion'],
                  corpusSha256=summary['corpusSha256'], truthSha256=summary['truthSha256'],
                  sourceSummarySha256=bytes_digest(baseline/'summary.json'), rawBindings=bindings,
                  reporterSha256=bytes_digest(pathlib.Path(__file__)), ruleMetadataSha256=bytes_digest(metadata_path),
                  bySplitFamily=groups,
                  byRule=stratify(data, labels, findings, rule_values={r: r for r in nominated}),
                  byLanguage=stratify(data, labels, findings, file_values=language_by_file),
                  byDetector=stratify(data, labels, findings, rule_values=kinds),
                  byArtifact=stratify(data, labels, findings, file_values=family_by_file),
                  algorithmAccuracy=dict(status='Not Tested', reason='Independent per-algorithm truth taxonomy is not present; rule families are not algorithm truth'),
                  outOfScopePredictions=summary['accuracy']['out_of_scope_predictions'],
                  errorDetails=[d for d in summary['accuracy']['details'] if d['disposition'] in ('fp', 'fn', 'excluded_prediction')],
                  coverage=repeats['samples'][0]['coverage'], repeatability=dict(repeats=repeats['repeats'], **stable),
                  benchmarks=benchmarks,
                  performanceMeasurement=dict(status='Verified' if benchmarks else 'Not Tested',
                      scope=('Raw workload samples and distributions checked; no performance acceptance claimed'
                             if benchmarks else 'Performance workload samples not measured')),
                  releaseDecision='Unknown',
                  axes={k: dict(status=status, scope=scope) for k, status, scope in [
                      ('Q1', 'Partial', 'Frozen nominated reference detection only; per-algorithm truth and population generalization absent'),
                      ('Q2', 'Partial', 'File partitions only; unenumerated population unknown, per-detector applicability not aggregated'),
                      ('Q3', 'Not Tested', 'Detection labels do not validate risk/severity/recommendations or cryptography'),
                      ('Q4', 'Not Tested', 'Separate safety tests required; successful scans do not prove robustness/security'),
                      ('Q5', 'Partial', 'Repeated semantics; ' + ('synthetic resource samples; ' if benchmarks else 'performance Not Tested; ')
                       + 'cold cache/multi-host/real product unmeasured'),
                      ('Q6', 'Not Tested', 'Separate CI/process/platform/Scorecard evidence required') ]})
    if reference is not None:
        result['accuracyComparison'] = compare_accuracy(summary, load_json(reference))
        result['referenceSha256'] = bytes_digest(reference)
    if data.get('annotationVersion'):
        from taxonomy import aggregate, rule_coverage, utc_time
        truth = load_json(corpus_directory/'truth.json')
        if utc_time(environment.get('clockUtc')) < utc_time(truth['freeze']['timestampUtc']):
            raise ValueError('Measurement timestamp precedes truth freeze')
        result['corpusDirectory'] = str(corpus_directory.relative_to(repo))
        result['annotationBindings'] = {name: bytes_digest(corpus_directory/safe_relative(name)) for name in
                                       ['truth.json', 'corpus.json', 'PROTOCOL.md', 'provenance.json', *truth['labelSources'],
                                        *truth.get('sourceFactReviewBindings', {})]}
        for field, name in [('algorithms', 'byAlgorithm'), ('usage', 'byUsage'),
                            ('algorithmParameters', 'byAlgorithmParameter'), ('caseKind', 'byCaseKind')]:
            result[name] = aggregate(data, truth, summary['accuracy'], field, metrics)
        result['byArtifact'] = aggregate(data, truth, summary['accuracy'], 'artifact', metrics)
        result['byInputFormat'] = stratify(data, labels, findings,
                                          file_values={f['path']: f['inputFormat'] for f in data['files']})
        result['ruleCoverage'] = rule_coverage(data, truth, [r['id'] for r in metadata['rules']])
        result['algorithmAccuracy'] = dict(status='Partial',
            scope='Algorithm-conditioned nominal reference detection; not algorithm classification correctness or runtime execution')
        result['axes']['Q1']['scope'] = 'Reviewed algorithm/use annotations; empty strata retained; correlated finite data, not population accuracy'
        result['reviewedTruth'] = truth['labels']
        result['adjudications'] = truth['adjudications']
        result['newDataEvaluation'] = dict(status='Verified', kind='Frozen corpus measurement; chronology is recorded separately',
            splitPolicy='Public upstream-authored holdout; selection bias and one upstream project; tuning is locally authored')
        duplicates = collections.defaultdict(list)
        lineages = collections.Counter()
        for file in data['files']:
            duplicates[file['sha256']].append(file['path'])
            lineages[file['split']+'/'+file['lineage']] += 1
        result['lineageAnalysis'] = dict(filesByLineage=dict(lineages),
            exactByteDuplicateGroups=[paths for paths in duplicates.values() if len(paths) > 1],
            inference='Filename/encoding controls and upstream parameter loops are correlated; no binomial confidence interval or population generalization')
        labels_by_id = {r['id']: r['label'] for r in truth['labels']}
        fp = [d for d in summary['accuracy']['details'] if d['disposition'] == 'fp']
        unattributed = sum(d['unit'] is None for d in fp)
        from taxonomy import annotation_audit_status
        audit_status = annotation_audit_status(corpus_directory, truth)
        result['annotationInterpretation'] = dict(status=audit_status if not unattributed else 'Partial',
            sourceAuditStatus=audit_status,
            predictionAttributionStatus='Partial' if unattributed else 'Verified',
            matchedNegativePredictionCount=sum(labels_by_id.get(d['unit']) == 'negative' for d in fp),
            duplicatePositivePredictionCount=sum(labels_by_id.get(d['unit']) == 'positive' for d in fp),
            unattributedPredictionCount=unattributed,
            scope='Precision/F1 measure agreement with frozen nominated annotations. Unattributed FP can be legitimate unannotated facts, not established product false positives.',
            followup=('Preserve this bounded source-audited truth/results; retain detector FP/FN for triage.'
                      if audit_status == 'Verified' and not unattributed else
                      'Source-based triage required; preserve this truth/results. Any corrected annotation version is post-observation tuning, not a fresh independent holdout evaluation.'))
        selection = load_json(corpus_directory/'provenance.json').get('selection', {})
        result['newDataEvaluation']['sourceSelection'] = selection
        result['newDataEvaluation']['splitPolicy'] = selection.get('independence', 'See bound source provenance; population independence is not established')
        if reference is not None:
            result['newDataEvaluation']['kind'] = ('Same-data regression or validation replay' if
                result['accuracyComparison']['status'] != 'Not Applicable' else 'Different-data evaluation; no same-data regression conclusion')
        if truth.get('metadataReplay'):
            result['newDataEvaluation']['kind'] = 'Metadata-only replay; not a new independent trial'
            result['newDataEvaluation']['metadataReplay'] = truth['metadataReplay']
        if applicability is not None:
            result['supportedScopeEvaluation'] = supported_scope(applicability, repo, corpus_directory,
                                                                 data, truth, findings, language_by_file, kinds)
    elif applicability is not None:
        raise ValueError('Applicability requires reviewed annotation taxonomy')
    return result


def write_assessment(result, output):
    output.mkdir(parents=True, exist_ok=False)
    (output/'quality.json').write_text(json.dumps(result, indent=2, ensure_ascii=False, allow_nan=False)+'\n')
    rows = ['# PQS-QAM 品質測定レポート', '', f"測定対象: `{result['measuredHead']}`。",
            f"コーパス: `{result['corpusVersion']}`。リリース判定: **{result['releaseDecision']}**。",
            '検出参照の有限データ評価です。安全性・実利用・PQC移行完了の認定ではありません。', '',
            '| 軸 | 状態 | 範囲 |', '|---|---|---|']
    rows += [f"| {k} | {v['status']} | {v['scope']} |" for k, v in result['axes'].items()]
    replay = result.get('newDataEvaluation', {}).get('metadataReplay')
    if replay:
        rows += ['', 'これは出典metadataを訂正した再評価です。新しい独立試行ではありません。',
                 f"訂正理由: {replay['reason']}", '']
    rows += ['', 'FPは所見数、FPRは誤報があったnegative単位数/negative単位数です。母数ゼロはundefined。', '',
             '| split/family | TP | FP | FN | TN | eligible | precision | recall | F1 | FPR | FNR |',
             '|---|---:|---:|---:|---:|---:|---|---|---|---|---|']
    for key, m in result['bySplitFamily'].items():
        values = [str(m[k]) for k in ('tp', 'fp', 'fn', 'tn', 'eligible')]
        values += ['undefined' if m[k] is None else f'{m[k]:.6f}' for k in ('precision', 'recall', 'f1', 'fpr', 'fnr')]
        rows.append('| ' + ' | '.join([key, *values]) + ' |')
    rows += ['', f"未評価の他ルール所見: {result['outOfScopePredictions']}。",
             '言語・ルール・検出器・入力種別の分母、FP/FN、coverage、環境、反復、性能の全値はquality.jsonを参照。',
             ('方式別・用途別はquality.jsonを参照。暗号分類や実行の正しさは未検証です。' if 'byAlgorithm' in result
              else 'アルゴリズム別: Not Tested。未列挙母数はunknownであり、0ではありません。'),
             '品質モデル・式・限界はdocs/quality/QUALITY-MODEL.mdとQUALITY-METRICS.mdを参照。', '']
    if 'accuracyComparison' in result:
        comparison = result['accuracyComparison']
        rows += [f"同一truthの回帰比較: **{comparison['status']}**。数値CIゲートは変更していません。",
                 '性能の同条件比較・調査信号は別評価であり、この判定に含みません。', '']
    if 'annotationInterpretation' in result:
        i = result['annotationInterpretation']
        rows += [f"指定した規則と入力の範囲での注釈確認: **{i['status']}**。負例に対応したFP所見{i['matchedNegativePredictionCount']}件、"
                 f"positiveへの重複{i['duplicatePositivePredictionCount']}件、帰属不能{i['unattributedPredictionCount']}件。",
                 '帰属不能には正しい未注釈の事実が含まれ得ます。製品の誤検出とは断定できません。Precision/F1は固定注釈への一致成績です。',
                 ('二者の全指定規則・全入力のソース監査と予測の帰属を確認しました。この有限範囲を超える検出精度の保証ではありません。検出器のFP/FNは保持します。'
                  if i['status'] == 'Verified' else
                  '注釈不足はソースから調査します。正解を結果に合わせて変更せず、修正版を事後の調整用評価として区別します。'), '']
    if 'supportedScopeEvaluation' in result:
        scoped = result['supportedScopeEvaluation']
        rows += ['公開対応範囲の別評価はsupported-scope.jsonを参照してください。',
                 f"元の属性正解・成績を保持し、二者が対応外と確認した{len(scoped['excludedUnits'])}単位を別表の適格分母から除きました。",
                 'これは測定後の適用範囲訂正です。検出器の改善や新しい独立評価とは扱いません。', '']
    (output/'quality.md').write_text('\n'.join(rows))
    if 'supportedScopeEvaluation' in result:
        (output/'supported-scope.json').write_text(json.dumps(result['supportedScopeEvaluation'],
            indent=2, ensure_ascii=False, allow_nan=False)+'\n')
    if 'reviewedTruth' in result:
        def write(name, value):
            (output/name).write_text(json.dumps(value, indent=2, ensure_ascii=False, allow_nan=False)+'\n')
        write('algorithm-truth.json', [dict(id=r['id'], label=r['label'], algorithms=r['algorithms'],
                                          parameters=r['parameters'], reason=r['reason']) for r in result['reviewedTruth']])
        write('usage-truth.json', [dict(id=r['id'], usage=r['usage'], artifact=r['artifact']) for r in result['reviewedTruth']])
        write('artifact-metrics.json', dict(byArtifact=result['byArtifact'], byInputFormat=result['byInputFormat']))
        write('adversarial-cases.json', dict(byCaseKind=result['byCaseKind'],
            cases=[dict(id=r['id'], caseKind=r['caseKind'], label=r['label']) for r in result['reviewedTruth']],
            errorDetails=result['errorDetails']))
        write('case-coverage.json', dict(rules=result['ruleCoverage'],
            byAlgorithm=result['byAlgorithm'], byUsage=result['byUsage'], byCaseKind=result['byCaseKind'],
            lineageAnalysis=result['lineageAnalysis'],
            limits=['Empty strata are Not Tested, not negative evidence',
                    'Size/truncation resource cases belong to TASK-055/047; not part of eligible accuracy denominator',
                    'Nominal presence is not execution, enabled dependency functionality or cryptographic correctness']))
        write('metrics.json', {k: result[k] for k in ['corpusSha256', 'truthSha256', 'measuredHead', 'sourceSummarySha256',
              'annotationInterpretation', 'bySplitFamily', 'byAlgorithm', 'byAlgorithmParameter',
              'byUsage', 'byRule', 'byLanguage', 'byDetector', 'byArtifact', 'byInputFormat', 'outOfScopePredictions']})
        write('comparison.json', dict(newData=result['newDataEvaluation'],
            annotationInterpretation=result['annotationInterpretation'],
            sameData=result.get('accuracyComparison', dict(status='Not Applicable', reason='No same-data reference supplied'))))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', required=True, type=pathlib.Path)
    parser.add_argument('--out-dir', required=True, type=pathlib.Path)
    parser.add_argument('--reference', type=pathlib.Path)
    parser.add_argument('--corpus-dir', type=pathlib.Path, help='Optional frozen corpus under tests/quality')
    parser.add_argument('--applicability', type=pathlib.Path, help='Optional separately reviewed public-support scope; original results retained')
    args = parser.parse_args()
    try:
        output = args.out_dir.resolve()
        if not output.is_relative_to(REPO/'target') or output == REPO/'target':
            raise ValueError('Output must be a new directory under target/')
        result = build_assessment(args.baseline.resolve(), reference=args.reference, corpus_dir=args.corpus_dir,
                                  applicability=args.applicability)
        write_assessment(result, output)
    except (ValueError, OSError, KeyError, TypeError) as error:
        print(f'Quality evidence/report failed: {type(error).__name__}; no quality pass asserted', file=sys.stderr)
        return 1
    print(f'Quality report saved: {output}; release decision Unknown; numeric gates unchanged')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
