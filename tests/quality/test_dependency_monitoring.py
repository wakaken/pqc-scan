"""Live monitoring must distinguish findings, warnings and unavailable checks."""
import importlib.util
import json
import re
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('dependency_monitoring', ROOT/'scripts/check-dependencies.py')
monitor = importlib.util.module_from_spec(spec)
spec.loader.exec_module(monitor)


def report(entries=None, warnings=None):
    entries = entries or []
    return {'settings': {'ignore': [], 'target_arch': [], 'target_os': [], 'severity': None}, 'lockfile': {'dependency-count': 1},
            'vulnerabilities': {'list': entries, 'count': len(entries), 'found': bool(entries)},
            'warnings': warnings or {}}


def finding():
    return {'advisory': {'id': 'RUSTSEC-2026-0001', 'package': 'synthetic'},
            'package': {'name': 'synthetic', 'version': '1.0.0',
                        'source': 'registry+https://github.com/rust-lang/crates.io-index'}}


class MonitoringTests(unittest.TestCase):
    def test_clean_findings_and_informational_warnings_are_distinct(self):
        self.assertEqual(monitor.classify(report(), 0), ('clean', []))
        self.assertEqual(monitor.classify(report([finding()]), 1)[0], 'vulnerabilities')
        state, items = monitor.classify(report(warnings={'unmaintained': [finding()]}), 0)
        self.assertEqual(state, 'warnings')
        self.assertEqual(items[0]['kind'], 'unmaintained')

    def test_invalid_partial_filtered_or_failed_report_is_not_clean(self):
        for data, code in [(report(), 2), (report(), 1), (report([finding()]), 0), ({}, 0)]:
            with self.subTest(code=code):
                with self.assertRaises((ValueError, KeyError)):
                    monitor.classify(data, code)
        for field, value in [('count', 2), ('found', 'false')]:
            data = report(); data['vulnerabilities'][field] = value
            with self.assertRaises(ValueError): monitor.classify(data, 0)
        data = report(); data['settings']['ignore'] = ['RUSTSEC-2026-0001']
        with self.assertRaises(ValueError): monitor.classify(data, 0)
        for field, value in [('target_arch', ['x86_64']), ('target_os', ['linux']), ('severity', 'high')]:
            data = report(); data['settings'][field] = value
            with self.assertRaises(ValueError): monitor.classify(data, 0)

    def execute(self, data, code=0, failure=None):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); lock = root/'Cargo.lock'; lock.write_text('version = 4\n[[package]]\nname = "synthetic"\nversion = "1.0.0"\n')
            db = root/'db'; db.mkdir(); binary = root/'cargo-audit'; binary.write_bytes(b'synthetic trusted tool')
            output = root/'output'; result = subprocess.CompletedProcess([], code, json.dumps(data).encode(), b'synthetic stderr')
            with patch.object(monitor.subprocess, 'run', side_effect=failure, return_value=result) as proc:
                rc = monitor.run(binary, db, lock, output)
            return rc, json.loads((output/'result.json').read_text()), proc.call_args, (output/'audit.json').read_bytes()

    def test_run_preserves_raw_evidence_and_uses_neutral_config_without_credentials(self):
        rc, summary, call, raw = self.execute(report())
        self.assertEqual(rc, 0); self.assertEqual(summary['status'], 'clean')
        self.assertEqual(json.loads(raw), report())
        self.assertEqual(summary['lockfilePackages'], 1)
        argv = call.args[0]
        self.assertIn('--no-fetch', argv); self.assertIn('--file', argv)
        self.assertNotIn('--ignore', argv); self.assertNotIn('--stale', argv)
        self.assertNotEqual(call.kwargs['cwd'], ROOT)
        self.assertEqual(set(call.kwargs['env']), {'PATH', 'HOME', 'CARGO_HOME', 'TMPDIR'})
        self.assertLessEqual(call.kwargs['timeout'], 180)

    def test_findings_fail_but_warning_only_stays_visible(self):
        rc, summary, _, _ = self.execute(report([finding()]), 1)
        self.assertEqual(rc, 1); self.assertEqual(summary['status'], 'vulnerabilities')
        rc, summary, _, _ = self.execute(report(warnings={'unmaintained': [finding()]}))
        self.assertEqual(rc, 0); self.assertEqual(summary['status'], 'warnings')
        self.assertEqual(len(summary['findings']), 1)

    def test_malformed_tool_output_timeout_and_population_mismatch_fail_closed(self):
        for data, failure in [({}, None), (report(), subprocess.TimeoutExpired('audit', 180)),
                              (report(), OSError('tool unavailable'))]:
            rc, summary, _, _ = self.execute(data, failure=failure)
            self.assertEqual(rc, 2); self.assertEqual(summary['status'], 'error')
        data = report(); data['lockfile']['dependency-count'] = 0
        self.assertEqual(self.execute(data)[0], 2)

    def test_workflow_trigger_and_token_boundaries(self):
        text = (ROOT/'.github/workflows/dependency-audit.yml').read_text()
        self.assertIn("cron: '23 0 * * 1'", text)
        self.assertIn('branches: [develop]', text)
        for event in ['push', 'pull_request']:
            block = re.search(r'^  '+event+r':\n(.*?)(?=^  \w+:|\Z)', text, re.M | re.S)[1]
            for path in ['Cargo.lock', 'Cargo.toml', 'crates/**/Cargo.toml']:
                self.assertIn("'"+path+"'", block)
        self.assertNotIn('pull_request_target:', text)
        audit, notify = text.split('  audit:\n')[1].split('  notify:\n')
        self.assertNotIn('issues: write', audit)
        self.assertIn('contents: read', text.split('jobs:')[0])
        self.assertIn("needs.audit.result == 'failure'", notify)
        self.assertIn("github.event_name != 'pull_request'", notify)
        self.assertIn('github.event.repository.default_branch', notify)
        self.assertIn('issues: write', notify)
        self.assertIn('group: dependency-audit-notification', notify)
        self.assertIn('cancel-in-progress: false', notify)
        self.assertEqual(text.count('persist-credentials: false'), 2)
        self.assertIn('if: always()', audit)
        self.assertIn('retention-days: 30', audit)
        self.assertIn('sha256sum --check --strict', audit)
        for value in re.findall(r'uses: ([^\s]+)', text):
            self.assertRegex(value, r'^[^@]+@[0-9a-f]{40}$')

    def test_missing_lockfile_does_not_launch_tool_or_generate_one(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            with patch.object(monitor.subprocess, 'run') as proc:
                self.assertEqual(monitor.run(root/'audit', root/'db', root/'Cargo.lock', root/'out'), 2)
                proc.assert_not_called()
            self.assertFalse((root/'Cargo.lock').exists())

if __name__ == '__main__': unittest.main()
