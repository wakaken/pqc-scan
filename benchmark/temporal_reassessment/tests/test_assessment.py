import copy
import json
from pathlib import Path
import sys
import unittest

from test_execution import PrivateFixture
from test_clocks import observed_row
import boundary as b
import execution
import storage


def synthetic_outputs(output):
    coverage = {name: 0 for name in ('read_failed_files', 'partial_files', 'analysis_failed_files', 'unsupported_files',
                'not_applicable_files', 'unknown_files', 'enumeration_failed_entries', 'excluded_enumerated_files')}
    coverage.update(selected_files=1, read_ok_files=1, analyzed_files=1, not_enumerated_population=True, status='complete_within_scope')
    native = dict(schema_version=1, identity_scheme='pqc-scan-length-prefixed-sha256-v1', scanner_version='0.4.0', catalog_version='synthetic',
                  profile='default', rules_digest='rules-v1-' + 'a' * 64,
                  run=dict(id='scan-v1-' + 'b' * 64, attempt_id='synthetic', generated_at='2026-11-01T00:00:04+00:00'),
                  repository_scope=None, scope=dict(max_file_size_bytes=2097152, git_ignore=True, git_global=True, git_exclude=True,
                  default_excludes=['.git', 'node_modules', 'target', 'dist', 'docs'], markdown_excluded=True),
                  coverage=coverage, diagnostics=[], native_completeness='complete_within_scope',
                  assets=[], observations=[], legacy_associations=[], unassociated_observation_ids=[], legacy_loss_records=[],
                  assessment_status='not_evaluated', assessments=[], export_loss_notes=[])
    loss = json.dumps(dict(counts={}, notes=['observations_are_properties_not_complete_native_records']))
    cdx = dict(bomFormat='CycloneDX', specVersion='1.7', version=1, components=[], properties=[dict(name='pqc-scan:export-loss', value=loss)])
    values = {'report.json': dict(generated_at='2026-11-01T00:00:04Z', findings=[], cbom=[], dependency_sbom=[],
              summary=dict(total_findings=0, by_severity={}, by_risk={})), 'scan.inventory.v1.json': native,
              'diagnostics.json': dict(coverage=coverage, diagnostics=[]), 'cbom.json': [], 'dependency-sbom.json': [],
              'cbom.cdx-1.7.json': cdx, 'dependency-sbom.cdx-1.7.json': copy.deepcopy(cdx),
              'report.sarif': dict(version='2.1.0', runs=[dict(tool=dict(driver=dict(name='public synthetic')), results=[])])}
    for name in b.FORMATS:
        if name in values:
            (output / name).write_text(json.dumps(values[name]))
        else:
            (output / name).write_text('<p>Public synthetic generated report</p>' if name.endswith('html') else '# Public synthetic report\n')
    return values


class AssessorReuseTests(PrivateFixture):
    def setUp(self):
        super().setUp()
        _, _, audit = b.import_auditors()
        self.validation, self.schemas = audit.validators()
        self.output = b.mkdir(self.root / 'output')
        self.values = synthetic_outputs(self.output)

    def test_existing_assessor_and_offline_schema_reference_audits_are_used(self):
        artifacts, assessment = execution.assess(self.output, {}, self.validation)
        self.assertEqual(len(artifacts), 11)
        self.assertEqual(assessment['descriptive']['findingCount'], 0)
        self.assertEqual(assessment['descriptive']['inspection'], 'Complete')
        self.assertIsNone(assessment['descriptive']['accuracy'])
        self.assertEqual(assessment['outputAudit']['schemaErrors'], [])
        self.assertEqual(assessment['outputAudit']['counts']['assets'], 0)
        self.assertTrue(assessment['outputAudit']['passed'])

    def test_coverage_inconsistency_is_not_complete(self):
        self.values['diagnostics.json']['coverage']['analyzed_files'] = 0
        (self.output / 'diagnostics.json').write_text(json.dumps(self.values['diagnostics.json']))
        with self.assertRaises(ValueError):
            execution.assess(self.output, {}, self.validation)

    def test_schema_unknown_field_rejected_without_external_schema_fetch(self):
        self.values['scan.inventory.v1.json']['invented_field'] = True
        (self.output / 'scan.inventory.v1.json').write_text(json.dumps(self.values['scan.inventory.v1.json']))
        with self.assertRaises(b.Rejected):
            execution.assess(self.output, {}, self.validation)

    def test_schema_failure_records_safe_codes_counts_and_withheld_dynamic_keys(self):
        native = self.values['scan.inventory.v1.json']
        native['invented-private-field-value'] = 'synthetic-private-instance-value'
        (self.output / 'scan.inventory.v1.json').write_text(json.dumps(native))
        with self.assertRaises(execution.AssessmentRejected) as raised:
            execution.assess(self.output, {}, self.validation)
        record = raised.exception.evidence
        self.assertEqual(record['schemaValidation'], 'Failed')
        self.assertGreater(record['schemaErrorsReportedCount'], 0)
        self.assertEqual(record['nativeSourceReferences'], 'NotRun')
        self.assertNotIn('synthetic-private-instance-value', json.dumps(record))
        self.assertNotIn('invented-private-field-value', json.dumps(record))
        sanitized = execution.safe_schema_errors([dict(file='report.sarif', validator='type',
                    path=['runs', 0, 'synthetic-private-dynamic-key'])], self.validation)
        self.assertEqual(sanitized[0]['path'][-1], '<withheld-field>')

    def test_failed_output_diagnostics_are_bound_without_claiming_audit_pass(self):
        available = execution.available_artifacts(self.output)
        self.assertEqual(available['state'], 'Present')
        self.assertEqual(available['known']['diagnostics.json']['state'], 'BoundNotAudited')
        self.assertEqual(available['unexpectedEntryCount'], 0)
        self.assertNotIn('coverage', available['known']['diagnostics.json'])

    def test_dangling_native_reference_rejected(self):
        native = self.values['scan.inventory.v1.json']
        native['unassociated_observation_ids'] = ['obs-v1-' + 'f' * 64]
        (self.output / 'scan.inventory.v1.json').write_text(json.dumps(native))
        with self.assertRaises((b.Rejected, AssertionError)):
            execution.assess(self.output, {}, self.validation)

    def test_resume_reassesses_compressed_bytes_and_rejects_changed_assessment(self):
        artifacts, assessment = execution.assess(self.output, {}, self.validation)
        compressed = b.mkdir(self.root / 'compressed')
        archived = storage.archive_outputs(self.output, compressed, artifacts)
        terminal = dict(artifacts=archived, assessment=assessment,
                        bindings=dict(input=dict(limitations={}), clock=observed_row()))
        execution.unpack_assess(terminal, self.root / 'check-original', self.validation)
        terminal['assessment']['descriptive']['findingCount'] = 900
        with self.assertRaises(b.Rejected):
            execution.unpack_assess(terminal, self.root / 'check-modified', self.validation)
        storage.validate_matrix(archived)
        self.assertFalse((self.root / 'check-original').exists())
        self.assertFalse((self.root / 'check-modified').exists())

    def test_authenticated_reader_checks_entire_matrix_before_json(self):
        artifacts, _ = execution.assess(self.output, {}, self.validation)
        compressed = b.mkdir(self.root / 'compressed')
        archived = storage.archive_outputs(self.output, compressed, artifacts)
        self.assertEqual(storage.read_document(archived, 'report.json')['findings'], [])
        (compressed / 'report.rules.html.gz').write_bytes(b'corrupted')
        with self.assertRaises(b.Rejected):
            storage.read_document(archived, 'report.json')


class SourceAdmissionTests(PrivateFixture):
    def setUp(self):
        super().setUp()
        self.oss, _, _ = b.import_auditors()
        self.source = b.mkdir(self.root / 'source')
        self.path = self.source / 'a.rs'; self.path.write_text('public synthetic source\n'); self.path.chmod(0o644)
        self.files = [dict(path='a.rs', bytes=self.path.stat().st_size, sha256=b.ref(self.path)['sha256'], mode='100644')]
        self.manifest = self.write('manifest.json', json.dumps(dict(commit='a' * 40, files=self.files)).encode())
        self.row = dict(id='synthetic', root=str(self.source), commit='a' * 40, scope='tracked synthetic tree',
                        inputManifest=str(self.manifest), inputManifestSha256=b.ref(self.manifest)['sha256'])

    def test_identical_relocated_input_has_explicit_manifest_binding(self):
        root, files, binding = self.oss._acquisition(self.row, self.root / 'new-output')
        self.assertEqual(root, self.source)
        self.assertEqual(files, self.files)
        self.assertEqual(binding['commit'], 'a' * 40)
        self.assertEqual(binding['manifest']['sha256'], b.ref(self.manifest)['sha256'])

    def test_changed_source_pin_manifest_or_executable_mode_rejected(self):
        changed = dict(self.row, commit='b' * 40)
        with self.assertRaises(ValueError):
            self.oss._acquisition(changed, self.root / 'new-output')
        self.path.chmod(0o755)
        with self.assertRaises(ValueError):
            self.oss._acquisition(self.row, self.root / 'new-output')
        self.path.chmod(0o644); self.path.write_text('different source bytes\n')
        with self.assertRaises(ValueError):
            self.oss._acquisition(self.row, self.root / 'new-output')

    def test_input_output_overlap_rejected_before_any_scan(self):
        with self.assertRaises(ValueError):
            self.oss._acquisition(self.row, self.source / 'output')

    def test_unsafe_manifest_name_is_rejected(self):
        self.files[0]['path'] = '../outside.rs'
        self.manifest.write_text(json.dumps(dict(commit='a' * 40, files=self.files)))
        self.row['inputManifestSha256'] = b.ref(self.manifest)['sha256']
        with self.assertRaises(ValueError):
            self.oss._acquisition(self.row, self.root / 'new-output')


if __name__ == '__main__':
    unittest.main()
