"""Hand-derived aggregation and fail-closed independent-annotation checks."""
import copy
import json
import pathlib
import tempfile
import unittest

from evaluate import load_truth, score, validate_corpus
from report import metrics
from taxonomy import aggregate, rule_coverage, validate_annotations, select_corpus


def example():
    files = [dict(path='a.js', source='a.js', split='tuning', lineage='one')]
    units = [dict(id=f'u{i}', path='a.js', ruleId='R', line=i, startColumn=1, endColumn=10)
             for i in range(1, 5)]
    data = dict(files=files, units=units, families={'api': ['R']}, annotationVersion=1)
    rows = [dict(id=u['id'], label=label, algorithms=['RSA'] if i != 2 else ['ECDH', 'ML-KEM'],
                 usage='signature', artifact='source', caseKind='ordinary', parameters=[],
                 reason='Source-only review of nominal reference')
            for i, (u, label) in enumerate(zip(units, ['positive', 'negative', 'ambiguous', 'unsupported']))]
    truth = dict(labels=rows, adjudications=[])
    reviewers = [dict(reviewer=n, labels=copy.deepcopy(rows)) for n in ['A', 'B']]
    return data, truth, reviewers


class TaxonomyTests(unittest.TestCase):
    def test_checked_in_expansion_is_frozen_and_old_corpus_remains_separate(self):
        repo = pathlib.Path(__file__).resolve().parents[2]
        directory = repo/'tests/quality/expanded'
        data, labels = load_truth(directory)
        validate_corpus(data, repo)
        old, _ = load_truth(repo/'tests/quality')
        self.assertEqual(len(old['units']), 57)
        self.assertEqual(len(labels), len(data['units']))
        self.assertEqual({f['split'] for f in data['files']}, {'tuning','holdout'})
        self.assertTrue(all(f['lineage']=='nodejs-upstream-2025' for f in data['files'] if f['split']=='holdout'))
        self.assertEqual({labels[k] for k in ['x057','x060','x063']}, {'ambiguous'})

    def test_duplicate_extra_exclusion_mixed_and_empty_denominators(self):
        data, truth, _ = example()
        labels = {r['id']: r['label'] for r in truth['labels']}
        def finding(line, column=1):
            return dict(rule_id='R', location=dict(file='a.js', line=line, column=column))
        findings = [finding(1), finding(1), finding(2), finding(2), finding(3), finding(4), finding(9)]
        scored = score(data, labels, findings)
        rows = aggregate(data, truth, scored, 'algorithms', metrics)
        rsa = rows['tuning/RSA']
        self.assertEqual([rsa[k] for k in ['tp', 'fp', 'fn', 'tn']], [1, 3, 0, 0])
        self.assertEqual((rsa['precision'], rsa['fpr'], rsa['fpr_denominator']), (.25, 1, 1))
        self.assertEqual(rows['tuning/ECDH+ML-KEM']['eligible'], 0)
        self.assertIsNone(rows['tuning/ECDH+ML-KEM']['recall'])
        extra = rows['tuning/unattributed']
        self.assertEqual((extra['fp'], extra['eligible']), (1, 0))
        self.assertEqual(extra['precision'], 0)
        self.assertIsNone(extra['fpr'])
        self.assertEqual(sum(r['fp'] for r in rows.values()), 4)
        self.assertEqual(sum(r['excluded_predictions'] for r in rows.values()), 2)

    def test_review_disagreements_require_exact_reasoned_adjudication(self):
        data, truth, reviewers = example()
        validate_annotations(data, truth, reviewers)
        reviewers[1]['labels'][0]['algorithms'] = ['DSA']
        with self.assertRaisesRegex(ValueError, 'adjudication'):
            validate_annotations(data, truth, reviewers)
        truth['adjudications'] = [dict(id='u1', reason='Original source explicitly names RSA')]
        with self.assertRaisesRegex(ValueError, 'adjudication'):
            validate_annotations(data, truth, reviewers)
        truth['adjudications'][0]['resolution'] = {k: truth['labels'][0][k] for k in
                                                  ('label', 'algorithms', 'usage', 'artifact', 'caseKind', 'parameters')}
        validate_annotations(data, truth, reviewers)
        truth['labels'][0]['usage'] = 'encryption'
        with self.assertRaisesRegex(ValueError, 'adjudication'):
            validate_annotations(data, truth, reviewers)
        truth['adjudications'][0]['resolution'] = {k: truth['labels'][0][k] for k in
                                                  ('label', 'algorithms', 'usage', 'artifact', 'caseKind', 'parameters')}
        validate_annotations(data, truth, reviewers)

    def test_missing_unknown_duplicate_and_unreviewed_final_annotations_rejected(self):
        for mutate in [lambda t: t['labels'][0].pop('usage'),
                       lambda t: t['labels'][0].update(algorithms=['made-up-safe']),
                       lambda t: t['labels'][0].update(algorithms=['RSA', 'RSA']),
                       lambda t: t['labels'][0].update(algorithms={'RSA': True}),
                       lambda t: t['labels'].append(t['labels'][0]),
                       lambda t: t['labels'][0].update(label='negative')]:
            data, truth, reviewers = example()
            mutate(truth)
            with self.assertRaises(ValueError):
                validate_annotations(data, truth, reviewers)

    def test_all_rules_ledger_keeps_untested_and_one_sided(self):
        data, truth, _ = example()
        ledger = rule_coverage(data, truth, ['R', 'UNTESTED'])
        self.assertEqual(ledger['R']['status'], 'Verified')
        self.assertEqual(ledger['UNTESTED']['status'], 'Not Tested')
        self.assertEqual(ledger['UNTESTED']['eligible'], 0)
        truth['labels'][1]['label'] = 'ambiguous'
        self.assertEqual(rule_coverage(data, truth, ['R'])['R']['status'], 'Partial')
        with self.assertRaises(ValueError):
            rule_coverage(data, truth, ['UNTESTED'])

    def test_selected_corpus_cannot_escape_or_follow_symlink(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = pathlib.Path(tmp).resolve()
            default = repo/'tests/quality'
            (default/'expanded').mkdir(parents=True)
            self.assertEqual(select_corpus(repo), default)
            self.assertEqual(select_corpus(repo, default/'expanded'), default/'expanded')
            with self.assertRaises(ValueError):
                select_corpus(repo, repo)
            (default/'link').symlink_to(default/'expanded', target_is_directory=True)
            with self.assertRaises(ValueError):
                select_corpus(repo, default/'link')


if __name__ == '__main__':
    unittest.main()
