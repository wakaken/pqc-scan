import importlib.util
from pathlib import Path
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('temporal_ci', ROOT / 'benchmark/temporal_reassessment/ci.py')
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)


class RuntimeArtifactTests(unittest.TestCase):
    def dependencies(self):
        return {'serde_json': 'serde-json-id', 'pqc_scan_core': 'core-id',
                'pqc_scan_rules': 'rules-id', 'pqc_scan_detectors': 'detectors-id'}

    def metadata(self):
        return {'packages': [{'id': identity, 'name': name,
                              'version': '1.0.149' if name == 'serde_json' else '0.5.0'}
                             for name, identity in self.dependencies().items()]}

    def test_runtime_features_exclude_procedural_macro_variants(self):
        output = (b'pqc_scan_core v0.5.0|default\npqc_scan_rules v0.5.0|\n'
                  b'pqc_scan_detectors v0.5.0|\n'
                  b'serde_json v1.0.149|default,std\nserde_json v1.0.149|default,std (*)\n')
        with mock.patch.object(ci, 'invoke', return_value=output) as invoke:
            features = ci.runtime_features(self.metadata(), self.dependencies())
        command = invoke.call_args.args[0]
        self.assertEqual(command[command.index('--edges') + 1], 'normal,no-proc-macro')
        self.assertIn('--locked', command)
        self.assertIn('--offline', command)
        rows = [self.row(['default', 'std', 'preserve_order'], 'host.rlib'),
                self.row(['default', 'std'], 'runtime.rlib')]
        self.assertEqual(ci.select_artifacts(rows, {'serde_json': 'serde-json-id'},
                         {'serde_json': features['serde_json']}),
                         {'serde_json': 'runtime.rlib'})

    def test_conflicting_remaining_runtime_features_reject(self):
        output = b'serde_json v1.0.149|default,std\nserde_json v1.0.149|preserve_order,std\n'
        with mock.patch.object(ci, 'invoke', return_value=output):
            with self.assertRaisesRegex(ValueError, 'E_CI_RUNTIME_FEATURE_AMBIGUOUS:serde_json'):
                ci.runtime_features(self.metadata(), self.dependencies())

    def test_missing_runtime_features_reject(self):
        with mock.patch.object(ci, 'invoke', return_value=b''):
            with self.assertRaisesRegex(ValueError, 'E_CI_RUNTIME_FEATURE_MISSING'):
                ci.runtime_features(self.metadata(), self.dependencies())

    def test_ambiguous_runtime_package_identity_rejects_before_tree(self):
        metadata = self.metadata()
        metadata['packages'].append({'id': 'other-source-id', 'name': 'serde_json',
                                     'version': '1.0.149'})
        with mock.patch.object(ci, 'invoke') as invoke:
            with self.assertRaisesRegex(ValueError, 'E_CI_FEATURE_PACKAGE_AMBIGUOUS'):
                ci.runtime_features(metadata, self.dependencies())
        invoke.assert_not_called()

    def row(self, features, output, identity='serde-json-id', name='serde_json'):
        return dict(reason='compiler-artifact', package_id=identity, target=dict(name=name),
                    features=features, filenames=[output])

    def test_host_feature_variant_does_not_replace_runtime_library(self):
        rows = [self.row(['default', 'std'], 'runtime.rlib'),
                self.row(['default', 'indexmap', 'preserve_order', 'std'], 'host.rlib')]
        self.assertEqual(ci.select_artifacts(rows, {'serde_json': 'serde-json-id'},
                         {'serde_json': frozenset(['default', 'std'])}), {'serde_json': 'runtime.rlib'})

    def test_missing_runtime_feature_variant_rejects(self):
        with self.assertRaisesRegex(ValueError, 'E_CI_ARTIFACT_MISSING'):
            ci.select_artifacts([self.row(['preserve_order'], 'host.rlib')],
                                {'serde_json': 'serde-json-id'}, {'serde_json': frozenset(['std'])})

    def test_duplicate_matching_runtime_artifacts_reject(self):
        with self.assertRaisesRegex(ValueError, 'E_CI_ARTIFACT_AMBIGUOUS'):
            ci.select_artifacts([self.row(['std'], 'a.rlib'), self.row(['std'], 'b.rlib')],
                                {'serde_json': 'serde-json-id'}, {'serde_json': frozenset(['std'])})

    def test_unrelated_build_artifact_duplicates_are_ignored(self):
        rows = [self.row(['std'], 'runtime.rlib'), self.row([], 'host.rlib', 'host-id', 'host'),
                self.row([], 'other-host.rlib', 'host-id', 'host')]
        self.assertEqual(ci.select_artifacts(rows, {'serde_json': 'serde-json-id'},
                         {'serde_json': frozenset(['std'])}), {'serde_json': 'runtime.rlib'})


if __name__ == '__main__':
    unittest.main()
