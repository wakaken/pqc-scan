"""Independent negative controls for the TASK-048 evidence collector."""
import json
from pathlib import Path
import tempfile
import unittest
from task048.contracts import inspect_html, verify_pins, check_projection


class Contracts048Tests(unittest.TestCase):
    def test_missing_or_changed_projected_provenance_and_loss_are_rejected(self):
        import copy
        native = {'assets': [{'id':'alg','identity_basis':'named','uncertainty':['parameters_unknown']}],
                  'observations':[{'id':'obs','asset_ids':['alg'],'method':'lexical_match','assurance':'lexical','usage':'unknown',
                                   'target':{'path':'a.py','line':3,'column':2,'column_kind':'unicode_code_points'}}]}
        observation = dict(id='obs',method='lexical_match',assurance='lexical',usage='unknown',path='a.py',line=3,column=2,column_kind='unicode_code_points')
        doc = {'properties':[{'name':'pqc-scan:export-loss','value':json.dumps({'counts':{'unprojected_observations':1},'notes':['observations_are_properties_not_complete_native_records']})}],
               'components':[{'bom-ref':'alg','properties':[{'name':'pqc-scan:observations','value':json.dumps([observation])},{'name':'pqc-scan:identity-basis','value':'named'},{'name':'pqc-scan:uncertainty','value':'parameters_unknown'}]}]}
        check_projection(native,doc,{'unprojected_observations':1})
        for mutation in ['observations','loss','usage','column','uncertainty']:
            broken=copy.deepcopy(doc)
            props=broken['components'][0]['properties']
            if mutation=='observations': props[0]['value']='[]'
            elif mutation=='loss': broken['properties'][0]['value']='{}'
            elif mutation=='uncertainty': props[2]['value']=''
            else:
                row=copy.deepcopy(observation); row[mutation]='verified' if mutation=='usage' else 99
                props[0]['value']=json.dumps([row])
            with self.subTest(mutation=mutation), self.assertRaises(AssertionError):
                check_projection(native,broken,{'unprojected_observations':1})

    def test_active_html_and_uri_are_rejected_but_code_text_survives(self):
        self.assertEqual(inspect_html('<pre>&lt;script&gt;x&lt;/script&gt;</pre>', set()), [])
        for html in ['<script>x</script>', '<img src=x onerror="x">',
                     '<a href="java&#x73;cript:x">x</a>', '<svg onload="x">',
                     '<iframe srcdoc="x"></iframe>']:
            with self.subTest(html=html):
                self.assertTrue(inspect_html(html, set()))

    def test_pinned_schema_or_license_tampering_is_rejected(self):
        import hashlib
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root/'schema').write_bytes(b'{}')
            (root/'LICENSE').write_bytes(b'MIT')
            pins = {'files': [dict(path='schema', sha256=hashlib.sha256(b'{}').hexdigest(), license='LICENSE')]}
            self.assertEqual(len(verify_pins(root, pins)), 1)
            (root/'schema').write_bytes(b'[]')
            with self.assertRaisesRegex(ValueError, 'digest'):
                verify_pins(root, pins)
            (root/'schema').write_bytes(b'{}')
            (root/'LICENSE').unlink()
            with self.assertRaisesRegex(ValueError, 'license'):
                verify_pins(root, pins)


if __name__ == '__main__': unittest.main()
