import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


RUNNER = Path(__file__).resolve().parents[2] / 'scripts/quality-with-performance-held.py'


class CandidateControls(unittest.TestCase):
    def fixture(self, base, body='pass'):
        root = base / 'repo'
        tests = root / 'tests/quality'
        tests.mkdir(parents=True)
        (root / 'benchmark').mkdir()
        (root / 'scripts').mkdir()
        observer = tests / 'run.py'
        observer.write_text(
            'from pathlib import Path\n'
            'def measure(*args, **kwargs):\n'
            '    Path(__file__).with_name("observer-called").write_text("measure")\n'
            '    return "disabled"\n'
            'def environment(*args, **kwargs):\n'
            '    Path(__file__).with_name("observer-called").write_text("environment")\n'
            '    return "disabled"\n')
        (tests / 'bounded_process.py').write_text(
            'import math\nfrom pathlib import Path\n'
            'def execute(command, output, *, timeout=2, cpu_seconds=2):\n'
            '    if not math.isfinite(timeout) or timeout <= 0 or type(cpu_seconds) is not int or cpu_seconds <= 0:\n'
            '        raise ValueError("invalid limits")\n'
            '    return _execute_validated(command, output, timeout=timeout, cpu_seconds=cpu_seconds)\n'
            'def _execute_validated(command, output, *, timeout, cpu_seconds):\n'
            '    Path(__file__).with_name("bounded-called").write_text("execute")\n')
        task = tests / 'task047'
        task.mkdir()
        (task / 'study.py').write_text(
            'import math\nfrom pathlib import Path\n'
            'from bounded_process import execute\n'
            'MAX_INPUT=1024*1024\n'
            'def trial(*args, **kwargs):\n'
            '    Path(__file__).with_name("trial-called").write_text("trial")\n'
            'def run_target(label, command, seeds, out, work, *, seconds, seed, max_cases=40000):\n'
            '    if not math.isfinite(seconds) or seconds <= 0 or type(max_cases) is not int or max_cases <= 0 or not seeds:\n'
            '        raise ValueError("invalid campaign limits")\n'
            '    return _run_target_validated(label, command, seeds, out, work, seconds=seconds, seed=seed, max_cases=max_cases)\n'
            'def _run_target_validated(label, command, seeds, out, work, *, seconds, seed, max_cases):\n'
            '    Path(__file__).with_name("target-called").write_text("run_target")\n')
        (tests / 'test_fixture.py').write_text(
            'import os, resource, unittest\nimport run\n'
            'class Checks(unittest.TestCase):\n'
            '    def test_held(self):\n'
            '        raise AssertionError("held method executed")\n'
            '    @unittest.skip("ordinary skip fixture")\n'
            '    def test_ordinary_skip(self):\n        pass\n'
            '    def test_selected(self):\n        ' + body + '\n')
        policy = base / 'policy.json'
        policy.write_text(json.dumps(dict(schemaVersion=2, reason='Synthetic hold', sourceRoots=['benchmark', 'tests', 'scripts'],
            heldTests=['test_fixture.Checks.test_held'],
            expectedTests=['test_fixture.Checks.' + name for name in ('test_held', 'test_ordinary_skip', 'test_selected')],
            sourcePins={})))
        self.repin(root, policy)
        return root, policy, observer

    def repin(self, root, policy):
        data = json.loads(policy.read_text())
        data['sourcePins'] = {str(path.relative_to(root)): hashlib.sha256(path.read_bytes()).hexdigest()
                              for directory in ('benchmark', 'tests', 'scripts')
                              for path in sorted((root / directory).rglob('*.py'))}
        policy.write_text(json.dumps(data))

    def invoke(self, base, root, policy, *extra):
        out = base / 'out'
        command = [sys.executable, '-B', str(RUNNER), '--repo', str(root),
                   '--out-dir', str(out), '--policy', str(policy), *extra]
        result = subprocess.run(command, capture_output=True, text=True)
        if os.environ.get('CI_CANDIDATE_CONTROL_LOG'):
            receipt = out / 'RESULT.json'
            with Path(os.environ['CI_CANDIDATE_CONTROL_LOG']).open('a') as stream:
                stream.write(json.dumps(dict(test=self.id(), argv=command, exitCode=result.returncode,
                    runnerSha256=hashlib.sha256(RUNNER.read_bytes()).hexdigest(),
                    policySha256=hashlib.sha256(policy.read_bytes()).hexdigest(),
                    stdout=result.stdout, stderr=result.stderr,
                    receipt=json.loads(receipt.read_text()) if receipt.exists() else None)) + '\n')
        return result, out

    def test_held_not_executed_and_ordinary_skip_separate(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, _ = self.fixture(base)
            result, out = self.invoke(base, root, policy)
            self.assertEqual(result.returncode, 0, result.stderr)
            receipt = json.loads((out / 'RESULT.json').read_text())
            self.assertEqual((receipt['discoveredMethods'], receipt['selectedMethods'], receipt['reportedTestsRun']), (3, 2, 2))
            self.assertEqual(len(receipt['ordinarySkips']), 1)
            self.assertIsNone(receipt['performance'])
            self.assertFalse(receipt['wholeQualityWorkflowPassed'])
            self.assertFalse(receipt['independentApproval'])
            self.assertFalse(receipt['fullOriginalCIPass'])
            self.assertFalse(receipt['releaseApproval'])

    def test_default_observers_rejected_before_original_called(self):
        for body in ('run.measure([])', 'run.environment()'):
            with self.subTest(body=body), tempfile.TemporaryDirectory() as name:
                base = Path(name)
                root, policy, observer = self.fixture(base, body)
                result, out = self.invoke(base, root, policy)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse(observer.with_name('observer-called').exists())
                self.assertEqual(len(json.loads((out / 'RESULT.json').read_text())['forbiddenObserverAttempts']), 1)

    def test_explicit_disabled_path_allowed(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, observer = self.fixture(base, 'self.assertEqual(run.measure([], collect_performance=False), "disabled")')
            result, _ = self.invoke(base, root, policy)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(observer.with_name('observer-called').read_text(), 'measure')

    def test_low_level_observers_blocked(self):
        for body in ('os.wait4(0, 0)', 'resource.getrusage(resource.RUSAGE_SELF)', 'os.cpu_count()', 'os.getloadavg()'):
            with self.subTest(body=body), tempfile.TemporaryDirectory() as name:
                base = Path(name)
                root, policy, _ = self.fixture(base, body)
                result, out = self.invoke(base, root, policy)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(len(json.loads((out / 'RESULT.json').read_text())['forbiddenObserverAttempts']), 1)

    def test_invalid_limits_reach_original_validation_without_observation(self):
        bodies = (
            'import bounded_process; self.assertRaises(ValueError, bounded_process.execute, [], "unused", timeout=float("inf"), cpu_seconds=1)',
            'from task047 import study; self.assertRaises(ValueError, study.run_target, "invalid", [], [{}], "unused", "unused", seconds=float("inf"), seed=1)',
        )
        for body in bodies:
            with self.subTest(body=body), tempfile.TemporaryDirectory() as name:
                base = Path(name)
                root, policy, _ = self.fixture(base, body)
                result, out = self.invoke(base, root, policy)
                self.assertEqual(result.returncode, 0, result.stderr)
                receipt = json.loads((out / 'RESULT.json').read_text())
                self.assertEqual(receipt['forbiddenObserverAttempts'], [])
                self.assertFalse((root / 'tests/quality/bounded-called').exists())
                self.assertFalse((root / 'tests/quality/task047/target-called').exists())

    def test_original_negative_validators_run_without_effects(self):
        import ast
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, _ = self.fixture(base)
            source = RUNNER.parents[1]
            for relative in ('bounded_process.py', 'task047/study.py',
                             'test_bounded_process.py', 'test_fuzz047.py'):
                destination = root / 'tests/quality' / relative
                destination.write_bytes((source / 'tests/quality' / relative).read_bytes())
            data = json.loads(policy.read_text())
            allowed = {
                'test_bounded_process.BoundedProcessTests.test_invalid_limits_are_rejected_before_execution',
                'test_fuzz047.Fuzz047Tests.test_unbounded_campaign_is_rejected_before_running_worker',
            }
            for relative in ('test_bounded_process.py', 'test_fuzz047.py'):
                path = root / 'tests/quality' / relative
                for cls in ast.parse(path.read_text()).body:
                    if not isinstance(cls, ast.ClassDef):
                        continue
                    for method in cls.body:
                        if isinstance(method, ast.FunctionDef) and method.name.startswith('test'):
                            ident = f'{path.stem}.{cls.name}.{method.name}'
                            data['expectedTests'].append(ident)
                            if ident not in allowed:
                                data['heldTests'].append(ident)
            policy.write_text(json.dumps(data))
            self.repin(root, policy)
            result, out = self.invoke(base, root, policy)
            self.assertEqual(result.returncode, 0, result.stderr)
            receipt = json.loads((out / 'RESULT.json').read_text())
            self.assertTrue(allowed.issubset(receipt['selectedTestIds']))
            self.assertEqual(receipt['reportedTestsRun'], 4)
            self.assertEqual(receipt['forbiddenObserverAttempts'], [])
            self.assertFalse(list((root / 'tests/quality').rglob('*-called')))

    def test_validated_continuations_are_forbidden_before_side_effects(self):
        bodies = (
            'import bounded_process; bounded_process.execute([], "unused", timeout=1, cpu_seconds=1)',
            'from task047 import study; study.run_target("valid", [], [{}], "unused", "unused", seconds=1, seed=1)',
            'import bounded_process; bounded_process._execute_validated([], "unused", timeout=1, cpu_seconds=1)',
            'from task047 import study; study._run_target_validated("valid", [], [{}], "unused", "unused", seconds=1, seed=1, max_cases=1)',
        )
        for body in bodies:
            with self.subTest(body=body), tempfile.TemporaryDirectory() as name:
                base = Path(name)
                root, policy, _ = self.fixture(base, body)
                result, out = self.invoke(base, root, policy)
                self.assertNotEqual(result.returncode, 0)
                receipt = json.loads((out / 'RESULT.json').read_text())
                self.assertEqual(len(receipt['forbiddenObserverAttempts']), 1)
                self.assertFalse((root / 'tests/quality/bounded-called').exists())
                self.assertFalse((root / 'tests/quality/task047/target-called').exists())

    def test_changed_source_pin_rejected(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, observer = self.fixture(base)
            observer.write_text(observer.read_text() + '\n')
            result, out = self.invoke(base, root, policy)
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(out.exists())

    def test_missing_held_id_rejected(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, _ = self.fixture(base)
            data = json.loads(policy.read_text())
            data['heldTests'] = ['test_fixture.Checks.test_missing']
            policy.write_text(json.dumps(data))
            result, _ = self.invoke(base, root, policy)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('held test missing', result.stderr)

    def test_existing_output_preserved(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, _ = self.fixture(base)
            (base / 'out').mkdir()
            marker = base / 'out/prior.log'
            marker.write_bytes(b'preserve')
            result, _ = self.invoke(base, root, policy)
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(marker.read_bytes(), b'preserve')

    def test_list_only_does_not_execute_selected(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, _ = self.fixture(base, 'raise AssertionError("must not execute")')
            result, out = self.invoke(base, root, policy, '--list-only')
            self.assertEqual(result.returncode, 0, result.stderr)
            receipt = json.loads((out / 'RESULT.json').read_text())
            self.assertEqual(receipt['state'], 'ListedOnly')
            self.assertIsNone(receipt['reportedTestsRun'])

    def test_unclassified_module_rejected_before_import(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, observer = self.fixture(base)
            (root / 'tests/quality/test_unknown.py').write_text(
                'from pathlib import Path\nPath(__file__).with_name("observer-called").write_text("imported")\n')
            result, out = self.invoke(base, root, policy)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('source path set changed', result.stderr)
            self.assertFalse(observer.with_name('observer-called').exists())
            self.assertFalse(out.exists())

    def test_unclassified_id_rejected_before_import(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, observer = self.fixture(base)
            test = root / 'tests/quality/test_fixture.py'
            test.write_text(test.read_text() +
                '    def test_unknown(self):\n        pass\n' +
                'from pathlib import Path\nPath(__file__).with_name("observer-called").write_text("imported")\n')
            self.repin(root, policy)
            result, out = self.invoke(base, root, policy)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('static test inventory changed', result.stderr)
            self.assertFalse(observer.with_name('observer-called').exists())
            self.assertFalse(out.exists())

    def test_missing_and_extra_source_paths_rejected_before_import(self):
        for change in ('missing', 'extra'):
            with self.subTest(change=change), tempfile.TemporaryDirectory() as name:
                base = Path(name)
                root, policy, observer = self.fixture(base)
                observer.write_text('from pathlib import Path\nPath(__file__).with_name("observer-called").write_text("imported")\n')
                self.repin(root, policy)
                if change == 'missing':
                    observer.rename(observer.with_suffix('.held'))
                else:
                    (root / 'scripts/unclassified.py').write_text('raise AssertionError("must not import")\n')
                result, out = self.invoke(base, root, policy)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn('source path set changed', result.stderr)
                self.assertFalse(observer.with_name('observer-called').exists())
                self.assertFalse(out.exists())

    def test_low_level_import_observers_blocked(self):
        for body in ('os.wait4(0, 0)', 'resource.getrusage(resource.RUSAGE_SELF)', 'os.cpu_count()', 'os.getloadavg()'):
            with self.subTest(body=body), tempfile.TemporaryDirectory() as name:
                base = Path(name)
                root, policy, observer = self.fixture(base)
                observer.write_text('import os, resource\n' + body + '\n' + observer.read_text())
                self.repin(root, policy)
                result, _ = self.invoke(base, root, policy)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn('resource observer forbidden', result.stderr)
                self.assertFalse(observer.with_name('observer-called').exists())

    def test_generated_id_rejected_after_guarded_discovery(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, _ = self.fixture(base, 'raise AssertionError("must not execute")')
            test = root / 'tests/quality/test_fixture.py'
            test.write_text(test.read_text() + 'setattr(Checks, "test_generated", lambda self: None)\n')
            self.repin(root, policy)
            result, _ = self.invoke(base, root, policy)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('discovered test inventory changed', result.stderr)

    def test_inherited_id_rejected_before_import(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, _ = self.fixture(base)
            test = root / 'tests/quality/test_fixture.py'
            test.write_text(test.read_text() + 'class Inherited(Checks):\n    def test_added(self):\n        pass\n')
            self.repin(root, policy)
            result, out = self.invoke(base, root, policy)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('unclassified test inheritance', result.stderr)
            self.assertFalse(out.exists())

    def test_load_tests_hook_rejected_before_import(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, _ = self.fixture(base)
            test = root / 'tests/quality/test_fixture.py'
            test.write_text(test.read_text() + 'def load_tests(*args):\n    raise AssertionError("must not execute")\n')
            self.repin(root, policy)
            result, out = self.invoke(base, root, policy)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('unclassified load_tests hook', result.stderr)
            self.assertFalse(out.exists())

    def test_held_indirect_and_child_consumers_never_execute(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, observer = self.fixture(base)
            child = root / 'scripts/child.py'
            child.write_text('from pathlib import Path\nPath(__file__).with_name("child-called").write_text("stub")\n')
            test = root / 'tests/quality/test_fixture.py'
            test.write_text(test.read_text() +
                'class Indirect(unittest.TestCase):\n'
                '    def test_default(self):\n        run.measure([])\n'
                '    def test_child(self):\n'
                '        import subprocess, sys\n'
                '        subprocess.run([sys.executable, ' + repr(str(child)) + '], check=True)\n')
            data = json.loads(policy.read_text())
            added = ['test_fixture.Indirect.test_default', 'test_fixture.Indirect.test_child']
            data['expectedTests'].extend(added)
            data['heldTests'].extend(added)
            policy.write_text(json.dumps(data))
            self.repin(root, policy)
            result, out = self.invoke(base, root, policy)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertFalse(observer.with_name('observer-called').exists())
            self.assertFalse(child.with_name('child-called').exists())
            receipt = json.loads((out / 'RESULT.json').read_text())
            self.assertEqual(receipt['selectedMethods'], 2)
            self.assertEqual(len(receipt['heldTests']), 3)

    def test_alternate_observer_entries_rejected_before_markers(self):
        calls = (
            ('bounded_process.execute([], "unused")', 'bounded_process._execute_validated'),
                        ('study.execute([], "unused")', 'bounded_process._execute_validated'),
            ('study.trial()', 'task047.study.trial'),
            ('study.run_target("x", [], [{"name": "x", "hex": ""}], "unused", "unused", seconds=1, seed=1)', 'task047.study._run_target_validated'),
        )
        for call, entry in calls:
            with self.subTest(entry=entry), tempfile.TemporaryDirectory() as name:
                base = Path(name)
                root, policy, _ = self.fixture(base, call)
                test = root / 'tests/quality/test_fixture.py'
                test.write_text('import bounded_process\nfrom task047 import study\n' + test.read_text())
                self.repin(root, policy)
                result, out = self.invoke(base, root, policy)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse(list((root / 'tests/quality').rglob('*-called')))
                receipt = json.loads((out / 'RESULT.json').read_text())
                self.assertEqual(receipt['forbiddenObserverAttempts'], [entry])

    def test_alternate_observer_import_calls_rejected_before_markers(self):
        calls = (
            'import bounded_process\nbounded_process.execute([], "unused")\n',
            'from task047 import study\nstudy.trial()\n',
            'from task047 import study\nstudy.run_target("x", [], [{"name": "x", "hex": ""}], "unused", "unused", seconds=1, seed=1)\n',
        )
        for call in calls:
            with self.subTest(call=call), tempfile.TemporaryDirectory() as name:
                base = Path(name)
                root, policy, _ = self.fixture(base)
                test = root / 'tests/quality/test_fixture.py'
                test.write_text(call + test.read_text())
                self.repin(root, policy)
                result, _ = self.invoke(base, root, policy)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse(list((root / 'tests/quality').rglob('*-called')))

    def test_shadow_observer_package_rejected_before_package_import(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, _ = self.fixture(base)
            package = root / 'benchmark/task047'
            package.mkdir()
            (package / '__init__.py').write_text(
                'from pathlib import Path\n'
                'Path(__file__).with_name("package-called").write_text("imported")\n')
            (package / 'study.py').write_text('raise AssertionError("must not import")\n')
            self.repin(root, policy)
            result, _ = self.invoke(base, root, policy)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('unexpected quality observer package', result.stderr)
            self.assertFalse((package / 'package-called').exists())

    def test_protocol_source_changes_rejected_before_any_project_import(self):
        for change in ('changed', 'extra'):
            with self.subTest(change=change), tempfile.TemporaryDirectory() as name:
                base = Path(name)
                root, policy, observer = self.fixture(base)
                protocol = root / 'tests/protocol'
                protocol.mkdir()
                source = protocol / 'container_entry.py'
                source.write_text('VALUE = 1\n')
                observer.write_text('from pathlib import Path\nPath(__file__).with_name("observer-called").write_text("imported")\n' + observer.read_text())
                self.repin(root, policy)
                if change == 'changed':
                    source.write_text('VALUE = 2\n')
                else:
                    (protocol / 'extra.py').write_text('VALUE = 1\n')
                result, out = self.invoke(base, root, policy)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse(observer.with_name('observer-called').exists())
                self.assertFalse(out.exists())

    def test_caught_observer_violation_still_fails(self):
        with tempfile.TemporaryDirectory() as name:
            base = Path(name)
            root, policy, observer = self.fixture(base, 'self.assertRaises(AssertionError, run.measure, [])')
            result, out = self.invoke(base, root, policy)
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(observer.with_name('observer-called').exists())
            receipt = json.loads((out / 'RESULT.json').read_text())
            self.assertEqual(receipt['failures'], 0)
            self.assertEqual(receipt['state'], 'Failed')
            self.assertEqual(receipt['forbiddenObserverAttempts'], ['run.measure'])


if __name__ == '__main__':
    unittest.main(verbosity=2)
