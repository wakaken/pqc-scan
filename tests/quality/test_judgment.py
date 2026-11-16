"""Hand-derived independent controls for judgment comparison, not matcher truth."""
import unittest
import judgment

class JudgmentTests(unittest.TestCase):
    def test_equal_wrong_unknown_and_characterization_have_separate_denominators(self):
        oracle={'cases':[{'id':'right','checks':[{'pointer':'/purpose','operation':'equals','expected':'signature'}]},
                         {'id':'wrong','checks':[{'pointer':'/purpose','operation':'equals','expected':'key_establishment'}]},
                         {'id':'unassessed','checks':[]}]}
        observed=[{'id':'right','actual':{'purpose':'signature'}},{'id':'wrong','actual':{'purpose':'signature'}},
                  {'id':'unassessed','actual':{'purpose':'unknown'}}]
        r=judgment.compare(oracle,observed)
        self.assertEqual((r['correct'],r['incorrect'],r['unassessed'],r['evaluated']),(1,1,1,2))
        self.assertEqual(r['mismatches'][0]['id'],'wrong')
    def test_missing_duplicate_and_extra_observations_are_rejected(self):
        oracle={'cases':[{'id':'one','checks':[]}]}
        for rows in [[],[{'id':'one','actual':{}},{'id':'one','actual':{}}],[{'id':'other','actual':{}}]]:
            with self.assertRaises(ValueError):judgment.compare(oracle,rows)
    def test_missing_property_is_incorrect_not_null_equivalence(self):
        oracle={'cases':[{'id':'a','checks':[{'pointer':'/missing','operation':'equals','expected':None}]}]}
        r=judgment.compare(oracle,[{'id':'a','actual':{}}]);self.assertEqual(r['incorrect'],1)
    def test_float_transport_tolerance_does_not_accept_wrong_judgment(self):
        oracle={'cases':[{'id':'a','checks':[{'pointer':'/v','operation':'approximately','expected':0.4,'tolerance':1e-7}]}]}
        self.assertEqual(judgment.compare(oracle,[{'id':'a','actual':{'v':0.40000000596}}])['correct'],1)
        self.assertEqual(judgment.compare(oracle,[{'id':'a','actual':{'v':0.41}}])['incorrect'],1)
    def test_empty_gold_has_no_manufactured_perfect_rate(self):
        r=judgment.compare({'cases':[]},[]);self.assertIsNone(r['correctnessRate']);self.assertEqual(r['evaluated'],0)

class SupplementalControls(unittest.TestCase):
    def test_reassessment_requires_identical_input_and_one_unassessed_original(self):
        original={'id':'old','input':{'version':None},'checks':[]}
        replacement={'id':'new','reassesses':'old','input':{'version':None},
            'checks':[{'pointer':'/ok','operation':'equals','expected':True}]}
        self.assertEqual([c['id'] for c in judgment.canonical_cases([original,replacement])],['new'])
        changed=dict(replacement,input={'version':'24'})
        for rows in [[original,changed],[original,replacement,dict(replacement,id='second')],
                     [dict(original,checks=replacement['checks']),replacement],[replacement],
                     [original,dict(replacement,checks=[])]]:
            with self.assertRaises(ValueError):judgment.canonical_cases(rows)

    def test_absence_and_exclusion_do_not_treat_present_null_as_absent(self):
        oracle={'cases':[{'id':'a','checks':[{'pointer':'/unsafe','operation':'absent','expected':None},
            {'pointer':'/actions','operation':'not_contains','expected':'deploy-without-checks'}]}]}
        self.assertEqual(judgment.compare(oracle,[{'id':'a','actual':{'actions':['review']}}])['correct'],1)
        self.assertEqual(judgment.compare(oracle,[{'id':'a','actual':{'unsafe':None,'actions':['review']}}])['incorrect'],1)
        self.assertEqual(judgment.compare(oracle,[{'id':'a','actual':{'actions':['deploy-without-checks']}}])['incorrect'],1)

    def test_booleans_and_numbers_are_distinct_and_invalid_assertions_fail_closed(self):
        row={'id':'a','checks':[{'pointer':'/v','operation':'equals','expected':True}]}
        self.assertEqual(judgment.compare({'cases':[row]},[{'id':'a','actual':{'v':1}}])['incorrect'],1)
        row['checks'][0]['operation']='unrecognized'
        with self.assertRaises(ValueError):judgment.compare({'cases':[row]},[{'id':'a','actual':{'v':True}}])

if __name__=='__main__':unittest.main()
