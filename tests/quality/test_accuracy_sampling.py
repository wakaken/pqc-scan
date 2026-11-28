"""Behavior checks for bounded, reproducible and prediction-blind packets."""
import datetime
import importlib.util
import json
import os
import pathlib
import random
import shutil
import subprocess
import tempfile
import unittest

MODULE = pathlib.Path(__file__).resolve().parents[2] / 'benchmark/accuracy_sampling.py'
SPEC = importlib.util.spec_from_file_location('accuracy_sampling', MODULE)
sampling = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(sampling)
REPO = pathlib.Path(os.environ.get('PQC_ACCURACY_REPO', pathlib.Path(__file__).resolve().parents[2]))


class PacketTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.base = pathlib.Path(self.temp.name).resolve()
        self.helpers = sampling.quality_helpers(REPO)

    def tearDown(self):
        self.temp.cleanup()

    def write(self, name, value):
        path = self.base / name
        path.parent.mkdir(parents=True, exist_ok=True)
        sampling.write_new(path, value)
        return path

    def fixture(self, count=2):
        repos, selections, roots = [], [], {}
        for i in range(count):
            rid = 'repo-' + str(i).zfill(3)
            root = self.base / rid
            root.mkdir()
            roots[rid] = str(root)
            items = []
            for j in range(4):
                path = 'crypto-' + str(j) + '.py'
                raw = ('first = 1\nrsa = object()\nlast = 2\n' if j else 'prefix = 1\n' * 40 + 'rsa = object()\n').encode()
                (root / path).write_bytes(raw)
                items.append(dict(path=path, bytes=len(raw), sha256=sampling.sha(raw), mode='100644'))
                if j < 3:
                    selections.append(dict(repositoryId=rid, population='primary', group='group-' + str(i % 9), split='holdout',
                                           stratum='lexical_path', sourcePath=path, sourceSha256=sampling.sha(raw), language='Python'))
            manifest = self.write(rid + '-manifest.json', items)
            repos.append(dict(id=rid, population='primary', group='group-' + str(i % 9), split='holdout',
                              root=str(root), commit='f' * 40, languages=['Python'], inputManifest=str(manifest),
                              inputManifestSha256=self.helpers.bytes_digest(manifest)))
        registry = self.write('registry.json', dict(repositories=repos))
        probe = self.write('selection.json', dict(registry=dict(path=str(registry), sha256=self.helpers.bytes_digest(registry)), selected=selections))
        roots_path = self.write('roots.json', roots)
        protocol = self.write('protocol.json', dict(version=1, studyId='test'))
        return dict(repo_root=REPO, registry_path=registry, source_selection_path=probe,
                    claim_selection_path=self.base / 'unused-claims.json', roots_path=roots_path,
                    protocol_path=protocol, study_id='test', include_claims=False)

    def packets(self, output, kind='source'):
        return [json.loads(p.read_text()) for p in sorted((output / kind).glob('primary-*.json'))]

    def claim_fixture(self, extra_fields=None):
        args = self.fixture()
        registry = json.loads(args['registry_path'].read_text())
        bindings, rows, all_selected, offset = [], [], [], 0
        for repo in registry['repositories']:
            findings = [dict(finding_id='native-' + repo['id'] + '-' + str(i), rule_id='RSA_REFERENCE',
                             category='Asymmetric', description='An RSA reference only; active protection unknown.',
                             location=dict(file='crypto-1.py', line=2, column=1),
                             evidence=dict(type='tree_sitter_match', metadata=dict(algorithm='RSA', scope='source')))
                        for i in range(3)]
            for finding in findings:
                finding.update(extra_fields or {})
            report = self.write(repo['id'] + '/output/report.json', dict(generated_at='2026-11-25T09:26:08Z', findings=findings))
            terminal = self.write(repo['id'] + '/terminal.json', dict(artifacts={'report.json': {'sha256': self.helpers.bytes_digest(report)}}))
            receipt = dict(path=str(terminal), sha256=self.helpers.bytes_digest(terminal))
            rows.append(dict(repositoryId=repo['id'], execution='Succeeded', assessment=dict(findingCount=len(findings)), receipt=receipt))
            bindings.append(dict(repositoryId=repo['id'], reportPath=str(report), reportSha256=self.helpers.bytes_digest(report), findingCount=len(findings), generatedAt='2026-11-25T09:26:08Z', terminal=receipt))
            for index, finding in enumerate(findings):
                all_selected.append(dict(sampleId='claim-b-' + str(offset + index).zfill(6), populationOrdinal=offset + index,
                                         repositoryId=repo['id'], nativeOrdinal=index, findingSha256=self.helpers.digest(finding),
                                         ruleId=finding['rule_id'], location=finding['location'], reportPath=str(report),
                                         reportSha256=self.helpers.bytes_digest(report)))
            offset += len(findings)
        binary = dict(path='/bound/historical/scanner', sha256='3' * 64)
        invocation = self.write('invocation.json', dict(rows=rows, scanner=dict(binary=binary)))
        chosen = set(random.Random(sampling.CLAIM_SEED).sample(range(offset), 3))
        probe = dict(bindings=bindings, invocation=dict(path=str(invocation), sha256=self.helpers.bytes_digest(invocation)),
                     scannerBinary=binary, sampleSeed=sampling.CLAIM_SEED, populationCount=offset, sampleCount=3,
                     inclusionProbability=3 / offset, selected=[x for x in all_selected if x['populationOrdinal'] in chosen])
        claim_path = self.write('claims.json', probe)
        args.update(claim_selection_path=claim_path, include_claims=True)
        return args, probe

    def test_complete_360_slots_and_exact_reproducibility(self):
        args = self.fixture(90)
        first, second = self.base / 'out-a', self.base / 'out-b'
        summary = sampling.build_packets(**args, output=first)
        sampling.build_packets(**args, output=second)
        self.assertEqual((summary['primarySourceSlots'], summary['primaryRepositories']), (360, 90))
        self.assertEqual(summary['sourceStatuses'], {'ready': 360})
        for path in first.rglob('*.json'):
            self.assertEqual(path.read_bytes(), (second / path.relative_to(first)).read_bytes())
        records = [row for p in self.packets(first) for row in p['records']]
        self.assertEqual(len({r['id'] for r in records}), 360)
        self.assertTrue(all(r['ownedLine'] == 41 for r in records if r['sourcePath'] == 'crypto-0.py'))
        self.assertTrue(all(r['contextEnd'] - r['contextStart'] < 41 for r in records))

    def test_exclusive_output_and_missing_rows_never_replaced(self):
        args = self.fixture()
        selection = json.loads(args['source_selection_path'].read_text())['selected']
        (self.base / 'repo-000/crypto-0.py').unlink()
        output = self.base / 'out'
        result = sampling.build_packets(**args, output=output)
        self.assertEqual(result['sourceStatuses']['missing_source'], 1)
        records = [row for p in self.packets(output) for row in p['records']]
        self.assertEqual(records[0]['sourcePath'], selection[0]['sourcePath'])
        self.assertEqual(records[0]['sourceStatus'], 'missing_source')
        self.assertIsNone(records[0]['ownedLine'])
        before = (output / 'index.json').read_bytes()
        with self.assertRaises(FileExistsError):
            sampling.build_packets(**args, output=output)
        self.assertEqual(before, (output / 'index.json').read_bytes())

    def test_source_packet_has_no_prediction_claim_or_rule_fields(self):
        args = self.fixture()
        output = self.base / 'out'
        sampling.build_packets(**args, output=output)
        for packet in self.packets(output):
            self.assertFalse(packet['provenance']['predictionsSeen'])
            for row in packet['records']:
                self.assertFalse({'nativeFinding', 'ruleId', 'rule_id', 'claimSummary', 'assertionChecklist', 'truth', 'issues'} & set(row))
                self.assertEqual(row['scopeMetadata']['unit'], 'one_owned_source_line')
                self.assertFalse(row['scopeMetadata']['contextIsCensus'])

    def test_manifest_registry_and_source_drift_are_explicit(self):
        args = self.fixture()
        (self.base / 'repo-000/crypto-0.py').write_text('changed')
        output = self.base / 'out'
        summary = sampling.build_packets(**args, output=output)
        self.assertEqual(summary['sourceStatuses']['sha256_drift'], 1)
        manifest = self.base / 'repo-001-manifest.json'
        manifest.write_text('[]')
        with self.assertRaisesRegex(ValueError, 'sha256_drift'):
            sampling.build_packets(**args, output=self.base / 'no-out')
        self.assertFalse((self.base / 'no-out').exists())

    def test_unsafe_paths_and_symlink_descendants(self):
        args = self.fixture()
        raw = (self.base / 'repo-000/crypto-0.py').read_bytes()
        target = self.base / 'outside.py'
        target.write_bytes(raw)
        source = self.base / 'repo-000/crypto-0.py'
        source.unlink()
        source.symlink_to(target)
        output = self.base / 'out'
        result = sampling.build_packets(**args, output=output)
        self.assertEqual(result['sourceStatuses']['unsafe_or_nonregular_binding'], 1)
        for name in ('../escape.py', '/absolute.py', 'x/../escape.py', 'x\\bad.py', 'x:bad'):
            with self.assertRaises(ValueError):
                self.helpers.safe_relative(name)

    def test_secret_canaries_keep_unicode_columns_and_newlines(self):
        text = '前 = 1\npassword = "CREDENTIAL_CANARY"\n-----BEGIN OPENSSH PRIVATE KEY-----\nPRIVATE_CANARY\n-----END OPENSSH PRIVATE KEY-----\n後 = rsa()\n'
        safe, changed = sampling.mask_secrets(text)
        self.assertTrue(changed)
        self.assertNotIn('CREDENTIAL_CANARY', safe)
        self.assertNotIn('PRIVATE_CANARY', safe)
        self.assertIn('BEGIN OPENSSH PRIVATE KEY', safe)
        self.assertEqual([len(x) for x in text.splitlines()], [len(x) for x in safe.splitlines()])
        self.assertEqual(text.splitlines()[-1], safe.splitlines()[-1])
        escaped = '-----BEGIN RSA PRIVATE KEY-----\\nESCAPED_CANARY\\n-----END RSA PRIVATE KEY-----'
        self.assertNotIn('ESCAPED_CANARY', sampling.mask_secrets(escaped)[0])
        self.assertEqual(escaped.count('\\n'), sampling.mask_secrets(escaped)[0].count('\\n'))
        self.assertNotIn('PLAIN_CANARY', sampling.mask_secrets('api_token: PLAIN_CANARY\n')[0])

    def test_native_finding_preserved_or_explicitly_redacted(self):
        native = dict(rule_id='RULE', category='PQC', description='An RSA reference.',
                      location=dict(file='a.py', line=1, column=3), evidence=dict(type='tree_sitter_match', metadata=dict(algorithm='RSA')))
        original = json.loads(json.dumps(native))
        self.assertEqual(sampling.safe_finding(native), original)
        self.assertEqual(native, original)
        self.assertEqual({x['fieldPath'] for x in sampling.assertion_checklist(native) if 'assertedValue' in x}, {'/rule_id', '/description', '/category', '/location', '/evidence'})
        native['source_snippet'] = dict(lines=[dict(line=1, text='-----BEGIN PRIVATE KEY-----'), dict(line=2, text='NATIVE_CANARY'), dict(line=3, text='-----END PRIVATE KEY-----')])
        self.assertNotIn('NATIVE_CANARY', json.dumps(sampling.safe_finding(native)))

    def test_exact_native_claim_draw_and_original_fields(self):
        args, probe = self.claim_fixture()
        output = self.base / 'out'
        result = sampling.build_packets(**args, output=output)
        self.assertEqual(result['claimCount'], 3)
        records = self.packets(output, 'claims')[0]['records']
        self.assertEqual([x['sampleId'] for x in records], [x['sampleId'] for x in probe['selected']])
        for record, selected in zip(records, probe['selected']):
            original = sampling.load_json(selected['reportPath'])['findings'][selected['nativeOrdinal']]
            self.assertEqual(record['nativeFinding'], original)
            self.assertEqual(record['findingSha256'], self.helpers.digest(original))
            self.assertEqual(record['nativeFinding']['location'], original['location'])
            self.assertEqual(record['asOf'], '2026-11-25T09:26:08Z')
            self.assertEqual(record['sourceReferences'][0]['status'], 'bound')
            self.assertEqual(json.loads((output / record['nativeFindingPrivatePath']).read_text()), original)
        self.assertTrue(self.packets(output, 'claims')[0]['provenance']['predictionsSeen'])

    def test_risk_snippet_and_nested_advice_are_required_native_obligations(self):
        extra = dict(risk='quantum-safe', source_snippet=dict(lines=[dict(line=2, text='rsa = object()')]),
                     migration_hint='Use API X, available only in version 7 on platform Y.',
                     recommended_actions=[dict(title='Replace RSA', rationale='API X requires provider Z.',
                                               steps=['If version 7 is installed, call API X.'],
                                               code_examples=[dict(condition='Only on platform Y with provider Z',
                                                                   code='api_x()')])])
        args, probe = self.claim_fixture(extra)
        output = self.base / 'out'
        sampling.build_packets(**args, output=output)
        records = sampling.load_json(output / 'claims/primary-all.json')['records']
        for record, selected in zip(records, probe['selected']):
            checks = {x['fieldPath']: x for x in record['assertionChecklist'] if 'assertedValue' in x}
            for key, value in extra.items():
                self.assertIn('/' + key, checks)
                self.assertEqual(checks['/' + key]['assertedValue'], value)
                self.assertEqual(record['nativeFinding'][key], value)
            self.assertIn('taxonomy', checks['/risk']['requirement'])
            self.assertIn('coordinates', checks['/source_snippet']['requirement'])
            self.assertIn('recursively', checks['/recommended_actions']['requirement'])
            self.assertIn('stated conditions', checks['/recommended_actions']['requirement'])
            self.assertIn('unasserted runtime execution', checks['/recommended_actions']['requirement'])
            original = sampling.load_json(selected['reportPath'])['findings'][selected['nativeOrdinal']]
            self.assertEqual(record['findingSha256'], self.helpers.digest(original))

    def test_imperative_advice_is_preserved_without_automatic_truth_label(self):
        finding = dict(migration_hint='Consider replacing this primitive.',
                       recommended_actions=[dict(title='Plan a migration', steps=['Choose a future algorithm.'])])
        checks = {x['fieldPath']: x for x in sampling.assertion_checklist(finding) if 'assertedValue' in x}
        for key, value in finding.items():
            self.assertEqual(checks['/' + key]['assertedValue'], value)
            self.assertIn('Exclude normative imperatives', checks['/' + key]['requirement'])
            self.assertIn('explicitly document', checks['/' + key]['requirement'])
            self.assertNotIn('verdict', checks['/' + key])

    def test_unclassified_native_fact_cannot_silently_pass(self):
        checks = sampling.assertion_checklist(dict(new_fact='API X is available on platform Y.', severity='high',
                                                   confidence=0.9, finding_id='opaque'))
        unknown = next(x for x in checks if x['id'] == 'unclassified-native-fields')
        self.assertEqual(unknown['undisposedFieldPaths'], ['/new_fact'])
        self.assertIn('unknown', unknown['requirement'])
        self.assertNotIn('assertedValue', unknown)

    def test_claim_rubric_addendum_binds_only_claims_and_rejects_wrong_protocol(self):
        args, probe = self.claim_fixture()
        args['comparison_claim_selection_path'] = self.write('comparison.json', sampling.freeze_comparison_claims(probe['invocation']['path'], self.helpers))
        addendum = dict(version=1, kind='claim-rubric-addendum', studyId='test',
                        baseProtocol=dict(path=str(args['protocol_path']), sha256=self.helpers.bytes_digest(args['protocol_path'])))
        path = self.write('addendum.json', addendum)
        args['claim_rubric_addendum_path'] = path
        output = self.base / 'out'
        sampling.build_packets(**args, output=output)
        packet = sampling.load_json(output / 'claims/primary-all.json')
        self.assertEqual(packet['scopeBinding']['claimRubricAddendum'],
                         dict(path=str(path), sha256=self.helpers.bytes_digest(path)))
        self.assertEqual(sampling.load_json(output / 'claims/comparison-all.json')['scopeBinding']['claimRubricAddendum'],
                         packet['scopeBinding']['claimRubricAddendum'])
        for packet in self.packets(output):
            self.assertNotIn('claimRubricAddendum', packet['scopeBinding'])
        addendum['baseProtocol']['sha256'] = 'f' * 64
        path.write_text(json.dumps(addendum))
        with self.assertRaisesRegex(ValueError, 'claim_rubric_addendum_identity_mismatch'):
            sampling.build_packets(**args, output=self.base / 'wrong-protocol')
        self.assertFalse((self.base / 'wrong-protocol').exists())

    def test_source_only_ignores_claim_rubric_addendum(self):
        args = self.fixture()
        first, second = self.base / 'out-a', self.base / 'out-b'
        sampling.build_packets(**args, output=first)
        sampling.build_packets(**args, output=second, claim_rubric_addendum_path=self.base / 'not-read.json')
        for path in (first / 'source').glob('*.json'):
            self.assertEqual(path.read_bytes(), (second / 'source' / path.name).read_bytes())

    def test_report_terminal_and_sample_drift_reject_before_output(self):
        args, probe = self.claim_fixture()
        selected = probe['selected'][0]
        report_path = pathlib.Path(selected['reportPath'])
        saved = report_path.read_bytes()
        report_path.write_bytes(saved + b' ')
        with self.assertRaisesRegex(ValueError, 'sha256_drift'):
            sampling.build_packets(**args, output=self.base / 'no-out-report')
        self.assertFalse((self.base / 'no-out-report').exists())
        report_path.write_bytes(saved)
        probe['selected'][0]['findingSha256'] = 'f' * 64
        args['claim_selection_path'].write_text(json.dumps(probe))
        with self.assertRaisesRegex(ValueError, 'native_finding_drift'):
            sampling.build_packets(**args, output=self.base / 'no-out-finding')
        self.assertFalse((self.base / 'no-out-finding').exists())

    def test_separate_comparison_controls_do_not_change_primary_draw(self):
        args, probe = self.claim_fixture()
        comparison = sampling.freeze_comparison_claims(probe['invocation']['path'], self.helpers)
        reports = sampling.validate_claim_frame(comparison, self.helpers, comparison=True)
        self.assertEqual(comparison['sampleCount'], 6)
        self.assertEqual(set(reports), {'repo-000', 'repo-001'})
        self.assertTrue(all(x['sampleId'].startswith('claim-comparison-') for x in comparison['selected']))
        self.assertEqual(probe['sampleCount'], 3)

    def test_empty_invalid_utf8_and_resource_limited_context_remain(self):
        args = self.fixture()
        registry = json.loads(args['registry_path'].read_text())
        root = pathlib.Path(registry['repositories'][0]['root'])
        manifest_path = pathlib.Path(registry['repositories'][0]['inputManifest'])
        items = json.loads(manifest_path.read_text())
        rows = json.loads(args['source_selection_path'].read_text())['selected']
        manifest = {x['path']: x for x in items}
        for raw, expected in [(b'\xff', 'undecodable_source'), (b'', 'empty_source'), (b' ' * (sampling.CONTEXT_LIMIT + 1) + b'rsa()', 'context_resource_limit')]:
            item = dict(items[0], bytes=len(raw), sha256=sampling.sha(raw))
            (root / item['path']).write_bytes(raw)
            row = dict(rows[0], sourceSha256=item['sha256'])
            record = sampling.source_record(row, 0, registry['repositories'][0], {item['path']: item}, {}, self.helpers)
            self.assertEqual(record['sourceStatus'], expected)
            self.assertIsNone(record['contextSafeText'])

    def test_unused_line_fallback_and_exhaustion_are_visible(self):
        args = self.fixture(1)
        repo = json.loads(args['registry_path'].read_text())['repositories'][0]
        raw = b'a=1\nb=2\nc=3\nd=4\n'
        path = pathlib.Path(repo['root']) / 'small.py'
        path.write_bytes(raw)
        item = dict(path='small.py', bytes=len(raw), sha256=sampling.sha(raw), mode='100644')
        prior = [dict(repositoryId=repo['id'], population='primary', group=repo['group'], split=repo['split'], sourcePath='small.py', sourceSha256=item['sha256'], language='Python', stratum='uniform_nonlexical_path')]
        extras = sampling.supplement_selections(prior, [repo], {repo['id']: {'small.py': item}}, {}, self.helpers)
        records = [sampling.source_record(row, i, repo, {'small.py': item}, {}, self.helpers) for i, row in enumerate(prior + extras)]
        self.assertEqual({r['ownedLine'] for r in records}, {1, 2, 3, 4})
        path.write_bytes(b'x')
        one = dict(item, bytes=1, sha256=sampling.sha(b'x'))
        prior[0]['sourceSha256'] = one['sha256']
        extras = sampling.supplement_selections(prior, [repo], {repo['id']: {'small.py': one}}, {}, self.helpers)
        self.assertEqual(sum(x['stratum'] == 'frame_exhausted' for x in extras), 3)

    def test_certificate_native_claim_record_string_time_and_static_facts(self):
        cert = self.base / 'cert.pem'; key = self.base / 'key.pem'
        subprocess.run(['/usr/bin/openssl', 'req', '-x509', '-newkey', 'rsa:2048', '-nodes', '-keyout', str(key),
                        '-out', str(cert), '-days', '1', '-subj', '/CN=STATIC_TEST'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=True)
        raw = cert.read_bytes()
        repo = dict(id='certificate-repo', population='primary', group='cert', languages=['Certificate'], split='holdout',
                    root=str(self.base), commit='a' * 40, inputManifestSha256='1' * 64)
        item = dict(path='cert.pem', bytes=len(raw), sha256=sampling.sha(raw), mode='100644')
        native = dict(rule_id='CERT_EXPIRED', location=dict(file='cert.pem', line=0, column=0),
                      evidence=dict(type='certificate', metadata=dict(assessment_time_unix='1795597205')))
        selected = dict(sampleId='native-cert-string', repositoryId=repo['id'], nativeOrdinal=0, populationOrdinal=0,
                        reportPath='/bound/report.json', reportSha256='2' * 64, findingSha256=self.helpers.digest(native))
        private = self.base / 'private'; private.mkdir()
        report = {'generated_at': '2099-01-01T00:00:00Z'}
        record = sampling.claim_record(selected, native, report, repo, {'cert.pem': item}, {}, self.helpers, private)
        metadata = record['staticMetadata']
        self.assertEqual(metadata['historicalAssessmentAt'], '2026-11-25T09:00:05+00:00')
        self.assertEqual(metadata['temporalVerdictEligibility'], 'timestamp_bound')
        before = datetime.datetime.strptime(metadata['notBefore'], '%b %d %H:%M:%S %Y GMT').replace(tzinfo=datetime.timezone.utc)
        after = datetime.datetime.strptime(metadata['notAfter'], '%b %d %H:%M:%S %Y GMT').replace(tzinfo=datetime.timezone.utc)
        instant = datetime.datetime.fromtimestamp(1795597205, datetime.timezone.utc)
        self.assertEqual(metadata['validAtAsOf'], before <= instant <= after)
        native['evidence']['metadata']['assessment_time_unix'] = None
        selected['sampleId'] = 'native-cert-timeless'
        timeless = sampling.claim_record(selected, native, report, repo, {'cert.pem': item}, {}, self.helpers, private)['staticMetadata']
        self.assertIsNone(timeless['historicalAssessmentAt'])
        self.assertIsNone(timeless['validAtAsOf'])
        self.assertEqual(timeless['keyBits'], metadata['keyBits'])
        self.assertEqual(timeless['certificateDerSha256'], metadata['certificateDerSha256'])

    def test_certificate_pem_der_metadata_and_no_target_execution(self):
        if not pathlib.Path('/usr/bin/openssl').is_file():
            self.skipTest('Installed OpenSSL unavailable')
        key, cert = self.base / 'key.pem', self.base / 'cert.pem'
        made = subprocess.run(['/usr/bin/openssl', 'req', '-x509', '-newkey', 'rsa:2048', '-nodes', '-keyout', str(key), '-out', str(cert), '-days', '1', '-subj', '/CN=NO_DN_IN_PACKET'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)
        self.assertEqual(made.returncode, 0)
        raw = cert.read_bytes()
        result = sampling.certificate_metadata(raw, '2020-01-01T00:00:00Z')
        self.assertEqual(result['status'], 'parsed')
        self.assertEqual(result['keyBits'], 2048)
        self.assertFalse(result['validAtAsOf'])
        timeless = sampling.certificate_metadata(raw, None)
        self.assertEqual(timeless['status'], 'parsed')
        self.assertEqual(timeless['keyBits'], 2048)
        self.assertEqual(timeless['certificateDerSha256'], result['certificateDerSha256'])
        self.assertIsNone(timeless['validAtAsOf'])
        self.assertNotIn('NO_DN_IN_PACKET', json.dumps(result))
        self.assertNotIn('modulus', json.dumps(result).lower())
        der = subprocess.run(['/usr/bin/openssl', 'x509', '-outform', 'DER'], input=raw, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, check=True).stdout
        self.assertEqual(sampling.certificate_metadata(der, '2020-01-01T00:00:00Z')['keyBits'], 2048)
        self.assertEqual(sampling.certificate_metadata(raw + raw, None)['status'], 'complex_or_private_bundle_unknown')
        marker = self.base / 'executed'
        script = b'#!/bin/sh\ntouch ' + str(marker).encode() + b'\n'
        self.assertEqual(sampling.certificate_metadata(script, None)['status'], 'unsupported_certificate_encoding')
        self.assertFalse(marker.exists())


if __name__ == '__main__':
    unittest.main()


class HistoricalTimestampTests(unittest.TestCase):
    def test_actual_native_string_timestamp_boundary(self):
        self.assertEqual(sampling.historical_assessment_at('1795597205'), '2026-11-25T09:00:05+00:00')
        self.assertEqual(sampling.historical_assessment_at(1795597205), sampling.historical_assessment_at('1795597205'))
        self.assertEqual(sampling.historical_assessment_at('-1'), '1969-12-31T23:59:59+00:00')
        for value in (None, True, False, 1.0, '1.0', '+1', '-0', '01', ' 1', '1\n', '١', '9' * 1000, 10 ** 100, -62135596801, 253402300800):
            with self.subTest(value=type(value).__name__):
                self.assertIsNone(sampling.historical_assessment_at(value))
