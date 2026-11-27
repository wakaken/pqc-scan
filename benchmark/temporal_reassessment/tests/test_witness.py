import copy
import hashlib
import json
import struct
from pathlib import Path
from unittest.mock import patch

import test_adapter
from test_execution import PrivateFixture
import adapter
import boundary as b


class WitnessConfigurationTests(PrivateFixture):
    def setUp(self):
        super().setUp()
        self.product = test_adapter.AdapterPlanTests.product(self)
        self.destination = b.mkdir(self.root / 'witness')
        self.source = b.source_bindings(self.product)
        self.bindings = {key: 'a' * 64 for key in adapter.WITNESS_SCHEMA['bindingKeys']}

    def configure(self, **changes):
        args = dict(root=self.product, selection=dict(include_non_scannable=False, exclude_tests=False),
                    detector_second=1793491200, bindings=self.bindings, final_source=self.source)
        args.update(changes)
        with patch.object(b, 'ROOT', self.product):
            return adapter.witness_configuration(self.destination, **args)

    def test_original_rules_are_copied_byte_identically_and_only_one_proof_rule_added(self):
        before = b.source_bindings(self.product)
        result = self.configure()
        config = b.document(result['configuration'])
        self.assertEqual(b.source_bindings(self.product), before)
        self.assertFalse(result['actualRunAdmitted'])
        self.assertEqual(result['compileSmoke'], 'PENDING')
        self.assertEqual(config['max_file_size_bytes'], 2097152)
        self.assertEqual(config['threads'], 1)
        self.assertEqual(config['witness_rule_id'], adapter.WITNESS_ID)
        rules = Path(config['rules_dir'])
        self.assertEqual((rules / 'cert.yml').read_bytes(), (self.product / 'rules/default/cert.yml').read_bytes())
        self.assertEqual((rules / 'temporal-private-witness.yml').read_bytes(), adapter.WITNESS_RULE)
        self.assertEqual(set(p.name for p in rules.iterdir()), {'cert.yml', 'temporal-private-witness.yml'})
        self.assertLessEqual(result['configuration']['bytes'], 65536)
        self.assertEqual(result['source']['sha256'], b.ref(Path(adapter.__file__).parent / 'temporal_witness.rs')['sha256'])

    def test_selection_clock_hash_and_output_path_boundaries(self):
        cases = [dict(detector_second=True), dict(detector_second=-1), dict(detector_second=9223372036854775808),
                 dict(selection=dict(include_non_scannable=1, exclude_tests=False)),
                 dict(selection=dict(include_non_scannable=False)), dict(bindings={'free_metadata': 'secret'}),
                 dict(output_name='../outside.json'), dict(output_name='witness-config.json')]
        for changes in cases:
            with self.subTest(changes=changes), self.assertRaises(b.Rejected):
                self.configure(**changes)
            self.assertFalse((self.destination / 'proof-rules').exists())

    def test_config_public_schema_excludes_findings_and_free_metadata(self):
        self.assertEqual(set(adapter.WITNESS_SCHEMA['objectKeys']), {
            'path', 'sourceSha256', 'certificateSha256', 'objectIndex', 'notBeforeUnix', 'notAfterUnix',
            'validityState', 'matchingDefaultValidityRuleIds'})
        self.assertNotIn('evidence', adapter.WITNESS_SCHEMA['topKeys'])
        self.assertNotIn('proofComplete', adapter.WITNESS_SCHEMA['topKeys'])

    def test_harness_hash_binds_the_rust_producer(self):
        bindings = b.harness_bindings()
        source = str(Path(adapter.__file__).parent / 'temporal_witness.rs')
        self.assertEqual(bindings[source], b.ref(source)['sha256'])

    def test_witness_source_uses_raw_product_detection_and_pipeline_before_lossy_conversion(self):
        source = (Path(adapter.__file__).parent / 'temporal_witness.rs').read_text()
        self.assertRegex(source, r'CertificateDetector::at\(\s*config.detector_second,?\s*\)')
        self.assertIn('scan_repository_with_scope', source)
        self.assertIn('outcome.detections()', source)
        self.assertIn('DetectorOutcome::new(Vec::new(), status)', source)
        self.assertNotIn('PublicCertificateFacts', source.split('#[cfg(test)]')[0])
        self.assertNotIn('parse_x509', source)
        self.assertNotIn('parse_pem', source)
        self.assertNotIn('snippet_preview', source.split('#[cfg(test)]')[0])
        self.assertNotIn('r#match', source)
        self.assertIn('O_NOFOLLOW', source)
        self.assertIn('O_EXCL', source)
        self.assertIn('E_WITNESS_PRODUCER_LOSS', source)
        self.assertIn('deny_unknown_fields', source)

    def test_storage_directory_is_separate_from_bound_code_and_product(self):
        before = b.harness_bindings()
        with patch.object(b, 'OWN', self.root), patch.object(b, 'ROOT', self.product):
            self.assertEqual(b.require_external_storage(), self.root)
        self.assertEqual(before, b.harness_bindings())
        with patch.object(b, 'OWN', self.product), patch.object(b, 'ROOT', self.product), self.assertRaises(b.Rejected):
            b.require_external_storage()

    def helper_envelope(self):
        envelope = test_adapter.AdapterPlanTests.envelope(self, self.product)
        dependencies = {name: b.ref(self.write(name + '.rlib', ('synthetic ' + name + '; never executed').encode()))
                        for name in adapter.WITNESS_EXTERNS}
        for row in envelope['crateInvocations']:
            row['argv'] = row['argv'][:-2]
            for name, binding in dependencies.items():
                row['argv'].extend(['--extern', name + '=' + binding['path']])
            row['dependencyArtifacts'] = list(dependencies.values())
        return envelope

    def paired_envelope(self, *, macho=False):
        envelope = self.helper_envelope()
        metadata = b'rust synthetic compiled-library identity; never executed'
        body = metadata
        if macho:
            header = struct.pack('<8I', 0xfeedfacf, 0, 0, 1, 1, 152, 0, 0)
            segment = struct.pack('<II16sQQQQIIII', 25, 152, b'', 0, 0, 0, 0, 0, 0, 1, 0)
            section = struct.pack('<16s16sQQIIIIIIII', b'.rmeta', b'__DWARF', 0, len(metadata), 184, 0, 0, 0, 0, 0, 0, 0)
            body = header + segment + section + metadata
        name = b'lib.rmeta\0\0\0'
        body = name + body
        header = b'#1/12'.ljust(16) + b'0'.ljust(12) + b'0'.ljust(6) + b'0'.ljust(6) + b'100644'.ljust(8) + str(len(body)).encode().ljust(10) + b'`\n'
        library_directory = self.root / 'paired-deps'
        library_directory.mkdir(mode=0o700, exist_ok=True)
        library = library_directory / 'libpqc_scan_core-synthetic.rlib'
        library.write_bytes(b'!<arch>\n' + header + body + (b'\n' if len(body) % 2 else b''))
        meta = library.with_suffix('.rmeta')
        meta.write_bytes(metadata)
        directory = adapter.directory_binding(library_directory)
        for index, row in enumerate(envelope['crateInvocations']):
            for position, arg in enumerate(row['argv'][:-1]):
                if arg == '--extern' and row['argv'][position + 1].startswith('pqc_scan_core='):
                    row['argv'][position + 1] = 'pqc_scan_core=' + str(library if index == 0 else meta)
            row['dependencyArtifacts'] = [item for item in row['dependencyArtifacts'] if not Path(item['path']).name.startswith('pqc_scan_core.')]
            row['dependencyArtifacts'].append(b.ref(library if index == 0 else meta))
            row['argv'].extend(['-L', 'dependency=' + str(library_directory)])
            row['linkSearchDirectories'] = [directory]
        return envelope, library, meta

    def test_pipelined_rmeta_links_bound_rlib_with_identical_metadata(self):
        envelope, library, meta = self.paired_envelope(macho=True)
        with patch.object(b, 'ROOT', self.product):
            plan = adapter.witness_build_plan(envelope, self.destination, self.source)
        self.assertIn('pqc_scan_core=' + str(library), plan['command'])
        self.assertNotIn('pqc_scan_core=' + str(meta), plan['command'])
        self.assertIn(b.ref(library), plan['dependencyArtifacts'])
        self.assertEqual(plan['metadataPairs'], [dict(crate='pqc_scan_core', linkArtifact=b.ref(library), metadataArtifact=b.ref(meta),
                         metadataSHA256=hashlib.sha256(meta.read_bytes()).hexdigest(), metadataBytes=meta.stat().st_size)])

    def test_metadata_only_extern_resolves_link_artifact_from_frozen_search_snapshot(self):
        envelope, library, meta = self.paired_envelope()
        for row in envelope['crateInvocations']:
            for index, arg in enumerate(row['argv'][:-1]):
                if arg == '--extern' and row['argv'][index + 1].startswith('pqc_scan_core='):
                    row['argv'][index + 1] = 'pqc_scan_core=' + str(meta)
            row['dependencyArtifacts'] = [item for item in row['dependencyArtifacts'] if item['path'] != str(library)]
            if b.ref(meta) not in row['dependencyArtifacts']:
                row['dependencyArtifacts'].append(b.ref(meta))
        with patch.object(b, 'ROOT', self.product):
            plan = adapter.witness_build_plan(envelope, self.destination, self.source)
        self.assertIn('pqc_scan_core=' + str(library), plan['command'])

    def test_paired_metadata_rejects_other_parent_stem_and_actual_identity(self):
        original, library, meta = self.paired_envelope()
        library_bytes, metadata_bytes = library.read_bytes(), meta.read_bytes()
        for variant in ('parent', 'stem', 'metadata', 'archive', 'unbound'):
            with self.subTest(variant=variant):
                envelope = copy.deepcopy(original)
                library.write_bytes(library_bytes)
                meta.write_bytes(metadata_bytes)
                other = self.write('other-core.rmeta', meta.read_bytes())
                if variant == 'parent':
                    directory = self.root / 'elsewhere'
                    directory.mkdir(mode=0o700)
                    other = directory / meta.name
                    other.write_bytes(meta.read_bytes())
                if variant in ('parent', 'stem'):
                    row = envelope['crateInvocations'][1]
                    row['argv'] = [arg.replace(str(meta), str(other)) for arg in row['argv']]
                    row['dependencyArtifacts'] = [b.ref(other) if item['path'] == str(meta) else item for item in row['dependencyArtifacts']]
                elif variant == 'metadata':
                    meta.write_bytes(b'rust different actual library identity')
                    for row in envelope['crateInvocations']:
                        row['dependencyArtifacts'] = [b.ref(meta) if item['path'] == str(meta) else item for item in row['dependencyArtifacts']]
                elif variant == 'archive':
                    library.write_bytes(b'not a metadata-bearing archive')
                    for row in envelope['crateInvocations']:
                        row['dependencyArtifacts'] = [b.ref(library) if item['path'] == str(library) else item for item in row['dependencyArtifacts']]
                else:
                    for row in envelope['crateInvocations']:
                        row['argv'] = [arg.replace(str(library), str(meta)) for arg in row['argv']]
                        row['dependencyArtifacts'] = [b.ref(meta) if item['path'] == str(library) else item for item in row['dependencyArtifacts']]
                snapshot = adapter.directory_binding(library.parent)
                if variant == 'unbound':
                    del snapshot['files'][library.name]
                for row in envelope['crateInvocations']:
                    row['linkSearchDirectories'] = [snapshot]
                with patch.object(b, 'ROOT', self.product), self.assertRaises(b.Rejected):
                    adapter.witness_build_plan(envelope, self.destination, self.source)
                other.unlink(missing_ok=True)
                if variant == 'parent':
                    directory.rmdir()

    def test_paired_receipt_rederives_identity_proof_and_rejects_forgery(self):
        envelope, library, meta = self.paired_envelope(macho=True)
        plan, receipt = self.helper_receipt(envelope)
        with patch.object(b, 'ROOT', self.product):
            self.assertEqual(adapter.verify_witness_build(receipt, self.source), receipt)
            forged = copy.deepcopy(plan)
            forged['metadataPairs'][0]['metadataSHA256'] = '0' * 64
            bad = dict(receipt, plan=b.publish(self.destination / 'forged-plan.json', forged))
            with self.assertRaises(b.Rejected):
                adapter.verify_witness_build(bad, self.source)
        meta.write_bytes(b'rust changed metadata after build')
        with patch.object(b, 'ROOT', self.product), self.assertRaises(b.Rejected):
            adapter.verify_witness_build(receipt, self.source)

    def test_archive_metadata_parser_rejects_truncation_and_duplicate_members(self):
        envelope, library, meta = self.paired_envelope()
        raw = library.read_bytes()
        self.assertEqual(adapter.rlib_metadata(raw), meta.read_bytes())
        for invalid in (raw[:-2], raw + raw[8:], b'!<arch>\n' + b'x' * 60):
            with self.subTest(size=len(invalid)), self.assertRaises(b.Rejected):
                adapter.rlib_metadata(invalid)

    def test_helper_plan_links_exact_first_party_libraries_and_remains_unexecuted(self):
        envelope = self.helper_envelope()
        with patch.object(b, 'ROOT', self.product):
            plan = adapter.witness_build_plan(envelope, self.destination, self.source)
        self.assertTrue(plan['unexecuted'])
        self.assertFalse(plan['actualRunAdmitted'])
        self.assertEqual(plan['compiler'], envelope['compiler'])
        self.assertEqual(plan['environment']['PQC_TEMPORAL_PRODUCT_SOURCE'], str(self.product))
        self.assertEqual(len(plan['dependencyArtifacts']), len(adapter.WITNESS_EXTERNS))
        self.assertIn('--test', plan['testCommand'])
        self.assertNotIn('--test', plan['command'])
        self.assertEqual(plan['source'], b.ref(b.CODE_DIR / 'temporal_witness.rs'))

    def test_missing_first_party_dependency_rejects_helper_build_plan(self):
        envelope = self.helper_envelope()
        for row in envelope['crateInvocations']:
            index = next(index for index, arg in enumerate(row['argv'][:-1]) if arg == '--extern' and row['argv'][index + 1].startswith('serde='))
            del row['argv'][index:index + 2]
        with patch.object(b, 'ROOT', self.product), self.assertRaises(b.Rejected):
            adapter.witness_build_plan(envelope, self.destination, self.source)
        self.assertFalse((self.destination / 'build').exists())

    def helper_receipt(self, envelope=None):
        envelope = self.helper_envelope() if envelope is None else envelope
        with patch.object(b, 'ROOT', self.product):
            plan = adapter.witness_build_plan(envelope, self.destination, self.source)
        rows = []
        for command_key, directory_key in [('command', 'outputDirectory'), ('testCommand', 'testOutputDirectory')]:
            output = Path(plan[directory_key]) / 'temporal_witness'
            output.write_bytes(('synthetic ' + command_key + '; never executed').encode())
            depinfo = self.write(command_key + '.d', (str(output) + ': ' + plan['source']['path'] + '\n').encode())
            log = b.ref(self.write(command_key + '.log', b'synthetic log; never executed'))
            rows.append(dict(argv=plan[command_key], environment=plan['environment'], cwd=plan['cwd'], exitCode=0,
                             sourceInputs=[plan['source']], outputs=[b.ref(output)], depInfo=b.ref(depinfo), stdout=log, stderr=log))
        receipt = dict(kind='temporal-producer-witness-build-receipt-v1', exitCode=0, sourceBindings=self.source,
                       plan=b.publish(self.destination / 'plan.json', plan), binary=rows[0]['outputs'][0],
                       compilerInvocation=rows[0], testCompilerInvocation=rows[1],
                       syntheticTests=dict(exitCode=0, command=[rows[1]['outputs'][0]['path'], '--test-threads=1'],
                                           environment=plan['environment'], stdout=rows[1]['stdout'], stderr=rows[1]['stderr']))
        return plan, receipt

    def test_helper_receipt_binds_source_dependencies_and_all_tests_command(self):
        plan, receipt = self.helper_receipt()
        with patch.object(b, 'ROOT', self.product):
            self.assertEqual(adapter.verify_witness_build(receipt, self.source), receipt)
        bad = copy.deepcopy(receipt)
        bad['syntheticTests']['command'].append('--list')
        with patch.object(b, 'ROOT', self.product), self.assertRaises(b.Rejected):
            adapter.verify_witness_build(bad, self.source)
        bad = copy.deepcopy(receipt)
        bad['compilerInvocation']['sourceInputs'] = []
        with patch.object(b, 'ROOT', self.product), self.assertRaises(b.Rejected):
            adapter.verify_witness_build(bad, self.source)

    def test_helper_receipt_rejects_post_build_dependency_changes(self):
        plan, receipt = self.helper_receipt()
        Path(plan['dependencyArtifacts'][0]['path']).write_bytes(b'changed library')
        with patch.object(b, 'ROOT', self.product), self.assertRaises(b.Rejected):
            adapter.verify_witness_build(receipt, self.source)
