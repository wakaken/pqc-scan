import importlib.util
import io
import gzip
import argparse
import json
from unittest.mock import patch
import pathlib
import tempfile
import unittest
import tarfile

SCRIPT = pathlib.Path(__file__).resolve().parents[2] / 'scripts/package-release.py'

class PackageTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        spec = importlib.util.spec_from_file_location('package_release', SCRIPT)
        cls.mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.mod)

    def test_installed_scan_requires_all_reviewed_rules_and_clean_negative_control(self):
        ids = ['TLS_RSA_KEY_EXCHANGE', 'TLS_RSA_CIPHER_SUITE', 'TS_TLS_RSA_CIPHER_LITERAL']
        def report():
            return {'summary': {'scanned_files': 2}, 'findings': [
                {'rule_id': rule, 'location': {'file': 'example.py', 'line': 1}}
                for rule in ids]}
        self.mod.validate_installed_scan(report())
        for mutation in ['missing', 'duplicate', 'extra', 'negative', 'line', 'unread']:
            value = report()
            if mutation == 'missing': value['findings'].pop()
            elif mutation == 'duplicate': value['findings'].append(value['findings'][0])
            elif mutation == 'extra': value['findings'][0]['rule_id'] = 'UNREVIEWED'
            elif mutation == 'negative': value['findings'][0]['location']['file'] = 'plain.py'
            elif mutation == 'line': value['findings'][0]['location']['line'] = 2
            else: value['summary']['scanned_files'] = 1
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                self.mod.validate_installed_scan(value)

    def test_installed_surface_rejects_missing_and_extra_protocols(self):
        top = "Commands:\n  scan  Scan\n  probe  Probe\n  help  Help\n"
        both = "Commands:\n  tls  TLS\n  ssh  SSH\n  help  Help\n"
        all_commands = both + "  ssh-kex  KEX\n"
        self.mod.validate_command_surface('static', "Commands:\n  scan  Scan\n", '')
        self.mod.validate_command_surface('combined-probes', top, both)
        self.mod.validate_command_surface('all-probes', top, all_commands)
        for variant, main, sub in [('all-probes', top, both),
                                   ('combined-probes', top, all_commands),
                                   ('static', top, both),
                                   ('all-probes', 'Commands:\n  scan  Scan\n', all_commands)]:
            with self.subTest(variant=variant, sub=sub), self.assertRaises(ValueError):
                self.mod.validate_command_surface(variant, main, sub)

    def test_all_probes_binds_features_metadata_and_dependency_notices(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            required = ['LICENSE', 'SECURITY.md', 'docs/user/DISTRIBUTION.md',
                        'docs/user/DISTRIBUTION.ja.md', 'docs/user/PROBING.md',
                        'docs/user/PROBING.ja.md', 'Cargo.lock', 'rust-toolchain.toml',
                        'third_party/notices/manifest.json', 'third_party/notices/README.md']
            for name in required:
                path = root/name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('fixture')
            (root/'rules/default').mkdir(parents=True)
            binary = root/'binary'
            binary.write_bytes(bytes.fromhex('cffaedfe0c000001') + b'fixture')
            dependency = root/'dependency'
            dependency.mkdir()
            (dependency/'LICENSE').write_text('Synthetic dependency license fixture')
            packages = [dict(id='cli', name='pqc_scan_cli', version='0.1.0', source=None,
                             license='MIT', manifest_path=str(root/'Cargo.toml')),
                        dict(id='negotiation', name='negotiation', version='1.0.0', source='fixture',
                             license='MIT', manifest_path=str(dependency/'Cargo.toml'))]
            nodes = [dict(id='cli', deps=[dict(pkg='negotiation', dep_kinds=[dict(kind=None)])]),
                     dict(id='negotiation', deps=[])]
            calls = []
            def run(directory, *args):
                calls.append(args)
                if args[:2] == ('cargo', 'metadata'):
                    return json.dumps(dict(packages=packages, resolve=dict(nodes=nodes)))
                return 'fixture'
            with patch.object(self.mod, 'run', side_effect=run), patch.object(self.mod.subprocess, 'check_output', return_value=b''):
                self.mod.prepare(argparse.Namespace(source_root=root, binary=binary,
                    target='aarch64-apple-darwin', variant='all-probes', out_dir=root/'output'))
            metadata, = [c for c in calls if c[:2] == ('cargo', 'metadata')]
            self.assertEqual(metadata[metadata.index('--features') + 1],
                             'pqc_scan_cli/probe-tls,pqc_scan_cli/probe-ssh,pqc_scan_cli/probe-ssh-kex')
            archive, = (root/'output').glob('*.tar.gz')
            self.mod.verify_archive(archive)
            with tarfile.open(archive, 'r:gz') as tar:
                prefix = archive.name.removesuffix('.tar.gz')
                provenance = json.load(tar.extractfile(prefix+'/PROVENANCE.json'))
                notices = json.load(tar.extractfile(prefix+'/DEPENDENCY-LICENSES.json'))
            self.assertEqual(provenance['variant'], 'all-probes')
            self.assertEqual(provenance['features'], ['probe-tls', 'probe-ssh', 'probe-ssh-kex'])
            self.assertIn('negotiation', [p['name'] for p in notices['packages']])
            self.assertTrue(all(p['notices'] for p in notices['packages']))

    def test_repeatable_and_tamper_detection(self):
        entries = {'bin/pqc-scan': (b'binary', 0o755), 'rules/test.yml': (b'id: test', 0o644)}
        with tempfile.TemporaryDirectory() as tmp:
            a, b = pathlib.Path(tmp)/'a.tar.gz', pathlib.Path(tmp)/'b.tar.gz'
            self.mod.write_archive(a, 'bundle', entries)
            self.mod.write_archive(b, 'bundle', entries)
            self.assertEqual(a.read_bytes(), b.read_bytes())
            self.mod.verify_archive(a)
            with tarfile.open(a, 'r:gz') as archive:
                values = {m.name: (archive.extractfile(m).read(), m.mode) for m in archive.getmembers()}
            values['bundle/bin/pqc-scan'] = (b'tampered', 0o755)
            self.mod.write_archive(b, '', values, manifest=False)
            with self.assertRaises(ValueError):
                self.mod.verify_archive(b)

    def test_unsafe_archive_names_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            for name in ('../secret', '/secret', 'safe/../../secret', 'x\nother', 'a//b', 'a/./b', '.'):
                with self.assertRaises(ValueError):
                    self.mod.write_archive(pathlib.Path(tmp)/'x.tar.gz', 'bundle', {name: (b'x', 0o644)})

    def test_streaming_limits_and_unsupported_metadata(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = pathlib.Path(tmp)/'bad.tar.gz'
            with gzip.open(path, 'wb') as compressed:
                with tarfile.open(fileobj=compressed, mode='w', format=tarfile.PAX_FORMAT) as archive:
                    info = tarfile.TarInfo('x' * 2000)
                    archive.addfile(info, io.BytesIO())
            with self.assertRaisesRegex(ValueError, 'unsupported archive format'):
                self.mod.verify_archive(path)
            self.mod.write_archive(path, 'bundle', {'x': (b'x' * 2048, 0o644)})
            with patch.object(self.mod, 'MAX_MEMBERS', 1):
                with self.assertRaisesRegex(ValueError, 'member count'):
                    self.mod.verify_archive(path)
            with patch.object(self.mod, 'MAX_DECODED_BYTES', 1024):
                with self.assertRaisesRegex(ValueError, 'resource limit'):
                    self.mod.verify_archive(path)

    def test_duplicate_and_nonregular_members(self):
        with tempfile.TemporaryDirectory() as tmp:
            for mode in ['duplicate', 'symlink']:
                path = pathlib.Path(tmp)/'bad.tar.gz'
                with tarfile.open(path, 'w:gz', format=tarfile.USTAR_FORMAT) as archive:
                    info = tarfile.TarInfo('bundle/x')
                    archive.addfile(info, io.BytesIO())
                    if mode == 'symlink':
                        info = tarfile.TarInfo('bundle/link')
                        info.type = tarfile.SYMTYPE
                        info.linkname = '/outside'
                    archive.addfile(info, io.BytesIO())
                with self.assertRaises(ValueError):
                    self.mod.verify_archive(path)

    def test_license_copy_rejects_escape_and_ignores_nonlicenses(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)/'crate'
            root.mkdir()
            (root/'LICENSE').write_text('license')
            (root/'.env').write_text('secret')
            outside = pathlib.Path(tmp)/'outside'
            outside.write_text('secret')
            (root/'NOTICE').symlink_to(outside)
            with self.assertRaises(ValueError):
                self.mod.license_files(root)
            (root/'NOTICE').unlink()
            self.assertEqual([p.name for p in self.mod.license_files(root)], ['LICENSE'])

if __name__ == '__main__':
    unittest.main()
