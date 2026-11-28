import copy
import json
import subprocess
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2]/'benchmark'))
from audit_outputs import sarif_locations, legacy_cbom_locations, projection_checks


class OutputAuditTests(unittest.TestCase):
    def fixture(self, file='src/a b.rs', line=4, column=7, uri='src/a%20b.rs'):
        findings = [dict(rule_id='RULE', location=dict(file=file, line=line, column=column))]
        result = dict(ruleId='RULE', locations=[dict(physicalLocation=dict(artifactLocation=dict(uri=uri), region=dict(startLine=line, startColumn=column)))])
        return findings, dict(runs=[dict(results=[result])])

    def test_escaped_and_absolute_paths_and_column_abstention_are_valid(self):
        for file, uri in [('src/a b.rs', 'src/a%20b.rs'), ('/tmp/a b.rs', 'file:///tmp/a%20b.rs')]:
            findings, sarif = self.fixture(file=file, uri=uri)
            self.assertEqual(sarif_locations(findings, sarif), 0)
            del sarif['runs'][0]['results'][0]['locations'][0]['physicalLocation']['region']['startColumn']
            self.assertEqual(sarif_locations(findings, sarif), 1)

    def test_unknown_region_and_unknown_file_keep_explicit_zero_position(self):
        findings, sarif = self.fixture(line=0, column=0)
        del sarif['runs'][0]['results'][0]['locations'][0]['physicalLocation']['region']
        self.assertEqual(sarif_locations(findings, sarif), 1)
        findings[0]['location'].update(file='', line=3, column=2)
        sarif['runs'][0]['results'][0]['locations'] = []
        self.assertEqual(sarif_locations(findings, sarif), 1)

    def cbom_fixture(self, **overrides):
        finding = dict(rule_id='RULE', location=dict(file='source.rs', line=2, column=3),
                       evidence=dict(type='regex_match', match='RSA', metadata={}),
                       category='PQC', risk='quantum-safe', migration_hint='Check reference')
        finding.update(overrides)
        row = dict(location='source.rs:2', usage_type='PQC', algorithm=finding['evidence']['match'],
                   quantum_risk='quantum-safe', migration_hint='Check reference')
        return finding, row

    def test_legacy_cbom_coalesces_only_repeated_builder_identity(self):
        finding, row = self.cbom_fixture()
        repeated = copy.deepcopy(finding)
        repeated['location']['column'] += 1
        counts = legacy_cbom_locations([finding, repeated], [row])
        self.assertEqual(counts['legacyCbomCoalesced'], 1)
        self.assertEqual(counts['legacyCbomUncertainGroups'], 0)
        with self.assertRaisesRegex(AssertionError, 'legacy_cbom'):
            legacy_cbom_locations([finding, repeated], [row, copy.deepcopy(row)])

    def test_legacy_cbom_rejects_missing_unique_locations_and_empty_exports(self):
        finding, row = self.cbom_fixture()
        other = copy.deepcopy(finding)
        other['location']['file'] = 'other.rs'
        rows = [row, dict(row, location='other.rs:2')]
        self.assertEqual(legacy_cbom_locations([finding, other], rows)['legacyCbomCoalesced'], 0)
        for invalid in [[row], []]:
            with self.subTest(rows=len(invalid)), self.assertRaisesRegex(AssertionError, 'legacy_cbom'):
                legacy_cbom_locations([finding, other], invalid)
        self.assertEqual(legacy_cbom_locations([], [])['legacyCbomCoalesced'], 0)

    def test_legacy_cbom_preserves_distinct_rules_and_matches_at_one_location(self):
        finding, row = self.cbom_fixture()
        other_rule = dict(finding, rule_id='OTHER')
        other_match = dict(finding, evidence=dict(type='regex_match', match='ECDSA', metadata={}))
        findings = [finding, other_rule, other_match]
        rows = [row, copy.deepcopy(row), dict(row, algorithm='ECDSA')]
        self.assertEqual(legacy_cbom_locations(findings, rows)['legacyCbomCoalesced'], 0)
        for invalid in [rows[:1], rows[:2], rows + [copy.deepcopy(row)],
                        [dict(row, location='missing.rs:2')], [dict(row, algorithm='unrelated')]*3]:
            with self.subTest(rows=len(invalid)), self.assertRaisesRegex(AssertionError, 'legacy_cbom'):
                legacy_cbom_locations(findings, invalid)

    def test_legacy_cbom_builder_keeps_first_projection_for_a_repeated_identity(self):
        finding, row = self.cbom_fixture()
        later = dict(finding, migration_hint='A later occurrence has different presentation')
        self.assertEqual(legacy_cbom_locations([finding, later], [row])['legacyCbomCoalesced'], 1)
        with self.assertRaisesRegex(AssertionError, 'legacy_cbom'):
            legacy_cbom_locations([finding, later], [dict(row, migration_hint=later['migration_hint'])])

    def test_legacy_cbom_delimiters_cannot_hide_distinct_references(self):
        first, first_row = self.cbom_fixture(evidence=dict(type='regex_match', match='part.rs:3:RSA', metadata={}))
        second = copy.deepcopy(first)
        second['location'].update(file='source.rs:2:part.rs', line=3)
        second['evidence']['match'] = 'RSA'
        second_row = dict(first_row, location='source.rs:2:part.rs:3', algorithm='RSA')
        self.assertEqual(legacy_cbom_locations([first, second], [first_row, second_row])['legacyCbomCoalesced'], 0)
        with self.assertRaisesRegex(AssertionError, 'legacy_cbom'):
            legacy_cbom_locations([first, second], [first_row])
        first['evidence']['match'] = 'source.rs:2:RSA'
        first_row['algorithm'] = 'source.rs:2:RSA'
        second['location'] = copy.deepcopy(first['location'])
        second['rule_id'] = 'RULE:source.rs:2'
        second_row.update(location='source.rs:2', algorithm='RSA')
        self.assertEqual(legacy_cbom_locations([first, second], [first_row, second_row])['legacyCbomCoalesced'], 0)

    def test_legacy_cbom_uncertain_group_still_deduplicates_known_identities(self):
        finding, row = self.cbom_fixture()
        masked = dict(finding, evidence=dict(type='private_key', match='[masked-sensitive-content]', metadata={}))
        findings = [finding, copy.deepcopy(finding), masked]
        self.assertEqual(legacy_cbom_locations(findings, [row])['legacyCbomUncertainGroups'], 1)
        self.assertEqual(legacy_cbom_locations(findings, [row, copy.deepcopy(row)])['legacyCbomUncertainGroups'], 1)
        with self.assertRaisesRegex(AssertionError, 'legacy_cbom'):
            legacy_cbom_locations(findings, [row]*3)
        distinct = dict(finding, evidence=dict(type='regex_match', match='ECDSA', metadata={}))
        with self.assertRaisesRegex(AssertionError, 'legacy_cbom'):
            legacy_cbom_locations([finding, distinct, masked], [row])

    def test_legacy_cbom_redaction_keeps_coverage_bounds_and_explicit_uncertainty(self):
        finding, row = self.cbom_fixture(evidence=dict(type='private_key', match='[masked-sensitive-content]', metadata={}))
        findings = [finding, copy.deepcopy(finding)]
        for rows in [[row], [row, copy.deepcopy(row)]]:
            counts = legacy_cbom_locations(findings, rows)
            self.assertEqual(counts['legacyCbomCoalesced'], 0)
            self.assertEqual(counts['legacyCbomUncertainGroups'], 1)
            self.assertEqual(counts['legacyCbomUnresolvedFindingRows'], len(findings)-len(rows))
        for invalid in [[], [row]*3]:
            with self.assertRaisesRegex(AssertionError, 'legacy_cbom'):
                legacy_cbom_locations(findings, invalid)
        with self.assertRaisesRegex(AssertionError, 'legacy_cbom'):
            legacy_cbom_locations([finding, dict(finding, rule_id='OTHER')], [row])

    def test_legacy_cbom_source_display_does_not_invent_original_match_identity(self):
        finding, row = self.cbom_fixture(evidence=dict(type='tree_sitter_match', match='public call()', metadata=dict(node_kind='bounded_source_fact')))
        row['algorithm'] = '[masked-sensitive-content]'
        findings = [finding, copy.deepcopy(finding)]
        for rows in [[row], [row, copy.deepcopy(row)]]:
            self.assertEqual(legacy_cbom_locations(findings, rows)['legacyCbomUncertainGroups'], 1)

    def test_projection_rejects_consistently_missing_legacy_arrays(self):
        _, sarif = self.fixture(file='source.rs', line=2, column=3, uri='source.rs')
        finding, row = self.cbom_fixture()
        finding.update(finding_id='one', severity='info')
        report = dict(findings=[finding], cbom=[row], dependency_sbom=[],
                      summary=dict(total_findings=1, by_severity={'info': 1}, by_risk={'quantum-safe': 1}))
        native = dict(assets=[], observations=[], legacy_associations=[])
        loss = dict(counts={}, notes=['observations_are_properties_not_complete_native_records'])
        document = dict(components=[], properties=[dict(name='pqc-scan:export-loss', value=json.dumps(loss))])
        self.assertEqual(projection_checks(report, native, sarif, [row], [], document, document)['findings'], 1)
        report['cbom'] = []
        with self.assertRaisesRegex(AssertionError, 'legacy_cbom'):
            projection_checks(report, native, sarif, [], [], document, document)

    def test_optimized_python_cannot_disable_audit_checks(self):
        script = Path(__file__).resolve().parents[2]/'benchmark/audit_outputs.py'
        run = subprocess.run([sys.executable, '-O', str(script), '--help'], capture_output=True, text=True)
        self.assertNotEqual(run.returncode, 0)
        self.assertIn('audit_requires_python_assertions', run.stderr)

    def test_wrong_present_column_and_missing_or_duplicate_findings_are_rejected(self):
        findings, sarif = self.fixture()
        for mutation in ['column', 'missing', 'duplicate']:
            invalid = copy.deepcopy(sarif)
            results = invalid['runs'][0]['results']
            if mutation == 'column':
                results[0]['locations'][0]['physicalLocation']['region']['startColumn'] += 1
            elif mutation == 'missing':
                results.clear()
            else:
                results.append(copy.deepcopy(results[0]))
            with self.subTest(mutation=mutation), self.assertRaises(AssertionError):
                sarif_locations(findings, invalid)
