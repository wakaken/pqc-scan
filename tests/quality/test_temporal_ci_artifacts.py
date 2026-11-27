import importlib.util
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('temporal_ci', ROOT / 'benchmark/temporal_reassessment/ci.py')
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)


class RuntimeArtifactTests(unittest.TestCase):
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
