import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import unittest
from unittest.mock import patch

from test_execution import PrivateFixture
from test_assessment import synthetic_outputs
import adapter
import boundary as b
import clocks
import execution as e
import fresh_cohort as f
import main


class ReplayRows(dict):
    def for_project(self, identity, phase):
        return clocks.ProjectReplay._from_row(self[(identity, phase)])


class FreshFixture(PrivateFixture):
    def setUp(self):
        super().setUp()
        owner_patch = patch.object(b, 'OWN', self.root)
        owner_patch.start()
        self.addCleanup(owner_patch.stop)
        self.source = b.mkdir(self.root / 'source')
        source = self.source / 'a.rs'
        source.write_text('public synthetic source\n')
        self.files = [dict(path='a.rs', bytes=source.stat().st_size, sha256=b.ref(source)['sha256'], mode='100644')]
        self.manifest = self.write('input.json', json.dumps(dict(commit='a' * 40, files=self.files)).encode())
        self.rows = [dict(id='synthetic-' + str(i), population='primary' if i < 90 else 'comparison',
                         split='public-synthetic', root=str(self.source), commit='a' * 40, scope='synthetic pinned tree',
                         inputManifest=str(self.manifest), inputManifestSha256=b.ref(self.manifest)['sha256']) for i in range(98)]
        self.oss, self.external, audit = b.import_auditors()
        self.validation, self.schemas = audit.validators()
        self.source_map = {'synthetic-source': b.ref(source)['sha256']}
        self.harness_map = {str(Path(f.__file__)): b.ref(Path(f.__file__))['sha256']}
        self.start_patch(patch.object(b, 'source_bindings', side_effect=lambda root=b.ROOT: {'synthetic-source': b.ref(source)['sha256']}))
        self.start_patch(patch.object(b, 'harness_bindings', side_effect=lambda: {str(Path(f.__file__)): b.ref(Path(f.__file__))['sha256']}))
        binary = b.ref(self.write('binary', b'public synthetic adapter'))
        shipping = b.ref(self.write('shipping', b'public synthetic shipping'))
        self.admission = dict(sourceEpoch='public-new-epoch', freshCohortEligible=True, sourceBindings=self.source_map,
                              harnessBindings=self.harness_map, validationEnvironment=b.validation_environment(),
                              scannerEnvironment={name: os.environ.get(name) for name in ('LANG', 'LC_ALL', 'LC_CTYPE', 'TZ')},
                              adapterBinary=binary, shippingBinary=shipping, authorId='/root/product-author', authorIds=['other-author'])
        self.admission_binding = b.publish(self.root / 'admission.json', self.admission)
        self.clock_rows = ReplayRows()
        self.history = {population + '-after': dict(attempts={}) for population in ('primary', 'comparison')}
        selection = self.external.selection_bindings(self.source, cwd=b.ROOT)
        self.output = self.root / 'run-fresh'
        for row in self.rows:
            _, _, inputs = self.oss._acquisition(row, self.output)
            original = dict(input=inputs, selection=selection, configuration=dict(environment=self.admission['scannerEnvironment']))
            phase = row['population'] + '-after'
            self.history[phase]['attempts'][row['id']] = (dict(repositoryId=row['id']), None, original)
            self.clock_rows[(row['id'], phase)] = dict(repositoryId=row['id'], phaseId=phase,
                clock=dict(assessmentTimeUnix=1793491201, kind='observed_original_second',
                           originalGeneratedAt='2026-11-01T00:00:04Z', originalNativeGeneratedAt='2026-11-01T00:00:04+00:00',
                           originalStartedUtc='2026-11-01T00:00:00+00:00', generatedSecond=1793491204,
                           generatedNanosecond=0, firstPossibleSecond=1793491200, lastPossibleSecond=1793491204))
        self.start_patch(patch.object(e, 'admit', return_value=(self.admission, self.rows, self.clock_rows, self.history)))
        self.start_patch(patch.object(f.shutil, 'disk_usage', return_value=type('Disk', (), {'free': 8 * 1024**3})()))
        self.calls = []
        self.states = {}
        self.hook = None
        self.start_patch(patch.object(f.subprocess, 'run', side_effect=self.scanner_io))

    def start_patch(self, item):
        value = item.start()
        self.addCleanup(item.stop)
        return value

    def scanner_io(self, command, **kwargs):
        output = Path(command[-1])
        identity = output.parent.parent.name
        attempt = int(output.parent.name.rsplit('-', 1)[1])
        self.calls.append((identity, attempt, kwargs['timeout']))
        self.assertFalse(kwargs['shell'])
        self.assertIs(kwargs['stdout'], subprocess.DEVNULL)
        self.assertIs(kwargs['stderr'], subprocess.DEVNULL)
        if self.hook:
            self.hook(identity, attempt)
        state = self.states.get((identity, attempt), 'Succeeded')
        if state == 'TimedOut':
            raise subprocess.TimeoutExpired(command, kwargs['timeout'])
        if state == 'Failed':
            return subprocess.CompletedProcess(command, 7)
        b.mkdir(output)
        synthetic_outputs(output)
        if state == 'InvalidEvidence':
            (output / 'report.json').write_text('{}')
        return subprocess.CompletedProcess(command, 0)

    def prepare(self, fallback=()):
        policy = dict(kind=f.KINDS['policy'], sourceEpoch='public-new-epoch', admission=self.admission_binding,
                      allowedFallbackIds=[row['id'] for row in self.rows if row['id'] in fallback],
                      initialTimeoutSeconds=600, fallbackTimeoutSeconds=3600, maxAttemptsPerProject=2,
                      threads=1, repeats=1, minFreeGiB=4, maxFileSizeMiB=2, maxArtifactBytes=b.MAX,
                      maxAttemptArtifactBytes=b.TOTAL, requiredArtifacts=list(b.FORMATS), performance=None,
                      performanceObservations=False, authorIds=['/root/policy-author'])
        self.policy = b.publish(self.root / 'policy.json', policy)
        self.plan = f.prepare_plan(self.admission_binding, self.policy, self.root / 'plan.json', self.output)
        plan = b.document(self.plan)
        review = dict(kind=f.KINDS['plan-review'], verdict='PASS', reviewerModel='codex:gpt-6.1-sol@high',
                      reviewerId='public-independent-reviewer', plan=self.plan,
                      implementationCorrectness='PASS', testAdequacy='PASS', evidenceAuthenticity='PASS', resourceDifferences='PASS',
                      **{name: plan[name] for name in ('sourceBindings', 'harnessBindings', 'validationEnvironment')})
        self.review = b.publish(self.root / 'review.json', review)
        return plan

    def stop_at(self, identity):
        def hook(current, attempt):
            if current == identity:
                (self.source / 'a.rs').write_text('changed public synthetic source\n')
        self.hook = hook

    def fixture_index(self):
        plan = b.document(self.plan)
        b.mkdir(self.output)
        started = b.publish(self.output / 'started.json', dict(kind=f.KINDS['started'], plan=self.plan,
                            review=self.review, selectedIds=[row['id'] for row in self.rows]))
        output_rows = []
        for row in self.rows:
            project = b.mkdir(self.output / row['id'])
            attempts, terminals, protected = [], [], []
            for number in range(2 if row['id'] in b.document(self.policy)['allowedFallbackIds'] else 1):
                root, files, bindings, command = f._project(self.admission, self.admission_binding, row, self.clock_rows,
                                                            self.history, self.output, self.schemas, b.document(self.policy), number)
                environment = self.clock_rows.for_project(row['id'], row['population'] + '-after').bind_process(self.admission['scannerEnvironment'], os.environ)
                reference, marker, terminal = f._launch(project / ('attempt-' + str(number)), bindings, command, environment,
                    self.plan, self.review, self.admission, root, files, protected, self.validation)
                protected.append((reference, marker))
                attempts.append(dict(record=reference, terminal=marker['terminal']))
                terminals.append(terminal)
                if terminal['execution'] != 'TimedOut':
                    break
            success = next((number for number, terminal in enumerate(terminals) if terminal['execution'] == 'Succeeded'), None)
            selected = terminals[success] if success is not None else None
            output_rows.append(dict(repositoryId=row['id'], population=row['population'], split=row['split'], sample=0,
                commit=row['commit'], inputManifestSha256=row['inputManifestSha256'], attempts=attempts, selectedAttempt=success,
                terminal=attempts[success]['terminal'] if selected else None, execution='Succeeded' if selected else terminals[-1]['execution'],
                originalBudgetSatisfied=terminals[0]['execution'] == 'Succeeded', artifacts=selected['artifacts'] if selected else None,
                assessment=selected['assessment'] if selected else None))
        return b.publish(self.output / 'index.json', dict(kind=f.KINDS['index'], schemaVersion=1, plan=self.plan, review=self.review,
            started=started, rows=output_rows, expectedProjects=98, primaryProjects=90, comparisonProjects=8, repeats=1,
            executionComplete=True, artifactCoverageComplete=all(row['selectedAttempt'] is not None for row in output_rows),
            evaluationComplete=False, performance=None, performanceObservations=False, uncompressedPathsAreLogicalArchiveOrigins=True))

    def read_terminal(self, identity, attempt=0):
        return b.document(b.ref(self.output / identity / ('attempt-' + str(attempt)) / 'terminal.json'))


class FreshControlTests(FreshFixture):
    def test_timeout_continues_and_preserves_original_600_failure(self):
        self.prepare()
        self.states[('synthetic-1', 0)] = 'TimedOut'
        self.stop_at('synthetic-2')
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_GLOBAL_STOP_PRESERVED'):
            f.run(self.plan, self.review)
        self.assertEqual(self.calls, [('synthetic-0', 0, 600), ('synthetic-1', 0, 600), ('synthetic-2', 0, 600)])
        terminal = self.read_terminal('synthetic-1')
        self.assertEqual((terminal['execution'], terminal['sharedInvariants']), ('TimedOut', 'Passed'))
        self.assertEqual(terminal['process']['resources']['timeoutSeconds'], 600)
        self.assertFalse((self.output / 'index.json').exists())

    def test_fallback_success_is_selected_once_without_original_budget_claim(self):
        plan = self.prepare(['synthetic-0'])
        self.states[('synthetic-0', 0)] = 'TimedOut'
        self.stop_at('synthetic-1')
        with self.assertRaises(b.Rejected):
            f.run(self.plan, self.review)
        first, second = self.read_terminal('synthetic-0'), self.read_terminal('synthetic-0', 1)
        markers = [b.document(b.ref(self.output / 'synthetic-0' / ('attempt-' + str(number) + '.json'))) for number in (0, 1)]
        row = f._row(self.rows[0], [dict(record={}, terminal=value['terminal']) for value in markers], [first, second])
        self.assertEqual(row['selectedAttempt'], 1)
        self.assertFalse(row['originalBudgetSatisfied'])
        self.assertEqual(len(row['artifacts']), 11)
        self.assertEqual(first['execution'], 'TimedOut')
        self.assertEqual(self.calls[:2], [('synthetic-0', 0, 600), ('synthetic-0', 1, 3600)])
        self.assertEqual((plan['maxLaunches'], plan['scannerBudgetSeconds']), (99, 62400))

    def test_two_timeouts_allow_no_third_and_nonzero_or_schema_failure_never_retry(self):
        self.prepare(['synthetic-0', 'synthetic-1', 'synthetic-2'])
        self.states.update({('synthetic-0', 0): 'TimedOut', ('synthetic-0', 1): 'TimedOut',
                            ('synthetic-1', 0): 'Failed', ('synthetic-2', 0): 'InvalidEvidence'})
        self.stop_at('synthetic-3')
        with self.assertRaises(b.Rejected):
            f.run(self.plan, self.review)
        self.assertEqual(self.calls, [('synthetic-0', 0, 600), ('synthetic-0', 1, 3600),
                                     ('synthetic-1', 0, 600), ('synthetic-2', 0, 600), ('synthetic-3', 0, 600)])
        self.assertEqual(self.read_terminal('synthetic-2')['execution'], 'InvalidEvidence')
        self.assertFalse((self.output / 'synthetic-1/attempt-1').exists())

    def test_started_ref_mutation_stops_before_next_spawn(self):
        self.prepare()
        def tamper(identity, attempt):
            path = self.output / identity / 'attempt-0/started.json'
            path.write_bytes(path.read_bytes() + b' ')
        self.hook = tamper
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_GLOBAL_STOP_PRESERVED'):
            f.run(self.plan, self.review)
        self.assertEqual(len(self.calls), 1)
        self.assertEqual(self.read_terminal('synthetic-0')['sharedInvariants'], 'Failed')

    def test_review_mutation_stops_before_next_spawn(self):
        self.prepare()
        def tamper(identity, attempt):
            path = Path(self.review['path'])
            path.write_bytes(path.read_bytes() + b' ')
        self.hook = tamper
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_GLOBAL_STOP_PRESERVED'):
            f.run(self.plan, self.review)
        self.assertEqual(len(self.calls), 1)

    def test_binary_drift_stops_before_next_spawn(self):
        self.prepare()
        self.hook = lambda identity, attempt: Path(self.admission['adapterBinary']['path']).write_bytes(b'changed public binary')
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_GLOBAL_STOP_PRESERVED'):
            f.run(self.plan, self.review)
        self.assertEqual(len(self.calls), 1)

    def test_protected_previous_attempt_metadata_drift_stops_before_third_spawn(self):
        self.prepare()
        def tamper(identity, attempt):
            if identity == 'synthetic-1':
                (self.output / 'synthetic-0/attempt-0/terminal.json').chmod(0o400)
        self.hook = tamper
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_GLOBAL_STOP_PRESERVED'):
            f.run(self.plan, self.review)
        self.assertEqual([item[0] for item in self.calls], ['synthetic-0', 'synthetic-1'])

    def test_environment_drift_stops_before_next_spawn(self):
        self.prepare()
        def tamper(identity, attempt):
            self.start_patch(patch.object(b, 'validation_environment', return_value={'changed': True}))
        self.hook = tamper
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_GLOBAL_STOP_PRESERVED'):
            f.run(self.plan, self.review)
        self.assertEqual(len(self.calls), 1)

    def test_harness_drift_stops_before_next_spawn(self):
        self.prepare()
        def tamper(identity, attempt):
            self.start_patch(patch.object(b, 'harness_bindings', return_value={'changed': 'b' * 64}))
        self.hook = tamper
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_GLOBAL_STOP_PRESERVED'):
            f.run(self.plan, self.review)
        self.assertEqual(len(self.calls), 1)

    def test_protected_parent_extra_entry_stops_before_third_launch(self):
        self.prepare()
        def tamper(identity, attempt):
            if identity == 'synthetic-1':
                b.publish(self.output / 'synthetic-0/unexpected.json', {'public': 'synthetic'})
            if identity == 'synthetic-2':
                raise b.Rejected('E_TEST_THIRD_LAUNCH_OBSERVED')
        self.hook = tamper
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_GLOBAL_STOP_PRESERVED'):
            f.run(self.plan, self.review)
        self.assertEqual([item[0] for item in self.calls], ['synthetic-0', 'synthetic-1'])

    def test_resume_gap_rejects_before_first_spawn(self):
        self.prepare()
        b.mkdir(self.output)
        b.publish(self.output / 'started.json', dict(kind=f.KINDS['started'], plan=self.plan, review=self.review,
                  selectedIds=[row['id'] for row in self.rows]))
        b.mkdir(self.output / 'synthetic-1')
        def unexpected(identity, attempt):
            raise b.Rejected('E_TEST_FIRST_LAUNCH_OBSERVED')
        self.hook = unexpected
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_RESUME_ORDER'):
            f.run(self.plan, self.review)
        self.assertEqual(self.calls, [])

    def test_input_drift_stops_before_next_spawn(self):
        self.prepare()
        self.hook = lambda identity, attempt: self.manifest.write_bytes(self.manifest.read_bytes() + b' ')
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_GLOBAL_STOP_PRESERVED'):
            f.run(self.plan, self.review)
        self.assertEqual(len(self.calls), 1)

    def test_completed_failure_resume_skips_it_and_stale_started_requires_inspection(self):
        self.prepare()
        self.states[('synthetic-0', 0)] = 'Failed'
        def interrupt(identity, attempt):
            if identity == 'synthetic-1':
                raise KeyboardInterrupt()
        self.hook = interrupt
        with self.assertRaises(KeyboardInterrupt):
            f.run(self.plan, self.review)
        before = b.ref(self.output / 'synthetic-0/attempt-0/terminal.json')
        self.hook = None
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_STALE_STARTED_REQUIRES_INSPECTION'):
            f.run(self.plan, self.review)
        self.assertEqual(self.calls, [('synthetic-0', 0, 600), ('synthetic-1', 0, 600)])
        self.assertEqual(before, b.ref(before['path']))

    def test_resume_can_launch_only_the_unstarted_allowed_fallback(self):
        self.prepare(['synthetic-0'])
        self.states[('synthetic-0', 0)] = 'TimedOut'
        original = f._launch
        def interrupt_after_terminal(*args, **kwargs):
            result = original(*args, **kwargs)
            if args[1]['attempt'] == 0:
                raise KeyboardInterrupt()
            return result
        with patch.object(f, '_launch', side_effect=interrupt_after_terminal), self.assertRaises(KeyboardInterrupt):
            f.run(self.plan, self.review)
        original_terminal = b.ref(self.output / 'synthetic-0/attempt-0/terminal.json')
        self.stop_at('synthetic-1')
        with self.assertRaises(b.Rejected):
            f.run(self.plan, self.review)
        self.assertEqual(self.calls[:3], [('synthetic-0', 0, 600), ('synthetic-0', 1, 3600), ('synthetic-1', 0, 600)])
        self.assertEqual(original_terminal, b.ref(original_terminal['path']))

    def test_plan_rejects_old_epoch_duplicates_and_unapproved_fallbacks(self):
        self.prepare()
        policy = b.document(self.policy)
        for change in (dict(sourceEpoch='old-epoch'), dict(allowedFallbackIds=['foreign']),
                       dict(allowedFallbackIds=['synthetic-0', 'synthetic-0']), dict(threads=True)):
            value = dict(policy, **change)
            reference = b.publish(self.root / ('invalid-' + str(len(os.listdir(self.root))) + '.json'), value)
            with self.subTest(change=change), self.assertRaises(b.Rejected):
                f._policy(reference, self.admission_binding, self.admission, self.rows)
        for identity in ('product-author', 'policy-author', f.AUTHOR, '/root/' + f.AUTHOR, *clocks.AUTHORS):
            review = dict(b.document(self.review), reviewerId=identity)
            reference = b.publish(self.root / ('review-' + str(len(os.listdir(self.root))) + '.json'), review)
            with self.subTest(identity=identity), self.assertRaisesRegex(b.Rejected, 'E_COHORT_REVIEW'):
                f._context(self.plan, reference)
        self.assertEqual(self.calls, [])

    def test_later_registry_root_drift_blocks_the_first_launch(self):
        later_root = b.mkdir(self.root / 'later-source')
        (later_root / 'a.rs').write_text('public synthetic source\n')
        self.rows[-1]['root'] = str(later_root)
        self.prepare()
        (later_root / 'a.rs').write_text('changed later public source\n')
        with self.assertRaises((b.Rejected, ValueError)):
            f.run(self.plan, self.review)
        self.assertEqual(self.calls, [])
        self.assertFalse(self.output.exists())

    def test_direct_api_rejects_storage_inside_product_root(self):
        self.prepare()
        with patch.object(b, 'ROOT', self.root), self.assertRaisesRegex(b.Rejected, 'E_STORAGE_INSIDE_PRODUCT_ROOT'):
            f.run(self.plan, self.review)
        self.assertEqual(self.calls, [])

    def test_actual_worker_and_parent_cannot_approve_when_policy_omits_them(self):
        plan = self.prepare()
        self.assertTrue({'/root/fresh98_runner_implementation', 'fresh98_runner_implementation', '/root', 'root'} <= set(plan['authorIds']))
        for identity in ('/root/fresh98_runner_implementation', 'fresh98_runner_implementation', '/root', 'root'):
            value = dict(b.document(self.review), reviewerId=identity)
            reference = b.publish(self.root / ('actual-review-' + str(len(os.listdir(self.root))) + '.json'), value)
            with self.subTest(identity=identity), self.assertRaisesRegex(b.Rejected, 'E_COHORT_REVIEW'):
                f._context(self.plan, reference)
        self.assertEqual(self.calls, [])

    def test_disk_and_foreign_output_reject_before_spawn(self):
        self.prepare()
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_OUTPUT_BINDING'):
            f.run(self.plan, self.review, self.root / 'run-foreign')
        with patch.object(f.shutil, 'disk_usage', return_value=type('Disk', (), {'free': 0})()), self.assertRaisesRegex(b.Rejected, 'E_FREE_DISK_4GIB'):
            f.run(self.plan, self.review)
        self.assertEqual(self.calls, [])

    def test_symlink_output_owner_rejects_before_launch(self):
        self.output.symlink_to(self.source, target_is_directory=True)
        with self.assertRaises((b.Rejected, OSError)):
            self.prepare()
        self.assertEqual(self.calls, [])

    def test_partial_98_index_preserves_failures_and_complete_reader_rejects(self):
        self.prepare()
        self.states = {(row['id'], 0): 'Failed' for row in self.rows}
        reference = self.fixture_index()
        index, rows = f.load_progress(reference)
        self.assertEqual(len(rows), 98)
        self.assertTrue(index['executionComplete'])
        self.assertFalse(index['artifactCoverageComplete'])
        self.assertFalse(index['evaluationComplete'])
        self.assertTrue(all(row['artifacts'] is None and row['selectedAttempt'] is None for row in rows.values()))
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_PARTIAL_INDEX'):
            f.load_index(reference)
        count = len(self.calls)
        self.assertEqual(f.run(self.plan, self.review), (reference, 1))
        self.assertEqual(len(self.calls), count)

    def test_cli_routes_bound_commands_and_preserves_failure_status(self):
        self.prepare()
        with patch.object(f, 'run', return_value=({'path': '/synthetic/index', 'sha256': 'a' * 64}, 1)) as runner:
            self.assertEqual(main.main(['run-cohort', '--plan', json.dumps(self.plan), '--review', json.dumps(self.review), '--output', str(self.output)]), 1)
            runner.assert_called_once_with(self.plan, self.review, self.output)
        with patch.object(f, 'prepare_plan', return_value=self.plan) as planner:
            self.assertEqual(main.main(['plan-cohort', '--admission', json.dumps(self.admission_binding), '--policy', json.dumps(self.policy),
                                        '--output', str(self.root / 'unused-plan'), '--run-output', str(self.output)]), 0)
            planner.assert_called_once()


class FreshCompleteTests(FreshFixture):
    def test_complete_98_artifact_matrix_loader_resume_and_corruption_rejection(self):
        self.prepare(['synthetic-0'])
        self.states[('synthetic-0', 0)] = 'TimedOut'
        reference = self.fixture_index()
        index, rows = f.load_index(reference)
        self.assertEqual(len(rows), 98)
        self.assertTrue(index['executionComplete'])
        self.assertTrue(index['artifactCoverageComplete'])
        self.assertFalse(rows['synthetic-0']['originalBudgetSatisfied'])
        self.assertEqual(rows['synthetic-0']['selectedAttempt'], 1)
        self.assertTrue(all(set(row['artifacts']) == set(b.FORMATS) for row in rows.values()))
        self.assertTrue(all(row['sample'] == 0 for row in rows.values()))
        self.assertEqual(f.run(self.plan, self.review), (reference, 0))
        self.assertEqual(len(self.calls), 99)
        original_index = Path(reference['path']).read_bytes()
        duplicated = copy.deepcopy(index)
        duplicated['rows'][0] = duplicated['rows'][1]
        Path(reference['path']).write_text(json.dumps(duplicated))
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_INDEX_MATRIX'):
            f.load_index(b.ref(reference['path']))
        Path(reference['path']).write_bytes(original_index)
        target = Path(rows['synthetic-0']['artifacts']['report.json']['compressedPath'])
        target.write_bytes(target.read_bytes()[:-1])
        with self.assertRaises((b.Rejected, OSError)):
            f.load_index(reference)


class BoundedProcessTests(PrivateFixture):
    def test_real_short_process_has_honest_3600_budget_and_no_payload_capture(self):
        logs = b.mkdir(self.root / 'logs')
        environment = dict(os.environ)
        command = [sys.executable, '-c', 'print("public synthetic payload"); raise SystemExit(7)']
        process = f.bounded_spawn(command, b.ROOT, environment, logs, f.resources(3600))
        f.validate_process(process, command, environment, f.resources(3600))
        self.assertEqual((process['execution'], process['exitCode']), ('Failed', 7))
        self.assertEqual(process['resources']['timeoutSeconds'], 3600)
        self.assertIsNone(process['measurement'])
        self.assertEqual(list(logs.iterdir()), [])
        self.assertNotIn('public synthetic payload', json.dumps(process))
        self.assertEqual(e.RESOURCE['timeoutSeconds'], 600)
        with self.assertRaises(b.Rejected):
            f.resources(3601)

    def test_corrupt_process_cannot_be_used_as_timeout_authority(self):
        command = [sys.executable, '-c', 'pass']
        logs = b.mkdir(self.root / 'logs')
        process = f.bounded_spawn(command, b.ROOT, dict(os.environ), logs, f.resources(600))
        process.update(execution='TimedOut', timedOut=True)
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_PROCESS'):
            f.validate_process(process, command, dict(os.environ), f.resources(600))


class SnapshotReadBoundaryTests(PrivateFixture):
    def test_file_read_uses_approved_open_and_rejects_before_body(self):
        path = self.write('public-protected.json', b'{"public":"synthetic"}\n')
        reads = []
        actual_open, actual_read = b.approved._open, os.read
        identity = (path.stat().st_dev, path.stat().st_ino)
        def checked_open(target, directory=False):
            if Path(target) == path and not directory:
                raise b.Rejected('E_SYNTHETIC_PRESEAL')
            return actual_open(target, directory=directory)
        def watched_read(fd, size):
            info = os.fstat(fd)
            if (info.st_dev, info.st_ino) == identity:
                reads.append(size)
            return actual_read(fd, size)
        with patch.object(b.approved, '_open', side_effect=checked_open), patch.object(os, 'read', side_effect=watched_read):
            with self.assertRaisesRegex(b.Rejected, 'E_SYNTHETIC_PRESEAL'):
                f.snapshot(self.root)
        self.assertEqual(reads, [])

    def test_benign_snapshot_preserves_binding_and_identity_metadata(self):
        path = self.write('public.json', b'{"public":"synthetic"}\n')
        info = path.stat()
        actual_open = b.approved._open
        with patch.object(b.approved, '_open', wraps=actual_open) as opened:
            value = f.snapshot(self.root)
        entry = value['./public.json']
        self.assertEqual(entry, dict(kind='file', binding=b.ref(path), device=info.st_dev, inode=info.st_ino,
            mode=info.st_mode, size=info.st_size, uid=info.st_uid, gid=info.st_gid, links=info.st_nlink,
            mtimeNs=info.st_mtime_ns, ctimeNs=info.st_ctime_ns))
        opened.assert_any_call(path)
        self.assertEqual(set(value), {'.', './public.json'})

    def test_link_mode_and_symlink_boundaries_still_reject(self):
        path = self.write('public.json', b'public synthetic\n')
        path.chmod(0o644)
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_PROTECTED_FILE'):
            f.snapshot(self.root)
        path.chmod(0o600)
        link = self.root / 'link.json'
        os.link(path, link)
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_PROTECTED_FILE'):
            f.snapshot(self.root)
        link.unlink()
        link.symlink_to(path)
        with self.assertRaisesRegex(b.Rejected, 'E_COHORT_PROTECTED_FILE'):
            f.snapshot(self.root)


class MaterializationReadBoundaryTests(FreshFixture):
    def test_expected_artifact_bindings_checked_before_assessment_reads(self):
        self.prepare()
        row = self.rows[0]
        policy = b.document(self.policy)
        root, files, bindings, command = f._project(self.admission, self.admission_binding, row, self.clock_rows,
            self.history, self.output, self.schemas, policy, 0)
        b.mkdir(self.output)
        b.mkdir(self.output / row['id'])
        environment = self.clock_rows.for_project(row['id'], row['population'] + '-after').bind_process(self.admission['scannerEnvironment'], os.environ)
        _, _, terminal = f._launch(self.output / row['id'] / 'attempt-0', bindings, command, environment,
            self.plan, self.review, self.admission, root, files, [], self.validation)
        actual_verify = e.verify_output_clocks
        with patch.object(b.approved, 'checked_file', wraps=b.approved.checked_file) as checked:
            def verified(workspace, clock):
                for name, item in terminal['artifacts'].items():
                    checked.assert_any_call(workspace / name, {key: item[key] for key in ('bytes', 'sha256')}, limit=b.MAX)
                return actual_verify(workspace, clock)
            with patch.object(e, 'verify_output_clocks', side_effect=verified):
                e.unpack_assess(terminal, self.root / 'verify-public', self.validation)
        self.assertFalse((self.root / 'verify-public').exists())


if __name__ == '__main__':
    unittest.main()
