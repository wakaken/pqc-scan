#!/usr/bin/env python3
"""Summarize verified OSS observations without accuracy or cryptographic-use inference."""
import argparse
import collections
import csv
import io
import json
import math
import os
import pathlib
import re
import sys
import unicodedata
import uuid

ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tests/quality'))
import oss
import run

LANGUAGES = ('C', 'C++', 'C#', 'Go', 'Java', 'JavaScript', 'PHP', 'Python',
             'Ruby', 'Rust', 'TypeScript')
EXTENSIONS = {'.c': 'C', '.cc': 'C++', '.cpp': 'C++', '.cxx': 'C++', '.hpp': 'C++',
              '.hh': 'C++', '.hxx': 'C++', '.cs': 'C#', '.go': 'Go', '.java': 'Java',
              '.js': 'JavaScript', '.jsx': 'JavaScript', '.mjs': 'JavaScript',
              '.cjs': 'JavaScript', '.php': 'PHP', '.py': 'Python', '.rb': 'Ruby',
              '.rs': 'Rust', '.ts': 'TypeScript', '.tsx': 'TypeScript',
              '.mts': 'TypeScript', '.cts': 'TypeScript'}
OTHER_LANGUAGES = ('C/C++ header unresolved', 'Manifest/configuration', 'Other/unattributed')
GROUP_ALIASES = {'JS/TS': 'JavaScript / TypeScript'}
COVERAGE_COUNTERS = ('selected_files', 'read_ok_files', 'read_failed_files', 'analyzed_files',
                     'partial_files', 'analysis_failed_files', 'unsupported_files',
                     'not_applicable_files', 'unknown_files', 'excluded_enumerated_files',
                     'enumeration_failed_entries')
RISKS = ('non-quantum-risk', 'quantum-safe', 'quantum-uncertain', 'quantum-vulnerable')
SEVERITIES = ('info', 'low', 'medium', 'high', 'critical')
MEASUREMENTS = ('wallSeconds', 'cpuSeconds', 'peakRssBytes')
RESOURCE_SCOPES = tuple(run.resource_scope(system=system, available=available)
                        for system in ('darwin', 'linux', 'other') for available in (False, True))


def location_language(filename):
    """Attribute only the finding location, never the repository's language declaration."""
    if not isinstance(filename, str):
        raise ValueError('Invalid finding location')
    name = pathlib.PurePosixPath(filename.replace('\\', '/')).name
    suffix = pathlib.PurePosixPath(name).suffix
    if suffix == '.C':
        return 'C++'
    suffix = suffix.lower()
    if suffix == '.h':
        return 'C/C++ header unresolved'
    if suffix in EXTENSIONS:
        return EXTENSIONS[suffix]
    if (suffix in {'.json', '.yaml', '.yml', '.toml', '.xml', '.lock', '.config', '.ini',
                   '.properties', '.csproj', '.fsproj', '.props', '.targets', '.gemspec', '.gradle'}
            or name in {'Gemfile', 'Gemfile.lock', 'go.mod', 'go.sum', 'Cargo.lock',
                        'CMakeLists.txt', 'requirements.txt', 'Pipfile', 'Pipfile.lock'}):
        return 'Manifest/configuration'
    return 'Other/unattributed'


def named_counts(values):
    counts = collections.Counter(values)
    if any(not isinstance(k, str) or not re.fullmatch(r'[A-Za-z0-9_.:/ -]{1,128}', k)
           for k in counts):
        raise ValueError('Invalid named report field')
    return dict(sorted(counts.items()))


def report_counts(report, expected_count):
    findings = report['findings']
    if len(findings) != expected_count:
        raise ValueError('Report finding count diverges from verified assessment')
    counts = {field: named_counts(f[field] for f in findings)
              for field in ('category', 'rule_id', 'risk', 'severity')}
    if set(counts['risk']) - set(RISKS) or set(counts['severity']) - set(SEVERITIES):
        raise ValueError('Unknown risk or severity')
    language = dict.fromkeys((*LANGUAGES, *OTHER_LANGUAGES), 0)
    for finding in findings:
        language[location_language(finding['location']['file'])] += 1
    summary = report['summary']
    if (set(summary['by_risk']) - set(RISKS) or set(summary['by_severity']) - set(SEVERITIES)
            or any(type(value) is not int or value < 0
                   for values in (summary['by_risk'], summary['by_severity']) for value in values.values())
            or any(type(summary[key]) is not int or summary[key] < 0
                   for key in ('total_findings', 'scanned_files', 'skipped_files'))):
        raise ValueError('Invalid named report summary')
    if (summary['total_findings'] != len(findings)
            or any({name: value for name, value in summary[key].items() if value} != counts[field]
                   for key, field in (('by_risk', 'risk'), ('by_severity', 'severity')))):
        raise ValueError('Report summary diverges from actual findings')
    return dict(findingCount=expected_count, nativeFields=counts,
                reportSummary=dict(total_findings=summary['total_findings'],
                                   by_risk=summary['by_risk'], by_severity=summary['by_severity'],
                                   scanned_files=summary['scanned_files'], skipped_files=summary['skipped_files']),
                locationLanguageCounts=language)


def measurement_projection(terminal):
    measurement = terminal.get('measurement')
    if measurement is None:
        return None
    values = {key: measurement.get(key) for key in MEASUREMENTS}
    if any(value is not None and (type(value) not in (float, int) or not math.isfinite(value)
                                 or value < 0) for value in values.values()):
        raise ValueError('Invalid resource observation')
    method, scope = measurement.get('resourceMethod'), measurement.get('resourceScope')
    if method not in (None, 'wait4 per child', 'unavailable', run.PERFORMANCE_DISABLED):
        raise ValueError('Unknown resource method')
    if method == run.PERFORMANCE_DISABLED and (any(value is not None for value in values.values())
            or scope is not None or measurement.get('rssNativeUnit') is not None):
        raise ValueError('Disabled resource observation contains performance data')
    known_scope = None
    if scope is not None:
        known_scope = next((known for known in RESOURCE_SCOPES if scope == known), None)
        if known_scope is None:
            raise ValueError('Unknown resource scope')
    values.update(resourceMethod=method, resourceScope=known_scope)
    return values


def invocation_observations(path, population, registry):
    supplied = oss._json(path)
    actual = pathlib.Path(supplied['receiptPath']) if 'receiptPath' in supplied else path
    invocation, verified = oss._verified_invocation(actual)
    expected = {row['id']: row for row in registry if row['population'] == population}
    if (set(invocation['selectedIds']) != set(expected)
            or invocation['selectedEntries'] != expected):
        raise ValueError('Invocation selection diverges from current population registry')
    if 'receiptPath' in supplied and {k: v for k, v in supplied.items() if k != 'receiptPath'} != invocation:
        raise ValueError('Invocation wrapper differs from immutable receipt')
    observations = collections.defaultdict(list)
    for (repository_id, sample), (row, terminal, _) in sorted(verified.items()):
        item = dict(sample=sample, execution=row['execution'], inspection='Unavailable',
                    findingCount=None, counts=None, coverage=None, measurement=None, accuracy=None)
        if row.get('receipt'):
            item['attempt'] = row['receipt']
            bound_terminal = terminal or oss._json(pathlib.Path(row['receipt']['path']))
            item['measurement'] = measurement_projection(bound_terminal)
        if terminal:
            assessment = terminal['assessment']
            item.update(inspection=oss.inspection_state(assessment['coverage'], assessment['inputLimitations']), findingCount=assessment['findingCount'],
                        coverage={key: assessment['coverage'][key] for key in COVERAGE_COUNTERS
                                  if key in assessment['coverage']})
            item['coverage']['not_enumerated_population'] = assessment['coverage'].get('not_enumerated_population')
            item['coverage']['status'] = assessment['coverage'].get('status')
            report = oss._json(pathlib.Path(row['receipt']['path']).parent / 'output/report.json')
            item['counts'] = report_counts(report, assessment['findingCount'])
        observations[repository_id].append(item)
    provenance = dict(invocation=oss._reference(actual), supplied=oss._reference(path),
                      phase=invocation['phase'], expectedSamples=invocation['expectedSamples'],
                      registry=invocation['registry'], harness=invocation['harness'],
                      scannerBinary=invocation['scanner']['binary'],
                      scannerBuildReceipt=invocation['scanner'].get('buildReceipt'),
                      rulesSha256=invocation['scanner']['rulesSha256'],
                      protocol=invocation['protocol'], configuration=invocation['configuration'])
    return observations, provenance


def aggregate(observations):
    successful = [r for r in observations if r['counts'] is not None]
    output = dict(observedSamples=len(observations), availableReportSamples=len(successful),
                  executionCounts=named_counts(r['execution'] for r in observations),
                  inspectionCounts=named_counts(r['inspection'] for r in observations),
                  findingCount=sum(r['findingCount'] for r in successful) if successful else None,
                  findingCountUnit='finding observations summed across successful samples',
                  nativeFields={}, reportSummary={}, locationLanguageCounts=None, coverage=None,
                  performanceByExecution={}, accuracy=None)
    for field in ('category', 'rule_id', 'risk', 'severity'):
        counts = collections.Counter()
        for row in successful:
            counts.update(row['counts']['nativeFields'][field])
        output['nativeFields'][field] = dict(sorted(counts.items())) if successful else None
    if successful:
        output['locationLanguageCounts'] = {lang: sum(r['counts']['locationLanguageCounts'][lang]
            for r in successful) for lang in (*LANGUAGES, *OTHER_LANGUAGES)}
        output['coverage'] = dict(counterTotals={key: (sum(r['coverage'].get(key, 0) for r in successful)
            if any(key in r['coverage'] for r in successful) else None) for key in COVERAGE_COUNTERS},
            counterAvailableSamples={key: sum(key in r['coverage'] for r in successful)
                                     for key in COVERAGE_COUNTERS},
            notEnumeratedPopulationSamples=sum(r['coverage']['not_enumerated_population'] is True for r in successful),
            unknownNotEnumeratedPopulationSamples=sum(r['coverage']['not_enumerated_population'] is None for r in successful),
            statusCounts=named_counts(r['coverage']['status'] or 'Unavailable' for r in successful),
            unavailableSamples=len(observations) - len(successful))
        output['reportSummary'] = {key: sum(r['counts']['reportSummary'][key] for r in successful)
            for key in ('total_findings', 'scanned_files', 'skipped_files')}
        for key in ('by_risk', 'by_severity'):
            counts = collections.Counter()
            for row in successful:
                counts.update(row['counts']['reportSummary'][key])
            output['reportSummary'][key] = dict(sorted(counts.items()))
    for execution in output['executionCounts']:
        selected = [r for r in observations if r['execution'] == execution]
        measured = [r['measurement'] for r in selected if r['measurement'] is not None
                    and any(r['measurement'][key] is not None for key in MEASUREMENTS)]
        output['performanceByExecution'][execution] = dict(measuredSamples=len(measured),
            unavailableSamples=len(selected) - len(measured),
            distributions=run.distributions(measured) if measured else None)
    return output


def summarize(registry_path, receipts):
    rows = oss.select_repositories(registry_path, oss.Selector(all=True))
    populations = collections.Counter(row['population'] for row in rows)
    if populations != {'primary': 90, 'comparison': 8}:
        raise ValueError('Expected the frozen primary90 and comparison8 registry')
    if {lang for row in rows for lang in row['languages']} != set(LANGUAGES):
        raise ValueError('Expected eleven declared language names')
    groups = {GROUP_ALIASES.get(row['group'], row['group']) for row in rows}
    if len(groups) != 9:
        raise ValueError('Expected nine normalized groups')
    registry_reference = oss._reference(registry_path)
    matrices, inputs = {}, {}
    for population in ('primary', 'comparison'):
        matrices[population], inputs[population] = {}, {}
        for phase in ('before', 'after'):
            matrix, provenance = invocation_observations(receipts[(population, phase)], population, rows)
            matrices[population][phase], inputs[population][phase] = matrix, provenance
    repositories = []
    for row in rows:
        result = dict(repositoryId=row['id'], population=row['population'], registryGroup=row['group'],
                      group=GROUP_ALIASES.get(row['group'], row['group']), declaredLanguages=row['languages'],
                      commit=row['commit'], split=row['split'], accuracy=None)
        for phase in ('before', 'after'):
            samples = matrices[row['population']][phase][row['id']]
            result[phase] = dict(samples=samples, aggregate=aggregate(samples))
        repositories.append(result)
    output = dict(schemaVersion=1, purpose='Descriptive counts of verified static scan observations',
                  registry=registry_reference, inputs=inputs,
                  tool=oss._reference(pathlib.Path(__file__)),
                  verificationSources={module.__name__: oss._reference(pathlib.Path(module.__file__).resolve())
                                       for module in (oss, run)},
                  accuracy=None, evaluationComplete=False,
                  accuracyReason='No independent qualified truth or false-negative denominator is admitted',
                  performanceInterpretation='Descriptive sample distributions only; no speedup claim. RSS is child high-water, not aggregate process-tree memory. Recorded resource scopes are retained with each sample.',
                  cryptographicFamilyMapping=None,
                  cryptographicFamilyMappingReason='Raw native category and rule counts are retained without inferring runtime use, purpose, reference semantics or PQC safety',
                  languageAttribution=dict(declaredLanguageNames=list(LANGUAGES),
                      method='Each finding location is assigned once by exact filename or extension; declarations do not split repository counts',
                      extensions=EXTENSIONS, extraBuckets=list(OTHER_LANGUAGES)),
                  groupAliases=GROUP_ALIASES, repositories=repositories, populations={})
    if any(not item['configuration'].get('collectPerformance', True)
           for phases in inputs.values() for item in phases.values()):
        output['performanceInterpretation'] = ('Performance collection disabled for one or more invocations; '
            'unmeasured observations are excluded from distributions. Performance evaluation is deferred.')
    for population in ('primary', 'comparison'):
        selected = [r for r in repositories if r['population'] == population]
        entry = dict(repositoryCount=len(selected), phases={}, groups={})
        for phase in ('before', 'after'):
            entry['phases'][phase] = aggregate([s for r in selected for s in r[phase]['samples']])
        for group in sorted({r['group'] for r in selected}):
            members = [r for r in selected if r['group'] == group]
            entry['groups'][group] = dict(repositoryCount=len(members), phases={phase: aggregate(
                [s for r in members for s in r[phase]['samples']]) for phase in ('before', 'after')})
        output['populations'][population] = entry
    return output


def csv_text(summary):
    output = io.StringIO(newline='')
    fields = ['repositoryId', 'population', 'registryGroup', 'group', 'declaredLanguages', 'commit', 'split']
    metrics = ('observedSamples', 'executionCounts', 'inspectionCounts', 'findingCount',
               'nativeFields', 'reportSummary', 'locationLanguageCounts', 'coverage', 'performanceByExecution')
    fields += [phase + '_' + metric for phase in ('before', 'after') for metric in metrics]
    fields += ['accuracy']
    writer = csv.DictWriter(output, fieldnames=fields, lineterminator='\n')
    writer.writeheader()
    for repository in summary['repositories']:
        row = {key: repository[key] for key in fields[:7]}
        row['declaredLanguages'] = json.dumps(row['declaredLanguages'], ensure_ascii=False)
        for phase in ('before', 'after'):
            for metric in metrics:
                value = repository[phase]['aggregate'][metric]
                row[phase + '_' + metric] = json.dumps(value, sort_keys=True, ensure_ascii=False, allow_nan=False)
        row['accuracy'] = 'null'
        if any(isinstance(value, str) and any(unicodedata.category(char) in {'Cc', 'Cf', 'Cs'}
                                             for char in value) for value in row.values()):
            raise ValueError('CSV metadata contains control characters')
        row = {key: ("'" + value if isinstance(value, str)
                     and value.lstrip().startswith(('=', '+', '-', '@', '\t', '\r', '\n')) else value)
               for key, value in row.items()}
        writer.writerow(row)
    return output.getvalue()


def publish_pair(output, csv_output, summary):
    paths = (pathlib.Path(output), pathlib.Path(csv_output))
    if paths[0].resolve() == paths[1].resolve() or any(p.exists() or p.is_symlink() for p in paths):
        raise ValueError('Outputs must be distinct new paths')
    texts = (json.dumps(summary, indent=2, ensure_ascii=False, allow_nan=False) + '\n', csv_text(summary))
    staged, published = [], []
    try:
        for path, text in zip(paths, texts):
            temporary = path.with_name('.' + path.name + '.' + uuid.uuid4().hex)
            with temporary.open('x', encoding='utf-8', newline='') as stream:
                staged.append(temporary)
                stream.write(text)
                stream.flush()
                os.fsync(stream.fileno())
        for temporary, path in zip(staged, paths):
            os.link(temporary, path)
            published.append((temporary, path))
    except OSError:
        for temporary, path in reversed(published):
            try:
                if os.path.samestat(temporary.stat(), path.lstat()):
                    path.unlink()
            except FileNotFoundError:
                pass
        raise
    finally:
        for temporary in staged:
            temporary.unlink()


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--registry', type=pathlib.Path, default=ROOT / 'benchmark/registry.json')
    for population in ('primary', 'comparison'):
        for phase in ('before', 'after'):
            parser.add_argument('--' + population + '-' + phase, type=pathlib.Path, required=True)
    parser.add_argument('--output', type=pathlib.Path, required=True)
    parser.add_argument('--csv-output', type=pathlib.Path, required=True)
    args = parser.parse_args(argv)
    try:
        receipts = {(population, phase): getattr(args, population + '_' + phase)
                    for population in ('primary', 'comparison') for phase in ('before', 'after')}
        result = summarize(args.registry, receipts)
        publish_pair(args.output, args.csv_output, result)
        print('Wrote descriptive JSON and 98-row CSV; accuracy remains null.')
        return 0
    except (OSError, ValueError, KeyError, TypeError) as error:
        print('OSS summary rejected: ' + type(error).__name__, file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
