"""Synthetic RFC7518 oth container privacy controls."""
import importlib.util
import json
import pathlib
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('oth_display', ROOT / 'benchmark/accuracy_display.py')
display = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(display)


class OtherPrimePrivacyTests(unittest.TestCase):
    def test_short_other_prime_values_masked_both_opaque_modes(self):
        text = '先頭🙂\r\n' + json.dumps(dict(kty='RSA', n='PUBLIC_MODULUS', e='AQAB', oth=[dict(r='SYNTH_R', t='SYNTH_T')]), ensure_ascii=False) + '\r\n'
        for opaque in (False, True):
            safe, changed = display.mask_text(text, opaque=opaque)
            self.assertTrue(changed)
            self.assertNotIn('SYNTH_R', safe)
            self.assertNotIn('SYNTH_T', safe)
            self.assertIn('PUBLIC_MODULUS', safe)
            self.assertIn('AQAB', safe)
            self.assertEqual(len(safe), len(text))
            self.assertEqual([i for i,c in enumerate(text) if c in '\r\n'], [i for i,c in enumerate(safe) if c in '\r\n'])
            self.assertEqual(display.mask_text(safe, opaque=opaque), (safe, False))

    def test_public_r_t_names_preserved_and_safe_native_withholds_container(self):
        finding = dict(finding_id='synthetic', rule_id='synthetic-rule', location=dict(file='s.js', line=1, column=1),
                       evidence=dict(metadata=dict(kty='RSA', n='PUBLIC_MODULUS', e='AQAB', r='PUBLIC_R', t='PUBLIC_T',
                                                   oth=[dict(r='SYNTH_R', t='SYNTH_T')], note='safe')))
        safe = display.safe_native(finding)
        self.assertEqual(safe['finding_id'], finding['finding_id'])
        self.assertEqual(safe['location'], finding['location'])
        self.assertEqual(safe['evidence']['metadata']['r'], 'PUBLIC_R')
        self.assertEqual(safe['evidence']['metadata']['t'], 'PUBLIC_T')
        self.assertNotIn('SYNTH_R', json.dumps(safe))
        self.assertNotIn('SYNTH_T', json.dumps(safe))
        self.assertEqual(display.safe_native(safe), safe)

    def test_nested_and_escaped_containers_do_not_mask_public_jwk(self):
        public = dict(kty='RSA', n='PUBLIC_MODULUS', e='AQAB', kid='public-id')
        for opaque in (False, True):
            text = json.dumps(public)
            self.assertEqual(display.mask_text(text, opaque=opaque), (text, False))
            private = json.dumps(json.dumps(dict(public, oth=[dict(r='SYNTH_R', t='SYNTH_T')])))
            safe, changed = display.mask_text(private, opaque=opaque)
            self.assertTrue(changed)
            self.assertNotIn('SYNTH_R', safe)
            self.assertNotIn('SYNTH_T', safe)
            self.assertEqual(display.mask_text(safe, opaque=opaque), (safe, False))


if __name__ == '__main__':
    unittest.main()
