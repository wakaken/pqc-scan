#!/usr/bin/env python3
"""Select, run and compare immutable descriptive OSS evidence."""
import argparse
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/quality'))
from oss import RunRequest, Selector, compare_runs, run_selected, select_repositories


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    for name in ('select', 'run'):
        child = commands.add_parser(name)
        child.add_argument('--registry', type=pathlib.Path, default=ROOT/'benchmark/registry.json')
        child.add_argument('--repo', action='append', default=[])
        child.add_argument('--group', action='append', default=[])
        child.add_argument('--population', choices=('primary', 'comparison'))
        child.add_argument('--priority', action='store_true')
        child.add_argument('--all', action='store_true')
        if name == 'run':
            child.add_argument('--phase', required=True)
            child.add_argument('--binary', required=True, type=pathlib.Path)
            child.add_argument('--rules', required=True, type=pathlib.Path)
            child.add_argument('--build-receipt', required=True, type=pathlib.Path)
            child.add_argument('--protocol', required=True, type=pathlib.Path)
            child.add_argument('--output', required=True, type=pathlib.Path)
            child.add_argument('--resume', action='store_true')
            child.add_argument('--timeout', type=float)
            child.add_argument('--min-free-gib', type=float)
            child.add_argument('--profile', default='default')
    compare = commands.add_parser('compare')
    compare.add_argument('--before', required=True, type=pathlib.Path)
    compare.add_argument('--after', required=True, type=pathlib.Path)
    compare.add_argument('--output', required=True, type=pathlib.Path)
    args = parser.parse_args(argv)
    try:
        if args.command == 'compare':
            result = compare_runs(args.before, args.after, args.output)
        else:
            selector = Selector(tuple(args.repo), tuple(args.group), args.population, args.priority, args.all)
            if args.command == 'select':
                result = select_repositories(args.registry, selector)
            else:
                result = run_selected(RunRequest(args.registry, selector, args.protocol, args.phase,
                    args.binary, args.rules, args.build_receipt, args.output, args.resume,
                    args.timeout, args.min_free_gib, args.profile))
        print(json.dumps(result, indent=2, ensure_ascii=False, allow_nan=False))
        return 0 if args.command != 'run' or result['executionComplete'] else 1
    except (OSError, ValueError, KeyError, TypeError) as error:
        print('OSS evidence request rejected: ' + type(error).__name__, file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
