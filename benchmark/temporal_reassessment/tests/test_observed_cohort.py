import copy
import json
from dataclasses import FrozenInstanceError
from unittest.mock import patch

import boundary as b
import clocks
import execution
import proof_coordinator as producer
from test_execution import PrivateFixture
from test_temporal import TemporalRecords
import test_temporal


class ObservedCohortTests(PrivateFixture):
    historical_fixture = test_temporal.CompleteClockPlanTests.historical_fixture

    def fixture(self, observed, invalid=None):
        records = TemporalRecords(self)

        def mutate(identity, phase, values):
            if phase.endswith('-after') and observed(identity):
                values['report.json']['findings'] = [dict(rule_id='CERT_EXPIRED', evidence=dict(
                    type='certificate', metadata=dict(detector='certificate_detector',
                        validity_status='expired', assessment_time_unix='1793491201')))]
            if invalid is not None and identity == 'primary-0' and phase == 'primary-after':
                invalid(values)

        policy, entries, originals, source_sha = self.historical_fixture(records, mutate)
        records.harness = b.harness_bindings()
        records.temporal_value['harnessBindings'] = records.harness
        rows = []
        for identity, entry in entries.items():
            phase = entry['population'] + '-after'
            records.original = originals[(identity, phase)]
            records.entry = entry
            row = records.coverage_row(identity, phase)
            witness = copy.deepcopy(records.witness_value)
            witness['objects'][0]['sourceSha256'] = source_sha
            witness['files'][0]['sourceSha256'] = source_sha
            row['witness'] = records.record(witness)
            coverage = copy.deepcopy(records.coverage_value)
            coverage.update(witness=row['witness'], files=witness['files'], selectedFiles={'a.pem': source_sha})
            row['coverage'] = records.record(coverage)
            row['producerReceipt'] = records.producer_receipt(row)
            rows.append(row)
        return records, policy, list(entries.values()), rows

    def index(self, records, rows, kind='temporal-required-raw-index-v2'):
        records.temporal_value['coverageIndex'] = records.record(dict(kind=kind, rows=rows))
        records.refresh_review()
        return records.admit()

    def derive(self, records, policy, rows, kind='temporal-required-raw-index-v2'):
        temporal, relation, admitted = self.index(records, rows, kind)
        return clocks.derive_all(policy, [], temporal, relation, admitted)

    def test_mixed_real196_98_classification_exact_subset_and_immutable_replay(self):
        observed = lambda identity: int(identity.rsplit('-', 1)[1]) % 3 == 0
        r, policy, registry, all_rows = self.fixture(observed)
        required = [row for row in all_rows if not observed(row['repositoryId'])]
        with patch.object(clocks, 'POLICY_SHA256', policy['sha256']):
            cohort = clocks.OriginalClockCohort.admit(policy, [], r.relation_value)
            expected_keys = {(row['repositoryId'], row['phaseId']) for row in required}
            self.assertEqual(cohort.required_keys, expected_keys)
            jobs = cohort.interval_jobs(registry)
            self.assertEqual({(job[0], job[1]) for job in jobs}, expected_keys)
            for job in jobs:
                original = job[4]
                self.assertEqual(original['originalPhaseId'], job[1])
                self.assertEqual(original['originalDiagnostics'], b.ref(original['originalDiagnostics']['path']))
                projection = clocks.adapter.original_diagnostic_projection(original)
                self.assertEqual(projection['originalTerminal'], original['originalTerminal'])
                self.assertEqual(projection['diagnostics'], original['originalDiagnostics'])
            with self.assertRaises(TypeError):
                cohort._after[('primary-0', 'primary-after')] = None
            with self.assertRaises(FrozenInstanceError):
                cohort.required_keys = frozenset()
            derived = self.derive(r, policy, required)
            self.assertEqual(len(derived['rows']), 196)
            self.assertEqual(sum('clock' in row for row in derived['rows']), 98)
            self.assertEqual(sum(row.get('clock', {}).get('kind') == 'observed_original_second'
                                 for row in derived['rows']), 98 - len(required))
            plan = clocks.ClockPlan.admit(r.record(derived), r.temporal, [], r.source, ['synthetic-author'], r.overlay)
            self.assertEqual(plan.for_project('primary-0', 'primary-after').detector_second, 1793491201)
            self.assertEqual(plan.for_project('primary-1', 'primary-after').detector_second, 1793491200)
            snapshot = plan[('primary-0', 'primary-after')]
            snapshot['clock']['assessmentTimeUnix'] = 0
            self.assertEqual(plan.for_project('primary-0', 'primary-after').detector_second, 1793491201)
            with self.assertRaises(b.Rejected):
                plan.for_project('primary-0', 'primary-baseline')
            registry[-1]['commit'] = 'changed'
            with patch.object(execution, 'spawn') as spawn, self.assertRaisesRegex(b.Rejected, 'E_PRODUCER_RECOVERY_PIN'):
                cohort.interval_jobs(registry)
            spawn.assert_not_called()

    def test_missing_extra_count_preserving_swap_duplicate_and_stored_scalar_reject(self):
        observed = lambda identity: identity == 'primary-0'
        r, policy, registry, rows = self.fixture(observed)
        required = [row for row in rows if not observed(row['repositoryId'])]
        extra = next(row for row in rows if observed(row['repositoryId']))
        with patch.object(clocks, 'POLICY_SHA256', policy['sha256']):
            for invalid in [required[1:], required + [extra], required[1:] + [extra], required + [required[0]]]:
                with self.subTest(size=len(invalid)), self.assertRaises(b.Rejected):
                    self.derive(r, policy, invalid)
            derived = self.derive(r, policy, required)
            observed_row = next(row for row in derived['rows'] if row['repositoryId'] == 'primary-0' and
                                row['phaseId'] == 'primary-after')
            observed_row['clock']['assessmentTimeUnix'] = 1793491202
            with self.assertRaisesRegex(b.Rejected, 'E_CLOCK_MAP_DERIVATION'):
                clocks.ClockPlan.admit(r.record(derived), r.temporal, [], r.source, ['synthetic-author'], r.overlay)

    def test_all_observed_empty_v2_preserves_failed_helper_and_never_launches(self):
        r, policy, registry, rows = self.fixture(lambda identity: True)
        with patch.object(clocks, 'POLICY_SHA256', policy['sha256']):
            cohort = clocks.OriginalClockCohort.admit(policy, [], r.relation_value)
            self.assertEqual(cohort.required_keys, frozenset())
            corrupted = copy.deepcopy(rows)
            corrupted[0]['coverage']['sha256'] = '0' * 64
            with self.assertRaises(b.Rejected):
                self.derive(r, policy, corrupted, 'temporal-coverage-index-v1')
            derived = self.derive(r, policy, [])
            self.assertEqual(sum(row.get('clock', {}).get('kind') == 'observed_original_second'
                                 for row in derived['rows']), 98)
            clocks.ClockPlan.admit(r.record(derived), r.temporal, [], r.source, ['synthetic-author'], r.overlay)
            gate = dict(sourceBindings=r.source, subjects=dict(sourceRelation=r.relation, witnessBinary=r.binary,
                witnessToolSource=r.tool_source, witnessRules=r.rules))
            gate_ref = r.record(gate)
            destination = b.mkdir(self.root / 'raw-run')
            b.publish(destination / 'run-gate.json', dict(gate=gate_ref))
            failed = b.mkdir(destination / 'primary-0')
            terminal = b.publish(failed / 'terminal.json', dict(execution='Failed', performance=None))
            with patch.object(producer, 'admit', return_value=(gate, r.relation_value, cohort, registry)), \
                 patch.object(execution, 'spawn') as spawn:
                result = producer.run(gate_ref, destination, None, b, clocks, execution)
            spawn.assert_not_called()
            self.assertEqual(b.document(result), dict(kind='temporal-required-raw-index-v2', rows=[]))
            self.assertEqual(b.ref(failed / 'terminal.json'), terminal)
            self.assertEqual(b.document(terminal)['execution'], 'Failed')
            changed = copy.deepcopy(registry)
            changed[-1]['commit'] = 'tampered-observed-pin'
            with patch.object(producer, 'admit', return_value=(gate, r.relation_value, cohort, changed)), \
                 patch.object(execution, 'spawn') as spawn, self.assertRaisesRegex(b.Rejected, 'E_PRODUCER_RECOVERY_PIN'):
                producer.run(gate_ref, self.root / 'never-created', None, b, clocks, execution)
            spawn.assert_not_called()
            self.assertFalse((self.root / 'never-created').exists())

    def test_all_missing_requires98_and_legacy_v1_keeps_exact_cardinality_and_raw_admission(self):
        r, policy, registry, rows = self.fixture(lambda identity: False)
        with patch.object(clocks, 'POLICY_SHA256', policy['sha256']):
            cohort = clocks.OriginalClockCohort.admit(policy, [], r.relation_value)
            self.assertEqual(len(cohort.required_keys), 98)
            self.assertEqual(len(cohort.interval_jobs(registry)), 98)
            self.assertEqual(self.derive(r, policy, rows), self.derive(r, policy, rows, 'temporal-coverage-index-v1'))
            for invalid in [[], rows[:-1], rows + [rows[0]]]:
                with self.subTest(size=len(invalid)), self.assertRaisesRegex(b.Rejected, 'E_TEMPORAL_COVERAGE_MATRIX'):
                    self.index(r, invalid, 'temporal-coverage-index-v1')
            with self.assertRaises(b.Rejected):
                self.index(r, rows, 'temporal-required-raw-index-v1')
            with self.assertRaises(b.Rejected):
                self.derive(r, policy, rows[:-1])
            corrupted = copy.deepcopy(rows)
            corrupted[0]['coverage']['sha256'] = '0' * 64
            with self.assertRaises(b.Rejected):
                self.derive(r, policy, corrupted, 'temporal-coverage-index-v1')

    def test_authenticated_null_original_rejects_with_complete_raw_available(self):
        def invalid(values):
            values['report.json']['findings'][0]['evidence']['metadata']['assessment_time_unix'] = None

        r, policy, registry, rows = self.fixture(lambda identity: True, invalid)
        with patch.object(clocks, 'POLICY_SHA256', policy['sha256']), patch.object(execution, 'spawn') as spawn:
            with self.assertRaisesRegex(b.Rejected, 'E_CLOCK_VALUE'):
                self.derive(r, policy, rows, 'temporal-coverage-index-v1')
        spawn.assert_not_called()

    def test_fourth_real_preflight_tamper_precedes_any_original_body_or_spawn(self):
        r, policy, registry, rows = self.fixture(lambda identity: True)
        value = b.document(policy)
        fourth = value['invocations'][3]['binding']
        reference = b.document(fourth)['rows'][0]['receipt']
        path = self.root / 'comparison-after/comparison-0/output/report.json'
        self.assertTrue(path.exists())
        path.write_bytes(path.read_bytes() + b' ')
        oss, _, _ = b.import_auditors()
        with patch.object(clocks, 'POLICY_SHA256', policy['sha256']), \
             patch.object(oss, '_verified_invocation') as bodies, patch.object(execution, 'spawn') as spawn, \
             self.assertRaises(b.Rejected):
            clocks.OriginalClockCohort.admit(policy, [], r.relation_value)
        bodies.assert_not_called()
        spawn.assert_not_called()


class MalformedObservedTests(PrivateFixture):
    def test_malformed_or_invalid_observation_never_uses_available_raw_proof(self):
        started = '2026-11-01T00:00:00+00:00'
        report = dict(generated_at='2026-11-01T00:00:04Z', findings=[])
        native = dict(run=dict(generated_at='2026-11-01T00:00:04+00:00'))
        proof = clocks._CoverageProof(dict(path='/synthetic/proof', sha256='a' * 64), (), ())
        finding = dict(rule_id='CERT_EXPIRED', evidence=dict(type='certificate', metadata=dict(
            detector='certificate_detector', validity_status='expired', assessment_time_unix='1793491201')))
        vectors = []
        for value in [None, True, False, 1793491201, '', '01793491201', ' 1793491201', '1e9', '1793491210']:
            changed = copy.deepcopy(finding)
            changed['evidence']['metadata']['assessment_time_unix'] = value
            vectors.append([changed])
        changed = copy.deepcopy(finding)
        changed['evidence']['type'] = 'source_code'
        vectors.append([changed])
        changed = copy.deepcopy(finding)
        changed['evidence']['metadata']['detector'] = 'other_detector'
        vectors.append([changed])
        changed = copy.deepcopy(finding)
        changed['evidence']['metadata']['assessment_time_unix'] = '1793491202'
        vectors.append([finding, changed])
        changed = copy.deepcopy(finding)
        del changed['evidence']['metadata']['assessment_time_unix']
        vectors.append([changed])
        changed = dict(rule_id='TIME_INDEPENDENT', evidence=dict(type='certificate', metadata=dict(
            detector='certificate_detector', assessment_time_unix=None)))
        vectors.append([changed])
        for metadata in [None, [], True, 'malformed']:
            changed = copy.deepcopy(finding)
            changed['evidence']['metadata'] = metadata
            vectors.append([changed])
        for findings in vectors:
            with self.subTest(vector=vectors.index(findings)), self.assertRaises(b.Rejected):
                clocks.derive_clock(dict(report, findings=findings), native, started, proof)
        for wrong_report, wrong_native, wrong_started in [
            (dict(report, generated_at='2026-11-01T00:00:04.1Z'), native, started),
            (report, dict(run=dict(generated_at='2026-11-01T00:00:05+00:00')), started),
            (report, native, '2026-11-01T00:00:05+00:00'),
            (dict(report, generated_at='malformed'), native, started)]:
            with self.assertRaises(b.Rejected):
                clocks.derive_clock(dict(wrong_report, findings=[finding]), wrong_native, wrong_started, proof)
