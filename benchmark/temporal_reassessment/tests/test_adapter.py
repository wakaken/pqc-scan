import copy
import hashlib
import json
import os
from pathlib import Path
from unittest.mock import patch

from test_execution import PrivateFixture
import adapter
import boundary as b
import capture_rustc as capture
import delta


class AdapterPlanTests(PrivateFixture):
    def product(self):
        product = b.mkdir(self.root / 'product')
        for name in ('crates', 'rules', 'schemas'):
            b.mkdir(product / name)
        for name in adapter.CHANGED_CRATES:
            crate = b.mkdir(product / 'crates' / name)
            b.mkdir(crate / 'src')
            (crate / 'Cargo.toml').write_bytes(b'synthetic manifest')
            (crate / 'src' / ('main.rs' if name == 'pqc_scan_cli' else 'lib.rs')).write_bytes(b'synthetic entry')
        for relative, (prefix, old, _) in adapter.REPLACEMENTS.items():
            (product / relative).write_bytes(b'UNCHANGED BEFORE\n' + prefix + old + b'\nUNCHANGED AFTER')
        for name in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml'):
            (product / name).write_bytes(b'synthetic source binding')
        (product / 'rules' / 'default').mkdir(mode=0o700)
        (product / 'rules' / 'default' / 'cert.yml').write_bytes(b'synthetic immutable rules')
        return product

    def envelope(self, product):
        source = b.source_bindings(product)
        compiler = b.ref(self.write('rustc', b'synthetic compiler identity; never executed'))
        cargo = b.ref(self.write('cargo', b'synthetic Cargo identity; never executed'))
        dependency = b.ref(self.write('dependency.rlib', b'synthetic dependency; never executed'))
        generated = b.ref(self.write('generated', b'synthetic generated input; never executed'))
        target = b.mkdir(self.root / 'exclusive-target')
        invocations = []
        for crate in adapter.CHANGED_CRATES:
            main = product / 'crates' / crate / 'src' / ('main.rs' if crate == 'pqc_scan_cli' else 'lib.rs')
            invocations.append(dict(crate=crate, features=[], sourceBindings=source, cwd=str(product),
                argv=[compiler['path'], '--crate-name', adapter.CRATE_NAMES[crate], str(main), '--out-dir', str(target), '--emit=link', '--extern', 'anyhow=' + dependency['path']],
                environment=dict(CARGO_MANIFEST_DIR=str(main.parent.parent)), dependencyArtifacts=[dependency],
                generatedInputs=[generated], linkSearchDirectories=[]))
        return dict(kind='shipping-overlay-cargo-envelope-v1', features=[], cwd=str(product), sourceBindings=source,
            compiler=compiler, cargo=cargo, shippingBinary=b.ref(self.write('shipping', b'synthetic shipping; never executed')),
            exclusiveTarget=str(target), exclusiveTargetHandoff=dict(target=str(target), finalRootValidationComplete=True, evidence=b.ref(self.write('handoff', b'synthetic handoff'))),
            environment=dict(PATH='/synthetic', CARGO_INCREMENTAL='0', CARGO_TARGET_DIR=str(target), CARGO_PROFILE_DEV_DEBUG='0', CARGO_PROFILE_TEST_DEBUG='0'), crateInvocations=invocations)

    def test_plan_copies_source_and_changes_exactly_two_expression_spans(self):
        product = self.product()
        envelope = self.envelope(product)
        destination = b.mkdir(self.root / 'adapter')
        before = b.source_bindings(product)
        with patch.object(b, 'ROOT', product):
            plan = adapter.build_plan(envelope, destination, before)
        self.assertEqual(before, b.source_bindings(product))
        self.assertFalse(plan['shippingBinaryEquivalent'])
        self.assertTrue(plan['unexecuted'])
        self.assertEqual(plan['clockEnvironment'], [adapter.DETECTOR_ENV, adapter.GENERATED_ENV])
        self.assertEqual(len(plan['sourceProof']), 2)
        self.assertIn('--locked', plan['command'])
        self.assertIn('--offline', plan['command'])
        overlay = Path(plan['cwd'])
        self.assertEqual(set(plan['sourceBindings']), set(plan['overlaySourceBindings']))
        for relative in before:
            original = (product / relative).read_bytes()
            actual = (overlay / relative).read_bytes()
            rewritten, span = adapter.overlay_bytes(relative, original)
            self.assertEqual(actual, rewritten)
            self.assertEqual(original == actual, span is None)
        self.assertIn('.with_timezone(&Utc)', (overlay / adapter.PIPELINE_PATH).read_text())
        self.assertNotIn('Utc::now()', (overlay / adapter.PIPELINE_PATH).read_text())
        self.assertEqual((overlay / adapter.DETECTOR_PATH).read_text().count('Some(second)'), 1)

    def test_actual_link_emit_triplet_is_preserved_and_non_link_or_clippy_rejected(self):
        product = self.product()
        envelope = self.envelope(product)
        for row in envelope['crateInvocations']:
            row['argv'][row['argv'].index('--emit=link')] = '--emit=dep-info,metadata,link'
        original = copy.deepcopy(envelope['crateInvocations'])
        with patch.object(b, 'ROOT', product):
            adapter.validate_invocations(envelope['crateInvocations'], envelope['compiler'], envelope['sourceBindings'])
        self.assertEqual(original, envelope['crateInvocations'])
        for extra in (['--emit=metadata'], ['--emit=dep-info,metadata'], ['--emit=dep-info'],
                      ['--emit=dep-info,metadata,link', '--cfg=clippy'], ['--emit=link', '--cfg', 'cargo-clippy']):
            bad = copy.deepcopy(original)
            argv = bad[0]['argv']
            argv[argv.index('--emit=dep-info,metadata,link'):argv.index('--emit=dep-info,metadata,link') + 1] = extra
            with patch.object(b, 'ROOT', product), self.subTest(extra=extra), self.assertRaises(b.Rejected):
                adapter.validate_invocations(bad, envelope['compiler'], envelope['sourceBindings'])

    def test_profile_zero_and_base_environment_are_preserved_in_capture_plan(self):
        product = self.product()
        envelope = self.envelope(product)
        envelope['actualShippingEnvironment'] = dict(envelope['environment'], RUSTC_WRAPPER='/historical/wrapper')
        with patch.object(b, 'ROOT', product):
            plan = adapter.build_plan(envelope, b.mkdir(self.root / 'adapter'), envelope['sourceBindings'])
            adapter.verify_capture_plan(plan, envelope)
        self.assertEqual(plan['baseEnvironment'], envelope['environment'])
        self.assertEqual(plan['profileEnvironment'], dict(CARGO_PROFILE_DEV_DEBUG='0', CARGO_PROFILE_TEST_DEBUG='0'))
        self.assertEqual(plan['environment']['RUSTC'], envelope['compiler']['path'])
        self.assertEqual(plan['environment']['RUSTC_WRAPPER'], plan['compilerCapture']['script']['path'])
        self.assertEqual(set(plan['environment']) - set(plan['baseEnvironment']), {'RUSTC', 'RUSTC_WRAPPER', capture.RECORDS_ENV})
        for key in adapter.PROFILE_KEYS:
            for value in (None, '2'):
                bad = copy.deepcopy(envelope)
                if value is None:
                    del bad['environment'][key]
                else:
                    bad['environment'][key] = value
                with self.subTest(key=key, value=value), self.assertRaises(b.Rejected):
                    adapter.cargo_environment(bad)
        bad = copy.deepcopy(envelope)
        bad['actualShippingEnvironment']['CARGO_PROFILE_DEV_DEBUG'] = '2'
        with self.assertRaises(b.Rejected):
            adapter.cargo_environment(bad)

    def test_capture_plan_rejects_changed_source_wrong_compiler_and_ambient_hooks(self):
        product = self.product()
        envelope = self.envelope(product)
        for hook in ('RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER', 'RUSTFLAGS', 'PYTHONPATH'):
            bad = copy.deepcopy(envelope)
            bad['environment'][hook] = '/ambient/hook'
            with self.subTest(hook=hook), self.assertRaises(b.Rejected):
                adapter.cargo_environment(bad)
        with patch.object(b, 'ROOT', product):
            plan = adapter.build_plan(envelope, b.mkdir(self.root / 'adapter'), envelope['sourceBindings'])
            for field, value in (('RUSTC', '/wrong/compiler'), (capture.RECORDS_ENV, str(self.root))):
                bad = copy.deepcopy(plan)
                bad['environment'][field] = value
                with self.subTest(field=field), self.assertRaises(b.Rejected):
                    adapter.verify_capture_plan(bad, envelope)
            source = Path(plan['compilerCapture']['script']['path'])
            source.chmod(0o600)
            source.write_bytes(source.read_bytes() + b'\n')
            with self.assertRaises(b.Rejected):
                adapter.verify_capture_plan(plan, envelope)

    def build_receipt(self):
        product = self.product()
        envelope = self.envelope(product)
        destination = b.mkdir(self.root / 'adapter')
        with patch.object(b, 'ROOT', product):
            plan = adapter.build_plan(envelope, destination, envelope['sourceBindings'])
        target = b.mkdir(Path(envelope['exclusiveTarget']) / 'debug')
        outputs = {}
        for crate in adapter.CHANGED_CRATES:
            name = 'pqc-scan' if crate == 'pqc_scan_cli' else 'lib' + crate + '.rlib'
            path = target / name
            path.write_bytes(('synthetic compiled ' + crate + '; never executed').encode())
            outputs[crate] = b.ref(path)
        rows = []
        for crate in adapter.CHANGED_CRATES:
            main = Path(plan['cwd']) / 'crates' / crate / 'src' / ('main.rs' if crate == 'pqc_scan_cli' else 'lib.rs')
            sources = [b.ref(main)]
            if crate == 'pqc_scan_core': sources.append(b.ref(Path(plan['cwd']) / adapter.PIPELINE_PATH))
            if crate == 'pqc_scan_detectors': sources.append(b.ref(Path(plan['cwd']) / adapter.DETECTOR_PATH))
            dependencies = [envelope['crateInvocations'][0]['dependencyArtifacts'][0]]
            if crate == 'pqc_scan_cli': dependencies += [outputs['pqc_scan_core'], outputs['pqc_scan_detectors']]
            argv = [envelope['compiler']['path'], '--crate-name', adapter.CRATE_NAMES[crate], str(main), '--emit=link']
            for index, dep in enumerate(dependencies):
                name = ('anyhow' if index == 0 else ('pqc_scan_core' if index == 1 else 'pqc_scan_detectors'))
                argv.extend(['--extern', name + '=' + dep['path']])
            depinfo = self.write(crate + '.d', (outputs[crate]['path'] + ': ' + ' '.join(item['path'] for item in sources) + '\n').encode())
            log = b.ref(self.write(crate + '.log', b'synthetic compiler log; never executed'))
            rows.append(dict(crate=crate, features=[], sourceBindings=plan['overlaySourceBindings'], cwd=plan['cwd'],
                argv=argv, environment=dict({key: value for key, value in plan['environment'].items() if key in capture.ENVIRONMENT_KEYS}, CARGO_MANIFEST_DIR=str(main.parent.parent)), dependencyArtifacts=dependencies,
                generatedInputs=[], linkSearchDirectories=[], exitCode=0, outputs=[outputs[crate]], depInfo=b.ref(depinfo),
                sourceInputs=sources, stdout=log, stderr=log))
        captures = []
        config = b.document(plan['compilerCapture']['configuration'])
        for index, row in enumerate(rows):
            raw = capture.invocation_record(config, plan['compilerCapture']['configuration']['sha256'], row['argv'], row['environment'], row['cwd'])
            row['rawCapture'] = b.publish(Path(plan['compilerCapture']['recordsDirectory']) / ('invocation-' + str(index) + '.json'), raw)
            captures.append(row['rawCapture'])
        smoke = b.publish(self.root / 'smoke.json', dict(kind='temporal-overlay-synthetic-cli-proof-v1', passed=True,
            adapterBinary=outputs['pqc_scan_cli'], sourceBindings=envelope['sourceBindings'],
            checks=['two_independent_clocks', 'nanosecond_serialization', 'invalid_pair_before_spawn', 'fixed_runtime_errors', 'native_assessment', 'references', 'required_formats']))
        receipt = dict(kind='execution-only-clock-adapter-build-receipt-v1', exitCode=0, sourceBindings=envelope['sourceBindings'],
            features=[], shippingBinaryEquivalent=False, smokeTestsPassed=True, shippingBinary=envelope['shippingBinary'],
            adapterBinary=outputs['pqc_scan_cli'], plan=b.publish(destination / 'plan.json', plan),
            command=plan['command'], environment=plan['environment'], cwd=plan['cwd'], crateInvocations=rows, compilerCaptures=captures,
            stdout=rows[0]['stdout'], stderr=rows[0]['stderr'], smokeEvidence=smoke)
        return product, plan, receipt

    def test_compiler_receipt_proves_actual_core_and_detector_archive_links_and_source_closure(self):
        product, plan, receipt = self.build_receipt()
        with patch.object(b, 'ROOT', product):
            self.assertEqual(adapter.verify_build(receipt, receipt['sourceBindings']), receipt)
        bad = copy.deepcopy(receipt)
        bad['crateInvocations'][1]['sourceInputs'].pop()
        with patch.object(b, 'ROOT', product), self.assertRaises(b.Rejected):
            adapter.verify_build(bad, receipt['sourceBindings'])
        bad = copy.deepcopy(receipt)
        bad['crateInvocations'][0]['argv'][-1] = 'pqc_scan_detectors=' + receipt['crateInvocations'][0]['dependencyArtifacts'][0]['path']
        with patch.object(b, 'ROOT', product), self.assertRaises(b.Rejected):
            adapter.verify_build(bad, receipt['sourceBindings'])

    def test_raw_capture_binding_profile_and_complete_record_set_are_required(self):
        product, plan, receipt = self.build_receipt()
        for mutation in ('argv', 'profile', 'outside', 'missing-record', 'unlisted-record'):
            bad = copy.deepcopy(receipt)
            if mutation == 'argv':
                bad['crateInvocations'][0]['argv'].append('--cfg=unrecorded')
            if mutation == 'profile':
                changed = b.document(receipt['plan'])
                changed['environment']['CARGO_PROFILE_DEV_DEBUG'] = '2'
                bad['environment'] = changed['environment']
                bad['plan'] = b.publish(self.root / 'changed-plan.json', changed)
            if mutation == 'outside':
                bad['crateInvocations'][0]['rawCapture'] = b.ref(self.write('outside.json', b'{}'))
            if mutation == 'missing-record':
                bad['compilerCaptures'].pop()
            extra = None
            if mutation == 'unlisted-record':
                extra = Path(plan['compilerCapture']['recordsDirectory']) / 'invocation-extra.json'
                extra.write_bytes(b'{}')
                extra.chmod(0o600)
            with patch.object(b, 'ROOT', product), self.subTest(mutation=mutation), self.assertRaises(b.Rejected):
                adapter.verify_build(bad, receipt['sourceBindings'])
            if extra:
                extra.unlink()

    def test_receipt_does_not_accept_an_extra_overlay_source_change(self):
        product, plan, receipt = self.build_receipt()
        (Path(plan['cwd']) / 'Cargo.toml').write_bytes(b'changed manifest')
        with patch.object(b, 'ROOT', product), self.assertRaises(b.Rejected):
            adapter.verify_build(receipt, receipt['sourceBindings'])

    def test_dep_info_unescapes_spaces_and_rejects_unbound_generated_input(self):
        first = self.write('a b.rs', b'synthetic space path')
        second = self.write('c.rs', b'synthetic include path')
        b.mkdir(self.root / 'sub')
        escaped = str(first).replace(' ', '\\ ')
        depinfo = b.ref(self.write('spaces.d', ('/output: ' + escaped + ' \\\n ' + str(self.root / 'sub' / '..' / second.name) + '\n' + escaped + ':\n').encode()))
        self.assertEqual(adapter.dep_info_inputs(depinfo, str(self.root)), {str(first), str(second)})
        product, plan, receipt = self.build_receipt()
        generated = b.ref(self.write('unbound-generated.rs', b'synthetic generated source'))
        row = receipt['crateInvocations'][0]
        row['sourceInputs'].append(generated)
        Path(row['depInfo']['path']).write_bytes((row['outputs'][0]['path'] + ': ' + ' '.join(item['path'] for item in row['sourceInputs']) + '\n').encode())
        row['depInfo'] = b.ref(Path(row['depInfo']['path']))
        with patch.object(b, 'ROOT', product), self.assertRaises(b.Rejected):
            adapter.verify_build(receipt, receipt['sourceBindings'])

    def test_old_main_only_facade_and_missing_changed_crate_are_rejected(self):
        product = self.product()
        envelope = self.envelope(product)
        for change in ('old', 'missing', 'features', 'environment', 'handoff'):
            bad = copy.deepcopy(envelope)
            if change == 'old': bad['kind'] = 'shipping-cli-rustc-envelope-v1'
            if change == 'missing': bad['crateInvocations'].pop()
            if change == 'features': bad['features'] = ['probe-tls']
            if change == 'environment': bad['environment']['RUSTFLAGS'] = '--cfg unsafe_override'
            if change == 'handoff': bad['exclusiveTargetHandoff']['finalRootValidationComplete'] = False
            destination = b.mkdir(self.root / ('case-' + change))
            with patch.object(b, 'ROOT', product), self.subTest(change=change), self.assertRaises(b.Rejected):
                adapter.build_plan(bad, destination, envelope['sourceBindings'])
            self.assertFalse((destination / 'overlay').exists())

    def test_changed_dependency_generated_input_or_source_is_rejected(self):
        product = self.product()
        envelope = self.envelope(product)
        dependency = Path(envelope['crateInvocations'][0]['dependencyArtifacts'][0]['path'])
        dependency.write_bytes(b'changed')
        with patch.object(b, 'ROOT', product), self.assertRaises(b.Rejected):
            adapter.build_plan(envelope, b.mkdir(self.root / 'adapter'), envelope['sourceBindings'])

    def test_ambiguous_or_missing_expression_anchor_is_rejected(self):
        for path, (prefix, old, _) in adapter.REPLACEMENTS.items():
            for raw in (b'no expression', (prefix + old) * 2):
                with self.subTest(path=path), self.assertRaises(b.Rejected):
                    adapter.overlay_bytes(path, raw)

    def test_runtime_blocks_decode_without_a_real_clock_fallback(self):
        self.assertNotIn('SystemTime', adapter.DETECTOR_BLOCK)
        self.assertNotIn('Utc::now', adapter.GENERATED_BLOCK)
        self.assertIn('E_REPLAY_DETECTOR_DECODE', adapter.DETECTOR_BLOCK)
        self.assertIn('E_REPLAY_GENERATED_DECODE', adapter.GENERATED_BLOCK)
        self.assertNotIn('253402300799', adapter.DETECTOR_BLOCK)
        self.assertNotIn('timestamp', adapter.GENERATED_BLOCK)
        self.assertIn('DateTime::parse_from_rfc3339', adapter.GENERATED_BLOCK)

    def test_parent_source_and_history_write_scope_are_rejected(self):
        with self.assertRaises(b.Rejected):
            b.create(b.ROOT / 'forbidden-new-record')
        with self.assertRaises(b.Rejected):
            b.mkdir(self.root / '..' / '..' / 'outside')


class NativeRemovalTests(PrivateFixture):
    def fixture(self):
        evidence = b.ref(self.write('proof', b'synthetic independently reviewed source proof'))
        finding = dict(findingId='false-legacy', reason='confirmed_csharp_predefined_type_false_positive', independentlyApproved=True,
                       astNodeKind='predefined_type', sourceSha256='a' * 64, ruleId='CS_TLS',
                       location=dict(file='a.cs', line=1, column=1), evidence=evidence)
        observation = dict(id='false-obs', producer='tree_sitter_detector', method='ast_pattern', asset_ids=['shared-algorithm'],
                           target=dict(path='a.cs', line=1, byte_start=0, byte_end=3))
        native = dict(assets=[dict(id='shared-algorithm', relations=[])], observations=[observation],
                      legacy_associations=[dict(legacy_id='false-legacy', legacy_index=0, observation_ids=['false-obs'])],
                      unassociated_observation_ids=[], legacy_loss_records=[])
        removal = dict(collection='observations', id='false-obs', findingId='false-legacy', reason=finding['reason'],
                       independentlyApproved=True, astNodeKind='predefined_type', evidence=evidence,
                       originalEntryDigest=b.digest(observation), sourcePath='a.cs', byteStart=0, byteEnd=3)
        return native, removal, finding, {'a.cs': dict(sha256='a' * 64, bytes=10)}

    def test_confirmed_ast_position_removal_keeps_shared_assets_and_legacy_integrity(self):
        native, removal, finding, manifest = self.fixture()
        normalized = delta.remove_native_false_positives(native, [removal], [finding], manifest, [])
        self.assertEqual(normalized['observations'], [])
        self.assertEqual(normalized['legacy_associations'], [])
        self.assertEqual(normalized['assets'], native['assets'])

    def test_wrong_source_span_and_changed_unapproved_producer_rejected(self):
        native, removal, finding, manifest = self.fixture()
        removal['byteStart'] = 1
        with self.assertRaises(b.Rejected):
            delta.remove_native_false_positives(native, [removal], [finding], manifest, [])
        removal['byteStart'] = 0; native['observations'][0]['producer'] = 'certificate_detector'
        removal['originalEntryDigest'] = b.digest(native['observations'][0])
        with self.assertRaises(b.Rejected):
            delta.remove_native_false_positives(native, [removal], [finding], manifest, [])

    def test_removing_asset_still_referenced_by_other_observation_rejected(self):
        native, removal, finding, manifest = self.fixture()
        native['observations'].append(dict(id='kept-obs', asset_ids=['shared-algorithm']))
        asset_removal = dict(removal, collection='assets', id='shared-algorithm', originalEntryDigest=b.digest(native['assets'][0]))
        with self.assertRaises(b.Rejected):
            delta.remove_native_false_positives(native, [removal, asset_removal], [finding], manifest, [])
