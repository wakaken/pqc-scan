import datetime as dt
from dataclasses import dataclass
from fractions import Fraction
import json
import os
import hashlib
from pathlib import Path
import re
from types import MappingProxyType
from typing import ClassVar, Mapping, TypeAlias

import adapter
import boundary as b

POLICY_SHA256 = 'c52ab335e1220587d938686e8781d5683a7d258c54c868539e98dca13efe27df'
PHASES = {'primary-baseline': ('primary', 'baseline', 90), 'primary-after': ('primary', 'after', 90),
          'comparison-baseline': ('comparison', 'baseline', 8), 'comparison-after': ('comparison', 'after', 8)}
MAX_SECOND = 253402300799
CLOCK_CLOSURE = frozenset({'registry_snapshot', 'certificate_parser', 'raw_detector_outcome', 'validity_predicates',
                          'validity_clock_emission', 'observation_retention', 'native_conversion',
                          'native_assessment', 'generation_serializer', 'walker_selection_limits', 'nondefault_paths'})
REVIEW_SCOPES = frozenset({'implementation_correctness', 'test_adequacy', 'source_relation',
                           'producer_coverage', 'witness_tool_build', 'overlay_build'})
AUTHORS = frozenset({'execution_only_oss_reassessment', 'execution_only_harness_review_fixes',
                     'temporal_clock_plan_implementation', 'temporal_overlay_and_witness_implementation'})


AUTHORIZED_FIXTURES = MappingProxyType({
    'nodejs__node': ('deps/v8/test/mjsunit/regress/regress-crbug-808192.js',
                     '40f598a4cbe489dd7222e88582f32f93fe62e0fb', 'PermissionError', 1),
    'discourse__discourse': ('spec/fixtures/images/fake.jpg',
                            'e59fa523ee03a3f7f9b9ac66eb6344d33c383837', 'FileNotFoundError', 2),
})
HISTORICAL_SCOPE = 'full tracked tree, subject to explicit scanner exclusions and resource limits'
EXCLUSION_AUTHORIZATION = 'User reply: この2ファイルを対象から除外するよう要件を変更する'
EXCLUSION_REASON = 'Explicit user scope amendment after endpoint protection quarantined upstream test fixture'


def _manifest_document(reference, error):
    try:
        value = json.loads(b.read(reference), object_pairs_hook=b.approved._unique,
                           parse_constant=lambda _: b.require(False, error))
    except (ValueError, UnicodeError, RecursionError):
        raise b.Rejected(error) from None
    b.require(isinstance(value, (dict, list)), error)
    return value


def admit_recovered_entry(current, historical, error):
    b.require(isinstance(current, dict) and isinstance(historical, dict) and
              all(name in historical and name in current for name in
                  ('id', 'population', 'split', 'commit', 'scope', 'inputManifestSha256')), error)
    original = {key: value for key, value in historical.items() if key != 'root'}
    recovered = {key: value for key, value in current.items() if key != 'root'}
    if b.digest(original) == b.digest(recovered):
        return
    identity = historical['id']
    b.require(isinstance(identity, str) and identity in AUTHORIZED_FIXTURES, error)
    path, blob, failure, number = AUTHORIZED_FIXTURES[identity]
    unavailable = dict(path=path, gitBlob=blob, errorType=failure, errno=number)
    old_limits, new_limits = historical.get('inputLimitations'), current.get('inputLimitations')
    b.require(isinstance(old_limits, dict) and isinstance(new_limits, dict) and
              historical['population'] == current['population'] == 'comparison' and
              historical['scope'] == HISTORICAL_SCOPE and
              current['scope'] == HISTORICAL_SCOPE + '; excludes user-authorized upstream test fixture ' + path and
              b.digest(old_limits.get('unavailable')) == b.digest([unavailable]) and
              'authorizedExclusions' not in old_limits and 'unavailable' not in new_limits and
              b.digest(new_limits.get('authorizedExclusions')) == b.digest([
                  dict(unavailable, reason=EXCLUSION_REASON, authorization=EXCLUSION_AUTHORIZATION)]) and
              b.digest({key: value for key, value in old_limits.items() if key != 'unavailable'}) ==
              b.digest({key: value for key, value in new_limits.items() if key != 'authorizedExclusions'}) and
              b.digest({key: value for key, value in original.items() if key not in ('scope', 'inputLimitations')}) ==
              b.digest({key: value for key, value in recovered.items() if key not in ('scope', 'inputLimitations')}) and
              all(isinstance(historical.get(key), str) and historical[key] for key in
                  ('commit', 'root', 'historicalInputBindingSha256', 'inputManifest', 'inputManifestSha256')), error)
    manifest = b.ref(historical['inputManifest'])
    b.require(manifest['sha256'] == historical['inputManifestSha256'], error)
    rows = _manifest_document(manifest, error)
    b.require(isinstance(rows, list) and rows and
              all(isinstance(row, dict) and isinstance(row.get('path'), str) for row in rows) and
              len({row['path'] for row in rows}) == len(rows) and path not in {row['path'] for row in rows}, error)
    b.require(isinstance(current.get('root'), str), error)
    descriptor = b.approved._open(current['root'], directory=True)
    try:
        try:
            parts = Path(path).parts
            for part in parts[:-1]:
                child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_NONBLOCK,
                                dir_fd=descriptor)
                os.close(descriptor)
                descriptor = child
            os.lstat(parts[-1], dir_fd=descriptor)
        except FileNotFoundError:
            return
        except OSError:
            raise b.Rejected(error) from None
        raise b.Rejected(error)
    finally:
        os.close(descriptor)



@dataclass(frozen=True, order=True)
class UtcInstant:
    second: int
    nanosecond: int
    original_lexeme: str

    @property
    def ticks(self):
        return self.second * 1_000_000_000 + self.nanosecond

    def serialized(self, native=False):
        whole = self.original_lexeme[:19]
        n = self.nanosecond
        digits = 0 if not n else 3 if n % 1_000_000 == 0 else 6 if n % 1000 == 0 else 9
        fraction = '.' + f'{n:09d}'[:digits] if digits else ''
        return whole + fraction + ('+00:00' if native else 'Z')


def instant(value):
    b.require(isinstance(value, str) and len(value) <= 40, 'E_CLOCK_DATE')
    match = re.fullmatch(r'(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})(?:\.(\d{1,9}))?(Z|\+00:00)', value)
    b.require(match is not None, 'E_CLOCK_DATE')
    year, month, day, hour, minute, second = map(int, match.groups()[:6])
    try:
        calendar = dt.date(year, month, day)
    except ValueError:
        raise b.Rejected('E_CLOCK_DATE') from None
    b.require(hour < 24 and minute < 60 and second < 60, 'E_CLOCK_DATE')
    seconds = (calendar.toordinal() - dt.date(1970, 1, 1).toordinal()) * 86400 + hour * 3600 + minute * 60 + second
    b.require(0 <= seconds <= MAX_SECOND, 'E_CLOCK_DATE')
    nanos = int((match.group(7) or '').ljust(9, '0'))
    return UtcInstant(seconds, nanos, value)


def unix_time(value):
    parsed = instant(value)
    return Fraction(parsed.ticks, 1_000_000_000)


def _second(value):
    b.require(type(value) is int and 0 <= value <= MAX_SECOND, 'E_CLOCK_VALUE')
    return value


def _metadata_seconds(report, first, last):
    b.require(isinstance(report.get('findings'), list), 'E_CLOCK_VALUE')
    values = set()
    for finding in report['findings']:
        b.require(isinstance(finding, dict) and isinstance(finding.get('rule_id'), str) and
                  isinstance(finding.get('evidence'), dict), 'E_CLOCK_PROVENANCE')
        evidence = finding['evidence']
        metadata = evidence.get('metadata', {})
        b.require(isinstance(metadata, dict), 'E_CLOCK_VALUE')
        dependent = finding['rule_id'] in {'CERT_EXPIRED', 'CERT_NOT_YET_VALID'} or 'validity_status' in metadata
        value = metadata.get('assessment_time_unix')
        if 'assessment_time_unix' in metadata:
            b.require(evidence.get('type') == 'certificate' and metadata.get('detector') == 'certificate_detector', 'E_CLOCK_PROVENANCE')
            b.require(isinstance(value, str) and re.fullmatch(r'0|[1-9][0-9]{0,11}', value), 'E_CLOCK_VALUE')
            clock = _second(int(value))
            b.require(first <= clock <= last, 'E_CLOCK_WINDOW')
            values.add(clock)
        b.require(not dependent or value is not None, 'E_CLOCK_MISSING')
    b.require(len(values) <= 1, 'E_CLOCK_CARDINALITY')
    return values


def _clock_times(report, native, started_utc):
    generated, started = instant(report['generated_at']), instant(started_utc)
    native_generated = instant(native['run']['generated_at'])
    b.require(started.ticks <= generated.ticks <= started.ticks + 601_000_000_000, 'E_CLOCK_WINDOW')
    b.require(generated.ticks == native_generated.ticks and report['generated_at'] == generated.serialized() and
              native['run']['generated_at'] == generated.serialized(native=True), 'E_CLOCK_SERIALIZATION')
    return started, generated


def _original_clock(report, native, started_utc):
    started, generated = _clock_times(report, native, started_utc)
    values = _metadata_seconds(report, started.second, generated.second)
    common = dict(originalGeneratedAt=report['generated_at'], originalNativeGeneratedAt=native['run']['generated_at'],
                  originalStartedUtc=started_utc, generatedSecond=generated.second, generatedNanosecond=generated.nanosecond,
                  firstPossibleSecond=started.second, lastPossibleSecond=generated.second)
    return started, generated, common, values


def derive_clock(report, native, started_utc, proof=None):
    started, generated, common, values = _original_clock(report, native, started_utc)
    if values:
        return dict(common, assessmentTimeUnix=values.pop(), kind='observed_original_second')
    b.require(isinstance(proof, _CoverageProof), 'E_CLOCK_UNPROVEN_INTERVAL')
    proof.prove_interval(started.second, generated.second)
    return dict(common, assessmentTimeUnix=started.second, kind='interval_equivalent_not_observed', witnessProof=proof.binding)


@dataclass(frozen=True)
class _CoverageProof:
    binding: dict
    objects: tuple
    emitted_clock_objects: tuple

    boundary_failures: tuple = ()
    original_inspection: str | None = None
    coverage_inspection: str | None = None

    def prove_interval(self, first, last):
        for before, after in self.objects:
            b.require(before <= after and (last < before or first >= before and last <= after or first > after),
                      'E_CLOCK_INTERVAL_TRANSITION')
        b.require(first == last or not self.emitted_clock_objects, 'E_CLOCK_EXACT_EMISSION_UNPROVEN')


def _review(reference, subjects, source, harness, authors):
    review = b.document(reference)
    b.require(review.get('kind') == 'temporal-independent-review-v1' and review.get('verdict') == 'PASS' and
              review.get('reviewerModel') == 'codex:gpt-6.1-sol@high' and
              isinstance(review.get('reviewerId'), str) and review['reviewerId'] and
              review['reviewerId'] not in AUTHORS | frozenset(authors) and
              set(review.get('scopes', [])) == REVIEW_SCOPES and review.get('subjects') == subjects and
              review.get('sourceBindings') == source and review.get('harnessBindings') == harness, 'E_TEMPORAL_REVIEW')
    return review


def _binding(value):
    b.require(isinstance(value, dict) and set(value) == {'path', 'sha256', 'bytes'} and
              isinstance(value['path'], str) and re.fullmatch(r'[0-9a-f]{64}', value['sha256']) is not None and
              type(value['bytes']) is int and 0 <= value['bytes'] <= b.MAX, 'E_TEMPORAL_BINDING')
    return value


def _raw_document(reference):
    return b.approved.decode(b.read(_binding(reference)))


def admit_temporal(reference, source, harness, authors, overlay_build):
    temporal = b.document(reference)
    b.require(temporal.get('kind') in {'temporal-replay-admission-v1', 'temporal-replay-admission-v2'} and temporal.get('sourceBindings') == source and
              temporal.get('harnessBindings') == harness and temporal.get('overlayBuildReceipt') == overlay_build and
              set(temporal.get('authorIds', [])) >= AUTHORS | frozenset(authors), 'E_TEMPORAL_ADMISSION')
    v2 = temporal['kind'] == 'temporal-replay-admission-v2'
    names = ('sourceRelation', 'witnessBuildReceipt', 'witnessBinary', 'witnessToolSource', 'witnessRules', 'coverageIndex', 'overlayBuildReceipt')
    subjects = {name: _binding(temporal[name]) for name in names + (('boundaryPolicyRelation',) if v2 else ())}
    b.require(v2 or 'boundaryPolicyRelation' not in temporal, 'E_TEMPORAL_ADMISSION')
    _review(temporal['review'], subjects, source, harness, temporal['authorIds'])
    relation = b.document(subjects['sourceRelation'])
    b.require(relation.get('kind') == 'temporal-source-relation-v1' and relation.get('finalSourceBindings') == source and
              set(relation.get('clockClosure', {})) == CLOCK_CLOSURE and
              set(relation.get('historicalSourceBindings', {})) == {'primary-after', 'comparison-after'}, 'E_TEMPORAL_SOURCE_RELATION')
    if v2:
        b.require(relation.get('boundaryPolicyRelation') == subjects['boundaryPolicyRelation'], 'E_BOUNDARY_POLICY_RELATION')
        adapter.boundary_policy_relation(subjects['boundaryPolicyRelation'], relation)
    else:
        b.require('boundaryPolicyRelation' not in relation, 'E_TEMPORAL_SOURCE_RELATION')
    for evidence in relation['clockClosure'].values():
        b.read(_binding(evidence))
    b.require(isinstance(relation.get('classifiedHunks'), list), 'E_TEMPORAL_SOURCE_RELATION')
    for hunk in relation['classifiedHunks']:
        b.require(set(hunk) == {'phaseId', 'path', 'beforeSha256', 'afterSha256', 'classification', 'evidence'} and
                  hunk['phaseId'] in relation['historicalSourceBindings'] and
                  hunk['classification'] in {'clock_transport', 'time_independent_presentation', 'time_independent_repair'} and
                  relation['historicalSourceBindings'][hunk['phaseId']].get(hunk['path']) == hunk['beforeSha256'] and
                  source.get(hunk['path']) == hunk['afterSha256'], 'E_TEMPORAL_UNCLASSIFIED_HUNK')
        b.read(_binding(hunk['evidence']))
    for phase, old in relation['historicalSourceBindings'].items():
        changed = {path for path in set(old) | set(source) if old.get(path) != source.get(path)}
        classified = [h['path'] for h in relation['classifiedHunks'] if h['phaseId'] == phase]
        b.require(len(classified) == len(set(classified)) and changed == set(classified), 'E_TEMPORAL_UNCLASSIFIED_HUNK')
    build = b.document(subjects['witnessBuildReceipt'])
    b.require(build.get('kind') == 'temporal-witness-build-receipt-v1' and build.get('exitCode') == 0 and
              build.get('binary') == subjects['witnessBinary'] and build.get('toolSource') == subjects['witnessToolSource'] and
              build.get('sourceBindings') == source and build.get('rules') == subjects['witnessRules'] and
              build.get('linkedSourceEvidence') is not None and build.get('boundaryTests') is not None, 'E_TEMPORAL_WITNESS_BUILD')
    if v2:
        b.require(adapter.boundary_policy_relation(subjects['boundaryPolicyRelation'], relation)['linkedBuild'] ==
                  build['linkedSourceEvidence'], 'E_BOUNDARY_POLICY_BUILD')
        b.require(build.get('witnessSchema') == adapter.WITNESS_SCHEMA_V2 and
                  build.get('schemaAuthority') == b.ref(b.CODE_DIR / 'adapter.py'), 'E_TEMPORAL_WITNESS_BUILD')
    else:
        b.require('witnessSchema' not in build and 'schemaAuthority' not in build, 'E_TEMPORAL_WITNESS_BUILD')
    for name in ('witnessBinary', 'witnessToolSource', 'witnessRules'):
        b.read(subjects[name])
    for name in ('linkedSourceEvidence', 'boundaryTests'):
        b.read(_binding(build[name]))
    rules = b.document(subjects['witnessRules'])
    b.require(rules.get('kind') == 'temporal-witness-rules-v1' and rules.get('defaultRuleCount') == 139 and
              isinstance(rules.get('files'), list) and isinstance(rules.get('defaultValidityRuleIds'), list) and
              len(rules['defaultValidityRuleIds']) == len(set(rules['defaultValidityRuleIds'])), 'E_TEMPORAL_RULES')
    expected_rules = {Path(path).name: sha for path, sha in source.items() if path.startswith('rules/default/')}
    b.require(expected_rules and len(expected_rules) == sum(path.startswith('rules/default/') for path in source), 'E_TEMPORAL_RULES')
    expected_rules['temporal-private-witness.yml'] = hashlib.sha256(adapter.WITNESS_RULE).hexdigest()
    actual_rules = {}
    for item in rules['files']:
        b.require(set(item) == {'path', 'sha256', 'bytes'} and Path(item['path']).name == item['path'] and
                  item['path'] not in actual_rules and type(item['bytes']) is int and 0 <= item['bytes'] <= 65536,
                  'E_TEMPORAL_RULES')
        actual_rules[item['path']] = item['sha256']
        if item['path'] == 'temporal-private-witness.yml':
            b.require(item['bytes'] == len(adapter.WITNESS_RULE), 'E_TEMPORAL_RULES')
    b.require(actual_rules == expected_rules, 'E_TEMPORAL_RULES')
    _, rows = _coverage_index(subjects['coverageIndex'])
    return temporal, relation, rows


def _coverage_index(reference):
    index = b.document(reference)
    kind, values = index.get('kind'), index.get('rows')
    b.require(kind in {'temporal-coverage-index-v1', 'temporal-required-raw-index-v2'} and
              isinstance(values, list) and len(values) <= 98 and
              (kind != 'temporal-coverage-index-v1' or len(values) == 98), 'E_TEMPORAL_COVERAGE_MATRIX')
    rows = {}
    for row in values:
        b.require(isinstance(row, dict) and
                  set(row) == {'repositoryId', 'phaseId', 'coverage', 'witness', 'config', 'producerReceipt'} and
                  isinstance(row['repositoryId'], str) and row['repositoryId'] and
                  isinstance(row['phaseId'], str) and row['phaseId'] in {'primary-after', 'comparison-after'},
                  'E_TEMPORAL_COVERAGE_MATRIX')
        key = row['repositoryId'], row['phaseId']
        b.require(key not in rows, 'E_TEMPORAL_COVERAGE_MATRIX')
        for name in ('coverage', 'witness', 'config', 'producerReceipt'):
            _binding(row[name])
        rows[key] = row
    return kind, rows


def _boundary_inspection(original):
    assessment = original['analysisStatus']
    declared, coverage = assessment.get('inspection'), assessment.get('coverage', {})
    keys = ('partial_files', 'analysis_failed_files', 'unknown_files', 'unsupported_files',
            'read_failed_files', 'enumeration_failed_entries')
    b.require(declared in {'Complete', 'Partial'} and all(type(coverage.get(key)) is int and
              coverage[key] >= 0 for key in keys), 'E_BOUNDARY_INSPECTION')
    oss, _, _ = b.import_auditors()
    limitations = original['input'].get('limitations', {})
    derived = oss.inspection_state(coverage, limitations)
    legacy = original.get('harness', {}).get('tests/quality/oss.py') == oss.LEGACY_ENUMERATION_ASSESSOR
    b.require(declared == derived or declared == 'Complete' and derived == 'Partial' and legacy and
              oss.inspection_state(dict(coverage, enumeration_failed_entries=0), limitations) == 'Complete',
              'E_BOUNDARY_INSPECTION')
    return declared, derived


def _boundary_v2(config, witness, coverage, temporal, relation, original, phase):
    b.require(config.get('kind') == adapter.WITNESS_SCHEMA_V2['configKind'] and
              set(config) == adapter.CONFIG_KEYS | adapter.BOUNDARY_CONFIG_KEYS and
              config['boundary_policy_relation'] == temporal.get('boundaryPolicyRelation') ==
              relation.get('boundaryPolicyRelation'), 'E_BOUNDARY_CONFIG')
    adapter.boundary_policy_relation(config['boundary_policy_relation'], relation)
    manifest = adapter.manifest_projection(original)
    diagnostics = adapter.original_diagnostic_projection(original)
    b.require(diagnostics['phaseId'] == phase and
              b.document(config['manifest_projection']) == manifest and
              b.document(config['original_diagnostic_projection']) == diagnostics and
              coverage.get('boundaryInputs') == {key: config[key] for key in
                  ('manifest_projection', 'original_diagnostic_projection', 'boundary_policy_relation')},
              'E_BOUNDARY_PROJECTION')
    pinned = {item['path']: item for item in manifest['entries']}
    expected = {}
    for item in diagnostics['entries']:
        b.require(item['code'] == 'read_failed' or adapter.original_nonboundary_diagnostic(item), 'E_BOUNDARY_DIAGNOSTIC')
        if item['code'] == 'read_failed':
            b.require(item['path'] in pinned and item['detector'] is None and
                      type(item['occurrences']) is int and item['occurrences'] == 1,
                      'E_BOUNDARY_DIAGNOSTIC')
            expected[item['path']] = pinned[item['path']]
    rows = witness['boundaryFailures']
    b.require(isinstance(rows, list) and len(rows) <= adapter.WITNESS_SCHEMA_V2['projectionRowLimit'], 'E_BOUNDARY_RECEIPTS')
    actual = {}
    for item in rows:
        b.require(isinstance(item, dict) and set(item) == set(adapter.WITNESS_SCHEMA_V2['boundaryKeys']) and
                  item.get('reason') == 'nofollow_symlink', 'E_BOUNDARY_RECEIPTS')
        path = adapter.relative_path(item.get('path'))
        b.require(path not in actual and path in expected and
                  {key: value for key, value in item.items() if key != 'reason'} == expected[path], 'E_BOUNDARY_RECEIPTS')
        actual[path] = item
    b.require(set(actual) == set(expected), 'E_BOUNDARY_RECEIPTS')
    counts, old = witness['coverage'], original['analysisStatus'].get('coverage', {})
    declared, derived = _boundary_inspection(original)
    b.require(type(old.get('enumeration_failed_entries')) is int and
              old['enumeration_failed_entries'] == counts['enumerationFailedEntries'] == len(rows) and
              counts['unknownFiles'] == 0 and
              witness['inspectionStatus'] == ('partial' if rows or counts['partialFiles'] or
                  counts['unsupportedFiles'] else 'complete_within_scope') and
              (not rows or witness['inspectionStatus'] == 'partial' and derived == 'Partial') and
              ('read_failed' in witness['diagnosticCodes']) == bool(rows), 'E_BOUNDARY_COUNTS')
    b.require(not set(actual) & {item.get('path') for item in witness['files']} and
              not set(actual) & {item.get('path') for item in witness['objects']}, 'E_BOUNDARY_RECEIPTS')
    return tuple(rows), declared, derived


def _coverage(row, temporal, relation, original, selected_entry, started, generated):
    coverage = b.document(row['coverage'])
    config_raw = b.read(row['config'], limit=65536)
    config = b.approved.decode(config_raw)
    witness = _raw_document(row['witness'])
    v2 = temporal.get('kind') == 'temporal-replay-admission-v2'
    schema = adapter.WITNESS_SCHEMA_V2 if v2 else adapter.WITNESS_SCHEMA
    b.require(config.get('kind') == schema.get('configKind') and
              set(config) == adapter.CONFIG_KEYS | (adapter.BOUNDARY_CONFIG_KEYS if v2 else set()),
              'E_TEMPORAL_WITNESS_CONFIG')
    producer = b.document(row['producerReceipt'])
    b.require(set(producer) == {'kind', 'execution', 'exitCode', 'timedOut', 'performance', 'scope', 'binary',
                  'configuration', 'output', 'sourceBindings', 'sourceBefore', 'sourceAfter', 'inputBefore', 'inputAfter',
                  'selectionBefore', 'selectionAfter', 'commandDigest', 'cwdDigest', 'startedUtc', 'endedUtc'} and
              producer.get('kind') == 'temporal-witness-process-v1' and producer.get('execution') == 'Succeeded' and
              producer.get('scope') == 'static_time_condition_proof' and
              type(producer.get('exitCode')) is int and producer['exitCode'] == 0 and producer.get('timedOut') is False and
              producer.get('performance') is None and producer.get('binary') == temporal['witnessBinary'] and
              producer.get('configuration') == row['config'] and producer.get('output') == row['witness'] and
              producer.get('sourceBindings') == producer.get('sourceBefore') == producer.get('sourceAfter') == temporal['sourceBindings'] and
              producer.get('inputBefore') == producer.get('inputAfter') == original['input']['manifest'] and
              producer.get('selectionBefore') == producer.get('selectionAfter') and
              producer.get('commandDigest') == b.digest([temporal['witnessBinary']['path'], row['config']['path']]) and
              producer.get('cwdDigest') == b.digest(str(b.ROOT)), 'E_TEMPORAL_PRODUCER_RECEIPT')
    b.require(instant(producer['startedUtc']).ticks <= instant(producer['endedUtc']).ticks, 'E_TEMPORAL_PRODUCER_RECEIPT')
    expected = dict(source=b.digest(temporal['sourceBindings']), rules=temporal['witnessRules']['sha256'],
                    input=selected_entry['inputManifestSha256'], selection=b.digest(original['selection']),
                    tool=temporal['witnessToolSource']['sha256'])
    b.require(coverage.get('kind') == ('temporal-producer-coverage-v2' if v2 else 'temporal-producer-coverage-v1') and coverage.get('bindings') == expected and
              coverage.get('repositoryId') == row['repositoryId'] and coverage.get('phaseId') == row['phaseId'] and
              coverage.get('originalSourceBindings') == relation['historicalSourceBindings'][row['phaseId']] and
              coverage.get('sourceRelation') == temporal['sourceRelation'] and coverage.get('witness') == row['witness'] and
              coverage.get('originalStatus') == original['analysisStatus'] and
              coverage.get('originalInput') == original['input'] and coverage.get('originalSelection') == original['selection'] and
              coverage.get('window') == {'firstSecond': started.second, 'lastSecond': generated.second} and
              set(coverage.get('clockClosure', [])) == CLOCK_CLOSURE, 'E_TEMPORAL_COVERAGE')
    current_root = config.get('root')
    if current_root == original['input']['root']:
        b.require('inputRelocation' not in coverage and producer['selectionBefore'] == original['selection'],
                  'E_TEMPORAL_INPUT_RELOCATION')
    else:
        import execution
        relocation = coverage.get('inputRelocation')
        b.require(isinstance(relocation, dict) and set(relocation) == {'kind', 'originalRoot', 'currentRoot', 'manifest',
                  'recoveryRegistry', 'registryEntry', 'selectionBefore', 'selectionAfter', 'sourceRelation', 'evidence'} and
                  relocation['kind'] == 'temporal-input-relocation-v1' and
                  relocation['originalRoot'] == original['input']['root'] and relocation['currentRoot'] == current_root and
                  relocation['manifest'] == original['input']['manifest'] and
                  relocation['selectionBefore'] == relocation['selectionAfter'] == producer['selectionBefore'] and
                  relocation['sourceRelation'] == temporal['sourceRelation'], 'E_TEMPORAL_INPUT_RELOCATION')
        registry = b.document(_binding(relocation['recoveryRegistry']))
        entries = registry.get('repositories', [])
        b.require(isinstance(entries, list), 'E_TEMPORAL_INPUT_RELOCATION')
        matches = [entry for entry in entries if isinstance(entry, dict) and entry.get('id') == row['repositoryId']]
        b.require(len(matches) == 1 and matches[0] == relocation['registryEntry'], 'E_TEMPORAL_INPUT_RELOCATION')
        recovered = matches[0]
        admit_recovered_entry(recovered, selected_entry, 'E_TEMPORAL_INPUT_RELOCATION')
        b.require(recovered['id'] == row['repositoryId'] and recovered['population'] + '-after' == row['phaseId'] and
                  recovered.get('root') == current_root and isinstance(current_root, str) and
                  Path(current_root).is_absolute() and Path(current_root).resolve(strict=True) == Path(current_root) and
                  recovered['inputManifestSha256'] == original['input']['manifest']['sha256'],
                  'E_TEMPORAL_INPUT_RELOCATION')
        manifest_binding = b.ref(recovered['inputManifest'])
        b.require(manifest_binding['sha256'] == original['input']['manifest']['sha256'] and
                  ('bytes' not in original['input']['manifest'] or
                   manifest_binding['bytes'] == original['input']['manifest']['bytes']),
                  'E_TEMPORAL_INPUT_RELOCATION')
        b.read(_binding(relocation['evidence']))
        execution.equivalent_selection(original['selection'], producer['selectionBefore'],
                                       original['input']['root'], current_root)
    b.require(config.get('detector_second') == started.second and type(config.get('detector_second')) is int and
              type(config.get('max_file_size_bytes')) is int and config['max_file_size_bytes'] == 2 * 1024**2 and
              type(config.get('threads')) is int and config['threads'] == 1 and
              config.get('selection') == {'include_non_scannable': False, 'exclude_tests': False} and
              all(type(value) is bool for value in config['selection'].values()) and config.get('bindings') == expected and
              config.get('rules_dir') == coverage.get('rulesDirectory') and
              config.get('witness_rule_id') == coverage.get('witnessRuleId'), 'E_TEMPORAL_WITNESS_CONFIG')
    rules = b.document(temporal['witnessRules'])
    b.require(config.get('rule_files') == rules['files'] and config.get('witness_rule_id') == adapter.WITNESS_SCHEMA['witnessRuleId'] and
              coverage.get('defaultValidityRules') == rules['defaultValidityRuleIds'], 'E_TEMPORAL_RULES')
    for item in config['rule_files']:
        b.read(dict(item, path=str(Path(config['rules_dir']) / item['path'])), limit=65536)
    b.require(set(witness) == set(schema['topKeys']) and
              isinstance(witness['files'], list) and all(isinstance(item, dict) for item in witness['files']) and
              isinstance(witness['objects'], list) and all(isinstance(item, dict) for item in witness['objects']) and
              witness['kind'] == schema['kind'] and witness['bindings'] == expected and
              witness['ruleCount'] == 140 and witness['status'] == adapter.WITNESS_SCHEMA['status'] and
              witness['configurationSha256'] == row['config']['sha256'] and
              witness['binarySha256'] == temporal['witnessBinary']['sha256'] and
              type(witness['detectorSecond']) is int and witness['detectorSecond'] == started.second, 'E_TEMPORAL_WITNESS')
    codes = witness['diagnosticCodes']
    b.require(isinstance(codes, list) and len(codes) == len(set(codes)) and
              set(codes) <= adapter.ALLOWED_DIAGNOSTICS | ({'read_failed'} if v2 else set()),
              'E_TEMPORAL_DIAGNOSTIC')
    for code in codes:
        proof = coverage.get('timeIndependentDiagnostics', {}).get(code)
        b.require(isinstance(proof, dict) and proof.get('closure') == temporal['sourceRelation'], 'E_TEMPORAL_DIAGNOSTIC')
        b.read(_binding(proof['evidence']))
    counts = witness['coverage']
    b.require(isinstance(counts, dict) and set(counts) == set(adapter.WITNESS_SCHEMA['coverageKeys']) and
              all(type(value) is int and value >= 0 for value in counts.values()) and
              counts['readFailedFiles'] == counts['analysisFailedFiles'] == counts['unknownFiles'] == 0 and
              (v2 or counts['enumerationFailedEntries'] == 0) and
              counts['selectedFiles'] == counts['readOkFiles'] == len(witness['files']) and
              counts['readOkFiles'] == sum(counts[name] for name in ('analyzedFiles', 'partialFiles', 'analysisFailedFiles',
                  'unsupportedFiles', 'notApplicableFiles', 'unknownFiles')), 'E_TEMPORAL_PRODUCER_COVERAGE')
    boundaries, original_inspection, coverage_inspection = _boundary_v2(
        config, witness, coverage, temporal, relation, original, row['phaseId']) if v2 else ((), None, None)
    if not v2:
        b.require('boundaryInputs' not in coverage, 'E_TEMPORAL_COVERAGE')
    original_coverage = original['analysisStatus'].get('coverage', {})
    b.require(all(type(original_coverage.get(old)) is int and original_coverage[old] == counts[new]
                  for old, new in (('selected_files', 'selectedFiles'), ('read_ok_files', 'readOkFiles'),
                                   ('read_failed_files', 'readFailedFiles'))), 'E_TEMPORAL_ORIGINAL_COVERAGE')
    b.require(coverage.get('files') == witness['files'] and coverage.get('producerCoverage') == witness['coverage'] and
              isinstance(witness['objects'], list) and len(witness['objects']) <= 1_000_000 and
              isinstance(witness['files'], list), 'E_TEMPORAL_PRODUCER_COVERAGE')
    manifest = _manifest_document(original['input']['manifest'], 'E_TEMPORAL_INPUT_MANIFEST')
    manifest_files = manifest.get('files', []) if isinstance(manifest, dict) else manifest
    if isinstance(manifest_files, dict):
        manifest_files = [dict(item, path=path) for path, item in manifest_files.items()]
    b.require(isinstance(manifest_files, list), 'E_TEMPORAL_INPUT_MANIFEST')
    input_files = {}
    for file in manifest_files:
        b.require(isinstance(file, dict) and isinstance(file.get('path'), str) and
                  file['path'] not in input_files, 'E_TEMPORAL_INPUT_MANIFEST')
        if file.get('mode') == '120000':
            b.require(set(file) in ({'mode', 'path', 'symlink'}, {'blob', 'mode', 'path', 'symlink'}) and
                      isinstance(file.get('symlink'), str) and file['symlink'] and '\0' not in file['symlink'],
                      'E_TEMPORAL_INPUT_MANIFEST')
            input_files[file['path']] = None
        else:
            b.require(file.get('mode') in (None, '100644', '100755') and 'symlink' not in file and
                      isinstance(file.get('sha256'), str) and
                      re.fullmatch('[0-9a-f]{64}', file['sha256']) is not None, 'E_TEMPORAL_INPUT_MANIFEST')
            input_files[file['path']] = file['sha256']
    selected_files = coverage.get('selectedFiles')
    b.require(isinstance(selected_files, dict) and len(selected_files) == len(witness['files']), 'E_TEMPORAL_SELECTED_FILES')
    file_rows = {}
    for file in witness['files']:
        b.require(set(file) == set(adapter.WITNESS_SCHEMA['fileKeys']), 'E_TEMPORAL_SELECTED_FILES')
        path = file['path']
        b.require(isinstance(path, str) and path and len(path) <= 4096 and not any(ord(char) < 32 for char in path) and
                  not Path(path).is_absolute() and Path(path).as_posix() == path and '..' not in Path(path).parts and
                  path not in file_rows and selected_files.get(path) == input_files.get(path) == file['sourceSha256'] and
                  re.fullmatch(r'[0-9a-f]{64}', file['sourceSha256']) is not None and
                  type(file['objects']) is int and file['objects'] >= 0, 'E_TEMPORAL_SELECTED_FILES')
        b.require(file['status'] in {'complete', 'partial', 'unsupported', 'not_applicable'}, 'E_TEMPORAL_FILE_STATUS')
        if file['status'] != 'complete':
            exclusion = coverage.get('timeIndependentStates', {}).get(path)
            b.require(isinstance(exclusion, dict) and exclusion.get('status') == file['status'] and
                      exclusion.get('sourceSha256') == file['sourceSha256'] and exclusion.get('closure') == temporal['sourceRelation'],
                      'E_TEMPORAL_UNPROVEN_STATE')
            b.read(_binding(exclusion['evidence']))
        file_rows[path] = file
    objects, emitted, counts, identities = [], [], {}, set()
    for obj in witness['objects']:
        b.require(set(obj) == {'path', 'sourceSha256', 'certificateSha256', 'objectIndex', 'notBeforeUnix', 'notAfterUnix',
                              'validityState', 'matchingDefaultValidityRuleIds'}, 'E_TEMPORAL_OBJECT')
        path = obj['path']
        b.require(path in file_rows and file_rows[path]['sourceSha256'] == obj['sourceSha256'] and
                  re.fullmatch(r'[0-9a-f]{64}', obj['certificateSha256']) is not None and
                  type(obj['objectIndex']) is int and obj['objectIndex'] >= 0, 'E_TEMPORAL_OBJECT')
        key = path, obj['objectIndex']
        b.require(key not in identities, 'E_TEMPORAL_OBJECT_DUPLICATE')
        identities.add(key)
        before, after = obj['notBeforeUnix'], obj['notAfterUnix']
        b.require(type(before) is int and type(after) is int and -62135596800 <= before <= after <= MAX_SECOND,
                  'E_TEMPORAL_ENDPOINTS')
        state = 'not_yet_valid' if started.second < before else 'expired' if started.second > after else 'valid'
        b.require(obj['validityState'] == state and isinstance(obj['matchingDefaultValidityRuleIds'], list) and
                  len(obj['matchingDefaultValidityRuleIds']) == len(set(obj['matchingDefaultValidityRuleIds'])) and
                  set(obj['matchingDefaultValidityRuleIds']) <= set(coverage.get('defaultValidityRules', [])), 'E_TEMPORAL_OBJECT_STATE')
        objects.append((before, after))
        if obj['matchingDefaultValidityRuleIds']:
            emitted.append(key)
        counts[path] = counts.get(path, 0) + 1
    b.require(all(file['objects'] == counts.get(path, 0) for path, file in file_rows.items()), 'E_TEMPORAL_OBJECT_LOSS')
    return _CoverageProof(row['coverage'], tuple(objects), tuple(emitted), boundaries,
                          original_inspection, coverage_inspection)


def admit_history(policy_binding, aliases):
    b.require(policy_binding['sha256'] == POLICY_SHA256, 'E_HISTORICAL_POLICY')
    policy = b.document(policy_binding)
    b.require(policy['kind'] == 'historical-score-policy' and len(policy['invocations']) == 4 and
              {row['id'] for row in policy['invocations']} == set(PHASES), 'E_HISTORICAL_PHASES')
    admitted = {}
    for row in policy['invocations']:
        population, phase, count = PHASES[row['id']]
        b.require((row['population'], row['phase']) == (population, phase), 'E_HISTORICAL_PHASE_ID')
        result = b.approved.preflight_invocation(row['binding'], required_artifacts=b.FORMATS, binary_aliases=aliases)
        invocation = result['invocation']
        b.require(invocation['phase'] == phase and invocation['expectedSamples'] == 1 and
                  len(invocation['selectedIds']) == count and
                  all(entry['population'] == population for entry in invocation['selectedEntries'].values()), 'E_HISTORICAL_SPLIT')
        admitted[row['id']] = result
    for population in ('primary', 'comparison'):
        before, after = (admitted[population + '-' + phase]['invocation'] for phase in ('baseline', 'after'))
        b.require(before['selectedIds'] == after['selectedIds'], 'E_HISTORICAL_MATRIX')
        for identity in before['selectedIds']:
            a, z = before['selectedEntries'][identity], after['selectedEntries'][identity]
            b.require(all(a[name] == z[name] for name in ('id', 'population', 'split', 'commit', 'inputManifestSha256', 'scope')),
                      'E_HISTORICAL_SOURCE')
    b.require(not set(admitted['primary-after']['invocation']['selectedIds']) &
              set(admitted['comparison-after']['invocation']['selectedIds']), 'E_HISTORICAL_DUPLICATE')
    return policy, admitted


def derive_all(policy_binding, aliases, temporal=None, relation=None, coverage_rows=None):
    cohort = OriginalClockCohort.admit(policy_binding, aliases, relation)
    b.require(temporal is not None and coverage_rows is not None, 'E_TEMPORAL_COVERAGE_MISSING')
    kind, indexed = _coverage_index(temporal['coverageIndex'])
    b.require(b.digest(list(indexed.values())) == b.digest(list(coverage_rows.values())), 'E_TEMPORAL_COVERAGE_MATRIX')
    expected_keys = set(cohort._after) if kind == 'temporal-coverage-index-v1' else cohort.required_keys
    b.require(set(coverage_rows) == expected_keys, 'E_CLOCK_MATRIX')
    result = json.loads(cohort._document)
    closed = set()
    for row in result['rows']:
        key = row['repositoryId'], row['phaseId']
        if key not in cohort._after:
            continue
        case = cohort._after[key]
        proof = None
        if key in coverage_rows:
            original = json.loads(case.original)
            proof = _coverage(coverage_rows[key], temporal, relation, original, json.loads(case.selected),
                              case.started, case.generated)
        clock = json.loads(case.clock)
        if isinstance(case, _NeedsIntervalCase):
            b.require(isinstance(proof, _CoverageProof), 'E_CLOCK_UNPROVEN_INTERVAL')
            proof.prove_interval(case.started.second, case.generated.second)
            clock.update(assessmentTimeUnix=case.started.second, kind='interval_equivalent_not_observed',
                         witnessProof=proof.binding)
        row['clock'] = clock
        ProjectReplay._from_row(row)
        closed.add(key)
    b.require(len(result['rows']) == 196 and len(closed) == 98 and closed == set(cohort._after), 'E_CLOCK_MATRIX')
    return result


@dataclass(frozen=True)
class ObservedOriginalSecond:
    second: int
    kind: ClassVar[str] = 'observed_original_second'


@dataclass(frozen=True)
class IntervalEquivalentNotObserved:
    second: int
    first_second: int
    last_second: int
    proof_digest: str
    kind: ClassVar[str] = 'interval_equivalent_not_observed'


DetectorTime: TypeAlias = ObservedOriginalSecond | IntervalEquivalentNotObserved


@dataclass(frozen=True, init=False)
class ProjectReplay:
    detector: DetectorTime
    generated_at: UtcInstant
    native_generated_at: str

    @classmethod
    def _from_row(cls, row):
        clock = row['clock']
        b.require(clock['kind'] in {'observed_original_second', 'interval_equivalent_not_observed'}, 'E_CLOCK_KIND')
        generated = instant(clock['originalGeneratedAt'])
        native = instant(clock['originalNativeGeneratedAt'])
        started = instant(clock['originalStartedUtc'])
        second = _second(clock['assessmentTimeUnix'])
        b.require(all(type(clock.get(name)) is int for name in
                  ('generatedSecond', 'generatedNanosecond', 'firstPossibleSecond', 'lastPossibleSecond')), 'E_CLOCK_PAIR')
        b.require(started.ticks <= generated.ticks <= started.ticks + 601_000_000_000 and
                  started.second <= second <= generated.second and generated.ticks == native.ticks and
                  generated.serialized() == generated.original_lexeme and generated.serialized(True) == native.original_lexeme and
                  (clock['generatedSecond'], clock['generatedNanosecond']) == (generated.second, generated.nanosecond) and
                  (clock['firstPossibleSecond'], clock['lastPossibleSecond']) == (started.second, generated.second), 'E_CLOCK_PAIR')
        b.require(clock['kind'] != 'interval_equivalent_not_observed' or second == started.second and 'witnessProof' in clock,
                  'E_CLOCK_PAIR')
        detector = (ObservedOriginalSecond(second) if clock['kind'] == 'observed_original_second' else
                    IntervalEquivalentNotObserved(second, started.second, generated.second, _binding(clock['witnessProof'])['sha256']))
        replay = object.__new__(cls)
        for name, value in (('detector', detector),
                            ('generated_at', generated), ('native_generated_at', native.original_lexeme)):
            object.__setattr__(replay, name, value)
        return replay

    @property
    def detector_second(self):
        return self.detector.second

    def bind_process(self, original_environment, ambient=None):
        environment = dict(ambient or {})
        for name in ('LANG', 'LC_ALL', 'LC_CTYPE', 'TZ', adapter.DETECTOR_ENV, adapter.GENERATED_ENV):
            environment.pop(name, None)
        for name in ('LANG', 'LC_ALL', 'LC_CTYPE', 'TZ'):
            value = original_environment.get(name)
            b.require(value is None or isinstance(value, str) and '\0' not in value, 'E_CLOCK_ENVIRONMENT')
            if value is not None:
                environment[name] = value
        environment[adapter.DETECTOR_ENV] = str(_second(self.detector_second))
        environment[adapter.GENERATED_ENV] = self.generated_at.original_lexeme
        parsed = instant(environment[adapter.GENERATED_ENV])
        b.require(parsed == self.generated_at and parsed.serialized() == environment[adapter.GENERATED_ENV], 'E_CLOCK_PAIR')
        return environment

    def verify_outputs(self, report, native):
        b.require(report.get('generated_at') == self.generated_at.serialized() and
                  native.get('run', {}).get('generated_at') == self.native_generated_at, 'E_CLOCK_OUTPUT_GENERATED')
        def check(value):
            if isinstance(value, dict):
                for name, item in value.items():
                    if name == 'assessment_time_unix':
                        b.require(type(item) is str and item == str(self.detector_second), 'E_CLOCK_OUTPUT_DETECTOR')
                    else:
                        check(item)
            elif isinstance(value, list):
                for item in value:
                    check(item)
        check(report)
        check(native)


@dataclass(frozen=True)
class _ObservedCase:
    selected: str
    original: str
    started: UtcInstant
    generated: UtcInstant
    clock: str
    replay: ProjectReplay


@dataclass(frozen=True)
class _NeedsIntervalCase:
    selected: str
    original: str
    started: UtcInstant
    generated: UtcInstant
    clock: str


@dataclass(frozen=True, init=False)
class OriginalClockCohort:
    _document: str
    _after: Mapping[tuple[str, str], _ObservedCase | _NeedsIntervalCase]
    required_keys: frozenset[tuple[str, str]]

    @classmethod
    def admit(cls, policy_binding, aliases, relation):
        policy, admitted = admit_history(policy_binding, aliases)
        oss, _, _ = b.import_auditors()
        document = dict(kind='execution-only-original-clocks-v2', policy=policy_binding,
                        phaseInvocations={row['id']: row['binding'] for row in policy['invocations']}, rows=[])
        after, keys = {}, set()
        for phase_id, admission in admitted.items():
            invocation, verified = oss._verified_invocation(admission['path'])
            if phase_id.endswith('-after'):
                b.require(relation is not None and invocation['scanner']['productSource']['sourceBindings'] ==
                          relation['historicalSourceBindings'][phase_id], 'E_TEMPORAL_HISTORICAL_SOURCE')
            for identity in invocation['selectedIds']:
                key = identity, phase_id
                b.require(key not in keys, 'E_CLOCK_MATRIX')
                keys.add(key)
                old_row, terminal, bindings = verified[(identity, 0)]
                attempt = Path(old_row['receipt']['path']).parent
                artifacts = terminal['artifacts']
                entry = invocation['selectedEntries'][identity]
                row = dict(repositoryId=identity, phaseId=phase_id, population=entry['population'], split=entry['split'],
                           commit=entry['commit'], inputManifestSha256=entry['inputManifestSha256'],
                           invocationPath=str(admission['path']), originalTerminal=old_row['receipt'],
                           originalReport=dict(path=str(attempt / 'output/report.json'), **artifacts['report.json']),
                           originalNative=dict(path=str(attempt / 'output/scan.inventory.v1.json'), **artifacts['scan.inventory.v1.json']))
                if phase_id.endswith('-after'):
                    started_meta = b.approved._metadata(attempt / 'started.json', terminal['startedDigest'])
                    report = b.approved.decode(b.approved.checked_file(attempt / 'output/report.json', artifacts['report.json'],
                                                                      limit=b.MAX, collect=True))
                    native = b.approved.decode(b.approved.checked_file(attempt / 'output/scan.inventory.v1.json',
                        artifacts['scan.inventory.v1.json'], limit=b.MAX, collect=True))
                    started, generated, clock, observed = _original_clock(report, native, started_meta['startedUtc'])
                    original_context = dict(bindings, analysisStatus=terminal['assessment'],
                        originalDiagnostics=dict(path=str(attempt / 'output/diagnostics.json'), **artifacts['diagnostics.json']),
                        originalTerminal=old_row['receipt'], originalPhaseId=phase_id)
                    adapter.original_diagnostic_projection(original_context)
                    common = dict(selected=json.dumps(entry, sort_keys=True),
                                  original=json.dumps(original_context, sort_keys=True),
                                  started=started, generated=generated)
                    if observed:
                        clock.update(assessmentTimeUnix=observed.pop(), kind='observed_original_second')
                        replay = ProjectReplay._from_row(dict(clock=clock))
                        case = _ObservedCase(**common, clock=json.dumps(clock, sort_keys=True), replay=replay)
                    else:
                        case = _NeedsIntervalCase(**common, clock=json.dumps(clock, sort_keys=True))
                    after[key] = case
                document['rows'].append(row)
        b.require(len(keys) == 196 and len(after) == 98, 'E_CLOCK_MATRIX')
        cohort = object.__new__(cls)
        object.__setattr__(cohort, '_document', json.dumps(document, sort_keys=True))
        object.__setattr__(cohort, '_after', MappingProxyType(after))
        object.__setattr__(cohort, 'required_keys', frozenset(key for key, case in after.items()
                                                          if isinstance(case, _NeedsIntervalCase)))
        return cohort

    def interval_jobs(self, registry):
        b.require(isinstance(registry, list) and len(registry) == 98 and
                  all(isinstance(row, dict) and isinstance(row.get('id'), str) for row in registry),
                  'E_PRODUCER_RECOVERY_MATRIX')
        recovered = {row['id']: row for row in registry}
        b.require(len(recovered) == 98 and set(recovered) == {key[0] for key in self._after}, 'E_PRODUCER_JOBS')
        for (identity, phase), case in self._after.items():
            admit_recovered_entry(recovered[identity], json.loads(case.selected), 'E_PRODUCER_RECOVERY_PIN')
        return [(identity, phase, recovered[identity], json.loads(case.selected), json.loads(case.original),
                 case.started, case.generated)
                for (identity, phase), case in self._after.items() if (identity, phase) in self.required_keys]

    def publish_interval_index(self, rows, destination):
        keys = [(row['repositoryId'], row['phaseId']) for row in rows]
        b.require(len(keys) == len(set(keys)) and set(keys) == self.required_keys, 'E_PRODUCER_INDEX')
        return b.publish(destination, dict(kind='temporal-required-raw-index-v2', rows=rows))


@dataclass(frozen=True, init=False)
class ClockPlan:
    _rows: Mapping[tuple[str, str], str]
    history: Mapping[str, dict]

    @classmethod
    def admit(cls, clock_map_binding, temporal_binding, aliases, source_bindings, author_ids, overlay_build):
        clock_map = b.document(clock_map_binding)
        b.require(clock_map.get('kind') == 'execution-only-original-clocks-v2' and
                  clock_map.get('policy', {}).get('sha256') == POLICY_SHA256, 'E_CLOCK_MAP_POLICY')
        policy, history = admit_history(clock_map['policy'], aliases)
        temporal, relation, coverage_rows = admit_temporal(temporal_binding, source_bindings, b.harness_bindings(), author_ids, overlay_build)
        expected = derive_all(clock_map['policy'], aliases, temporal, relation, coverage_rows)
        b.require(b.digest(clock_map) == b.digest(expected), 'E_CLOCK_MAP_DERIVATION')
        rows = {}
        for row in expected['rows']:
            key = row['repositoryId'], row['phaseId']
            b.require(key not in rows, 'E_CLOCK_MAP_MATRIX')
            rows[key] = json.dumps(row, sort_keys=True)
            if row['phaseId'].endswith('-after'):
                ProjectReplay._from_row(row)
            else:
                b.require('clock' not in row, 'E_BASELINE_CLOCK_FABRICATED')
        b.require(len(rows) == 196, 'E_CLOCK_MAP_MATRIX')
        plan = object.__new__(cls)
        object.__setattr__(plan, '_rows', MappingProxyType(rows))
        object.__setattr__(plan, 'history', MappingProxyType(history))
        return plan

    def for_project(self, repository_id, phase_id):
        b.require(phase_id in {'primary-after', 'comparison-after'} and (repository_id, phase_id) in self._rows,
                  'E_CLOCK_PROJECT')
        return ProjectReplay._from_row(json.loads(self._rows[(repository_id, phase_id)]))

    def __getitem__(self, key):
        return json.loads(self._rows[key])
