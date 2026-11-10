import importlib.util
import json
import hashlib
from pathlib import Path
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[2]/'scripts/package-release.py'
SOURCE = 'registry+https://github.com/rust-lang/crates.io-index'

class ReviewedNoticeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        spec = importlib.util.spec_from_file_location('package_release', SCRIPT)
        cls.mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.mod)

    def fixture(self, root):
        crate = root/'cache/example';crate.mkdir(parents=True)
        (crate/'Cargo.toml').write_text('[package]\nname="example"\n')
        vcs={'git':{'sha1':'a'*40},'path_in_vcs':'crate'}
        (crate/'.cargo_vcs_info.json').write_text(json.dumps(vcs))
        (root/'Cargo.lock').write_text('version = 4\n[[package]]\nname="example"\nversion="1.2.3"\nsource="'+SOURCE+'"\nchecksum="'+'b'*64+'"\n')
        directory=root/'third_party/notices';(directory/'texts/example').mkdir(parents=True)
        data=b'Synthetic license test fixture, not a third-party license.\n'
        (directory/'texts/example/LICENSE').write_bytes(data)
        package={'name':'example','version':'1.2.3','source':SOURCE,'license':'MIT','repository':'https://github.com/example/example','manifest_path':str(crate/'Cargo.toml')}
        entry={'name':'example','version':'1.2.3','source':SOURCE,'registry_checksum':'b'*64,'license_expression':'MIT','upstream_repository':package['repository'],'upstream_revision':'a'*40,'path_in_vcs':'crate','review_required':True,'review_note':'Statement-only fixture remains review-required.','files':[{'path':'texts/example/LICENSE','sha256':hashlib.sha256(data).hexdigest(),'upstream_url':'https://raw.githubusercontent.com/example/example/'+'a'*40+'/LICENSE','kind':'upstream_license_statement'}]}
        manifest={'version':1,'packages':[entry]}
        (directory/'manifest.json').write_text(json.dumps(manifest))
        return package, manifest, directory, data

    def write_manifest(self,directory,manifest):
        (directory/'manifest.json').write_text(json.dumps(manifest))

    def test_exact_registry_and_revision_binding_retains_review_required(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp);p,m,d,data=self.fixture(root)
            records=self.mod.reviewed_notice_files(root,p)
            self.assertEqual(len(records),1)
            self.assertEqual(records[0]['data'],data)
            self.assertTrue(records[0]['review_required'])
            self.assertEqual(records[0]['kind'],'upstream_license_statement')
            self.assertEqual(records[0]['upstream_revision'],'a'*40)

    def test_tampering_and_stale_package_or_vcs_fail_closed(self):
        for change in ['content','version','license','source','checksum','vcs','repository','url-query','url-traversal','review-flag']:
            with self.subTest(change=change), tempfile.TemporaryDirectory() as tmp:
                root=Path(tmp);p,m,d,data=self.fixture(root)
                if change=='content':(d/'texts/example/LICENSE').write_bytes(b'tampered')
                elif change=='version':p['version']='9.9.9'
                elif change=='license':p['license']='Apache-2.0'
                elif change=='source':p['source']='registry+https://elsewhere.invalid/index'
                elif change=='repository':p['repository']='https://github.com/other/repo'
                elif change=='checksum':m['packages'][0]['registry_checksum']='c'*64;self.write_manifest(d,m)
                elif change=='url-query':m['packages'][0]['files'][0]['upstream_url']+='?token=synthetic-canary';self.write_manifest(d,m)
                elif change=='url-traversal':m['packages'][0]['files'][0]['upstream_url']=m['packages'][0]['files'][0]['upstream_url'].replace('/LICENSE','/../LICENSE');self.write_manifest(d,m)
                elif change=='review-flag':m['packages'][0]['review_required']=False;self.write_manifest(d,m)
                else:(Path(p['manifest_path']).parent/'.cargo_vcs_info.json').write_text(json.dumps({'git':{'sha1':'d'*40},'path_in_vcs':'crate'}))
                with self.assertRaises(ValueError):self.mod.reviewed_notice_files(root,p)

    def test_escape_symlink_nonlicense_and_duplicate_registration_are_rejected(self):
        for change in ['escape','symlink','nonlicense','duplicate']:
            with self.subTest(change=change), tempfile.TemporaryDirectory() as tmp:
                root=Path(tmp);p,m,d,data=self.fixture(root)
                entry=m['packages'][0]
                if change=='escape':entry['files'][0]['path']='../../Cargo.lock'
                elif change=='nonlicense':entry['files'][0]['path']='texts/example/.env';(d/'texts/example/.env').write_bytes(data)
                elif change=='symlink':
                    path=d/'texts/example/LICENSE';path.unlink();outside=root/'outside';outside.write_bytes(data);path.symlink_to(outside)
                else:m['packages'].append(dict(entry))
                self.write_manifest(d,m)
                with self.assertRaises(ValueError):self.mod.reviewed_notice_files(root,p)
