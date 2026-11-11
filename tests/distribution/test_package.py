import importlib.util
import io
import gzip
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
