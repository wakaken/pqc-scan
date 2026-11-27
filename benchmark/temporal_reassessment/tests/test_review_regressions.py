import copy
import json
import os
from pathlib import Path
import unittest
from unittest.mock import patch

from test_execution import PrivateFixture
from test_assessment import synthetic_outputs
import adapter
import boundary as b
import delta
import execution
import clocks
import comparison
import main
import storage


class LedgerReviewIndependenceTests(PrivateFixture):
    def ledger(self, identity):
        directory = b.mkdir(self.root / ('case-' + str(len(list(self.root.iterdir())))))
        source = {'synthetic-source': 'a' * 64}
        clock = b.publish(directory / 'clock.json', dict(policy={}))
        admission = b.publish(directory / 'admission.json', dict(sourceBindings=source,
            authorId='synthetic-coordinator', authorIds=['synthetic-ledger-author'], clockMap=clock))
        index = dict(path='/synthetic/index', sha256='b' * 64)
        ledger = dict(kind='execution-only-oss98-compatibility-allowances-v1', index=index,
                      sourceBindingsDigest=b.digest(source), projects={})
        ledger['independentReview'] = b.publish(directory / 'review.json', dict(verdict='PASS',
            reviewerModel='codex:gpt-6.1-sol@high', reviewerId=identity, allowancesDigest=b.digest(ledger)))
        return index, b.publish(directory / 'ledger.json', ledger), admission

    def test_authors_and_invalid_reviewer_identities_reject_before_comparison(self):
        identities = ['synthetic-coordinator', 'synthetic-ledger-author', *sorted(clocks.AUTHORS),
                      None, '', 1, [], {}]
        for identity in identities:
            with self.subTest(identity=identity):
                index, ledger, admission = self.ledger(identity)
                with patch.object(execution, 'load_index', return_value=({'admission': admission}, {})), \
                     patch.object(clocks, 'admit_history', side_effect=AssertionError('E_GATE_NOT_ENFORCED')) as history:
                    with self.assertRaisesRegex(b.Rejected, 'E_DELTA_LEDGER_REVIEW'):
                        comparison.compare_all(index, ledger)
                    history.assert_not_called()

    def test_independent_reviewer_reaches_historical_comparison(self):
        class GateAccepted(Exception):
            pass

        index, ledger, admission = self.ledger('synthetic-independent-reviewer')
        with patch.object(execution, 'load_index', return_value=({'admission': admission}, {})), \
             patch.object(clocks, 'admit_history', side_effect=GateAccepted) as history:
            with self.assertRaises(GateAccepted):
                comparison.compare_all(index, ledger)
            history.assert_called_once_with({}, [])


class ReviewedIndexTests(PrivateFixture):
    def setUp(self):
        super().setUp()
        self.oss, external, auditor = b.import_auditors()
        self.validation, self.schemas = auditor.validators()
        self.output = b.mkdir(self.root / 'run-synthetic')
        self.source = b.mkdir(self.root / 'source')
        source_file = self.source / 'a.rs'
        source_file.write_text('public synthetic source\n')
        self.files = [dict(path='a.rs', bytes=source_file.stat().st_size,
                           sha256=b.ref(source_file)['sha256'], mode='100644')]
        self.manifest = self.write('input.json', json.dumps(dict(commit='a' * 40, files=self.files)).encode())
        self.admission_ref = dict(path='/synthetic/final-admission', sha256='a' * 64)
        self.admission = dict(sourceBindings={'required-source': 'b' * 64}, harnessBindings={'required-harness': 'c' * 64},
                              scannerEnvironment={name: os.environ.get(name) for name in ('LANG', 'LC_ALL', 'LC_CTYPE', 'TZ')},
                              adapterBinary=dict(path='/synthetic/exact-adapter', sha256='d' * 64))
        self.registry = [dict(id='synthetic-' + str(i), population='primary' if i < 90 else 'comparison',
                         split='public-synthetic', root=str(self.source), commit='a' * 40, scope='synthetic pinned tree',
                         inputManifest=str(self.manifest), inputManifestSha256=b.ref(self.manifest)['sha256']) for i in range(98)]
        self.clock_rows = {}
        self.history = {population + '-after': dict(attempts={}) for population in ('primary', 'comparison')}
        selection = external.selection_bindings(self.source, cwd=b.ROOT)
        for row in self.registry:
            _, _, inputs = self.oss._acquisition(row, self.output)
            original = dict(input=inputs, selection=selection, configuration=dict(environment=self.admission['scannerEnvironment']))
            phase = row['population'] + '-after'
            self.history[phase]['attempts'][row['id']] = (dict(repositoryId=row['id']), None, original)
            self.clock_rows[(row['id'], phase)] = dict(repositoryId=row['id'], phaseId=phase,
                                                       clock=dict(assessmentTimeUnix=1793491201, kind='observed_original_second',
                                                           originalGeneratedAt='2026-11-01T00:00:04Z',
                                                           originalNativeGeneratedAt='2026-11-01T00:00:04+00:00',
                                                           originalStartedUtc='2026-11-01T00:00:00+00:00',
                                                           generatedSecond=1793491204, generatedNanosecond=0,
                                                           firstPossibleSecond=1793491200, lastPossibleSecond=1793491204))
        spool = b.mkdir(self.root / 'output')
        synthetic_outputs(spool)
        native_path = spool / 'scan.inventory.v1.json'
        native = json.loads(native_path.read_text())
        native['run']['generated_at'] = '2026-11-01T00:00:04+00:00'
        native_path.write_text(json.dumps(native))
        artifacts, assessment = execution.assess(spool, {}, self.validation)
        archive = storage.archive_outputs(spool, b.mkdir(self.root / 'compressed'), artifacts)
        initial = dict(kind='execution-only-oss98-started-v1', admission=self.admission_ref,
                       selectedIds=[row['id'] for row in self.registry], expectedSamples=1, configuration=execution.RESOURCE)
        started = b.publish(self.output / 'started.json', initial)
        self.terminals = {}
        rows = []
        for row in self.registry:
            project = b.mkdir(self.output / row['id'])
            original = self.history[row['population'] + '-after']['attempts'][row['id']][2]
            bindings = dict(kind='execution-only-oss98-bindings-v1', registryEntry=row, input=original['input'],
                            originalInput=original['input'], relocation=dict(originalRoot=str(self.source), currentRoot=str(self.source),
                            sameManifestAndFileHashes=True, physicalCopyOnWriteClaim=False),
                            clock=self.clock_rows[(row['id'], row['population'] + '-after')], selection=selection,
                            configuration=execution.RESOURCE, admission=self.admission_ref,
                            sourceBindings=self.admission['sourceBindings'], harnessBindings=self.admission['harnessBindings'],
                            environment=self.admission['scannerEnvironment'], validationEnvironment=b.validation_environment(), schemas=self.schemas)
            command = ['/synthetic/exact-adapter', 'scan', str(self.source), '--format', 'all', '--diagnostics',
                       '--inventory-json', '--cyclonedx-17', '--threads', '1', '--max-file-size-mib', '2',
                       '--lang', 'en', '--rules-dir', str(b.ROOT / 'rules/default'), '--out-dir', str(project / 'output')]
            environment = {name: clocks.ProjectReplay._from_row(bindings['clock']).bind_process(self.admission['scannerEnvironment']).get(name)
                           for name in ('LANG', 'LC_ALL', 'LC_CTYPE', 'TZ', adapter.DETECTOR_ENV, adapter.GENERATED_ENV)}
            process = dict(kind='execution-only-process-v1', startedUtc='2026-11-01T00:00:00+00:00',
                           endedUtc='2026-11-01T00:00:04+00:00', execution='Succeeded',
                           commandDigest=b.digest(command), cwdDigest=b.digest(str(b.ROOT)), environmentDigest=b.digest(environment),
                           exitCode=0, timedOut=False, measurement=None, resources=execution.RESOURCE,
                           errorCode=None, errorKind=None, stdout=dict(state='NotCaptured', destination='DEVNULL'),
                           stderr=dict(state='NotCaptured', destination='DEVNULL'))
            terminal = dict(kind='execution-only-oss98-terminal-v1', executionKey=b.digest(bindings), bindings=bindings,
                            execution='Succeeded', process=process, command=command, assessment=assessment, artifacts=archive,
                            evaluationComplete=False, performance=None)
            b.publish(project / 'started.json', dict(bindings=bindings, executionKey=terminal['executionKey']))
            self.terminals[row['id']] = terminal
            reference = b.publish(project / 'terminal.json', terminal)
            rows.append(dict(repositoryId=row['id'], population=row['population'], split=row['split'], sample=0,
                             terminal=reference, execution='Succeeded', inputManifestSha256=row['inputManifestSha256'],
                             commit=row['commit'], artifacts=archive, assessment=assessment))
        self.index = dict(kind='execution-only-after-repair-artifact-index-v1', admission=self.admission_ref, started=started,
                          rows=rows, executionComplete=True, expectedProjects=98, repeats=1)
        self.index_path = self.output / 'index.json'
        b.publish(self.index_path, self.index)
        self.admit_patch = patch.object(execution, 'admit', return_value=(self.admission, self.registry, self.clock_rows, self.history))
        self.admit_patch.start()
        self.addCleanup(self.admit_patch.stop)
        self.path_patch = patch.object(execution, '__file__', str(self.root / 'execution.py'))
        self.path_patch.start()
        self.storage_patch = patch.object(b, 'OWN', self.root)
        self.storage_patch.start()
        self.addCleanup(self.storage_patch.stop)
        self.addCleanup(self.path_patch.stop)

    def overwrite_synthetic(self, path, value):
        path.write_text(json.dumps(value))

    def replace_terminal(self, terminal, *, started=True):
        project = self.output / 'synthetic-0'
        self.overwrite_synthetic(project / 'terminal.json', terminal)
        self.index['rows'][0]['terminal'] = b.ref(project / 'terminal.json')
        if started:
            self.overwrite_synthetic(project / 'started.json', dict(bindings=terminal['bindings'], executionKey=terminal['executionKey']))
        self.overwrite_synthetic(self.index_path, self.index)

    def test_complete_bound_98_project_index_is_accepted(self):
        _, rows = execution.load_index(b.ref(self.index_path))
        self.assertEqual(len(rows), 98)

    def test_self_consistent_wrong_binding_is_rejected_before_archive_read(self):
        changes = dict(sourceBindings={'wrong-source': '0' * 64}, harnessBindings={},
                       clock=dict(clock=dict(assessmentTimeUnix=0)), environment={}, validationEnvironment={},
                       schemas={}, configuration={}, input={}, originalInput={}, selection={}, relocation={})
        original = self.terminals['synthetic-0']
        for name, value in changes.items():
            with self.subTest(name=name):
                terminal = copy.deepcopy(original)
                terminal['bindings'][name] = value
                terminal['executionKey'] = b.digest(terminal['bindings'])
                self.replace_terminal(terminal)
                with patch.object(storage, 'validate_matrix') as archive, self.assertRaises(b.Rejected):
                    execution.load_index(b.ref(self.index_path))
                archive.assert_not_called()

    def test_missing_or_wrong_terminal_and_process_receipts_are_rejected(self):
        mutations = [(('kind',), None), (('process',), None), (('command',), ['/synthetic/wrong']),
                     (('process', 'kind'), 'wrong'), (('process', 'exitCode'), 7), (('process', 'exitCode'), False),
                     (('process', 'execution'), 'Failed'), (('process', 'timedOut'), True),
                     (('process', 'commandDigest'), 'wrong'), (('process', 'cwdDigest'), 'wrong'),
                     (('process', 'environmentDigest'), 'wrong'), (('process', 'resources'), {}),
                     (('process', 'errorCode'), 'E_SCANNER_EXIT_NONZERO'),
                     (('process', 'endedUtc'), '2026-10-31T23:59:59+00:00')]
        for path, value in mutations:
            with self.subTest(path=path, value=value):
                terminal = copy.deepcopy(self.terminals['synthetic-0'])
                target = terminal
                for key in path[:-1]:
                    target = target[key]
                if value is None:
                    target.pop(path[-1])
                else:
                    target[path[-1]] = value
                self.replace_terminal(terminal)
                with patch.object(storage, 'validate_matrix') as archive, self.assertRaises(b.Rejected):
                    execution.load_index(b.ref(self.index_path))
                archive.assert_not_called()

    def test_project_started_and_index_started_are_bound(self):
        project_started = self.output / 'synthetic-0' / 'started.json'
        self.overwrite_synthetic(project_started, dict(bindings={}, executionKey='wrong'))
        with self.assertRaises(b.Rejected):
            execution.load_index(b.ref(self.index_path))
        self.replace_terminal(copy.deepcopy(self.terminals['synthetic-0']))
        self.overwrite_synthetic(self.output / 'started.json', dict(kind='wrong'))
        with self.assertRaises(b.Rejected):
            execution.load_index(b.ref(self.index_path))

    def test_current_input_bytes_are_reauthenticated(self):
        (self.source / 'a.rs').write_text('changed synthetic source\n')
        with self.assertRaises(ValueError):
            execution.load_index(b.ref(self.index_path))

    def test_resume_rejects_wrong_source_before_reassessment_or_launch(self):
        terminal = copy.deepcopy(self.terminals['synthetic-0'])
        terminal['bindings']['sourceBindings'] = {'wrong-source': '0' * 64}
        terminal['executionKey'] = b.digest(terminal['bindings'])
        self.replace_terminal(terminal)
        self.index_path.unlink()
        with patch.object(b, 'verify_source'), patch.object(b, 'harness_bindings', return_value=self.admission['harnessBindings']), \
                patch.object(execution, 'unpack_assess') as reassess, patch.object(execution, 'spawn') as launch, \
                self.assertRaises(b.Rejected):
            execution.run(self.admission_ref, self.output)
        reassess.assert_not_called()
        launch.assert_not_called()

    def test_original_forged_98_row_probe_is_rejected(self):
        for row in self.index['rows']:
            terminal = self.terminals[row['repositoryId']]
            terminal.pop('kind')
            terminal.pop('process')
            terminal['bindings']['sourceBindings'] = {'wrong-source': '0' * 64}
            terminal['executionKey'] = b.digest(terminal['bindings'])
            project = self.output / row['repositoryId']
            self.overwrite_synthetic(project / 'terminal.json', terminal)
            self.overwrite_synthetic(project / 'started.json', dict(bindings=terminal['bindings'], executionKey=terminal['executionKey']))
            row['terminal'] = b.ref(project / 'terminal.json')
        self.overwrite_synthetic(self.index_path, self.index)
        with self.assertRaises(b.Rejected):
            execution.load_index(b.ref(self.index_path))


class ReviewedSchemaTests(PrivateFixture):
    def setUp(self):
        super().setUp()
        _, _, self.audit = b.import_auditors()
        self.validation, _ = self.audit.validators()
        self.output = b.mkdir(self.root / 'output')

    def test_malformed_assets_preserve_schema_evidence_and_block_projection(self):
        for assets in ([dict(id='asset-v1-' + 'a' * 64)], ['synthetic-private-instance-value']):
            with self.subTest(assets=assets):
                values = synthetic_outputs(self.output)
                native = values['scan.inventory.v1.json']
                native['assets'] = assets
                with self.assertRaises((KeyError, TypeError)):
                    self.audit.projection_checks(*(values[name] for name in ('report.json', 'scan.inventory.v1.json',
                        'report.sarif', 'cbom.json', 'dependency-sbom.json', 'cbom.cdx-1.7.json', 'dependency-sbom.cdx-1.7.json')))
                (self.output / 'scan.inventory.v1.json').write_text(json.dumps(native))
                with self.assertRaises(execution.AssessmentRejected) as raised:
                    execution.assess(self.output, {}, self.validation)
                record = raised.exception.evidence
                self.assertEqual(record['errorCode'], 'E_OUTPUT_SCHEMA')
                self.assertEqual(record['schemaValidation'], 'Failed')
                self.assertGreater(record['schemaErrorsReportedCount'], 0)
                self.assertTrue(record['schemaErrorsComplete'])
                self.assertFalse(record['schemaErrorsTruncated'])
                self.assertEqual(record['exportProjectionReferences'], 'BlockedBySchema')
                self.assertEqual(record['nativeSourceReferences'], 'NotRun')
                self.assertNotIn('synthetic-private-instance-value', json.dumps(record))
                self.assertTrue(any(row['path'][:2] == ['assets', 0] for row in record['schemaErrors']))

    def test_schema_error_list_is_bounded_with_explicit_truncation(self):
        values = synthetic_outputs(self.output)
        values['scan.inventory.v1.json']['assets'] = [{}] * 50
        (self.output / 'scan.inventory.v1.json').write_text(json.dumps(values['scan.inventory.v1.json']))
        with self.assertRaises(execution.AssessmentRejected) as raised:
            execution.assess(self.output, {}, self.validation)
        record = raised.exception.evidence
        self.assertEqual(record['schemaErrorsReportedCount'], 32)
        self.assertEqual(len(record['schemaErrors']), 32)
        self.assertFalse(record['schemaErrorsComplete'])
        self.assertTrue(record['schemaErrorsTruncated'])


class ReviewedNativeTests(PrivateFixture):
    def test_schema_valid_algorithm_drift_cannot_be_waived_as_wording(self):
        import accuracy_native
        output = b.mkdir(self.root / 'output')
        before = synthetic_outputs(output)['scan.inventory.v1.json']
        before['assets'] = [dict(id='algorithm-v1-' + 'a' * 64, kind='algorithm', identity_basis='public synthetic identity',
                                 attributes=dict(namespace='named', identifier='RSA', purpose='reference', parameters={}),
                                 relations=[], uncertainty=[])]
        after = copy.deepcopy(before)
        after['assets'][0]['attributes']['identifier'] = 'ECDSA'
        _, _, audit = b.import_auditors()
        validation, _ = audit.validators()
        for value in (before, after):
            accuracy_native.validate_native(value, {}, validation['scan.inventory.v1.json'].schema)
        change = dict(path=['assets', 0, 'attributes', 'identifier'], reason='description_wording', independentlyApproved=True,
                      evidence=b.ref(self.write('proof.json', b'{"kind":"public synthetic independent proof"}')),
                      beforeDigest=b.digest('RSA'), afterDigest=b.digest('ECDSA'))
        with self.assertRaises(b.Rejected):
            delta.compare_native(before, after, [change])
        self.assertFalse(delta.compare_native(before, after, [])['passed'])
        self.assertTrue(delta.compare_native(before, copy.deepcopy(before), [])['passed'])

    def test_boolean_to_number_is_an_unapproved_json_change(self):
        result = delta.compare_values({'rows': [True]}, {'rows': [1]}, [])
        self.assertFalse(result['passed'])
        self.assertEqual(result['unexpectedPaths'], [['rows', 0]])

    def test_exact_numeric_serialization_change_is_not_silent(self):
        result = delta.compare_values({'rows': [1]}, {'rows': [1.0]}, [])
        self.assertFalse(result['passed'])
        self.assertEqual(result['unexpectedPaths'], [['rows', 0]])
        self.assertTrue(delta.compare_values({'rows': [1.0]}, {'rows': [1.0]}, [])['passed'])

    def test_fix_author_cannot_approve_own_candidate(self):
        review = dict(verdict='PASS', reviewerModel='codex:gpt-6.1-sol@high',
                      reviewerId='execution_only_harness_review_fixes', harnessBindings={})
        with patch.object(b, 'document', return_value=review), self.assertRaises(b.Rejected):
            main.review_gate(dict(path='/synthetic/review', sha256='a' * 64))

    def test_temporal_design_gate_blocks_even_an_enabled_admission(self):
        admission = dict(kind='execution-only-oss98-final-admission-v1', eligibleActualRuns=True,
                         features=[], performanceDeferredByUser=True)
        with patch.object(b, 'document', return_value=admission) as documents, \
                patch.object(b, 'verify_source') as source, self.assertRaises(b.Rejected) as raised:
            execution.admit(dict(path='/synthetic/admission', sha256='a' * 64))
        self.assertEqual(str(raised.exception), 'E_TEMPORAL_REPLAY_DESIGN_UNIMPLEMENTED')
        self.assertEqual(documents.call_count, 1)
        source.assert_not_called()


if __name__ == '__main__':
    unittest.main()
