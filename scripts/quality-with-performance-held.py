import argparse
import ast
from contextlib import ExitStack
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys
import unittest
from unittest import mock


OBSERVER_ENTRIES = (
    ('run', 'tests/quality/run.py', ('measure', 'environment'), True),
    ('bounded_process', 'tests/quality/bounded_process.py', ('_execute_validated',), False),
    ('task047.study', 'tests/quality/task047/study.py', ('trial', '_run_target_validated'), False),
)


def cases(suite):
    for item in suite:
        if isinstance(item, unittest.TestSuite):
            yield from cases(item)
        else:
            yield item


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo', type=Path, required=True)
    parser.add_argument('--out-dir', type=Path, required=True)
    parser.add_argument('--policy', type=Path, default=Path(__file__).with_name('quality-performance-held-policy.json'))
    parser.add_argument('--list-only', action='store_true')
    args = parser.parse_args()
    root = args.repo.resolve(strict=True)
    policy = json.loads(args.policy.read_text())
    held, expected = policy['heldTests'], policy['expectedTests']
    if (policy['schemaVersion'] != 2 or not held or not expected
            or len(held) != len(set(held)) or len(expected) != len(set(expected))):
        raise ValueError('invalid test policy')
    if policy['sourceRoots'] != ['benchmark', 'tests', 'scripts']:
        raise ValueError('unexpected source roots')
    if set(held) - set(expected):
        raise ValueError('held test missing from expected inventory')
    paths = {str(path.relative_to(root)): path
             for directory in policy['sourceRoots']
             for path in (root / directory).rglob('*.py')}
    if set(paths) != set(policy['sourcePins']):
        raise ValueError('source path set changed; policy review required')
    inventory = []
    for name, path in sorted(paths.items()):
        if path.is_symlink() or not path.resolve(strict=True).is_relative_to(root):
            raise ValueError('unsafe source pin')
        content = path.read_bytes()
        if hashlib.sha256(content).hexdigest() != policy['sourcePins'][name]:
            raise ValueError('source pin changed; policy review required')
        if path.parent != root / 'tests/quality' or not path.name.startswith('test_'):
            continue
        tree = ast.parse(content, filename=name)
        if any(isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and node.name == 'load_tests'
               for node in ast.walk(tree)):
            raise ValueError('unclassified load_tests hook; policy review required')
        for cls in tree.body:
            if not isinstance(cls, ast.ClassDef):
                continue
            methods = [node.name for node in cls.body
                       if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
                       and node.name.startswith('test')]
            if methods and [ast.unparse(base) for base in cls.bases] != ['unittest.TestCase']:
                raise ValueError('unclassified test inheritance; policy review required')
            inventory.extend(f'{path.stem}.{cls.name}.{method}' for method in methods)
    if sorted(inventory) != sorted(expected):
        raise ValueError('static test inventory changed; policy review required')
    args.out_dir.mkdir(parents=True, exist_ok=False)
    os.environ['PQC_CANONICAL_QUALITY'] = str(root / 'tests/quality')
    sys.path[:0] = [str(root / 'tests/quality'), str(root / 'benchmark')]
    violations = []

    def forbidden(name):
        def reject(*args, **kwargs):
            violations.append(name)
            raise AssertionError('resource observer forbidden: ' + name)
        return reject

    def require_disabled(function, name):
        def guarded(*args, **kwargs):
            if kwargs.get('collect_performance') is not False:
                return forbidden(name)(*args, **kwargs)
            return function(*args, **kwargs)
        return guarded

    with ExitStack() as guards:
        for name in ('wait4', 'cpu_count', 'getloadavg'):
            if hasattr(os, name):
                guards.enter_context(mock.patch.object(os, name, forbidden('os.' + name)))
        try:
            import resource
        except ImportError:
            resource = None
        if resource is not None:
            guards.enter_context(mock.patch.object(resource, 'getrusage', forbidden('resource.getrusage')))
        for module_name, relative_path, functions, disabled_path in OBSERVER_ENTRIES:
            if '.' in module_name:
                package_name = module_name.rsplit('.', 1)[0]
                package_spec = importlib.util.find_spec(package_name)
                package_path = str((root / relative_path).parent)
                if (package_spec is None or package_spec.origin is not None
                        or list(package_spec.submodule_search_locations or ()) != [package_path]):
                    raise ValueError('unexpected quality observer package')
            spec = importlib.util.find_spec(module_name)
            expected_path = root / relative_path
            if spec is None or spec.origin is None or Path(spec.origin).resolve() != expected_path:
                raise ValueError('unexpected quality observer module')
            module = importlib.import_module(module_name)
            if Path(module.__file__).resolve() != expected_path:
                raise ValueError('unexpected imported quality observer module')
            for function_name in functions:
                name = module_name + '.' + function_name
                replacement = (require_disabled(getattr(module, function_name), name)
                               if disabled_path else forbidden(name))
                guards.enter_context(mock.patch.object(module, function_name, replacement))
        discovered = list(cases(unittest.defaultTestLoader.discover(str(root / 'tests/quality'), pattern='test_*.py')))
        ids = [case.id() for case in discovered]
        if sorted(ids) != sorted(policy['expectedTests']):
            raise ValueError('discovered test inventory changed; policy review required')
        selected = [case for case in discovered if case.id() not in set(held)]
        result = None
        if not args.list_only:
            with (args.out_dir / 'unittest.log').open('x') as stream:
                result = unittest.TextTestRunner(stream=stream, verbosity=2).run(unittest.TestSuite(selected))
    success = result is not None and result.wasSuccessful() and not violations
    state = 'ListedOnly' if args.list_only else ('SelectedChecksPassedWithPerformanceHeld' if success else 'Failed')
    receipt = dict(state=state, discoveredMethods=len(ids), selectedMethods=len(selected),
                   heldTests=held, heldReason=policy['reason'],
                   heldReasons=policy.get('heldReasons', {}), policy=policy,
                   selectedTestIds=[case.id() for case in selected],
                   reportedTestsRun=result.testsRun if result is not None else None,
                   ordinarySkips=[dict(id=case.id(), reason=reason) for case, reason in result.skipped] if result is not None else None,
                   failures=len(result.failures) if result is not None else None,
                   errors=len(result.errors) if result is not None else None,
                   forbiddenObserverAttempts=violations, performance=None,
                   wholeQualityWorkflowPassed=False, fullOriginalCIPass=False,
                   releaseApproval=False, independentApproval=False)
    with (args.out_dir / 'RESULT.json').open('x') as stream:
        json.dump(receipt, stream, indent=2)
        stream.write('\n')
    print(json.dumps({key: receipt[key] for key in ('state', 'discoveredMethods', 'selectedMethods', 'reportedTestsRun', 'failures', 'errors')}))
    return 0 if args.list_only and not violations or success else 1


if __name__ == '__main__':
    raise SystemExit(main())
