import copy
import json
from pathlib import Path
from unittest.mock import patch

from test_execution import PrivateFixture
import adapter
import boundary as b
import clocks
import proof_coordinator as producer
from test_temporal import TemporalRecords


class NoFollowBoundaryTests(PrivateFixture):
    def setUp(self):
        super().setUp()
        self.r = r = TemporalRecords(self, [dict(path='a.pem', mode='100644', sha256='d' * 64),
            dict(path='alias.der', mode='120000', symlink='../outside', blob='e' * 40),
            dict(path='.git/alias', mode='120000', symlink='excluded')])
        r.original['analysisStatus'] = dict(inspection='Partial', coverage=dict(selected_files=1,
            read_ok_files=1, read_failed_files=0, enumeration_failed_entries=1, analysis_failed_files=0, unknown_files=0, partial_files=0, unsupported_files=0))
        self.diagnostics = dict(coverage=r.original['analysisStatus']['coverage'], diagnostics=[
            dict(code='read_failed', file='alias.der', message='PRIVATE_SENTINEL'),
            dict(code='detector_partial', file='a.pem', detector='other_detector', message='PRIVATE_SENTINEL')])
        self.diagnostic_artifact(self.diagnostics)
        self.policy = r.record(dict(kind='temporal-nofollow-policy-relation-v1',
            historicalSourceBindings=r.relation_value['historicalSourceBindings'], finalSourceBindings=r.source,
            policy='static_nofollow_symlink_read_failed_enumeration_v1',
            phaseEvidence={phase:r.evidence for phase in ('primary-after','comparison-after')},
            finalEvidence=r.evidence, linkedBuild=r.evidence))
        r.relation_value['boundaryPolicyRelation'] = self.policy
        r.relation = r.record(r.relation_value)
        r.temporal_value.update(kind='temporal-replay-admission-v2', sourceRelation=r.relation, boundaryPolicyRelation=self.policy)
        self.config = b.document(r.row['config'])
        self.config.update(kind='temporal-witness-config-v2',
            manifest_projection=r.record(adapter.manifest_projection(r.original)),
            original_diagnostic_projection=r.record(adapter.original_diagnostic_projection(r.original)),
            boundary_policy_relation=self.policy)
        self.witness = copy.deepcopy(r.witness_value)
        link = adapter.manifest_projection(r.original)['entries'][1]
        self.witness.update(kind='temporal-producer-witness-v2', inspectionStatus='partial',
                            boundaryFailures=[dict(link, reason='nofollow_symlink')], diagnosticCodes=['read_failed'])
        self.witness['coverage']['enumerationFailedEntries'] = 1
        self.coverage = copy.deepcopy(r.coverage_value)
        self.coverage.update(kind='temporal-producer-coverage-v2', sourceRelation=r.relation,
            originalStatus=r.original['analysisStatus'], producerCoverage=self.witness['coverage'],
            boundaryInputs={key:self.config[key] for key in adapter.BOUNDARY_CONFIG_KEYS - {'kind'}},
            timeIndependentDiagnostics={'read_failed':dict(closure=r.relation, evidence=r.evidence)})

    def diagnostic_artifact(self, value):
        r = self.r
        attempt = b.mkdir(self.root / ('original-' + str(r.counter)))
        output = b.mkdir(attempt / 'output')
        diag = b.publish(output / 'diagnostics.json', value)
        terminal = r.record({})
        terminal = b.publish(attempt / 'terminal.json', dict(assessment=r.original['analysisStatus'], artifacts={'diagnostics.json':
            {key:value for key,value in diag.items() if key != 'path'}}))
        r.original.update(originalDiagnostics=diag, originalTerminal=terminal, originalPhaseId='primary-after')

    def proof(self, witness=None, coverage=None, config=None, temporal=None):
        r = self.r
        row = dict(r.row, config=r.record(self.config if config is None else config))
        witness = copy.deepcopy(self.witness if witness is None else witness)
        witness['configurationSha256'] = row['config']['sha256']
        row['witness'] = r.record(witness)
        value = copy.deepcopy(self.coverage if coverage is None else coverage)
        value['witness'] = row['witness']
        row['coverage'] = r.record(value)
        row['producerReceipt'] = r.producer_receipt(row)
        return clocks._coverage(row, r.temporal_value if temporal is None else temporal,
            r.relation_value, r.original, r.entry, r.started, r.generated)

    def test_v2_retains_partial_counts_regular_file_and_every_object(self):
        proof = self.proof()
        proof.prove_interval(self.r.started.second, self.r.generated.second)
        self.assertEqual(len(proof.objects), 1)
        self.assertEqual(proof.original_inspection, 'Partial')
        self.assertEqual([row['path'] for row in proof.boundary_failures], ['alias.der'])
        self.assertEqual(self.witness['coverage']['enumerationFailedEntries'], 1)
        projection = adapter.original_diagnostic_projection(self.r.original)
        self.assertEqual(len(projection['entries']), 2)
        self.assertNotIn('PRIVATE_SENTINEL', json.dumps(projection))
        self.assertNotIn('.git/alias', {row['path'] for row in proof.boundary_failures})

    def test_framed_hash_matches_rust_and_binds_path_mode_literal_and_target_bytes(self):
        self.assertEqual(adapter.link_metadata_hash('dir/公.der', '../公開.der'), '61f91a35dd32535dcc21fb45346b31714f55333ab79e292023aafbaf9e9884d1')
        self.assertNotEqual(adapter.link_metadata_hash('a', 'bc'), adapter.link_metadata_hash('ab', 'c'))
        for path in ['', '/a', 'a/../b', './a', 'a//b', 'a\n', 'a\x7f']:
            with self.subTest(path=path), self.assertRaises(b.Rejected):
                adapter.link_metadata_hash(path, 'target')

    def test_path_detector_multiplicity_manifest_entry_and_hash_swaps_reject(self):
        for mutation in ['path', 'detector', 'duplicate', 'count', 'regular', 'unlocated']:
            value = copy.deepcopy(self.diagnostics)
            row = value['diagnostics'][0]
            if mutation == 'path': row['file'] = '.git/alias'
            elif mutation == 'detector': row['detector'] = 'certificate_detector'
            elif mutation == 'duplicate': value['diagnostics'].append(copy.deepcopy(row))
            elif mutation == 'count': value['coverage']['enumeration_failed_entries'] = True
            elif mutation == 'regular': row['file'] = 'a.pem'
            else: row.pop('file')
            with self.subTest(mutation=mutation):
                self.diagnostic_artifact(value)
                with self.assertRaises(b.Rejected): self.proof()
        self.diagnostic_artifact(self.diagnostics)
        for key,value in [('symlink','changed'),('blob','f'*40),('mode','100644')]:
            manifest = json.loads(b.read(self.r.original['input']['manifest']))
            manifest[1][key] = value
            self.r.original['input']['manifest'] = self.r.record(manifest)
            with self.subTest(key=key), self.assertRaises(b.Rejected): self.proof()

    def test_counter_bool_unknown_regular_read_failures_and_receipt_loss_reject(self):
        mutations = []
        for key in adapter.WITNESS_SCHEMA['coverageKeys']:
            value = copy.deepcopy(self.witness); value['coverage'][key] = True; mutations.append(value)
        for key in ['unknownFiles', 'readFailedFiles', 'analysisFailedFiles', 'enumerationFailedEntries']:
            value=copy.deepcopy(self.witness);value['coverage'][key]=2;mutations.append(value)
        for changes in [dict(boundaryFailures=[]), dict(boundaryFailures=self.witness['boundaryFailures']*2),
                        dict(inspectionStatus='complete_within_scope'), dict(diagnosticCodes=[]),
                        dict(diagnosticCodes=['read_failed','metadata_failed']),
                        dict(diagnosticCodes=['read_failed','detector_truncated']),
                        dict(diagnosticCodes=['read_failed','detector_not_reported'])]:
            mutations.append(dict(self.witness,**changes))
        for value in mutations:
            with self.subTest(index=mutations.index(value)), self.assertRaises(b.Rejected): self.proof(value)

    def test_v1_keeps_old_boundaries_and_v2_has_no_schema_fallback(self):
        for changes in [dict(kind='temporal-producer-witness-v1'), dict(extra='unknown')]:
            with self.assertRaises(b.Rejected): self.proof(dict(self.witness,**changes))
        with self.assertRaises(b.Rejected):
            self.proof(temporal=dict(self.r.temporal_value,kind='temporal-replay-admission-v1'))
        for key in adapter.BOUNDARY_CONFIG_KEYS:
            value=copy.deepcopy(self.config);value.pop(key)
            with self.subTest(key=key), self.assertRaises(b.Rejected): self.proof(config=value)

    def test_all_interval_and_default_emission_obligations_survive_boundaries(self):
        r=self.r
        value=copy.deepcopy(self.witness)
        value['objects'][0].update(notBeforeUnix=r.started.second+2,validityState='not_yet_valid')
        with self.assertRaisesRegex(b.Rejected,'E_CLOCK_INTERVAL_TRANSITION'):
            self.proof(value).prove_interval(r.started.second,r.generated.second)
        value=copy.deepcopy(self.witness)
        value['objects'][0]['matchingDefaultValidityRuleIds']=['CERT_EXPIRED']
        with self.assertRaisesRegex(b.Rejected,'E_CLOCK_EXACT_EMISSION_UNPROVEN'):
            self.proof(value).prove_interval(r.started.second,r.generated.second)
        value=copy.deepcopy(self.witness);value['objects']=[]
        with self.assertRaisesRegex(b.Rejected,'E_TEMPORAL_OBJECT_LOSS'): self.proof(value)

    def test_bound_original_diagnostic_and_projection_bytes_cannot_be_spliced(self):
        value=copy.deepcopy(self.config)
        value['original_diagnostic_projection']['sha256']='0'*64
        with self.assertRaises(b.Rejected): self.proof(config=value)
        value=copy.deepcopy(self.config)
        value['manifest_projection']=self.config['original_diagnostic_projection']
        with self.assertRaises(b.Rejected): self.proof(config=value)
        value=copy.deepcopy(self.r.original['originalTerminal']);value['sha256']='0'*64
        self.r.original['originalTerminal']=value
        with self.assertRaises(b.Rejected): self.proof()

    def test_v2_configuration_publishes_rederived_bounded_exact_projections(self):
        import test_adapter
        product = test_adapter.AdapterPlanTests.product(self)
        destination = b.mkdir(self.root / 'v2-configuration')
        source = b.source_bindings(product)
        with patch.object(b, 'ROOT', product):
            result = adapter.witness_configuration_v2(destination, original=self.r.original, boundary_policy=self.policy,
                root=product, selection=dict(include_non_scannable=False,exclude_tests=False),
                detector_second=self.r.started.second, bindings=self.config['bindings'], final_source=source)
        config = b.document(result['configuration'])
        self.assertEqual(result['schema'], adapter.WITNESS_SCHEMA_V2)
        self.assertEqual(set(config), adapter.CONFIG_KEYS | adapter.BOUNDARY_CONFIG_KEYS)
        self.assertEqual(b.document(config['manifest_projection']), adapter.manifest_projection(self.r.original))
        self.assertEqual(b.document(config['original_diagnostic_projection']), adapter.original_diagnostic_projection(self.r.original))
        self.assertEqual(config['boundary_policy_relation'], self.policy)
        self.assertLessEqual(result['configuration']['bytes'],65536)

    def test_original_other_detector_state_is_retained_without_excusing_raw_failures(self):
        for code in ['detector_failed','detector_truncated','detector_not_reported']:
            value=copy.deepcopy(self.diagnostics)
            value['diagnostics'].append(dict(code=code,file='a.pem',detector='other_detector'))
            self.diagnostic_artifact(value)
            config=copy.deepcopy(self.config)
            config['original_diagnostic_projection']=self.r.record(adapter.original_diagnostic_projection(self.r.original))
            coverage=copy.deepcopy(self.coverage)
            coverage['boundaryInputs']['original_diagnostic_projection']=config['original_diagnostic_projection']
            self.proof(config=config,coverage=coverage).prove_interval(self.r.started.second,self.r.generated.second)
            for detector in ['certificate_detector',None]:
                changed=copy.deepcopy(value)
                changed['diagnostics'][-1]['detector']=detector
                self.diagnostic_artifact(changed)
                config['original_diagnostic_projection']=self.r.record(adapter.original_diagnostic_projection(self.r.original))
                coverage['boundaryInputs']['original_diagnostic_projection']=config['original_diagnostic_projection']
                with self.subTest(code=code,detector=detector),self.assertRaises(b.Rejected):
                    self.proof(config=config,coverage=coverage)

    def test_v2_fresh_gate_requires_eleven_subjects_current_schema_and_policy_build(self):
        r=self.r
        actual_plan=r.record(dict(schema=adapter.WITNESS_SCHEMA_V2))
        actual=dict(binary=r.binary,plan=actual_plan)
        wrapper=r.record(dict(r.build_value,toolSource=b.ref(b.CODE_DIR/'temporal_witness.rs'),
            witnessSchema=adapter.WITNESS_SCHEMA_V2,schemaAuthority=b.ref(b.CODE_DIR/'adapter.py')))
        subjects=dict(policy=r.record(dict(kind='synthetic policy')),binaryAliases=r.record(dict(aliases=[])),
            recoveryRegistry=r.record(dict(repositories=[dict(id='synthetic-'+str(i)) for i in range(98)])),
            sourceRelation=r.relation,witnessBuildReceipt=wrapper,witnessBinary=r.binary,
            witnessToolSource=b.ref(b.CODE_DIR/'temporal_witness.rs'),witnessRules=r.rules,overlayBuildReceipt=r.overlay,
            coordinator=b.ref(producer.__file__),boundaryPolicyRelation=self.policy)
        gate=dict(kind='temporal-witness-run-gate-v2',verdict='PASS',reviewerModel='codex:gpt-6.1-sol@high',
            reviewerId='synthetic-independent-reviewer',authorIds=sorted(clocks.AUTHORS),scopes=sorted(clocks.REVIEW_SCOPES),
            sourceBindings=r.source,harnessBindings=b.harness_bindings(),subjects=subjects)
        def admit(value):
            with patch.object(b,'verify_source'),patch.object(adapter,'verify_build'), \
                 patch.object(adapter,'verify_witness_build',return_value=actual), \
                 patch.object(clocks.OriginalClockCohort,'admit',return_value='synthetic cohort'):
                return producer.admit(r.record(value),adapter,b,clocks)
        self.assertEqual(admit(gate)[2],'synthetic cohort')
        for change in [dict(kind='temporal-witness-run-gate-v1'),
            dict(subjects={key:value for key,value in subjects.items() if key!='boundaryPolicyRelation'}),
            dict(subjects=dict(subjects,boundaryPolicyRelation=r.evidence))]:
            with self.subTest(change=list(change)),self.assertRaises(b.Rejected):admit(dict(gate,**change))
        actual['plan']=r.record(dict(schema=adapter.WITNESS_SCHEMA))
        with self.assertRaises(b.Rejected):admit(gate)

    def test_v2_temporal_admission_requires_eight_independently_reviewed_subjects(self):
        r=self.r
        r.temporal_value['witnessBuildReceipt']=r.record(dict(r.build_value,witnessSchema=adapter.WITNESS_SCHEMA_V2,
            schemaAuthority=b.ref(b.CODE_DIR/'adapter.py')))
        r.refresh_review()
        with self.assertRaisesRegex(b.Rejected,'E_TEMPORAL_REVIEW'): r.admit()
        review=copy.deepcopy(r.review_value)
        review['subjects']['boundaryPolicyRelation']=self.policy
        r.temporal_value['review']=r.record(review)
        r.temporal=r.record(r.temporal_value)
        self.assertEqual(r.admit()[1],r.relation_value)
        changed=copy.deepcopy(review)
        changed['subjects']['boundaryPolicyRelation']=r.evidence
        r.temporal_value['review']=r.record(changed)
        r.temporal=r.record(r.temporal_value)
        with self.assertRaisesRegex(b.Rejected,'E_TEMPORAL_REVIEW'):r.admit()

    def set_declared_inspection(self, declared, legacy):
        self.r.original['analysisStatus']['inspection'] = declared
        oss, _, _ = b.import_auditors()
        self.r.original['harness'] = {'tests/quality/oss.py': oss.LEGACY_ENUMERATION_ASSESSOR if legacy else '0' * 64}
        self.diagnostic_artifact(self.diagnostics)
        self.config['original_diagnostic_projection'] = self.r.record(adapter.original_diagnostic_projection(self.r.original))
        self.coverage['originalStatus'] = copy.deepcopy(self.r.original['analysisStatus'])
        self.coverage['boundaryInputs'] = {key: self.config[key] for key in adapter.BOUNDARY_CONFIG_KEYS - {'kind'}}

    def test_legacy_declared_complete_preserves_coverage_derived_partial(self):
        self.set_declared_inspection('Complete', True)
        proof = self.proof()
        self.assertEqual(proof.original_inspection, 'Complete')
        self.assertEqual(proof.coverage_inspection, 'Partial')
        self.assertEqual(self.r.original['analysisStatus']['inspection'], 'Complete')
        self.assertEqual(self.r.original['analysisStatus']['coverage']['enumeration_failed_entries'], 1)
        self.assertEqual(self.witness['inspectionStatus'], 'partial')
        proof.prove_interval(self.r.started.second, self.r.generated.second)

    def test_legacy_qualification_does_not_excuse_other_loss_or_unknown_assessor(self):
        for legacy, other_loss in [(False, False), (True, True)]:
            with self.subTest(legacy=legacy, other_loss=other_loss):
                self.r.original['analysisStatus']['coverage']['unknown_files'] = int(other_loss)
                self.set_declared_inspection('Complete', legacy)
                with self.assertRaises(b.Rejected):
                    self.proof()

    def test_historical_two_key_references_have_authenticated_full_projection_bindings(self):
        original_manifest = dict(self.r.original['input']['manifest'])
        original_terminal = dict(self.r.original['originalTerminal'])
        self.r.original['input']['manifest'] = {k: v for k, v in original_manifest.items() if k != 'bytes'}
        self.r.original['originalTerminal'] = {k: v for k, v in original_terminal.items() if k != 'bytes'}
        manifest = adapter.manifest_projection(self.r.original)
        diagnostics = adapter.original_diagnostic_projection(self.r.original)
        self.assertEqual(manifest['manifest'], original_manifest)
        self.assertEqual(diagnostics['originalTerminal'], original_terminal)
        self.assertEqual(set(self.r.original['input']['manifest']), {'path', 'sha256'})
        self.assertEqual(set(self.r.original['originalTerminal']), {'path', 'sha256'})

    def test_historical_projection_binding_drift_and_extra_keys_reject(self):
        for mutation in ['sha256', 'bytes', 'extra']:
            original = copy.deepcopy(self.r.original)
            reference = original['input']['manifest']
            if mutation == 'sha256': reference['sha256'] = '0' * 64
            elif mutation == 'bytes': reference['bytes'] += 1
            else: reference['optional_skip'] = True
            with self.subTest(mutation=mutation), self.assertRaises(b.Rejected):
                adapter.manifest_projection(original)
