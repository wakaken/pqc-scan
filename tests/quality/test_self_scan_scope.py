"""The CI crypto self-scan reads product files, not the repository's test corpus."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

REPO = Path(__file__).resolve().parents[2]


class ProductScopeTests(unittest.TestCase):
    def test_self_scan_uses_only_the_prepared_product_scope_and_archives_its_inventory(self):
        scan = (REPO/'.github/workflows/pqc-scan.yml').read_text().split('  scan:\n', 1)[1]
        self.assertIn('python3 scripts/prepare-self-scan.py --root .', scan)
        self.assertIn('scan "$RUNNER_TEMP/pqc-self-scan/input" --format sarif --diagnostics', scan)
        self.assertNotIn('scan . --format', scan)
        self.assertIn('--root "$RUNNER_TEMP/pqc-self-scan/input"', scan)
        self.assertIn('--expectations ./pqc-report/self-scan-expectations.json', scan)
        self.assertIn('python3 scripts/prepare-self-scan.py --verify --root .', scan)
        self.assertLess(scan.index('name: Verify every declared product input'),
                        scan.index('name: Prepare actionable Code Scanning results'))
        for name in ['self-scan-inputs.json', 'self-scan-expectations.json', 'diagnostics.json', 'self-scan-verification.json']:
            self.assertIn('./pqc-report/'+name, scan)

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='pqc-product-scope-')
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.root = self.base/'repo'
        self.root.mkdir()
        self.kept = {
            'Cargo.toml': '[workspace]\nmembers = ["crates/demo"]\n',
            'Cargo.lock': 'version = 4\n',
            'crates/demo/Cargo.toml': '[package]\nname = "demo"\n',
            'crates/demo/src/lib.rs': '#[cfg(test)] mod tests {}\npub fn run() {}\n',
            'crates/demo/src/view.css': 'body { color: black; }\n',
            'crates/demo/build.rs': 'fn main() {}\n',
            'rules/default/demo.yml': '[]\n',
        }
        self.excluded = ['tests/positive.rs', 'examples/main.rs', 'tasks/054/evidence/result.json',
                         'crates/demo/tests/integration.rs', 'crates/demo/benches/bench.rs',
                         'scripts/controller.py', '.github/workflows/ci.yml', 'README.md']
        for name, text in self.kept.items():
            self.put(name, text)
        for name in self.excluded:
            self.put(name, 'NOT_A_PRODUCT_SOURCE\n')
        self.put('policy.json', json.dumps({'version': 1, 'files': []}))
        spec = importlib.util.spec_from_file_location('product_scope', REPO/'scripts/prepare-self-scan.py')
        self.module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.module)

    def put(self, name, text):
        file = self.root/name
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text(text)

    def prepare(self, out=None):
        return self.module.prepare(self.root, out or self.base/'bundle', 'policy.json')

    def diagnostics(self, changes=None):
        coverage = dict.fromkeys(['read_failed_files', 'partial_files', 'analysis_failed_files',
                                  'unsupported_files', 'not_applicable_files', 'unknown_files',
                                  'excluded_enumerated_files', 'enumeration_failed_entries'], 0)
        coverage.update(selected_files=len(self.kept), read_ok_files=len(self.kept),
                        analyzed_files=len(self.kept), status='complete_within_scope')
        coverage.update(changes or {})
        file = self.base/'diagnostics.json'
        file.write_text(json.dumps({'schema_version': '1', 'coverage': coverage}))
        return file

    def test_runtime_verification_binds_inventory_copied_bytes_and_read_counts(self):
        self.prepare()
        result = self.module.verify(self.root, self.base/'bundle', self.diagnostics())
        self.assertEqual(result['verifiedFiles'], len(self.kept))

    @unittest.skipUnless(os.environ.get('PQC_SELF_SCAN_BINARY'), 'requires built scanner')
    def test_actual_scanner_ignore_cannot_pass_inventory_verification(self):
        self.put('crates/demo/src/.ignore', 'hidden.rs\n')
        self.put('crates/demo/src/hidden.rs', 'pub fn hidden() {}\n')
        manifest = self.prepare()
        output = self.base/'report'
        run = subprocess.run([os.environ['PQC_SELF_SCAN_BINARY'], 'scan',
                              str(self.base/'bundle/input'), '--format', 'sarif', '--diagnostics',
                              '--out-dir', str(output), '--rules-dir', str(REPO/'rules')],
                             capture_output=True, timeout=30)
        self.assertEqual(run.returncode, 0, run.stderr.decode(errors='replace'))
        diagnostics = output/'diagnostics.json'
        coverage = json.loads(diagnostics.read_text())['coverage']
        self.assertLess(coverage['selected_files'], manifest['fileCount'])
        with self.assertRaisesRegex(ValueError, 'did not completely read'):
            self.module.verify(self.root, self.base/'bundle', diagnostics)
        self.assertFalse((self.base/'bundle/verification.json').exists())

    def test_ignore_missing_read_failure_or_partial_counters_cannot_pass(self):
        self.prepare()
        for change in [{'selected_files': len(self.kept)-1}, {'read_ok_files': len(self.kept)-1},
                       {'read_failed_files': 1}, {'excluded_enumerated_files': 1},
                       {'enumeration_failed_entries': 1}, {'partial_files': 1},
                       {'status': 'partial'}, {'unknown_files': False}]:
            with self.subTest(change=change), self.assertRaises(ValueError):
                self.module.verify(self.root, self.base/'bundle', self.diagnostics(change))
        self.assertFalse((self.base/'bundle/verification.json').exists())

    def test_extra_or_changed_copied_input_cannot_hide_a_missing_product_file(self):
        self.prepare()
        extra = self.base/'bundle/input/tests/extra.rs'
        extra.parent.mkdir()
        extra.write_text('unexpected')
        with self.assertRaises(ValueError):
            self.module.verify(self.root, self.base/'bundle', self.diagnostics())
        extra.unlink()
        target = self.base/'bundle/input/crates/demo/src/lib.rs'
        target.chmod(0o644)
        target.write_text('changed')
        with self.assertRaises(ValueError):
            self.module.verify(self.root, self.base/'bundle', self.diagnostics())

    def test_empty_forged_manifest_and_malformed_diagnostics_fail(self):
        self.prepare()
        file = self.base/'diagnostics.json'
        file.write_text('null')
        with self.assertRaises(ValueError):
            self.module.verify(self.root, self.base/'bundle', file)
        manifest = self.base/'bundle/manifest.json'
        value = json.loads(manifest.read_text())
        value.update(files=[], fileCount=0, byteCount=0)
        manifest.write_text(json.dumps(value))
        with self.assertRaises(ValueError):
            self.module.verify(self.root, self.base/'bundle', self.diagnostics())

    def test_only_product_sources_manifests_build_scripts_and_rules_are_copied(self):
        manifest = self.prepare()
        output = self.base/'bundle'
        self.assertEqual({item['path'] for item in manifest['files']}, set(self.kept))
        self.assertEqual(manifest['fileCount'], len(self.kept))
        self.assertEqual(manifest['byteCount'], sum(len(x.encode()) for x in self.kept.values()))
        for name, text in self.kept.items():
            self.assertEqual((output/'input'/name).read_bytes(), text.encode())
        for name in self.excluded:
            self.assertFalse((output/'input'/name).exists(), name)
        self.assertFalse((output/'input/manifest.json').exists())
        self.assertFalse((output/'input/expectations.json').exists())

    def test_expected_references_are_intersected_with_selected_paths_without_learning(self):
        entry = {'path': 'crates/demo/src/lib.rs', 'sha256': '0'*64,
                 'reason': 'Existing exact source expectations', 'results': []}
        outside = dict(entry, path='tests/positive.rs')
        before = {'version': 1, 'files': [entry, outside]}
        self.put('policy.json', json.dumps(before))
        self.prepare()
        actual = json.loads((self.base/'bundle/expectations.json').read_text())
        self.assertEqual(actual, {'version': 1, 'files': [entry]})
        self.assertEqual(json.loads((self.root/'policy.json').read_text()), before)
        self.assertEqual(actual['files'][0]['sha256'], '0'*64, 'Never refresh a stale allowance hash')

    def test_untracked_new_product_sources_are_included(self):
        self.put('crates/demo/src/new_module.rs', 'pub fn added() {}\n')
        result = self.prepare()
        self.assertIn('crates/demo/src/new_module.rs', [f['path'] for f in result['files']])

    def test_source_file_and_parent_symlink_aliases_fail_closed(self):
        source = self.root/'crates/demo/src/lib.rs'
        source.unlink()
        source.symlink_to(self.root/'tests/positive.rs')
        with self.assertRaises((ValueError, OSError)):
            self.prepare()

    def test_empty_product_source_cannot_be_silently_accepted(self):
        (self.root/'crates/demo/src/lib.rs').unlink()
        with self.assertRaises((ValueError, OSError)):
            self.prepare()

    def test_parent_directory_alias_is_rejected_before_traversal(self):
        import shutil
        shutil.rmtree(self.root/'crates/demo/src')
        (self.root/'crates/demo/src').symlink_to(self.root/'tests', target_is_directory=True)
        with self.assertRaises(ValueError):
            self.prepare()

    def test_existing_output_or_output_inside_repository_is_not_overwritten(self):
        output = self.base/'bundle'
        output.mkdir()
        (output/'sentinel').write_text('preserve')
        with self.assertRaises((ValueError, OSError)):
            self.prepare(output)
        self.assertEqual((output/'sentinel').read_text(), 'preserve')
        with self.assertRaises(ValueError):
            self.prepare(self.root/'input-copy')

    def test_scan_root_under_default_excluded_directory_is_rejected(self):
        with self.assertRaises(ValueError):
            self.prepare(self.base/'target/product-scan')

    def test_selected_path_that_the_scanner_would_ignore_is_not_declared_complete(self):
        self.put('crates/demo/src/docs/hidden.rs', 'pub fn hidden() {}\n')
        with self.assertRaises(ValueError):
            self.prepare()

    def test_special_or_oversized_sources_fail_without_a_completed_manifest(self):
        fifo = self.root/'crates/demo/src/pipe.rs'
        os.mkfifo(fifo)
        with self.assertRaises(ValueError):
            self.prepare(self.base/'special')
        self.assertFalse((self.base/'special/manifest.json').exists())
        fifo.unlink()
        (self.root/'crates/demo/src/large.rs').write_bytes(b'x'*(self.module.MAX_FILE_BYTES+1))
        with self.assertRaises(ValueError):
            self.prepare(self.base/'large')
        self.assertFalse((self.base/'large/manifest.json').exists())

    def test_source_mutation_during_copy_fails_instead_of_rebinding_expectations(self):
        original = self.module.source.source_bytes
        changed = False

        def read(root, name, **kwargs):
            nonlocal changed
            data = original(root, name, **kwargs)
            if name == 'crates/demo/src/lib.rs' and not changed:
                changed = True
                self.put(name, 'pub fn changed() {}\n')
            return data

        with mock.patch.object(self.module.source, 'source_bytes', side_effect=read):
            with self.assertRaises(ValueError):
                self.prepare()
        self.assertFalse((self.base/'bundle/manifest.json').exists())

    def test_cli_failure_does_not_echo_input_payload(self):
        self.put('policy.json', '{SENSITIVE_CANARY')
        run = subprocess.run([sys.executable, str(REPO/'scripts/prepare-self-scan.py'),
                              '--root', str(self.root), '--out-dir', str(self.base/'bundle'),
                              '--expectations', 'policy.json'], capture_output=True, text=True)
        self.assertNotEqual(run.returncode, 0)
        self.assertNotIn('SENSITIVE_CANARY', run.stderr)
        self.assertIn('preparation failed', run.stderr)

    def test_missing_manifests_empty_product_scope_and_malformed_policy_fail(self):
        (self.root/'Cargo.lock').unlink()
        with self.assertRaises(ValueError):
            self.prepare()
        self.put('Cargo.lock', self.kept['Cargo.lock'])
        self.put('policy.json', '{invalid')
        with self.assertRaises((ValueError, json.JSONDecodeError)):
            self.prepare()
        self.put('policy.json', json.dumps({'version': True, 'files': []}))
        with self.assertRaises(ValueError):
            self.prepare()
        self.put('policy.json', json.dumps({'version': 1, 'files': []}))
        for name in self.kept:
            if '/src/' in name:
                (self.root/name).unlink()
        with self.assertRaises(ValueError):
            self.prepare()


if __name__ == '__main__':
    unittest.main()
