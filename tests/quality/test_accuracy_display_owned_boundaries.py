import importlib.util
import os
import pathlib
import sys
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
MODULE = pathlib.Path(os.environ.get('PQC_DISPLAY_TEST_MODULE', ROOT / 'benchmark/accuracy_display.py'))
SPEC = importlib.util.spec_from_file_location('display_owned_boundary_subject', MODULE)
display = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = display
SPEC.loader.exec_module(display)


def nested_raw(depth, body):
    for width in range(1, depth + 1):
        marker = '#' * width
        body = 'r' + marker + '"' + body + '"' + marker
    return 'let expected = ' + body + ';\nPUBLIC_TAIL'


class OwnedBoundaryPrivacyTests(unittest.TestCase):
    def check_private(self, text, value, opaque):
        safe, changed = display.mask_text(text, opaque=opaque)
        self.assertTrue(changed)
        self.assertNotIn(value, safe)
        self.assertIn('PUBLIC_TAIL', safe)
        self.assertEqual(len(text), len(safe))
        self.assertEqual([(i,c) for i,c in enumerate(text) if c in '\r\n'],
                         [(i,c) for i,c in enumerate(safe) if c in '\r\n'])
        self.assertEqual(display.mask_text(safe, opaque=opaque)[0], safe)

    def test_owned_rsa_public_components_cannot_split_private_owner(self):
        for component in ('priv', 'privateExponent', 'prime1', 'prime2', 'exponent1', 'exponent2', 'coefficient'):
            for opener, closer in [('r#"', '"#'), ('br###"', '"###'), ('cr#"', '"#'), ('"""', '"""'), ('`', '`'), ('/*', '*/')]:
                for quote_before_label in (False, True):
                    before, after = ('"\r\n', '') if quote_before_label else ('', '"\r\n')
                    body = ('prose "Private-Key: (2048 bit)\r\nmodulus:\r\n  00:11\r\n'
                            'publicExponent: 65537\r\n' + before + component + ':\r\n' + after
                            + '  11:22:33:44\r\n  55:66:77:88\r\n')
                    text = '公開 PREFIX; ' + opener + body + closer + ';\r\nPUBLIC_TAIL'
                    for opaque in (False, True):
                        with self.subTest(component=component, opener=opener, quote_before_label=quote_before_label, opaque=opaque):
                            self.check_private(text, '11:22:33:44', opaque)
                            self.check_private(text, '55:66:77:88', opaque)
        text = '/*\r\n * prose "PRIVATE-KEY: (2048 bit)\r\n * modulus:\r\n *   00:11\r\n * publicExponent: 65537\r\n * "\r\n * PRIME1:\r\n *   11:22:33:44\r\n *   55:66:77:88\r\n */\r\nPUBLIC_TAIL'
        for opaque in (False, True):
            self.check_private(text, '11:22:33:44', opaque)
            self.check_private(text, '55:66:77:88', opaque)

    def test_owned_rsa_requires_later_private_component_in_same_owner(self):
        bodies = [
            'OWNER_PREFIX prose "Private-Key: (2048 bit)\nmodulus:\n  00:11\npublicExponent: 65537\n"\nOWNER_SUFFIX',
            'OWNER_PREFIX\nprime1:\n  aa:bb:cc:dd\nprose "Private-Key: (2048 bit)\nmodulus:\n  00:11\n"\nOWNER_SUFFIX',
        ]
        texts = ['r#"' + body + '"#;\nPUBLIC_TAIL' for body in bodies]
        texts.append('first=r#"OWNER_PREFIX prose \'Private-Key: (2048 bit)\nmodulus:\n  00:11\n\'\nOWNER_SUFFIX"#;\nsecond=r#"prime1:\n  aa:bb:cc:dd"#;\nPUBLIC_TAIL')
        for text in texts:
            for opaque in (False, True):
                with self.subTest(text=text, opaque=opaque):
                    safe, _ = display.mask_text(text, opaque=opaque)
                    for public in ('OWNER_PREFIX', 'OWNER_SUFFIX', 'PUBLIC_TAIL'):
                        self.assertIn(public, safe)
                    if 'aa:bb:cc:dd' in text:
                        self.assertIn('aa:bb:cc:dd', safe)
        public = 'r#"prose "Public-Key: (2048 bit)\nmodulus:\n  00:11\npublicExponent: 65537\n"\nprime1:\n  aa:bb:cc:dd\n"#; PUBLIC_TAIL'
        for opaque in (False, True):
            self.assertEqual(display.mask_text(public, opaque=opaque), (public, False))

    def test_owned_dump_inner_quotes_cannot_end_private_body(self):
        body = 'prose "Private-Key:\npriv:\n"\n  11:22:33:44\n'
        for prefix in ('r#', 'br###', 'cr#'):
            text = '公開 let dump = ' + prefix + '"' + body + '"' + '#' * prefix.count('#') + ';\nPUBLIC_TAIL'
            for opaque in (False, True):
                with self.subTest(prefix=prefix, opaque=opaque):
                    self.check_private(text, '11:22:33:44', opaque)
        for opener, closer in [('/*', '*/'), ('"""', '"""'), ('`', '`')]:
            for value in (body, body.replace('\n', r'\n')):
                text = '公開 ' + opener + value + closer + ';\nPUBLIC_TAIL'
                for opaque in (False, True):
                    with self.subTest(opener=opener, escaped=value != body, opaque=opaque):
                        self.check_private(text, '11:22:33:44', opaque)

    def test_owned_public_dump_inner_quotes_remain_visible(self):
        body = 'prose "Public-Key:\npublic bytes:\n"\n  11:22:33:44\n'
        for opener, closer in [('r#"', '"#'), ('br###"', '"###'), ('cr#"', '"#'), ('/*', '*/')]:
            text = opener + body + closer + ';\nPUBLIC_TAIL'
            for opaque in (False, True):
                with self.subTest(opener=opener, opaque=opaque):
                    safe, _ = display.mask_text(text, opaque=opaque)
                    self.assertIn('11:22:33:44', safe)
                    self.assertIn('PUBLIC_TAIL', safe)

    def test_nested_valid_raw_literals_fail_closed(self):
        body = 'Private-Key: (256 bit)\npriv:\n  11:22:33:44\npub:\n  55:66:77:88'
        for depth in (1,4,5,6,12):
            for opaque in (False,True):
                with self.subTest(depth=depth,opaque=opaque):
                    self.check_private(nested_raw(depth,body),'11:22:33:44',opaque)

    def test_ed25519_no_bit_header_respects_line_comment_owner(self):
        for prefix in ('//','#'):
            for newline in ('\n','\r\n'):
                text=newline.join([prefix+' ED25519 Private-Key:',prefix+' priv:',prefix+'    13:24:35:46','PUBLIC_TAIL'])
                for opaque in (False,True):
                    with self.subTest(prefix=prefix,newline=newline,opaque=opaque):
                        self.check_private(text,'13:24:35:46',opaque)

    def test_decorated_block_owner_preserves_outside_tail(self):
        text='/* ED25519 Private-Key:\n * priv:\n *   13:24:35:46\n */\nPUBLIC_TAIL'
        for opaque in (False,True):
            self.check_private(text,'13:24:35:46',opaque)

    def test_noncomment_assignment_crosses_comment_to_actual_value(self):
        text='private_key = // annotation\n "SYNTH_BOUNDARY_SECRET";\nPUBLIC_TAIL'
        for opaque in (False,True):
            self.check_private(text,'SYNTH_BOUNDARY_SECRET',opaque)

    def test_public_dump_is_not_private_and_deep_budget_keeps_tail(self):
        public='// ED25519 Public-Key:\n// pub:\n//   13:24:35:46\nPUBLIC_TAIL'
        for opaque in (False,True):
            self.assertEqual(display.mask_text(public,opaque=opaque),(public,False))
            deep=nested_raw(12,'a public description')
            safe,_=display.mask_text(deep,opaque=opaque)
            self.assertIn('PUBLIC_TAIL',safe)
            self.assertEqual(len(deep),len(safe))
            self.assertEqual(display.mask_text(safe,opaque=opaque)[0],safe)


if __name__ == '__main__':
    unittest.main()
