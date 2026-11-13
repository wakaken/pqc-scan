"""Hand-derived LLVM coverage accounting; no target code executes here."""
import unittest
from task044.coverage import summarize


class Coverage044Tests(unittest.TestCase):
    def test_production_only_and_zero_denominators(self):
        def row(name, total, covered):
            return {'filename': name, 'summary': {
                kind: {'count': total, 'covered': covered}
                for kind in ('lines', 'regions', 'functions', 'branches')}}
        result = summarize({'data': [{'files': [
            row('/source/crates/a/src/lib.rs', 4, 3),
            row('/source/crates/a/tests/check.rs', 100, 100),
            row('/scratch/cargo/registry/dep.rs', 100, 100),
            row('/source/crates/b/src/lib.rs', 0, 0)]}]})
        self.assertEqual(result['totals']['lines'], {'count': 4, 'covered': 3, 'percent': 75.0})
        self.assertEqual(result['files']['crates/b/src/lib.rs']['lines']['percent'], None)
        self.assertEqual(set(result['files']), {'crates/a/src/lib.rs', 'crates/b/src/lib.rs'})
        self.assertIsNone(result['totals']['branches']['percent'])
        self.assertEqual(result['totals']['branches']['status'], 'Not Measured')

    def test_invalid_evidence_and_duplicate_paths_fail(self):
        for value in ({}, {'data': []}, {'data': [{'files': []}]}):
            with self.subTest(value=value), self.assertRaises(ValueError):
                summarize(value)
        row = {'filename': '/source/crates/a/src/lib.rs', 'summary': {
            kind: {'count': 2, 'covered': 3} for kind in ('lines','regions','functions','branches')}}
        with self.assertRaises(ValueError):
            summarize({'data': [{'files': [row]}]})
        for kind in row['summary'].values():
            kind['covered'] = 1
        with self.assertRaises(ValueError):
            summarize({'data': [{'files': [row, row]}]})

    def test_branch_counters_are_not_inferred_from_lines(self):
        row = {'filename': '/source/crates/a/src/lib.rs', 'summary': {
            kind: {'count': 6, 'covered': 2} for kind in ('lines','regions','functions','branches')}}
        result = summarize({'data': [{'files': [row]}]}, branches=True)
        self.assertAlmostEqual(result['totals']['branches']['percent'], 100 / 3)
        self.assertEqual(result['totals']['branches']['status'], 'Measured')


class CoverageEntryTests(unittest.TestCase):
    def check_mode(self, mode, expected_measurements):
        import builtins
        import importlib.util
        from pathlib import Path
        import sys
        import tempfile
        from unittest import mock

        original_import = builtins.__import__
        def without_miri(name, *args, **kwargs):
            if 'miri' in name:
                raise ImportError('Miri support is deliberately unavailable')
            return original_import(name, *args, **kwargs)

        here = Path(__file__).resolve().parent
        spec = importlib.util.spec_from_file_location('coverage_study', here/'task044/entry.py')
        entry = importlib.util.module_from_spec(spec)
        with mock.patch.object(sys, 'path', [str(here.parent/'protocol'), *sys.path]), \
                mock.patch('builtins.__import__', side_effect=without_miri):
            spec.loader.exec_module(entry)
        with tempfile.TemporaryDirectory() as temp:
            entry.SCRATCH = Path(temp)
            entry.STUDY_MODE = mode
            measurements, commands = [], []
            entry.boundary = lambda: {'checked': True}
            entry.write = lambda *args: None
            entry.smoke = lambda: None
            entry.measure = lambda label, **kwargs: measurements.append((label, kwargs))
            def run(command, *args, **kwargs):
                self.assertNotIn('miri', command)
                if mode == 'coverage044':
                    self.assertFalse(any('nightly' in arg for arg in command))
                commands.append(command)
            entry.run = run
            entry.main()
            self.assertEqual(measurements, expected_measurements)
            self.assertEqual(len(commands), 1)

    def test_stable_coverage_needs_neither_miri_nor_nightly(self):
        self.check_mode('coverage044', [('default', {}), ('all-features', {'all_features': True})])

    def test_branch_coverage_is_explicit_and_has_no_miri_trial(self):
        self.check_mode('coverage044-branch', [
            ('branch-default', {'nightly': True}),
            ('branch-all-features', {'nightly': True, 'all_features': True})])
