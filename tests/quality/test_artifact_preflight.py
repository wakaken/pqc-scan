"""Synthetic transitive admission tests never open target artifacts."""
import copy
import hashlib
import importlib.util
import json
import pathlib
import unittest
from unittest.mock import patch
import test_accuracy_assessment as fixtures
a, save = fixtures.a, fixtures.save

SPEC = importlib.util.spec_from_file_location('artifact_preflight', pathlib.Path(__file__).resolve().parents[2] / 'benchmark/artifact_preflight.py')
p = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(p)


class PreflightTests(unittest.TestCase):
    def setUp(self):
        self.fixture = fixtures.AdmissionTests()
        self.fixture.setUp()
        self.addCleanup(self.fixture.tearDown)
        self.fixture.historical_fixture()
        self.wrapper_path = self.fixture.invocation_paths['primary-after']
        self.wrapper = a.load(self.wrapper_path)
        self.actual_path = pathlib.Path(self.wrapper['receiptPath'])
        self.terminal_path = pathlib.Path(self.wrapper['rows'][0]['receipt']['path'])
        self.output = self.terminal_path.parent / 'output'
        self.oss, _ = a.load_historical(fixtures.QUALITY)

    def rebind(self):
        actual = a.load(self.actual_path)
        actual['rows'][0]['receipt'] = a.reference(self.terminal_path)
        save(self.actual_path, actual)
        save(self.wrapper_path, dict(actual, receiptPath=str(self.actual_path)))
        return a.reference(self.wrapper_path)

    def verify(self):
        admitted = p.preflight_invocation(a.reference(self.wrapper_path))
        return self.oss._verified_invocation(admitted['path'])

    def assert_before_output_parse(self, action):
        original = self.oss._json
        read = []
        def traced(path, *args, **kwargs):
            if pathlib.Path(path).parent.name == 'output':
                read.append(True)
            return original(path, *args, **kwargs)
        with patch.object(self.oss, '_json', traced):
            with self.assertRaises(ValueError):
                action()
        self.assertFalse(read, 'output body was parsed before admission')

    def test_assessment_consumer_preflights_before_preserved_verifier(self):
        report = self.output / 'report.json'
        report.write_bytes(b'synthetic hash drift')
        registry = {'r': self.wrapper['selectedEntries']['r']}
        self.assert_before_output_parse(lambda: a.historical_snapshot(a.reference(self.wrapper_path), 'primary', 'after', registry, self.oss))

    def test_correct_row_reaches_unchanged_verifier(self):
        observed = []
        original = self.oss._json
        def traced(path, *args, **kwargs):
            observed.append(pathlib.Path(path).name)
            return original(path, *args, **kwargs)
        with patch.object(self.oss, '_json', traced):
            verified, rows = self.verify()
        self.assertEqual(verified['phase'], 'after')
        self.assertEqual(len(rows), 1)
        self.assertIn('report.json', observed)

    def test_ancestor_symlink_rejected_before_output_parse(self):
        actual_output = self.output.with_name('outside')
        self.output.rename(actual_output)
        self.output.symlink_to(actual_output, target_is_directory=True)
        self.assert_before_output_parse(self.verify)

    def test_metadata_ancestor_symlinks_and_missing_artifacts_rejected(self):
        actual_directory = self.actual_path.parent
        alias = actual_directory.parent / 'metadata-link'
        alias.symlink_to(actual_directory,target_is_directory=True)
        wrapper = dict(self.wrapper,receiptPath=str(alias/self.actual_path.name))
        save(self.wrapper_path,wrapper)
        self.assert_before_output_parse(self.verify)
        save(self.wrapper_path,self.wrapper)
        (self.output/'report.json').unlink()
        self.assert_before_output_parse(self.verify)

    def test_artifact_hash_and_size_rejected_before_output_parse(self):
        original = a.load(self.terminal_path)
        for field, value in [('sha256', '0' * 64), ('bytes', 9999999)]:
            terminal = copy.deepcopy(original)
            terminal['artifacts']['report.json'][field] = value
            save(self.terminal_path, terminal)
            self.rebind()
            self.assert_before_output_parse(self.verify)

    def test_terminal_hash_rejected_before_terminal_decode(self):
        self.terminal_path.write_bytes(b'not JSON')
        with patch.object(p, 'decode', wraps=p.decode) as decode:
            self.assert_before_output_parse(self.verify)
        self.assertEqual(decode.call_count, 3)

    def test_attempt_started_digest_rejected_before_output_parse(self):
        path = self.terminal_path.parent / 'started.json'
        started = a.load(path); started['phase'] = 'different'; save(path, started)
        self.assert_before_output_parse(self.verify)

    def test_unexpected_traversal_nested_and_symlink_artifacts_rejected(self):
        original = a.load(self.terminal_path)
        for name in ('../outside.json', 'sub/report.json', 'unexpected.json'):
            terminal = copy.deepcopy(original)
            terminal['artifacts'][name] = terminal['artifacts']['report.json']
            save(self.terminal_path, terminal); self.rebind()
            self.assert_before_output_parse(self.verify)
        save(self.terminal_path, original); self.rebind()
        report = self.output / 'report.json'; target = self.output.parent / 'other.json'
        report.rename(target); report.symlink_to(target)
        self.assert_before_output_parse(self.verify)

    def test_missing_duplicate_matrix_and_metadata_ancestor_rejected(self):
        actual = a.load(self.actual_path)
        for rows in ([], actual['rows'] * 2):
            changed = dict(actual, rows=rows); save(self.actual_path, changed)
            save(self.wrapper_path, dict(changed, receiptPath=str(self.actual_path)))
            self.assert_before_output_parse(self.verify)

    def test_large_html_streamed_and_oversize_rejected(self):
        html = self.output / 'report.html'
        with html.open('wb') as stream:
            for _ in range(65): stream.write(b'X' * (1024 * 1024))
        terminal = a.load(self.terminal_path)
        ref = p.checked_file(html, limit=p.MAX_ARTIFACT)
        terminal['artifacts']['report.html'] = {k: ref[k] for k in ('bytes', 'sha256')}
        save(self.terminal_path, terminal); self.rebind()
        admitted = p.preflight_invocation(a.reference(self.wrapper_path))
        self.assertEqual(admitted['invocation']['phase'], 'after')
        with patch.object(p, 'MAX_ARTIFACT', 64 * 1024 * 1024):
            self.assert_before_output_parse(self.verify)

    def test_fifo_and_directory_rejected_without_blocking(self):
        import os
        path = self.output / 'report.json'; path.unlink(); os.mkfifo(path)
        self.assert_before_output_parse(self.verify)
        path.unlink(); path.mkdir()
        self.assert_before_output_parse(self.verify)

    def test_metadata_budget_and_noncanonical_paths_rejected(self):
        path = self.output.parent / 'large-metadata.json'
        path.write_bytes(b'X' * 1025)
        with self.assertRaises(ValueError): p.checked_file(path, limit=1024, collect=True)
        for name in ('/synthetic/../escape', '/synthetic//double', '/synthetic/./dot'):
            with self.assertRaises(ValueError): p.checked_file(name)

    def test_bound_stdout_stderr_and_unexpected_nested_directory_rejected(self):
        terminal = a.load(self.terminal_path)
        for name in ('stdout.log','stderr.log'):
            (self.output/name).write_bytes(b'synthetic log')
            ref = p.checked_file(self.output/name)
            terminal['artifacts'][name] = {key:ref[key] for key in ('bytes','sha256')}
        save(self.terminal_path,terminal); self.rebind()
        p.preflight_invocation(a.reference(self.wrapper_path))
        (self.output/'stdout.log').write_bytes(b'changed')
        self.assert_before_output_parse(self.verify)
        (self.output/'stdout.log').write_bytes(b'synthetic log')
        (self.output/'unexpected-directory').mkdir()
        self.assert_before_output_parse(self.verify)

    def test_source_and_binary_hash_drift_rejected_before_output_parse(self):
        self.fixture.source.write_text('synthetic change')
        manifest = pathlib.Path(self.wrapper['selectedEntries']['r']['inputManifest'])
        manifest.write_text('[]')
        self.assert_before_output_parse(self.verify)


if __name__ == '__main__':
    unittest.main()
