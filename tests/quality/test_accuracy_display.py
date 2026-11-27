"""Display privacy regressions use generated synthetic values only."""
import importlib.util
import json
import pathlib
import secrets
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('accuracy_sampling', ROOT / 'benchmark/accuracy_sampling.py')
sampling = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(sampling)


class DisplayPrivacyTests(unittest.TestCase):
    def check_mask(self, text, values):
        safe, changed = sampling.mask_secrets(text)
        self.assertTrue(changed, 'redaction must be reported')
        self.assertEqual(len(safe), len(text))
        self.assertEqual([i for i,c in enumerate(text) if c in '\r\n'], [i for i,c in enumerate(safe) if c in '\r\n'])
        for value in values:
            self.assertTrue(value not in safe, 'synthetic private value survived')
        return safe

    def test_jwk_private_fields_and_public_labels(self):
        for key in ('d','p','q','dp','dq','qi','k'):
            value = secrets.token_urlsafe(24)
            text = json.dumps({'kty':'EC','crv':'P-256',key:value})
            safe = self.check_mask(text,[value])
            self.assertIn('P-256',safe)

    def test_shared_seed_and_scalar_variants(self):
        for key in ('shared','shared_secret','seed','privateScalar','private_key','secretKey','authorization','key_material'):
            for format in ('"{key}": "{value}"',"{key} = '{value}'",'{key}: [{value}, {value}]'):
                value=secrets.token_hex(24)
                self.check_mask(format.format(key=key,value=value),[value])

    def test_private_container_short_values_and_numeric_arrays(self):
        value=str(secrets.randbelow(90000000)+10000000)
        self.check_mask('private = { a: "'+value+'", nested: [12345678, 87654321] }',[value,'12345678','87654321'])

    def test_multiline_concatenated_strings(self):
        first,second=secrets.token_hex(20),secrets.token_hex(20)
        self.check_mask('shared = (\r\n    "'+first+'"\r\n    "'+second+'"\r\n)\r\nrsa = 1\r\n',[first,second])

    def test_multiline_sensitive_body_is_masked_before_excerpt(self):
        value=secrets.token_hex(20)
        text='password = """\n'+value+'\n"""\n'+'rsa\n'*50
        context=sampling.context_at(text,22)
        self.assertTrue(value not in context['contextSafeText'])
        self.assertEqual(context['contextStart'],2)

    def test_opaque_hex_decimal_and_encoded_constants(self):
        values=[secrets.token_hex(32),str(secrets.randbits(250)),secrets.token_urlsafe(48), 'A'*43]
        for value in values:
            self.check_mask('unclassified = "'+value+'"',[value])

    def test_escaped_json_and_unicode_private_key_name(self):
        value=secrets.token_urlsafe(25)
        source=json.dumps(json.dumps({'d':value,'shared':value}))
        self.check_mask(source,[value])
        self.check_mask('"\\u0064": "'+value+'"',[value])

    def test_physical_and_escaped_newline_unicode_columns(self):
        value=secrets.token_hex(20)
        text='見出し🙂\r\nseed = "'+value+'\\n'+value+'\\r'+value+'"\r\n'
        safe=self.check_mask(text,[value])
        self.assertEqual(safe[:len('見出し🙂\r\n')],'見出し🙂\r\n')
        self.assertIn('\\n',safe)
        self.assertIn('\\r',safe)

    def test_reference_private_pem_masked_before_cut(self):
        value=secrets.token_urlsafe(40)
        text='heading\n-----BEGIN PRIVATE KEY-----\n'+value+'\n-----END PRIVATE KEY-----\ntrailer'
        safe=self.check_mask(text,[value])
        self.assertTrue(value not in '\n'.join(safe.splitlines()[2:3]))

    def test_benign_code_unchanged(self):
        text='rsa = object()\nalgorithm = "RSA"\npath = "docs/private-key-guide.md"\n'
        safe,changed=sampling.mask_secrets(text)
        self.assertEqual(safe,text)
        self.assertFalse(changed)

    def test_native_identity_hashes_locations_preserved(self):
        value=secrets.token_hex(32)
        identity=secrets.token_hex(32)
        finding={'finding_id':identity,'rule_id':'RSA_REFERENCE','location':{'file':identity+'.py','line':10,'column':5},
                 'evidence':{'type':'regex_match','metadata':{'sha256':identity,'d':value},'snippet_preview':'"shared": "'+value+'"'},
                 'description':'RSA reference','source_snippet':{'lines':[{'line':1,'text':'shared = """'},{'line':2,'text':value},{'line':3,'text':'"""'}]}}
        safe=sampling.safe_finding(finding)
        self.assertEqual(safe['finding_id'],identity)
        self.assertEqual(safe['location'],finding['location'])
        self.assertEqual(safe['evidence']['metadata']['sha256'],identity)
        self.assertTrue(value not in json.dumps(safe),'synthetic private value survived native display')

    def test_native_nested_advice_content_scrubbed(self):
        value=secrets.token_hex(32)
        finding={'finding_id':'id','location':{'file':'safe.py','line':1,'column':1},
                 'recommended_actions':[{'code_examples':['shared = "'+value+'"'],'metadata':{'private':[value]}}]}
        self.assertTrue(value not in json.dumps(sampling.safe_finding(finding)))

    def test_oversize_display_fails_closed(self):
        with self.assertRaisesRegex(ValueError,'display_resource_limit'):
            sampling.mask_secrets('a'*(2*1024*1024+1))


    def test_html_entity_private_fields(self):
        value=secrets.token_urlsafe(25)
        self.check_mask('&quot;d&quot;: &quot;'+value+'&quot;',[value])

    def test_unclosed_private_container_fails_closed(self):
        value=secrets.token_urlsafe(25)
        self.check_mask('private = { broken\n'+value,[value])

    def test_display_excerpt_checks_binding_and_masks_whole_file(self):
        import tempfile
        with tempfile.TemporaryDirectory() as directory:
            root=pathlib.Path(directory).resolve()
            value=secrets.token_urlsafe(25)
            raw=('shared = """\n'+value+'\n"""\n').encode()
            source=root/'source.py';source.write_bytes(raw)
            result=sampling.display_excerpt(source,sampling.sha(raw),2,2)
            self.assertTrue(value not in result['safeText'])
            with self.assertRaisesRegex(ValueError,'sha256_drift'):
                sampling.display_excerpt(source,'0'*64,2,2)
            linked=root/'link.py';linked.symlink_to(source)
            with self.assertRaisesRegex(ValueError,'unsafe_or_nonregular_binding'):
                sampling.display_excerpt(linked,sampling.sha(raw))

    def test_view_exclusive_owner_only_and_immutable_source_identity(self):
        import tempfile
        with tempfile.TemporaryDirectory() as directory:
            root=pathlib.Path(directory).resolve()
            value=secrets.token_urlsafe(25)
            raw=('shared = "'+value+'"\r\n').encode()
            source=root/'source.py';source.write_bytes(raw)
            row={'id':'source-1','sourceRoot':str(root),'sourcePath':'source.py','sourceSha256':sampling.sha(raw),
                 'ownedLine':1,'contextStart':1,'contextEnd':1,'contextSafeText':raw.decode()}
            packet={'version':1,'kind':'source','records':[row]}
            path=root/'packet.json';sampling.write_new(path,packet)
            old=path.read_bytes();old_mode=source.stat().st_mode
            result=sampling.display_view(path,sampling.sha(old),root/'view')
            view=sampling.load_json(result['path'])
            self.assertTrue(value not in view['records'][0]['contextSafeText'])
            self.assertEqual(view['records'][0]['sourceSha256'],row['sourceSha256'])
            self.assertEqual(path.read_bytes(),old)
            self.assertEqual(source.stat().st_mode,old_mode)
            self.assertEqual((root/'view').stat().st_mode & 0o777,0o700)
            self.assertEqual(pathlib.Path(result['path']).stat().st_mode & 0o777,0o600)
            with self.assertRaises(FileExistsError):
                sampling.display_view(path,sampling.sha(old),root/'view')


    def test_proven_public_jwk_coordinates_and_public_pem_survive(self):
        public=secrets.token_urlsafe(32)
        private=secrets.token_urlsafe(32)
        text=json.dumps({'kty':'EC','x':public,'d':private})
        safe=self.check_mask(text,[private])
        self.assertIn(public,safe)
        text='-----BEGIN CERTIFICATE-----\n'+public+'\n-----END CERTIFICATE-----'
        self.assertEqual(sampling.mask_secrets(text),(text,False))

    def test_native_public_opaque_payload_is_not_classified_private_by_length(self):
        public=secrets.token_urlsafe(64)
        finding={'evidence':{'type':'public_key','metadata':{'public_key':public,'fingerprint':public}}}
        self.assertEqual(sampling.safe_finding(finding),finding)


    def test_untrusted_jwk_type_and_repeated_open_public_markers_are_bounded(self):
        text=json.dumps({'kty':[],'d':secrets.token_urlsafe(25)})
        safe,changed=sampling.mask_secrets(text)
        self.assertTrue(changed)
        text='-----BEGIN PUBLIC KEY-----\n'*1024
        self.assertEqual(len(sampling.mask_secrets(text)[0]),len(text))

    def test_deeply_encoded_private_structure_is_withheld(self):
        value=secrets.token_urlsafe(25)
        text=json.dumps({'d':value})
        for _ in range(6): text=json.dumps(text)
        self.check_mask(text,[value])

    def test_existing_context_mask_is_never_restored(self):
        import tempfile
        with tempfile.TemporaryDirectory() as directory:
            root=pathlib.Path(directory).resolve();raw=b'algorithm = "RSA"\n'
            (root/'source.py').write_bytes(raw)
            row={'id':'s','sourceRoot':str(root),'sourcePath':'source.py','sourceSha256':sampling.sha(raw),
                 'ownedLine':1,'contextStart':1,'contextEnd':1,'contextSafeText':'algorithm = "███"\n'}
            packet=root/'old.json';sampling.write_new(packet,{'version':1,'kind':'source','records':[row]})
            result=sampling.display_view(packet,sampling.sha(packet.read_bytes()),root/'view')
            view=sampling.load_json(result['path'])
            self.assertEqual(view['records'][0]['contextSafeText'],row['contextSafeText'])


    def test_dependency_identity_and_compound_public_names_survive(self):
        finding={'evidence':{'metadata':{'dep_name':'io.jsonwebtoken:jjwt-api','algorithm':'RSA'}}}
        self.assertEqual(sampling.safe_finding(finding),finding)
        value=secrets.token_urlsafe(25)
        self.check_mask('prefixPrivateKey = "'+value+'"',[value])
        self.assertEqual(sampling.mask_secrets('jsonwebtoken: public_api'),('jsonwebtoken: public_api',False))


    def test_display_cli_masks_whole_reference_before_excerpt(self):
        import subprocess
        import sys
        import tempfile
        with tempfile.TemporaryDirectory() as directory:
            root=pathlib.Path(directory).resolve();value=secrets.token_urlsafe(25)
            raw=('-----BEGIN PRIVATE KEY-----\n'+value+'\n-----END PRIVATE KEY-----\n').encode()
            path=root/'reference.txt';path.write_bytes(raw)
            command=[sys.executable,str(ROOT/'benchmark/accuracy_display.py'),'excerpt','--path',str(path),
                     '--sha256',sampling.sha(raw),'--start','2','--end','2']
            result=subprocess.run(command,capture_output=True,text=True,check=False)
            self.assertEqual(result.returncode,0)
            displayed=json.loads(result.stdout)
            self.assertTrue(value not in displayed['safeText'])
            self.assertEqual(len(displayed['safeText']),len(value)+1)
            command[command.index('--sha256')+1]='0'*64
            result=subprocess.run(command,capture_output=True,text=True,check=False)
            self.assertEqual(result.returncode,2)
            self.assertEqual(json.loads(result.stdout)['status'],'withheld')
            self.assertTrue(value not in result.stdout+result.stderr)


    def test_private_marker_overrides_surrounding_public_marker(self):
        value=secrets.token_urlsafe(32)
        text='-----BEGIN CERTIFICATE-----\n-----BEGIN PRIVATE KEY-----\n'+value+'\n-----END PRIVATE KEY-----\n-----END CERTIFICATE-----'
        self.check_mask(text,[value])


    def test_comment_delimiters_do_not_close_private_containers(self):
        value = 'SYNTH_COMMENT_PRIVATE'
        for comment in ('/* } ] ) \" don\'t */', '// } ] ) \" don\'t\n', '# } ] ) \" don\'t\r\n'):
            for opening, closing in (('{', '}'), ('[', ']'), ('(', ')')):
                with self.subTest(comment=comment, opening=opening):
                    text = 'private=' + opening + comment + '\"' + value + '\"' + closing + '; public=17'
                    safe = self.check_mask(text, [value])
                    self.assertIn('; public=17', safe)
                    self.assertEqual(sampling.mask_secrets(safe)[0], safe)

    def test_private_assignments_inside_comments_remain_protected(self):
        value = 'SYNTH_COMMENT_PRIVATE'
        for prefix, suffix in (('// don\'t ', '\n'), ('# don\'t ', '\r\n'), ('/* don\'t ', ' */')):
            for private in ('{d:\"' + value + '\"}', 'private={value:\"' + value + '\"}'):
                text = prefix + private + suffix + '\npublic=17'
                safe = self.check_mask(text, [value])
                self.assertIn('public=17', safe)
                self.assertEqual(sampling.mask_secrets(safe)[0], safe)

    def test_comment_quotes_before_live_private_assignments(self):
        value = 'SYNTH_COMMENT_PRIVATE'
        for comment in ('// don\'t \"\n', '# don\'t \"\r\n', '/* don\'t \" */'):
            text = comment + '{d:\"' + value + '\"}; public=17'
            safe = self.check_mask(text, [value])
            self.assertIn('public=17', safe)
            self.assertEqual(sampling.mask_secrets(safe)[0], safe)

    def test_comment_between_private_assignment_and_value(self):
        value = 'SYNTH_COMMENT_PRIVATE'
        for comment in ('/* } \" don\'t */', '// } \"\n', '# } \"\n'):
            text = 'private=' + comment + '{x:\"' + value + '\"}; public=17'
            safe = self.check_mask(text, [value])
            self.assertIn('public=17', safe)
            self.assertEqual(sampling.mask_secrets(safe)[0], safe)

    def test_comment_between_adjacent_private_literals(self):
        for comment in ('/* } \" don\'t */', '// } \"\n', '# } \"\n'):
            text = 'shared=\"SYNTH_PRIVATE_FIRST\" + ' + comment + '\"SYNTH_PRIVATE_SECOND\"; public=17'
            safe = self.check_mask(text, ['SYNTH_PRIVATE_FIRST', 'SYNTH_PRIVATE_SECOND'])
            self.assertIn('public=17', safe)
            self.assertEqual(sampling.mask_secrets(safe)[0], safe)

    def test_multiline_private_mask_is_idempotent_and_bounded(self):
        text = 'private={\n x:\"SYNTH_PRIVATE_FIRST\",\n y:\"SYNTH_PRIVATE_SECOND\"\n}; public=17\n'
        safe = self.check_mask(text, ['SYNTH_PRIVATE_FIRST', 'SYNTH_PRIVATE_SECOND'])
        self.assertIn('; public=17', safe)
        self.assertEqual(sampling.mask_secrets(safe)[0], safe)

    def test_comment_markers_inside_public_strings_do_not_hide_private_values(self):
        for public in ('https://example.invalid', '# public', '/* public */', "don't"):
            text = 'url=' + json.dumps(public) + '; d=\"SYNTH_PRIVATE_VALUE\"; public=17'
            safe = self.check_mask(text, ['SYNTH_PRIVATE_VALUE'])
            self.assertIn(json.dumps(public), safe)
            self.assertIn('; public=17', safe)
            self.assertEqual(sampling.mask_secrets(safe)[0], safe)


    def test_nested_block_comments_do_not_close_private_array(self):
        text = 'private=[/* outer /* inner */ ] \" don\'t */ \"SYNTH_NESTED_PRIVATE\"]; public=17'
        safe = self.check_mask(text, ['SYNTH_NESTED_PRIVATE'])
        self.assertIn('; public=17', safe)
        self.assertEqual(sampling.mask_secrets(safe)[0], safe)

    def test_comment_prose_braces_quotes_do_not_hide_private_jwk_or_scalar(self):
        for prefix, suffix in (('// prose { don\'t \" ', '\r\n'), ('# prose { don\'t \" ', '\n'), ('/* prose { don\'t \" ', ' */')):
            for private in ('{kty:\"RSA\",d:\"SYNTH_PROSE_PRIVATE\"}', 'scalar=\"SYNTH_PROSE_PRIVATE\"'):
                text = prefix + private + suffix + '\npublic=17'
                safe = self.check_mask(text, ['SYNTH_PROSE_PRIVATE'])
                self.assertIn('public=17', safe)
                self.assertEqual(sampling.mask_secrets(safe)[0], safe)

    def test_multiline_comment_private_container_is_conservatively_masked(self):
        for marker in ('//', '#'):
            text = marker + ' prose don\'t private={\r\n' + marker + ' value:\"SYNTH_MULTILINE_PRIVATE\"\r\n' + marker + ' }\r\npublic=17'
            safe = self.check_mask(text, ['SYNTH_MULTILINE_PRIVATE'])
            self.assertIn('public=17', safe)
            self.assertEqual(sampling.mask_secrets(safe)[0], safe)

    def test_nested_comment_private_material_is_protected_independently(self):
        text = '/* prose { don\'t \" /* scalar=\"SYNTH_NESTED_PRIVATE\" */ tail */ public=17'
        safe = self.check_mask(text, ['SYNTH_NESTED_PRIVATE'])
        self.assertIn('public=17', safe)
        self.assertEqual(sampling.mask_secrets(safe)[0], safe)


    def test_public_jwk_fields_survive_repeated_private_masks(self):
        public = 'SYNTH_PUBLIC_VALUE_1234567890abcdefghijklmnop'
        for key in ('n', 'e', 'x', 'y'):
            for private in ('d', 'p', 'q', 'dp', 'dq', 'qi', 'k'):
                for value in ('1', 12345, ['1', 12345], {'nested': '1'}):
                    with self.subTest(public_field=key, private_field=private, value_type=type(value).__name__):
                        text = json.dumps({'kty': 'RSA' if key in ('n', 'e') else 'EC', private: value, key: public})
                        safe, changed = sampling.mask_secrets(text)
                        self.assertTrue(changed)
                        self.assertIn(public, safe)
                        self.assertEqual(sampling.mask_secrets(safe), (safe, False))

    def test_nested_public_jwk_and_masked_containers_remain_idempotent(self):
        public = 'SYNTH_PUBLIC_VALUE_1234567890abcdefghijklmnop'
        source = {'keys': [{'kty': 'OKP', 'd': '1', 'x': public},
                           {'kty': 'RSA', 'private': {'values': ['SYNTH_PRIVATE_VALUE']}, 'n': public}],
                  'shared': ['SYNTH_PRIVATE_VALUE']}
        for indent in (None, 2):
            text = json.dumps(source, indent=indent).replace('\n', '\r\n')
            safe = self.check_mask(text, ['SYNTH_PRIVATE_VALUE'])
            self.assertIn(public, safe)
            self.assertEqual(sampling.mask_secrets(safe), (safe, False))

    def test_public_jwk_idempotence_through_encoded_projection(self):
        public = 'SYNTH_PUBLIC_VALUE_1234567890abcdefghijklmnop'
        text = json.dumps({'kty': 'EC', 'd': '1', 'x': public})
        for encoded in (json.dumps(text), text.replace('"', '&quot;')):
            safe, changed = sampling.mask_secrets(encoded)
            self.assertTrue(changed)
            self.assertIn(public, safe)
            self.assertEqual(sampling.mask_secrets(safe), (safe, False))

    def test_jwk_private_pem_overrides_public_slot_and_repeated_masks(self):
        public = 'SYNTH_PUBLIC_VALUE_1234567890abcdefghijklmnop'
        private = 'SYNTH_PRIVATE_PEM_VALUE_1234567890abcdef'
        pem = '-----BEGIN PRIVATE KEY-----\n' + private + '\n-----END PRIVATE KEY-----'
        text = json.dumps({'kty': 'EC', 'x': pem, 'y': public, 'd': '1'})
        safe = self.check_mask(text, [private])
        self.assertIn(public, safe)
        self.assertEqual(sampling.mask_secrets(safe), (safe, False))

    def test_internal_mask_projection_retains_strict_json_public_proof(self):
        public = 'SYNTH_PUBLIC_VALUE_1234567890abcdefghijklmnop'
        for invalid in ('{"kty":"EC", "d":███ + value, "x":"' + public + '"}',
                        '{"kty":"EC", "d":███, /* comment */ "x":"' + public + '"}',
                        '{"kty":"EC", "kty":"RSA", "d":███, "x":"' + public + '"}',
                        '{"kty":[], "d":███, "x":"' + public + '"}',
                        '{"kty":"EC", "d":███, "x":computed("' + public + '")}'):
            with self.subTest(invalid=invalid):
                safe, changed = sampling.mask_secrets(invalid)
                self.assertTrue(changed)
                self.assertNotIn(public, safe)
                self.assertEqual(sampling.mask_secrets(safe), (safe, False))

    def test_quoted_mask_text_does_not_become_json_structure(self):
        public = 'SYNTH_PUBLIC_VALUE_1234567890abcdefghijklmnop'
        text = json.dumps({'kty': '███', 'd': '1', 'x': public})
        safe, changed = sampling.mask_secrets(text)
        self.assertTrue(changed)
        self.assertNotIn(public, safe)
        self.assertEqual(sampling.mask_secrets(safe), (safe, False))

    def test_public_value_reused_in_private_slot_remains_masked_there(self):
        public = 'SYNTH_PUBLIC_VALUE_1234567890abcdefghijklmnop'
        text = json.dumps({'kty': 'EC', 'd': public, 'x': public})
        safe, changed = sampling.mask_secrets(text)
        self.assertTrue(changed)
        self.assertEqual(safe.count(public), 1)
        self.assertEqual(sampling.mask_secrets(safe), (safe, False))



    def test_generated_native_mask_forces_redacted_flag(self):
        import tempfile
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory).resolve()
            native = {'finding_id': 'f' * 64, 'rule_id': 'RSA_REFERENCE',
                      'location': {'file': 'public.py', 'line': 1, 'column': 1},
                      'evidence': {'metadata': {'d': '1', 'public_key': 'SYNTH_PUBLIC_VALUE' * 4,
                                               'sha256': 'b' * 64, 'dep_name': 'io.jsonwebtoken:jjwt-api'}}}
            packet = root / 'packet.json'
            sampling.write_new(packet, {'version': 1, 'kind': 'claim', 'records': [
                {'id': 'synthetic-claim', 'nativeFinding': native, 'nativeFindingRedacted': False}]})
            result = sampling.display_view(packet, sampling.sha(packet.read_bytes()), root / 'view')
            row = sampling.load_json(result['path'])['records'][0]
            self.assertTrue(row['displayPrivacy']['addedNativeMask'])
            self.assertTrue(row['nativeFindingRedacted'])
            self.assertFalse(row['displayPrivacy']['originalNativeFindingRedacted'])
            self.assertEqual(row['nativeFinding']['location'], native['location'])
            self.assertEqual(row['nativeFinding']['finding_id'], native['finding_id'])
            for key in ('public_key', 'sha256', 'dep_name'):
                self.assertEqual(row['nativeFinding']['evidence']['metadata'][key], native['evidence']['metadata'][key])



    def test_public_pem_padding_does_not_become_private_assignment(self):
        public = 'SYNTH_PUBLIC_BODY_1234567890/p='
        for label in ('CERTIFICATE', 'PUBLIC KEY', 'RSA PUBLIC KEY'):
            text = '-----BEGIN ' + label + '-----\n' + public + '\n-----END ' + label + '-----\nalgorithm="RSA"\n'
            for source in (text, 'const certificate = ' + json.dumps(text) + '; public=17'):
                with self.subTest(label=label, escaped=source != text):
                    self.assertEqual(sampling.mask_secrets(source), (source, False))

    def test_private_pem_padding_and_nested_marker_keep_trailing_public_text(self):
        private = 'SYNTH_PRIVATE_BODY_1234567890/p='
        for public_label in ('CERTIFICATE', 'PUBLIC KEY'):
            text = ('-----BEGIN ' + public_label + '-----\n-----BEGIN PRIVATE KEY-----\n' + private +
                    '\n-----END PRIVATE KEY-----\n-----END ' + public_label + '-----\nalgorithm="RSA"\n')
            for source in (text, 'const certificate = ' + json.dumps(text) + '; public=17'):
                safe = self.check_mask(source, [private])
                self.assertIn('-----END ' + public_label + '-----', safe)
                self.assertEqual(sampling.mask_secrets(safe), (safe, False))
                self.assertTrue('public=17' in safe or 'algorithm="RSA"' in safe)

    def test_proven_public_jwk_padding_is_not_private_assignment_syntax(self):
        for key in ('p', 'q', 'd', 'k'):
            public = 'SYNTH_PUBLIC_VALUE_1234567890/' + key + '='
            for public_key in ('x', 'y', 'n', 'e'):
                text = json.dumps({'kty': 'RSA' if public_key in ('n', 'e') else 'EC', 'd': '1', public_key: public}, indent=2)
                safe, changed = sampling.mask_secrets(text)
                self.assertTrue(changed)
                self.assertIn(public, safe)
                self.assertEqual(sampling.mask_secrets(safe), (safe, False))



    def test_embedded_private_assignment_stays_inside_outer_json_string(self):
        public = 'SYNTH_PUBLIC_VALUE_1234567890abcdefghijklmnop'
        for private in ('d=SYNTH_PRIVATE', "d='SYNTH_PRIVATE'", 'secret=SYNTH_PRIVATE'):
            text = json.dumps({'keys': [{'kty': 'EC', 'x': public}],
                               'message': 'url?' + private,
                               'other': 'public text',
                               'later': {'kty': 'EC', 'y': public}}, indent=2)
            safe = self.check_mask(text, ['SYNTH_PRIVATE'])
            self.assertIn('public text', safe)
            self.assertEqual(safe.count(public), 2)
            self.assertEqual(sampling.mask_secrets(safe), (safe, False))

    def test_private_assignment_inside_source_string_preserves_outer_syntax(self):
        for delimiter in ('"', "'", '`', '"""'):
            text = 'message=' + delimiter + 'url?d=SYNTH_PRIVATE' + delimiter + '; public=17'
            safe = self.check_mask(text, ['SYNTH_PRIVATE'])
            self.assertTrue(safe.endswith(delimiter + '; public=17'))
            self.assertEqual(sampling.mask_secrets(safe), (safe, False))


if __name__ == '__main__':
    unittest.main()
