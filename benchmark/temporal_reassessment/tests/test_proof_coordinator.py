import copy
import io
import json
from pathlib import Path
from unittest.mock import patch
import unittest
from types import SimpleNamespace

import adapter
import boundary as b
import clocks
import execution
import proof_coordinator as producer
from test_execution import PrivateFixture
from test_temporal import TemporalRecords


class CoordinatorGateTests(PrivateFixture):
    def setUp(self):
        super().setUp()
        self.records = r = TemporalRecords(self)
        self.cohort = SimpleNamespace(interval_jobs=lambda registry: [],
            required_keys=frozenset(('synthetic-' + str(i), 'primary-after') for i in range(98)))
        self.cohort.publish_interval_index = lambda rows, path: clocks.OriginalClockCohort.publish_interval_index(self.cohort, rows, path)
        self.tool = self.write('temporal_witness.rs', b'synthetic tool')
        self.coordinator = self.write('producer.py', b'synthetic coordinator identity')
        self.actual = r.record(dict(kind='synthetic compiled receipt'))
        wrapper = r.record(dict(kind='temporal-witness-build-receipt-v1', exitCode=0,
            binary=r.binary, toolSource=b.ref(self.tool), sourceBindings=r.source, rules=r.rules,
            linkedSourceEvidence=self.actual, boundaryTests=r.evidence))
        self.subjects = dict(policy=r.record(dict(kind='synthetic policy')),
            binaryAliases=r.record(dict(aliases=[])), recoveryRegistry=r.record(dict(repositories=[
                dict(id='synthetic-' + str(i)) for i in range(98)])), sourceRelation=r.relation,
            witnessBuildReceipt=wrapper, witnessBinary=r.binary, witnessToolSource=b.ref(self.tool),
            witnessRules=r.rules, overlayBuildReceipt=r.overlay, coordinator=b.ref(self.coordinator))
        self.gate = dict(kind='temporal-witness-run-gate-v1', verdict='PASS',
            reviewerModel='codex:gpt-6.1-sol@high', reviewerId='synthetic-independent-reviewer',
            authorIds=sorted(clocks.AUTHORS), scopes=sorted(clocks.REVIEW_SCOPES),
            sourceBindings=r.source, harnessBindings=r.harness, subjects=self.subjects)

    def admit(self, gate=None):
        r = self.records
        with patch.object(producer, '__file__', str(self.coordinator)), patch.object(b, 'CODE_DIR', self.root), \
             patch.object(b, 'verify_source') as verify_source, \
             patch.object(b, 'harness_bindings', return_value=r.harness), \
             patch.object(adapter, 'verify_build') as overlay, \
             patch.object(adapter, 'verify_witness_build', return_value=dict(binary=r.binary)) as actual, \
             patch.object(clocks.OriginalClockCohort, 'admit', return_value=self.cohort) as historical:
            result = producer.admit(r.record(self.gate if gate is None else gate), adapter, b, clocks)
            verify_source.assert_called_once_with(r.source)
            overlay.assert_called_once()
            actual.assert_called_once()
            historical.assert_called_once_with(self.subjects['policy'], [], r.relation_value)
            return result

    def test_explicit_independent_gate_authenticates_all_ten_subjects(self):
        gate, relation, history, rows = self.admit()
        self.assertEqual(gate, self.gate)
        self.assertEqual(relation, self.records.relation_value)
        self.assertEqual(len(rows), 98)

    def test_wrong_reviewer_source_harness_or_subject_cannot_reach_history_or_launch(self):
        changes = [dict(reviewerId=next(iter(clocks.AUTHORS))), dict(reviewerModel='codex:gpt-6.1-terra@high'),
                   dict(scopes=[]), dict(verdict='PASS_SCOPED'), dict(harnessBindings={}),
                   dict(subjects=dict(self.subjects, coordinator=self.records.evidence)),
                   dict(subjects={k: v for k, v in self.subjects.items() if k != 'recoveryRegistry'})]
        for change in changes:
            gate = dict(self.gate, **change)
            with self.subTest(change=change), patch.object(clocks.OriginalClockCohort, 'admit') as historical, \
                 patch.object(execution, 'spawn') as spawn, self.assertRaises(b.Rejected):
                self.admit(gate)
            historical.assert_not_called()
            spawn.assert_not_called()
        gate = copy.deepcopy(self.gate)
        gate['subjects']['witnessBinary']['sha256'] = '0' * 64
        with patch.object(clocks.OriginalClockCohort, 'admit') as historical, patch.object(execution, 'spawn') as spawn, \
             self.assertRaises(b.Rejected):
            self.admit(gate)
        historical.assert_not_called()
        spawn.assert_not_called()

    def test_all_history_preflight_finishes_before_any_job_or_process(self):
        events = []
        r = self.records
        with patch.object(producer, '__file__', str(self.coordinator)), patch.object(b, 'CODE_DIR', self.root), \
             patch.object(b, 'verify_source'), patch.object(b, 'harness_bindings', return_value=r.harness), \
             patch.object(adapter, 'verify_build'), \
             patch.object(adapter, 'verify_witness_build', return_value=dict(binary=r.binary)), \
             patch.object(clocks.OriginalClockCohort, 'admit', side_effect=lambda *args: events.append('all-four-preflight') or self.cohort), \
             patch.object(self.cohort, 'interval_jobs', side_effect=lambda *args: events.append('output-body-jobs') or []), \
             patch.object(execution, 'spawn') as spawn, self.assertRaisesRegex(b.Rejected, 'E_PRODUCER_INDEX'):
            producer.run(r.record(self.gate), self.root / 'run', adapter, b, clocks, execution)
        self.assertEqual(events, ['all-four-preflight', 'output-body-jobs'])
        spawn.assert_not_called()
        self.assertFalse((self.root / 'run/coverage-index.json').exists())

    def test_one_synthetic_process_produces_exact_receipt_but_cannot_publish_98_index(self):
        r = self.records
        input_root = b.mkdir(self.root / 'synthetic-input')
        r.original['input']['root'] = str(input_root)
        r.row = r.coverage_row('synthetic-0', 'primary-after')
        gate_ref = r.record(self.gate)
        job = ('synthetic-0', 'primary-after', dict(id='synthetic-0'), r.entry, r.original, r.started, r.generated)
        oss = SimpleNamespace(_acquisition=lambda *args: (input_root, [], r.original['input']),
                              _validate_input=lambda *args: None)
        external = SimpleNamespace(selection_bindings=lambda *args, **kwargs: r.original['selection'])

        def configuration(destination, **kwargs):
            value = b.document(r.row['config'])
            value.update(root=str(kwargs['root']), bindings=kwargs['bindings'], output=str(destination / 'witness.json'))
            return dict(configuration=b.publish(destination / 'witness-config.json', value))

        def synthetic_process(command, cwd, environment, logs, *, timeout):
            self.assertEqual(timeout, 600)
            config = b.document(b.ref(command[1]))
            witness = copy.deepcopy(r.witness_value)
            witness.update(bindings=config['bindings'], configurationSha256=b.ref(command[1])['sha256'])
            b.publish(Path(config['output']), witness)
            return dict(execution='Succeeded', exitCode=0, timedOut=False,
                commandDigest=b.digest(command), cwdDigest=b.digest(str(cwd)),
                startedUtc='2026-11-01T00:00:00+00:00', endedUtc='2026-11-01T00:00:04+00:00')

        destination = self.root / 'run'
        with patch.object(producer, 'admit', return_value=(self.gate, r.relation_value, self.cohort, [])), \
             patch.object(self.cohort, 'interval_jobs', return_value=[job]), \
             patch.object(b, 'import_auditors', return_value=(oss, external, None)), \
             patch.object(b, 'verify_source'), patch.object(b, 'source_bindings', return_value=r.source), \
             patch.object(b, 'harness_bindings', return_value=r.harness), \
             patch.object(adapter, 'witness_configuration', side_effect=configuration), \
             patch.object(execution, 'spawn', side_effect=synthetic_process) as spawn, \
             patch.object(producer.shutil, 'disk_usage', return_value=SimpleNamespace(free=4 * 1024**3)), \
             self.assertRaisesRegex(b.Rejected, 'E_PRODUCER_INDEX'):
            producer.run(gate_ref, destination, adapter, b, clocks, execution)
        spawn.assert_called_once()
        terminal = b.document(b.ref(destination / 'synthetic-0/terminal.json'))
        self.assertEqual(terminal['execution'], 'Succeeded')
        receipt = b.document(terminal['row']['producerReceipt'])
        self.assertIsNone(receipt['performance'])
        self.assertEqual(receipt['selectionBefore'], r.original['selection'])
        self.assertEqual(receipt['inputBefore'], r.original['input']['manifest'])
        self.assertFalse((destination / 'coverage-index.json').exists())

    def test_import_or_path_failure_emits_only_a_safe_error_code(self):
        error_output = io.StringIO()
        with patch.object(producer.sys, 'argv', ['producer', '--harness', str(self.root),
            '--gate', '{}', '--output', str(self.root / 'unused')]), \
             patch.object(producer, 'load_harness', side_effect=ImportError('synthetic private payload')), \
             patch.object(producer.sys, 'stderr', error_output):
            self.assertEqual(producer.main(), 2)
        self.assertEqual(json.loads(error_output.getvalue()),
                         dict(status='rejected', errorCode='E_PRODUCER_REJECTED', payloadOmitted=True))

    def test_incomplete_existing_project_is_not_overwritten_or_reexecuted(self):
        r = self.records
        gate_ref = r.record(self.gate)
        destination = b.mkdir(self.root / 'run')
        b.publish(destination / 'run-gate.json', dict(gate=gate_ref))
        project = b.mkdir(destination / 'synthetic-0')
        started = b.publish(project / 'started.json', dict(synthetic='interrupted'))
        job = ('synthetic-0', 'primary-after', {}, r.entry, r.original, r.started, r.generated)
        with patch.object(producer, 'admit', return_value=(self.gate, r.relation_value, self.cohort, [])), \
             patch.object(self.cohort, 'interval_jobs', return_value=[job]), patch.object(execution, 'spawn') as spawn, \
             self.assertRaises(b.Rejected):
            producer.run(gate_ref, destination, adapter, b, clocks, execution)
        spawn.assert_not_called()
        self.assertEqual(b.ref(project / 'started.json'), started)
        self.assertFalse((project / 'terminal.json').exists())


if __name__ == '__main__':
    unittest.main()
