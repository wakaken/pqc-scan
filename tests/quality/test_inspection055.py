"""Independent accounting examples; bad evidence must fail closed."""
import copy
import unittest
from inspection055 import partition, rates, detector_counts


class InspectionAccountingTests(unittest.TestCase):
    def test_partition_rejects_double_counts_and_negative_or_unknown_counters(self):
        row={'selected_files':3,'read_ok_files':2,'read_failed_files':1,
             'analyzed_files':1,'partial_files':1,'analysis_failed_files':0,
             'unsupported_files':0,'not_applicable_files':0,'unknown_files':0}
        partition(row)
        for key in ['selected_files','read_ok_files','partial_files']:
            changed=dict(row);changed[key]+=1
            with self.subTest(key=key), self.assertRaises(ValueError): partition(changed)
        for value in [-1, True, None]:
            changed=dict(row,unknown_files=value)
            with self.assertRaises(ValueError):partition(changed)

    def test_three_denominators_and_unknown_population(self):
        out=rates(selected=3,enumerated=5,known_total=None,read_ok=2,applicable=1,completed=0)
        self.assertIsNone(out['enumeration']['percent'])
        self.assertEqual(out['applicability']['denominator'],2)
        self.assertEqual(out['applicability']['percent'],50)
        self.assertEqual(out['completion']['percent'],0)
        self.assertIsNone(rates(selected=0,enumerated=0,known_total=0,read_ok=0,applicable=0,completed=0)['completion']['percent'])
        for args in [dict(selected=3,enumerated=2,known_total=5,read_ok=2,applicable=1,completed=0),
                     dict(selected=3,enumerated=5,known_total=4,read_ok=2,applicable=1,completed=0)]:
            with self.assertRaises(ValueError):rates(**args)

    def test_per_detector_rows_do_not_become_file_counts(self):
        rows=[{'case':'a','format':'source','detector':'x','status':'Complete'},
              {'case':'a','format':'source','detector':'y','status':'Failed'},
              {'case':'b','format':'source','detector':'x','status':'NotApplicable'}]
        out=detector_counts(rows)
        self.assertEqual(out['source']['x'],{'Complete':1,'NotApplicable':1})
        self.assertEqual(out['source']['y'],{'Failed':1})
        for bad in [rows+[rows[0]], [dict(rows[0],status='invented')]]:
            with self.assertRaises(ValueError):detector_counts(bad)

    def test_collection_rejects_unbound_or_missing_detector_cells(self):
        import hashlib
        import json
        from pathlib import Path
        import tempfile
        from inspection055 import collect
        counts=dict(selected_files=1,read_ok_files=1,read_failed_files=0,analyzed_files=1,
                    partial_files=0,analysis_failed_files=0,unsupported_files=0,not_applicable_files=0,unknown_files=0)
        names=['tree_sitter_detector','regex_detector','dependency_detector','certificate_detector','key_detector']
        fixture={'core055.json':{'files':[{'file':'a','expected_terminal':'analyzed_files','expectedApplicable':True,'coverage':counts}], 'batch':counts, 'mixedFailure':{}},
                 'selection055.json':{'knownInputs':[['a','x']], 'knownTotal':1,'selected':['a'],'enumeratedExcluded':[],'notEnumerated':[],'diagnostics':{'coverage':counts}},
                 'formats055.json':{'cases':[{'id':0,'format':'text','inputHex':'78','inputSha256':hashlib.sha256(b'x').hexdigest(),'detectors':[{'case':0,'format':'text','detector':n,'status':'Complete','expected':'Complete'} for n in names]}]},
                 'limits055.json':{'malformedSibling':{}}, 'work-limits055.json':{'cases':[]}}
        for change in ['missing','mismatched_case','mismatched_format','input_hash','status','duplicate_file','valid']:
            data=copy.deepcopy(fixture)
            case=data['formats055.json']['cases'][0]
            if change=='missing':case['detectors'].pop()
            if change=='mismatched_case':case['detectors'][0]['case']=99
            if change=='mismatched_format':case['detectors'][0]['format']='other'
            if change=='input_hash':case['inputHex']='79'
            if change=='status':case['detectors'][0]['status']='Failed'
            if change=='duplicate_file':data['core055.json']['files']*=2
            with tempfile.TemporaryDirectory() as tmp:
                root=Path(tmp)
                for name,value in data.items():(root/name).write_text(json.dumps(value))
                if change=='valid':
                    result=collect(root,root/'out');self.assertEqual(result['detector-coverage.json']['caseCount'],1)
                    self.assertIn('1ファイル',(root/'out/enumeration-review.md').read_text())
                else:
                    with self.subTest(change=change),self.assertRaises(ValueError):collect(root,root/'out')
