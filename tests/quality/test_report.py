"""Hand-derived reporting checks; truth never comes from scanner predictions."""
import copy
import json
import pathlib
import tempfile
import unittest

import evaluate
import report
from run import distributions


class MetricsTests(unittest.TestCase):
    def test_fpr_counts_negative_units_not_duplicate_or_extra_predictions(self):
        self.assertIsNotNone(report, "quality report implementation is missing")
        # One TP, one FN; one negative hit twice, one TN; one extra prediction.
        # FP=3, but only 1 of 2 negative units was falsely reported.
        m = report.metrics(dict(tp=1, fp=3, fn=1, tn=1, positive=2, negative=2,
                                ambiguous=1, unsupported=1, excluded_predictions=2))
        self.assertEqual(m['precision'], 1/4)
        self.assertEqual(m['recall'], 1/2)
        self.assertEqual(m['f1'], 1/3)
        self.assertEqual(m['fpr'], 1/2)
        self.assertEqual(m['fnr'], 1/2)
        self.assertEqual(m['fpr_denominator'], 2)
        self.assertEqual(m['false_positive_units'], 1)
        self.assertEqual(m['eligible'], 4)

    def test_empty_and_excluded_populations_have_null_rates(self):
        self.assertIsNotNone(report, "quality report implementation is missing")
        m = report.metrics(dict(tp=0, fp=0, fn=0, tn=0, positive=0, negative=0,
                                ambiguous=1, unsupported=2, excluded_predictions=3))
        for key in ['precision', 'recall', 'f1', 'fpr', 'fnr']:
            self.assertIsNone(m[key], key)
        self.assertEqual(m['eligible'], 0)

    def test_comparison_requires_identical_truth_and_input(self):
        self.assertIsNotNone(report, "quality report implementation is missing")
        base = dict(corpusSha256='a', truthSha256='b', corpusInput={'sha256': 'c'},
                    accuracy={'groups': {'tuning/api': {'tp': 1, 'fp': 0, 'fn': 1}}})
        current = copy.deepcopy(base)
        current['accuracy']['groups']['tuning/api']['fp'] = 1
        result = report.compare_accuracy(current, base)
        self.assertEqual(result['status'], 'Failed')
        self.assertEqual(result['deltas']['tuning/api']['fp'], 1)
        current['truthSha256'] = 'different'
        result = report.compare_accuracy(current, base)
        self.assertEqual(result['status'], 'Not Applicable')
        self.assertNotIn('deltas', result)

    def test_zero_detections_do_not_produce_perfect_precision_or_recall(self):
        m = report.metrics(dict(tp=0, fp=0, fn=2, tn=3, positive=2, negative=3,
                                ambiguous=0, unsupported=0, excluded_predictions=0))
        self.assertIsNone(m['precision'])
        self.assertEqual((m['recall'], m['f1'], m['fpr'], m['fnr']), (0, 0, 0, 1))


def save(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value) + '\n')


class EvidenceTests(unittest.TestCase):
    def applicability(self, q):
        directory = self.repo/'tests/quality/applicability'
        directory.mkdir()
        (directory/'PROTOCOL.md').write_text('Separate source-based applicability, not new blind truth.\n')
        (self.repo/'contract.txt').write_text('The synthetic p property is outside the documented subset.\n')
        record = dict(version=1, scope='documented-public-input-subset',
            corpusSha256=evaluate.bytes_digest(q/'corpus.json'), truthSha256=evaluate.bytes_digest(q/'truth.json'),
            protocolSha256=evaluate.bytes_digest(directory/'PROTOCOL.md'),
            contractBindings={'contract.txt': evaluate.bytes_digest(self.repo/'contract.txt')},
            dispositions=[dict(id='p', status='unsupported', reason='Explicitly outside the bound public subset')],
            sourceBased=True, postObservation=True, historicalMeasurementsSeen=True,
            otherApplicabilityReviewSeen=False, remainingUnitsReviewed=True, unresolved=[])
        for who in ['a', 'b']:
            save(directory/f'{who}.json', dict(record, reviewer=who, timestampUtc='2026-01-01T00:00:00Z'))
        record.update(reviewSources={f'{who}.json': evaluate.bytes_digest(directory/f'{who}.json') for who in ['a', 'b']},
                      timestampUtc='2026-01-02T00:00:00Z')
        save(directory/'assessment.json', record)
        return directory/'assessment.json'

    def test_applicability_preserves_original_and_rescores_excluded_predictions(self):
        q = self.enrich()
        path = self.applicability(q)
        result = report.build_assessment(self.baseline, self.repo, corpus_dir=q, applicability=path)
        self.assertEqual(result['bySplitFamily']['tuning/source_api']['tp'], 1)
        scoped = result['supportedScopeEvaluation']
        m = scoped['bySplitFamily']['tuning/source_api']
        self.assertEqual((m['tp'], m['tn'], m['eligible'], m['unsupported'], m['excluded_predictions']), (0, 1, 1, 1, 1))
        self.assertIsNone(m['precision'])
        self.assertEqual(scoped['byAlgorithm']['tuning/RSA']['unsupported'], 1)
        self.assertTrue(scoped['postObservation'])
        self.assertIn('byInputFormat', scoped)
        self.assertEqual(scoped['excludedUnits'][0]['originalLabel'], 'positive')
        report.write_assessment(result, self.repo/'output')
        self.assertEqual(json.loads((self.repo/'output/supported-scope.json').read_text())['bySplitFamily'], scoped['bySplitFamily'])

    def test_applicability_excludes_unreported_positive_and_negative_units(self):
        q = self.enrich()
        path = self.applicability(q)
        manifest = json.loads(path.read_text())
        manifest['dispositions'].append(dict(id='n', status='unsupported', reason='Second property outside the documented subset'))
        for name in manifest['reviewSources']:
            row = json.loads((path.parent/name).read_text())
            row['dispositions'] = manifest['dispositions']
            save(path.parent/name, row)
            manifest['reviewSources'][name] = evaluate.bytes_digest(path.parent/name)
        save(path, manifest)
        data, labels = evaluate.load_truth(q)
        before = evaluate.score(data, labels, [])['groups']['tuning/source_api']
        self.assertEqual((before['fn'], before['tn']), (1, 1))
        scoped = report.supported_scope(path, self.repo, q, data, json.loads((q/'truth.json').read_text()),
                                        [], {'a.js': 'JavaScript'}, {'API': 'tree_sitter'})
        after = scoped['bySplitFamily']['tuning/source_api']
        self.assertEqual((after['fn'], after['tn'], after['unsupported'], after['eligible']), (0, 0, 2, 0))
        self.assertIsNone(after['recall'])

    def test_applicability_rejects_stale_unreviewed_or_unresolved_scope(self):
        q = self.enrich()
        path = self.applicability(q)
        original = json.loads(path.read_text())
        review_path = path.parent/'b.json'
        original_review = json.loads(review_path.read_text())
        for mutation in ['truth', 'hash', 'identity', 'unresolved', 'unknown', 'duplicate', 'safe_negative', 'reason', 'time']:
            manifest = copy.deepcopy(original)
            row = copy.deepcopy(original_review)
            if mutation == 'truth': manifest['truthSha256'] = '0'*64
            if mutation == 'identity': row['reviewer'] = 'a'
            if mutation == 'unresolved': row['unresolved'] = ['Opaque input scope not decided']
            if mutation == 'time': row['timestampUtc'] = '2026-01-03T00:00:00Z'
            if mutation == 'unknown': manifest['dispositions'][0]['id'] = 'missing'
            if mutation == 'duplicate': manifest['dispositions'] *= 2
            if mutation == 'safe_negative': manifest['dispositions'][0]['status'] = 'negative'
            if mutation == 'reason': manifest['dispositions'][0]['reason'] = ''
            save(review_path, row)
            manifest['reviewSources']['b.json'] = evaluate.bytes_digest(review_path)
            if mutation == 'hash': manifest['reviewSources']['b.json'] = '0'*64
            save(path, manifest)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                report.build_assessment(self.baseline, self.repo, corpus_dir=q, applicability=path)
        save(path, original)
        save(review_path, original_review)
        (self.repo/'contract.txt').write_text('changed scope')
        with self.assertRaises(ValueError):
            report.build_assessment(self.baseline, self.repo, corpus_dir=q, applicability=path)

    def test_full_annotation_audit_requires_two_bound_complete_source_reviews(self):
        from taxonomy import annotation_audit_status
        q = self.enrich()
        truth = json.loads((q/'truth.json').read_text())
        truth['annotationAuditRequired'] = True
        with self.assertRaises(ValueError): annotation_audit_status(q, truth)
        for who in ['a','b']:
            row = json.loads((q/f'{who}.json').read_text())
            row['coverageAudit'] = dict(status='Verified', scope='all_nominated_rules_all_files',
                fileCount=1, nominatedRuleCount=1, reviewedFileRulePairs=1,
                missingFacts=[], unresolvedDefinitions=[], reason='All declared properties reviewed from source',
                protocolSha256=truth['freeze']['protocolSha256'])
            save(q/f'{who}.json', row)
            truth['labelSources'][f'{who}.json'] = evaluate.bytes_digest(q/f'{who}.json')
        self.assertEqual(annotation_audit_status(q, truth), 'Verified')
        valid_record = json.loads((q/'b.json').read_text())
        for mutation in ['missing', 'definition', 'protocol', 'reason', 'population',
                         'files', 'rules', 'status', 'scope', 'binding']:
            row = copy.deepcopy(valid_record)
            if mutation == 'missing': row['coverageAudit']['missingFacts'] = ['unannotated RSA signature']
            if mutation == 'definition': row['coverageAudit']['unresolvedDefinitions'] = ['literal or API?']
            if mutation == 'protocol': row['coverageAudit']['protocolSha256'] = '0'*64
            if mutation == 'reason': row['coverageAudit']['reason'] = ''
            if mutation == 'population': row['coverageAudit']['reviewedFileRulePairs'] = 0
            if mutation == 'files': row['coverageAudit']['fileCount'] = 0
            if mutation == 'rules': row['coverageAudit']['nominatedRuleCount'] = 0
            if mutation == 'status': row['coverageAudit']['status'] = 'Partial'
            if mutation == 'scope': row['coverageAudit']['scope'] = 'listed_units_only'
            save(q/'b.json', row)
            truth['labelSources']['b.json'] = evaluate.bytes_digest(q/'b.json')
            if mutation == 'binding': truth['labelSources']['b.json'] = '0'*64
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): annotation_audit_status(q, truth)

    def test_corpus_audit_requirement_cannot_be_removed_from_truth(self):
        q = self.enrich()
        data = json.loads((q/'corpus.json').read_text())
        data['annotationAuditRequired'] = True
        save(q/'corpus.json', data)
        truth = json.loads((q/'truth.json').read_text())
        truth['corpusSha256'] = evaluate.bytes_digest(q/'corpus.json')
        for who in ['a','b']:
            row = json.loads((q/f'{who}.json').read_text())
            row['corpusSha256'] = truth['corpusSha256']
            row['coverageAudit'] = dict(status='Verified', scope='all_nominated_rules_all_files',fileCount=1,
                nominatedRuleCount=1,reviewedFileRulePairs=1,missingFacts=[],unresolvedDefinitions=[],
                protocolSha256=truth['freeze']['protocolSha256'],reason='All source properties reviewed')
            save(q/f'{who}.json', row)
            truth['labelSources'][f'{who}.json'] = evaluate.bytes_digest(q/f'{who}.json')
        for flag in [None, False]:
            changed = copy.deepcopy(truth)
            if flag is not None: changed['annotationAuditRequired'] = flag
            save(q/'truth.json',changed)
            with self.assertRaisesRegex(ValueError,'requires'): evaluate.load_truth(q)
        truth['annotationAuditRequired'] = True
        save(q/'truth.json',truth)
        evaluate.load_truth(q)

    def test_same_data_reference_does_not_claim_first_independent_evaluation(self):
        q = self.enrich()
        result = report.build_assessment(self.baseline,self.repo,reference=self.baseline/'summary.json',corpus_dir=q)
        self.assertEqual(result['accuracyComparison']['status'],'Passed')
        self.assertEqual(result['newDataEvaluation']['kind'],'Same-data regression or validation replay')

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.repo = pathlib.Path(self.tmp.name).resolve()
        for name in ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml',
                     'tests/quality/evaluate.py', 'tests/quality/run.py', 'crates/demo/src/lib.rs']:
            target = self.repo/name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text('synthetic source binding; not executed\n')
        q = self.repo / 'tests/quality'
        (self.repo / 'input.js').write_text('api(); other();\n')
        data = dict(version=1, corpusVersion='test', license='MIT', families={'source_api': ['API']},
                    files=[dict(path='a.js', source='input.js', split='holdout', lineage='independent',
                                license='MIT', sha256=evaluate.bytes_digest(self.repo / 'input.js'))],
                    units=[dict(id='p', path='a.js', ruleId='API', line=1, startColumn=1, endColumn=6),
                           dict(id='n', path='a.js', ruleId='API', line=1, startColumn=8, endColumn=15)])
        save(q/'corpus.json', data)
        labels = [dict(id='p', label='positive'), dict(id='n', label='negative')]
        for who in ['a', 'b']:
            save(q/f'{who}.json', dict(reviewer=who, corpusSha256=evaluate.bytes_digest(q/'corpus.json'), labels=labels))
        save(q/'truth.json', dict(corpusSha256=evaluate.bytes_digest(q/'corpus.json'), labels=labels,
                                  labelSources={f'{who}.json': evaluate.bytes_digest(q/f'{who}.json') for who in ['a', 'b']}))
        save(self.repo/'docs/quality/rules/RULE-QUALITY-METADATA.json', dict(source_files=[], rules=[
            dict(id='API', rule_snapshot={'kind': 'tree_sitter'}, algorithm_families=['unknown'])]))
        raw = dict(findings=[dict(rule_id='API', location=dict(file='a.js', line=1, column=1))])
        coverage = dict(selected_files=1, read_ok_files=1, read_failed_files=0, analyzed_files=0,
                        partial_files=0, analysis_failed_files=0, unsupported_files=0,
                        not_applicable_files=0, unknown_files=1, not_enumerated_population=True, status='unknown')
        diagnostics = dict(coverage=coverage)
        self.baseline = self.repo/'baseline'
        samples = []
        for i in range(2):
            save(self.baseline/f'accuracy-{i}/report.json', raw)
            save(self.baseline/f'accuracy-{i}/diagnostics.json', diagnostics)
            sample = dict(exitCode=0, timedOut=False, semanticDigest=evaluate.semantic_digest(raw),
                          orderDigest=evaluate.order_digest(raw), diagnosticDigest=evaluate.digest(diagnostics),
                          coverage=coverage, findingCount=1)
            save(self.baseline/f'accuracy-{i}/measurement.json', sample)
            samples.append(sample)
        self.summary = dict(version=1, corpusVersion='test', corpusSha256=evaluate.bytes_digest(q/'corpus.json'),
                            truthSha256=evaluate.bytes_digest(q/'truth.json'),
                            corpusInput=dict(files=1, bytes=16, sha256=evaluate.digest({'a.js': dict(sha256=data['files'][0]['sha256'], bytes=16)}),
                                             bindings={'a.js': dict(sha256=data['files'][0]['sha256'], bytes=16)}),
                            environment={'head': {'stdout': 'a'*40},
                                'runtimeSourceBindings': {name: evaluate.bytes_digest(self.repo/name) for name in
                                    ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'crates/demo/src/lib.rs']},
                                'harnessBindings': {name: evaluate.bytes_digest(self.repo/name) for name in
                                    ['tests/quality/evaluate.py', 'tests/quality/run.py']}},
                            accuracy=evaluate.score(data, {'p': 'positive', 'n': 'negative'}, raw['findings']),
                            repeatability=dict(repeats=2, semanticStable=True, diagnosticStable=True,
                                               rawOrderStable=True, scoreStable=True, samples=samples), benchmarks={})
        self.summary['environment']['runtimeSourceDigest'] = evaluate.digest(self.summary['environment']['runtimeSourceBindings'])
        self.summary['environment']['harnessDigest'] = evaluate.digest(self.summary['environment']['harnessBindings'])
        save(self.baseline/'summary.json', self.summary)

    def build(self):
        self.assertIsNotNone(report, "quality report implementation is missing")
        return report.build_assessment(self.baseline, self.repo)

    def test_raw_evidence_produces_stratified_report_without_claiming_whole_product_pass(self):
        result = self.build()
        self.assertEqual(result['bySplitFamily']['holdout/source_api']['tp'], 1)
        self.assertEqual(result['byRule']['holdout/API']['tn'], 1)
        self.assertEqual(result['byLanguage']['holdout/JavaScript']['eligible'], 2)
        self.assertEqual(result['byDetector']['holdout/tree_sitter']['tp'], 1)
        self.assertEqual(result['byArtifact']['holdout/source']['tp'], 1)
        self.assertEqual(result['algorithmAccuracy']['status'], 'Not Tested')
        self.assertTrue(result['coverage']['not_enumerated_population'])
        self.assertEqual(result['axes']['Q3']['status'], 'Not Tested')
        self.assertEqual(result['axes']['Q4']['status'], 'Not Tested')
        self.assertEqual(result['axes']['Q6']['status'], 'Not Tested')
        self.assertEqual(result['releaseDecision'], 'Unknown')
        self.assertEqual(result['performanceMeasurement']['status'], 'Not Tested')
        self.assertEqual(result['performanceMeasurement']['scope'], 'Performance workload samples not measured')
        self.assertNotIn('synthetic resource samples;', result['axes']['Q5']['scope'])

    def test_missing_runtime_or_evaluator_bindings_cannot_be_source_bound(self):
        self.build()
        for mutation in ['empty_runtime', 'missing_runtime', 'runtime_digest', 'missing_evaluator', 'missing_runner', 'harness_digest']:
            changed = copy.deepcopy(self.summary)
            env = changed['environment']
            if mutation == 'empty_runtime': env['runtimeSourceBindings'] = {}
            if mutation == 'missing_runtime': env['runtimeSourceBindings'].pop('Cargo.lock')
            if mutation == 'runtime_digest': env['runtimeSourceDigest'] = '0'*64
            if mutation == 'missing_evaluator': env['harnessBindings'].pop('tests/quality/evaluate.py')
            if mutation == 'missing_runner': env['harnessBindings'].pop('tests/quality/run.py')
            if mutation == 'harness_digest': env['harnessDigest'] = '0'*64
            # Keep digest self-consistency on missing-population cases so these
            # assertions exercise mandatory source membership, not only hashes.
            if mutation in ['empty_runtime', 'missing_runtime']:
                env['runtimeSourceDigest'] = evaluate.digest(env['runtimeSourceBindings'])
            if mutation in ['missing_evaluator', 'missing_runner']:
                env['harnessDigest'] = evaluate.digest(env['harnessBindings'])
            save(self.baseline/'summary.json', changed)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): self.build()

    def test_summary_score_raw_report_and_failed_samples_are_rejected(self):
        self.build()
        for mutation in ['score', 'raw', 'exit', 'timeout', 'coverage', 'source', 'digest', 'input', 'repeat']:
            with self.subTest(mutation=mutation):
                changed = copy.deepcopy(self.summary)
                if mutation == 'score': changed['accuracy']['groups']['holdout/source_api']['tp'] = 2
                if mutation == 'exit': changed['repeatability']['samples'][0]['exitCode'] = 1
                if mutation == 'timeout': changed['repeatability']['samples'][0]['timedOut'] = True
                if mutation == 'coverage': changed['repeatability']['samples'][0]['coverage']['analyzed_files'] = 1
                if mutation == 'source': changed['environment']['runtimeSourceBindings'] = {'input.js': '0'*64}
                if mutation == 'digest': changed['corpusSha256'] = '0'*64
                if mutation == 'input': changed['corpusInput']['sha256'] = '0'*64
                if mutation == 'repeat': changed['repeatability']['semanticStable'] = False
                if mutation == 'raw': save(self.baseline/'accuracy-0/report.json', {'findings': []})
                save(self.baseline/'summary.json', changed)
                with self.assertRaises(ValueError): self.build()
                if mutation == 'raw':
                    save(self.baseline/'accuracy-0/report.json', dict(findings=[dict(rule_id='API', location=dict(file='a.js', line=1, column=1))]))
        save(self.baseline/'summary.json', self.summary)

    def test_benchmark_distributions_and_failures_are_checked_against_raw_samples(self):
        samples = [dict(exitCode=0, timedOut=False, wallSeconds=i, cpuSeconds=i/2,
                        peakRssBytes=1024*i, cacheClass='first-measured' if i == 1 else 'warm')
                   for i in range(1, 5)]
        for i, sample in enumerate(samples):
            save(self.baseline/f'small-{i}/measurement.json', {k: v for k, v in sample.items() if k != 'cacheClass'})
        self.summary['benchmarks'] = {'small': dict(samples=samples, firstMeasured=samples[0],
            warmDistribution=distributions(samples[1:]), input=self.summary['corpusInput'],
            recipe='1 deterministic complete corpus copies; not accuracy truth')}
        save(self.baseline/'summary.json', self.summary)
        self.assertEqual(self.build()['benchmarks']['small']['warmDistribution']['wallSeconds']['median'], 3)
        for mutation in ['distribution', 'timeout', 'input', 'recipe', 'missing_raw']:
            changed = copy.deepcopy(self.summary)
            if mutation == 'distribution': changed['benchmarks']['small']['warmDistribution']['wallSeconds']['median'] = 0
            if mutation == 'timeout': changed['benchmarks']['small']['samples'][1]['timedOut'] = True
            if mutation == 'input': changed['benchmarks']['small']['input'] = dict(files=0, bytes=0, sha256='0'*64)
            if mutation == 'recipe': changed['benchmarks']['small']['recipe'] = 'real production service'
            if mutation == 'missing_raw': save(self.baseline/'small-0/measurement.json', {})
            save(self.baseline/'summary.json', changed)
            with self.assertRaises(ValueError): self.build()

    def test_truth_tamper_and_missing_rule_provenance_are_rejected(self):
        self.build()
        truth_path = self.repo/'tests/quality/truth.json'
        original = truth_path.read_bytes()
        truth = json.loads(original)
        truth['labels'][0]['label'] = 'negative'
        save(truth_path, truth)
        with self.assertRaises(ValueError): self.build()
        truth_path.write_bytes(original)
        save(self.repo/'docs/quality/rules/RULE-QUALITY-METADATA.json', dict(source_files=[], rules=[]))
        with self.assertRaises(ValueError): self.build()

    def test_generated_markdown_matches_metrics_and_overwrite_is_refused(self):
        result = self.build()
        output = self.repo/'out'
        report.write_assessment(result, output)
        loaded = json.loads((output/'quality.json').read_text())
        self.assertEqual(loaded, result)
        text = (output/'quality.md').read_text()
        self.assertIn('holdout/source_api', text)
        self.assertIn('Unknown', text)
        self.assertIn('Not Tested', text)
        before = (output/'quality.json').read_bytes()
        with self.assertRaises(FileExistsError): report.write_assessment(result, output)
        self.assertEqual((output/'quality.json').read_bytes(), before)

    def enrich(self):
        """Source-independent tiny fixture; never generated from product results."""
        q = self.repo/'tests/quality/expanded'
        old = self.repo/'tests/quality'
        data = json.loads((old/'corpus.json').read_text())
        data['annotationVersion'] = 1
        data['files'][0].update(split='tuning', inputFormat='JavaScript', artifact='source')
        labels = [dict(id=u, label=l, algorithms=['RSA'], usage='signature', artifact='source',
                       caseKind='ordinary', parameters=['2048'], reason='Explicit synthetic source fact')
                  for u, l in [('p', 'positive'), ('n', 'negative')]]
        save(q/'corpus.json', data)
        (q/'PROTOCOL.md').write_text('Source-only instructions\n')
        save(q/'provenance.json', dict(files={'a.js':dict(sha256=data['files'][0]['sha256'], lineage='independent')}, licenseBindings={}))
        for who in ['a', 'b']:
            save(q/f'{who}.json', dict(reviewer=who, corpusSha256=evaluate.bytes_digest(q/'corpus.json'),
                protocolSha256=evaluate.bytes_digest(q/'PROTOCOL.md'), sourceOnly=True,
                otherLabelsSeen=False, scannerResultsSeen=False, labels=labels, timestampUtc='2025-12-31T00:00:00Z'))
        save(q/'truth.json', dict(corpusSha256=evaluate.bytes_digest(q/'corpus.json'), labels=labels,
            adjudications=[], labelSources={f'{who}.json':evaluate.bytes_digest(q/f'{who}.json') for who in ['a','b']},
            freeze=dict(timestampUtc='2026-01-01T00:00:00Z', beforeFirstMeasurement=True,
                protocolSha256=evaluate.bytes_digest(q/'PROTOCOL.md'), provenanceSha256=evaluate.bytes_digest(q/'provenance.json'))))
        (old/'taxonomy.py').write_text('Synthetic fixture annotation binding; not executable\n')
        self.summary.update(corpusDirectory='tests/quality/expanded', corpusSha256=evaluate.bytes_digest(q/'corpus.json'),
                            truthSha256=evaluate.bytes_digest(q/'truth.json'))
        self.summary['environment']['clockUtc'] = '2026-01-02T00:00:00Z'
        harness = self.summary['environment']['harnessBindings']
        harness['tests/quality/taxonomy.py'] = evaluate.bytes_digest(old/'taxonomy.py')
        self.summary['environment']['harnessDigest'] = evaluate.digest(harness)
        findings = json.loads((self.baseline/'accuracy-0/report.json').read_text())['findings']
        self.summary['accuracy'] = evaluate.score(data, {'p':'positive','n':'negative'}, findings)
        save(self.baseline/'summary.json',self.summary)
        return q

    def test_selected_frozen_annotations_produce_all_task_outputs(self):
        q = self.enrich()
        result = report.build_assessment(self.baseline, self.repo, corpus_dir=q)
        self.assertEqual(result['byAlgorithm']['tuning/RSA']['tp'], 1)
        self.assertEqual(result['byUsage']['tuning/signature']['tn'], 1)
        self.assertIsNone(result['byAlgorithm']['tuning/ML-KEM']['recall'])
        self.assertEqual(result['ruleCoverage']['API']['status'], 'Verified')
        self.assertEqual(result['newDataEvaluation']['status'], 'Verified')
        self.assertEqual(result['annotationInterpretation']['unattributedPredictionCount'], 0)
        output = self.repo/'expanded-output'
        report.write_assessment(result, output)
        for name in ['algorithm-truth', 'usage-truth', 'artifact-metrics', 'adversarial-cases',
                     'case-coverage', 'comparison', 'metrics']:
            self.assertIsInstance(json.loads((output/f'{name}.json').read_text()), (dict,list))
        comparison = json.loads((output/'comparison.json').read_text())
        self.assertEqual(comparison['sameData']['status'], 'Not Applicable')
        self.assertEqual(comparison['sameData']['reason'], 'No same-data reference supplied')
        self.assertNotIn('アルゴリズム別: Not Tested', (output/'quality.md').read_text())
        # No accidental use of alternative truth with the default corpus.
        with self.assertRaises(ValueError): self.build()

    def test_rehashed_review_disagreement_and_protocol_or_lineage_tamper_rejected(self):
        q = self.enrich()
        truth = json.loads((q/'truth.json').read_text())
        b = json.loads((q/'b.json').read_text())
        b['labels'][0]['usage'] = 'encryption'
        save(q/'b.json', b)
        truth['labelSources']['b.json'] = evaluate.bytes_digest(q/'b.json')
        save(q/'truth.json', truth)
        with self.assertRaisesRegex(ValueError, 'adjudication'): evaluate.load_truth(q)
        truth['adjudications'] = [dict(id='p',reason='Original source describes signature',
            resolution={k: truth['labels'][0][k] for k in ('label','algorithms','usage','artifact','caseKind','parameters')})]
        save(q/'truth.json', truth)
        evaluate.load_truth(q)
        (q/'PROTOCOL.md').write_text('Changed labels after results\n')
        with self.assertRaisesRegex(ValueError, 'freeze'): evaluate.load_truth(q)

    def test_missing_annotation_harness_binding_rejected_even_with_recomputed_digest(self):
        q = self.enrich()
        harness = self.summary['environment']['harnessBindings']
        harness.pop('tests/quality/taxonomy.py')
        self.summary['environment']['harnessDigest'] = evaluate.digest(harness)
        save(self.baseline/'summary.json', self.summary)
        with self.assertRaises(ValueError): report.build_assessment(self.baseline,self.repo,corpus_dir=q)

    def test_freeze_chronology_and_source_only_attestations_fail_closed(self):
        q = self.enrich()
        original_truth = json.loads((q/'truth.json').read_text())
        original_reviewer = json.loads((q/'b.json').read_text())
        for mutation in ['invalid_time', 'naive_time', 'labels_after_freeze', 'results_seen', 'other_seen', 'protocol']:
            b = copy.deepcopy(original_reviewer)
            if mutation == 'invalid_time': b['timestampUtc'] = 'yesterday'
            if mutation == 'naive_time': b['timestampUtc'] = '2025-12-31T00:00:00'
            if mutation == 'labels_after_freeze': b['timestampUtc'] = '2026-01-03T00:00:00Z'
            if mutation == 'results_seen': b['scannerResultsSeen'] = True
            if mutation == 'other_seen': b['otherLabelsSeen'] = True
            if mutation == 'protocol': b['protocolSha256'] = '0'*64
            save(q/'b.json', b)
            truth = copy.deepcopy(original_truth)
            truth['labelSources']['b.json'] = evaluate.bytes_digest(q/'b.json')
            save(q/'truth.json', truth)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): evaluate.load_truth(q)
        save(q/'b.json', original_reviewer)
        save(q/'truth.json', original_truth)
        self.summary['environment']['clockUtc'] = '2025-12-30T00:00:00Z'
        save(self.baseline/'summary.json', self.summary)
        with self.assertRaisesRegex(ValueError, 'precedes'):
            report.build_assessment(self.baseline,self.repo,corpus_dir=q)

    def test_supplemental_fact_corrections_bind_both_reviewers_and_original_records(self):
        q = self.enrich()
        truth = json.loads((q/'truth.json').read_text())
        records = {}
        for who in ['a','b']:
            records[who] = dict(reviewer=who, timestampUtc='2025-12-31T01:00:00Z', sourceOnly=True,
                otherLabelsSeen=False, scannerResultsSeen=False, originalLabelsSha256=truth['labelSources'][f'{who}.json'])
            save(q/f'fact-{who}.json',records[who])
        truth['sourceFactReviewBindings'] = {f'fact-{who}.json':evaluate.bytes_digest(q/f'fact-{who}.json') for who in ['a','b']}
        save(q/'truth.json', truth)
        evaluate.load_truth(q)
        for mutation in ['results_seen','late','wrong_original','other_valid_original','wrong_identity','bytes_changed']:
            row = copy.deepcopy(records['b'])
            changed = copy.deepcopy(truth)
            if mutation == 'results_seen': row['scannerResultsSeen'] = True
            if mutation == 'late': row['timestampUtc'] = '2026-01-04T00:00:00Z'
            if mutation == 'wrong_original': row['originalLabelsSha256'] = '0'*64
            if mutation == 'other_valid_original': row['originalLabelsSha256'] = truth['labelSources']['a.json']
            if mutation == 'wrong_identity': row['reviewer'] = 'a'
            if mutation == 'bytes_changed': row['note'] = 'An altered record without a new binding'
            save(q/'fact-b.json',row)
            if mutation != 'bytes_changed':
                changed['sourceFactReviewBindings']['fact-b.json'] = evaluate.bytes_digest(q/'fact-b.json')
            save(q/'truth.json',changed)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): evaluate.load_truth(q)

    def test_unannotated_fp_is_not_claimed_as_established_product_false_positive(self):
        q = self.enrich()
        data, labels = evaluate.load_truth(q)
        for i in range(2):
            directory = self.baseline/f'accuracy-{i}'
            raw = json.loads((directory/'report.json').read_text())
            raw['findings'] += [dict(rule_id='API', location=dict(file='a.js',line=1,column=c)) for c in [6,8]]
            save(directory/'report.json',raw)
            sample = self.summary['repeatability']['samples'][i]
            sample.update(semanticDigest=evaluate.semantic_digest(raw),orderDigest=evaluate.order_digest(raw),findingCount=3)
            save(directory/'measurement.json',sample)
        self.summary['accuracy'] = evaluate.score(data,labels,raw['findings'])
        save(self.baseline/'summary.json',self.summary)
        result = report.build_assessment(self.baseline,self.repo,corpus_dir=q)
        interpretation = result['annotationInterpretation']
        self.assertEqual(interpretation['status'],'Partial')
        self.assertEqual(interpretation['matchedNegativePredictionCount'],1)
        self.assertEqual(interpretation['unattributedPredictionCount'],1)
        self.assertEqual(result['byArtifact']['tuning/source']['fp'],2)
        self.assertEqual(result['byAlgorithm']['tuning/unattributed']['fp'],1)
        output = self.repo/'partial-report'
        report.write_assessment(result,output)
        self.assertIn('製品の誤検出とは断定できません',(output/'quality.md').read_text())
        for name in ['metrics','comparison']:
            saved = json.loads((output/f'{name}.json').read_text())
            self.assertEqual(saved['annotationInterpretation'],interpretation)

    def test_zero_predictions_cannot_verify_annotation_completeness(self):
        q = self.enrich()
        data, labels = evaluate.load_truth(q)
        for i in range(2):
            directory = self.baseline/f'accuracy-{i}'
            raw = dict(findings=[])
            save(directory/'report.json', raw)
            sample = self.summary['repeatability']['samples'][i]
            sample.update(semanticDigest=evaluate.semantic_digest(raw), orderDigest=evaluate.order_digest(raw), findingCount=0)
            save(directory/'measurement.json', sample)
        self.summary['accuracy'] = evaluate.score(data, labels, [])
        save(self.baseline/'summary.json', self.summary)
        result = report.build_assessment(self.baseline, self.repo, corpus_dir=q)
        self.assertEqual(result['annotationInterpretation']['status'], 'Partial')
        self.assertEqual(result['annotationInterpretation']['unattributedPredictionCount'], 0)
        self.assertIsNone(result['byAlgorithm']['tuning/RSA']['precision'])

    def test_metadata_replay_is_visible_in_machine_and_human_reports(self):
        result=report.build_assessment(self.baseline,self.repo)
        result['newDataEvaluation']={'kind':'Metadata-only replay; not a new independent trial',
            'metadataReplay':{'reason':'Correct upstream versus serialized source hashes'}}
        output=self.repo/'metadata-replay-output';report.write_assessment(result,output)
        self.assertIn('not a new independent trial',(output/'quality.json').read_text())
        text=(output/'quality.md').read_text()
        self.assertIn('新しい独立試行ではありません',text)
        self.assertIn('Correct upstream versus serialized source hashes',text)


if __name__ == '__main__':
    unittest.main()
