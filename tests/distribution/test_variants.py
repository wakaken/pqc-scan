"""Cross-check shipped configurations and the executable CI command contract."""
import importlib.util
from pathlib import Path
import shlex
import unittest

ROOT = Path(__file__).resolve().parents[2]


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, ROOT/'scripts'/filename)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


class VariantContractTests(unittest.TestCase):
    def test_package_and_assessment_match_the_two_distribution_requirements(self):
        package = load('package_variants', 'package-release.py')
        review = load('review_variants', 'review-dependencies.py')
        expected = {'static': [], 'network': ['probe-tls', 'probe-ssh', 'probe-ssh-kex']}
        self.assertEqual(package.VARIANT_FEATURES, expected)
        self.assertEqual(review.VARIANTS, expected)

    def test_ci_builds_packages_and_installs_exactly_the_two_configurations(self):
        text = (ROOT/'.github/workflows/release.yml').read_text()
        commands = [shlex.split(line.strip()) for line in text.splitlines()
                    if line.strip().startswith(('cargo build ', 'python3 scripts/package-release.py ',
                                                'python3 tests/distribution/check-install.py '))]
        builds = [c for c in commands if c[:2] == ['cargo', 'build']]
        packages = [c for c in commands if c[:2] == ['python3', 'scripts/package-release.py']]
        installs = [c for c in commands if c[:2] == ['python3', 'tests/distribution/check-install.py']]
        self.assertEqual(len(builds), 2)
        self.assertEqual(len(packages), 4)
        self.assertEqual(len(installs), 2)
        self.assertNotIn('--features', builds[0])
        self.assertEqual(builds[1][builds[1].index('--features')+1], 'probe-tls,probe-ssh,probe-ssh-kex')
        for variant in ['static', 'network']:
            selected = [c for c in packages if c[c.index('--variant')+1] == variant]
            self.assertEqual(len(selected), 2)
            destinations = {c[c.index('--out-dir')+1] for c in selected}
            self.assertEqual(destinations, {f'target/task-038/{variant}-first', f'target/task-038/{variant}-second'})
            self.assertEqual(sum(c[2] == f'target/task-038/{variant}-first/*.tar.gz' for c in installs), 1)
            binary = 'target/release/pqc-scan' if variant == 'static' else 'target/network-release-build/release/pqc-scan'
            self.assertTrue(all(c[c.index('--binary')+1] == binary for c in selected))
        self.assertIn('cmp target/task-038/network-first/*.tar.gz target/task-038/network-second/*.tar.gz', text)
        self.assertNotIn('combined-probes', text)
        self.assertNotIn('all-probes', text)
        self.assertIn('workflow_dispatch:', text)
        self.assertNotIn('gh release ', text)
        self.assertNotIn('git push', text)
        self.assertEqual(text.count('subject-path:'), 2)
        for variant in ['static', 'network']:
            self.assertIn(f'subject-path: target/task-038/{variant}-first/*.tar.gz', text)
            self.assertIn(f'ATTESTATION_BUNDLE: ${{{{ steps.{variant}-provenance.outputs.bundle-path }}}}', text)
            self.assertIn(f'cp "$ATTESTATION_BUNDLE" target/task-038/{variant}-first/build-provenance.jsonl', text)
        self.assertEqual(text.count('--source-digest "$SOURCE_COMMIT"'), 2)


if __name__ == '__main__':
    unittest.main()
