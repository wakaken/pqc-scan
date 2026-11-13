"""Independent arithmetic and destructive controls for performance evidence."""
import copy
import os
import json
import pathlib
import sys
import tempfile
import unittest
import run


class Performance050Tests(unittest.TestCase):
    def test_resource_scope_has_units_and_unavailable_is_null(self):
        self.assertTrue(hasattr(run, 'resource_scope'), 'missing explicit resource scope')
        self.assertEqual(run.resource_scope('win32', False)['cpuScope'], None)
        self.assertEqual(run.resource_scope('linux', True)['rssNativeUnit'], 'KiB')
        self.assertEqual(run.resource_scope('darwin', True)['rssNativeUnit'], 'bytes')

    def test_success_denominator_and_failures_are_separate(self):
        from task050.study import throughput
        sample={'exitCode':0,'timedOut':False,'wallSeconds':2}
        self.assertEqual(throughput(sample,{'files':4,'bytes':100}),{'filesPerSecond':2,'bytesPerSecond':50})
        for bad in [dict(sample,exitCode=2),dict(sample,timedOut=True)]:
            self.assertIsNone(throughput(bad,{'files':4,'bytes':100}))

    def test_same_semantics_different_order_and_duplicates(self):
        from task050.study import fingerprints
        a={'generated_at':'a','findings':[{'id':'a'},{'id':'b'},{'id':'a'}]}
        b=copy.deepcopy(a);b['generated_at']='b';b['findings'].reverse()
        # reverse of a palindrome is equal, use a true permutation
        b['findings']=[{'id':'b'},{'id':'a'},{'id':'a'}]
        fa=fingerprints('report.json',a);fb=fingerprints('report.json',b)
        self.assertEqual(fa['semantic'],fb['semantic']);self.assertNotEqual(fa['order'],fb['order'])
        for changed in [{'findings':[{'id':'a'},{'id':'b'}]}, {'findings':[{'id':'x'},{'id':'b'},{'id':'a'}]}]:
            self.assertNotEqual(fa['semantic'],fingerprints('report.json',changed)['semantic'])
        self.assertNotEqual(fingerprints('report.json',{'summary':{'generated_at':1}})['semantic'],fingerprints('report.json',{'summary':{'generated_at':2}})['semantic'])

    def test_nested_positional_arrays_are_not_sorted_away(self):
        from task050.study import fingerprints
        a=[{'id':'a','parameters':[1,2]}];b=[{'id':'a','parameters':[2,1]}]
        self.assertNotEqual(fingerprints('cbom.json',a)['semantic'],fingerprints('cbom.json',b)['semantic'])

    def test_incomplete_matrix_is_rejected(self):
        from task050.study import validate_study
        with self.assertRaises(ValueError):validate_study([])

    def test_run_and_observation_ids_never_silently_disappear(self):
        from task050.study import fingerprints
        a={'run':{'id':'scan-a','generated_at':'a'},'observations':[{'id':'o-a','occurrence_key':'k'}]}
        b=copy.deepcopy(a);b['run']['generated_at']='b'
        self.assertEqual(fingerprints('scan.inventory.v1.json',a),fingerprints('scan.inventory.v1.json',b))
        b['observations'][0]['id']='o-b'
        self.assertNotEqual(fingerprints('scan.inventory.v1.json',a)['semantic'],fingerprints('scan.inventory.v1.json',b)['semantic'])

    def test_gate_retains_two_x_signal_and_rejects_mixed_environment(self):
        from task050.study import comparison
        env={'binarySha256':'x','platform':'macOS','machine':'arm64','buildProfile':'release','features':'default','runtimeSourceDigest':'r','hardware':{},'rust':{},'constraints':None}
        a={'environment':env,'input':{'sha256':'a'},'configuration':{'threads':1},'warmMax':{'wallSeconds':2,'peakRssBytes':100}}
        b=copy.deepcopy(a);b['warmMax']={'wallSeconds':4.1,'peakRssBytes':200}
        c=comparison(a,b);self.assertTrue(c['comparable']);self.assertEqual(c['investigate'],['wallSeconds'])
        for field in ['binarySha256','harnessDigest','python','cpuCount','wallTimer','cachePolicy']:
            changed=copy.deepcopy(a);changed['environment'][field]='different'
            changed['warmMax']['peakRssBytes']=250
            self.assertFalse(comparison(a,changed)['comparable'],field)
            self.assertIn(field,comparison(a,changed)['reasons'])
            self.assertEqual(comparison(a,changed)['investigate'],[])
        b['environment']['platform']='Linux'
        self.assertFalse(comparison(a,b)['comparable']);self.assertEqual(comparison(a,b)['investigate'],[])

    def test_manifest_is_actual_bytes_and_detects_mutation(self):
        from task050.study import inventory, verify_input
        with tempfile.TemporaryDirectory() as d:
            root=pathlib.Path(d);(root/'a.rs').write_bytes(b'abc');(root/'b.rs').write_bytes(b'12345')
            info=inventory(root);self.assertEqual((info['files'],info['bytes']),(2,8));verify_input(root,info)
            (root/'a.rs').write_bytes(b'changed')
            with self.assertRaises(ValueError):verify_input(root,info)

    def test_facts_read_actual_legacy_id_field(self):
        from task050.study import facts_from_documents
        result=facts_from_documents({'findings':[{'finding_id':'f1'},{'finding_id':'f1'}]}, {'assets':[{'id':'a'}],'observations':[{'id':'o','occurrence_key':'k'}],'run':{'id':'r'}})
        self.assertEqual(result['findingIds'],['f1','f1'])
        self.assertEqual(result['assetIds'],['a'])

    @unittest.skipUnless(os.environ.get('PQC050_BINARY'), 'explicit product binary required')
    def test_real_threshold_failure_retains_findings(self):
        from task050.study import failure_fixture
        with tempfile.TemporaryDirectory() as d:
            root=pathlib.Path(d)/'input';root.mkdir();failure_fixture(root)
            out=pathlib.Path(d)/'out'
            result=run.measure([os.environ['PQC050_BINARY'],'scan',str(root),'--fail-on','high','--format','all','--diagnostics','--rules-dir',str(run.REPO/'rules'),'--out-dir',str(out)],out,10)
            self.assertEqual(result['exitCode'],2)
            report=json.loads((out/'report.json').read_text())
            self.assertGreater(len(report['findings']),0)
            incomplete=pathlib.Path(d)/'incomplete'
            result=run.measure([os.environ['PQC050_BINARY'],'scan',str(root),'--require-complete','--format','all','--diagnostics','--rules-dir',str(run.REPO/'rules'),'--out-dir',str(incomplete)],incomplete,10)
            self.assertEqual(result['exitCode'],3)
            self.assertGreater(json.loads((incomplete/'diagnostics.json').read_text())['coverage']['partial_files'],0)

    def test_failure_keeps_actual_logs_and_measurement(self):
        with tempfile.TemporaryDirectory() as d:
            p=pathlib.Path(d)
            sample=run.measure([sys.executable,'-c',"import sys;print('retained');sys.exit(7)"],p,5)
            self.assertEqual(sample['exitCode'],7);self.assertGreater(sample['wallSeconds'],0)
            self.assertEqual((p/'stdout.log').read_text(),'retained\n')
