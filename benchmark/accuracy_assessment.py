"""Admit sealed source/claim evidence and reuse the existing canonical scorers.

This tool validates bindings and structure. It does not discover facts, infer
labels from predictions, approve semantics, execute scanned code, or measure
performance. Original rater bytes and complete native finding objects survive.
"""
import argparse
import hashlib
import importlib
import importlib.util
import json
import math
import os
import pathlib
import re
import sys
from collections import Counter

MAX_BYTES = 64 * 1024 * 1024
VERDICTS = {'supported': 'positive', 'refuted': 'negative', 'unknown': 'ambiguous'}
FACT_KINDS = {'positive_fact', 'negative_proposition', 'unknown'}
SOURCE_QUESTIONS = {'algorithm_reference', 'operation_role', 'explicit_parameter', 'artifact_kind',
                    'dependency_provenance', 'protocol_configuration', 'public_certificate_property'}
MAPPING_STATES = {'mapped', 'no_rule', 'unsupported_input', 'canonical_unrepresentable', 'unknown'}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def text(value):
    return isinstance(value, str) and bool(value.strip())


def canonical(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(',', ':'), allow_nan=False)


def digest(value):
    return hashlib.sha256(canonical(value).encode('utf-8')).hexdigest()


def unique_object(pairs):
    value = {}
    for key, item in pairs:
        require(key not in value, 'Duplicate JSON key')
        value[key] = item
    return value


def safe_path(path):
    path = pathlib.Path(path)
    require(path.is_absolute() and path.resolve() == path and path.is_file(), 'Unavailable or unsafe input path')
    return path


def read_bytes(path):
    with safe_path(path).open('rb') as stream:
        data = stream.read(MAX_BYTES + 1)
    require(len(data) <= MAX_BYTES, 'Input exceeds size limit')
    return data


def reference(path):
    path = safe_path(path)
    return {'path': str(path), 'sha256': hashlib.sha256(read_bytes(path)).hexdigest()}


def bound(binding):
    require(isinstance(binding, dict) and set(binding) == {'path', 'sha256'}, 'Invalid binding schema')
    require(isinstance(binding['sha256'], str) and re.fullmatch('[0-9a-f]{64}', binding['sha256']), 'Invalid binding digest')
    raw = read_bytes(binding['path'])
    require(hashlib.sha256(raw).hexdigest() == binding['sha256'], 'Input binding drift')
    return raw


def decode(raw):
    value = json.loads(raw, object_pairs_hook=unique_object,
                       parse_constant=lambda value: (_ for _ in ()).throw(ValueError('Nonfinite JSON')))
    require(isinstance(value, dict), 'Expected JSON object')
    return value


def load(path):
    return decode(read_bytes(path))


def index(rows, key='id'):
    require(isinstance(rows, list), 'Expected record list')
    result = {}
    for row in rows:
        require(isinstance(row, dict) and text(row.get(key)) and row[key] not in result, 'Invalid or duplicate identity')
        result[row[key]] = row
    return result


def relative(name):
    require(text(name) and '\\' not in name and ':' not in name, 'Invalid relative path')
    path = pathlib.PurePosixPath(name)
    require(not path.is_absolute() and all(x not in {'', '.', '..'} for x in name.split('/')), 'Unsafe relative path')
    return pathlib.Path(*path.parts)


def evidence(row, factual=True):
    require(text(row.get('reason')), 'Evidence requires reason')
    sources, keys = row.get('sourceEvidence'), row.get('primaryReferenceKeys')
    require(isinstance(sources, list) and isinstance(keys, list), 'Missing evidence list')
    if factual:
        require(bool(sources) and bool(keys), 'Missing factual evidence')
    require(all(text(key) for key in keys), 'Primary reference keys must be explicit strings')
    for source in sources:
        require(isinstance(source, dict) and {'path', 'sha256', 'line', 'description'} <= source.keys() and
                type(source['line']) is int and source['line'] >= 0 and text(source['description']), 'Invalid pinned source evidence')
        bound({key: source[key] for key in ('path', 'sha256')})


def context(row):
    bindings = row.get('contextBindings')
    require(isinstance(bindings, list), 'Missing consulted-context bindings')
    for binding in bindings:
        bound(binding)


def load_canonical(quality_dir):
    quality_dir = pathlib.Path(quality_dir)
    files = [safe_path(quality_dir / name) for name in ('evaluate.py', 'external.py')]
    modules = []
    old_evaluate = sys.modules.get('evaluate')
    try:
        for name, path in zip(('evaluate', '_accuracy_external'), files):
            spec = importlib.util.spec_from_file_location(name, path)
            module = importlib.util.module_from_spec(spec)
            sys.modules[name] = module
            spec.loader.exec_module(module)
            modules.append(module)
    finally:
        if old_evaluate is None:
            sys.modules.pop('evaluate', None)
        else:
            sys.modules['evaluate'] = old_evaluate
        sys.modules.pop('_accuracy_external', None)
    return modules[0], modules[1], [reference(path) for path in files]


def hypergeometric_interval(population, sample, successes):
    """Exact equal-tail 95% inversion using integer tail comparisons to 1/40.

    Inclusion uses >= at both endpoints. K is restricted to feasible population
    successes. Integer combinatorics avoids tail cancellation and lgamma error.
    """
    require(all(type(x) is int for x in (population, sample, successes)) and
            0 < sample <= population and 0 <= successes <= sample, 'Invalid finite population counts')
    denominator = math.comb(population, sample)
    lo, hi = successes, population - sample + successes

    def included(k, upper_tail):
        first = max(0, sample - (population - k))
        last = min(sample, k)
        if upper_tail:
            first = max(first, successes)
        else:
            last = min(last, successes)
        numerator = sum(math.comb(k, i) * math.comb(population - k, sample - i)
                        for i in range(first, last + 1))
        return numerator * 40 >= denominator

    left, right = lo, hi
    while left < right:
        middle = (left + right) // 2
        if included(middle, True):
            right = middle
        else:
            left = middle + 1
    lower = left
    left, right = lo, hi
    while left < right:
        middle = (left + right + 1) // 2
        if included(middle, False):
            left = middle
        else:
            right = middle - 1
    return {'method': 'exact-hypergeometric-equal-tail-inversion', 'confidence': 0.95,
            'populationSuccessBounds': [lower, left], 'bounds': [lower / population, left / population],
            'tailInclusion': 'integer tail numerator * 40 >= comb(N,n)',
            'limitation': 'SRS sampling uncertainty only; excludes annotation and model error'}


def claim_statistics(verdicts, population=None, expected_sample=None, srs=False):
    counts = Counter(verdicts)
    require(set(counts) <= set(VERDICTS), 'Invalid claim verdict')
    n = sum(counts.values())
    require(n > 0 and (expected_sample is None or n == expected_sample), 'Incomplete claim sample')
    t, f, u = (counts[value] for value in ('supported', 'refuted', 'unknown'))
    result = {'n': n, 'T': t, 'F': f, 'U': u, 'identificationBounds': [t / n, (t + u) / n],
              'resolvedConditionalPrecision': t / (t + f) if t + f else None,
              'pointPrecision': t / n if u == 0 else None, 'confidenceInterval': None,
              'populationCount': population, 'samplingQualified': False}
    if srs:
        require(type(population) is int and population >= n, 'Invalid SRS population')
        if u == 0:
            result['confidenceInterval'] = hypergeometric_interval(population, n, t)
            result['samplingQualified'] = True
    if u:
        result['confidenceIntervalUnavailableReason'] = 'Unknown labels; identification bounds are not a confidence interval'
    elif not srs:
        result['confidenceIntervalUnavailableReason'] = 'Not the frozen primary SRS population'
    return result


def header(record, packet, protocol_sha, adjudication=False):
    require(type(record.get('version')) is int and record['version'] == 1, 'Unsupported assessment version')
    for field in ('studyId', 'kind'):
        require(record.get(field) == packet[field], 'Assessment scope mismatch')
    require(record.get('protocolSha256') == protocol_sha and record.get('packetSha256') == packet['_sha256'], 'Assessment binding mismatch')
    if packet['kind'] == 'claim' and '_displayView' in packet:
        require(record.get('displayViewBinding') == packet['_displayView'], 'Assessment display view binding mismatch')
    require(text(record.get('reviewer')) and text(record.get('model')), 'Missing reviewer identity/model')
    require(record.get('predictionsSeen') is (packet['kind'] == 'claim') and
            record.get('otherLabelsSeen') is adjudication, 'Exposure mismatch')
    assessments = index(record.get('assessments'))
    require(assessments.keys() == index(packet['records']).keys(), 'Incomplete or extra assessment coverage')
    return assessments


def validate_claim(row, original):
    require(row.get('verdict') in VERDICTS, 'Invalid claim verdict')
    if original.get('nativeFindingRedacted', False):
        require(row['verdict'] != 'supported', 'Redacted native record cannot be admitted fully supported')
    context(row)
    assertions = index(row.get('assertions'))
    expected = index(original['assertionChecklist'])
    require(assertions.keys() == expected.keys() and bool(assertions), 'Incomplete factual assertion checklist')
    verdicts = []
    for assertion in assertions.values():
        require(assertion.get('verdict') in VERDICTS and text(assertion.get('claim')), 'Invalid factual assertion')
        evidence(assertion, assertion['verdict'] != 'unknown')
        verdicts.append(assertion['verdict'])
    combined = 'refuted' if 'refuted' in verdicts else 'unknown' if 'unknown' in verdicts else 'supported'
    require(row['verdict'] == combined, 'Whole-claim verdict does not preserve assertion strength')
    if row['verdict'] != 'unknown':
        require(bool(row['contextBindings']), 'Resolved claim requires consulted evidence binding')


def validate_source(row, original):
    require(type(row.get('complete')) is bool, 'Missing source completeness')
    context(row)
    facts = index(row.get('facts'), 'factId')
    if not facts:
        require(text(row.get('emptyReason')), 'Empty source census requires explicit reason')
    semantic_keys = set()
    for fact in facts.values():
        require(fact.get('kind') in FACT_KINDS and fact.get('question') in SOURCE_QUESTIONS and text(fact.get('occurrenceId')) and
                type(fact.get('inScope')) is bool, 'Invalid semantic fact')
        for field in ('algorithm', 'role', 'use'):
            require(field in fact and (fact[field] is None or text(fact[field])), 'Invalid semantic question field')
        require('parameters' in fact and (fact['parameters'] is None or isinstance(fact['parameters'], dict)), 'Invalid explicit parameters')
        anchor = fact.get('anchor', {})
        require(all(type(anchor.get(k)) is int for k in ('line', 'startColumn', 'endColumn')) and
                anchor['line'] == original['ownedLine'] and 0 < anchor['startColumn'] < anchor['endColumn'], 'Fact not owned by sampled line')
        if original.get('sourceStatus') == 'ready':
            source_path = pathlib.Path(original['sourceRoot']) / relative(original['sourcePath'])
            lines = bound({'path': str(source_path), 'sha256': original['sourceSha256']}).decode('utf-8').splitlines()
            require(anchor['endColumn'] <= len(lines[anchor['line'] - 1]) + 1, 'Semantic fact anchor outside source')
        semantic = canonical({key: fact[key] for key in ('occurrenceId', 'question', 'kind', 'inScope', 'algorithm', 'role', 'parameters', 'use', 'anchor')})
        require(semantic not in semantic_keys, 'Duplicate semantic proposition')
        semantic_keys.add(semantic)
        evidence(fact, fact['kind'] != 'unknown')
    if facts and any(f['kind'] != 'unknown' for f in facts.values()):
        require(bool(row['contextBindings']), 'Factual census requires consulted source binding')
    if original.get('sourceStatus') != 'ready':
        require(not row['complete'] and all(f['kind'] == 'unknown' for f in facts.values()), 'Unavailable source cannot become complete truth')


def verify_scope(value):
    require(isinstance(value, dict) and bool(value), 'Missing frozen scope binding')
    def visit(item):
        if isinstance(item, dict):
            if {'path', 'sha256'} <= item.keys():
                bound({key: item[key] for key in ('path', 'sha256')})
            for nested in item.values():
                visit(nested)
        elif isinstance(item, list):
            for nested in item:
                visit(nested)
    visit(value)


def verify_packet(packet, kind, protocol_sha):
    require(packet.get('version') == 1 and packet.get('kind') == kind and
            packet.get('protocolSha256') == protocol_sha, 'Packet protocol/kind mismatch')
    verify_scope(packet['scopeBinding'])
    provenance = packet.get('provenance', {})
    require(provenance.get('predictionsSeen') is (kind == 'claim') and provenance.get('otherLabelsSeen') is False, 'Packet exposure mismatch')
    rows = index(packet['records'])
    reports = {}
    require(bool(rows), 'Empty packet')
    for row in rows.values():
        relative(row['sourcePath'] if kind == 'source' else row['nativeFinding']['location']['file'])
        require(text(row.get('repositoryId')) and row.get('population') in {'primary', 'comparison'}, 'Invalid packet population')
        if kind == 'source' and row.get('sourceStatus') == 'ready':
            root = pathlib.Path(row['sourceRoot'])
            require(root.is_absolute() and root.resolve() == root, 'Unsafe source root')
            raw = bound({'path': str(root / relative(row['sourcePath'])), 'sha256': row['sourceSha256']})
            lines = raw.decode('utf-8').splitlines()
            require(type(row.get('ownedLine')) is int and 0 < row['ownedLine'] <= len(lines), 'Owned line unavailable')
        elif kind == 'claim':
            report_key = (row['reportPath'], row['reportSha256'])
            if report_key not in reports:
                reports[report_key] = decode(bound({'path': row['reportPath'], 'sha256': row['reportSha256']}))
            report = reports[report_key]
            ordinal = row['nativeOrdinal']
            require(type(ordinal) is int and 0 <= ordinal < len(report['findings']), 'Native ordinal mismatch')
            original = report['findings'][ordinal]
            require(digest(original) == row['findingSha256'], 'Native finding binding mismatch')
            if not row.get('nativeFindingRedacted', False):
                require(row['nativeFinding'] == original, 'Native finding fields changed')
    return rows


def display_ancestry(packet, packet_ref):
    """Validate immutable old identity and actual display masking provenance."""
    view = packet.get('displayViewBinding')
    if view is None:
        return packet_ref
    require(isinstance(view,dict) and view.get('version') == 1, 'Invalid display view binding')
    original_ref = view.get('originalPacket')
    original = decode(bound(original_ref))
    require('displayViewBinding' not in original, 'Display ancestry must bind an original packet')
    require({k:v for k,v in packet.items() if k not in {'records','displayViewBinding'}} ==
            {k:v for k,v in original.items() if k != 'records'}, 'Display packet identity changed')
    require(len(packet['records']) == len(original['records']), 'Display record population changed')
    mutable = {'nativeFinding','nativeFindingRedacted','assertionChecklist','claimSummary','originalPreservation',
               'contextSafeText','contextStatus','redacted','redactionPreventsVerification','displayPrivacy'}
    if packet['kind'] == 'source':
        mutable = {'contextSafeText','contextStatus','redacted','redactionPreventsVerification','displayPrivacy'}
    for row,old in zip(packet['records'],original['records']):
        require({k:v for k,v in row.items() if k not in mutable} ==
                {k:v for k,v in old.items() if k not in mutable}, 'Display record identity changed')
        privacy = row.get('displayPrivacy',{})
        native_changed = row.get('nativeFinding') != old.get('nativeFinding')
        context_changed = row.get('contextSafeText') != old.get('contextSafeText')
        require(privacy.get('addedNativeMask') is native_changed and
                privacy.get('addedContextMask') is context_changed, 'Display masking provenance mismatch')
        if packet['kind'] == 'claim':
            expected = bool(old.get('nativeFindingRedacted') or native_changed)
            require(row.get('nativeFindingRedacted') is expected, 'Display native redaction flag mismatch')
            for key in ('finding_id','rule_id','category','risk','priority','severity','confidence','location'):
                require(row['nativeFinding'].get(key) == old['nativeFinding'].get(key), 'Display native identity changed')
            old_checklist = [(x['id'],x.get('fieldPath')) for x in old['assertionChecklist']]
            require([(x['id'],x.get('fieldPath')) for x in row['assertionChecklist']] == old_checklist,
                    'Display assertion identity changed')
            for entry,before in zip(row['assertionChecklist'],old['assertionChecklist']):
                require({k:v for k,v in entry.items() if k != 'assertedValue'} ==
                        {k:v for k,v in before.items() if k != 'assertedValue'}, 'Display assertion requirement changed')
                if 'assertedValue' in entry:
                    field = entry['fieldPath'].removeprefix('/')
                    require(field in row['nativeFinding'] and entry['assertedValue'] == row['nativeFinding'][field],
                            'Display assertion payload mismatch')
            if 'claimSummary' in row:
                require(row['claimSummary'] == {key:row['nativeFinding'][key] for key in
                        ('rule_id','category','description','location','evidence') if key in row['nativeFinding']},
                        'Display summary payload mismatch')
    packet['_displayView'] = packet_ref
    return original_ref


def admit_inputs(protocol_path, packet_path, rater_paths, adjudication_path, kind):
    protocol_ref, packet_ref = reference(protocol_path), reference(packet_path)
    protocol, packet = load(protocol_path), load(packet_path)
    original_ref = display_ancestry(packet, packet_ref)
    packet['_sha256'] = original_ref['sha256']
    require(packet['studyId'] == protocol['studyId'], 'Protocol study mismatch')
    rows = verify_packet(packet, kind, protocol_ref['sha256'])
    if kind == 'claim' and 'sample' in protocol.get('primaryClaims', {}):
        selection = decode(bound(protocol['primaryClaims']['sample']))
        expected = index(selection['selected'], 'sampleId')
        primary = {r['sampleId']: r for r in rows.values() if r['population'] == 'primary'}
        if primary:
            require(primary.keys() == expected.keys(), 'Primary sample identity changed')
            for identity, row in primary.items():
                require(all(row.get(field) == expected[identity].get(field) for field in
                            ('repositoryId', 'nativeOrdinal', 'findingSha256', 'reportPath', 'reportSha256')), 'Primary sample record changed')
            require(packet['scopeBinding'].get('claimSelectionSha256') == protocol['primaryClaims']['sample']['sha256'] and
                    packet['scopeBinding'].get('populationCount') == protocol['primaryClaims']['populationCount'], 'Historical primary SRS binding mismatch')
    require(len(rater_paths) == 2, 'Exactly two original raters required')
    raters = [load(path) for path in rater_paths]
    refs = [reference(path) for path in rater_paths]
    require(len({r['reviewer'] for r in raters}) == 2 and len({r['sha256'] for r in refs}) == 2, 'Independent raters required')
    adjudication = load(adjudication_path)
    require(adjudication.get('raterBindings') == refs, 'Adjudication does not bind original raters in order')
    require(adjudication.get('reviewer') not in {r['reviewer'] for r in raters}, 'Adjudicator must be separate')
    for record, adjudicated in [(r, False) for r in raters] + [(adjudication, True)]:
        assessments = header(record, packet, protocol_ref['sha256'], adjudicated)
        for identity, row in assessments.items():
            (validate_claim if kind == 'claim' else validate_source)(row, rows[identity])
        if kind == 'source':
            facts = [fact for row in assessments.values() for fact in row['facts']]
            index(facts, 'factId')
    bindings = {'protocol': protocol_ref, 'packet': original_ref, 'raters': refs,
                'adjudication': reference(adjudication_path)}
    if '_displayView' in packet:
        bindings['displayView'] = packet_ref
    return protocol, packet, raters, adjudication, bindings


def write_json(path, value):
    with os.fdopen(os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), 'w', encoding='utf-8') as stream:
        stream.write(json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + '\n')


def output_dir(path):
    path = pathlib.Path(path)
    require(path.is_absolute() and path.parent.resolve() == path.parent, 'Unsafe output directory')
    path.mkdir(mode=0o700, exist_ok=False)
    return path


def preserve_inputs(out, bindings):
    def preserve(path, binding):
        with os.fdopen(os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), 'wb') as stream:
            stream.write(bound(binding))
    for name in ('protocol', 'packet', 'adjudication'):
        if name == 'packet' and 'displayView' in bindings:
            private = out / 'private'
            private.mkdir(mode=0o700)
            preserve(private / 'original-packet.json', bindings['packet'])
            preserve(out / 'packet.json', bindings['displayView'])
        else:
            preserve(out / (name + '.json'), bindings[name])
    for i, binding in enumerate(bindings['raters'], 1):
        preserve(out / f'rater-{i}.json', binding)


def atomic_claim_score(native, verdict, evaluate):
    loc = native['location']
    relative(loc['file'])
    require(type(loc['line']) is int and loc['line'] >= 0 and
            type(loc['column']) is int and loc['column'] >= 0, 'Invalid native claim location')
    unit = {'id': 'native-claim', 'path': loc['file'], 'ruleId': native['rule_id'],
            'line': loc['line'], 'startColumn': loc['column'], 'endColumn': loc['column'] + 1}
    if loc['line'] == 0:
        dep = native.get('evidence', {}).get('metadata', {}).get('dep_name')
        require(text(dep), 'Line-zero native claim lacks canonical dependency identity')
        unit['dependencyName'] = dep
    data = {'families': {'native': [native['rule_id']]}, 'files': [{'path': loc['file'], 'split': 'holdout'}], 'units': [unit]}
    return evaluate.score(data, {'native-claim': VERDICTS[verdict]}, [native])


def admit_claims(protocol_path, packet_path, rater_paths, adjudication_path, quality_dir, out_path):
    protocol, packet, raters, adjudication, bindings = admit_inputs(protocol_path, packet_path, rater_paths, adjudication_path, 'claim')
    evaluate, _, canonical_refs = load_canonical(quality_dir)
    records = index(packet['records'])
    results, dispositions = [], {'primary': [], 'comparison': []}
    for assessment in adjudication['assessments']:
        row = records[assessment['id']]
        verdict = assessment['verdict']
        native = row['nativeFinding']
        if row.get('nativeFindingRedacted', False):
            require(verdict != 'supported', 'Redacted native record cannot be admitted fully supported')
            native = decode(bound({'path': row['reportPath'], 'sha256': row['reportSha256']}))['findings'][row['nativeOrdinal']]
            score = atomic_claim_score(native, verdict, evaluate) if native['location']['line'] > 0 or text(native.get('evidence', {}).get('metadata', {}).get('dep_name')) else None
        elif native['location']['line'] == 0 and not text(native.get('evidence', {}).get('metadata', {}).get('dep_name')):
            require(verdict != 'supported', 'Canonical-unrepresentable native claim cannot be fully supported')
            score = None
        else:
            score = atomic_claim_score(native, verdict, evaluate)
        dispositions[row['population']].append(verdict)
        results.append({'id': row['id'], 'population': row['population'], 'findingSha256': row['findingSha256'],
                        'verdict': verdict, 'canonicalScore': score})
    primary = protocol['primaryClaims']
    stats = {}
    if dispositions['primary']:
        stats['primary'] = claim_statistics(dispositions['primary'], primary['populationCount'], primary['sampleCount'], True)
    if dispositions['comparison']:
        stats['comparison'] = claim_statistics(dispositions['comparison'], expected_sample=protocol.get('comparisonClaims', {}).get('sampleCount', 24))
    duplicates = Counter((r['repositoryId'], r['findingSha256']) for r in records.values())
    receipt = {'version': 1, 'kind': 'native-claim-assessment', 'studyId': protocol['studyId'],
               'bindings': bindings, 'canonicalBindings': canonical_refs, 'statistics': stats, 'rows': results,
               'duplicateEmissions': {'extraSampleRecords': sum(n - 1 for n in duplicates.values()),
                                      'scope': 'Sampled exact full-record duplicates only; separate from atomic factual correctness'},
               'exposure': {'sourceBlind': False, 'predictionsSeen': True,
                            'raters': [{k: r[k] for k in ('reviewer', 'model', 'predictionsSeen', 'otherLabelsSeen')} for r in raters],
                            'limitation': 'Identity checks do not establish epistemic or model-family independence'},
               'scope': 'Frozen historical native claim occurrences; no source recall or runtime safety conclusion'}
    out = output_dir(out_path)
    preserve_inputs(out, bindings)
    write_json(out / 'assessment.json', receipt)
    return receipt


def rule_ids(bindings, trusted_rules):
    require(isinstance(bindings, list) and bool(bindings), 'Missing frozen existing rules')
    supplied = {}
    ids = []
    for binding in bindings:
        path = safe_path(binding['path'])
        name = pathlib.PurePosixPath(*path.parts[-2:]).as_posix()
        require(name in trusted_rules and name not in supplied and binding['sha256'] == trusted_rules[name], 'Untrusted rule file')
        supplied[name] = binding['sha256']
        raw = bound(binding).decode('utf-8')
        ids.extend(re.findall(r'^- id:\s*([A-Za-z0-9_-]+)\s*$', raw, flags=re.MULTILINE))
    require(supplied == trusted_rules, 'Missing trusted rule files')
    require(len(ids) == 139 and len(ids) == len(set(ids)), 'Invalid existing rule registry')
    return set(ids)


def validate_projection(projection, check, packet, adjudication, raters, bindings, trusted_rules):
    for record, kind in ((projection, 'source-projection'), (check, 'source-projection-check')):
        require(record.get('version') == 1 and record.get('kind') == kind and record.get('studyId') == packet['studyId'] and
                record.get('protocolSha256') == bindings['protocol']['sha256'] and
                record.get('packetSha256') == bindings['packet']['sha256'], 'Projection scope mismatch')
        require(text(record.get('reviewer')) and text(record.get('model')) and
                record.get('predictionsSeen') is False and record.get('otherLabelsSeen') is True, 'Projection exposure/identity mismatch')
        require(record.get('adjudicationBinding') == bindings['adjudication'], 'Projection adjudication binding mismatch')
        context(record)
        require(bool(record['contextBindings']), 'Mapping requires consulted source/rule contracts')
    identities = {r['reviewer'] for r in raters} | {adjudication['reviewer']}
    require(projection['reviewer'] not in identities and check['reviewer'] not in identities | {projection['reviewer']}, 'Mapping/checker independence missing')
    require(check.get('projectionBinding') == bindings['projection'] and check.get('accepted') is True and text(check.get('reason')), 'Independent mapping check unavailable')
    existing = rule_ids(projection['ruleBindings'], trusted_rules)
    families = projection.get('families')
    require(isinstance(families, dict) and bool(families), 'Missing rule families')
    nominated = [rule for rules in families.values() for rule in rules]
    require(all(isinstance(rules, list) and bool(rules) for rules in families.values()) and
            len(nominated) == len(set(nominated)) and set(nominated) <= existing, 'Nonexistent or repeated canonical rule')
    records, units = index(packet['records']), index(projection.get('units'))
    for unit in units.values():
        require(unit.get('recordId') in records and unit.get('ruleId') in nominated and unit.get('label') in set(VERDICTS.values()) | {'unsupported'}, 'Invalid projected unit')
        source = records[unit['recordId']]
        require(unit.get('path') == source['sourcePath'] and source.get('sourceStatus') == 'ready', 'Projected unit source mismatch')
        require(all(type(unit.get(k)) is int for k in ('line', 'startColumn', 'endColumn')) and
                unit['line'] > 0 and 0 < unit['startColumn'] < unit['endColumn'], 'Invalid canonical coordinates')
        if 'dependencyName' in unit:
            require(text(unit['dependencyName']), 'Invalid canonical dependency identity')
        lines = bound({'path': str(pathlib.Path(source['sourceRoot']) / relative(source['sourcePath'])), 'sha256': source['sourceSha256']}).decode('utf-8').splitlines()
        require(unit['line'] <= len(lines) and unit['endColumn'] <= len(lines[unit['line'] - 1]) + 1, 'Projected interval outside source')
    facts = {f['factId']: f for a in adjudication['assessments'] for f in a['facts']}
    links = index(projection.get('facts'), 'factId')
    require(links.keys() == facts.keys(), 'Incomplete fact mapping')
    used = set()
    ownership = {f['factId']: a['id'] for a in adjudication['assessments'] for f in a['facts']}
    for identity, link in links.items():
        mapping, fact = link['mapping'], facts[identity]
        require(mapping.get('status') in MAPPING_STATES and mapping.get('predicate') in {'any', 'all'} and
                isinstance(mapping.get('unitIds'), list) and len(mapping['unitIds']) == len(set(mapping['unitIds'])), 'Invalid fact mapping predicate')
        linked = mapping['unitIds']
        if mapping['status'] == 'mapped':
            require(bool(linked) and set(linked) <= units.keys() and
                    all(units[u]['recordId'] == ownership[identity] for u in linked), 'Invalid fact-to-unit relation')
            if fact['kind'] == 'positive_fact':
                require(all(units[u]['label'] == 'positive' for u in linked), 'Positive fact mapped to nonpositive unit')
        else:
            require(not linked, 'Unmapped fact carries canonical links')
        used.update(linked)
    require(used == units.keys(), 'Orphan canonical unit')
    projections = projection.get('raterProjections')
    require(isinstance(projections, list) and len(projections) == 2, 'Two explicit source-rater projections required')
    for rater, binding, projected in zip(raters, bindings['raters'], projections):
        require(projected.get('raterBinding') == binding and projected.get('sourceCensusSha256') == digest(rater['assessments']) and
                projected.get('projectionAuthor') == projection['reviewer'], 'Original source census projection binding mismatch')
        labels = index(projected.get('labels'))
        require(labels.keys() == units.keys() and all(r['label'] in {'positive', 'negative', 'ambiguous', 'unsupported'} for r in labels.values()), 'Invalid source-rater canonical projection')
    return units


def admit_source(protocol_path, packet_path, rater_paths, adjudication_path, projection_path, check_path, quality_dir, out_path, scoring_policy_path):
    protocol, packet, raters, adjudication, bindings = admit_inputs(protocol_path, packet_path, rater_paths, adjudication_path, 'source')
    projection, check = load(projection_path), load(check_path)
    bindings.update(projection=reference(projection_path), mappingCheck=reference(check_path))
    historical = admit_historical_inputs(scoring_policy_path, protocol, packet, bindings, quality_dir)
    bindings['scoringPolicy'] = reference(scoring_policy_path)
    units = validate_projection(projection, check, packet, adjudication, raters, bindings, historical['trustedRules'])
    evaluate, external, canonical_refs = load_canonical(quality_dir)
    records = index(packet['records'])
    components = []
    for repo in sorted({r['repositoryId'] for r in records.values()}):
        require(re.fullmatch('[A-Za-z0-9_.-]+', repo) and repo not in {'.', '..'}, 'Unsafe repository identity')
        repo_records = {i: r for i, r in records.items() if r['repositoryId'] == repo}
        require(len({r['population'] for r in repo_records.values()}) == 1, 'Repository crosses populations')
        selected_units = [u for u in units.values() if u['recordId'] in repo_records]
        paths = {}
        for unit in selected_units:
            record = records[unit['recordId']]
            previous = paths.setdefault(unit['path'], record)
            require(previous['sourceSha256'] == record['sourceSha256'], 'Source path collision')
        corpus = {'version': 1, 'license': 'external-source-evidence', 'families': projection['families'],
                  'files': [{'path': path, 'split': r['split'], 'source': str(pathlib.Path(r['sourceRoot']) / relative(path)),
                             'sha256': r['sourceSha256']} for path, r in sorted(paths.items())],
                  'units': [{k: v for k, v in unit.items() if k not in {'recordId', 'label'}} for unit in selected_units]}
        labels = [{'id': u['id'], 'label': u['label']} for u in selected_units]
        evaluate.validate_labels(corpus, evaluate.label_map(labels))
        if corpus['units']:
            external.score_annotated(corpus, evaluate.label_map(labels), [])
        components.append((repo, repo_records, corpus, labels))
    out = output_dir(out_path)
    preserve_inputs(out, bindings)
    for name in ('projection', 'mappingCheck'):
        (out / (name + '.json')).write_bytes(bound(bindings[name]))
    component_receipts = []
    for repo, repo_records, corpus, labels in components:
        directory = out / repo
        directory.mkdir()
        write_json(directory / 'corpus.json', corpus)
        corpus_sha = reference(directory / 'corpus.json')['sha256']
        label_sources = {}
        for i, (rater, projected) in enumerate(zip(raters, projection['raterProjections']), 1):
            projected_labels = [label for label in projected['labels'] if label['id'] in {u['id'] for u in corpus['units']}]
            name = f'canonical-rater-{i}.json'
            record = {'reviewer': rater['reviewer'], 'model': rater['model'], 'corpusSha256': corpus_sha,
                      'labels': projected_labels, 'originalRaterBinding': bindings['raters'][i - 1],
                      'sourceCensusSha256': digest(rater['assessments']), 'projectionAuthor': projection['reviewer'],
                      'sourceRaterAuthoredRuleLabels': False, 'predictionsSeen': False}
            write_json(directory / name, record)
            label_sources[name] = reference(directory / name)['sha256']
        census = [a for a in adjudication['assessments'] if a['id'] in repo_records]
        truth = {'version': 1, 'corpusSha256': corpus_sha, 'labelSources': label_sources, 'labels': labels,
                 'sourceCensus': census, 'sourceCensusAuthority': bindings['adjudication'],
                 'projectionBinding': bindings['projection'], 'mappingCheckBinding': bindings['mappingCheck'],
                 'factMappings': [f for f in projection['facts'] if f['factId'] in {f['factId'] for a in census for f in a['facts']}],
                 'population': next(iter(repo_records.values()))['population'],
                 'scope': 'Owned lines only; canonical rule labels are explicit mapper projections, not independent rater-authored rule labels'}
        write_json(directory / 'truth.json', truth)
        evaluate.load_truth(directory)
        component_receipts.append({'repositoryId': repo, 'directory': str(directory), 'corpus': reference(directory / 'corpus.json'),
                                   'truth': reference(directory / 'truth.json'), 'ownedLines': len(census), 'canonicalUnits': len(corpus['units'])})
    receipt = {'version': 1, 'kind': 'source-assessment-admission', 'studyId': protocol['studyId'], 'bindings': bindings,
               'canonicalBindings': canonical_refs, 'components': component_receipts, 'historicalInputs': historical,
               'exposure': {'predictionsSeen': False, 'sameModelFamilyPossible': True,
                            'limitation': 'Identity checks cannot establish semantic or epistemic independence'},
               'scope': 'No predictions attached; sourceCensus is authoritative; no whole-source recall'}
    write_json(out / 'admission.json', receipt)
    return receipt


def source_statistics(census, mappings, canonical_score):
    facts = [fact for assessment in census for fact in assessment['facts']]
    links = index(mappings, 'factId')
    require(set(links) == {f['factId'] for f in facts}, 'Source fact mapping drift')
    tps = {d['unit'] for d in canonical_score['accuracy']['details'] if d['disposition'] == 'tp'}
    positives = [f for f in facts if f['kind'] == 'positive_fact' and f['inScope']]
    detected, states, missed = [], Counter(), []
    for fact in positives:
        mapping = links[fact['factId']]['mapping']
        states[mapping['status']] += 1
        if mapping['status'] != 'mapped':
            continue
        hits = [u in tps for u in mapping['unitIds']]
        detected_now = all(hits) if mapping['predicate'] == 'all' else any(hits)
        (detected if detected_now else missed).append(fact['factId'])
    unknown = [f['factId'] for f in facts if f['kind'] == 'unknown']
    incomplete = [a['id'] for a in census if not a['complete']]
    s, d = len(positives), len(detected)
    unresolved = sum(count for state, count in states.items() if state != 'mapped')
    return {'S': s, 'D': d, 'canonicalProvenLowerFraction': d / s if s else None,
            'finiteKnownSourceCapabilityCoverage': d / s if s and not unresolved else None,
            'unresolvedPositiveMappings': unresolved,
            'coverageQualification': 'D is canonical-TP-proven only; its complement is not a miss',
            'detectedFactIds': detected, 'mappedMissFactIds': missed,
            'mappingStatusCounts': {key: states[key] for key in sorted(MAPPING_STATES)},
            'positiveOccurrenceCount': len({f['occurrenceId'] for f in positives}),
            'positivePropositionCount': s, 'scopeFactIds': [f['factId'] for f in facts],
            'allFactMappingStatusCounts': dict(sorted(Counter(links[f['factId']]['mapping']['status'] for f in facts).items())),
            'outsideMigrationScopeFactIds': [f['factId'] for f in facts if f['kind'] == 'positive_fact' and not f['inScope']],
            'unknownFactIds': unknown, 'incompleteOwnedLineIds': incomplete,
            'broadRecallUpperBound': None,
            'upperBoundUnavailableReason': 'No whole-source population recall claim; unknown or incomplete enumeration may have unbounded missing facts',
            'scope': 'Finite known source propositions beginning on sampled owned lines only'}


def public_score(score):
    """Keep canonical arithmetic while publishing only ordered native identities."""
    result = dict(score)
    for name in ('unattributed', 'other_rules'):
        result[name] = [{'findingSha256': digest(finding)} for finding in score[name]]
        result[name + 'Count'] = len(score[name])
    return result


def load_historical(quality_dir):
    """Load the existing invocation verifier and bind its local dependencies."""
    quality_dir = safe_path(pathlib.Path(quality_dir) / 'oss.py').parent
    names = ('evaluate', 'external', 'taxonomy', 'run', 'task050', 'task050.study', 'oss')
    previous = {name: sys.modules.get(name) for name in names}
    old_path = list(sys.path)
    try:
        for name in names:
            sys.modules.pop(name, None)
        sys.path.insert(0, str(quality_dir))
        module = importlib.import_module('oss')
        refs = [reference(quality_dir / name) for name in
                ('evaluate.py', 'external.py', 'taxonomy.py', 'run.py', 'task050/study.py', 'oss.py')]
        return module, refs
    finally:
        sys.path[:] = old_path
        for name, old in previous.items():
            if old is None:
                sys.modules.pop(name, None)
            else:
                sys.modules[name] = old


def source_identity(record, entry):
    require(record['repositoryId'] == entry['id'] and record['population'] == entry['population'] and
            record['commit'] == entry['commit'], 'Source repository snapshot mismatch')


def historical_snapshot(binding, population, phase, registry, oss):
    """Verify the preserved CLI wrapper against its canonical frozen receipt."""
    spec = importlib.util.spec_from_file_location('artifact_preflight', safe_path(pathlib.Path(__file__).with_name('artifact_preflight.py')))
    artifact_preflight = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(artifact_preflight)
    admitted = artifact_preflight.preflight_invocation(binding)
    wrapper = admitted['wrapper']
    actual_path = safe_path(wrapper['receiptPath'])
    actual = load(actual_path)
    require(wrapper == dict(actual, receiptPath=str(actual_path)), 'Invocation wrapper diverges from canonical receipt')
    started_path = actual_path.parent / 'started.json'
    started = load(started_path)
    require(actual['phase'] == phase and actual['expectedSamples'] == 1, 'Wrong historical phase or samples')
    expected = {identity for identity, entry in registry.items() if entry['population'] == population}
    require(set(actual['selectedIds']) == expected, 'Historical population mismatch')
    for identity, entry in actual['selectedEntries'].items():
        current = registry[identity]
        require(all(entry[key] == current[key] for key in ('id', 'population', 'commit', 'inputManifestSha256')),
                'Historical registry source identity mismatch')
    verified, results = oss._verified_invocation(actual_path)
    require(verified == actual, 'Canonical invocation verification mismatch')
    scanner = actual['scanner']
    bound(scanner['binary'])
    require(digest(scanner['rules']) == scanner['rulesSha256'], 'Historical rules digest mismatch')
    rows = {}
    for (identity, sample), (row, terminal, attempt) in results.items():
        require(sample == 0 and terminal is not None and attempt is not None, 'Historical report is unavailable')
        entry = actual['selectedEntries'][identity]
        manifest = {'path': entry['inputManifest'], 'sha256': entry['inputManifestSha256']}
        bound(manifest)
        require(attempt['input']['commit'] == entry['commit'] and
                attempt['input']['manifest']['sha256'] == entry['inputManifestSha256'], 'Historical started source identity mismatch')
        terminal_path = safe_path(row['receipt']['path'])
        artifact = terminal['artifacts']['report.json']
        report_path = safe_path(terminal_path.parent / 'output/report.json')
        require(report_path.stat().st_size == artifact['bytes'] and report_path.stat().st_size <= MAX_BYTES,
                'Historical report byte length mismatch')
        require(reference(report_path)['sha256'] == artifact['sha256'], 'Historical report binding drift')
        rows[identity] = {'repositoryId': identity, 'population': population, 'commit': entry['commit'],
                          'inputManifestSha256': entry['inputManifestSha256'], 'inputManifest': manifest,
                          'terminal': row['receipt'], 'started': reference(terminal_path.parent / 'started.json'),
                          'report': {'path': str(report_path), 'sha256': artifact['sha256']}, 'reportBytes': artifact['bytes'],
                          'configurationSha256': digest(attempt['configuration']), 'scannerSha256': digest(attempt['scanner'])}
    return {'binding': binding, 'receipt': reference(actual_path), 'started': reference(started_path),
            'population': population, 'phase': phase, 'scannerSha256': digest(scanner),
            'rulesSha256': scanner['rulesSha256'], 'configurationSha256': digest(actual['configuration']), 'rows': rows}


def admit_historical_inputs(policy_path, protocol, packet, bindings, quality_dir):
    """Admit source/projection metadata against explicitly frozen phase snapshots."""
    policy = load(policy_path)
    require(policy.get('version') == 1 and policy.get('kind') == 'historical-score-policy' and
            policy.get('studyId') == protocol['studyId'], 'Invalid historical scoring policy')
    require(policy['baseProtocol'] == bindings['protocol'] and policy['sourcePacket'] == bindings['packet'],
            'Historical policy scope mismatch')
    selection_ref = protocol['primaryClaims']['sample']
    selection = decode(bound(selection_ref))
    anchor = selection['invocation']
    require(policy['primaryAfterAnchor'] == anchor and policy['projectionRulesInvocationId'] == 'primary-after',
            'Untrusted historical after anchor')
    anchor_invocation = decode(bound(anchor))
    require(anchor_invocation['scanner']['binary'] == selection['scannerBinary'] and
            protocol['primaryClaims']['binarySha256'] == selection['scannerBinary']['sha256'], 'Trusted binary mismatch')
    rules = anchor_invocation['scanner']['rules']
    require(len(rules) == 13 and digest(rules) == anchor_invocation['scanner']['rulesSha256'], 'Invalid trusted rules registry')
    registry = index(decode(bound(policy['registry']))['repositories'])
    require(packet['scopeBinding'].get('registrySha256') == policy['registry']['sha256'], 'Source packet registry mismatch')
    for record in packet['records']:
        require(record['repositoryId'] in registry, 'Source repository outside historical registry')
        entry = registry[record['repositoryId']]
        source_identity(record, entry)
        manifest_raw = bound({'path': entry['inputManifest'], 'sha256': entry['inputManifestSha256']})
        manifest = index(json.loads(manifest_raw, object_pairs_hook=unique_object), 'path')
        require(record['sourcePath'] in manifest and manifest[record['sourcePath']]['sha256'] == record['sourceSha256'],
                'Source packet manifest mismatch')
    invocations = index(policy['invocations'])
    expected = {f'{population}-{phase}' for population in ('primary', 'comparison') for phase in ('baseline', 'after')}
    require(set(invocations) == expected and invocations['primary-after']['binding'] == anchor,
            'Incomplete historical phase policy')
    oss, refs = load_historical(quality_dir)
    spec = importlib.util.spec_from_file_location('artifact_preflight', safe_path(pathlib.Path(__file__).with_name('artifact_preflight.py')))
    artifact_preflight = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(artifact_preflight)
    for identity, row in invocations.items():
        require(identity == row['population'] + '-' + row['phase'], 'Historical invocation identity mismatch')
        admitted = artifact_preflight.preflight_invocation(row['binding'])
        actual = admitted['invocation']
        expected_ids = {key for key, entry in registry.items() if entry['population'] == row['population']}
        require(actual['phase'] == row['phase'] and actual['expectedSamples'] == 1 and
                set(actual['selectedIds']) == expected_ids, 'Historical phase or population mismatch')
        require(all(all(entry[key] == registry[repo][key] for key in
                        ('id', 'population', 'commit', 'inputManifestSha256'))
                    for repo, entry in actual['selectedEntries'].items()), 'Historical registry source identity mismatch')
    snapshots = {}
    for identity, row in invocations.items():
        snapshots[identity] = historical_snapshot(row['binding'], row['population'], row['phase'], registry, oss)
    return {'policy': reference(policy_path), 'verifierBindings': refs,
            'trustedRuleChain': {'sample': selection_ref, 'invocation': anchor,
                                 'scannerBinary': selection['scannerBinary'], 'rulesSha256': digest(rules)},
            'trustedRules': rules, 'invocations': snapshots}


def verify_prediction(admission, repository_id, population, predictions_path, invocation_id, quality_dir):
    """Recheck frozen provenance and the per-use original native report before matching."""
    historical = admission['historicalInputs']
    policy = decode(bound(historical['policy']))
    require(historical['policy'] == admission['bindings']['scoringPolicy'], 'Historical policy drift')
    _, refs = load_historical(quality_dir)
    require(refs == historical['verifierBindings'], 'Historical verifier changed')
    declared = index(policy['invocations'])
    require(invocation_id in declared and invocation_id in historical['invocations'], 'Undeclared historical invocation')
    snapshot = historical['invocations'][invocation_id]
    require(snapshot['binding'] == declared[invocation_id]['binding'] and snapshot['population'] == population,
            'Wrong historical prediction population')
    wrapper = decode(bound(snapshot['binding']))
    actual = decode(bound(snapshot['receipt']))
    invocation_started = decode(bound(snapshot['started']))
    require(wrapper == dict(actual, receiptPath=snapshot['receipt']['path']) and
            all(actual.get(key) == value for key, value in invocation_started.items()), 'Historical invocation request drift')
    require(actual['phase'] == snapshot['phase'] and digest(actual['scanner']) == snapshot['scannerSha256'] and
            digest(actual['configuration']) == snapshot['configurationSha256'] and
            actual['scanner']['rulesSha256'] == snapshot['rulesSha256'], 'Historical scanner/configuration mismatch')
    bound(actual['scanner']['binary'])
    require(repository_id in snapshot['rows'] and repository_id in actual['selectedEntries'], 'Wrong historical repository')
    row = snapshot['rows'][repository_id]
    entry = actual['selectedEntries'][repository_id]
    terminal = decode(bound(row['terminal']))
    started = decode(bound(row['started']))
    request = started['bindings']
    expected_row = [item for item in actual['rows'] if item['repositoryId'] == repository_id and item['sample'] == 0]
    require(len(expected_row) == 1 and expected_row[0]['receipt'] == row['terminal'] and
            expected_row[0]['execution'] == terminal['execution'] == 'Succeeded' and
            terminal['startedDigest'] == digest(started) and terminal['executionKey'] == digest(request),
            'Historical terminal/request mismatch')
    require(started['repositoryId'] == repository_id and request['registryEntry'] == entry and
            entry['population'] == population and entry['commit'] == row['commit'] and
            entry['inputManifestSha256'] == row['inputManifestSha256'] and
            request['input']['commit'] == row['commit'] and
            request['input']['manifest']['sha256'] == row['inputManifestSha256'] and
            request['phase'] == started['phase'] == snapshot['phase'] and
            request['configuration'] == actual['configuration'] and request['scanner'] == actual['scanner'],
            'Historical source/scanner identity mismatch')
    bound(row['inputManifest'])
    artifact = terminal['artifacts']['report.json']
    expected_path = str(safe_path(pathlib.Path(row['terminal']['path']).parent / 'output/report.json'))
    require(str(safe_path(predictions_path)) == row['report']['path'] == expected_path and
            artifact == {'bytes': row['reportBytes'], 'sha256': row['report']['sha256']}, 'Predictions are not the frozen report')
    raw = bound(row['report'])
    require(len(raw) == row['reportBytes'], 'Historical report byte length drift')
    return {'invocationId': invocation_id, 'policy': historical['policy'], 'invocation': snapshot['binding'],
            'receipt': snapshot['receipt'], 'invocationStarted': snapshot['started'], 'terminal': row['terminal'],
            'started': row['started'], 'report': row['report'], 'reportBytes': row['reportBytes'],
            'commit': row['commit'], 'inputManifestSha256': row['inputManifestSha256'],
            'scannerSha256': snapshot['scannerSha256'], 'rulesSha256': snapshot['rulesSha256'],
            'configurationSha256': snapshot['configurationSha256']}


def score_source(admission_path, repository_id, predictions_path, quality_dir, out_path, invocation_id):
    admission = load(admission_path)
    require(admission.get('kind') == 'source-assessment-admission', 'Not a source admission')
    for name, binding in admission['bindings'].items():
        for item in binding if name == 'raters' else [binding]:
            raw = bound(item)
            verify_scope(decode(raw))
    for binding in admission['canonicalBindings']:
        bound(binding)
    evaluate, external, refs = load_canonical(quality_dir)
    require(refs == admission['canonicalBindings'], 'Canonical scorer binding changed')
    components = index(admission['components'], 'repositoryId')
    require(repository_id in components, 'Unknown source component')
    component = components[repository_id]
    bound(component['corpus']); bound(component['truth'])
    directory = pathlib.Path(component['directory'])
    require(reference(directory / 'corpus.json') == component['corpus'] and reference(directory / 'truth.json') == component['truth'], 'Component location drift')
    data, labels = evaluate.load_truth(directory)
    for file in data['files']:
        bound({'path': file['source'], 'sha256': file['sha256']})
    truth = load(directory / 'truth.json')
    provenance = verify_prediction(admission, repository_id, truth['population'], predictions_path, invocation_id, quality_dir)
    predictions = decode(bound(provenance['report']))
    require(isinstance(predictions.get('findings'), list), 'Expected unchanged native findings')
    score = external.score_annotated(data, labels, predictions['findings']) if data['units'] else {
        'accuracy': evaluate.score(data, labels, []), 'unattributed': predictions['findings'], 'other_rules': [],
        'scope': 'No canonical units admitted; all predictions remain unattributed'}
    result = {'version': 1, 'kind': 'source-assessment-score', 'repositoryId': repository_id, 'population': truth['population'],
              'admissionBinding': reference(admission_path), 'predictionBinding': reference(predictions_path),
              'canonicalBindings': refs, 'canonicalScore': public_score(score), 'predictionProvenance': provenance,
              'sourceStatistics': source_statistics(truth['sourceCensus'], truth['factMappings'], score)}
    output = pathlib.Path(out_path)
    require(output.is_absolute() and output.parent.resolve() == output.parent, 'Unsafe output path')
    write_json(output, result)
    return result


def aggregate_source_scores(scores):
    groups, seen_components, seen_facts = {}, set(), set()
    for score in scores:
        require(score.get('kind') == 'source-assessment-score' and score.get('population') in {'primary', 'comparison'}, 'Invalid source score component')
        component = (score['population'], score['repositoryId'])
        require(component not in seen_components, 'Duplicate source score component')
        seen_components.add(component)
        stats = score['sourceStatistics']
        facts = set(stats['scopeFactIds'])
        require(not facts & seen_facts, 'Source facts counted more than once')
        seen_facts.update(facts)
        group = groups.setdefault(score['population'], {'S': 0, 'D': 0, 'positiveOccurrenceCount': 0, 'canonicalMappedMisses': 0,
                                                       'unknownFacts': 0, 'incompleteOwnedLines': 0, 'outsideMigrationScopeFacts': 0,
                                                       'mappingStatusCounts': {key: 0 for key in sorted(MAPPING_STATES)}, 'components': []})
        for key in ('S', 'D', 'positiveOccurrenceCount'):
            group[key] += stats[key]
        for key, value in stats['mappingStatusCounts'].items():
            group['mappingStatusCounts'][key] += value
        for target, source in (('canonicalMappedMisses', 'mappedMissFactIds'), ('unknownFacts', 'unknownFactIds'),
                               ('incompleteOwnedLines', 'incompleteOwnedLineIds'), ('outsideMigrationScopeFacts', 'outsideMigrationScopeFactIds')):
            group[target] += len(stats[source])
        group['components'].append(score['repositoryId'])
    for group in groups.values():
        unresolved = sum(count for state, count in group['mappingStatusCounts'].items() if state != 'mapped')
        group['canonicalProvenLowerFraction'] = group['D'] / group['S'] if group['S'] else None
        group['finiteKnownSourceCapabilityCoverage'] = group['D'] / group['S'] if group['S'] and not unresolved else None
        group['unresolvedPositiveMappings'] = unresolved
        group['coverageQualification'] = 'D is canonical-TP-proven only; its complement is not a miss'
        group['broadRecallUpperBound'] = None
        group['scope'] = 'Finite known owned-line source propositions; no whole-source recall'
    return groups


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    for command in ('admit-claims', 'admit-source'):
        child = sub.add_parser(command)
        for name in ('protocol', 'packet', 'adjudication', 'quality-dir', 'out'):
            child.add_argument('--' + name, required=True, type=pathlib.Path)
        child.add_argument('--rater', action='append', required=True, type=pathlib.Path)
        if command == 'admit-source':
            child.add_argument('--projection', required=True, type=pathlib.Path)
            child.add_argument('--mapping-check', required=True, type=pathlib.Path)
            child.add_argument('--scoring-policy', required=True, type=pathlib.Path)
    child = sub.add_parser('score-source')
    for name in ('admission', 'predictions', 'quality-dir', 'out'):
        child.add_argument('--' + name, required=True, type=pathlib.Path)
    child.add_argument('--repository-id', required=True)
    child.add_argument('--invocation-id', required=True)
    args = parser.parse_args(argv)
    try:
        if args.command == 'admit-claims':
            admit_claims(args.protocol, args.packet, args.rater, args.adjudication, args.quality_dir, args.out)
        elif args.command == 'admit-source':
            admit_source(args.protocol, args.packet, args.rater, args.adjudication, args.projection, args.mapping_check, args.quality_dir, args.out, args.scoring_policy)
        else:
            score_source(args.admission, args.repository_id, args.predictions, args.quality_dir, args.out, args.invocation_id)
        return 0
    except (ValueError, OSError, KeyError, TypeError, UnicodeError, IndexError) as error:
        print(f'Assessment failed: {error}', file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
