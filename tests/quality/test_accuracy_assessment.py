"""Behavior checks for sealed evidence admission and finite-population inference."""
import copy
import importlib.util
import itertools
import json
import os
import pathlib
import tempfile
import unittest
from fractions import Fraction

MODULE = pathlib.Path(__file__).resolve().parents[2] / 'benchmark' / 'accuracy_assessment.py'
SPEC = importlib.util.spec_from_file_location('accuracy_assessment', MODULE)
a = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(a)
QUALITY = pathlib.Path(os.environ['PQC_CANONICAL_QUALITY'])


def save(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    return path


class HypergeometricTests(unittest.TestCase):
    def test_exhaustive_small_population_sample_outcomes_and_coverage(self):
        for population in range(1, 11):
            for sample in range(1, population + 1):
                for successes in range(sample + 1):
                    result = a.hypergeometric_interval(population, sample, successes)
                    accepted = []
                    for k in range(population + 1):
                        outcomes = [sum(i < k for i in draw) for draw in itertools.combinations(range(population), sample)]
                        upper = Fraction(sum(x >= successes for x in outcomes), len(outcomes))
                        lower = Fraction(sum(x <= successes for x in outcomes), len(outcomes))
                        if upper >= Fraction(1, 40) and lower >= Fraction(1, 40):
                            accepted.append(k)
                    self.assertEqual(result['populationSuccessBounds'], [min(accepted), max(accepted)])
                for k in range(population + 1):
                    outcomes = [sum(i < k for i in draw) for draw in itertools.combinations(range(population), sample)]
                    intervals = {x: a.hypergeometric_interval(population, sample, x)['populationSuccessBounds'] for x in set(outcomes)}
                    covered = sum(intervals[x][0] <= k <= intervals[x][1] for x in outcomes)
                    self.assertGreaterEqual(Fraction(covered, len(outcomes)), Fraction(95, 100))

    def test_census_and_extremes(self):
        for successes in range(11):
            self.assertEqual(a.hypergeometric_interval(10, 10, successes)['populationSuccessBounds'], [successes, successes])
        self.assertEqual(a.hypergeometric_interval(100, 1, 0)['populationSuccessBounds'], [0, 97])
        self.assertEqual(a.hypergeometric_interval(100, 1, 1)['populationSuccessBounds'], [3, 100])
        self.assertEqual(a.hypergeometric_interval(100, 10, 5)['populationSuccessBounds'], [20, 80])

    def test_large_population_reference_and_unknown_policy(self):
        interval = a.hypergeometric_interval(91231, 300, 280)
        self.assertEqual(interval['populationSuccessBounds'], [82014, 87469])
        result = a.claim_statistics(['supported', 'refuted', 'unknown'], 100, 3, True)
        self.assertEqual(result['identificationBounds'], [1 / 3, 2 / 3])
        self.assertEqual(result['resolvedConditionalPrecision'], 0.5)
        self.assertIsNone(result['confidenceInterval'])
        self.assertIsNone(result['pointPrecision'])
        self.assertFalse(result['samplingQualified'])
        self.assertIsNone(a.claim_statistics(['supported'])['confidenceInterval'])

    def test_invalid_counts(self):
        for args in ((0, 0, 0), (5, 6, 1), (5, 2, 3), (True, 1, 1), (5, -1, 0)):
            with self.assertRaises(ValueError):
                a.hypergeometric_interval(*args)


class AdmissionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self.temp.name).resolve()
        self.protocol = save(self.root / 'protocol.json', {'version': 1, 'studyId': 'unit-study',
                             'primaryClaims': {'sampleCount': 3, 'populationCount': 10}})
        self.scope = save(self.root / 'scope.json', {'frozen': True})
        self.source = self.root / 'a.js'
        self.source.write_text('rsa.sign(key)\nno_crypto()\n', encoding='utf-8')
        self.evidence = a.reference(self.source)
        self.native = {'rule_id': 'API_NODE_CRYPTO_SIGN_RSA', 'location': {'file': 'a.js', 'line': 1, 'column': 1},
                       'description': 'RSA signing reference', 'evidence': {'metadata': {'algorithm': 'RSA'}}}
        self.report = save(self.root / 'report.json', {'generated_at': 'historical', 'findings': [self.native] * 3})
        self.rater_paths = []

    def tearDown(self):
        self.temp.cleanup()

    def historical_fixture(self):
        oss, _ = a.load_historical(QUALITY)
        manifest = save(self.root / 'manifest.json', [{'path': 'a.js', 'mode': '100644', 'bytes': self.source.stat().st_size,
                                                      'sha256': self.evidence['sha256']}])
        entries = {identity: {'id': identity, 'population': population, 'commit': 'a' * 40, 'split': 'holdout',
                             'inputManifest': str(manifest), 'inputManifestSha256': a.reference(manifest)['sha256']}
                   for identity, population in [('r', 'primary'), ('c', 'comparison')]}
        self.registry = save(self.root / 'registry.json', {'repositories': list(entries.values())})
        binary = self.root / 'scanner'; binary.write_bytes(b'fixture binary identity only')
        self.rule_bindings = [a.reference(path) for path in sorted((QUALITY.parents[1] / 'rules/default').glob('*.yml'))]
        rules = {'default/' + pathlib.Path(item['path']).name: item['sha256'] for item in self.rule_bindings}
        scanner = {'binary': a.reference(binary), 'rules': rules, 'rulesSha256': a.digest(rules)}
        configuration = {'repeats': 1, 'requiredArtifacts': ['report.json', 'diagnostics.json', 'scan.inventory.v1.json']}
        environment = {name: 'synthetic' for name in ('python', 'platform', 'machine', 'cpuCount', 'hardware', 'rust', 'cargo')}
        self.historical_reports, self.invocation_paths = {}, {}
        policy_rows = []
        for population, identity in [('primary', 'r'), ('comparison', 'c')]:
            for phase in ('baseline', 'after'):
                name = population + '-' + phase
                directory = self.root / name; output = directory / 'attempt/output'; output.mkdir(parents=True, exist_ok=True)
                self.historical_reports[name] = save(output / 'report.json', {'generated_at': 'historical',
                       'findings': copy.deepcopy(getattr(self, 'source_findings', [self.native] * 3))})
                coverage = {key: 0 for key in ('read_failed_files', 'partial_files', 'analysis_failed_files', 'unsupported_files',
                            'not_applicable_files', 'unknown_files', 'enumeration_failed_entries')}
                coverage.update(selected_files=1, read_ok_files=1, analyzed_files=1)
                save(output / 'diagnostics.json', {'coverage': coverage})
                save(output / 'scan.inventory.v1.json', {'files': []})
                common = {'phase': phase, 'registry': a.reference(self.registry), 'protocol': {'synthetic': True},
                          'configuration': configuration, 'scanner': scanner, 'harness': {'tests/quality/oss.py': a.reference(QUALITY / 'oss.py')['sha256']},
                          'coordinatorEnvironment': environment, 'selectedIds': [identity], 'selectedEntries': {identity: entries[identity]}, 'expectedSamples': 1}
                save(directory / 'started.json', common)
                request = {'registryEntry': entries[identity], 'configuration': configuration, 'scanner': scanner,
                           'protocol': common['protocol'], 'harness': common['harness'], 'environment': environment,
                           'sample': 0, 'phase': phase, 'input': {'root': str(self.root), 'manifest': a.reference(manifest), 'commit': 'a' * 40}}
                started = {'repositoryId': identity, 'phase': phase, 'bindings': request}
                save(output.parent / 'started.json', started)
                artifacts, assessment = oss._artifacts(output, configuration['requiredArtifacts'])
                terminal = save(output.parent / 'terminal.json', {'execution': 'Succeeded', 'startedDigest': a.digest(started),
                                     'executionKey': a.digest(request), 'artifacts': artifacts, 'assessment': assessment})
                invocation = dict(common, rows=[{'repositoryId': identity, 'sample': 0, 'population': population, 'split': 'holdout',
                                                'execution': 'Succeeded', 'receipt': a.reference(terminal), 'assessment': assessment}])
                receipt = save(directory / 'invocation.json', invocation)
                wrapper = save(self.root / (name + '.json'), dict(invocation, receiptPath=str(receipt)))
                self.invocation_paths[name] = wrapper
                policy_rows.append({'id': name, 'population': population, 'phase': phase, 'binding': a.reference(wrapper)})
        anchor = a.reference(self.invocation_paths['primary-after'])
        selection = save(self.root / 'selection.json', {'invocation': anchor, 'scannerBinary': scanner['binary']})
        protocol = a.load(self.protocol); protocol['primaryClaims'].update(sample=a.reference(selection), binarySha256=scanner['binary']['sha256'])
        save(self.protocol, protocol)
        self.policy_value = {'version': 1, 'kind': 'historical-score-policy', 'studyId': 'unit-study', 'baseProtocol': a.reference(self.protocol),
                             'registry': a.reference(self.registry), 'primaryAfterAnchor': anchor, 'projectionRulesInvocationId': 'primary-after', 'invocations': policy_rows}

    def original_predictions(self, invocation_id='primary-after'):
        return self.historical_reports[invocation_id]

    def packet(self, kind):
        if kind == 'source':
            self.historical_fixture()
        common = {'repositoryId': 'r', 'population': 'primary', 'group': 'g', 'language': 'JavaScript', 'split': 'holdout'}
        if kind == 'claim':
            rows = [dict(common, id=f'c{i}', sampleId=f'c{i}', nativeFinding=copy.deepcopy(self.native),
                         findingSha256=a.digest(self.native), reportPath=str(self.report), reportSha256=a.reference(self.report)['sha256'],
                         nativeOrdinal=i, assertionChecklist=[{'id': 'a', 'fieldPath': 'description'}]) for i in range(3)]
        else:
            rows = [dict(common, id='s1', sourcePath='a.js', sourceRoot=str(self.root), sourceSha256=self.evidence['sha256'],
                         commit='a' * 40, sourceStatus='ready', ownedLine=1, contextStart=1, contextEnd=2, contextSafeText='rsa.sign(key)')]
        value = {'version': 1, 'studyId': 'unit-study', 'kind': kind, 'protocolSha256': a.reference(self.protocol)['sha256'],
                 'scopeBinding': a.reference(self.scope), 'provenance': {'predictionsSeen': kind == 'claim', 'otherLabelsSeen': False},
                 'records': rows}
        if kind == 'source':
            value['scopeBinding'] = dict(a.reference(self.scope), registrySha256=a.reference(self.registry)['sha256'])
        self.packet_path = save(self.root / 'packet.json', value)
        if kind == 'source':
            self.scoring_policy = save(self.root / 'policy.json', dict(self.policy_value, sourcePacket=a.reference(self.packet_path)))
        return value

    def fact(self, identity='f1', occurrence='o1', kind='positive_fact'):
        return {'factId': identity, 'occurrenceId': occurrence, 'kind': kind, 'question': 'operation_role', 'inScope': True, 'algorithm': 'RSA',
                'role': 'sign', 'parameters': None, 'use': None, 'anchor': {'line': 1, 'startColumn': 1, 'endColumn': 9},
                'sourceEvidence': [dict(self.evidence, line=1, description='Source definition')], 'primaryReferenceKeys': ['pinned-source-definition'], 'reason': 'Source definition'}

    def labels(self, packet, verdict='supported'):
        if packet['kind'] == 'claim':
            rows = [{'id': r['id'], 'verdict': verdict, 'assertions': [{'id': 'a', 'claim': 'RSA signing reference', 'verdict': verdict,
                     'sourceEvidence': [dict(self.evidence, line=1, description='Source definition')], 'primaryReferenceKeys': ['source-definition'], 'reason': 'Source definition'}],
                     'contextBindings': [self.evidence]} for r in packet['records']]
        else:
            rows = [{'id': 's1', 'complete': True, 'emptyReason': '', 'facts': [self.fact()], 'contextBindings': [self.evidence]}]
        common = {'version': 1, 'kind': packet['kind'], 'studyId': packet['studyId'], 'protocolSha256': packet['protocolSha256'],
                  'packetSha256': a.reference(self.packet_path)['sha256'], 'model': 'test-independent-model',
                  'predictionsSeen': packet['kind'] == 'claim', 'otherLabelsSeen': False, 'assessments': rows}
        self.rater_paths = [save(self.root / f'rater{i}.json', dict(common, reviewer=f'rater{i}')) for i in (1, 2)]
        adjudication = dict(common, reviewer='adjudicator', otherLabelsSeen=True,
                            raterBindings=[a.reference(p) for p in self.rater_paths])
        self.adjudication = save(self.root / 'adjudication.json', adjudication)
        return adjudication

    def source_projection(self, packet):
        unit = {'id': 'u1', 'recordId': 's1', 'path': 'a.js', 'ruleId': 'API_NODE_CRYPTO_SIGN_RSA', 'line': 1,
                'startColumn': 1, 'endColumn': 9, 'label': 'positive'}
        projections = [{'raterBinding': a.reference(p), 'labels': [{'id': 'u1', 'label': 'positive'}],
                        'sourceCensusSha256': a.digest(a.load(p)['assessments']), 'projectionAuthor': 'mapper'} for p in self.rater_paths]
        value = {'version': 1, 'kind': 'source-projection', 'studyId': packet['studyId'],
                 'protocolSha256': packet['protocolSha256'], 'packetSha256': a.reference(self.packet_path)['sha256'],
                 'adjudicationBinding': a.reference(self.adjudication), 'reviewer': 'mapper', 'model': 'test-model',
                 'predictionsSeen': False, 'otherLabelsSeen': True, 'ruleBindings': self.rule_bindings,
                 'families': {'api': ['API_NODE_CRYPTO_SIGN_RSA']}, 'units': [unit], 'raterProjections': projections,
                 'facts': [{'factId': 'f1', 'mapping': {'status': 'mapped', 'predicate': 'any', 'unitIds': ['u1']}}],
                 'contextBindings': [self.evidence]}
        self.projection_path = save(self.root / 'projection.json', value)
        check = {k: v for k, v in value.items() if k in {'version', 'studyId', 'protocolSha256', 'packetSha256', 'adjudicationBinding',
                 'model', 'predictionsSeen', 'otherLabelsSeen', 'contextBindings'}}
        check.update(kind='source-projection-check', reviewer='mapping-checker', projectionBinding=a.reference(self.projection_path),
                     accepted=True, reason='Independent source contract review')
        self.check_path = save(self.root / 'check.json', check)
        return value

    def admit_source(self, output='source-out'):
        return a.admit_source(self.protocol, self.packet_path, self.rater_paths, self.adjudication,
                              self.projection_path, self.check_path, QUALITY, self.root / output, self.scoring_policy)

    def test_nested_enriched_bindings_reject_drift_and_symlinks(self):
        valid = dict(self.evidence, nested={'items': [self.evidence]})
        a.verify_scope(valid)
        with self.assertRaises(ValueError):
            a.verify_scope(dict(self.evidence, nested={'items': [dict(self.evidence, sha256='0' * 64)]}))
        with self.assertRaises(ValueError):
            a.verify_scope(dict(self.evidence, nested={'path': str(self.root / '..' / self.root.name / 'a.js'), 'sha256': self.evidence['sha256']}))
        link = self.root / 'nested-link'; link.symlink_to(self.source)
        with self.assertRaises(ValueError):
            a.verify_scope(dict(self.evidence, nested={'path': str(link), 'sha256': self.evidence['sha256']}))

    def test_public_score_omits_secret_bodies(self):
        canary = 'password=CANARY_SOURCE_SECRET_7411'
        self.source_findings = [dict(self.native, rule_id='OTHER', source_snippet=canary, evidence={'metadata': {'password': canary}}),
                                dict(self.native, location={'file': 'a.js', 'line': 2, 'column': 1}, source_snippet=canary,
                                     evidence={'metadata': {'private_token': canary}})]
        packet = self.packet('source'); self.labels(packet); self.source_projection(packet); self.admit_source()
        result = a.score_source(self.root / 'source-out/admission.json', 'r', self.original_predictions(), QUALITY, self.root / 'secret-score.json', 'primary-after')
        self.assertNotIn(canary, json.dumps(result))
        self.assertNotIn(canary, (self.root / 'secret-score.json').read_text())
        self.assertEqual(result['canonicalScore']['other_rulesCount'], 1)
        self.assertEqual(result['canonicalScore']['unattributedCount'], 1)

    def test_frozen_prediction_provenance_rechecked_on_every_use(self):
        packet = self.packet('source'); self.labels(packet); self.source_projection(packet); self.admit_source()
        admission_path = self.root / 'source-out/admission.json'
        admission = a.load(admission_path)
        original = self.original_predictions()
        a.score_source(admission_path, 'r', original, QUALITY, self.root / 'initial-score.json', 'primary-after')
        for name, path, invocation in [('copy', save(self.root / 'copy.json', a.load(original)), 'primary-after'),
                                       ('other-phase', self.original_predictions('primary-baseline'), 'primary-after'),
                                       ('wrong-repo', self.original_predictions('comparison-after'), 'primary-after'),
                                       ('population', original, 'comparison-after'), ('undeclared', original, 'foreign-after')]:
            with self.subTest(name=name), self.assertRaises(ValueError):
                a.score_source(admission_path, 'r', path, QUALITY, self.root / (name + '-score.json'), invocation)
        snapshot = admission['historicalInputs']['invocations']['primary-after']
        row = snapshot['rows']['r']
        targets = [('wrapper', snapshot['binding']['path']), ('invocation', snapshot['receipt']['path']),
                   ('invocation-started', snapshot['started']['path']), ('terminal', row['terminal']['path']),
                   ('attempt-started', row['started']['path']), ('report', row['report']['path']),
                   ('manifest', row['inputManifest']['path'])]
        for name, filename in targets:
            path = pathlib.Path(filename); before = path.read_bytes()
            try:
                path.write_bytes(b'{"findings": []}' if name == 'report' else before + b' ')
                with self.subTest(name=name), self.assertRaises(ValueError):
                    a.score_source(admission_path, 'r', original, QUALITY, self.root / (name + '-score.json'), 'primary-after')
            finally:
                path.write_bytes(before)
        binary = pathlib.Path(a.load(snapshot['binding']['path'])['scanner']['binary']['path']); before = binary.read_bytes()
        try:
            binary.write_bytes(b'other binary')
            with self.assertRaises(ValueError):
                a.score_source(admission_path, 'r', original, QUALITY, self.root / 'binary-score.json', 'primary-after')
        finally:
            binary.write_bytes(before)
        result = a.score_source(admission_path, 'r', self.original_predictions('primary-baseline'), QUALITY, self.root / 'baseline-score.json', 'primary-baseline')
        self.assertEqual(result['predictionProvenance']['invocationId'], 'primary-baseline')

    def test_rule_file_set_requires_all_real_hashes(self):
        packet = self.packet('source'); self.labels(packet); original = self.source_projection(packet)
        for mutation in ('missing', 'duplicate', 'extra', 'hash', 'id'):
            projection = copy.deepcopy(original)
            if mutation == 'missing': projection['ruleBindings'].pop()
            if mutation == 'duplicate': projection['ruleBindings'].append(projection['ruleBindings'][0])
            if mutation == 'extra':
                fake = self.root / 'fake.yml'; fake.write_text('- id: VIRTUAL_UNREVIEWED_RULE\n')
                projection['ruleBindings'].append(a.reference(fake))
            if mutation == 'hash': projection['ruleBindings'][0]['sha256'] = '0' * 64
            if mutation == 'id': projection['families']['api'] = ['VIRTUAL_UNREVIEWED_RULE']
            save(self.projection_path, projection)
            check = a.load(self.check_path); check['projectionBinding'] = a.reference(self.projection_path); save(self.check_path, check)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): self.admit_source(mutation)
        self.assertEqual(len(self.rule_bindings), 13)

    def test_fabricated_yaml_cannot_extend_product_registry(self):
        packet = self.packet('source'); self.labels(packet); projection = self.source_projection(packet)
        fake = self.root / 'fake.yml'; fake.write_text('- id: VIRTUAL_UNREVIEWED_RULE\n')
        projection['ruleBindings'] = [a.reference(fake)]
        projection['families'] = {'api': ['VIRTUAL_UNREVIEWED_RULE']}
        projection['units'][0]['ruleId'] = 'VIRTUAL_UNREVIEWED_RULE'
        save(self.projection_path, projection)
        check = a.load(self.check_path); check['projectionBinding'] = a.reference(self.projection_path); save(self.check_path, check)
        with self.assertRaises(ValueError): self.admit_source()

    def test_empty_unbound_predictions_rejected(self):
        packet = self.packet('source'); self.labels(packet); self.source_projection(packet); self.admit_source()
        empty = save(self.root / 'empty.json', {'findings': []})
        with self.assertRaises(ValueError):
            a.score_source(self.root / 'source-out/admission.json', 'r', empty, QUALITY, self.root / 'empty-score.json', 'primary-after')

    def test_unrepresentable_positive_only_proves_lower_fraction(self):
        census = [{'id': 's', 'complete': True, 'facts': [self.fact()]}]
        mappings = [{'factId': 'f1', 'mapping': {'status': 'canonical_unrepresentable', 'predicate': 'any', 'unitIds': []}}]
        stats = a.source_statistics(census, mappings, {'accuracy': {'details': []}})
        self.assertEqual(stats.get('canonicalProvenLowerFraction'), 0.0)
        self.assertIsNone(stats.get('finiteKnownSourceCapabilityCoverage'))
        self.assertEqual(stats['mappedMissFactIds'], [])
        grouped = a.aggregate_source_scores([{'kind': 'source-assessment-score', 'repositoryId': 'r', 'population': 'primary', 'sourceStatistics': stats}])
        self.assertEqual(grouped['primary'].get('canonicalProvenLowerFraction'), 0.0)
        self.assertIsNone(grouped['primary'].get('finiteKnownSourceCapabilityCoverage'))

    def test_claim_atomic_native_scoring_and_original_bytes_survive(self):
        packet = self.packet('claim')
        self.labels(packet)
        originals = [p.read_bytes() for p in self.rater_paths]
        receipt = a.admit_claims(self.protocol, self.packet_path, self.rater_paths, self.adjudication, QUALITY, self.root / 'out')
        self.assertEqual(receipt['statistics']['primary']['T'], 3)
        self.assertTrue(all(r['canonicalScore']['details'][0]['disposition'] == 'tp' for r in receipt['rows']))
        self.assertEqual(receipt['duplicateEmissions']['extraSampleRecords'], 2)
        self.assertEqual([a.read_bytes(self.root / 'out' / f'rater-{i}.json') for i in (1, 2)], originals)
        with self.assertRaises(FileExistsError):
            a.admit_claims(self.protocol, self.packet_path, self.rater_paths, self.adjudication, QUALITY, self.root / 'out')

    def test_native_fields_or_exposure_or_assertion_strength_cannot_change(self):
        packet = self.packet('claim')
        self.labels(packet)
        for mutation in ('field', 'exposure', 'conjunction', 'checklist', 'evidence', 'identity'):
            with self.subTest(mutation=mutation):
                packet = self.packet('claim'); self.labels(packet)
                if mutation == 'field':
                    changed = a.load(self.packet_path); changed['records'][0]['nativeFinding']['description'] = 'Weaker claim'; save(self.packet_path, changed)
                else:
                    changed = a.load(self.rater_paths[0])
                    if mutation == 'exposure': changed['otherLabelsSeen'] = True
                    if mutation == 'conjunction': changed['assessments'][0]['assertions'][0]['verdict'] = 'refuted'
                    if mutation == 'checklist': changed['assessments'][0]['assertions'][0]['id'] = 'missing'
                    if mutation == 'evidence': changed['assessments'][0]['assertions'][0]['sourceEvidence'] = []
                    if mutation == 'identity': changed['reviewer'] = 'rater2'
                    save(self.rater_paths[0], changed)
                    adj = a.load(self.adjudication); adj['raterBindings'] = [a.reference(p) for p in self.rater_paths]; save(self.adjudication, adj)
                with self.assertRaises(ValueError):
                    a.admit_claims(self.protocol, self.packet_path, self.rater_paths, self.adjudication, QUALITY, self.root / mutation)

    def test_unknown_claim_retained_and_unrepresentable_line_zero_is_unknown(self):
        self.native['location'].update(line=0, column=0)
        save(self.report, {'findings': [self.native] * 3})
        packet = self.packet('claim'); self.labels(packet, 'unknown')
        receipt = a.admit_claims(self.protocol, self.packet_path, self.rater_paths, self.adjudication, QUALITY, self.root / 'unknown')
        self.assertEqual(receipt['statistics']['primary']['U'], 3)
        self.assertIsNone(receipt['statistics']['primary']['confidenceInterval'])
        self.assertTrue(all(r['canonicalScore'] is None for r in receipt['rows']))

    def test_source_native_scorer_and_duplicate_batch_semantics(self):
        self.source_findings = [self.native, self.native, dict(self.native, rule_id='OTHER'),
                                dict(self.native, location={'file': 'a.js', 'line': 2, 'column': 1})]
        packet = self.packet('source'); self.labels(packet); self.source_projection(packet)
        receipt = self.admit_source()
        predictions = self.original_predictions()
        result = a.score_source(self.root / 'source-out' / 'admission.json', 'r', predictions, QUALITY, self.root / 'score.json', 'primary-after')
        self.assertEqual((result['sourceStatistics']['D'], result['sourceStatistics']['S']), (1, 1))
        canonical_score = result['canonicalScore']
        self.assertEqual([r['disposition'] for r in canonical_score['accuracy']['details']], ['tp', 'fp'])
        self.assertEqual(len(canonical_score['other_rules']), 1)
        self.assertEqual(len(canonical_score['unattributed']), 1)
        data, labels = a.load_canonical(QUALITY)[0].load_truth(pathlib.Path(receipt['components'][0]['directory']))
        expected = a.load_canonical(QUALITY)[1].score_annotated(data, labels, a.load(predictions)['findings'])
        self.assertEqual(canonical_score['accuracy'], expected['accuracy'])
        self.assertEqual(canonical_score, a.public_score(expected))
        projected = a.load(self.root / 'source-out' / 'r' / 'canonical-rater-1.json')
        self.assertFalse(projected['sourceRaterAuthoredRuleLabels'])

    def test_source_rejects_fake_rule_bad_mapping_wrong_owner_and_missing_checker(self):
        for mutation in ('fake_rule', 'wrong_owner', 'orphan', 'checker', 'rater_projection', 'anchor'):
            with self.subTest(mutation=mutation):
                packet = self.packet('source'); self.labels(packet); projection = self.source_projection(packet)
                if mutation == 'fake_rule': projection['families']['api'] = ['VIRTUAL']
                if mutation == 'wrong_owner': projection['units'][0]['recordId'] = 'other'
                if mutation == 'orphan': projection['facts'][0]['mapping'].update(status='no_rule', unitIds=[])
                if mutation == 'rater_projection': projection['raterProjections'][0]['sourceCensusSha256'] = '0' * 64
                if mutation == 'anchor': projection['units'][0]['endColumn'] = 100
                save(self.projection_path, projection)
                check = a.load(self.check_path); check['projectionBinding'] = a.reference(self.projection_path)
                if mutation == 'checker': check['accepted'] = False
                save(self.check_path, check)
                with self.assertRaises(ValueError): self.admit_source(mutation)

    def test_drift_symlink_duplicate_keys_and_no_overwrite(self):
        packet = self.packet('source'); self.labels(packet); self.source_projection(packet); self.admit_source()
        predictions = save(self.root / 'predictions.json', {'findings': []})
        self.source.write_text('changed\n')
        with self.assertRaisesRegex(ValueError, 'drift'):
            a.score_source(self.root / 'source-out' / 'admission.json', 'r', predictions, QUALITY, self.root / 'score.json', 'primary-after')
        link = self.root / 'link'; link.symlink_to(self.source)
        with self.assertRaises(ValueError): a.reference(link)
        with self.assertRaises(ValueError): a.decode(b'{"x":1,"x":2}')
        with self.assertRaises(ValueError): a.relative('../escape')

    def test_many_to_many_distinct_fact_coverage_and_unknowns(self):
        facts = [self.fact('f1'), self.fact('f2'), self.fact('f3', 'o2'), self.fact('f4', 'o3', 'unknown')]
        facts.append(dict(self.fact('outside', 'o4'), inScope=False))
        census = [{'id': 's', 'complete': False, 'facts': facts}]
        mappings = [{'factId': 'f1', 'mapping': {'status': 'mapped', 'predicate': 'any', 'unitIds': ['u1', 'u2']}},
                    {'factId': 'f2', 'mapping': {'status': 'mapped', 'predicate': 'all', 'unitIds': ['u1', 'u2']}},
                    {'factId': 'f3', 'mapping': {'status': 'no_rule', 'predicate': 'any', 'unitIds': []}},
                    {'factId': 'f4', 'mapping': {'status': 'unknown', 'predicate': 'any', 'unitIds': []}},
                    {'factId': 'outside', 'mapping': {'status': 'canonical_unrepresentable', 'predicate': 'any', 'unitIds': []}}]
        score = {'accuracy': {'details': [{'unit': 'u1', 'disposition': 'tp'}, {'unit': 'u1', 'disposition': 'fp'}]}}
        result = a.source_statistics(census, mappings, score)
        self.assertEqual((result['D'], result['S'], result['positiveOccurrenceCount']), (1, 3, 2))
        self.assertEqual(result['mappedMissFactIds'], ['f2'])
        self.assertEqual(result['unknownFactIds'], ['f4'])
        self.assertEqual(result['incompleteOwnedLineIds'], ['s'])
        self.assertEqual(result['outsideMigrationScopeFactIds'], ['outside'])
        self.assertIsNone(result['broadRecallUpperBound'])

    def test_comparison_claims_stay_separate_and_are_not_srs_qualified(self):
        packet = self.packet('claim')
        for row in packet['records']:
            row['population'] = 'comparison'
        save(self.packet_path, packet)
        protocol = a.load(self.protocol); protocol['comparisonClaims'] = {'sampleCount': 3}; save(self.protocol, protocol)
        packet['protocolSha256'] = a.reference(self.protocol)['sha256']; save(self.packet_path, packet)
        self.labels(packet)
        result = a.admit_claims(self.protocol, self.packet_path, self.rater_paths, self.adjudication, QUALITY, self.root / 'comparison')
        self.assertEqual(set(result['statistics']), {'comparison'})
        self.assertIsNone(result['statistics']['comparison']['confidenceInterval'])

    def test_empty_census_generates_no_rule_negatives(self):
        canary = 'SOURCE_NO_UNIT_CANARY_8634'
        self.source_findings = [dict(self.native, source_snippet=canary, evidence={'metadata': {'secret': canary}})] * 3
        packet = self.packet('source'); self.labels(packet); projection = self.source_projection(packet)
        for path in self.rater_paths + [self.adjudication]:
            record = a.load(path); record['assessments'][0].update(facts=[], emptyReason='No qualifying proposition starts on this owned line')
            save(path, record)
        adjudication = a.load(self.adjudication); adjudication['raterBindings'] = [a.reference(p) for p in self.rater_paths]; save(self.adjudication, adjudication)
        projection.update(adjudicationBinding=a.reference(self.adjudication), units=[], facts=[])
        for projected, path in zip(projection['raterProjections'], self.rater_paths):
            projected.update(raterBinding=a.reference(path), sourceCensusSha256=a.digest(a.load(path)['assessments']), labels=[])
        save(self.projection_path, projection)
        check = a.load(self.check_path); check.update(projectionBinding=a.reference(self.projection_path), adjudicationBinding=a.reference(self.adjudication)); save(self.check_path, check)
        receipt = self.admit_source()
        truth = a.load(pathlib.Path(receipt['components'][0]['directory']) / 'truth.json')
        self.assertEqual(truth['labels'], [])
        self.assertEqual(truth['sourceCensus'][0]['facts'], [])
        canary = 'SOURCE_NO_UNIT_CANARY_8634'
        internal = {'accuracy': a.load_canonical(QUALITY)[0].score({'families': {}, 'files': [], 'units': []}, {}, []),
                    'unattributed': [dict(self.native, source_snippet=canary, evidence={'metadata': {'secret': canary}})], 'other_rules': []}
        self.assertNotIn(canary, json.dumps(a.public_score(internal)))
        result = a.score_source(self.root / 'source-out/admission.json', 'r', self.original_predictions(), QUALITY, self.root / 'zero-unit-score.json', 'primary-after')
        self.assertEqual(result['canonicalScore']['unattributedCount'], 3)
        self.assertNotIn(canary, json.dumps(result))
        self.assertNotIn(canary, (self.root / 'zero-unit-score.json').read_text())
        self.assertEqual(result['sourceStatistics']['S'], 0)

    def test_population_aggregation_rejects_recounting(self):
        census = [{'id': 's1', 'complete': True, 'facts': [self.fact()]}]
        mapping = [{'factId': 'f1', 'mapping': {'status': 'no_rule', 'predicate': 'any', 'unitIds': []}}]
        stats = a.source_statistics(census, mapping, {'accuracy': {'details': []}})
        primary = {'kind': 'source-assessment-score', 'repositoryId': 'r', 'population': 'primary', 'sourceStatistics': stats}
        comparison = copy.deepcopy(primary); comparison.update(repositoryId='c', population='comparison')
        comparison['sourceStatistics']['scopeFactIds'] = ['comparison-f1']
        result = a.aggregate_source_scores([primary, comparison])
        self.assertEqual(result['primary']['S'], 1)
        self.assertEqual(result['comparison']['S'], 1)
        with self.assertRaises(ValueError): a.aggregate_source_scores([primary, primary])
        comparison['sourceStatistics']['scopeFactIds'] = ['f1']
        with self.assertRaises(ValueError): a.aggregate_source_scores([primary, comparison])

    def test_cli_admits_and_scores_without_overwriting(self):
        packet = self.packet('claim'); self.labels(packet)
        args = ['admit-claims', '--protocol', str(self.protocol), '--packet', str(self.packet_path),
                '--rater', str(self.rater_paths[0]), '--rater', str(self.rater_paths[1]),
                '--adjudication', str(self.adjudication), '--quality-dir', str(QUALITY), '--out', str(self.root / 'cli-output')]
        self.assertEqual(a.main(args), 0)
        self.assertTrue((self.root / 'cli-output' / 'assessment.json').is_file())
        self.assertEqual(a.main(args), 2)

    def test_source_cli_uses_declared_historical_policy_and_phase(self):
        packet = self.packet('source'); self.labels(packet); self.source_projection(packet)
        args = ['admit-source', '--protocol', str(self.protocol), '--packet', str(self.packet_path),
                '--rater', str(self.rater_paths[0]), '--rater', str(self.rater_paths[1]), '--adjudication', str(self.adjudication),
                '--projection', str(self.projection_path), '--mapping-check', str(self.check_path), '--scoring-policy', str(self.scoring_policy),
                '--quality-dir', str(QUALITY), '--out', str(self.root / 'cli-source')]
        self.assertEqual(a.main(args), 0)
        score = self.root / 'cli-source-score.json'
        args = ['score-source', '--admission', str(self.root / 'cli-source/admission.json'), '--repository-id', 'r',
                '--invocation-id', 'primary-after', '--predictions', str(self.original_predictions()),
                '--quality-dir', str(QUALITY), '--out', str(score)]
        self.assertEqual(a.main(args), 0)
        self.assertEqual(a.load(score)['sourceStatistics']['D'], 1)
        self.assertEqual(a.main(args), 2)

    def test_duplicate_source_propositions_rejected(self):
        packet = self.packet('source'); self.labels(packet)
        row = a.load(self.rater_paths[0])['assessments'][0]
        row['facts'].append(dict(row['facts'][0], factId='other-id'))
        with self.assertRaisesRegex(ValueError, 'Duplicate semantic'):
            a.validate_source(row, packet['records'][0])

    def test_source_question_and_types_cannot_be_invented(self):
        packet = self.packet('source'); self.labels(packet)
        original = a.load(self.rater_paths[0])['assessments'][0]
        for mutation in ('question', 'algorithm', 'parameters', 'anchor'):
            changed = copy.deepcopy(original)
            if mutation == 'question': changed['facts'][0]['question'] = 'whole_project_safe'
            if mutation == 'algorithm': changed['facts'][0]['algorithm'] = []
            if mutation == 'parameters': changed['facts'][0]['parameters'] = 'RSA2048'
            if mutation == 'anchor': changed['facts'][0]['anchor']['endColumn'] = 999
            with self.assertRaises(ValueError): a.validate_source(changed, packet['records'][0])

    def test_line_zero_projection_uses_existing_dependency_branch(self):
        self.native['location'].update(line=0, column=0)
        self.native['evidence']['metadata']['dep_name'] = 'crypto-dep'
        packet = self.packet('source'); self.labels(packet); projection = self.source_projection(packet)
        projection['units'][0].update(dependencyName='crypto-dep')
        save(self.projection_path, projection)
        check = a.load(self.check_path); check['projectionBinding'] = a.reference(self.projection_path); save(self.check_path, check)
        self.admit_source()
        predictions = self.original_predictions()
        result = a.score_source(self.root / 'source-out' / 'admission.json', 'r', predictions, QUALITY, self.root / 'score.json', 'primary-after')
        self.assertEqual(result['sourceStatistics']['D'], 1)
        self.assertEqual(result['canonicalScore']['accuracy']['details'][0]['disposition'], 'tp')


    def display_packet(self, kind='claim', native_mask=False):
        packet = self.packet(kind)
        self.labels(packet, 'unknown' if native_mask else 'supported')
        original_ref = a.reference(self.packet_path)
        view = copy.deepcopy(packet)
        for row in view['records']:
            if native_mask:
                row['nativeFinding']['description'] = '████'
            if kind == 'claim':
                row['nativeFindingRedacted'] = native_mask
            row['displayPrivacy'] = {'addedNativeMask':native_mask, 'addedContextMask':False}
        view['displayViewBinding'] = {'version':1,'originalPacket':original_ref}
        view_path = save(self.root / 'display-view.json',view)
        if kind == 'claim':
            view_ref = a.reference(view_path)
            for path in self.rater_paths:
                value=a.load(path);value['displayViewBinding']=view_ref;save(path,value)
            value=a.load(self.adjudication);value['displayViewBinding']=view_ref
            value['raterBindings']=[a.reference(path) for path in self.rater_paths]
            save(self.adjudication,value)
        self.packet_path=view_path
        return view,original_ref

    def test_display_view_preserves_original_claim_binding_and_private_copy(self):
        view,original_ref=self.display_packet(native_mask=True)
        result=a.admit_claims(self.protocol,self.packet_path,self.rater_paths,self.adjudication,QUALITY,self.root/'view-out')
        self.assertEqual(result['bindings']['packet'],original_ref)
        self.assertEqual(result['bindings']['displayView'],a.reference(self.packet_path))
        self.assertEqual((self.root/'view-out/packet.json').read_bytes(),self.packet_path.read_bytes())
        self.assertEqual((self.root/'view-out/private/original-packet.json').read_bytes(),a.bound(original_ref))
        self.assertEqual((self.root/'view-out').stat().st_mode & 0o777,0o700)
        self.assertEqual((self.root/'view-out/private/original-packet.json').stat().st_mode & 0o777,0o600)

    def test_display_view_requires_fresh_claim_label_binding(self):
        self.display_packet()
        value=a.load(self.rater_paths[0]);value.pop('displayViewBinding');save(self.rater_paths[0],value)
        adjud=a.load(self.adjudication);adjud['raterBindings']=[a.reference(path) for path in self.rater_paths];save(self.adjudication,adjud)
        with self.assertRaisesRegex(ValueError,'display view binding'):
            a.admit_inputs(self.protocol,self.packet_path,self.rater_paths,self.adjudication,'claim')

    def test_display_view_native_mask_cannot_be_fully_supported(self):
        view,_=self.display_packet(native_mask=True)
        for path in [*self.rater_paths,self.adjudication]:
            value=a.load(path)
            for row in value['assessments']:
                row['verdict']='supported'
                for assertion in row['assertions']: assertion['verdict']='supported'
            if path==self.adjudication: value['raterBindings']=[a.reference(p) for p in self.rater_paths]
            save(path,value)
        with self.assertRaisesRegex(ValueError,'cannot be admitted fully supported'):
            a.admit_inputs(self.protocol,self.packet_path,self.rater_paths,self.adjudication,'claim')

    def test_display_view_mask_flags_and_identity_are_verified(self):
        view,_=self.display_packet(native_mask=True)
        for change in ('flag','identity'):
            modified=copy.deepcopy(view)
            if change=='flag': modified['records'][0]['nativeFindingRedacted']=False
            else: modified['records'][0]['nativeFinding']['location']['line']=2
            with self.assertRaises(ValueError):
                a.display_ancestry(modified,a.reference(self.packet_path))

    def test_source_display_view_accepts_historical_gold_header(self):
        view,original_ref=self.display_packet('source')
        _,packet,_,_,bindings=a.admit_inputs(self.protocol,self.packet_path,self.rater_paths,self.adjudication,'source')
        self.assertEqual(packet['_sha256'],original_ref['sha256'])
        self.assertEqual(bindings['packet'],original_ref)

    def test_context_only_claim_mask_does_not_force_unknown(self):
        view,_=self.display_packet()
        row=view['records'][0]
        row['contextSafeText']='████'
        row['displayPrivacy']['addedContextMask']=True
        ref=a.reference(self.packet_path)
        a.display_ancestry(view,ref)
        self.assertFalse(row['nativeFindingRedacted'])


if __name__ == '__main__':
    unittest.main()
