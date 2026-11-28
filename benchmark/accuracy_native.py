"""Mechanical, source-position-bound capture for the unchanged judgment comparator."""
import argparse
import hashlib
import importlib.util
import json
import pathlib
import re
import sys

if __package__:
    from . import artifact_preflight as boundary
else:
    import artifact_preflight as boundary

FAMILIES = {'RSA': 'RSA', 'Ed25519': 'Ed25519', 'X25519': 'X25519', 'ECDH': 'ECDH', 'ECDSA': 'ECDSA',
            'ML-KEM': 'ML-KEM', 'ML-KEM-512': 'ML-KEM', 'ML-KEM-768': 'ML-KEM', 'ML-KEM-1024': 'ML-KEM',
            'ML-DSA': 'ML-DSA', 'ML-DSA-44': 'ML-DSA', 'ML-DSA-65': 'ML-DSA', 'ML-DSA-87': 'ML-DSA'}
KINDS = {'algorithm_family_reference', 'explicit_rsa_key_bits', 'explicit_named_parameter_set'}
ID = re.compile(r'[a-z_]+-v1-[0-9a-f]{64}')
CAPTURE_STATES = {'legacy_reported':'complete', 'complete':'complete', 'partial':'partial', 'truncated':'partial',
                  'unsupported':'unsupported', 'failed':'failed', 'analysisfailed':'failed', 'parsefailed':'failed',
                  'unknown':'unknown', 'invalid_typed_facts_omitted':'unknown'}


def require(ok, code):
    boundary.require(ok, code)


def _relative(name):
    require(isinstance(name, str) and name and not name.startswith('/') and '\\' not in name and ':' not in name and
            all(part not in ('', '.', '..') for part in name.split('/')), 'E_NATIVE_SOURCE_PATH')
    return name


def _index(rows):
    require(isinstance(rows, list), 'E_NATIVE_ID')
    result = {}
    for row in rows:
        require(isinstance(row, dict) and isinstance(row.get('id'), str) and ID.fullmatch(row['id']) and
                row['id'] not in result, 'E_NATIVE_ID')
        result[row['id']] = row
    return result


def validate_native(native, source_manifest, schema):
    import jsonschema
    require(next(jsonschema.Draft7Validator(schema).iter_errors(native), None) is None, 'E_NATIVE_SCHEMA')
    assets, observations = _index(native['assets']), _index(native['observations'])
    for asset in assets.values():
        require(all(relation['target'] in assets for relation in asset['relations']), 'E_NATIVE_REFERENCE')
    for observation in observations.values():
        require(all(identity in assets for identity in observation['asset_ids']), 'E_NATIVE_REFERENCE')
        target = observation['target']
        start, end = target['byte_start'], target['byte_end']
        if target['path'] is not None:
            path = _relative(target['path'])
            require(path in source_manifest, 'E_NATIVE_UNINSPECTED_PATH')
            size = source_manifest[path]['bytes']
            require(start is None or type(start) is int and 0 <= start <= size, 'E_NATIVE_SPAN')
            require(end is None or type(end) is int and 0 <= end <= size, 'E_NATIVE_SPAN')
            require(start is None or end is None or start <= end, 'E_NATIVE_SPAN')
        else:
            require(start is None and end is None, 'E_NATIVE_SPAN')
    for ordinal, association in enumerate(native['legacy_associations']):
        require(association['legacy_index'] == ordinal and
                all(identity in observations for identity in association['observation_ids']), 'E_NATIVE_REFERENCE')
    require(all(identity in observations for identity in native['unassociated_observation_ids']), 'E_NATIVE_REFERENCE')
    require(all(row['observation_id'] in observations and row['count'] > 0 for row in native['legacy_loss_records']),
            'E_NATIVE_REFERENCE')
    return assets, observations


def _source_bytes(raw, binding, selector):
    require(isinstance(raw, bytes) and len(raw) <= 2 * 1024 * 1024 and
            hashlib.sha256(raw).hexdigest() == binding['sha256'], 'E_NATIVE_SOURCE_HASH')
    require(not raw.startswith((b'\xef\xbb\xbf', b'\xff\xfe', b'\xfe\xff', b'\x00\x00\xfe\xff')) and
            b'\0' not in raw, 'E_NATIVE_SOURCE_ENCODING')
    try:
        require(raw.decode('utf-8').encode('utf-8') == raw, 'E_NATIVE_SOURCE_ENCODING')
    except UnicodeError:
        raise boundary.Rejected('E_NATIVE_SOURCE_ENCODING') from None
    start = selector['byte_start']
    require(type(start) is int and 0 <= start < len(raw) and raw[start] & 0xc0 != 0x80, 'E_NATIVE_SELECTOR')


def _cell(attributes, kind, normalization):
    namespace, identifier, purpose = (attributes[name] for name in ('namespace', 'identifier', 'purpose'))
    if namespace != 'named' or identifier not in normalization or purpose != 'reference':
        return None, 'unrecognized_value'
    if kind == 'algorithm_family_reference':
        return dict(namespace=namespace, family=normalization[identifier], purpose=purpose), 'projected'
    cell = dict(namespace=namespace, identifier=identifier, purpose=purpose)
    if kind == 'explicit_rsa_key_bits':
        if 'key_bits' not in attributes['parameters']:
            return None, 'no_linked_property'
        value = attributes['parameters']['key_bits']
        if not isinstance(value, str) or not re.fullmatch('[1-9][0-9]{0,4}', value) or int(value) > 32768:
            return None, 'unrecognized_value'
        cell.update(parameter='key_bits', value=value)
    return cell, 'projected'


def capture_native_properties(capture_manifest, normalization, admitted_native, schema=None):
    """Pure capture. Inputs contain no oracle checks or expected field values."""
    if schema is None:
        schema = boundary.decode(boundary.checked_file(pathlib.Path(__file__).absolute().parents[1] / 'schemas/native-scan-v1.schema.json', collect=True))
    require(normalization.get('namespace') == 'named' and normalization.get('identifierToFamily') == FAMILIES,
            'E_NATIVE_NORMALIZATION')
    cases = capture_manifest['cases']
    require(isinstance(cases, list) and len({row['id'] for row in cases}) == len(cases) and
            all(row.get('expectedValuesPresent') is False and 'checks' not in row and 'expected' not in row for row in cases),
            'E_NATIVE_MANIFEST')
    validated = {}
    for repository, admitted in admitted_native.items():
        require(admitted['repositoryId'] == repository, 'E_NATIVE_REPOSITORY')
        require(set(admitted['artifact']) == {'path', 'sha256', 'bytes'} and
                re.fullmatch('[0-9a-f]{64}', admitted['artifact']['sha256']), 'E_NATIVE_ARTIFACT')
        assets, observations = validate_native(admitted['native'], admitted['sourceManifest'], schema)
        validated[repository] = assets, observations
    captured = []
    for case in cases:
        identity, repository, kind = case['id'], case['repositoryId'], case['propertyKind']
        actual = dict(values=[], joinStatus='not_assessed', references=[], completeness='not_assessed',
                      pointRateEligible=False, sourceSha256=case['sourceBinding']['sha256'])
        envelope = dict(id=identity, actual=actual)
        captured.append(envelope)
        if kind is None:
            require(case['selector'] is None and case['join'] == 'not_assessed', 'E_NATIVE_MANIFEST')
            continue
        require(kind in KINDS and isinstance(case['selector'], dict) and
                set(case['selector']) == {'path', 'byte_start'}, 'E_NATIVE_SELECTOR')
        selector = case['selector']; path = _relative(selector['path'])
        admitted = admitted_native.get(repository)
        if admitted is None:
            actual.update(joinStatus='missing_artifact', completeness='unknown')
            continue
        actual['artifact'] = admitted['artifact']
        source_manifest = admitted['sourceManifest']
        require(path in source_manifest and source_manifest[path]['sha256'] == case['sourceBinding']['sha256'],
                'E_NATIVE_SOURCE_MANIFEST')
        _source_bytes(admitted['sourceFiles'][path], case['sourceBinding'], selector)
        assets, observations = validated[repository]
        exact = [row for row in observations.values() if row['target']['path'] == path and
                 row['target']['byte_start'] == selector['byte_start']]
        complete = admitted['inspection'] == 'Complete' and admitted['native']['native_completeness'] == 'complete_within_scope'
        header = admitted['native']['native_completeness']
        actual['inspectionState'] = {'Complete':'complete','Partial':'partial'}.get(admitted['inspection'], 'unknown')
        actual['nativeCompleteness'] = header if header in {'complete_within_scope','not_exhaustive','partial','unsupported','failed','unknown'} else 'unknown'
        actual['completeness'] = ('complete' if complete else 'partial' if admitted['inspection'] == 'Partial' or header == 'partial' else
                                  'not_exhaustive' if header == 'not_exhaustive' else 'unknown')
        if not exact:
            unjoinable = any(row['target']['path'] is None or row['target']['path'] == path and row['target']['byte_start'] is None for row in observations.values())
            actual.update(joinStatus='unjoinable_location' if unjoinable else 'no_exact_location',
                          pointRateEligible=complete and not unjoinable)
            continue
        values, references = {}, set()
        unknown = False
        capture_states = set()
        for observation in exact:
            capture_states.add(CAPTURE_STATES.get(observation['evidence']['capture_status'], 'unknown'))
            if observation['target']['byte_end'] is None:
                capture_states.add('partial')
            for asset_id in observation['asset_ids']:
                references.add((observation['id'], asset_id))
                asset = assets[asset_id]
                if asset['kind'] != 'algorithm':
                    continue
                cell, status = _cell(asset['attributes'], kind, normalization['identifierToFamily'])
                unknown |= status == 'unrecognized_value'
                if cell is not None:
                    values[json.dumps(cell, sort_keys=True)] = cell
        actual['values'] = [values[key] for key in sorted(values)]
        actual['references'] = [dict(observationId=observation, assetId=asset) for observation, asset in sorted(references)]
        actual['captureStates'] = sorted(capture_states)
        actual['joinStatus'] = ('unsupported_capture' if 'unsupported' in capture_states else
                                'failed_capture' if 'failed' in capture_states else
                                'partial_capture' if 'partial' in capture_states else
                                'unknown_capture' if 'unknown' in capture_states else
                                'unrecognized_value' if unknown else 'projected' if values else 'no_linked_property')
        actual['pointRateEligible'] = complete and capture_states == {'complete'} and not unknown
    return captured


def _module(name, binding):
    boundary.checked_file(binding['path'], binding)
    spec = importlib.util.spec_from_file_location(name, binding['path'])
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _review_gate(request):
    review = boundary.bound_json(request['sourceOnlyReview'])
    require(review.get('kind') == 'native-source-only-implementation-review-v1' and review.get('verdict') == 'PASS' and
            review.get('model') == 'codex:gpt-6.1-sol@high' and isinstance(review.get('reviewer'), str) and
            bool(review['reviewer'].strip()) and isinstance(review.get('author'), str) and bool(review['author'].strip()) and review['reviewer'] != review['author'] and review.get('targetOutputsOpened') is False and
            review.get('all129CasesReviewed') is True and review.get('all30SelectorsReviewed') is True and
            review.get('implementationCorrectness') == 'PASS' and review.get('testAdequacy') == 'PASS', 'E_NATIVE_REVIEW_GATE')
    expected = {key: request[key] for key in ('captureManifest', 'normalization', 'coordinateProof', 'nativeSchema',
                                             'historicalPolicy', 'assessmentHelper', 'verifierBindings', 'qualityDirectory', 'oracleBinding', 'judgmentBinding')}
    require(review.get('bindings') == expected, 'E_NATIVE_REVIEW_BINDINGS')
    module = pathlib.Path(__file__).absolute()
    own = boundary.checked_file(module)
    require(review.get('adapter') == own, 'E_NATIVE_REVIEW_ADAPTER')
    boundary.checked_file(request['judgmentBinding']['path'], request['judgmentBinding'])
    preflight = boundary.checked_file(pathlib.Path(boundary.__file__).absolute())
    require(review.get('artifactPreflight') == preflight, 'E_NATIVE_REVIEW_PREFLIGHT')
    return review


def capture_from_policy(request):
    """Gate all target access on the final independent source-only review."""
    _review_gate(request)
    manifest = boundary.bound_json(request['captureManifest'])
    normalization = boundary.bound_json(request['normalization'])
    proof = boundary.bound_json(request['coordinateProof'])
    schema = boundary.bound_json(request['nativeSchema'])
    require(manifest['normalizationBinding'] == request['normalization'] and len(manifest['cases']) == 129 and
            sum(case['propertyKind'] is not None for case in manifest['cases']) == 30, 'E_NATIVE_SCOPE')
    comparable = {case['id']: case for case in manifest['cases'] if case['propertyKind'] is not None}
    require(len(proof['rows']) == 30 and {row['factId'] for row in proof['rows']} == set(comparable) and
            all(row['sourceBinding'] == comparable[row['factId']]['sourceBinding'] and
                all(row.get(flag) is True for flag in ('strictUtf8', 'bomAbsent', 'wideRecognitionLaneAbsent',
                                                       'decodedUtf8EqualsOriginalBytes')) for row in proof['rows']),
            'E_NATIVE_COORDINATE_PROOF')
    assessment = _module('native_accuracy_assessment', request['assessmentHelper'])
    quality = pathlib.Path(request['qualityDirectory'])
    expected_paths = {str(quality / name) for name in ('oss.py', 'evaluate.py', 'external.py', 'taxonomy.py', 'run.py', 'task050/study.py')}
    require({ref['path'] for ref in request['verifierBindings']} == expected_paths, 'E_NATIVE_VERIFIER_BINDINGS')
    for ref in request['verifierBindings']:
        boundary.checked_file(ref['path'], ref)
    oss, refs = assessment.load_historical(quality)
    require(refs == request['verifierBindings'], 'E_NATIVE_VERIFIER_BINDINGS')
    policy = boundary.bound_json(request['historicalPolicy'])
    require(policy.get('kind') == 'historical-score-policy' and policy.get('version') == 1, 'E_NATIVE_POLICY')
    matrix = [(row['id'], row['population'], row['phase']) for row in policy['invocations']]
    require(matrix == [('primary-baseline', 'primary', 'baseline'), ('primary-after', 'primary', 'after'),
                       ('comparison-baseline', 'comparison', 'baseline'), ('comparison-after', 'comparison', 'after')],
            'E_NATIVE_POLICY_MATRIX')
    registry = boundary.bound_json(policy['registry'])
    entries = {row['id']: row for row in registry['repositories']}
    require(len(entries) == len(registry['repositories']), 'E_NATIVE_REGISTRY')
    snapshots = []
    for record in policy['invocations']:
        preflight = boundary.preflight_invocation(record['binding'])
        actual = preflight['invocation']
        expected_ids = {identity for identity, entry in entries.items() if entry['population'] == record['population']}
        require(actual['phase'] == record['phase'] and actual['expectedSamples'] == 1 and
                set(actual['selectedIds']) == expected_ids and len(expected_ids) == (90 if record['population'] == 'primary' else 8),
                'E_NATIVE_POLICY_MATRIX')
        require(all(actual['selectedEntries'][identity] == entries[identity] for identity in expected_ids), 'E_NATIVE_REGISTRY')
        snapshots.append((record, preflight))
    admitted = {}
    for record, preflight in snapshots:
        actual, rows = oss._verified_invocation(preflight['path'])
        require(actual == preflight['invocation'], 'E_NATIVE_VERIFIER')
        if record['phase'] != 'after':
            continue
        for identity in actual['selectedIds']:
            row, terminal, bindings = rows[(identity, 0)]
            require(terminal is not None and bindings is not None and identity not in admitted, 'E_NATIVE_ARTIFACT')
            source_manifest_ref = bindings['input']['manifest']
            source_raw = boundary.checked_file(source_manifest_ref['path'], source_manifest_ref, collect=True)
            source_rows = json.loads(source_raw, object_pairs_hook=boundary._unique)
            source_manifest = {item['path']: item for item in source_rows}
            require(len(source_manifest) == len(source_rows), 'E_NATIVE_SOURCE_MANIFEST')
            relevant = [case for case in comparable.values() if case['repositoryId'] == identity]
            source_files = {}
            for case in relevant:
                path = case['selector']['path']
                source_files[path] = boundary.checked_file(case['sourceBinding']['path'], case['sourceBinding'],
                                                           limit=2 * 1024 * 1024, collect=True)
            artifact = dict(path=str(pathlib.Path(row['receipt']['path']).parent / 'output/scan.inventory.v1.json'),
                            **terminal['artifacts']['scan.inventory.v1.json'])
            native = boundary.bound_json(artifact)
            require(native['coverage'] == terminal['assessment']['coverage'], 'E_NATIVE_COVERAGE_BINDING')
            admitted[identity] = dict(repositoryId=identity, artifact=artifact, native=native,
                                      sourceManifest=source_manifest, sourceFiles=source_files,
                                      inspection=terminal['assessment']['inspection'])
    observations = capture_native_properties(manifest, normalization, admitted, schema)
    return dict(kind='native-positive-property-capture-v1', observations=observations,
                pointRatesPermitted=all(row['actual']['pointRateEligible'] for row in observations
                                        if row['actual']['joinStatus'] != 'not_assessed'),
                interpretation='Known-positive property representation only; contains permits extra false attributes.',
                bindings={key: request[key] for key in ('sourceOnlyReview', 'captureManifest', 'normalization',
                                                       'coordinateProof', 'historicalPolicy', 'oracleBinding', 'judgmentBinding')})


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--request', required=True)
    parser.add_argument('--output', required=True)
    args = parser.parse_args(argv)
    try:
        request = boundary.decode(boundary.checked_file(args.request, collect=True))
        result = capture_from_policy(request)
        with pathlib.Path(args.output).open('x', encoding='utf-8') as stream:
            json.dump(result, stream, ensure_ascii=False, sort_keys=True, allow_nan=False)
            stream.write('\n')
    except (ValueError, OSError, KeyError, TypeError, IndexError, RecursionError):
        print('E_NATIVE_CAPTURE_REJECTED', file=sys.stderr)
        return 2
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
