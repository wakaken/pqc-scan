"""Observable libtest/Miri outcome parsing; never confuse missing tests with pass."""
import json
import unittest
from task044.miri_support import parse_test_events, classify_failure


class MiriEvidenceTests(unittest.TestCase):
    def test_completed_active_and_ignored_are_separate(self):
        known=['scope::ok','scope::blocked','scope::later','scope::ignored']
        log='\n'.join(json.dumps(x) for x in [
            {'type':'test','event':'started','name':'scope::ok'},
            {'type':'test','event':'ok','name':'scope::ok'},
            {'type':'test','event':'started','name':'scope::ignored'},
            {'type':'test','event':'ignored','name':'scope::ignored'},
            {'type':'test','event':'started','name':'scope::blocked'}])+'\nerror: unsupported operation: ts_parser_new\n'
        events=parse_test_events(log,known)
        self.assertEqual(events['passed'],['scope::ok'])
        self.assertEqual(events['ignored'],['scope::ignored'])
        self.assertEqual(events['active'],['scope::blocked'])
        self.assertEqual(events['unseen'],['scope::later'])
        self.assertEqual(classify_failure(1,log),'Unsupported')

    def test_undefined_behavior_is_failure_even_with_unsupported_in_backtrace(self):
        log='error: Undefined Behavior: null pointer\nnote: unsupported is not the primary diagnostic'
        self.assertEqual(classify_failure(1,log),'Failed: UB')
        self.assertEqual(classify_failure('timeout',''),'Partial: timeout')
        self.assertEqual(classify_failure(101,'test a ... FAILED\nassertion failed'),'Failed: assertion')
        self.assertEqual(classify_failure(1,'tool exited'),'Unknown: execution error')

    def test_no_success_from_empty_summary_or_foreign_test_names(self):
        events=parse_test_events('test result: ok. 0 passed; 0 failed\n',['a'])
        self.assertEqual(events['passed'],[])
        self.assertEqual(events['unseen'],['a'])
        with self.assertRaises(ValueError):
            parse_test_events(json.dumps({'type':'test','event':'started','name':'foreign'}),['a'])
        with self.assertRaises(ValueError):
            parse_test_events('\n'.join(json.dumps({'type':'test','event':event,'name':'a'}) for event in ['started','ok','failed']),['a'])

    def test_libtest_timeout_warning_keeps_test_active(self):
        log='\n'.join(json.dumps({'type':'test','event':e,'name':'slow'}) for e in ['started','timeout'])
        self.assertEqual(parse_test_events(log,['slow'])['active'],['slow'])
        self.assertEqual(parse_test_events(log,['slow'])['passed'],[])
        log+='\n'+json.dumps({'type':'test','event':'ok','name':'slow'})
        self.assertEqual(parse_test_events(log,['slow'])['passed'],['slow'])
