"""Performance opt-out retains execution safety and immutable evidence checks."""
import copy
import importlib.util
import json
import os
import pathlib
import subprocess
import sys
import tempfile
import time
import unittest
from unittest import mock

import oss
import run
import test_benchmark
from test_measure_safety import descendant_command

SPEC = importlib.util.spec_from_file_location('no_performance_summary', oss.REPO / 'benchmark/summarize.py')
summary = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(summary)


def assert_unmeasured(case, value):
    for field in ('wallSeconds', 'cpuSeconds', 'peakRssBytes', 'rssNativeUnit', 'resourceScope'):
        case.assertIn(field, value)
        case.assertIsNone(value[field])
    case.assertEqual(value['resourceMethod'], run.PERFORMANCE_DISABLED)


class NoPerformanceProcessTests(unittest.TestCase):
    def test_success_and_nonzero_preserve_logs_without_resource_or_wall_observation(self):
        for code in (0, 7):
            with self.subTest(code=code), tempfile.TemporaryDirectory() as temporary:
                root = pathlib.Path(temporary)
                command = [sys.executable, '-c', f"import sys;print('kept');print('error',file=sys.stderr);sys.exit({code})"]
                with mock.patch.object(run.os, 'wait4', create=True, side_effect=AssertionError('wait4 forbidden')), \
                     mock.patch.object(run.time, 'monotonic', side_effect=AssertionError('wall measurement forbidden')):
                    sample = run.measure(command, root, 3, collect_performance=False)
                self.assertEqual(sample['exitCode'], code)
                self.assertFalse(sample['timedOut'])
                self.assertEqual((root / 'stdout.log').read_text(), 'kept\n')
                self.assertEqual((root / 'stderr.log').read_text(), 'error\n')
                assert_unmeasured(self, sample)

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_success_nonzero_and_timeout_kill_grandchild_and_reap_leader(self):
        for code in (0, 7, None):
            with self.subTest(code=code), tempfile.TemporaryDirectory() as temporary:
                root = pathlib.Path(temporary)
                ending = 'time.sleep(20)' if code is None else f'raise SystemExit({code})'
                command = descendant_command(root, ending)
                command[-1] = command[-1].replace('time.sleep(.4)', 'time.sleep(1)')
                with mock.patch.object(run.os, 'wait4', create=True, side_effect=AssertionError('wait4 forbidden')):
                    sample = run.measure(command, root / 'logs', .5 if code is None else 3, collect_performance=False)
                assert_unmeasured(self, sample)
                self.assertEqual(sample['timedOut'], code is None)
                if code is not None:
                    self.assertEqual(sample['exitCode'], code)
                else:
                    self.assertNotEqual(sample['exitCode'], 0)
                self.assertTrue((root / 'ready').exists())
                time.sleep(1.1)
                self.assertFalse((root / 'effect').exists())
                with self.assertRaises(ChildProcessError):
                    os.waitpid(int((root / 'leader').read_text()), os.WNOHANG)

    def test_invalid_timeout_and_prior_evidence_are_rejected_without_execution(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            marker = root / 'executed'
            command = [sys.executable, '-c', f"from pathlib import Path;Path({str(marker)!r}).touch()"]
            for timeout in (0, -1, float('inf'), float('nan')):
                with self.subTest(timeout=timeout), self.assertRaises(ValueError):
                    run.measure(command, root / 'invalid', timeout, collect_performance=False)
            self.assertFalse((root / 'invalid').exists())
            prior = root / 'prior'
            prior.mkdir()
            (prior / 'stdout.log').write_text('retained')
            with self.assertRaises(ValueError):
                run.measure(command, prior, 3, collect_performance=False)
            self.assertFalse(marker.exists())
            self.assertEqual((prior / 'stdout.log').read_text(), 'retained')

    def test_environment_keeps_source_bindings_without_hardware_probes(self):
        read_text = pathlib.Path.read_text
        def checked_read(path, *args, **kwargs):
            if str(path).startswith('/proc/'):
                raise AssertionError('proc hardware probe forbidden')
            return read_text(path, *args, **kwargs)
        def command_text(command):
            if command[0] == 'sysctl':
                raise AssertionError('sysctl hardware probe forbidden')
            return dict(command=command, exitCode=0, stdout='fixture provenance')
        with tempfile.TemporaryDirectory() as temporary:
            binary = pathlib.Path(temporary) / 'scanner'
            binary.write_bytes(b'fixture scanner identity')
            with mock.patch.object(run, 'command_text', side_effect=command_text), \
                 mock.patch.object(run.os, 'cpu_count', side_effect=AssertionError('CPU count forbidden')), \
                 mock.patch.object(pathlib.Path, 'read_text', checked_read):
                value = run.environment(binary, collect_performance=False)
            self.assertIsNone(value['cpuCount'])
            self.assertEqual(value['hardware'], {})
            self.assertEqual(value['binarySha256'], oss.bytes_digest(binary))
            self.assertTrue(value['runtimeSourceBindings'])
            self.assertTrue(value['harnessBindings'])
            self.assertEqual(value['wallTimer'], run.PERFORMANCE_DISABLED)
            self.assertEqual(value['rust']['exitCode'], 0)
            self.assertEqual(value['cargo']['exitCode'], 0)


class NoPerformanceBenchmarkTests(unittest.TestCase):
    setUp = test_benchmark.OssBenchmarkTests.setUp
    refresh_input = test_benchmark.OssBenchmarkTests.refresh_input
    refresh_registry = test_benchmark.OssBenchmarkTests.refresh_registry
    refresh_build = test_benchmark.OssBenchmarkTests.refresh_build
    make_binary = test_benchmark.OssBenchmarkTests.make_binary
    request = test_benchmark.OssBenchmarkTests.request
    run_request = test_benchmark.OssBenchmarkTests.run_request
    terminal = test_benchmark.OssBenchmarkTests.terminal

    def disable_performance(self):
        protocol = json.loads(self.protocol.read_text())
        test_benchmark.save(self.protocol, dict(protocol, collectPerformance=False))

    def test_flag_is_strict_boolean_and_default_configuration_remains_identical(self):
        protocol = json.loads(self.protocol.read_text())
        baseline = oss._configuration(self.request())[1]
        test_benchmark.save(self.protocol, dict(protocol, collectPerformance=True))
        self.assertEqual(oss._configuration(self.request())[1], baseline)
        for value in (None, 0, 1, 'false', [], {}):
            test_benchmark.save(self.protocol, dict(protocol, collectPerformance=value))
            with self.subTest(value=value), self.assertRaises(ValueError):
                oss._configuration(self.request())
        self.disable_performance()
        disabled = oss._configuration(self.request())[1]
        self.assertIs(disabled.pop('collectPerformance'), False)
        disabled['resourceCapabilities'].update(cpu=baseline['resourceCapabilities']['cpu'], rss=baseline['resourceCapabilities']['rss'])
        self.assertEqual(disabled, baseline)

    def test_fake_scan_resume_compare_and_summary_without_performance(self):
        self.disable_performance()
        with mock.patch.object(run.os, 'wait4', create=True, side_effect=AssertionError('wait4 forbidden')):
            first = self.run_request()
            resumed = self.run_request(resume=True)
        self.assertTrue(first['executionComplete'])
        self.assertTrue(resumed['rows'][0]['reused'])
        run.environment.assert_called_with(self.binary.resolve(), collect_performance=False)
        terminal = oss._json(self.terminal(first))
        assert_unmeasured(self, terminal['measurement'])
        comparison = oss.compare_runs(pathlib.Path(first['receiptPath']), pathlib.Path(resumed['receiptPath']), self.base / 'comparison.json')
        self.assertEqual(comparison['rows'][0]['status'], 'Comparable')
        repository = comparison['repositories'][self.row['id']]
        self.assertIsNone(repository['beforePerformance'])
        self.assertIsNone(repository['afterPerformance'])
        self.assertIn('deferred', repository['performanceInterpretation'])
        projection = summary.measurement_projection(terminal)
        aggregate = summary.aggregate([dict(execution='Succeeded', inspection='Complete', counts=None, measurement=projection)])
        self.assertEqual(aggregate['performanceByExecution']['Succeeded'], dict(measuredSamples=0, unavailableSamples=1, distributions=None))

    def test_mode_change_forces_new_attempt(self):
        first = self.run_request()
        self.disable_performance()
        second = self.run_request(resume=True)
        self.assertTrue(second['executionComplete'])
        self.assertFalse(second['rows'][0]['reused'])
        self.assertNotEqual(self.terminal(first), self.terminal(second))

    def test_invalid_outputs_and_input_mutation_are_invalid_evidence(self):
        self.disable_performance()
        for mode in ('bad-json', 'mutate-input'):
            with self.subTest(mode=mode):
                self.make_binary(mode)
                result = self.run_request(phase=mode)
                self.assertEqual(result['rows'][0]['execution'], 'InvalidEvidence')
                assert_unmeasured(self, oss._json(self.terminal(result))['measurement'])
                (self.root / 'a.rs').write_text('fixture static input')

    def test_tampered_performance_cannot_be_reused_or_compared(self):
        self.disable_performance()
        first = self.run_request()
        terminal_path = self.terminal(first)
        original = oss._json(terminal_path)
        for field, value in (('wallSeconds', .01), ('cpuSeconds', 1), ('peakRssBytes', 4096),
                             ('rssNativeUnit', 'bytes'), ('resourceScope', {}), ('resourceMethod', 'wait4 per child')):
            changed = copy.deepcopy(original)
            changed['measurement'][field] = value
            test_benchmark.save(terminal_path, changed)
            with self.subTest(field=field), self.assertRaises(ValueError):
                oss._verify_terminal(terminal_path)
        second = self.run_request(resume=True)
        self.assertFalse(second['rows'][0]['reused'])
        with self.assertRaises(ValueError):
            oss.compare_runs(pathlib.Path(first['receiptPath']), pathlib.Path(second['receiptPath']), self.base / 'tampered.json')

    def test_failed_receipt_cannot_smuggle_performance_into_summary(self):
        self.disable_performance()
        self.make_binary('failed')
        result = self.run_request()
        self.assertEqual(result['rows'][0]['execution'], 'Failed')
        terminal_path = self.terminal(result)
        terminal = oss._json(terminal_path)
        terminal['measurement'].update(wallSeconds=1, resourceMethod='wait4 per child')
        test_benchmark.save(terminal_path, terminal)
        invocation_path = pathlib.Path(result['receiptPath'])
        invocation = oss._json(invocation_path)
        invocation['rows'][0]['receipt'] = oss._reference(terminal_path)
        test_benchmark.save(invocation_path, invocation)
        with self.assertRaises(ValueError):
            oss._verified_invocation(invocation_path)

    def test_actual_entry_cli_runs_resumes_and_compares_without_performance(self):
        self.disable_performance()
        def invoke(arguments):
            result = subprocess.run([sys.executable, str(oss.REPO / 'benchmark/entry.py'), *arguments],
                                    cwd=oss.REPO, capture_output=True, text=True, timeout=30)
            self.assertEqual(result.returncode, 0, result.stderr)
            return json.loads(result.stdout)
        arguments = ['run', '--registry', str(self.registry), '--all', '--phase', 'cli-no-perf',
                     '--binary', str(self.binary), '--rules', str(self.rules), '--build-receipt', str(self.build),
                     '--protocol', str(self.protocol), '--output', str(self.base / 'cli-evidence')]
        first = invoke(arguments)
        resumed = invoke(arguments + ['--resume'])
        self.assertTrue(resumed['rows'][0]['reused'])
        self.assertEqual(first['coordinatorEnvironment']['hardware'], {})
        self.assertIsNone(first['coordinatorEnvironment']['cpuCount'])
        assert_unmeasured(self, oss._json(self.terminal(first))['measurement'])
        compared = invoke(['compare', '--before', first['receiptPath'], '--after', resumed['receiptPath'],
                           '--output', str(self.base / 'cli-comparison.json')])
        self.assertEqual(compared['rows'][0]['status'], 'Comparable')
        self.assertIsNone(compared['repositories'][self.row['id']]['beforePerformance'])


if __name__ == '__main__':
    unittest.main()
