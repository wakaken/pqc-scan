"""Offline aggregation must reuse existing jobs without enabling costly probes."""
from pathlib import Path
import re
import unittest

WORKFLOW = Path(__file__).resolve().parents[2]/'.github/workflows/pqc-scan.yml'


class EvidenceWorkflowTests(unittest.TestCase):
    def test_existing_jobs_aggregate_and_always_retain_with_explicit_lifetime(self):
        text = WORKFLOW.read_text()
        for name in ['quality', 'baseline']:
            block = re.search(r'^  '+name+r':\n(.*?)(?=^  \w+:|\Z)', text, re.M | re.S)[1]
            self.assertIn('--evidence-catalog docs/quality/EVIDENCE-CATALOG.json', block)
            upload = block[block.index('name: Preserve'):]
            self.assertIn('if: always()', upload)
            self.assertIn('retention-days: 30', upload)
            self.assertIn('target/quality-integrated/', upload)
        quality = text.split('  quality:\n',1)[1].split('  protocols:\n',1)[0]
        self.assertNotIn('tests/quality/run.py', quality)
        self.assertNotIn('tests/protocol/run-isolated.py', quality)

    def test_event_selection_and_paused_protocol_policy_remain_explicit(self):
        text = WORKFLOW.read_text()
        self.assertIn("github.event_name == 'pull_request' || github.event_name == 'workflow_dispatch'", text)
        self.assertEqual(text.count("github.event_name == 'push' && github.ref == 'refs/heads/develop'"),2)
        protocols = text.split('  protocols:\n',1)[1].split('  baseline:\n',1)[0]
        self.assertIn('if: ${{ false }}', protocols)
        self.assertNotIn('schedule:',text)


if __name__ == '__main__':
    unittest.main()
