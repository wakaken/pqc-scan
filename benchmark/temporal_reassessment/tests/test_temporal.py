import copy
import json
from pathlib import Path
from unittest.mock import patch

from test_execution import PrivateFixture
from test_clocks import observed_row
import boundary as b
import clocks
import execution
import adapter
import hashlib
import storage


class TemporalRecords:
    def __init__(self, case, manifest_files=None):
        self.case = case
        self.counter = 0
        self.source = {'crates/core.rs': 'a' * 64, 'rules/default/a.yml': 'b' * 64}
        self.harness = {'synthetic-harness': 'c' * 64}
        self.evidence = self.record(dict(kind='synthetic-source-proof', statement='All records are synthetic test inputs.'))
        self.overlay = self.record(dict(kind='synthetic-overlay-build'))
        self.tool_source = self.record(dict(kind='synthetic-first-party-tool-source'))
        self.binary = self.record(dict(kind='synthetic-tool-binary-identity'))
        self.rules_directory = b.mkdir(case.root / 'proof-rules')
        (self.rules_directory / 'a.yml').write_bytes(b'synthetic default139 rule identity')
        (self.rules_directory / 'temporal-private-witness.yml').write_bytes(adapter.WITNESS_RULE)
        self.rule_files = [{key: value for key, value in b.ref(path).items() if key != 'path'} | {'path': path.name}
                           for path in sorted(self.rules_directory.iterdir())]
        self.source['rules/default/a.yml'] = self.rule_files[0]['sha256']
        self.rules = self.record(dict(kind='temporal-witness-rules-v1', defaultRuleCount=139, files=self.rule_files,
                                     defaultValidityRuleIds=['CERT_EXPIRED', 'CERT_NOT_YET_VALID']))
        self.relation_value = dict(kind='temporal-source-relation-v1', finalSourceBindings=self.source,
            historicalSourceBindings={phase: self.source for phase in ('primary-after', 'comparison-after')},
            clockClosure={name: self.evidence for name in clocks.CLOCK_CLOSURE}, classifiedHunks=[])
        self.relation = self.record(self.relation_value)
        self.build_value = dict(kind='temporal-witness-build-receipt-v1', exitCode=0, binary=self.binary,
            toolSource=self.tool_source, sourceBindings=self.source, rules=self.rules,
            linkedSourceEvidence=self.evidence, boundaryTests=self.evidence)
        self.build = self.record(self.build_value)
        self.input_ref = self.record([dict(path='a.pem', sha256='d' * 64)] if manifest_files is None else manifest_files)
        self.original = dict(input=dict(root='/synthetic/input', manifest=self.input_ref),
            selection=dict(files={}, effectiveGlobalIgnore='/synthetic/ignore'), analysisStatus=dict(inspection='Complete', coverage=dict(selected_files=1, read_ok_files=1, read_failed_files=0)))
        self.entry = dict(inputManifestSha256=self.input_ref['sha256'])
        self.started = clocks.instant('2026-11-01T00:00:00+00:00')
        self.generated = clocks.instant('2026-11-01T00:00:04Z')
        self.row = self.coverage_row('synthetic-0', 'primary-after')
        rows = [dict(self.row, repositoryId='synthetic-' + str(i), phaseId='primary-after' if i < 90 else 'comparison-after') for i in range(98)]
        self.coverage_index = self.record(dict(kind='temporal-coverage-index-v1', rows=rows))
        self.temporal_value = dict(kind='temporal-replay-admission-v1', sourceBindings=self.source, harnessBindings=self.harness,
            overlayBuildReceipt=self.overlay, sourceRelation=self.relation, witnessBuildReceipt=self.build, witnessBinary=self.binary,
            witnessToolSource=self.tool_source, witnessRules=self.rules, coverageIndex=self.coverage_index,
            authorIds=sorted(clocks.AUTHORS | {'synthetic-author'}))
        self.refresh_review()

    def record(self, value):
        self.counter += 1
        return b.publish(self.case.root / ('synthetic-' + str(self.counter) + '.json'), value)

    def refresh_review(self, **changes):
        subjects = {name: self.temporal_value[name] for name in ('sourceRelation', 'witnessBuildReceipt', 'witnessBinary',
            'witnessToolSource', 'witnessRules', 'coverageIndex', 'overlayBuildReceipt')}
        self.review_value = dict(kind='temporal-independent-review-v1', verdict='PASS', reviewerModel='codex:gpt-6.1-sol@high',
            reviewerId='synthetic-independent-reviewer', scopes=sorted(clocks.REVIEW_SCOPES), subjects=subjects,
            sourceBindings=self.source, harnessBindings=self.harness)
        self.review_value.update(changes)
        self.temporal_value['review'] = self.record(self.review_value)
        self.temporal = self.record(self.temporal_value)

    def admit(self):
        return clocks.admit_temporal(self.temporal, self.source, self.harness, ['synthetic-author'], self.overlay)

    def coverage_row(self, identity, phase):
        config = dict(root=self.original['input']['root'], max_file_size_bytes=2 * 1024**2, threads=1,
            selection=dict(include_non_scannable=False, exclude_tests=False), detector_second=self.started.second,
            rules_dir=str(self.rules_directory), witness_rule_id=adapter.WITNESS_SCHEMA['witnessRuleId'],
            rule_files=self.rule_files, output='/synthetic/witness.json')
        bindings = dict(source=b.digest(self.source), rules=self.rules['sha256'], input=self.entry['inputManifestSha256'],
            selection=b.digest(self.original['selection']), tool=self.tool_source['sha256'])
        config['bindings'] = bindings
        config_ref = self.record(config)
        objects = [dict(path='a.pem', sourceSha256='d' * 64, certificateSha256='e' * 64, objectIndex=0,
                        notBeforeUnix=0, notAfterUnix=2000000000, validityState='valid', matchingDefaultValidityRuleIds=[])]
        files = [dict(path='a.pem', sourceSha256='d' * 64, status='complete', objects=1)]
        self.witness_value = dict(kind='temporal-producer-witness-v1', objects=objects, files=files,
            coverage={name: (1 if name in {'selectedFiles', 'readOkFiles', 'analyzedFiles'} else 0)
                      for name in adapter.WITNESS_SCHEMA['coverageKeys']},
            diagnosticCodes=[], bindings=bindings, ruleCount=140, status=adapter.WITNESS_SCHEMA['status'],
            configurationSha256=config_ref['sha256'], binarySha256=self.binary['sha256'], detectorSecond=self.started.second)
        witness_ref = self.record(self.witness_value)
        self.coverage_value = dict(kind='temporal-producer-coverage-v1', bindings=bindings, repositoryId=identity, phaseId=phase,
            originalSourceBindings=self.source, sourceRelation=self.relation, witness=witness_ref,
            originalStatus=self.original['analysisStatus'], originalInput=self.original['input'], originalSelection=self.original['selection'],
            window=dict(firstSecond=self.started.second, lastSecond=self.generated.second), clockClosure=sorted(clocks.CLOCK_CLOSURE),
            rulesDirectory=config['rules_dir'], witnessRuleId=config['witness_rule_id'], files=files,
            producerCoverage=self.witness_value['coverage'], selectedFiles={'a.pem': 'd' * 64}, defaultValidityRules=['CERT_EXPIRED', 'CERT_NOT_YET_VALID'])
        row = dict(repositoryId=identity, phaseId=phase, coverage=self.record(self.coverage_value), witness=witness_ref, config=config_ref)
        row['producerReceipt'] = self.producer_receipt(row)
        return row

    def producer_receipt(self, row):
        return self.record(dict(kind='temporal-witness-process-v1', execution='Succeeded', exitCode=0, timedOut=False,
            performance=None, scope='static_time_condition_proof', startedUtc='2026-11-01T00:00:00+00:00',
            endedUtc='2026-11-01T00:00:04+00:00', sourceBefore=self.source, sourceAfter=self.source, binary=self.binary, configuration=row['config'], output=row['witness'], sourceBindings=self.source,
            inputBefore=self.original['input']['manifest'], inputAfter=self.original['input']['manifest'],
            selectionBefore=self.original['selection'], selectionAfter=self.original['selection'],
            commandDigest=b.digest([self.binary['path'], row['config']['path']]), cwdDigest=b.digest(str(b.ROOT))))

    def proof(self, witness=None, coverage=None):
        row = dict(self.row)
        if witness is not None:
            row['witness'] = self.record(witness)
        value = copy.deepcopy(self.coverage_value if coverage is None else coverage)
        value['witness'] = row['witness']
        row['coverage'] = self.record(value)
        row['producerReceipt'] = self.producer_receipt(row)
        return clocks._coverage(row, self.temporal_value, self.relation_value, self.original, self.entry, self.started, self.generated)


class ManifestSymbolicLinkTests(PrivateFixture):
    def regular(self):
        return dict(path='a.pem', mode='100644', bytes=1, sha256='d' * 64)

    def link(self, path='outside-link'):
        return dict(path=path, mode='120000', symlink='../../outside')

    def test_unselected_symlink_keeps_regular_file_hash_verification(self):
        records = TemporalRecords(self, [self.regular(), self.link()])
        self.assertEqual(len(records.proof().objects), 1)

    def test_selected_symlink_is_not_a_regular_source_file(self):
        records = TemporalRecords(self, [self.link('a.pem')])
        with self.assertRaisesRegex(b.Rejected, 'E_TEMPORAL_SELECTED_FILES'):
            records.proof()

    def test_symlink_and_regular_duplicate_path_reject(self):
        records = TemporalRecords(self, [self.regular(), self.link('a.pem')])
        with self.assertRaisesRegex(b.Rejected, 'E_TEMPORAL_INPUT_MANIFEST'):
            records.proof()

    def test_regular_file_without_hash_rejects_with_fixed_code(self):
        records = TemporalRecords(self, [dict(path='a.pem', mode='100644', bytes=1)])
        with self.assertRaisesRegex(b.Rejected, 'E_TEMPORAL_INPUT_MANIFEST'):
            records.proof()

    def test_symlink_with_regular_file_hash_rejects(self):
        records = TemporalRecords(self, [self.regular(), dict(self.link(), sha256='f' * 64)])
        with self.assertRaisesRegex(b.Rejected, 'E_TEMPORAL_INPUT_MANIFEST'):
            records.proof()


class TemporalAdmissionTests(PrivateFixture):
    def setUp(self):
        super().setUp()
        self.records = TemporalRecords(self)

    def test_bound_independent_records_admit_and_raw_objects_rederive_interval(self):
        temporal, relation, rows = self.records.admit()
        self.assertEqual(len(rows), 98)
        self.assertEqual(relation['finalSourceBindings'], self.records.source)
        proof = self.records.proof()
        report = dict(generated_at='2026-11-01T00:00:04Z', findings=[])
        native = dict(run=dict(generated_at='2026-11-01T00:00:04+00:00'), assets=[])
        clock = clocks.derive_clock(report, native, '2026-11-01T00:00:00+00:00', proof)
        self.assertEqual((clock['kind'], clock['assessmentTimeUnix']), ('interval_equivalent_not_observed', 1793491200))

    def manifest_proof(self, raw):
        r = self.records
        r.counter += 1
        r.input_ref = b.ref(self.write('manifest-' + str(r.counter) + '.json', raw))
        r.original['input']['manifest'] = r.input_ref
        r.entry['inputManifestSha256'] = r.input_ref['sha256']
        r.row = r.coverage_row('synthetic-0', 'primary-after')
        return r.proof()

    def test_array_manifest_matches_actual_corpus_shape_and_object_forms_remain_supported(self):
        self.assertIsInstance(self.records.proof(), clocks._CoverageProof)
        for value in (dict(files=[dict(path='a.pem', sha256='d' * 64)]),
                      dict(files={'a.pem': dict(sha256='d' * 64)})):
            with self.subTest(value=value):
                self.assertIsInstance(self.manifest_proof(json.dumps(value).encode()), clocks._CoverageProof)

    def test_manifest_duplicate_key_nonfinite_malformed_and_scalar_are_rejected(self):
        for raw in (b'[{"path":"a.pem","path":"a.pem","sha256":"' + b'd' * 64 + b'"}]',
                    b'[{"path":"a.pem","sha256":NaN}]',
                    b'[{"path":"a.pem","sha256":Infinity}]', b'[', b'null', b'42'):
            with self.subTest(raw=raw), self.assertRaises(b.Rejected):
                self.manifest_proof(raw)

    def relocated_row(self):
        r = self.records
        root = b.mkdir(self.root / 'recovered-input')
        original_root = r.original['input']['root']
        external_ignore = str(self.write('external-ignore', b'synthetic external ignore'))
        original_ignore = original_root + '/.gitignore'
        r.original['selection'] = dict(effectiveGlobalIgnore=external_ignore, files={
            original_ignore: dict(exists=True, sha256='f' * 64, resolved=original_ignore),
            external_ignore: dict(exists=True, sha256=b.ref(external_ignore)['sha256'], resolved=external_ignore)})
        current = copy.deepcopy(r.original['selection'])
        current['files'][str(root / '.gitignore')] = current['files'].pop(original_ignore)
        current['files'][str(root / '.gitignore')]['resolved'] = str(root / '.gitignore')
        r.entry.update(id='synthetic-0', population='primary', split='holdout', commit='a' * 40,
                       scope='synthetic fixed scope', inputManifest=r.input_ref['path'])
        registry_entry = dict(r.entry, root=str(root), inputManifest=r.input_ref['path'])
        registry = r.record(dict(repositories=[registry_entry]))
        config = b.document(r.row['config'])
        config.update(root=str(root), bindings=dict(config['bindings'], selection=b.digest(r.original['selection'])))
        config_ref = r.record(config)
        witness = copy.deepcopy(r.witness_value)
        witness.update(configurationSha256=config_ref['sha256'], bindings=config['bindings'])
        witness_ref = r.record(witness)
        coverage = copy.deepcopy(r.coverage_value)
        coverage.update(bindings=config['bindings'], witness=witness_ref, originalSelection=r.original['selection'],
            inputRelocation=dict(kind='temporal-input-relocation-v1', originalRoot=original_root, currentRoot=str(root),
                manifest=r.input_ref, recoveryRegistry=registry, registryEntry=registry_entry,
                selectionBefore=current, selectionAfter=current, sourceRelation=r.relation, evidence=r.evidence))
        row = dict(r.row, config=config_ref, witness=witness_ref, coverage=r.record(coverage))
        producer = b.document(r.producer_receipt(row))
        producer.update(selectionBefore=current, selectionAfter=current)
        row['producerReceipt'] = r.record(producer)
        return row

    def relocated_proof(self, row):
        r = self.records
        return clocks._coverage(row, r.temporal_value, r.relation_value, r.original, r.entry, r.started, r.generated)

    def test_relocated_root_uses_actual_selection_and_keeps_historical_binding(self):
        row = self.relocated_row()
        self.assertIsInstance(self.relocated_proof(row), clocks._CoverageProof)
        r = self.records
        coverage = b.document(row['coverage'])
        producer = b.document(row['producerReceipt'])
        self.assertEqual(coverage['originalSelection'], r.original['selection'])
        self.assertNotEqual(producer['selectionBefore'], r.original['selection'])
        self.assertEqual(coverage['bindings']['selection'], b.digest(r.original['selection']))
        del r.original['input']['manifest']['bytes']
        coverage['originalInput'] = copy.deepcopy(r.original['input'])
        coverage['inputRelocation']['manifest'] = copy.deepcopy(r.original['input']['manifest'])
        producer['inputBefore'] = producer['inputAfter'] = copy.deepcopy(r.original['input']['manifest'])
        row.update(coverage=r.record(coverage), producerReceipt=r.record(producer))
        self.assertIsInstance(self.relocated_proof(row), clocks._CoverageProof)

    def test_relocated_root_rejects_wrong_root_manifest_registry_pin_and_source_splice(self):
        row = self.relocated_row()
        r = self.records
        for field, value in [('currentRoot', str(self.root)), ('manifest', r.evidence),
                             ('sourceRelation', r.evidence), ('registryEntry', {}), ('originalRoot', '/wrong')]:
            coverage = b.document(row['coverage'])
            coverage['inputRelocation'][field] = value
            changed = dict(row, coverage=r.record(coverage))
            with self.subTest(field=field), self.assertRaises(b.Rejected):
                self.relocated_proof(changed)
        for field in ('commit', 'scope', 'inputManifestSha256'):
            coverage = b.document(row['coverage'])
            registry = b.document(coverage['inputRelocation']['recoveryRegistry'])
            registry['repositories'][0][field] = 'changed'
            coverage['inputRelocation']['registryEntry'] = registry['repositories'][0]
            coverage['inputRelocation']['recoveryRegistry'] = r.record(registry)
            with self.subTest(field=field), self.assertRaises(b.Rejected):
                self.relocated_proof(dict(row, coverage=r.record(coverage)))
        for field in ('inputRelocation',):
            coverage = b.document(row['coverage'])
            del coverage[field]
            with self.assertRaises(b.Rejected):
                self.relocated_proof(dict(row, coverage=r.record(coverage)))

    def test_relocated_root_rejects_selection_ignore_changes_and_before_after_splice(self):
        row = self.relocated_row()
        r = self.records
        original_coverage = b.document(row['coverage'])
        original_producer = b.document(row['producerReceipt'])
        current = original_producer['selectionBefore']
        external = current['effectiveGlobalIgnore']
        cases = []
        changed = copy.deepcopy(current)
        changed['effectiveGlobalIgnore'] = '/changed-global-ignore'
        cases.append(changed)
        for field, value in [('sha256', '0' * 64), ('resolved', '/outside-root/ignore'), ('exists', False)]:
            changed = copy.deepcopy(current)
            changed['files'][external][field] = value
            cases.append(changed)
        for after_only in (False, True):
            for selection in cases:
                coverage = copy.deepcopy(original_coverage)
                producer = copy.deepcopy(original_producer)
                for field in ('selectionAfter',) if after_only else ('selectionBefore', 'selectionAfter'):
                    producer[field] = selection
                    coverage['inputRelocation'][field] = selection
                with self.subTest(after_only=after_only, selection=selection), self.assertRaises(b.Rejected):
                    self.relocated_proof(dict(row, coverage=r.record(coverage), producerReceipt=r.record(producer)))

    def test_author_model_scope_and_stale_subjects_cannot_approve(self):
        r = self.records
        for change in [dict(reviewerId='synthetic-author'), dict(reviewerId='temporal_clock_plan_implementation'),
                       dict(reviewerModel='codex:gpt-6.1-terra@high'), dict(scopes=['implementation_correctness']),
                       dict(subjects={}), dict(harnessBindings={}), dict(verdict='PASS_SCOPED')]:
            r.refresh_review(**change)
            with self.subTest(change=change), self.assertRaises(b.Rejected):
                r.admit()

    def test_changed_source_and_unclassified_hunks_invalidate_proof(self):
        r = self.records
        r.source = dict(r.source, **{'crates/parser.rs': 'f' * 64})
        with self.assertRaises(b.Rejected):
            r.admit()
        r.source = r.relation_value['finalSourceBindings']
        changed = copy.deepcopy(r.relation_value)
        changed['historicalSourceBindings']['primary-after'] = dict(r.source, **{'crates/parser.rs': 'f' * 64})
        r.temporal_value['sourceRelation'] = r.record(changed)
        r.refresh_review()
        with self.assertRaisesRegex(b.Rejected, 'E_TEMPORAL_UNCLASSIFIED_HUNK'):
            r.admit()

    def test_hashed_metadata_without_actual_tool_and_build_inputs_is_rejected(self):
        r = self.records
        changed = dict(r.build_value, linkedSourceEvidence=None)
        r.temporal_value['witnessBuildReceipt'] = r.record(changed)
        r.refresh_review()
        with self.assertRaises(b.Rejected):
            r.admit()
        r.temporal_value['witnessBinary'] = dict(r.binary, sha256='0' * 64)
        r.refresh_review()
        with self.assertRaises(b.Rejected):
            r.admit()

    def test_raw_witness_binding_state_object_loss_and_quotas_fail_closed(self):
        r = self.records
        changes = []
        for field, value in [('source', '0' * 64), ('rules', '0' * 64),
                             ('input', '0' * 64), ('selection', '0' * 64)]:
            witness = copy.deepcopy(r.witness_value)
            witness['bindings'][field] = value
            changes.append(witness)
        for update in [dict(objects=[]), dict(diagnosticCodes=['E_QUOTA']), dict(ruleCount=139), dict(status='complete')]:
            changes.append(dict(r.witness_value, **update))
        for field, value in [('validityState', 'expired'), ('notBeforeUnix', None), ('notAfterUnix', -1),
                             ('sourceSha256', '0' * 64), ('objectIndex', True)]:
            witness = copy.deepcopy(r.witness_value)
            witness['objects'][0][field] = value
            changes.append(witness)
        for witness in changes:
            with self.subTest(witness=witness), self.assertRaises(b.Rejected):
                r.proof(witness)

    def test_partial_unsupported_and_read_failures_need_explicit_source_proof(self):
        r = self.records
        for status in ['partial', 'unsupported', 'unknown', 'read_failed', 'quota', 'complete']:
            witness = copy.deepcopy(r.witness_value)
            witness['files'][0]['status'] = status
            coverage = copy.deepcopy(r.coverage_value)
            coverage['files'] = witness['files']
            if status == 'complete':
                r.proof(witness, coverage)
            else:
                with self.subTest(status=status), self.assertRaises(b.Rejected):
                    r.proof(witness, coverage)

    def test_empty_native_assets_do_not_hide_raw_boundary_or_clock_emission(self):
        r = self.records
        witness = copy.deepcopy(r.witness_value)
        witness['objects'][0]['notBeforeUnix'] = 1793491202
        witness['objects'][0]['validityState'] = 'not_yet_valid'
        proof = r.proof(witness)
        with self.assertRaisesRegex(b.Rejected, 'E_CLOCK_INTERVAL_TRANSITION'):
            proof.prove_interval(r.started.second, r.generated.second)
        witness = copy.deepcopy(r.witness_value)
        witness['objects'][0]['matchingDefaultValidityRuleIds'] = ['CERT_EXPIRED']
        proof = r.proof(witness)
        with self.assertRaisesRegex(b.Rejected, 'E_CLOCK_EXACT_EMISSION_UNPROVEN'):
            proof.prove_interval(r.started.second, r.generated.second)

    def test_raw_hashes_without_bound_successful_producer_receipt_cannot_admit(self):
        r = self.records
        receipt = b.document(r.row['producerReceipt'])
        changes = [dict(execution='NotRun'), dict(exitCode=True), dict(timedOut=True), dict(scope='accuracy_scan'),
                   dict(sourceAfter={}), dict(configuration=r.binary), dict(commandDigest='0' * 64),
                   dict(output=r.binary), dict(selectionAfter={}), dict(cpuSeconds=1),
                   dict(startedUtc='2026-11-01T00:00:05+00:00')]
        for change in changes:
            row = dict(r.row, producerReceipt=r.record(dict(receipt, **change)))
            with self.subTest(change=change), self.assertRaises(b.Rejected):
                clocks._coverage(row, r.temporal_value, r.relation_value, r.original, r.entry, r.started, r.generated)

    def test_missing_one_project_or_duplicate_coverage_never_shrinks_denominator(self):
        r = self.records
        index = b.document(r.coverage_index)
        for rows in [index['rows'][:-1], index['rows'][:-1] + [index['rows'][0]]]:
            r.temporal_value['coverageIndex'] = r.record(dict(index, rows=rows))
            r.refresh_review()
            with self.subTest(count=len(rows)), self.assertRaises(b.Rejected):
                r.admit()

    def test_original_rule_files_are_rechecked_in_addition_to_manifest_hash(self):
        r = self.records
        (r.rules_directory / 'a.yml').write_bytes(b'changed synthetic rule content')
        with self.assertRaises(b.Rejected):
            r.proof()

    def test_raw_partial_status_remains_partial_only_with_bound_source_exclusion(self):
        r = self.records
        witness = copy.deepcopy(r.witness_value)
        witness['files'][0]['status'] = 'partial'
        witness['coverage']['analyzedFiles'] = 0
        witness['coverage']['partialFiles'] = 1
        witness['diagnosticCodes'] = ['detector_partial']
        coverage = copy.deepcopy(r.coverage_value)
        r.original['analysisStatus']['inspection'] = 'Partial'
        coverage['originalStatus'] = r.original['analysisStatus']
        coverage['files'], coverage['producerCoverage'] = witness['files'], witness['coverage']
        coverage['timeIndependentStates'] = {'a.pem': dict(status='partial', sourceSha256='d' * 64,
            closure=r.relation, evidence=r.evidence)}
        coverage['timeIndependentDiagnostics'] = {'detector_partial': dict(closure=r.relation, evidence=r.evidence)}
        r.proof(witness, coverage).prove_interval(r.started.second, r.generated.second)
        self.assertEqual(witness['files'][0]['status'], 'partial')



class OutputReplayTests(PrivateFixture):
    def test_live_and_archived_outputs_share_clock_validation(self):
        row = observed_row()
        out = b.mkdir(self.root / 'output')
        for name in b.FORMATS:
            (out / name).write_text('{}' if name.endswith('.json') else 'synthetic')
        report = dict(generated_at='2026-11-01T00:00:04Z', findings=[])
        native = dict(run=dict(generated_at='2026-11-01T00:00:04+00:00'))
        (out / 'report.json').write_text(json.dumps(report))
        (out / 'scan.inventory.v1.json').write_text(json.dumps(native))
        execution.verify_output_clocks(out, row)
        expected = {name: {key: value for key, value in b.ref(out / name).items() if key != 'path'} for name in b.FORMATS}
        archive = storage.archive_outputs(out, b.mkdir(self.root / 'compressed'), expected)
        terminal = dict(bindings=dict(clock=row), artifacts=archive)
        execution.verify_archived_clocks(terminal)
        changed = copy.deepcopy(row)
        changed['clock']['assessmentTimeUnix'] = 1793491202
        changed['clock']['originalNativeGeneratedAt'] = '2026-11-01T00:00:03+00:00'
        with self.assertRaises(b.Rejected):
            execution.verify_archived_clocks(dict(terminal, bindings=dict(clock=changed)))


class CompleteClockPlanTests(PrivateFixture):
    def historical_fixture(self, records, mutate_outputs=None):
        from test_assessment import synthetic_outputs
        oss, _, _ = b.import_auditors()
        source_root = b.mkdir(self.root / 'source')
        source = source_root / 'a.pem'
        source.write_text('Public synthetic certificate fixture identity, never parsed or executed.\n')
        file_sha = b.ref(source)['sha256']
        manifest = records.record(dict(files=[dict(path='a.pem', sha256=file_sha, bytes=source.stat().st_size, mode='100644')]))
        entries = {identity: dict(id=identity, population=population, split='synthetic', commit='a' * 40,
            scope='synthetic static fixture', inputManifest=manifest['path'], inputManifestSha256=manifest['sha256'])
            for population, count in [('primary', 90), ('comparison', 8)]
            for identity in [population + '-' + str(i) for i in range(count)]}
        scanner = dict(binary=records.binary, rules={}, rulesSha256=b.digest({}),
                       productSource=dict(sourceBindings=records.source))
        configuration = dict(repeats=1, requiredArtifacts=list(b.FORMATS), environment={})
        coordinator = {name: 'synthetic' for name in ('python', 'platform', 'machine', 'cpuCount', 'hardware', 'rust', 'cargo')}
        selection = dict(files={}, effectiveGlobalIgnore='/synthetic/ignore')
        original_input = dict(root=str(source_root), manifest=manifest, commit='a' * 40)
        policy_rows, originals = [], {}
        for phase_id, (population, phase, _) in clocks.PHASES.items():
            directory = b.mkdir(self.root / phase_id)
            selected = [identity for identity, entry in entries.items() if entry['population'] == population]
            common = dict(phase=phase, selectedIds=selected, selectedEntries={identity: entries[identity] for identity in selected},
                expectedSamples=1, configuration=configuration, scanner=scanner, protocol=dict(synthetic=True),
                harness={'synthetic.py': 'f' * 64}, coordinatorEnvironment=coordinator)
            b.publish(directory / 'started.json', common)
            rows = []
            for identity in selected:
                attempt = b.mkdir(directory / identity)
                output = b.mkdir(attempt / 'output')
                values = synthetic_outputs(output)
                if mutate_outputs is not None:
                    mutate_outputs(identity, phase_id, values)
                    for name in ('report.json', 'scan.inventory.v1.json'):
                        (output / name).write_text(json.dumps(values[name]))
                (output / 'stdout.log').write_bytes(b'')
                (output / 'stderr.log').write_bytes(b'')
                bindings = dict(registryEntry=entries[identity], phase=phase, sample=0, input=original_input,
                    selection=selection, configuration=configuration, scanner=scanner, protocol=common['protocol'],
                    harness=common['harness'], environment=coordinator)
                started = dict(repositoryId=identity, phase=phase, bindings=bindings, startedUtc='2026-11-01T00:00:00+00:00')
                b.publish(attempt / 'started.json', started)
                artifacts, assessment = oss._artifacts(output, configuration['requiredArtifacts'])
                terminal = dict(execution='Succeeded', startedDigest=b.digest(started), executionKey=b.digest(bindings),
                    artifacts=artifacts, assessment=assessment)
                terminal_ref = b.publish(attempt / 'terminal.json', terminal)
                rows.append(dict(repositoryId=identity, sample=0, population=population, split='synthetic',
                    execution='Succeeded', receipt=terminal_ref, assessment=assessment))
                originals[(identity, phase_id)] = dict(bindings, analysisStatus=assessment)
            invocation = dict(common, rows=rows)
            actual = b.publish(directory / 'invocation.json', invocation)
            wrapper = records.record(dict(invocation, receiptPath=actual['path']))
            policy_rows.append(dict(id=phase_id, population=population, phase=phase, binding=wrapper))
        policy = records.record(dict(kind='historical-score-policy', invocations=policy_rows))
        return policy, entries, originals, file_sha

    def test_real_historical_admission_and_rederivation_without_clock_or_review_mocks(self):
        r = TemporalRecords(self)
        policy_ref, entries, originals, source_sha = self.historical_fixture(r)
        r.harness = b.harness_bindings()
        r.temporal_value['harnessBindings'] = r.harness
        rows = []
        for identity, entry in entries.items():
            phase = entry['population'] + '-after'
            r.original = originals[(identity, phase)]
            r.entry = entry
            row = r.coverage_row(identity, phase)
            witness = copy.deepcopy(r.witness_value)
            witness['objects'][0]['sourceSha256'] = source_sha
            witness['files'][0]['sourceSha256'] = source_sha
            row['witness'] = r.record(witness)
            coverage = copy.deepcopy(r.coverage_value)
            coverage['witness'] = row['witness']
            coverage['files'] = witness['files']
            coverage['selectedFiles'] = {'a.pem': source_sha}
            row['coverage'] = r.record(coverage)
            row['producerReceipt'] = r.producer_receipt(row)
            rows.append(row)
        r.temporal_value['coverageIndex'] = r.record(dict(kind='temporal-coverage-index-v1', rows=rows))
        r.refresh_review()
        with patch.object(clocks, 'POLICY_SHA256', policy_ref['sha256']):
            history = clocks.admit_history(policy_ref, [])
            temporal, relation, coverage_rows = r.admit()
            derived = clocks.derive_all(policy_ref, [], temporal, relation, coverage_rows)
            plan_ref = r.record(derived)
            plan = clocks.ClockPlan.admit(plan_ref, r.temporal, [], r.source, ['synthetic-author'], r.overlay)
            self.assertEqual(len(derived['rows']), 196)
            self.assertEqual(sum('clock' in row for row in derived['rows']), 98)
            self.assertEqual(plan.for_project('primary-0', 'primary-after').detector_second, 1793491200)
            self.assertTrue(all('clock' not in row for row in derived['rows'] if row['phaseId'].endswith('-baseline')))
            with self.assertRaises(b.Rejected):
                plan.for_project('primary-0', 'primary-baseline')
            changed = copy.deepcopy(derived)
            after = next(row for row in changed['rows'] if row['phaseId'] == 'primary-after')
            after['clock']['assessmentTimeUnix'] += 1
            rehashed = r.record(changed)
            with self.assertRaisesRegex(b.Rejected, 'E_CLOCK_MAP_DERIVATION'):
                clocks.ClockPlan.admit(rehashed, r.temporal, [], r.source, ['synthetic-author'], r.overlay)
            copy_row = plan[('primary-0', 'primary-after')]
            copy_row['clock']['assessmentTimeUnix'] = 0
            self.assertEqual(plan.for_project('primary-0', 'primary-after').detector_second, 1793491200)
