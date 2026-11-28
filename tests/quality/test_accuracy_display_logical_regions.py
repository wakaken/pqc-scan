"""Synthetic logical-region privacy and public-boundary regressions."""
import json
import re
import unittest
import accuracy_display as d

DUMP = 'Private-Key: (256 bit)\npriv:\n    11:22:33:44\npub:\n    55:66:77:88'


class LogicalDumpRegionTests(unittest.TestCase):
    def assert_dump(self, text, visible=('PUBLIC_TAIL',)):
        for opaque in (False, True):
            with self.subTest(opaque=opaque):
                safe, changed = d.mask_text(text, opaque)
                self.assertTrue(changed)
                self.assertNotIn('11:22:33:44', safe)
                self.assertNotIn('55:66:77:88', safe)
                for value in visible:
                    self.assertIn(value, safe)
                self.assertEqual(len(safe), len(text))
                for match in re.finditer(r'\\+[nr]|[\r\n]', text):
                    self.assertEqual(safe[match.start():match.end()], match[0])
                self.assertEqual(d.mask_text(safe, opaque), (safe, False))

    def test_contiguous_line_comment_regions(self):
        for prefix in ('// ', '# '):
            for newline in ('\n', '\r\n', '\r'):
                text = newline.join(prefix+line for line in DUMP.splitlines())
                self.assert_dump('公開\n'+text+newline+'PUBLIC_TAIL', ('公開', 'PUBLIC_TAIL'))

    def test_decorated_and_bare_block_regions(self):
        for newline in ('\n', '\r\n', '\r'):
            for body in (DUMP.replace('\n', newline), newline+newline.join(' * '+line for line in DUMP.splitlines())+newline+' '):
                self.assert_dump('/* '+body+' */'+newline+'PUBLIC_TAIL')

    def test_rust_raw_prefixes_hash_widths_and_internal_quotes(self):
        for prefix in ('r', 'br', 'cr'):
            for width in (0, 1, 3, 255):
                hashes = '#'*width
                self.assert_dump('let value = '+prefix+hashes+'"'+DUMP+'"'+hashes+';\nPUBLIC_TAIL')
        self.assert_dump('r###"note " and "#\n'+DUMP+'"###; PUBLIC_TAIL')

    def test_cpp_raw_delimiters(self):
        for delimiter in ('', 'tag', 'abcdefghijklmnop'):
            self.assert_dump('R"'+delimiter+'('+DUMP+')'+delimiter+'"; PUBLIC_TAIL')

    def test_physical_and_escaped_literal_regions(self):
        for text in ('"""'+DUMP+'"""', "'''"+DUMP+"'''", json.dumps(DUMP)):
            self.assert_dump(text+'; PUBLIC_TAIL')

    def test_double_and_triple_serialized_regions(self):
        for depth in (1, 2, 3):
            text = DUMP
            for _ in range(depth):
                text = json.dumps(text)
            self.assert_dump(text+'; PUBLIC_TAIL')

    def test_nested_serialized_body_keeps_its_own_public_tail(self):
        text = json.dumps(json.dumps(DUMP)+'; INNER_TAIL')+'; PUBLIC_TAIL'
        self.assert_dump(text, ('INNER_TAIL', 'PUBLIC_TAIL'))

    def test_literal_hash_line_is_not_a_source_comment(self):
        self.assert_dump('"""# prefix\n'+DUMP+'"""; PUBLIC_TAIL')
        self.assert_dump(json.dumps('# prefix\n'+DUMP)+'; PUBLIC_TAIL')

    def test_unclosed_literal_and_comment_end_at_eof(self):
        for text in ('r###"'+DUMP, '/*\n * '+DUMP.replace('\n','\n * '), '"""'+DUMP):
            self.assert_dump(text, ())

    def test_blank_line_and_code_stop_line_comment_groups(self):
        for gap in ('\n\n', '\npublic_call();\n'):
            text = '// '+DUMP.replace('\n','\n// ')+gap+'// PUBLIC_TAIL'
            self.assert_dump(text)

    def test_marker_like_source_text_is_not_stripped(self):
        text = 'Public-Key: (256 bit)\n// priv:\n// 11:22:33:44\nPUBLIC_TAIL'
        for opaque in (False, True):
            safe, _ = d.mask_text(text, opaque)
            self.assertIn('11:22:33:44', safe)
            self.assertIn('PUBLIC_TAIL', safe)

    def test_public_dump_and_certificate_controls(self):
        for heading in ('Public-Key', 'Certificate'):
            body = DUMP.replace('Private-Key', heading).replace('priv:', 'pub:')
            for text in ('/*\n'+'\n'.join(' * '+line for line in body.splitlines())+'\n */',
                         'r#"'+body+'"#', json.dumps(json.dumps(body))):
                for opaque in (False, True):
                    self.assertIn('11:22:33:44', d.mask_text(text, opaque)[0])

    def test_raw_private_field_body_retains_public_tail(self):
        for text in ('private_key = r#"short synthetic material"#; PUBLIC_TAIL',
                     'private_key = R"tag(short synthetic material)tag"; PUBLIC_TAIL'):
            safe, _ = d.mask_text(text, False)
            self.assertNotIn('short synthetic material', safe)
            self.assertIn('PUBLIC_TAIL', safe)
            self.assertEqual(d.mask_text(safe, False), (safe, False))

    def test_python_prefixed_triple_literal(self):
        self.assert_dump('r"""'+DUMP+'"""; PUBLIC_TAIL')

    def test_native_and_content_boundaries_use_whole_region(self):
        finding = dict(finding_id='fixed-id', location=dict(file='fixture.rs', line=2, column=1),
                       source_snippet=dict(lines=[dict(text='// '+line) for line in DUMP.splitlines()]))
        safe = d.safe_native(finding)
        self.assertEqual(safe['finding_id'], finding['finding_id'])
        self.assertEqual(safe['location'], finding['location'])
        self.assertNotIn('11:22:33:44', json.dumps(safe))
        self.assertNotIn('11:22:33:44', d.safe_content('/* '+DUMP+' */'))


if __name__ == '__main__':
    unittest.main()
