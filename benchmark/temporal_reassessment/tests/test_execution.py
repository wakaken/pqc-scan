import ast
import copy
import gzip
import hashlib
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import adapter
import boundary as b
import clocks
import delta
import execution
import main
import storage


class PrivateFixture(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(dir=b.OWN)
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.root.chmod(0o700)

    def write(self, name, raw):
        path = self.root / name
        path.write_bytes(raw)
        path.chmod(0o600)
        return path

    def zipped(self, raw=b'public synthetic\n'):
        source = self.write('plain', raw)
        target = self.root / 'plain.gz'
        return source, target, storage.compress(source, target, b.ref(source))


class ClockTests(unittest.TestCase):
    def setUp(self):
        self.started = '2026-11-01T00:00:00+00:00'
        self.generated = '2026-11-01T00:00:04Z'
        self.instant = 1793491201
        self.report = dict(generated_at=self.generated, findings=[])
        self.native = dict(assets=[], run=dict(generated_at='2026-11-01T00:00:04+00:00'))

    def finding(self, clock):
        return dict(rule_id='CERT_EXPIRED', evidence=dict(type='certificate', metadata=dict(
                    assessment_time_unix=clock, validity_status='expired', detector='certificate_detector')))

    def test_original_one_clock_not_current_time(self):
        self.report['findings'] = [self.finding(str(self.instant))] * 3
        result = clocks.derive_clock(self.report, self.native, self.started)
        self.assertEqual(result['assessmentTimeUnix'], 1793491201)
        self.assertEqual(result['kind'], 'observed_original_second')

    def test_multiple_clocks_rejected(self):
        self.report['findings'] = [self.finding(str(self.instant)), self.finding(str(self.instant + 1))]
        with self.assertRaises(b.Rejected):
            clocks.derive_clock(self.report, self.native, self.started)

    def test_invalid_bool_integer_whitespace_and_outside_window(self):
        for value in (True, 1793491201, ' 1793491201', '01793491201', '-1', '1793491210', '0'):
            with self.subTest(value=value), self.assertRaises(b.Rejected):
                self.report['findings'] = [self.finding(value)]
                clocks.derive_clock(self.report, self.native, self.started)

    def test_missing_validity_metadata_rejected(self):
        finding = self.finding(str(self.instant)); del finding['evidence']['metadata']['assessment_time_unix']
        self.report['findings'] = [finding]
        with self.assertRaises(b.Rejected):
            clocks.derive_clock(self.report, self.native, self.started)

    def test_empty_native_assets_cannot_establish_clock_equivalence(self):
        with self.assertRaises(b.Rejected):
            clocks.derive_clock(self.report, self.native, self.started)

    def test_missing_certificate_findings_require_complete_producer_proof(self):
        self.native['assets'] = [dict(kind='certificate')]
        with self.assertRaises(b.Rejected):
            clocks.derive_clock(self.report, self.native, self.started)

    def test_standalone_key_facts_do_not_establish_producer_coverage(self):
        self.native['assets'] = [dict(kind='public_key')]
        with self.assertRaises(b.Rejected):
            clocks.derive_clock(self.report, self.native, self.started)

    def test_generated_time_before_started_or_after_timeout_rejected(self):
        for generated in ('2026-10-31T23:59:59Z', '2026-11-01T00:11:00Z', '2026-11-01T00:00:04', '2026-11-01T01:00:04+01:00'):
            self.report['generated_at'] = generated
            with self.subTest(generated=generated), self.assertRaises(b.Rejected):
                clocks.derive_clock(self.report, self.native, self.started)

    def test_all_four_preflights_precede_any_legacy_body_parse(self):
        policy = dict(kind='historical-score-policy', invocations=[dict(id=identity, population=pop, phase=phase,
                      binding=dict(path='/synthetic/' + identity, sha256='a' * 64))
                      for identity, (pop, phase, _) in clocks.PHASES.items()])
        seen = []
        def preflight(binding, **kwargs):
            identity = Path(binding['path']).name
            seen.append(identity)
            if len(seen) == 4:
                raise b.Rejected('synthetic fourth phase corruption')
            population, phase, count = clocks.PHASES[identity]
            ids = [population + str(index) for index in range(count)]
            return dict(invocation=dict(phase=phase, expectedSamples=1, selectedIds=ids,
                        selectedEntries={identity: dict(population=population) for identity in ids}))
        with patch.object(b, 'document', return_value=policy), patch.object(b.approved, 'preflight_invocation', preflight), \
                patch.object(b, 'import_auditors') as body_consumer, self.assertRaises(b.Rejected):
            clocks.derive_all(dict(path='/synthetic/policy', sha256=clocks.POLICY_SHA256), [])
        self.assertEqual(seen, list(clocks.PHASES))
        body_consumer.assert_not_called()


class CompressionTests(PrivateFixture):
    def test_deterministic_gzip_and_plain_binding(self):
        source, target, item = self.zipped(b'public synthetic\n' * 200)
        other = self.root / 'second.gz'
        second = storage.compress(source, other, b.ref(source))
        self.assertEqual(target.read_bytes(), other.read_bytes())
        self.assertEqual(target.read_bytes()[3:8], b'\0' * 5)
        self.assertEqual(storage.compressed_read(item), source.read_bytes())
        self.assertEqual(item['sha256'], hashlib.sha256(source.read_bytes()).hexdigest())
        self.assertEqual(second['compressedSha256'], item['compressedSha256'])

    def test_gzip_corruption_truncation_and_appended_member_rejected(self):
        source, target, item = self.zipped()
        raw = target.read_bytes()
        for altered in (raw[:-3], raw[:12] + b'broken' + raw[18:], raw + gzip.compress(b'extra', mtime=0), raw + b'trailing'):
            target.write_bytes(altered)
            item.update(compressedBytes=len(altered), compressedSha256=hashlib.sha256(altered).hexdigest())
            with self.subTest(size=len(altered)), self.assertRaises(b.Rejected):
                storage.compressed_read(item)

    def test_bounded_expansion_and_wrong_plain_digest_rejected(self):
        _, _, item = self.zipped(b'X' * 10000)
        bad = dict(item, bytes=100)
        with self.assertRaises(b.Rejected):
            storage.compressed_read(bad)
        with self.assertRaises(b.Rejected):
            storage.compressed_read(dict(item, sha256='0' * 64))
        with self.assertRaises(b.Rejected):
            storage.compressed_read(item, limit=1000)

    def test_changed_compressed_bytes_rejected_without_parsing(self):
        _, target, item = self.zipped()
        target.write_bytes(gzip.compress(b'new payload', mtime=0))
        with self.assertRaises(b.Rejected):
            storage.compressed_read(item)

    def test_symlink_ancestor_and_final_path_rejected(self):
        source, target, item = self.zipped()
        alias = self.root / 'alias'; alias.symlink_to(self.root, target_is_directory=True)
        with self.assertRaises(b.Rejected):
            storage.compressed_read(dict(item, compressedPath=str(alias / target.name)))
        target.unlink(); target.symlink_to(source)
        with self.assertRaises(b.Rejected):
            storage.compressed_read(item)

    def test_no_existing_file_is_overwritten(self):
        source, target, item = self.zipped()
        previous = target.read_bytes()
        with self.assertRaises(FileExistsError):
            storage.compress(source, target, b.ref(source))
        self.assertEqual(target.read_bytes(), previous)

    def make_matrix(self):
        output = b.mkdir(self.root / 'output'); compressed = b.mkdir(self.root / 'compressed')
        for name in b.FORMATS:
            (output / name).write_bytes((name + '\n').encode())
        expected = {name: {key: value for key, value in b.ref(output / name).items() if key != 'path'} for name in b.FORMATS}
        return output, compressed, expected

    def test_all_eleven_formats_verified_before_own_spool_cleanup(self):
        output, compressed, expected = self.make_matrix()
        items = storage.archive_outputs(output, compressed, expected)
        self.assertFalse(output.exists())
        storage.validate_matrix(items)
        self.assertEqual(len(items), 11)
        self.assertEqual(set(compressed.iterdir()), {compressed / (name + '.gz') for name in b.FORMATS})

    def test_missing_format_and_extra_output_preserve_uncompressed(self):
        output, compressed, expected = self.make_matrix()
        (output / 'unexpected').write_text('private')
        with self.assertRaises(b.Rejected):
            storage.archive_outputs(output, compressed, expected)
        self.assertTrue((output / 'report.json').exists())
        (output / 'unexpected').unlink(); (output / 'report.rules.html').unlink()
        with self.assertRaises(b.Rejected):
            storage.archive_outputs(output, compressed, expected)
        self.assertTrue((output / 'report.json').exists())

    def test_resume_matrix_rejects_missing_compressed_file(self):
        output, compressed, expected = self.make_matrix()
        items = storage.archive_outputs(output, compressed, expected)
        (compressed / 'report.md.gz').unlink()
        with self.assertRaises((b.Rejected, OSError)):
            storage.validate_matrix(items)


class ExecutionTests(PrivateFixture):
    def test_execution_has_timeout_and_no_performance_observations_or_payload_logs(self):
        logs = b.mkdir(self.root / 'logs')
        result = execution.spawn([sys.executable, '-c', 'print("synthetic private payload")'], self.root, os.environ.copy(), logs)
        self.assertEqual(result['execution'], 'Succeeded')
        self.assertIsNone(result['measurement'])
        self.assertIsNone(result['resources']['wallSeconds'])
        self.assertIsNone(result['resources']['cpuSeconds'])
        self.assertIsNone(result['resources']['peakRssBytes'])
        self.assertFalse(result['resources']['timingComparisonEligible'])
        self.assertEqual(result['stdout']['state'], 'NotCaptured')
        self.assertEqual(result['stderr']['state'], 'NotCaptured')
        self.assertEqual(list(logs.iterdir()), [])
        self.assertNotIn('synthetic private payload', json.dumps(result))

    def test_timeout_and_nonzero_remain_distinct(self):
        logs = b.mkdir(self.root / 'logs')
        timeout = execution.spawn([sys.executable, '-c', 'import time; time.sleep(2)'], self.root, os.environ.copy(), logs, timeout=0.02)
        failed = execution.spawn([sys.executable, '-c', 'raise SystemExit(7)'], self.root, os.environ.copy(), logs)
        self.assertEqual((timeout['execution'], timeout['exitCode']), ('TimedOut', None))
        self.assertEqual((failed['execution'], failed['exitCode']), ('Failed', 7))

    def test_launch_error_has_fixed_code_and_no_raw_exception_message(self):
        logs = b.mkdir(self.root / 'logs')
        result = execution.spawn([str(self.root / 'synthetic-private-unavailable-binary')], self.root, os.environ.copy(), logs)
        self.assertEqual(result['execution'], 'NotRun')
        self.assertEqual(result['errorCode'], 'E_SCANNER_LAUNCH')
        self.assertEqual(result['errorKind'], 'FileNotFoundError')
        self.assertNotIn('synthetic-private-unavailable-binary', json.dumps(result))

    def test_subprocess_shell_and_measurement_apis_are_absent(self):
        tree = ast.parse((Path(execution.__file__)).read_text())
        calls = [ast.unparse(node.func) for node in ast.walk(tree) if isinstance(node, ast.Call)]
        self.assertNotIn('run.measure', calls)
        self.assertFalse(any('wait4' in call or 'getrusage' in call for call in calls))
        self.assertFalse(any(isinstance(node.value, ast.Constant) and node.value.value is True for node in ast.walk(tree) if isinstance(node, ast.keyword) and node.arg == 'shell'))

    def test_relocation_requires_equal_effective_existing_selection_inputs(self):
        old = dict(effectiveGlobalIgnore='/private/global', files={'/old/.gitignore': dict(exists=True, sha256='a', resolved='/old/.gitignore'), '/old/../.ignore': dict(exists=False)})
        new = dict(effectiveGlobalIgnore='/private/global', files={'/new/.gitignore': dict(exists=True, sha256='a', resolved='/new/.gitignore'), '/new/parent/.ignore': dict(exists=False)})
        execution.equivalent_selection(old, new, '/old', '/new')
        new['files']['/new/.gitignore']['sha256'] = 'b'
        with self.assertRaises(b.Rejected):
            execution.equivalent_selection(old, new, '/old', '/new')

    def test_current_author_cannot_approve_clock_derivation(self):
        review = dict(verdict='PASS', reviewerModel='codex:gpt-6.1-sol@high', reviewerId='execution_only_oss_reassessment', harnessBindings={})
        with patch.object(b, 'document', return_value=review), self.assertRaises(b.Rejected):
            main.review_gate(dict(path='/synthetic/review', sha256='a' * 64))

    def test_either_temporal_writer_cannot_approve_clock_derivation(self):
        for identity in ('temporal_clock_plan_implementation', 'temporal_overlay_and_witness_implementation'):
            review = dict(verdict='PASS', reviewerModel='codex:gpt-6.1-sol@high', reviewerId=identity, harnessBindings={})
            with self.subTest(identity=identity), patch.object(b, 'document', return_value=review), self.assertRaises(b.Rejected):
                main.review_gate(dict(path='/synthetic/review', sha256='a' * 64))

    def test_actual_run_stays_disabled_before_other_data_reads(self):
        with patch.object(b, 'document', return_value=dict(kind='execution-only-oss98-final-admission-v1', eligibleActualRuns=False)), \
                patch.object(b, 'verify_source') as source, self.assertRaises(b.Rejected):
            execution.admit(dict(path='/synthetic/admission', sha256='a' * 64))
        source.assert_not_called()

    def test_metadata_publish_refuses_history_overwrite(self):
        path = self.root / 'receipt.json'
        b.publish(path, dict(kind='synthetic', state='original'))
        original = path.read_bytes()
        with self.assertRaises(FileExistsError):
            b.publish(path, dict(kind='synthetic', state='new'))
        self.assertEqual(path.read_bytes(), original)


class DeltaTests(PrivateFixture):
    def finding(self, identity='f1'):
        return dict(finding_id=identity, rule_id='C_SHARP', location=dict(file='a.cs', line=1, column=1),
                    description='before', migration_hint='before', severity='low', risk='unknown',
                    evidence=dict(type='source', match='RSA', metadata={}))

    def test_legacy_rule_position_and_order_are_preserved(self):
        old = dict(findings=[self.finding('f1'), self.finding('f2')])
        self.assertTrue(delta.compare_legacy(old, copy.deepcopy(old))['passed'])
        changed = copy.deepcopy(old); changed['findings'][0]['rule_id'] = 'CHANGED'
        self.assertFalse(delta.compare_legacy(old, changed)['passed'])
        reordered = dict(findings=old['findings'][::-1])
        with self.assertRaises(b.Rejected):
            delta.compare_legacy(old, reordered)
        with self.assertRaises(b.Rejected):
            delta.compare_legacy(old, dict(findings=[]))

    def test_only_bound_independently_confirmed_csharp_removals_are_allowed(self):
        old = dict(findings=[self.finding()]); evidence = b.ref(self.write('evidence.json', b'public synthetic AST proof'))
        removal = dict(findingId='f1', reason='confirmed_csharp_predefined_type_false_positive', independentlyApproved=True,
                       astNodeKind='predefined_type', sourceSha256='a' * 64, ruleId='C_SHARP', location=old['findings'][0]['location'], evidence=evidence)
        result = delta.compare_legacy(old, dict(findings=[]), [removal])
        self.assertEqual((result['originalCount'], result['newCount'], result['removedCount']), (1, 0, 1))
        removal['astNodeKind'] = 'identifier'
        with self.assertRaises(b.Rejected):
            delta.compare_legacy(old, dict(findings=[]), [removal])

    def test_presentation_delta_requires_exact_before_after_digests(self):
        old = dict(findings=[self.finding()]); new = copy.deepcopy(old); new['findings'][0]['description'] = 'after'
        change = dict(findingId='f1', path=['description'], reason='description_wording', independentlyApproved=True,
                      beforeDigest=b.digest('before'), afterDigest=b.digest('after'), evidence=b.ref(self.write('proof', b'public proof')))
        self.assertTrue(delta.compare_legacy(old, new, changes=[change])['passed'])
        change['afterDigest'] = '0' * 64
        with self.assertRaises(b.Rejected):
            delta.compare_legacy(old, new, changes=[change])

    def test_rule_and_positions_cannot_be_presentation_normalizations(self):
        old = dict(findings=[self.finding()]); new = copy.deepcopy(old); new['findings'][0]['rule_id'] = 'NEW'
        change = dict(findingId='f1', path=['rule_id'], reason='description_wording', independentlyApproved=True,
                      beforeDigest=b.digest('C_SHARP'), afterDigest=b.digest('NEW'), evidence=b.ref(self.write('proof', b'public proof')))
        with self.assertRaises(b.Rejected):
            delta.compare_legacy(old, new, changes=[change])

    def test_native_occurrence_mapping_rewrites_references_not_arbitrary_payloads(self):
        native = dict(assets=[dict(id='asset1', relations=[])], observations=[dict(id='obs1', asset_ids=['asset1'])],
                      legacy_associations=[dict(observation_ids=['obs1'])], unassociated_observation_ids=['obs1'],
                      legacy_loss_records=[dict(observation_id='obs1')], run=dict(id='new-run', generated_at='new-time'), rules_digest='new-rules')
        mapped = delta.normalize_native(native, [dict(id='asset1', occurrenceKey='source-a'), dict(id='obs1', occurrenceKey='source-o')],
                                        [['run', 'id'], ['run', 'generated_at'], ['rules_digest']])
        self.assertEqual(mapped['observations'][0]['asset_ids'], ['source-a'])
        self.assertEqual(mapped['legacy_associations'][0]['observation_ids'], ['source-o'])
        with self.assertRaises(b.Rejected):
            delta.normalize_native(native, [], [])
        with self.assertRaises(b.Rejected):
            delta.normalize_native(native, [dict(id='asset1', occurrenceKey='a'), dict(id='obs1', occurrenceKey='o')], [['observations']])


if __name__ == '__main__':
    unittest.main()
