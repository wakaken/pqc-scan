import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import tarfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, ROOT/'scripts'/filename)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


review = load('dependency_review', 'review-dependencies.py')
packaging = load('package_release', 'package-release.py')
runner = load('dependency_runner', 'run-dependency-review.py')


class DependencyReviewTests(unittest.TestCase):
    def audit(self, vulnerabilities=(), warnings=None):
        return {'settings': {'ignore': []},
                'vulnerabilities': {'list': list(vulnerabilities), 'count': len(vulnerabilities),
                                    'found': bool(vulnerabilities)}, 'warnings': warnings or {}}

    def finding(self, version='1.2.3', source=review.REGISTRY):
        return {'advisory': {'id': 'RUSTSEC-2026-0001', 'package': 'example', 'title': 'Synthetic advisory'},
                'package': {'name': 'example', 'version': version, 'source': source},
                'versions': {'patched': ['>=1.2.4'], 'unaffected': []}, 'affected': None}

    def test_clean_and_vulnerable_audits_keep_actions_and_no_exception(self):
        self.assertEqual(review.audit_findings(self.audit(), 0), [])
        finding = review.audit_findings(self.audit([self.finding()]), 1)[0]
        self.assertEqual(finding['versions']['patched'], ['>=1.2.4'])
        self.assertFalse(finding['exception']['accepted'])
        self.assertIsNone(finding['exception']['expires_at'])
        self.assertIn('upgrade/replacement', finding['action'])

    def test_maintenance_and_unsound_warnings_are_not_dropped(self):
        result = review.audit_findings(self.audit(warnings={'unmaintained': [self.finding()],
                                                           'unsound': [self.finding('2.0.0')]}), 0)
        self.assertEqual([f['kind'] for f in result], ['unmaintained', 'unsound'])
        self.assertEqual([f['package']['version'] for f in result], ['1.2.3', '2.0.0'])

    def test_errors_filtered_and_inconsistent_audits_are_rejected(self):
        for mutation in ['tool-error', 'false-clean', 'wrong-count', 'ignored', 'malformed', 'foreign-source']:
            with self.subTest(mutation=mutation):
                data = self.audit(); code = 0
                if mutation == 'tool-error': code = 2
                elif mutation == 'false-clean': code = 1
                elif mutation == 'wrong-count': data['vulnerabilities']['count'] = 1
                elif mutation == 'ignored': data['settings']['ignore'] = ['RUSTSEC-2026-0001']
                elif mutation == 'malformed': del data['vulnerabilities']['list']
                else: data = self.audit([self.finding(source='git+https://example.invalid')]); code = 1
                with self.assertRaises((ValueError, KeyError)):
                    review.audit_findings(data, code)
        with self.assertRaises(ValueError):
            review.audit_findings(self.audit([self.finding()]), 0)

    def test_closure_includes_build_shared_cycles_and_excludes_dev_only(self):
        names = ['pqc_scan_cli', 'normal', 'build', 'dev', 'shared']
        packages = [{'id': name, 'name': name} for name in names]
        def dep(name, *kinds):
            return {'pkg': name, 'dep_kinds': [{'kind': k} for k in kinds]}
        nodes = [{'id': names[0], 'deps': [dep('normal', None), dep('build', 'build'), dep('dev', 'dev')]},
                 {'id': 'normal', 'deps': [dep('shared', 'dev', None)]},
                 {'id': 'build', 'deps': [dep('shared', 'build')]},
                 {'id': 'dev', 'deps': []},
                 {'id': 'shared', 'deps': [dep('normal', None)]}]
        result = review.closure({'packages': packages, 'resolve': {'nodes': nodes}})
        self.assertEqual({p['id'] for p, _ in result}, {names[0], 'normal', 'build', 'shared'})
        with self.assertRaises(ValueError):
            review.closure({'packages': [], 'resolve': {'nodes': []}})

    def test_identity_does_not_join_other_version_or_source(self):
        original = self.finding()['package']
        self.assertNotEqual(review.identity(original), review.identity(self.finding('1.2.4')['package']))
        self.assertNotEqual(review.identity(original), review.identity(self.finding(source=None)['package']))

    def fixture(self, root):
        crate = root/'registry/src/index/example-1.2.3'; crate.mkdir(parents=True)
        (root/'LICENSE').write_text('Synthetic workspace license')
        data = b'Synthetic registry license\n'
        (crate/'LICENSE').write_bytes(data)
        (crate/'NOTICE').write_bytes(b'Synthetic attribution notice\n')
        (crate/'Cargo.toml').write_text('Synthetic registry manifest')
        archive = root/'registry/cache/index/example-1.2.3.crate'
        archive.parent.mkdir(parents=True)
        with tarfile.open(archive, 'w:gz') as stream:
            for name in ['LICENSE', 'NOTICE', 'Cargo.toml']:
                content = (crate/name).read_bytes(); member = tarfile.TarInfo('example-1.2.3/'+name)
                member.size = len(content); stream.addfile(member, io.BytesIO(content))
        package = {'name': 'example', 'version': '1.2.3', 'source': review.REGISTRY,
                   'license': 'MIT OR Apache-2.0', 'manifest_path': str(crate/'Cargo.toml')}
        lock = {'package': [dict(package, checksum=hashlib.sha256(archive.read_bytes()).hexdigest())]}
        output = root/'output'; (output/'notices').mkdir(parents=True)
        return package, lock, output, crate

    def test_exact_notice_bytes_features_and_registry_binding(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); package, lock, output, _ = self.fixture(root)
            result = review.package_record(root, package, {'features': ['b', 'a']}, lock, packaging, output)
            self.assertEqual(result['features'], ['a', 'b'])
            self.assertEqual(result['registry_checksum'], lock['package'][0]['checksum'])
            self.assertEqual(result['license_expression'], 'MIT OR Apache-2.0')
            self.assertFalse(result['notice_review_required'])
            self.assertEqual((output/result['notices'][0]['path']).read_bytes(), b'Synthetic registry license\n')

    def test_tampered_missing_license_source_and_checksum_fail_closed(self):
        for mutation in ['notice', 'checksum', 'license', 'lock', 'source', 'missing', 'partial-notice', 'archive', 'manifest']:
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory() as temp:
                root = Path(temp); package, lock, output, crate = self.fixture(root)
                if mutation == 'notice': (crate/'LICENSE').write_text('tampered')
                elif mutation == 'checksum': lock['package'][0]['checksum'] = 'c'*64
                elif mutation == 'license': package['license'] = None
                elif mutation == 'lock': lock['package'] = []
                elif mutation == 'source':
                    package['source'] = 'git+https://example.invalid'; lock['package'][0]['source'] = package['source']
                elif mutation == 'partial-notice': (crate/'NOTICE').unlink()
                elif mutation == 'manifest': (crate/'Cargo.toml').write_text('tampered manifest')
                elif mutation == 'archive': (root/'registry/cache/index/example-1.2.3.crate').write_bytes(b'tampered archive')
                else: (crate/'LICENSE').unlink()
                with self.assertRaises((ValueError, FileNotFoundError)):
                    review.package_record(root, package, {'features': []}, lock, packaging, output)

    def test_license_mapping_retains_open_decisions_and_per_variant_counts(self):
        package = {'name': 'example', 'version': '1.2.3', 'source': review.REGISTRY,
                   'license_expression': 'MIT', 'notice_review_required': True,
                   'notices': [{'sha256': 'a'*64, 'path': 'notices/a.txt', 'review_required': True,
                                'review_note': 'Synthetic unresolved publication condition.'}]}
        variants = {name: {'packages': [] if name == 'static' else [copy.deepcopy(package)]}
                    for name in review.VARIANTS}
        text = review.license_markdown({'source_head': 'b'*40, 'variants': variants})
        self.assertIn('| static | 0 | 0 |', text)
        self.assertIn('| network | 1 | 1 |', text)
        self.assertIn('No unresolved issue is accepted', text)
        self.assertIn('Synthetic unresolved publication condition.', text)
        self.assertIn('(notices/a.txt)', text)

    def test_database_binds_content_commit_and_retrieval_receipt(self):
        for mutation in [None, 'bytes', 'commit', 'timestamp', 'manifest']:
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory() as temp:
                root = Path(temp); db = root/'advisory-db'; db.mkdir()
                (db/'README.md').write_text('Synthetic database')
                files = {'README.md': hashlib.sha256(b'Synthetic database').hexdigest()}
                digest = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
                record = {'files': files, 'files_sha256': digest, 'commit': 'a'*40,
                          'retrieved_at': '2026-11-14T00:00:00+00:00'}
                if mutation == 'bytes': (db/'README.md').write_text('tampered')
                elif mutation == 'commit': record['commit'] = 'b'*40
                elif mutation == 'timestamp': record['retrieved_at'] = ''
                elif mutation == 'manifest': record['files_sha256'] = 'c'*64
                (root/'database.json').write_text(json.dumps(record))
                with patch.object(review.subprocess, 'check_output', return_value='a'*40+'\n'):
                    if mutation:
                        with self.assertRaises(ValueError): review.database_binding(root)
                    else:
                        self.assertEqual(review.database_binding(root), record)

    def test_dirty_metadata_does_not_override_a_completed_source_comparison(self):
        package = {'name': 'reviewed', 'version': '1.2.3', 'source': review.REGISTRY,
                   'license_expression': 'MIT', 'notice_review_required': False,
                   'vcs': {'git': {'dirty': True}},
                   'notices': [{'sha256': 'a'*64, 'path': 'notices/a.txt', 'review_required': False}]}
        text = review.license_markdown({'source_head': 'b'*40,
                                       'variants': {'network': {'packages': [package]}}})
        self.assertIn('reviewed 1.2.3', text)
        self.assertIn('dirty=true is preserved', text)
        self.assertNotIn('upstream differences not cleared', text)

    def test_runner_refuses_mutable_images_and_overwriting_evidence_before_docker(self):
        with tempfile.TemporaryDirectory() as temp:
            for image, expected in [('latest', SystemExit), ('sha256:'+'a'*64, FileExistsError)]:
                argv = ['runner', '--image', image, '--preparation', temp, '--out-dir', temp]
                with self.subTest(image=image), patch('sys.argv', argv), patch.object(runner.subprocess, 'run') as docker:
                    with self.assertRaises(expected): runner.main()
                    docker.assert_not_called()


if __name__ == '__main__':
    unittest.main()
