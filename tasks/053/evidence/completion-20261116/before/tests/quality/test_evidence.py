"""All-axis evidence tests use independent literal requirements and observations."""
import copy
import hashlib
import json
import pathlib
import subprocess
import sys
import tempfile
import unittest

import report
import evidence


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data)+'\n')


class EvidenceIndexTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = pathlib.Path(self.tmp.name)
        self.req = self.root/'requirements.md'
        self.req.write_text('\n'.join(f'| Q{i}-01 Requirement {i} | old | gap | [00{i}](../tasks/00{i}/task.yaml) | acceptance | output |' for i in range(1, 7))+'\n')
        (self.root/'source.rs').write_text('source version one\n')
        save(self.root/'raw.json', {'counts':{'tp':2,'fp':1}, 'rate':None, 'failure':'retained'})
        (self.root/'review.md').write_text('Implementation and test review of source version one.\n')
        self.bindings = {'source.rs':sha(self.root/'source.rs')}
        self.record = {'id':'study-a','task':'001','requirements':['Q1-01'],
            'result':{'status':'Failed','summary':'One real product failure retained'},
            'measurement':{'head':'a'*40,'version':'0.2.0','features':['default'],
                'standards':['finite nominated truth'],'method':'hand-labelled synthetic corpus',
                'environment':'offline fixture','date':'2026-11-15',
                'commands':[{'argv':['scanner','scan','fixture'], 'exitCode':0}],
                'executionStatus':'Passed','comparison':'Same-data regression: Failed'},
            'sourceBindings':self.bindings,
            'artifacts':[{'path':'raw.json','sha256':sha(self.root/'raw.json'),'purpose':'raw counts'},
                         {'path':'review.md','sha256':sha(self.root/'review.md'),'purpose':'review'}],
            'observations':[{'label':'true positives','artifact':'raw.json','pointer':'/counts/tp'},
                            {'label':'unknown rate','artifact':'raw.json','pointer':'/rate'}],
            'reviews':[], 'limitations':['Only this fixed corpus'],
            'concerns':[{'priority':'high','text':'One known failure'}]}
        self.catalog = {'version':1,'requirements':{'path':'requirements.md','sha256':sha(self.req)},
                        'records':[self.record]}

    def build(self, catalog=None):
        save(self.root/'catalog.json', catalog or self.catalog)
        return evidence.build_index(self.root/'catalog.json', self.root)

    def test_all_six_axes_and_missing_requirements_stay_visible(self):
        result = self.build()
        self.assertEqual(list(result['axes']), ['Q1','Q2','Q3','Q4','Q5','Q6'])
        self.assertEqual(len(result['requirements']),6)
        rows = {r['id']:r for r in result['requirements']}
        self.assertEqual(rows['Q2-01']['status'],'Not Tested')
        self.assertEqual(rows['Q2-01']['ownerTask'],'002')
        self.assertTrue(rows['Q2-01']['reason'])
        self.assertEqual(rows['Q1-01']['status'],'Failed')
        self.assertEqual(result['releaseDecision'],'Unknown')

    def test_product_failure_is_a_successful_report_with_raw_null_and_counts(self):
        result = self.build()
        record = result['records'][0]
        self.assertEqual(record['observations'][0]['value'],2)
        self.assertIsNone(record['observations'][1]['value'])
        self.assertEqual(record['result']['status'],'Failed')
        self.assertEqual(record['measurement']['executionStatus'],'Passed')
        self.assertEqual(record['sourceApplicability']['status'],'Verified')
        self.assertEqual(result['infrastructureStatus'],'Verified')

    def test_changed_measurement_source_is_historical_not_current_pass(self):
        self.record['result']['status']='Passed'
        (self.root/'source.rs').write_text('source version two\n')
        result = self.build()
        self.assertEqual(result['records'][0]['result']['status'],'Passed')
        self.assertEqual(result['records'][0]['sourceApplicability']['status'],'Partial')
        self.assertEqual(result['requirements'][0]['status'],'Partial')
        self.assertIn('source.rs',result['records'][0]['sourceApplicability']['mismatched'])

    def test_log_success_never_manufactures_test_review(self):
        result=self.build()
        self.assertEqual(result['testEvidence'][0]['executionStatus'],'Passed')
        self.assertEqual(result['testEvidence'][0]['adequacyStatus'],'Not Tested')
        self.assertTrue(result['testEvidence'][0]['reason'])

    def test_test_index_keeps_hash_bound_review_artifacts_without_inventing_acceptance(self):
        result=self.build();test=result['testEvidence'][0]
        refs=test['reviewEvidenceArtifacts']
        self.assertEqual(refs,[{'path':'review.md','sha256':sha(self.root/'review.md'),'purpose':'review'}])
        self.assertEqual(test['measuredSourceBindings'],self.bindings)
        self.assertEqual(test['adequacyStatus'],'Not Tested')

    def test_reviews_need_two_stages_distinct_ids_and_exact_source(self):
        source_digest=evidence.binding_digest(self.bindings)
        self.record['reviews']=[{'stage':stage,'reviewer':who,'status':'accepted',
            'artifact':'review.md','sourceDigest':source_digest,'sourceBindings':dict(self.bindings)} for stage,who in
            [('implementation','impl-a'),('test','test-b')]]
        self.assertEqual(self.build()['testEvidence'][0]['adequacyStatus'],'Verified')
        self.record['reviews'][1]['sourceDigest']='0'*64
        self.assertEqual(self.build()['testEvidence'][0]['adequacyStatus'],'Partial')
        self.record['reviews'][1]['sourceDigest']=source_digest
        self.record['reviews'][1]['reviewer']='impl-a'
        self.assertEqual(self.build()['testEvidence'][0]['adequacyStatus'],'Partial')

    def test_review_controller_drift_is_not_hidden_by_matching_product_source(self):
        controller=self.root/'controller.py'
        controller.write_text('original measurement controller')
        scope=dict(self.bindings, **{'controller.py':sha(controller)})
        self.record['reviews']=[{'stage':stage,'reviewer':who,'status':'accepted',
            'artifact':'review.md','sourceDigest':evidence.binding_digest(scope),
            'sourceBindings':dict(scope)} for stage,who in [('implementation','impl-a'),('test','test-b')]]
        self.assertEqual(self.build()['testEvidence'][0]['adequacyStatus'],'Verified')
        controller.write_text('changed measurement controller')
        result=self.build()
        self.assertEqual(result['records'][0]['sourceApplicability']['status'],'Verified')
        self.assertEqual(result['testEvidence'][0]['adequacyStatus'],'Partial')
        self.assertIn('controller.py',result['testEvidence'][0]['reviewSourceApplicability'][0]['mismatched'])

    def test_review_bindings_must_cover_the_declared_product_population(self):
        unrelated={'review.md':sha(self.root/'review.md')}
        self.record['reviews']=[{'stage':stage,'reviewer':who,'status':'accepted',
            'artifact':'review.md','sourceDigest':evidence.binding_digest(unrelated),
            'sourceBindings':unrelated} for stage,who in [('implementation','impl-a'),('test','test-b')]]
        self.assertEqual(self.build()['testEvidence'][0]['adequacyStatus'],'Partial')

    def test_tampered_artifact_is_infrastructure_error(self):
        (self.root/'raw.json').write_text('{"counts":{"tp":999}}')
        with self.assertRaisesRegex(ValueError,'hash'):self.build()

    def test_requirement_catalog_cannot_omit_or_reassign_authoritative_ids(self):
        self.record['requirements']=['Q9-99']
        with self.assertRaises(ValueError):self.build()
        self.record['requirements']=['Q2-01']
        with self.assertRaises(ValueError):self.build()
        self.record['requirements']=['Q1-01']
        self.catalog['records'].append(copy.deepcopy(self.record))
        with self.assertRaises(ValueError):self.build()

    def test_unsafe_paths_and_symlinks_are_rejected(self):
        original=copy.deepcopy(self.catalog)
        for path in ['../raw.json','/etc/passwd','a/../raw.json','a\\raw.json']:
            catalog=copy.deepcopy(original);catalog['records'][0]['artifacts'][0]['path']=path
            with self.subTest(path=path),self.assertRaises(ValueError):self.build(catalog)
        (self.root/'alias.json').symlink_to(self.root/'raw.json')
        catalog=copy.deepcopy(original);catalog['records'][0]['artifacts'][0]['path']='alias.json'
        with self.assertRaises((ValueError,OSError)):self.build(catalog)

    def test_bad_pointer_and_nonfinite_or_duplicate_json_are_rejected(self):
        self.record['observations'][0]['pointer']='/counts/missing'
        with self.assertRaises(ValueError):self.build()
        self.record['observations'][0]['pointer']='/counts/tp'
        for text in ['{"rate":NaN}','{"counts":{},"counts":{"tp":2}}']:
            (self.root/'raw.json').write_text(text)
            self.record['artifacts'][0]['sha256']=sha(self.root/'raw.json')
            with self.assertRaises(ValueError):self.build()

    def test_unknown_measurement_has_explicit_reason_and_no_invented_exit(self):
        self.record['result']={'status':'Unknown','summary':'No authorized network observation'}
        self.record['measurement']['executionStatus']='Not Tested'
        self.record['measurement']['commands']=[{'argv':['not-run'],'exitCode':None,'reason':'Not authorized'}]
        result=self.build()
        self.assertEqual(result['requirements'][0]['status'],'Unknown')
        self.assertIsNone(result['testEvidence'][0]['commands'][0]['exitCode'])
        self.assertEqual(result['testEvidence'][0]['adequacyStatus'],'Not Tested')

    def test_renderer_preserves_json_facts_and_refuses_overwrite(self):
        result=self.build();output=self.root/'output'
        evidence.write_index(result,output)
        raw=json.loads((output/'quality-evidence-index.json').read_text())
        self.assertEqual(raw,result)
        tests=json.loads((output/'test-evidence-index.json').read_text())
        self.assertEqual(tests['records'],result['testEvidence'])
        text=(output/'quality-assessment.md').read_text()
        self.assertIn('Q2-01 | Not Tested',text)
        self.assertIn('true positives: `2`',text)
        self.assertIn('unknown rate: `null`',text)
        self.assertIn('Failed',text)
        self.assertIn('Original recorded outcome',text)
        self.assertIn('Q1: Partial',text)
        with self.assertRaises((ValueError,FileExistsError)):evidence.write_index(result,output)

    def test_catalog_commands_are_data_never_executed(self):
        marker=self.root/'must-not-exist'
        self.record['measurement']['commands'][0]['argv']=['sh','-c',f'touch {marker}']
        self.build();self.assertFalse(marker.exists())

    def test_combined_mode_preserves_every_baseline_output_byte(self):
        from test_report import EvidenceTests
        fixture=EvidenceTests()
        fixture.setUp()
        self.addCleanup(fixture.doCleanups)
        for name in ['requirements.md','source.rs','raw.json','review.md']:
            (fixture.repo/name).write_bytes((self.root/name).read_bytes())
        save(fixture.repo/'catalog.json',self.catalog)
        old=fixture.repo/'target/baseline-only';combined=fixture.repo/'target/combined'
        self.assertEqual(report.main(['--baseline',str(fixture.baseline),'--out-dir',str(old)],repo=fixture.repo),0)
        self.assertEqual(report.main(['--baseline',str(fixture.baseline),
            '--evidence-catalog',str(fixture.repo/'catalog.json'),'--out-dir',str(combined)],repo=fixture.repo),0)
        original=list(old.iterdir())
        self.assertTrue(original)
        for path in original:
            self.assertEqual((combined/path.name).read_bytes(),path.read_bytes(),path.name)
        self.assertTrue((combined/'quality-evidence-index.json').is_file())

    def test_report_entrypoint_catalog_mode_reuses_existing_dispatch(self):
        save(self.root/'catalog.json',self.catalog)
        code=report.main(['--evidence-catalog',str(self.root/'catalog.json'),'--out-dir',str(self.root/'target/report')],repo=self.root)
        self.assertEqual(code,0)
        self.assertTrue((self.root/'target/report/quality-evidence-index.json').is_file())
        code=report.main(['--evidence-catalog',str(self.root/'catalog.json'),'--out-dir',str(self.root/'outside')],repo=self.root)
        self.assertEqual(code,1)


    def test_observation_values_and_record_count_are_bounded(self):
        save(self.root/'raw.json', {'large':'x'*65537})
        self.record['artifacts'][0]['sha256']=sha(self.root/'raw.json')
        self.record['observations'][0]['pointer']='/large'
        with self.assertRaisesRegex(ValueError,'limit'):self.build()

    def test_fifo_and_oversize_files_fail_without_reading_or_blocking(self):
        import os
        os.mkfifo(self.root/'pipe')
        with self.assertRaises(ValueError):evidence.read_bytes(self.root,'pipe')
        with (self.root/'oversize').open('wb') as stream:stream.truncate(evidence.MAX_FILE+1)
        with self.assertRaises(ValueError):evidence.read_bytes(self.root,'oversize')

    def test_invalid_catalog_shapes_fail_closed_at_entrypoint(self):
        for data in [[], {'version':True,'records':[]}, {'version':1,'records':[None]}]:
            save(self.root/'invalid.json',data)
            with self.subTest(data=data):
                code=report.main(['--evidence-catalog',str(self.root/'invalid.json'),
                    '--out-dir',str(self.root/'target/invalid')],repo=self.root)
                self.assertEqual(code,1)
                error=self.root/'target/invalid/quality-infrastructure-error.json'
                self.assertEqual(json.loads(error.read_text())['infrastructureStatus'],'Failed')
                self.assertFalse((self.root/'target/invalid/quality-evidence-index.json').exists())
                error.unlink();error.parent.rmdir()

    def test_catalog_integrity_failure_retains_only_infrastructure_receipt(self):
        save(self.root/'catalog.json',self.catalog)
        (self.root/'raw.json').write_text('tampered evidence')
        output=self.root/'target/failed-aggregate'
        code=report.main(['--evidence-catalog',str(self.root/'catalog.json'),'--out-dir',str(output)],repo=self.root)
        self.assertEqual(code,1)
        error=json.loads((output/'quality-infrastructure-error.json').read_text())
        self.assertEqual(error['infrastructureStatus'],'Failed')
        self.assertEqual(error['releaseDecision'],'Unknown')
        self.assertFalse((output/'quality-evidence-index.json').exists())
        self.assertEqual(report.main(['--evidence-catalog',str(self.root/'catalog.json'),'--out-dir',str(output)],repo=self.root),1)
        self.assertEqual(json.loads((output/'quality-infrastructure-error.json').read_text()),error)

    def test_task052_partial_cannot_disappear_behind_source_matching(self):
        self.record['result']={'status':'Unknown','summary':'Ownership facts not established'}
        self.record['sourceBindings']={}
        row=self.build()['requirements'][0]
        self.assertEqual(row['status'],'Unknown')
        self.assertIn('unbound',row['reason'])

    def test_tool_failure_has_its_own_state_separate_from_product_failure(self):
        self.record['measurement']['executionStatus']='Tool Error'
        self.record['measurement']['commands'][0]['exitCode']=1
        self.record['result']={'status':'Unknown','summary':'Tool failed before product measurement'}
        result=self.build()
        self.assertEqual(result['testEvidence'][0]['executionStatus'],'Tool Error')
        self.assertEqual(result['requirements'][0]['status'],'Unknown')
        self.assertEqual(result['infrastructureStatus'],'Verified')


class EnrolledEvidenceTests(unittest.TestCase):
    def test_real_catalog_preserves_all_requirements_and_selected_actual_facts(self):
        root=pathlib.Path(__file__).resolve().parents[2]
        result=evidence.build_index(root/'docs/quality/EVIDENCE-CATALOG.json',root)
        self.assertEqual(len(result['requirements']),73)
        rows={r['id']:r for r in result['requirements']}
        self.assertEqual(rows['Q6-05']['ownerTask'],'052')
        self.assertEqual(rows['Q6-05']['status'],'Unknown')
        self.assertEqual(rows['Q6-17']['status'],'Not Tested')
        records={r['task']:r for r in result['records']}
        values={x['label']:x['value'] for x in records['061']['observations']}
        self.assertEqual(values['counts'],{'validAccepted':2,'invalidRejected':30,'unassessed':0})
        values={x['label']:x['value'] for x in records['056']['observations']}
        self.assertEqual(values['probe']['correct'],1531)
        self.assertEqual(values['pipeline']['correct'],16)
        self.assertEqual(result['releaseDecision'],'Unknown')
        self.assertEqual(len(result['testEvidence']),18)


if __name__=='__main__':
    unittest.main()
