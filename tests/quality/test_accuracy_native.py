"""Synthetic exact-position and same-asset controls for native capture."""
import copy
import hashlib
import importlib.util
import json
import pathlib
import unittest
import types
from unittest.mock import patch
import accuracy_native as n

ROOT = pathlib.Path(__file__).resolve().parents[2]
SCHEMA = json.loads((ROOT / 'schemas/native-scan-v1.schema.json').read_bytes())
SPEC = importlib.util.spec_from_file_location('native_test_judgment', ROOT / 'tests/quality/judgment.py')
judgment = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(judgment)


def identity(prefix, number):
    return prefix + '-v1-' + format(number, '064x')


def asset(number=1, identifier='RSA', key_bits='1024', namespace='named', purpose='reference'):
    return dict(id=identity('asset', number), kind='algorithm', identity_basis='syntactic_reference',
                attributes=dict(namespace=namespace, identifier=identifier, purpose=purpose,
                                parameters={} if key_bits is None else dict(key_bits=key_bits)), relations=[], uncertainty=[])


def observation(number=1, start=8, asset_numbers=(1,), path='s.py', end=None):
    return dict(id=identity('obs', number), occurrence_key=None, identity_confidence='stable',
                asset_ids=[identity('asset', number) for number in asset_numbers],
                target=dict(path=path, line=1, column=9, column_kind='legacy_mixed', byte_start=start,
                            byte_end=None if start is None else start+3 if end is None else end, object_index=None),
                method='ast_pattern', producer='tree_sitter_detector', assurance='syntactic', usage='referenced', rule_id=None,
                evidence=dict(marker_kind=None, capture_status='legacy_reported'), uncertainty=[])


def native():
    coverage = dict(selected_files=1, read_ok_files=1, read_failed_files=0, analyzed_files=1, partial_files=0,
                    analysis_failed_files=0, unsupported_files=0, not_applicable_files=0, unknown_files=0,
                    excluded_enumerated_files=0, enumeration_failed_entries=0, not_enumerated_population=True,
                    status='complete_within_scope')
    scope = dict(max_file_size_bytes=2097152, git_ignore=True, git_global=True, git_exclude=True,
                 default_excludes=['.git','node_modules','target','dist','docs'], markdown_excluded=True)
    return dict(schema_version=1, identity_scheme='pqc-scan-length-prefixed-sha256-v1', scanner_version='0.4.0', catalog_version='1',
                profile='default', rules_digest=identity('rules', 1), run=dict(id=identity('scan', 1), attempt_id='synthetic', generated_at='2026-11-01T00:00:00Z'),
                repository_scope=None, scope=scope, coverage=coverage, diagnostics=[], native_completeness='complete_within_scope',
                assets=[asset()], observations=[observation()], legacy_associations=[], unassociated_observation_ids=[],
                legacy_loss_records=[], assessment_status='not_evaluated', assessments=[], export_loss_notes=[])


def fixture():
    raw = '日本🙂RSACALL public synthetic\n'.encode()
    sha = hashlib.sha256(raw).hexdigest()
    start = raw.index(b'RSA')
    report = native(); report['observations'][0]['target'].update(byte_start=start, byte_end=start+3)
    case = dict(id='synthetic-fact', sourceBinding=dict(path='/synthetic/s.py', sha256=sha), repositoryId='synthetic-repo',
                propertyKind='algorithm_family_reference', selector=dict(path='s.py', byte_start=start),
                join='exact', expectedValuesPresent=False)
    admitted = dict(repositoryId='synthetic-repo', artifact=dict(path='/synthetic/scan.inventory.v1.json', sha256='a'*64, bytes=100),
                    native=report, sourceManifest={'s.py':dict(bytes=len(raw), sha256=sha)}, sourceFiles={'s.py':raw}, inspection='Complete')
    return dict(cases=[case]), dict(namespace='named', identifierToFamily=copy.deepcopy(n.FAMILIES)), {'synthetic-repo':admitted}


class NativeCaptureTests(unittest.TestCase):
    def setUp(self):
        self.manifest, self.normalization, self.admitted = fixture()
        self.repo = self.admitted['synthetic-repo']
        self.case = self.manifest['cases'][0]
        self.document = self.repo['native']

    def capture(self):
        return n.capture_native_properties(self.manifest, self.normalization, self.admitted, SCHEMA)

    def actual(self):
        return self.capture()[0]['actual']

    def test_exact_original_byte_start_unicode_columns_are_not_used(self):
        self.document['observations'][0]['target'].update(line=999, column=999, column_kind='legacy_mixed')
        self.assertEqual(self.actual()['values'], [dict(namespace='named', family='RSA', purpose='reference')])
        self.assertTrue(self.actual()['pointRateEligible'])

    def test_nearby_decoy_is_not_a_fallback(self):
        self.document['assets'].append(asset(2, 'Ed25519'))
        decoy = observation(2, start=self.case['selector']['byte_start']-1, asset_numbers=(2,))
        self.document['observations'].append(decoy)
        self.document['observations'][0]['target']['byte_start'] += 1
        self.assertEqual(self.actual()['values'], [])
        self.assertEqual(self.actual()['joinStatus'], 'no_exact_location')

    def test_repository_boundary_with_identical_path(self):
        other = copy.deepcopy(self.repo); other['repositoryId'] = 'other'
        other['native']['assets'][0]['attributes']['identifier'] = 'Ed25519'
        self.admitted['other'] = other
        self.assertEqual(self.actual()['values'][0]['family'], 'RSA')

    def test_null_span_is_unjoinable_and_suppresses_point_rate(self):
        self.document['observations'][0]['target'].update(byte_start=None, byte_end=None)
        actual = self.actual()
        self.assertEqual(actual['joinStatus'], 'unjoinable_location')
        self.assertFalse(actual['pointRateEligible'])

    def test_individually_unavailable_span_ends_remain_unresolved(self):
        self.document['observations'][0]['target']['byte_start'] = None
        self.assertEqual(self.actual()['joinStatus'], 'unjoinable_location')
        self.document['observations'][0]['target'].update(byte_start=self.case['selector']['byte_start'], byte_end=None)
        self.assertEqual(self.actual()['joinStatus'], 'partial_capture')
        self.assertFalse(self.actual()['pointRateEligible'])

    def test_duplicate_native_ids_dangling_observation_and_relations_rejected(self):
        for mutate in [lambda d:d['assets'].append(copy.deepcopy(d['assets'][0])),
                       lambda d:d['observations'].append(copy.deepcopy(d['observations'][0])),
                       lambda d:d['observations'][0]['asset_ids'].append(identity('asset', 9)),
                       lambda d:d['assets'][0]['relations'].append(dict(relationship='algorithm', target=identity('asset', 9)))]:
            with self.subTest(mutation='identity/reference'):
                original = copy.deepcopy(self.document); mutate(self.document)
                with self.assertRaises(n.boundary.Rejected): self.capture()
                self.document.clear(); self.document.update(original)

    def test_invalid_and_unknown_ids_are_not_published(self):
        self.document['observations'][0]['id'] = 'SYNTH_ARBITRARY_ATTRIBUTE'
        with self.assertRaises(n.boundary.Rejected): self.capture()

    def test_same_asset_conjunction_prevents_cross_asset_parameter_mixing(self):
        self.case['propertyKind'] = 'explicit_rsa_key_bits'
        self.document['assets'][0]['attributes']['parameters'] = {}
        self.document['assets'].append(asset(2, 'ECDH', key_bits='1024'))
        self.document['observations'][0]['asset_ids'].append(identity('asset',2))
        actual = self.actual()
        expected = dict(namespace='named', identifier='RSA', purpose='reference', parameter='key_bits', value='1024')
        result = judgment.compare(dict(cases=[dict(id=self.case['id'], checks=[dict(pointer='/values', operation='contains', expected=expected)])]), self.capture())
        self.assertEqual(result['rows'][0]['status'], 'Failed')
        self.assertNotIn(expected, actual['values'])

    def test_multifact_independence_and_duplicate_cells_preserve_reference_ledger(self):
        self.document['assets'][0]['attributes']['parameters'] = {}
        self.document['observations'].append(copy.deepcopy(self.document['observations'][0]))
        self.document['observations'][-1]['id'] = identity('obs', 2)
        self.manifest['cases'].append(dict(copy.deepcopy(self.case), id='synthetic-parameter', propertyKind='explicit_rsa_key_bits'))
        rows = self.capture()
        self.assertEqual(len(rows[0]['actual']['values']), 1)
        self.assertEqual(len(rows[0]['actual']['references']), 2)
        self.assertEqual(rows[1]['actual']['values'], [])
        self.assertEqual(rows[1]['actual']['joinStatus'], 'no_linked_property')
        self.assertTrue(rows[1]['actual']['pointRateEligible'])

    def test_finite_exact_normalization_and_unknown_strings_withheld(self):
        for value in ('rsa', 'ML-KEM-X', 'EC', 'SYNTH_UNKNOWN_PRIVATE_VALUE'):
            self.document['assets'][0]['attributes']['identifier'] = value
            actual = self.actual()
            self.assertEqual(actual['values'], [])
            self.assertEqual(actual['joinStatus'], 'unrecognized_value')
            self.assertNotIn(value, json.dumps(actual))
        self.document['assets'][0]['attributes']['identifier'] = 'ML-KEM-1024'
        self.assertEqual(self.actual()['values'][0]['family'], 'ML-KEM')
        self.case['propertyKind'] = 'explicit_named_parameter_set'
        self.assertEqual(self.actual()['values'][0]['identifier'], 'ML-KEM-1024')

    def test_no_numeric_bool_or_noncanonical_parameter_coercion(self):
        self.case['propertyKind'] = 'explicit_rsa_key_bits'
        for value in (1024, True):
            self.document['assets'][0]['attributes']['parameters']['key_bits'] = value
            with self.assertRaises(n.boundary.Rejected): self.capture()
        for value in ('01024', ' 1024', '1024.0', '０１０２４', '0', '32769'):
            self.document['assets'][0]['attributes']['parameters']['key_bits'] = value
            self.assertEqual(self.actual()['values'], [])
        self.document['assets'][0]['attributes']['parameters']['key_bits'] = '1024'
        self.assertEqual(self.actual()['values'][0]['value'], '1024')

    def test_no_expected_family_filter_and_no_assessment_role_inference(self):
        self.document['assets'].append(asset(2, 'Ed25519'))
        self.document['observations'][0]['asset_ids'].append(identity('asset', 2))
        self.assertEqual({cell['family'] for cell in self.actual()['values']}, {'RSA', 'Ed25519'})
        self.assertTrue(all(cell['purpose'] == 'reference' for cell in self.actual()['values']))
        self.document['assessments'] = [dict(algorithm=dict(purpose='sign'))]
        with self.assertRaises(n.boundary.Rejected): self.capture()

    def test_partial_capture_partial_inspection_and_unknown_purpose(self):
        self.document['observations'][0]['evidence']['capture_status'] = 'truncated'
        self.assertEqual(self.actual()['joinStatus'], 'partial_capture')
        self.assertFalse(self.actual()['pointRateEligible'])
        self.document['observations'][0]['evidence']['capture_status'] = 'legacy_reported'
        self.repo['inspection'] = 'Partial'
        self.assertFalse(self.actual()['pointRateEligible'])
        self.repo['inspection'] = 'Complete'
        self.document['assets'][0]['attributes']['purpose'] = 'SYNTH_ARBITRARY_PURPOSE'
        self.assertNotIn('SYNTH_ARBITRARY_PURPOSE', json.dumps(self.actual()))

    def test_unsupported_failed_unknown_capture_and_nonexhaustive_header_retained(self):
        for status, expected in [('unsupported','unsupported_capture'),('failed','failed_capture'),
                                 ('SYNTH_UNKNOWN_STATUS','unknown_capture')]:
            self.document['observations'][0]['evidence']['capture_status'] = status
            actual = self.actual()
            self.assertEqual(actual['joinStatus'],expected)
            self.assertFalse(actual['pointRateEligible'])
            if status.startswith('SYNTH_'): self.assertNotIn(status,json.dumps(actual))
        self.document['observations'][0]['evidence']['capture_status'] = 'legacy_reported'
        self.document['native_completeness'] = 'not_exhaustive'
        actual = self.actual()
        self.assertEqual(actual['values'][0]['family'],'RSA')
        self.assertEqual(actual['completeness'],'not_exhaustive')
        self.assertFalse(actual['pointRateEligible'])

    def test_missing_artifact_and_absent_linked_property_distinct(self):
        self.admitted.clear(); self.assertEqual(self.actual()['joinStatus'], 'missing_artifact')
        self.admitted['synthetic-repo'] = self.repo
        self.document['observations'][0]['asset_ids'] = []
        self.assertEqual(self.actual()['joinStatus'], 'no_linked_property')

    def test_encoding_bom_wide_invalid_utf8_and_midcodepoint_excluded(self):
        for raw in (b'\xef\xbb\xbfRSA', b'R\0S\0A\0', b'\xffRSA'):
            self.repo['sourceFiles']['s.py'] = raw
            sha = hashlib.sha256(raw).hexdigest()
            self.case['sourceBinding']['sha256'] = sha
            self.repo['sourceManifest']['s.py'].update(bytes=len(raw), sha256=sha)
            self.document['observations'] = []
            with self.assertRaises(n.boundary.Rejected): self.capture()
        self.manifest, self.normalization, self.admitted = fixture()
        self.manifest['cases'][0]['selector']['byte_start'] = 1
        with self.assertRaises(n.boundary.Rejected): self.capture()

    def test_out_of_bounds_span_rejected(self):
        self.document['observations'][0]['target']['byte_end'] = 100000
        with self.assertRaises(n.boundary.Rejected): self.capture()

    def test_all_129_case_ids_and_102_unassessed_retained_without_output_access(self):
        base = copy.deepcopy(self.case)
        self.manifest['cases'] = [dict(copy.deepcopy(base), id=f'synthetic-{number}',
                                      **({} if number < 27 else dict(propertyKind=None, selector=None, join='not_assessed')))
                                  for number in range(129)]
        rows = self.capture()
        self.assertEqual([row['id'] for row in rows], [row['id'] for row in self.manifest['cases']])
        self.assertEqual(sum(row['actual']['joinStatus'] == 'not_assessed' for row in rows), 102)

    def test_source_hash_drift_and_fabricated_normalization_rejected(self):
        self.repo['sourceFiles']['s.py'] += b'change'
        with self.assertRaises(n.boundary.Rejected): self.capture()
        self.manifest, self.normalization, self.admitted = fixture()
        self.normalization['identifierToFamily']['RSA'] = 'SYNTH_ARBITRARY_VALUE'
        with self.assertRaises(n.boundary.Rejected): self.capture()

    def test_all_four_policy_phases_preflight_before_legacy_body_parsing(self):
        entries = {f'r{number}': dict(id=f'r{number}', population='primary') for number in range(90)}
        entries.update({f'c{number}':dict(id=f'c{number}', population='comparison') for number in range(8)})
        manifest = copy.deepcopy(self.manifest)
        base = dict(manifest['cases'][0], repositoryId='r0')
        manifest['normalizationBinding'] = {'path':'/synthetic/normalization', 'sha256':'0'*64}
        manifest['cases'] = [dict(copy.deepcopy(base), id=f'synthetic-{number}',
                                  **({} if number < 30 else dict(propertyKind=None, selector=None, join='not_assessed')))
                              for number in range(129)]
        proof = dict(rows=[dict(factId=case['id'], sourceBinding=case['sourceBinding'], strictUtf8=True,
                                bomAbsent=True, wideRecognitionLaneAbsent=True, decodedUtf8EqualsOriginalBytes=True)
                           for case in manifest['cases'][:30]])
        records = [dict(id=population+'-'+phase, population=population, phase=phase, binding={'path':population+'-'+phase, 'sha256':'0'*64})
                   for population in ('primary','comparison') for phase in ('baseline','after')]
        policy = dict(kind='historical-score-policy', version=1, registry={'path':'registry'}, invocations=records)
        quality = pathlib.Path('/synthetic/quality')
        refs = [dict(path=str(quality/name),sha256='0'*64) for name in ('oss.py','evaluate.py','external.py','taxonomy.py','run.py','task050/study.py')]
        request = dict(oracleBinding={'path':'synthetic-oracle-identity-only-not-read'}, judgmentBinding={'path':'synthetic-judgment'}, sourceOnlyReview={'path':'synthetic-review-not-real-approval'}, captureManifest={'path':'manifest'}, normalization=manifest['normalizationBinding'],
                       coordinateProof={'path':'proof'}, nativeSchema={'path':'schema'}, historicalPolicy={'path':'policy'},
                       assessmentHelper={'path':'assessment'}, verifierBindings=refs, qualityDirectory=str(quality))
        events, invocations = [], {}
        def preflight(binding):
            record = next(row for row in records if row['binding'] == binding)
            ids = [identity for identity, entry in entries.items() if entry['population'] == record['population']]
            actual = dict(phase=record['phase'], expectedSamples=1, selectedIds=ids,
                          selectedEntries={identity:entries[identity] for identity in ids})
            invocations[record['id']] = actual
            events.append('preflight')
            return dict(invocation=actual,path=record['id'])
        def verified(path):
            events.append('legacy-body-parse')
            actual = invocations[path]
            rows = {(identity,0):(dict(receipt=dict(path='/synthetic/'+identity+'/terminal.json')),
                    dict(artifacts={'scan.inventory.v1.json':dict(bytes=100,sha256='a'*64)},
                         assessment=dict(inspection='Complete',coverage=self.document['coverage'])),
                    dict(input=dict(manifest=dict(path='source-manifest')))) for identity in actual['selectedIds']}
            return actual, rows
        oss = types.SimpleNamespace(_verified_invocation=verified)
        assessment = types.SimpleNamespace(load_historical=lambda _: (oss,refs))
        def read_json(binding):
            objects = dict(manifest=manifest, normalization=self.normalization, proof=proof, schema=SCHEMA,
                           policy=policy, registry={'repositories':list(entries.values())})
            return objects.get(pathlib.Path(binding['path']).name, self.document)
        def checked(path, expected=None, **kwargs):
            if str(path) == 'source-manifest':
                return json.dumps([dict(path='s.py',**self.repo['sourceManifest']['s.py'])]).encode()
            return self.repo['sourceFiles']['s.py'] if kwargs.get('collect') else {}
        with patch.object(n, '_review_gate'), patch.object(n, '_module', return_value=assessment), \
             patch.object(n.boundary, 'bound_json', read_json), patch.object(n.boundary, 'checked_file', checked), \
             patch.object(n.boundary, 'preflight_invocation', preflight):
            result = n.capture_from_policy(request)
        self.assertEqual(events[:4], ['preflight']*4)
        self.assertEqual(events[4:], ['legacy-body-parse']*4)
        self.assertEqual(len(result['observations']),129)
        self.assertEqual(sum(row['actual']['joinStatus']=='not_assessed' for row in result['observations']),99)

    def test_review_gate_binds_oracle_identity_source_and_independent_reviewer(self):
        keys = ('captureManifest','normalization','coordinateProof','nativeSchema','historicalPolicy','assessmentHelper',
                'verifierBindings','qualityDirectory','oracleBinding','judgmentBinding')
        request = {key:{'path':'/synthetic/'+key,'sha256':'0'*64} for key in keys}
        request['qualityDirectory'] = '/synthetic/quality'
        request['verifierBindings'] = []
        request['sourceOnlyReview'] = {'path':'/synthetic/review','sha256':'0'*64}
        own = dict(path=str(pathlib.Path(n.__file__).absolute()),bytes=1,sha256='a'*64)
        preflight = dict(path=str(pathlib.Path(n.boundary.__file__).absolute()),bytes=1,sha256='b'*64)
        review = dict(kind='native-source-only-implementation-review-v1',verdict='PASS',model='codex:gpt-6.1-sol@high',
                      author='synthetic-author',reviewer='synthetic-independent-reviewer',targetOutputsOpened=False,
                      all129CasesReviewed=True,all30SelectorsReviewed=True,implementationCorrectness='PASS',testAdequacy='PASS',
                      bindings={key:request[key] for key in keys},adapter=own,artifactPreflight=preflight)
        reads = []
        def checked(path,*args,**kwargs):
            reads.append(str(path))
            return own if str(path)==own['path'] else preflight
        with patch.object(n.boundary,'bound_json',return_value=review), patch.object(n.boundary,'checked_file',checked):
            self.assertEqual(n._review_gate(request),review)
            self.assertNotIn(request['oracleBinding']['path'],reads)
            for field,value in [('reviewer','synthetic-author'),('targetOutputsOpened',True),('all129CasesReviewed',False),
                                ('model','different-model'),('implementationCorrectness','PENDING'),('all30SelectorsReviewed',False)]:
                altered = dict(review,**{field:value})
                with patch.object(n.boundary,'bound_json',return_value=altered):
                    with self.assertRaises(n.boundary.Rejected): n._review_gate(request)
            altered = dict(review,adapter=dict(own,sha256='c'*64))
            with patch.object(n.boundary,'bound_json',return_value=altered):
                with self.assertRaises(n.boundary.Rejected): n._review_gate(request)

    def test_target_gate_rejects_before_policy_or_native_read(self):
        with patch.object(n, '_review_gate', side_effect=n.boundary.Rejected('E_NATIVE_REVIEW_GATE')):
            with patch.object(n.boundary, 'bound_json') as read:
                with self.assertRaises(n.boundary.Rejected): n.capture_from_policy({})
                read.assert_not_called()


if __name__ == '__main__':
    unittest.main()
