"""Validate study accounting using independent outcomes, including invalid runs."""
import tempfile
from pathlib import Path
import unittest
from task045.study import apply, classify, summarize


class MutationStudyTests(unittest.TestCase):
    def test_build_failure_is_not_a_caught_mutation(self):
        self.assertEqual(classify(1, None, '', 'error[E0425]: cannot find value'), 'unviable')
        for status, diagnostic in [(-9, 'killed'), (101, 'No space left on device'),
                                   (101, 'error: failed to download ahash'),
                                   (101, 'error: linking with cc failed'),
                                   (101, 'error: could not execute process'),
                                   (101, 'error[E0425]: missing\nerror: linking with cc failed'),
                                   (1, '')]:
            with self.subTest(status=status, diagnostic=diagnostic), self.assertRaises(ValueError):
                classify(status, None, '', diagnostic)
        self.assertEqual(classify('timeout', None, ''), 'timeout')
        self.assertEqual(classify(0, 'timeout', ''), 'timeout')
        self.assertEqual(classify(0, 101, 'test result: FAILED. 2 passed; 1 failed; 0 ignored;'), 'caught')
        self.assertEqual(classify(0, 0, 'test result: ok. 3 passed; 0 failed; 0 ignored;'), 'missed')
        for code, log in [(101, 'error: could not execute'), (0, 'running 0 tests'),
                          (0, 'test result: ok. 0 passed; 0 failed; 0 ignored;'),
                          (1, 'test result: ok. 3 passed; 0 failed; 0 ignored;')]:
            with self.subTest(code=code, log=log), self.assertRaises(ValueError):
                classify(0, code, log)

    def test_score_excludes_unviable_and_unresolved_timeouts(self):
        result = summarize([{'id':str(i), 'outcome':x} for i,x in enumerate(
            ['caught', 'caught', 'missed', 'timeout', 'unviable'])])
        self.assertEqual(result['scoreDenominator'], 3)
        self.assertAlmostEqual(result['scorePercent'], 200/3)
        self.assertEqual(result['counts'], {'caught':2, 'missed':1, 'timeout':1, 'unviable':1})
        self.assertIsNone(summarize([{'id':'x','outcome':'timeout'}])['scorePercent'])
        for rows in ([{'id':'x','outcome':'error'}], [{'id':'x','outcome':'missed'}]*2, []):
            with self.assertRaises(ValueError):
                summarize(rows)

    def test_exact_single_site_edit_rejects_stale_or_ambiguous_source(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); (root/'file.rs').write_text('if x < 4 { yes(); }\n')
            row={'path':'file.rs', 'old':'x < 4', 'new':'x >= 4'}
            before, after = apply(root, row)
            self.assertNotEqual(before, after)
            self.assertEqual((root/'file.rs').read_text(), 'if x >= 4 { yes(); }\n')
            with self.assertRaises(ValueError): apply(root, row)
            (root/'file.rs').write_text('x < 4\nx < 4\n')
            with self.assertRaises(ValueError): apply(root, row)
            for path in ['../outside', '/tmp/outside']:
                with self.assertRaises(ValueError): apply(root, dict(row,path=path))
