"""Finite synthetic public/private context controls for the display boundary."""
import json
import unittest
import accuracy_display as d


class DisplayContextTests(unittest.TestCase):
    def assert_safe(self, text, hidden, visible):
        for opaque in (False, True):
            with self.subTest(opaque=opaque):
                safe, _ = d.mask_text(text, opaque=opaque)
                for value in hidden:
                    self.assertNotIn(value, safe)
                for value in visible:
                    self.assertIn(value, safe)
                self.assertEqual(len(safe), len(text))
                self.assertEqual([i for i,c in enumerate(text) if c in '\r\n'],
                                 [i for i,c in enumerate(safe) if c in '\r\n'])
                self.assertEqual(d.mask_text(safe, opaque=opaque), (safe, False))

    def test_ordinary_oth_and_sibling_objects_remain_public(self):
        value = dict(kty='RSA', n='PUBLIC_N', e='AQAB', sibling=dict(oth=[dict(r='PUBLIC_R', t='PUBLIC_T')]))
        for text in (json.dumps(value), 'wrap(' + json.dumps(value) + ')', '// ' + json.dumps(value)):
            self.assert_safe(text, [], ['PUBLIC_R', 'PUBLIC_T', 'PUBLIC_N', 'AQAB'])
        plain = dict(oth=[dict(r='PUBLIC_R', t='PUBLIC_T')], r='TOP_R', t='TOP_T')
        self.assertEqual(d.safe_content(plain), plain)
        self.assert_safe(json.dumps(plain), [], ['PUBLIC_R', 'PUBLIC_T', 'TOP_R', 'TOP_T'])

    def test_rsa_oth_field_order_source_wrappers_comments_and_serialization(self):
        for value in (dict(kty='RSA', oth=[dict(r='SYNTH_R', d='SYNTH_D', t='SYNTH_T')], n='PUBLIC_N', e='AQAB'),
                      dict(oth=[dict(r='SYNTH_R', d='SYNTH_D', t='SYNTH_T')], n='PUBLIC_N', e='AQAB', kty='RSA')):
            text = json.dumps(value)
            for wrapped in (text, 'wrap('+text+')', '// '+text, '/* '+text+' */', json.dumps(text), json.dumps(json.dumps(text))):
                self.assert_safe(wrapped, ['SYNTH_R','SYNTH_D','SYNTH_T'], ['PUBLIC_N','AQAB'])
            safe = d.safe_content(value)
            for marker in ('SYNTH_R','SYNTH_D','SYNTH_T'):
                self.assertNotIn(marker, json.dumps(safe))
            self.assertEqual(d.safe_content(safe), safe)

    def test_non_rsa_sibling_and_false_kty_strings_do_not_grant_context(self):
        for text in ('[{"kty":"RSA"},{"oth":[{"r":"PUBLIC_R","t":"PUBLIC_T"}]}]',
                     '{"message":"kty: RSA", "oth":[{"r":"PUBLIC_R","t":"PUBLIC_T"}]}',
                     '{"nested":{"kty":"RSA"}, "oth":[{"r":"PUBLIC_R","t":"PUBLIC_T"}]}',
                     '{"kty":"EC", "oth":[{"r":"PUBLIC_R","t":"PUBLIC_T"}]}'):
            self.assert_safe(text, [], ['PUBLIC_R','PUBLIC_T'])

    def test_unquoted_members_newlines_and_nested_comments(self):
        text = "{oth: [/* synthetic */ {r: 'SYNTH_R', d: 'SYNTH_D', t: 'SYNTH_T'}],\n kty: 'RSA', n: 'PUBLIC_N', e: 'AQAB'}"
        self.assert_safe(text, ['SYNTH_R','SYNTH_D','SYNTH_T'], ['PUBLIC_N','AQAB'])
        self.assert_safe(json.dumps(text), ['SYNTH_R','SYNTH_D','SYNTH_T'], ['PUBLIC_N','AQAB'])

    def test_incomplete_string_object_cannot_borrow_sibling_kty(self):
        text = '[{"text":"{", "oth":[{"r":"PUBLIC_R","t":"PUBLIC_T"}]}, {"kty":"RSA"}]'
        self.assert_safe(text, [], ['PUBLIC_R','PUBLIC_T'])

    def test_public_jwk_coordinates_and_native_identity_retained(self):
        value = dict(kty='RSA', n='PUBLIC_N', e='AQAB', x='PUBLIC_X', y='PUBLIC_Y',
                     oth=[dict(r='SYNTH_R', d='SYNTH_D', t='SYNTH_T')])
        finding = dict(finding_id='immutable-id', rule_id='immutable-rule', location=dict(file='public.py', line=9, column=3), evidence=value)
        safe = d.safe_native(finding)
        self.assertEqual(safe['finding_id'], finding['finding_id'])
        self.assertEqual(safe['location'], finding['location'])
        for key in ('n','e','x','y'):
            self.assertEqual(safe['evidence'][key], value[key])
        self.assertNotIn('SYNTH_R', json.dumps(safe))

    def test_private_dump_inside_literal_preserves_public_tail(self):
        for heading in ('Private-Key: (256 bit)', 'ED25519 Private-Key:'):
            dump = heading + '\npriv:\n    a1:b2:c3:d4\npub:\n    e5:f6:77:88\n'
            for wrapped in ('expected = """'+dump+'"""\nPUBLIC_TAIL', json.dumps(dump)+'; PUBLIC_TAIL',
                            '/* '+dump+' */\nPUBLIC_TAIL'):
                self.assert_safe(wrapped, ['a1:b2:c3:d4','e5:f6:77:88'], ['PUBLIC_TAIL'])

    def test_private_dump_eof_and_public_dump_controls(self):
        self.assert_safe('Private-Key: (256 bit)\npriv:\n    a1:b2:c3:d4\npub:\n    e5:f6:77:88',
                         ['a1:b2:c3:d4','e5:f6:77:88'], [])
        text = 'Public-Key: (256 bit)\npub:\n    a1:b2:c3:d4\nPUBLIC_TAIL'
        self.assert_safe(text, [], ['a1:b2:c3:d4','PUBLIC_TAIL'])


if __name__ == '__main__':
    unittest.main()
