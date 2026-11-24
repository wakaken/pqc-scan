"""Infrastructure controls are separate from the24-case product population."""
import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from migration071 import reference_integrity, summarize, validate_manifest, verify_freeze


def population():
    return dict(cases=[dict(id=f'C{i}{member}', pair=f'P{i}', group=f'G{i}',
                           split='tuning' if i < 4 else 'final', format='inventory-v1')
                      for i in range(12) for member in 'AB'])


class StudyInfrastructure(unittest.TestCase):
    def test_complete_pair_allocation_and_related_family_leakage(self):
        data = population()
        self.assertEqual(len(validate_manifest(data)), 24)
        data['cases'][8]['group'] = data['cases'][0]['group']
        with self.assertRaises(ValueError):
            validate_manifest(data)

    def test_missing_duplicate_and_split_pair_reject(self):
        for mutation in ('missing', 'duplicate', 'split'):
            data = population()
            if mutation == 'missing':
                data['cases'].pop()
            elif mutation == 'duplicate':
                data['cases'][1]['id'] = data['cases'][0]['id']
            else:
                data['cases'][1]['split'] = 'final'
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                validate_manifest(data)

    def test_frozen_source_and_input_bytes_cannot_drift(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root/'input.json').write_bytes(b'{"fact":false}')
            freeze = dict(frozen_at='before', bindings={
                'input.json': hashlib.sha256((root/'input.json').read_bytes()).hexdigest()})
            verify_freeze(root, freeze)
            (root/'input.json').write_bytes(b'{"fact":true}')
            with self.assertRaises(ValueError):
                verify_freeze(root, freeze)
            freeze['bindings'] = {}
            with self.assertRaises(ValueError):
                verify_freeze(root, freeze)

    def test_missing_extra_duplicate_observations_fail_before_aggregation(self):
        data = population()
        oracle = dict(cases=[dict(id=r['id'], checks=[dict(pointer='/value',
                      operation='equals', expected=False, dimension='technical')])
                      for r in data['cases']], unassessed=[])
        observations = [dict(id=r['id'], actual={'value':False}, repeated={'value':False})
                        for r in data['cases']]
        for bad in [observations[:-1], observations+[dict(id='EXTRA', actual={})],
                    observations[:-1]+[observations[0]]]:
            with self.assertRaises(ValueError):
                summarize(data, oracle, bad)

    def test_failures_and_unassessed_are_not_success_or_human_measurement(self):
        data = population()
        oracle = dict(cases=[dict(id=r['id'], checks=[dict(pointer='/value',
                      operation='equals', expected=False, dimension='technical')])
                      for r in data['cases']], unassessed=[{'status':'Not Tested'}])
        observations = [dict(id=r['id'], actual={'value':False}, repeated={'value':False})
                        for r in data['cases']]
        observations[0]['actual']['value'] = True
        result = summarize(data, oracle, observations)
        self.assertEqual(result['combined']['incorrect'], 1)
        self.assertEqual(result['combined']['correct'], 23)
        self.assertEqual(result['dimensions']['technical']['properties'],
                         {'correct':23, 'incorrect':1, 'eligible':24})
        self.assertEqual(result['categorical_strata']['role']['not_declared']['evaluated'], 24)
        self.assertEqual(result['categorical_strata']['role']['not_declared']
                         ['dimensions']['technical']['properties']['incorrect'], 1)
        self.assertEqual(result['splits']['final']['dimensions']['technical']['correct'], 16)
        self.assertEqual(result['splits']['final']['dimensions']['conformance']['evaluated'], 0)
        self.assertEqual(result['dimensions']['conformance']['unassessed'], 24)
        self.assertEqual(result['reproducibility']['equal'], 0)
        self.assertEqual(result['human']['observed'], 0)
        self.assertIsNone(result['human']['rate'])
        self.assertIsNone(result['numerical_validity']['conclusion'])
        self.assertEqual(len(result['unassessed']), 1)

    def test_complete_semantic_repetition_differs_from_byte_order(self):
        data = population()
        oracle = dict(cases=[dict(id=r['id'], checks=[]) for r in data['cases']], unassessed=[])
        observations = [dict(id=r['id'], actual={'format':'pqc-scan.migration-assessment',
                            'x':False}, repeated={'x':False,
                            'format':'pqc-scan.migration-assessment'}) for r in data['cases']]
        self.assertEqual(summarize(data, oracle, observations)['reproducibility']['equal'], 24)
        observations[0]['repeated']['x'] = 0
        self.assertEqual(summarize(data, oracle, observations)['reproducibility']['equal'], 23)

    def test_influential_claim_source_target_and_candidate_joins(self):
        report = dict(targets=[{'id':'t'}], claims=[{'id':'c'}], sources=[{'id':'s'}],
                      candidates=[dict(id='a', target_id='t', source_ids=['s'],
                                       prerequisites=[{'applied_claim_ids':['c']}])],
                      decisions=[dict(target_id='t', applied_claim_ids=['c'], source_ids=['s'],
                                      action={'candidate_ids':['a']})])
        self.assertTrue(reference_integrity(report))
        for field in ('target_id', 'applied_claim_ids', 'source_ids'):
            bad = copy.deepcopy(report)
            bad['decisions'][0][field] = 'missing' if field == 'target_id' else ['missing']
            self.assertFalse(reference_integrity(bad))
        bad = copy.deepcopy(report)
        bad['candidates'][0]['prerequisites'][0]['applied_claim_ids'] = ['missing']
        self.assertFalse(reference_integrity(bad))

    def test_primary_reason_is_not_merely_present_later(self):
        data = population()
        oracle = dict(cases=[dict(id=r['id'], checks=[dict(pointer='/reason_codes/0',
                      operation='equals', expected='required_primary', dimension='conformance')])
                      for r in data['cases']], unassessed=[])
        observations = [dict(id=r['id'], actual={'reason_codes':['required_primary']},
                             repeated={}) for r in data['cases']]
        observations[0]['actual']['reason_codes'] = ['wrong_primary', 'required_primary']
        result = summarize(data, oracle, observations)
        self.assertEqual(result['combined']['incorrect'], 1)
        self.assertEqual(result['dimensions']['conformance']['properties']['incorrect'], 1)


if __name__ == '__main__':
    unittest.main()
