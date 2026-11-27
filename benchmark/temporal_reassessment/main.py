import argparse
import json
import re
import uuid
import datetime as dt
from pathlib import Path
import sys

import adapter
import boundary as b
import clocks
import comparison
import execution
import fresh_cohort


def binding(text):
    try:
        value = json.loads(text)
        b.require(isinstance(value, dict) and set(value) in ({'path', 'sha256'}, {'path', 'sha256', 'bytes'}), 'E_CLI_BINDING')
        return value
    except (ValueError, TypeError):
        raise argparse.ArgumentTypeError('Invalid file binding; payload omitted.') from None


def review_gate(reference):
    review = b.document(reference)
    b.require(review['verdict'] == 'PASS' and review['reviewerModel'] == 'codex:gpt-6.1-sol@high' and
              review['reviewerId'] not in clocks.AUTHORS and
              review['harnessBindings'] == b.harness_bindings(), 'E_CODE_REVIEW_GATE')


def main(argv=None):
    parser = argparse.ArgumentParser(description='Execution-only static rescan, with authenticated compressed artifacts and historical certificate clocks.')
    commands = parser.add_subparsers(dest='command', required=True)
    derive = commands.add_parser('derive-clocks')
    derive.add_argument('--policy', type=binding, required=True)
    derive.add_argument('--temporal-admission', type=binding, required=True)
    derive.add_argument('--source', type=binding, required=True)
    derive.add_argument('--overlay-build', type=binding, required=True)
    derive.add_argument('--binary-aliases', type=binding)
    derive.add_argument('--review', type=binding, required=True)
    derive.add_argument('--output', type=Path, required=True)
    plan = commands.add_parser('plan-adapter')
    plan.add_argument('--envelope', type=binding, required=True)
    plan.add_argument('--source', type=binding, required=True)
    plan.add_argument('--review', type=binding, required=True)
    plan.add_argument('--destination', type=Path, required=True)
    run = commands.add_parser('run')
    run.add_argument('--admission', type=binding, required=True)
    run.add_argument('--output', type=Path, required=True)
    cohort_plan = commands.add_parser('plan-cohort')
    cohort_plan.add_argument('--admission', type=binding, required=True)
    cohort_plan.add_argument('--policy', type=binding, required=True)
    cohort_plan.add_argument('--output', type=Path, required=True)
    cohort_plan.add_argument('--run-output', type=Path)
    cohort_run = commands.add_parser('run-cohort')
    cohort_run.add_argument('--plan', type=binding, required=True)
    cohort_run.add_argument('--review', type=binding, required=True)
    cohort_run.add_argument('--output', type=Path)
    compare = commands.add_parser('compare')
    compare.add_argument('--index', type=binding, required=True)
    compare.add_argument('--allowances', type=binding, required=True)
    compare.add_argument('--review', type=binding, required=True)
    compare.add_argument('--output', type=Path, required=True)
    args = parser.parse_args(argv)
    b.require_external_storage()
    if args.command == 'derive-clocks':
        review_gate(args.review)
        aliases = b.document(args.binary_aliases)['aliases'] if args.binary_aliases else []
        source = b.document(args.source)['sourceBindings']
        b.verify_source(source)
        adapter.verify_build(b.document(args.overlay_build), source)
        temporal, relation, rows = clocks.admit_temporal(args.temporal_admission,
            source, b.harness_bindings(),
            ['temporal_clock_plan_implementation'], args.overlay_build)
        result = b.publish(args.output, clocks.derive_all(args.policy, aliases, temporal, relation, rows))
    elif args.command == 'plan-adapter':
        review_gate(args.review)
        source = b.document(args.source)['sourceBindings']
        b.mkdir(args.destination)
        result = b.publish(args.destination / 'build-plan.json', adapter.build_plan(b.document(args.envelope), args.destination, source))
    elif args.command == 'compare':
        review_gate(args.review)
        result = b.publish(args.output, comparison.compare_all(args.index, args.allowances))
    elif args.command == 'plan-cohort':
        result = fresh_cohort.prepare_plan(args.admission, args.policy, args.output, args.run_output)
    elif args.command == 'run-cohort':
        result, code = fresh_cohort.run(args.plan, args.review, args.output)
        print(json.dumps(dict(status='record_written', record=result), sort_keys=True))
        return code
    else:
        result = execution.run(args.admission, args.output)
    print(json.dumps(dict(status='record_written', record=result), sort_keys=True))
    return 0


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except (OSError, ValueError, KeyError, TypeError, AssertionError) as error:
        code = str(error) if isinstance(error, b.Rejected) and re.fullmatch(r'E_[A-Z0-9_]+', str(error)) else 'E_HARNESS_REJECTED'
        directory = b.OWN / 'rejections'
        if not directory.exists():
            b.mkdir(directory)
        record = dict(kind='execution-only-rejection-v1', errorCode=code, errorKind=type(error).__name__,
                      createdUtc=dt.datetime.now(dt.timezone.utc).isoformat(), payloadOmitted=True,
                      validation=getattr(error, 'evidence', None), evaluationComplete=False)
        reference = b.publish(directory / (uuid.uuid4().hex + '.json'), record)
        print(json.dumps(dict(status='rejected', errorCode=code, errorKind=type(error).__name__, record=reference, payloadOmitted=True)), file=sys.stderr)
        raise SystemExit(2) from None
