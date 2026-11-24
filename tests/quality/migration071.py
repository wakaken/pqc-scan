"""Bounded adapter for the amended task071 study; expectations stay separate."""
import argparse
import collections
import hashlib
import json
from pathlib import Path
import subprocess

from evidence import read_bytes
from judgment import compare


def digest(data):
    return hashlib.sha256(data).hexdigest()


def load(path):
    return json.loads(Path(path).read_text())


def validate_manifest(manifest):
    rows = manifest['cases']
    ids = [r['id'] for r in rows]
    if len(rows) != 24 or len(set(ids)) != 24:
        raise ValueError('Expected24 unique cases')
    pairs = collections.defaultdict(list)
    groups = collections.defaultdict(set)
    for row in rows:
        if row['split'] not in {'tuning', 'final'}:
            raise ValueError('Invalid split')
        pairs[row['pair']].append(row)
        groups[row['group']].add(row['split'])
    if (len(pairs) != 12 or any(len(v) != 2 or len({r['split'] for r in v}) != 1
                              for v in pairs.values())
            or any(len(v) != 1 for v in groups.values())
            or collections.Counter(r['split'] for r in rows) != {'tuning': 8, 'final': 16}):
        raise ValueError('Pair/family split leakage or invalid denominator')
    return rows


def verify_freeze(repo, freeze):
    if not freeze.get('bindings') or not freeze.get('frozen_at'):
        raise ValueError('Missing pre-observation freeze')
    for name, expected in freeze['bindings'].items():
        if digest(read_bytes(repo, name)) != expected:
            raise ValueError('Frozen input/source mismatch: ' + name)


def reference_integrity(report):
    """Mechanical reference joins only; no technical or policy judgments."""
    try:
        targets = {r['id'] for r in report['targets']}
        claims = {r['id'] for r in report['claims']}
        sources = {r['id'] for r in report['sources']}
        candidates = {r['id']: r for r in report['candidates']}
        if (len(targets) != len(report['targets']) or len(claims) != len(report['claims'])
                or len(sources) != len(report['sources'])
                or len(candidates) != len(report['candidates'])):
            return False
        return all(c['target_id'] in targets and set(c['source_ids']) <= sources
                   and all(set(p['applied_claim_ids']) <= claims for p in c['prerequisites'])
                   for c in candidates.values()) and all(
                   d['target_id'] in targets and set(d['applied_claim_ids']) <= claims
                   and set(d['source_ids']) <= sources
                   and all(c in candidates and candidates[c]['target_id'] == d['target_id']
                           for c in d['action']['candidate_ids'])
                   for d in report['decisions'])
    except (KeyError, TypeError):
        return False


def observe(repo, manifest, freeze, output):
    """Reads only frozen capture manifest; does not load the oracle or case truth."""
    rows = validate_manifest(manifest)
    verify_freeze(repo, freeze)
    cli = Path(repo) / manifest['cli']
    if digest(cli.read_bytes()) != freeze['cli_sha256']:
        raise ValueError('CLI mismatch before execution')
    output = Path(output)
    output.mkdir(parents=True, exist_ok=False)
    observations = []
    commands = []
    for row in rows:
        reports = []
        for repeat in (1, 2):
            out = output / row['id'] / str(repeat)
            argv = [str(cli), 'assess', str(Path(repo) / row['input']), '--input-format',
                    row['format'], '--context', str(Path(repo) / row['context']),
                    '--as-of', manifest['as_of'], '--lang', 'en', '--out-dir', str(out)]
            try:
                result = subprocess.run(argv, capture_output=True, text=True, timeout=20)
                code, stdout, stderr = result.returncode, result.stdout, result.stderr
            except subprocess.TimeoutExpired:
                code, stdout, stderr = None, '', 'timeout20s'
            commands.append(dict(id=row['id'], repeat=repeat, argv=argv,
                                 exitCode=code, stdout=stdout, stderr=stderr))
            if code == 0:
                report = load(out / 'migration.assessment.v1.json')
                # Only the observation adapter adds this mechanical fact. The exact
                # original product files remain preserved in each run directory.
                report['_study_reference_integrity'] = reference_integrity(report)
                reports.append(report)
            else:
                reports.append({'execution_error': stderr})
        observations.append(dict(id=row['id'], actual=reports[0], repeated=reports[1]))
    if digest(cli.read_bytes()) != freeze['cli_sha256']:
        raise ValueError('CLI changed during execution')
    (output / 'observations.json').write_text(json.dumps(observations, indent=2) + '\n')
    (output / 'commands.json').write_text(json.dumps(commands, indent=2) + '\n')
    return observations


def summarize(manifest, oracle, observations):
    rows = validate_manifest(manifest)
    expected_ids = {r['id'] for r in rows}
    if {r['id'] for r in oracle['cases']} != expected_ids:
        raise ValueError('Oracle/manifest population mismatch')
    # Existing v0.3 canonical comparator supplies strict joins/type distinctions.
    combined = compare(oracle, observations)
    by_id = {r['id']: r for r in observations}
    def property_counts(cases, observed):
        actual = {r['id']: r for r in observed}
        correct = incorrect = 0
        for case in cases:
            for check in case['checks']:
                result = compare(dict(cases=[dict(case, checks=[check])]), [actual[case['id']]])
                correct += result['correct']
                incorrect += result['incorrect']
        return dict(correct=correct, incorrect=incorrect, eligible=correct+incorrect)

    def aggregate(cases, observed, with_dimensions=True):
        result = dict(compare(dict(cases=cases), observed),
                      properties=property_counts(cases, observed))
        if with_dimensions:
            result['dimensions'] = {
                name: aggregate([dict(c, checks=[check for check in c['checks']
                                                 if check['dimension'] == name]) for c in cases],
                                observed, with_dimensions=False)
                for name in ('technical', 'conformance')}
        return result

    combined['properties'] = property_counts(oracle['cases'], observations)
    strata = {}
    for split in ('tuning', 'final'):
        ids = {r['id'] for r in rows if r['split'] == split}
        strata[split] = aggregate([c for c in oracle['cases'] if c['id'] in ids],
                                  [o for o in observations if o['id'] in ids])
    dimensions = {}
    for name in ('technical', 'conformance'):
        selected = dict(cases=[dict(c, checks=[check for check in c['checks']
                                             if check['dimension'] == name])
                               for c in oracle['cases']])
        dimensions[name] = aggregate(selected['cases'], observations, with_dimensions=False)
        dimensions[name]['eligible_assertions'] = sum(len(c['checks']) for c in selected['cases'])
    repeat = [by_id[r['id']]['actual'].get('format') == 'pqc-scan.migration-assessment'
              and by_id[r['id']]['repeated'].get('format') == 'pqc-scan.migration-assessment'
              and json.dumps(by_id[r['id']]['actual'], sort_keys=True, allow_nan=False)
              == json.dumps(by_id[r['id']]['repeated'], sort_keys=True, allow_nan=False)
              for r in rows]
    strata_by_input = {}
    for kind in sorted({r['format'] for r in rows}):
        ids = {r['id'] for r in rows if r['format'] == kind}
        strata_by_input[kind] = aggregate([c for c in oracle['cases'] if c['id'] in ids],
                                          [o for o in observations if o['id'] in ids])
    categorical = {}
    for field in ('role', 'algorithm', 'assurance'):
        categorical[field] = {}
        for value in sorted({r.get(field, 'not_declared') for r in rows}):
            ids = {r['id'] for r in rows if r.get(field, 'not_declared') == value}
            categorical[field][value] = aggregate(
                [c for c in oracle['cases'] if c['id'] in ids],
                [o for o in observations if o['id'] in ids])
    return dict(combined=combined, splits=strata, dimensions=dimensions,
                input_strata=strata_by_input, categorical_strata=categorical,
                reproducibility=dict(equal=sum(repeat), compared=len(repeat)),
                unassessed=oracle['unassessed'],
                human=dict(status='Not Tested', observed=0, planned_original_judgments=96,
                           rate=None),
                numerical_validity=dict(status='Not Tested', conclusion=None),
                interpretation='Finite synthetic technical/specification agreement only; '
                               'no human utility, population accuracy or score disposition')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('operation', choices=['observe', 'compare'])
    parser.add_argument('--repo', type=Path, required=True)
    parser.add_argument('--study', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    manifest = load(args.study / 'manifest.json')
    freeze = load(args.study / 'FREEZE.json')
    verify_freeze(args.repo, freeze)
    if args.operation == 'observe':
        observe(args.repo, manifest, freeze, args.output)
    else:
        result = summarize(manifest, load(args.study / 'oracle.json'),
                           load(args.output / 'observations.json'))
        (args.output / 'comparison.json').write_text(json.dumps(result, indent=2) + '\n')
        print(json.dumps(dict(correct=result['combined']['correct'],
                              incorrect=result['combined']['incorrect'],
                              reproducibility=result['reproducibility'])))


if __name__ == '__main__':
    main()
