"""The actual CI applicability input must remain bound to its reviewed sources."""
import copy
import json
from pathlib import Path
import re
import shutil
import tempfile
import unittest

import report

ROOT = Path(__file__).resolve().parents[2]
CURRENT = 'tests/quality/applicability-document-relocation-20261114/assessment.json'
HISTORICAL = 'tests/quality/applicability-044-develop-integration/assessment.json'
CANDIDATES = {'x154', 'x156', 'v2-audit4', 'v2-audit5', 'v2-audit6', 'v2-audit7'}


def ci_assessment():
    paths = re.findall(r'--applicability\s+(\S+)', (ROOT/'.github/workflows/pqc-scan.yml').read_text())
    if len(paths) != 1:
        raise AssertionError('CI must select one explicit applicability assessment')
    return paths[0]


def scoped(repo, selected):
    directory = repo/'tests/quality/expanded-v2'
    data = json.loads((directory/'corpus.json').read_text())
    truth = json.loads((directory/'truth.json').read_text())
    return report.supported_scope(repo/selected, repo, directory, data, truth, [],
                                  {f['path']: f['inputFormat'] for f in data['files']},
                                  {rule: 'reviewed' for rules in data['families'].values() for rule in rules})


class CIApplicabilityTests(unittest.TestCase):
    def test_workflow_uses_current_complete_review_without_changing_scope(self):
        selected = ci_assessment()
        self.assertEqual(selected, CURRENT)
        result = scoped(ROOT, selected)
        self.assertEqual(result['status'], 'Verified')
        self.assertTrue(result['postObservation'])
        self.assertEqual({u['id'] for u in result['excludedUnits']}, CANDIDATES)
        current = json.loads((ROOT/selected).read_text())
        old = json.loads((ROOT/HISTORICAL).read_text())
        for key in ['corpusSha256', 'truthSha256', 'scope']:
            self.assertEqual(current[key], old[key])
        self.assertEqual({u['id']: u['status'] for u in current['dispositions']},
                         {u['id']: u['status'] for u in old['dispositions']})

    def test_historical_binding_remains_rejected_against_current_document(self):
        with self.assertRaisesRegex(ValueError, 'binding changed'):
            scoped(ROOT, HISTORICAL)

    def test_actual_ci_input_rejects_source_and_review_corruption(self):
        selected = ci_assessment()
        original = json.loads((ROOT/selected).read_text())
        mutations = ['contract', 'parser', 'protocol', 'corpus', 'truth', 'review_bytes',
                     'missing_review', 'same_identity', 'unresolved', 'decision', 'review_binding']
        for mutation in mutations:
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory() as temp:
                repo = Path(temp).resolve()
                folder = repo/Path(selected).parent
                shutil.copytree((ROOT/selected).parent, folder)
                directory = repo/'tests/quality/expanded-v2'; directory.mkdir(parents=True)
                for name in ['corpus.json', 'truth.json']:
                    shutil.copyfile(ROOT/'tests/quality/expanded-v2'/name, directory/name)
                for name in original['contractBindings']:
                    target = repo/name; target.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copyfile(ROOT/name, target)
                manifest = copy.deepcopy(original)
                review_name = list(manifest['reviewSources'])[1]
                review_path = folder/review_name
                review = json.loads(review_path.read_text())
                if mutation in ['contract', 'parser']:
                    name = 'docs/user/COVERAGE.md' if mutation == 'contract' else 'crates/pqc_scan_detectors/src/certificate_detector/pkcs12.rs'
                    with (repo/name).open('a') as stream: stream.write('\nchanged\n')
                if mutation == 'protocol': (folder/'PROTOCOL.md').write_text('changed')
                if mutation in ['corpus', 'truth']:
                    target = directory/f'{mutation}.json'
                    changed = json.loads(target.read_text()); changed['binding_probe'] = True
                    target.write_text(json.dumps(changed))
                if mutation == 'review_bytes': review['timestampUtc'] = '2099-01-01T00:00:00Z'
                if mutation == 'missing_review': del manifest['reviewSources'][review_name]
                if mutation == 'same_identity':
                    review['reviewer'] = json.loads((folder/list(manifest['reviewSources'])[0]).read_text())['reviewer']
                if mutation == 'unresolved': review['unresolved'] = ['Scope conflict']
                if mutation == 'decision': review['dispositions'][0]['status'] = 'retained'
                if mutation == 'review_binding': review['contractBindings']['docs/user/COVERAGE.md'] = '0'*64
                if mutation in ['review_bytes', 'same_identity', 'unresolved', 'decision', 'review_binding']:
                    review_path.write_text(json.dumps(review))
                    if mutation != 'review_bytes': manifest['reviewSources'][review_name] = report.bytes_digest(review_path)
                (repo/selected).write_text(json.dumps(manifest))
                with self.assertRaises(ValueError):
                    scoped(repo, selected)
