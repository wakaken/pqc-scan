import dataclasses
import gzip
import json
import pathlib
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

import oss
from evaluate import bytes_digest, digest


def save(path, value):
    path.write_text(json.dumps(value))


SCANNER = '''#!/usr/bin/env python3
import json, pathlib, sys, time
MODE = {mode!r}
if '--version' in sys.argv:
    print('fake scanner fixture 1')
    raise SystemExit(0)
if MODE == 'timeout':
    time.sleep(5)
root = pathlib.Path(sys.argv[2])
out = pathlib.Path(sys.argv[sys.argv.index('--out-dir') + 1])
out.mkdir(exist_ok=True)
assert '--inventory-json' in sys.argv and '--cyclonedx-17' in sys.argv
report = {{'generated_at':'fixture-time','findings':[], 'cbom':[], 'dependency_sbom':[], 'summary':{{'scanned_files':1}}}}
coverage = {{'selected_files':1, 'read_ok_files':1, 'read_failed_files':0, 'analyzed_files':1,
             'partial_files':0,'analysis_failed_files':0,'unsupported_files':0,'not_applicable_files':0,'unknown_files':0,'enumeration_failed_entries':0}}
if MODE == 'partial':
    coverage.update(analyzed_files=0,partial_files=1)
if MODE == 'unsupported':
    coverage.update(analyzed_files=0,unsupported_files=1)
if MODE == 'enumeration-failed':
    coverage.update(enumeration_failed_entries=1)
if MODE == 'invalid-enumeration':
    coverage.update(enumeration_failed_entries=-1)
if MODE == 'missing-enumeration':
    coverage.pop('enumeration_failed_entries')
for name in {required!r}:
    value = report if name == 'report.json' else {{'coverage':coverage,'diagnostics':[]}} if name == 'diagnostics.json' else {{'run':{{'id':'fixture-run','generated_at':'fixture-time'}},'assets':[],'observations':[]}} if name == 'scan.inventory.v1.json' else {{}}
    (out/name).write_text(json.dumps(value) if name.endswith(('.json','.sarif')) else 'fixture report')
if MODE == 'bad-json':
    (out/'report.json').write_text('{{')
if MODE == 'missing':
    (out/'cbom.cdx-1.7.json').unlink()
if MODE == 'mutate-input':
    (root/'a.rs').write_text('changed during execution')
if MODE == 'failed':
    print('preserved fixture diagnostic', file=sys.stderr)
    raise SystemExit(7)
'''


class OssBenchmarkTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='pqc-oss-harness-test-')
        self.addCleanup(self.temporary.cleanup)
        self.base = pathlib.Path(self.temporary.name).resolve()
        self.root = self.base/'input'
        self.root.mkdir()
        (self.root/'a.rs').write_text('fixture static input')
        self.manifest = self.base/'input.json'
        self.registry = self.base/'registry.json'
        self.protocol = self.base/'protocol.json'
        self.build = self.base/'build.json'
        self.binary = self.base/'scanner'
        self.rules = self.base/'rules'
        self.rules.mkdir()
        (self.rules/'fixture.yml').write_text('fixture rule')
        self.row = dict(id='fixture__one', population='primary', group='Rust', languages=['Rust'], priority=True,
            repository='https://example.invalid/fixture/one', commit='1'*40, split='tuning',
            root=str(self.root), scope='fixture tracked tree', inputManifest=str(self.manifest))
        self.refresh_input()
        save(self.protocol, dict(repeats=1, threads=1, timeoutSeconds=3, minFreeGiB=0))
        self.make_binary()
        self.environment = {key: 'fixture' for key in ('python','platform','machine','hardware','rust','cargo')}
        self.environment['cpuCount'] = 1
        patch = mock.patch.object(oss.run, 'environment', return_value=self.environment.copy())
        patch.start()
        self.addCleanup(patch.stop)

    def refresh_input(self):
        save(self.manifest, [dict(path=p.relative_to(self.root).as_posix(), mode='100644', bytes=p.stat().st_size,
                                  sha256=bytes_digest(p)) for p in sorted(self.root.rglob('*')) if p.is_file()])
        self.row['inputManifestSha256'] = bytes_digest(self.manifest)
        self.refresh_registry()

    def refresh_registry(self):
        save(self.registry, dict(schemaVersion=1, repositories=[self.row]))

    def refresh_build(self):
        rules = {p.relative_to(self.rules).as_posix(): bytes_digest(p) for p in self.rules.rglob('*') if p.is_file()}
        save(self.build, dict(source={'revision':'saved-product-source'}, binarySha256=bytes_digest(self.binary),
             rulesSha256=digest(rules), features=[], buildProfile='fixture'))

    def make_binary(self, mode='normal'):
        self.binary.write_text(SCANNER.format(mode=mode, required=oss.REQUIRED))
        self.binary.chmod(0o755)
        self.refresh_build()

    def request(self, **changes):
        request = oss.RunRequest(self.registry, oss.Selector(all=True), self.protocol, 'before', self.binary,
                                 self.rules, self.build, self.base/'evidence')
        return dataclasses.replace(request, **changes)

    def run_request(self, **changes):
        return oss.run_selected(self.request(**changes))

    def terminal(self, result):
        return pathlib.Path(result['rows'][0]['receipt']['path'])

    def test_fresh_resume_and_immutable_attempts(self):
        first = self.run_request()
        terminal = self.terminal(first)
        prior = terminal.read_bytes()
        self.assertTrue(first['executionComplete'])
        self.assertTrue(first['inspectionComplete'])
        self.assertFalse(first['evaluationComplete'])
        self.assertIsNone(first['rows'][0]['assessment']['accuracy'])
        self.assertEqual(first['scanner']['productSource'], {'revision':'saved-product-source'})
        self.assertIn('Coordinator', first['coordinatorEnvironment']['provenanceRole'])
        second = self.run_request(resume=True)
        self.assertTrue(second['rows'][0]['reused'])
        self.assertEqual(terminal, self.terminal(second))
        self.assertEqual(prior, terminal.read_bytes())
        self.assertNotEqual(first['receiptPath'], second['receiptPath'])
        self.assertTrue((terminal.parent/'started.json').is_file())
        self.assertFalse((terminal.parent/'output'/'started.json').exists())

    def test_resume_rejects_attempt_copied_from_another_phase(self):
        first = self.run_request()
        original = self.terminal(first)
        copied = self.base/'evidence'/'runs'/self.row['id']/'after'/'copied-attempt'
        shutil.copytree(original.parent, copied)
        original_bytes = original.read_bytes()
        second = self.run_request(phase='after', resume=True)
        self.assertFalse(second['rows'][0]['reused'])
        self.assertNotEqual(self.terminal(second).parent, copied)
        self.assertEqual(original.read_bytes(), original_bytes)
        started = json.loads((self.terminal(second).parent/'started.json').read_text())
        self.assertEqual(started['phase'], 'after')

    def test_resume_rejects_mismatched_started_repository_identity(self):
        first = self.run_request()
        terminal = self.terminal(first)
        started_path = terminal.parent/'started.json'
        started = json.loads(started_path.read_text())
        started['repositoryId'] = 'fixture__other'
        save(started_path, started)
        receipt = json.loads(terminal.read_text())
        receipt['startedDigest'] = digest(started)
        save(terminal, receipt)
        second = self.run_request(resume=True)
        self.assertFalse(second['rows'][0]['reused'])
        self.assertNotEqual(self.terminal(second), terminal)
        self.assertTrue(second['executionComplete'])

    def test_mutated_output_forces_new_attempt_and_preserves_old(self):
        first = self.run_request()
        terminal = self.terminal(first)
        prior = terminal.read_bytes()
        (terminal.parent/'output'/'report.html').write_text('corrupted')
        second = self.run_request(resume=True)
        self.assertFalse(second['rows'][0]['reused'])
        self.assertNotEqual(terminal, self.terminal(second))
        self.assertEqual(prior, terminal.read_bytes())
        self.assertEqual((terminal.parent/'output'/'report.html').read_text(), 'corrupted')

    def test_input_change_blocks_until_new_frozen_manifest_then_new_attempt(self):
        first = self.run_request()
        (self.root/'a.rs').write_text('new input')
        blocked = self.run_request(resume=True)
        self.assertFalse(blocked['executionComplete'])
        self.assertEqual(blocked['rows'][0]['execution'], 'NotRun')
        self.refresh_input()
        second = self.run_request(resume=True)
        self.assertTrue(second['executionComplete'])
        self.assertNotEqual(self.terminal(first), self.terminal(second))

    def test_changed_selection_protocol_harness_and_configuration_do_not_reuse(self):
        for change in ('selection', 'protocol', 'harness', 'configuration'):
            with self.subTest(change=change):
                first = self.run_request(phase=change)
                if change == 'selection':
                    (self.base/'.ignore').write_text('*.tmp\n')
                    second = self.run_request(phase=change, resume=True)
                elif change == 'protocol':
                    protocol = json.loads(self.protocol.read_text())
                    save(self.protocol, dict(protocol, note='new frozen protocol'))
                    second = self.run_request(phase=change, resume=True)
                elif change == 'harness':
                    bindings = oss._harness()
                    with mock.patch.object(oss, '_harness', return_value=dict(bindings, coordinator='new-version')):
                        second = self.run_request(phase=change, resume=True)
                else:
                    second = self.run_request(phase=change, resume=True, timeout=2)
                self.assertTrue(second['executionComplete'])
                self.assertFalse(second['rows'][0]['reused'])
                self.assertNotEqual(self.terminal(first), self.terminal(second))

    def test_stale_build_receipt_rejects_binary_and_rules_changes(self):
        for change in ('rules', 'binary'):
            with self.subTest(change=change):
                first = self.run_request(phase=change)
                path = self.rules/'fixture.yml' if change == 'rules' else self.binary
                path.write_text(path.read_text()+'\n# fixture revision\n')
                with self.assertRaises(ValueError):
                    self.run_request(phase=change, resume=True)
                self.refresh_build()
                second = self.run_request(phase=change, resume=True)
                self.assertFalse(second['rows'][0]['reused'])
                self.assertNotEqual(self.terminal(first), self.terminal(second))

    def test_failed_timeout_invalid_and_abandoned_attempts_never_resume(self):
        for mode, expected in [('failed','Failed'), ('timeout','TimedOut'), ('bad-json','InvalidEvidence'),
                               ('missing','InvalidEvidence'), ('mutate-input','InvalidEvidence')]:
            with self.subTest(mode=mode):
                self.make_binary(mode)
                timeout = 0.1 if mode == 'timeout' else None
                first = self.run_request(phase=mode, timeout=timeout)
                self.assertEqual(first['rows'][0]['execution'], expected)
                self.assertFalse(first['executionComplete'])
                if mode == 'mutate-input':
                    (self.root/'a.rs').write_text('fixture static input')
                second = self.run_request(phase=mode, timeout=timeout, resume=True)
                self.assertFalse(second['rows'][0]['reused'])
                self.assertNotEqual(self.terminal(first), self.terminal(second))
        self.make_binary()
        (self.root/'a.rs').write_text('fixture static input')
        abandoned = self.base/'evidence'/'runs'/self.row['id']/'abandoned'/'fixture-abandoned'
        abandoned.mkdir(parents=True)
        save(abandoned/'started.json', {'incomplete':'attempt'})
        result = self.run_request(phase='abandoned', resume=True)
        self.assertTrue(result['executionComplete'])
        self.assertFalse(result['rows'][0]['reused'])
        self.assertEqual(json.loads((abandoned/'started.json').read_text()), {'incomplete':'attempt'})
        self.assertFalse((abandoned/'terminal.json').exists())

    def test_interrupted_repeat_retains_exact_expected_matrix(self):
        self.make_binary('mutate-input')
        save(self.protocol, dict(repeats=2, threads=1, timeoutSeconds=3, minFreeGiB=0))
        result = self.run_request()
        self.assertEqual([(row['sample'], row['execution']) for row in result['rows']],
                         [(0, 'InvalidEvidence'), (1, 'NotRun')])
        self.assertFalse(result['executionComplete'])

    def test_partial_and_unsupported_are_authentic_reusable_observations(self):
        for mode in ('partial', 'unsupported', 'enumeration-failed'):
            with self.subTest(mode=mode):
                self.make_binary(mode)
                first = self.run_request(phase=mode)
                self.assertTrue(first['executionComplete'])
                self.assertFalse(first['inspectionComplete'])
                self.assertEqual(first['rows'][0]['assessment']['inspection'], 'Partial')
                second = self.run_request(phase=mode, resume=True)
                self.assertTrue(second['rows'][0]['reused'])
                self.assertFalse(second['inspectionComplete'])

    def test_invalid_enumeration_counter_cannot_certify_inspection(self):
        for mode in ('invalid-enumeration', 'missing-enumeration'):
            with self.subTest(mode=mode):
                self.make_binary(mode)
                result = self.run_request(phase=mode)
                self.assertFalse(result['executionComplete'])
                self.assertEqual(result['rows'][0]['execution'], 'InvalidEvidence')

    def test_historical_enumeration_assessment_is_verified_but_not_repeated(self):
        self.make_binary('enumeration-failed')
        harness = dict(oss._harness(), **{'tests/quality/oss.py': oss.LEGACY_ENUMERATION_ASSESSOR})
        artifacts = oss._artifacts
        with mock.patch.object(oss, '_harness', return_value=harness), mock.patch.object(
                oss, '_artifacts', side_effect=lambda *args, **kw: artifacts(*args, historical_enumeration=True)):
            old = self.run_request()
        terminal = self.terminal(old)
        original = terminal.read_bytes()
        self.assertEqual(oss._verify_terminal(terminal)[0]['assessment']['inspection'], 'Complete')
        comparison = oss.compare_runs(pathlib.Path(old['receiptPath']), pathlib.Path(old['receiptPath']), self.base/'comparison.json')
        self.assertEqual(comparison['rows'][0]['beforeInspection'], 'Partial')
        self.assertEqual(original, terminal.read_bytes())
        current = self.run_request(resume=True)
        self.assertFalse(current['rows'][0]['reused'])
        self.assertEqual(current['rows'][0]['assessment']['inspection'], 'Partial')
        started_path = terminal.parent/'started.json'
        started = json.loads(started_path.read_text())
        started['bindings']['harness']['tests/quality/oss.py'] = 'f'*64
        save(started_path, started)
        tampered = json.loads(terminal.read_text())
        tampered.update(startedDigest=digest(started), executionKey=digest(started['bindings']))
        save(terminal, tampered)
        with self.assertRaisesRegex(ValueError, 'assessment changed'):
            oss._verify_terminal(terminal)

    def test_missing_and_declared_unavailable_inputs_remain_explicit(self):
        self.row.pop('inputManifest')
        self.refresh_registry()
        missing = self.run_request()
        self.assertEqual(missing['rows'][0]['execution'], 'NotRun')
        self.assertFalse(missing['executionComplete'])
        self.row['inputManifest'] = str(self.manifest)
        self.row['inputLimitations'] = {'missing':['original-file.rs']}
        self.refresh_registry()
        partial = self.run_request()
        self.assertTrue(partial['executionComplete'])
        self.assertFalse(partial['inspectionComplete'])
        self.assertEqual(partial['rows'][0]['assessment']['inputLimitations'], self.row['inputLimitations'])
        self.row['inputLimitations'] = {'unreadablePresent':['blocked-file.rs']}
        self.refresh_registry()
        blocked = self.run_request()
        self.assertEqual(blocked['rows'][0]['execution'], 'NotRun')
        self.assertIn('unreadable', blocked['rows'][0]['reason'])

    def test_subset_receipt_overrides_full_root_and_cannot_fallback(self):
        self.row['id'] = 'Sustainsys__Saml2'
        self.row['repository'] = 'https://github.com/Sustainsys/Saml2'
        original = self.base/'full-root'
        original.mkdir()
        (original/'unlicensed.cs').write_text('not in licensed population')
        decision = self.base/'subset.json'
        record = dict(repository='Sustainsys/Saml2', url=self.row['repository'], commit=self.row['commit'],
                      root=str(self.root), scopeSelection='licensed fixture subtree',
                      originalInventorySha256='2'*64, inventory=str(self.manifest), inventorySha256=bytes_digest(self.manifest))
        save(decision, [record])
        self.row['historicalEvidence'] = [dict(record=str(self.base/'must-not-read.json')),
                                         dict(record=str(decision), inventory=str(self.manifest), inventorySha256=bytes_digest(self.manifest), scopeSelection=record['scopeSelection'])]
        self.refresh_registry()
        good = self.run_request()
        self.assertTrue(good['executionComplete'])
        start = json.loads((self.terminal(good).parent/'started.json').read_text())
        self.assertEqual(start['bindings']['input']['root'], str(self.root))
        record['root'] = str(original)
        save(decision, [record])
        rejected = self.run_request(resume=True)
        self.assertEqual(rejected['rows'][0]['execution'], 'NotRun')
        self.assertFalse(rejected['executionComplete'])

    def test_selectors_are_explicit_intersections_in_registry_order(self):
        another = dict(self.row, id='fixture__two', group='Java', priority=False, population='comparison')
        save(self.registry, {'repositories':[self.row, another]})
        self.assertEqual([r['id'] for r in oss.select_repositories(self.registry, oss.Selector(all=True))], [self.row['id'], another['id']])
        self.assertEqual(oss.select_repositories(self.registry, oss.Selector(population='comparison')), [another])
        for selector in (oss.Selector(), oss.Selector(repositories=('missing',)), oss.Selector(groups=('missing',)),
                         oss.Selector(all=True, priority=True), oss.Selector(population='comparison', priority=True)):
            with self.subTest(selector=selector), self.assertRaises(ValueError):
                oss.select_repositories(self.registry, selector)

    def test_comparison_revalidates_outputs_and_detects_incompatibility(self):
        before = self.run_request()
        after = self.run_request(phase='after')
        comparison = oss.compare_runs(pathlib.Path(before['receiptPath']), pathlib.Path(after['receiptPath']), self.base/'comparison.json')
        self.assertEqual(comparison['rows'][0]['status'], 'Comparable')
        self.assertTrue(comparison['rows'][0]['semanticEqual'])
        self.assertEqual(set(comparison['rows'][0]['outputComparisons']), set(oss.REQUIRED))
        self.assertEqual(comparison['populations']['primary']['comparableSamples'], 1)
        self.assertIsNone(comparison['accuracy'])
        (self.base/'.ignore').write_text('changed selection')
        changed = self.run_request(phase='changed')
        incompatible = oss.compare_runs(pathlib.Path(before['receiptPath']), pathlib.Path(changed['receiptPath']), self.base/'incompatible.json')
        self.assertEqual(incompatible['rows'][0]['status'], 'Incompatible')
        (self.terminal(after).parent/'output'/'report.json').write_text('{}')
        with self.assertRaises(ValueError):
            oss.compare_runs(pathlib.Path(before['receiptPath']), pathlib.Path(after['receiptPath']), self.base/'rejected.json')
        self.assertFalse((self.base/'rejected.json').exists())

    def test_alternate_registry_attempts_cannot_replace_frozen_request_rows(self):
        before = self.run_request()
        after = self.run_request(phase='after')
        self.row.update(group='Java', languages=['Java'])
        self.refresh_registry()
        other_before = self.run_request()
        other_after = self.run_request(phase='after')
        for original, alternate in [(before, other_before), (after, other_after)]:
            path = pathlib.Path(original['receiptPath'])
            value = json.loads(path.read_text())
            value['rows'] = json.loads(pathlib.Path(alternate['receiptPath']).read_text())['rows']
            save(path, value)
        with self.assertRaises(ValueError):
            oss.compare_runs(pathlib.Path(before['receiptPath']), pathlib.Path(after['receiptPath']), self.base/'registry-takeover.json')
        self.assertFalse((self.base/'registry-takeover.json').exists())

    def test_comparison_requires_same_harness_and_build_flags(self):
        before = self.run_request()
        harness = oss._harness()
        with mock.patch.object(oss, '_harness', return_value=dict(harness, fixture='new coordinator')):
            after = self.run_request(phase='new-harness')
        result = oss.compare_runs(pathlib.Path(before['receiptPath']), pathlib.Path(after['receiptPath']), self.base/'harness-comparison.json')
        self.assertEqual(result['rows'][0]['status'], 'Incompatible')
        build = json.loads(self.build.read_text())
        save(self.build, dict(build, features=['optional-fixture']))
        after = self.run_request(phase='new-features')
        result = oss.compare_runs(pathlib.Path(before['receiptPath']), pathlib.Path(after['receiptPath']), self.base/'features-comparison.json')
        self.assertEqual(result['rows'][0]['status'], 'Incompatible')

    def test_comparison_does_not_drop_failed_samples(self):
        before = self.run_request()
        self.make_binary('failed')
        after = self.run_request(phase='after')
        comparison = oss.compare_runs(pathlib.Path(before['receiptPath']), pathlib.Path(after['receiptPath']), self.base/'comparison.json')
        self.assertEqual(comparison['rows'][0]['status'], 'Unavailable')
        self.assertEqual(comparison['repositories'][self.row['id']]['comparableSamples'], 0)

    def test_resource_and_json_boundaries_fail_closed(self):
        with mock.patch.object(oss.shutil, 'disk_usage', return_value=type('Disk', (), {'free':0})()):
            result = self.run_request(min_free_gib=1)
        self.assertEqual(result['rows'][0]['execution'], 'NotRun')
        for timeout in (float('nan'), float('inf'), 0, -1):
            with self.subTest(timeout=timeout), self.assertRaises(ValueError):
                self.run_request(timeout=timeout)
        compressed = self.base/'expansion.json.gz'
        with gzip.open(compressed, 'wb') as stream:
            stream.write(b'['+b'0,'*1000+b'0]')
        with self.assertRaises(ValueError):
            oss._json(compressed, 128)
        rejected = self.run_request(output=self.root/'evidence')
        self.assertEqual(rejected['rows'][0]['execution'], 'NotRun')
        self.assertFalse(rejected['executionComplete'])

    def test_overlapping_output_never_writes_input_or_evidence(self):
        for output in (self.root/'evidence', self.base):
            with self.subTest(output=output):
                before = {str(p.relative_to(self.base)): bytes_digest(p) if p.is_file() else None
                          for p in self.base.rglob('*')}
                result = self.run_request(output=output)
                after = {str(p.relative_to(self.base)): bytes_digest(p) if p.is_file() else None
                         for p in self.base.rglob('*')}
                self.assertEqual(after, before)
                self.assertTrue(all(row['execution'] == 'NotRun' for row in result['rows']))
                self.assertFalse(result['executionComplete'])
                self.assertNotIn('receiptPath', result)
                self.assertIn('disjoint', result['rejectionReason'])

    def test_later_selected_input_overlap_prevents_all_publication(self):
        first_root = self.base/'first-input'
        first_root.mkdir()
        (first_root/'a.rs').write_text('another static input')
        first_manifest = self.base/'first-input.json'
        save(first_manifest, [dict(path='a.rs', mode='100644', bytes=(first_root/'a.rs').stat().st_size,
                                  sha256=bytes_digest(first_root/'a.rs'))])
        first = dict(self.row, id='fixture__first', root=str(first_root), inputManifest=str(first_manifest),
                     inputManifestSha256=bytes_digest(first_manifest))
        save(self.registry, dict(repositories=[first, self.row]))
        before = {str(p.relative_to(self.base)): bytes_digest(p) if p.is_file() else None
                  for p in self.base.rglob('*')}
        result = self.run_request(output=self.root/'evidence')
        after = {str(p.relative_to(self.base)): bytes_digest(p) if p.is_file() else None
                 for p in self.base.rglob('*')}
        self.assertEqual(after, before)
        self.assertEqual([row['execution'] for row in result['rows']], ['NotRun', 'NotRun'])
        self.assertNotIn('receiptPath', result)

    def test_declared_root_overlap_rejected_even_when_admission_fails(self):
        for source in ('registry', 'acquisition'):
            with self.subTest(source=source):
                self.row['inputManifestSha256'] = '0'*64
                self.row['acquisition'] = {'root':str(self.base/'unused-root')}
                if source == 'acquisition':
                    self.row['root'] = str(self.base/'unused-root')
                    self.row['acquisition']['root'] = str(self.root)
                self.refresh_registry()
                before = {str(p.relative_to(self.base)): bytes_digest(p) if p.is_file() else None
                          for p in self.base.rglob('*')}
                result = self.run_request(output=self.root/'evidence')
                after = {str(p.relative_to(self.base)): bytes_digest(p) if p.is_file() else None
                         for p in self.base.rglob('*')}
                self.assertEqual(after, before)
                self.assertEqual(result['rows'][0]['execution'], 'NotRun')
                self.assertNotIn('receiptPath', result)

    def test_comparison_requires_frozen_registry_entry(self):
        before = self.run_request()
        original = dict(self.row)
        for index, changes in enumerate(({'population':'comparison', 'split':'comparison'}, {'split':'holdout'},
                                         {'group':'Java'}, {'languages':['Java']})):
            with self.subTest(changes=changes):
                self.row = dict(original, **changes)
                self.refresh_registry()
                after = self.run_request(phase='after')
                result = oss.compare_runs(pathlib.Path(before['receiptPath']), pathlib.Path(after['receiptPath']),
                                          self.base/('registry-comparison-'+str(index)+'.json'))
                self.assertEqual(result['rows'][0]['status'], 'Incompatible')
                self.assertEqual(result['populations']['primary']['comparableSamples'], 0)
                self.assertEqual(result['populations']['comparison']['comparableSamples'], 0)

    def test_comparison_rejects_receipt_projection_takeover(self):
        save(self.registry, dict(repositories=[self.row, dict(self.row, id='fixture__two')]))
        save(self.protocol, dict(repeats=2, threads=1, timeoutSeconds=3, minFreeGiB=0))
        before = self.run_request()
        after = self.run_request(phase='after')
        destination = pathlib.Path(after['receiptPath'])
        original = json.loads(destination.read_text())
        for change in ('repository', 'sample', 'phase', 'configuration', 'scanner', 'protocol', 'harness',
                       'population', 'split', 'matrix'):
            with self.subTest(change=change):
                invocation = json.loads(json.dumps(original))
                if change in ('repository', 'sample', 'phase'):
                    source = original['rows'][2 if change == 'repository' else 1] if change != 'phase' else before['rows'][0]
                    for field in ('receipt', 'assessment'):
                        invocation['rows'][0][field] = source[field]
                elif change in ('population', 'split'):
                    invocation['rows'][0][change] = 'comparison'
                elif change == 'matrix':
                    invocation['selectedIds'][0] = 'fixture__replacement'
                    for row in invocation['rows']:
                        if row['repositoryId'] == self.row['id']:
                            row['repositoryId'] = 'fixture__replacement'
                else:
                    invocation[change] = {'replaced':'request binding'}
                save(destination, invocation)
                output = self.base/(change+'-rejected.json')
                with self.assertRaises(ValueError):
                    oss.compare_runs(pathlib.Path(before['receiptPath']), destination, output)
                self.assertFalse(output.exists())
        save(destination, original)
        result = oss.compare_runs(pathlib.Path(before['receiptPath']), destination, self.base/'unaltered-comparison.json')
        self.assertEqual([row['status'] for row in result['rows']], ['Comparable']*4)

    def test_comparison_rejects_attempt_from_different_frozen_request(self):
        before = self.run_request()
        after = self.run_request(phase='after')
        destination = pathlib.Path(after['receiptPath'])
        original = json.loads(destination.read_text())
        original_build = self.build.read_bytes()
        original_protocol = self.protocol.read_bytes()
        for change in ('configuration', 'scanner', 'protocol', 'harness'):
            with self.subTest(change=change):
                self.build.write_bytes(original_build)
                self.protocol.write_bytes(original_protocol)
                if change == 'configuration':
                    alternate = self.run_request(phase='after', timeout=2)
                elif change == 'scanner':
                    build = json.loads(self.build.read_text())
                    save(self.build, dict(build, features=['alternate-scanner']))
                    alternate = self.run_request(phase='after')
                elif change == 'protocol':
                    protocol = json.loads(self.protocol.read_text())
                    save(self.protocol, dict(protocol, note='alternate-request'))
                    alternate = self.run_request(phase='after')
                else:
                    harness = oss._harness()
                    with mock.patch.object(oss, '_harness', return_value=dict(harness, fixture='alternate-request')):
                        alternate = self.run_request(phase='after')
                invocation = json.loads(json.dumps(original))
                invocation['rows'][0] = alternate['rows'][0]
                save(destination, invocation)
                output = self.base/(change+'-attempt-rejected.json')
                with self.assertRaises(ValueError):
                    oss.compare_runs(pathlib.Path(before['receiptPath']), destination, output)
                self.assertFalse(output.exists())

    def test_actual_entry_cli_select_run_resume_compare_and_errors(self):
        entry = oss.REPO/'benchmark/entry.py'
        def invoke(arguments, code=0):
            result = subprocess.run([sys.executable, str(entry), *arguments], cwd=oss.REPO,
                                    capture_output=True, text=True, timeout=30)
            self.assertEqual(result.returncode, code, result.stderr)
            return json.loads(result.stdout) if result.stdout else None
        selected = invoke(['select','--registry',str(self.registry),'--repo',self.row['id']])
        self.assertEqual([r['id'] for r in selected], [self.row['id']])
        arguments = ['run','--registry',str(self.registry),'--all','--phase','driver','--binary',str(self.binary),
                     '--rules',str(self.rules),'--build-receipt',str(self.build),'--protocol',str(self.protocol),
                     '--output',str(self.base/'driver-evidence')]
        first = invoke(arguments)
        resumed = invoke(arguments+['--resume'])
        self.assertTrue(resumed['rows'][0]['reused'])
        compared = invoke(['compare','--before',first['receiptPath'],'--after',resumed['receiptPath'],
                           '--output',str(self.base/'driver-comparison.json')])
        self.assertEqual(compared['rows'][0]['status'], 'Comparable')
        invoke(['select','--registry',str(self.registry)], 2)
        invoke(['select','--registry',str(self.registry),'--repo','unknown'], 2)


if __name__ == '__main__':
    unittest.main()
