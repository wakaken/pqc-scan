import copy
import json
import tempfile
from pathlib import Path
from fractions import Fraction
import unittest

import boundary as b
import clocks


def observed_row():
    return dict(repositoryId='synthetic', phaseId='primary-after', clock=dict(
        kind='observed_original_second', assessmentTimeUnix=1793491201,
        originalGeneratedAt='2026-11-01T00:00:04Z', originalNativeGeneratedAt='2026-11-01T00:00:04+00:00',
        originalStartedUtc='2026-11-01T00:00:00+00:00', generatedSecond=1793491204, generatedNanosecond=0,
        firstPossibleSecond=1793491200, lastPossibleSecond=1793491204))


class ExactInstantTests(unittest.TestCase):
    def test_nanoseconds_never_round_across_second(self):
        left = clocks.instant('2026-11-01T00:00:00.999999999Z')
        right = clocks.instant('2026-11-01T00:00:01.000000001Z')
        self.assertEqual((left.second, left.nanosecond), (1793491200, 999999999))
        self.assertEqual((right.second, right.nanosecond), (1793491201, 1))
        self.assertEqual(clocks.unix_time(right.original_lexeme) - clocks.unix_time(left.original_lexeme), Fraction(2, 1_000_000_000))
        self.assertEqual(left.serialized(True), '2026-11-01T00:00:00.999999999+00:00')
        self.assertEqual(right.serialized(), right.original_lexeme)

    def test_chrono_auto_fraction_spelling(self):
        for fraction, canonical in [('.1', '.100'), ('.1234', '.123400'), ('.1234567', '.123456700'), ('', '')]:
            parsed = clocks.instant('2026-11-01T00:00:00' + fraction + 'Z')
            self.assertEqual(parsed.serialized(), '2026-11-01T00:00:00' + canonical + 'Z')

    def test_calendar_and_epoch_bounds(self):
        self.assertEqual(clocks.instant('1970-01-01T00:00:00Z').second, 0)
        self.assertEqual(clocks.instant('9999-12-31T23:59:59.999999999Z').second, 253402300799)
        self.assertEqual(clocks.instant('2000-02-29T00:00:00Z').second, 951782400)
        for value in ['1969-12-31T23:59:59Z', '0000-01-01T00:00:00Z', '1900-02-29T00:00:00Z',
                      '2026-11-31T00:00:00Z', '2026-11-01T24:00:00Z', '2026-11-01T00:00:60Z',
                      '10000-01-01T00:00:00Z']:
            with self.subTest(value=value), self.assertRaises(b.Rejected):
                clocks.instant(value)

    def test_unknown_precision_and_zone_rejected(self):
        for suffix in ['.1234567890Z', '.1e2Z', '.Z', 'z', '+01:00', '-00:00', '+0000', '', 'Z\n']:
            with self.subTest(suffix=suffix), self.assertRaises(b.Rejected):
                clocks.instant('2026-11-01T00:00:00' + suffix)

    def test_legacy_native_relation_requires_exact_source_serializer(self):
        report = dict(generated_at='2026-11-01T00:00:04.000000001Z', findings=[])
        native = dict(run=dict(generated_at='2026-11-01T00:00:04.000000001+00:00'), assets=[])
        started, generated = clocks._clock_times(report, native, '2026-11-01T00:00:00.999999999+00:00')
        self.assertEqual((started.second, generated.second, generated.nanosecond), (1793491200, 1793491204, 1))
        for wrong in ['2026-11-01T00:00:04Z', '2026-11-01T00:00:04.000000002+00:00',
                      '2026-11-01T00:00:04.000000001Z']:
            native['run']['generated_at'] = wrong
            with self.subTest(wrong=wrong), self.assertRaises(b.Rejected):
                clocks._clock_times(report, native, '2026-11-01T00:00:00+00:00')


class IntervalTests(unittest.TestCase):
    def proof(self, endpoints=(), emissions=()):
        return clocks._CoverageProof(dict(path='/synthetic/reviewed-proof', sha256='a' * 64, bytes=100), tuple(endpoints), tuple(emissions))

    def test_before_inside_after_cells_are_inclusive_at_valid_endpoints(self):
        for first, last, before, after in [(1, 2, 3, 4), (3, 4, 3, 4), (5, 6, 3, 4),
                                           (3, 3, 3, 4), (4, 4, 3, 4), (3, 3, 3, 3)]:
            with self.subTest(first=first, last=last):
                self.proof([(before, after)]).prove_interval(first, last)

    def test_every_possible_second_must_remain_in_one_cell(self):
        for window in [(2, 3), (4, 5), (2, 5)]:
            with self.subTest(window=window), self.assertRaises(b.Rejected):
                self.proof([(3, 4)]).prove_interval(*window)
        with self.assertRaises(b.Rejected):
            self.proof([(3, 4), (2, 3)]).prove_interval(3, 4)
        with self.assertRaises(b.Rejected):
            self.proof([(4, 3)]).prove_interval(5, 6)

    def test_numeric_clock_emission_breaks_interval_equivalence(self):
        with self.assertRaisesRegex(b.Rejected, 'E_CLOCK_EXACT_EMISSION_UNPROVEN'):
            self.proof([(3, 9)], [('a.pem', 0)]).prove_interval(4, 5)
        self.proof([(3, 9)], [('a.pem', 0)]).prove_interval(4, 4)

    def test_interval_is_explicitly_qualified_and_lower_bound_selected(self):
        report = dict(generated_at='2026-11-01T00:00:04.000000001Z', findings=[])
        native = dict(run=dict(generated_at='2026-11-01T00:00:04.000000001+00:00'), assets=[])
        result = clocks.derive_clock(report, native, '2026-11-01T00:00:00.999999999+00:00', self.proof([(0, 2000000000)]))
        self.assertEqual(result['kind'], 'interval_equivalent_not_observed')
        self.assertEqual((result['assessmentTimeUnix'], result['lastPossibleSecond']), (1793491200, 1793491204))
        self.assertEqual(result['generatedNanosecond'], 1)
        self.assertNotIn('fallback', result)

    def test_caller_boolean_or_native_absence_is_never_coverage(self):
        report = dict(generated_at='2026-11-01T00:00:04Z', findings=[])
        native = dict(run=dict(generated_at='2026-11-01T00:00:04+00:00'), assets=[])
        for proof in [None, True, dict(complete=True), dict(objects=[])]:
            with self.subTest(proof=proof), self.assertRaises(b.Rejected):
                clocks.derive_clock(report, native, '2026-11-01T00:00:00+00:00', proof)


class ReplayTests(unittest.TestCase):
    def test_whole_pair_before_spawn_and_discard_ambient_clock_overrides(self):
        import adapter
        row = observed_row()
        replay = clocks.ProjectReplay._from_row(row)
        environment = replay.bind_process(dict(LANG='C', TZ=None),
            {adapter.DETECTOR_ENV: 'bad', adapter.GENERATED_ENV: 'bad', 'TZ': 'bad', 'LANG': 'bad', 'OTHER': 'preserved'})
        self.assertEqual(environment[adapter.DETECTOR_ENV], '1793491201')
        self.assertEqual(environment[adapter.GENERATED_ENV], '2026-11-01T00:00:04Z')
        self.assertEqual(environment['LANG'], 'C')
        self.assertNotIn('TZ', environment)
        self.assertEqual(environment['OTHER'], 'preserved')
        for name, value in [('assessmentTimeUnix', True), ('assessmentTimeUnix', -1), ('generatedNanosecond', False),
                            ('originalGeneratedAt', '2026-11-01T00:00:04.1Z'),
                            ('originalNativeGeneratedAt', '2026-11-01T00:00:04Z')]:
            changed = copy.deepcopy(row)
            changed['clock'][name] = value
            with self.subTest(name=name, value=value), self.assertRaises(b.Rejected):
                clocks.ProjectReplay._from_row(changed)

    def test_both_output_clocks_and_every_emission_are_checked(self):
        replay = clocks.ProjectReplay._from_row(observed_row())
        report = dict(generated_at='2026-11-01T00:00:04Z', findings=[dict(evidence=dict(metadata=dict(assessment_time_unix='1793491201')))])
        native = dict(run=dict(generated_at='2026-11-01T00:00:04+00:00'), observations=[])
        replay.verify_outputs(report, native)
        for target, path, value in [('report', ('generated_at',), '2026-11-01T00:00:05Z'),
                                    ('native', ('run', 'generated_at'), '2026-11-01T00:00:05+00:00')]:
            changed_report, changed_native = copy.deepcopy(report), copy.deepcopy(native)
            current = changed_report if target == 'report' else changed_native
            for part in path[:-1]:
                current = current[part]
            current[path[-1]] = value
            with self.subTest(target=target), self.assertRaises(b.Rejected):
                replay.verify_outputs(changed_report, changed_native)
        for value in ['1793491202', 1793491201, True, '01793491201']:
            report['findings'][0]['evidence']['metadata']['assessment_time_unix'] = value
            with self.subTest(value=value), self.assertRaises(b.Rejected):
                replay.verify_outputs(report, native)

    def test_one_binary_uses_independent_process_pairs(self):
        import adapter
        first = clocks.ProjectReplay._from_row(observed_row())
        row = observed_row()
        row['clock']['assessmentTimeUnix'] = 1793491202
        second = clocks.ProjectReplay._from_row(row)
        env1, env2 = first.bind_process({}), second.bind_process({})
        self.assertEqual((env1[adapter.DETECTOR_ENV], env2[adapter.DETECTOR_ENV]), ('1793491201', '1793491202'))
        self.assertEqual(first.bind_process({}), env1)


class HistoricalEntryCompatibilityTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(dir=b.OWN)
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.recovered_root = self.root / 'recovered'
        self.recovered_root.mkdir()
        self.manifest = self.root / 'manifest.json'
        self.manifest.write_text(json.dumps([dict(path='public/retained.txt', sha256='a' * 64)]))

    def entries(self, identity):
        path, blob, failure, number = clocks.AUTHORIZED_FIXTURES[identity]
        unavailable = dict(path=path, gitBlob=blob, errorType=failure, errno=number)
        old = dict(id=identity, population='comparison', split='comparison', commit='a' * 40,
            root='/historical/public-tree', scope=clocks.HISTORICAL_SCOPE,
            inputManifest=str(self.manifest), inputManifestSha256=b.ref(self.manifest)['sha256'],
            historicalInputBindingSha256='b' * 64, repository='https://example.test/public',
            inputLimitations=dict(unavailable=[unavailable], uninitializedSubmodules=[]))
        current = copy.deepcopy(old)
        current.update(root=str(self.recovered_root),
            scope=old['scope'] + '; excludes user-authorized upstream test fixture ' + path)
        current['inputLimitations'].pop('unavailable')
        current['inputLimitations']['authorizedExclusions'] = [dict(unavailable,
            reason=clocks.EXCLUSION_REASON, authorization=clocks.EXCLUSION_AUTHORIZATION)]
        return old, current

    def reject(self, current, old):
        with self.assertRaisesRegex(b.Rejected, 'E_COMPATIBILITY_TEST'):
            clocks.admit_recovered_entry(current, old, 'E_COMPATIBILITY_TEST')

    def test_both_authorized_amendments_preserve_original_and_recovery_metadata(self):
        for identity in clocks.AUTHORIZED_FIXTURES:
            old, current = self.entries(identity)
            before = copy.deepcopy((old, current))
            clocks.admit_recovered_entry(current, old, 'E_COMPATIBILITY_TEST')
            self.assertEqual((old, current), before)

    def test_unchanged_scope_and_root_only_relocation_remain_compatible(self):
        old, _ = self.entries('nodejs__node')
        old['id'] = 'public-unaffected'
        clocks.admit_recovered_entry(copy.deepcopy(old), old, 'E_COMPATIBILITY_TEST')
        clocks.admit_recovered_entry(dict(old, root='/recovered/tree'), old, 'E_COMPATIBILITY_TEST')
        self.reject(dict(old, scope='different scope'), old)

    def test_all_other_entry_fields_remain_exact(self):
        old, current = self.entries('nodejs__node')
        for name in old:
            if name in ('root', 'scope', 'inputLimitations'):
                continue
            changed = copy.deepcopy(current)
            changed[name] = 'changed'
            with self.subTest(name=name):
                self.reject(changed, old)
        self.reject(dict(current, addedMetadata=True), old)
        for name in ('scope', 'inputManifest', 'commit'):
            changed = copy.deepcopy(current)
            del changed[name]
            self.reject(changed, old)

    def test_exclusion_identity_error_authorization_and_cardinality_are_exact(self):
        for identity in clocks.AUTHORIZED_FIXTURES:
            old, current = self.entries(identity)
            for name, value in [('path', 'other/public.txt'), ('gitBlob', 'f' * 40),
                    ('errorType', 'OtherError'), ('errno', True), ('reason', 'other reason'),
                    ('authorization', 'other authorization')]:
                changed = copy.deepcopy(current)
                changed['inputLimitations']['authorizedExclusions'][0][name] = value
                with self.subTest(identity=identity, name=name):
                    self.reject(changed, old)
            changed = copy.deepcopy(current)
            changed['inputLimitations']['authorizedExclusions'] *= 2
            self.reject(changed, old)
            changed = copy.deepcopy(current)
            changed['inputLimitations']['unavailable'] = []
            self.reject(changed, old)
            changed = copy.deepcopy(current)
            changed['inputLimitations']['uninitializedSubmodules'] = ['changed']
            self.reject(changed, old)
            changed = copy.deepcopy(old)
            changed['inputLimitations']['unavailable'][0]['errno'] = True
            self.reject(current, changed)
            changed = copy.deepcopy(old)
            changed['inputLimitations']['authorizedExclusions'] = []
            self.reject(current, changed)
            self.reject(dict(current, scope=current['scope'] + '; another exclusion'), old)
            self.reject(dict(current, id='unknown'), dict(old, id='unknown'))

    def test_manifest_hash_path_unique_rows_and_absent_fixture_are_required(self):
        for rows in ([dict(path=clocks.AUTHORIZED_FIXTURES['nodejs__node'][0])], [],
                     [dict(path='public/a'), dict(path='public/a')], [dict(other='public/a')],
                     dict(files=[dict(path='public/a')])):
            self.manifest.write_text(json.dumps(rows))
            old, current = self.entries('nodejs__node')
            with self.subTest(rows=rows):
                self.reject(current, old)
        self.manifest.write_text(json.dumps([dict(path='public/a')]))
        old, current = self.entries('nodejs__node')
        self.manifest.write_text(json.dumps([dict(path='public/changed')]))
        self.reject(current, old)
        old, current = self.entries('nodejs__node')
        other = self.root / 'other-manifest.json'
        other.write_bytes(self.manifest.read_bytes())
        self.reject(dict(current, inputManifest=str(other)), old)

    def test_new_excluded_file_and_dangling_symlink_are_rejected_without_reading_them(self):
        for identity in clocks.AUTHORIZED_FIXTURES:
            old, current = self.entries(identity)
            target = self.recovered_root / clocks.AUTHORIZED_FIXTURES[identity][0]
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text('Public synthetic fixture, never read by compatibility validation.')
            self.reject(current, old)
            target.unlink()
            target.symlink_to(self.root / 'missing-target')
            self.assertFalse(target.exists())
            self.reject(current, old)
            target.unlink()
            clocks.admit_recovered_entry(current, old, 'E_COMPATIBILITY_TEST')

    def test_symlink_ancestor_cannot_hide_an_excluded_fixture(self):
        old, current = self.entries('nodejs__node')
        outside = self.root / 'outside'
        outside.mkdir()
        (self.recovered_root / 'deps').symlink_to(outside, target_is_directory=True)
        self.reject(current, old)
