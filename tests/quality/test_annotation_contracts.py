import copy
import hashlib
import importlib.util
import json
import pathlib
import subprocess
import sys
import tempfile
import unittest


TOOL = pathlib.Path(__file__).resolve().parents[2] / 'benchmark/annotation_contracts.py'
SPEC = importlib.util.spec_from_file_location('annotation_contracts', TOOL)
contracts = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(contracts)


def save(path, value):
    path.write_text(json.dumps(value), encoding='utf-8')


def binding(path):
    return {'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}


class AnnotationContractsTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='pqc-annotation-contracts-')
        self.addCleanup(self.temp.cleanup)
        self.root = pathlib.Path(self.temp.name).resolve()
        self.source = self.root / 'example.py'
        self.source.write_text('result = padding.PKCS1v15()\n', encoding='utf-8')
        self.routing = self.root / 'routing.txt'
        self.routing.write_text('Reviewed input routing and full-callee source anchor contract.')
        self.unit = {'id': 'u1', 'path': 'repo/example.py', 'ruleId': 'RSA',
                     'line': 1, 'startColumn': 18, 'endColumn': 26}
        self.corpus = {'version': 1, 'families': {'source': ['RSA']},
                       'files': [{'path': 'repo/example.py', 'source': str(self.source),
                                  'root': str(self.root), 'sourceRelativePath': 'example.py',
                                  'sha256': binding(self.source)['sha256']}],
                       'units': [self.unit]}
        self.truth = {'version': 1, 'labels': [{'id': 'u1', 'label': 'positive'}]}
        self.audit = {'rows': [{'id': 'u1', 'frozenUnit': copy.deepcopy(self.unit),
                               'originalLabel': 'positive',
                               'sourceFactPreserved': 'A padding constructor call.'}]}
        self.sidecar = {'version': 1, 'sourceContractBindings': [binding(self.routing)],
                        'exposure': {'predictionsSeen': True, 'sameModelFamily': True,
                                     'limitations': 'Prediction-exposed, bounded anchor audit; no independence claim.'},
                        'units': [{'id': 'u1', 'originalLabel': 'positive',
                                   'sourceFact': 'A padding constructor call.',
                                   'supportedInput': True, 'expectedAnchorColumn': 10,
                                   'sourceContractPaths': [str(self.routing)]}]}
        self.corpus_path = self.root / 'corpus.json'
        self.truth_path = self.root / 'truth.json'
        self.audit_path = self.root / 'audit.json'
        self.sidecar_path = self.root / 'contracts.json'

    def run_check(self):
        save(self.corpus_path, self.corpus)
        self.truth['corpusSha256'] = binding(self.corpus_path)['sha256']
        save(self.truth_path, self.truth)
        save(self.audit_path, self.audit)
        self.sidecar.update(corpus=binding(self.corpus_path), truth=binding(self.truth_path),
                            audit=binding(self.audit_path), checker=binding(TOOL))
        save(self.sidecar_path, self.sidecar)
        return contracts.check(self.corpus_path, self.truth_path, self.sidecar_path)

    def set_interval(self, start, end):
        self.unit.update(startColumn=start, endColumn=end)
        self.audit['rows'][0]['frozenUnit'] = copy.deepcopy(self.unit)

    def test_terminal_callee_rejected_and_original_positive_preserved(self):
        result = self.run_check()
        row = result['rows'][0]
        self.assertEqual(row['evaluability'], 'not-evaluable')
        self.assertEqual(row['reasons'], ['anchor_outside_interval'])
        self.assertEqual(row['originalLabel'], 'positive')
        self.assertEqual(row['originalUnit'], self.unit)
        self.assertEqual(row['sourceFact'], 'A padding constructor call.')
        self.assertNotIn('accuracy', result)

    def test_full_callee_control(self):
        self.set_interval(10, 26)
        self.assertEqual(self.run_check()['rows'][0]['evaluability'], 'evaluable')

    def test_unicode_callee_contract_uses_codepoints(self):
        self.source.write_text('結果 = 暗号.PKCS1v15()\n', encoding='utf-8')
        self.corpus['files'][0]['sha256'] = binding(self.source)['sha256']
        self.sidecar['units'][0]['expectedAnchorColumn'] = 6
        self.set_interval(9, 17)
        self.assertEqual(self.run_check()['rows'][0]['reasons'], ['anchor_outside_interval'])
        self.set_interval(6, 17)
        self.assertEqual(self.run_check()['rows'][0]['evaluability'], 'evaluable')

    def test_unsupported_pyproject_and_dependency_column_one(self):
        self.source.write_text('    "cryptography>=3.4"\n')
        self.corpus['files'][0]['sha256'] = binding(self.source)['sha256']
        self.set_interval(6, 18)
        self.sidecar['units'][0].update(supportedInput=False, expectedAnchorColumn=1)
        self.assertEqual(self.run_check()['rows'][0]['reasons'],
                         ['unsupported_input', 'anchor_outside_interval'])
        self.sidecar['units'][0]['supportedInput'] = True
        self.assertEqual(self.run_check()['rows'][0]['reasons'], ['anchor_outside_interval'])
        self.set_interval(1, 18)
        self.assertEqual(self.run_check()['rows'][0]['evaluability'], 'evaluable')

    def test_supported_negative_uses_same_contract(self):
        self.truth['labels'][0]['label'] = 'negative'
        self.audit['rows'][0]['originalLabel'] = 'negative'
        self.sidecar['units'][0]['originalLabel'] = 'negative'
        self.set_interval(1, 26)
        self.sidecar['units'][0]['expectedAnchorColumn'] = 1
        self.assertEqual(self.run_check()['rows'][0]['evaluability'], 'evaluable')
        self.set_interval(2, 26)
        self.assertEqual(self.run_check()['rows'][0]['evaluability'], 'not-evaluable')

    def test_original_ineligible_stays_excluded_without_contract(self):
        self.truth['labels'][0]['label'] = 'unsupported'
        self.audit['rows'][0]['originalLabel'] = 'unsupported'
        self.sidecar['units'] = []
        self.assertEqual(self.run_check()['rows'][0]['evaluability'], 'unchanged-ineligible')

    def test_missing_duplicate_extra_contracts_fail_closed(self):
        original = copy.deepcopy(self.sidecar['units'])
        for units in ([], original * 2, original + [dict(original[0], id='extra')]):
            with self.subTest(units=units):
                self.sidecar['units'] = units
                with self.assertRaises(ValueError):
                    self.run_check()

    def test_bad_schema_columns_and_labels_rejected(self):
        original = copy.deepcopy(self.sidecar)
        for column in (None, True, 0, -1, '10', 999):
            self.sidecar = copy.deepcopy(original)
            self.sidecar['units'][0]['expectedAnchorColumn'] = column
            with self.subTest(column=column), self.assertRaises(ValueError):
                self.run_check()
        for key, value in (('supportedInput', 1), ('originalLabel', 'negative'),
                           ('sourceFact', 'altered fact'), ('sourceContractPaths', [])):
            self.sidecar = copy.deepcopy(original)
            self.sidecar['units'][0][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                self.run_check()
        self.sidecar = copy.deepcopy(original)
        self.sidecar['version'] = 2
        with self.assertRaises(ValueError):
            self.run_check()

    def test_source_and_contract_hash_drift_rejected(self):
        self.run_check()
        self.routing.write_text('changed')
        with self.assertRaises(ValueError):
            contracts.check(self.corpus_path, self.truth_path, self.sidecar_path)
        self.sidecar['sourceContractBindings'] = [binding(self.routing)]
        self.run_check()
        self.source.write_text('changed source')
        with self.assertRaises(ValueError):
            contracts.check(self.corpus_path, self.truth_path, self.sidecar_path)

    def test_bound_inputs_checker_and_audit_drift_rejected(self):
        self.run_check()
        for key in ('corpus', 'truth', 'checker', 'audit'):
            value = copy.deepcopy(self.sidecar)
            value[key]['sha256'] = '0' * 64
            save(self.sidecar_path, value)
            with self.subTest(key=key), self.assertRaises(ValueError):
                contracts.check(self.corpus_path, self.truth_path, self.sidecar_path)

    def test_incomplete_truth_and_unbound_source_contract_rejected(self):
        self.truth['labels'] = []
        with self.assertRaises(ValueError):
            self.run_check()
        self.truth['labels'] = [{'id': 'u1', 'label': 'positive'}]
        self.sidecar['units'][0]['sourceContractPaths'] = [str(self.root / 'unbound')]
        with self.assertRaises(ValueError):
            self.run_check()

    def test_bad_source_coordinates_and_duplicate_input_ids_rejected(self):
        original = copy.deepcopy(self.unit)
        for key, value in (('line', True), ('line', 0), ('line', 2),
                           ('startColumn', True), ('startColumn', 0),
                           ('endColumn', 999), ('endColumn', 18)):
            self.unit.clear()
            self.unit.update(original)
            self.unit[key] = value
            self.audit['rows'][0]['frozenUnit'] = copy.deepcopy(self.unit)
            with self.subTest(key=key, value=value), self.assertRaises(ValueError):
                self.run_check()
        self.unit.clear()
        self.unit.update(original)
        self.audit['rows'][0]['frozenUnit'] = copy.deepcopy(self.unit)
        self.corpus['units'].append(copy.deepcopy(self.unit))
        with self.assertRaises(ValueError):
            self.run_check()

    def test_source_root_escape_and_symlink_rejected(self):
        self.corpus['files'][0]['sourceRelativePath'] = '../example.py'
        with self.assertRaises(ValueError):
            self.run_check()
        self.corpus['files'][0]['sourceRelativePath'] = 'alias.py'
        alias = self.root / 'alias.py'
        alias.symlink_to(self.source)
        self.corpus['files'][0]['source'] = str(alias)
        with self.assertRaises(ValueError):
            self.run_check()

    def test_cli_success_and_missing_contract_fail_without_receipt(self):
        self.set_interval(10, 26)
        self.run_check()
        output = self.root / 'success.json'
        command = [sys.executable, str(TOOL), '--corpus', str(self.corpus_path),
                   '--truth', str(self.truth_path), '--contracts', str(self.sidecar_path),
                   '--out', str(output)]
        success = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(success.returncode, 0, success.stderr)
        self.assertEqual(json.loads(output.read_text())['rejectedUnitIds'], [])
        self.sidecar_path.unlink()
        command[-1] = str(self.root / 'must-not-exist.json')
        failure = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(failure.returncode, 2)
        self.assertFalse((self.root / 'must-not-exist.json').exists())

    def test_cli_rejects_existing_output_and_reports_rejection(self):
        self.run_check()
        output = self.root / 'result.json'
        command = [sys.executable, str(TOOL), '--corpus', str(self.corpus_path),
                   '--truth', str(self.truth_path), '--contracts', str(self.sidecar_path),
                   '--out', str(output)]
        first = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(first.returncode, 1, first.stderr)
        saved = output.read_bytes()
        self.assertEqual(json.loads(saved)['rejectedUnitIds'], ['u1'])
        second = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(second.returncode, 2)
        self.assertEqual(output.read_bytes(), saved)


if __name__ == '__main__':
    unittest.main()
