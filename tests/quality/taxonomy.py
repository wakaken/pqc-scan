"""Reviewed truth annotations; aggregate existing scorer dispositions, never match findings."""
import collections
import datetime
import pathlib

from evaluate import LABELS, bytes_digest, label_map, safe_relative

ALGORITHMS = ('RSA', 'DSA', 'ECDSA', 'ECDH', 'DH', 'EdDSA', 'ML-KEM', 'ML-DSA',
              'SLH-DSA', 'Hybrid', 'pre_standard', 'unknown', 'not_applicable', 'AES', 'HMAC')
USAGES = ('signature', 'key_exchange', 'kem', 'encryption', 'key_generation',
          'dependency_presence', 'certificate_signature', 'public_key', 'key_marker',
          'configuration', 'algorithm_reference', 'unknown', 'not_applicable')
ARTIFACTS = ('source', 'configuration', 'certificate', 'key_marker', 'dependency_manifest', 'sbom')
CASES = ('ordinary', 'comment', 'string', 'namesake', 'alias', 'shadowing', 'dynamic',
         'dead_code', 'disabled', 'mixed', 'same_line', 'typed', 'malformed', 'unsupported')
FIELDS = ('label', 'algorithms', 'usage', 'artifact', 'caseKind', 'parameters')
COUNTS = ('tp', 'fp', 'fn', 'tn', 'positive', 'negative', 'ambiguous', 'unsupported', 'excluded_predictions')


def annotation_audit_status(directory, truth):
    """Require bound source reviews, never infer completeness from predictions."""
    import json
    if not truth.get('annotationAuditRequired'):
        return 'Partial'
    data = json.loads((directory/'corpus.json').read_text())
    file_count = len(data['files'])
    rule_count = len({r for ids in data['families'].values() for r in ids})
    identities = set()
    for name, expected in truth['labelSources'].items():
        path = directory/safe_relative(name)
        if bytes_digest(path) != expected:
            raise ValueError('Annotation audit review binding changed')
        record = json.loads(path.read_text())
        audit = record.get('coverageAudit', {})
        if (audit.get('status') != 'Verified'
                or audit.get('scope') != 'all_nominated_rules_all_files'
                or audit.get('missingFacts') != [] or audit.get('unresolvedDefinitions') != []
                or not audit.get('reason', '').strip()
                or audit.get('fileCount') != file_count
                or audit.get('nominatedRuleCount') != rule_count
                or audit.get('reviewedFileRulePairs') != file_count * rule_count
                or audit.get('protocolSha256') != truth['freeze']['protocolSha256']):
            raise ValueError('Required annotation coverage audit incomplete')
        identities.add(record['reviewer'])
    if len(identities) != 2:
        raise ValueError('Two independent full annotation audits required')
    return 'Verified'


def select_corpus(repo, directory=None):
    root = repo/'tests/quality'
    selected = pathlib.Path(directory) if directory is not None else root
    if not selected.is_absolute():
        selected = repo/selected
    if (selected.resolve() != selected or not selected.is_dir()
            or not selected.is_relative_to(root)):
        raise ValueError('Corpus must be a nonsymlink directory under tests/quality')
    return selected


def annotation(row):
    try:
        if (row['label'] not in LABELS or not row['reason'].strip()
                or row['usage'] not in USAGES or row['artifact'] not in ARTIFACTS
                or row['caseKind'] not in CASES or not isinstance(row['algorithms'], list) or not row['algorithms']
                or len(set(row['algorithms'])) != len(row['algorithms'])
                or any(a not in ALGORITHMS for a in row['algorithms'])
                or not isinstance(row['parameters'], list)
                or any(not isinstance(p, str) or not p.strip() for p in row['parameters'])):
            raise ValueError('Invalid reviewed annotation')
        return {k: sorted(row[k]) if k in ('algorithms', 'parameters') else row[k] for k in FIELDS}
    except (KeyError, TypeError, AttributeError) as error:
        raise ValueError('Missing/invalid reviewed annotation') from error


def validate_annotations(data, truth, reviewers):
    if len(reviewers) != 2 or len({r['reviewer'] for r in reviewers}) != 2:
        raise ValueError('Two distinct annotation reviewers required')
    expected = {u['id'] for u in data['units']}
    maps = []
    for record in [truth, *reviewers]:
        labels = label_map(record['labels'])
        if set(labels) != expected:
            raise ValueError('Missing reviewed truth units')
        maps.append({r['id']: annotation(r) for r in record['labels']})
    decisions = {r['id']: r for r in truth['adjudications']}
    if len(decisions) != len(truth['adjudications']) or not set(decisions).issubset(expected):
        raise ValueError('Duplicate/unknown adjudication')
    for unit in expected:
        final, a, b = (m[unit] for m in maps)
        if final == a == b:
            if unit in decisions:
                raise ValueError('Unnecessary adjudication hides agreement')
            continue
        decision = decisions.get(unit, {})
        if not decision.get('reason', '').strip():
            raise ValueError('Missing source-only adjudication')
        if decision.get('resolution') != final:
            raise ValueError('Final annotation not bound to adjudication')
    artifacts = {f['path']: f.get('artifact') for f in data['files']}
    for unit in data['units']:
        expected_artifact = artifacts[unit['path']]
        if expected_artifact is not None and any(m[unit['id']]['artifact'] != expected_artifact for m in maps):
            raise ValueError('Artifact annotation differs from input manifest')


def validate_metadata_replay(directory, data, truth, reviewers):
    """Reuse blind evidence only when bytes, questions, definitions and labels are unchanged."""
    import json
    from evaluate import load_truth
    replay = truth['metadataReplay']
    previous = directory/safe_relative(replay['originalDirectory'])
    if previous.resolve() != previous:
        raise ValueError('Metadata replay cannot follow symlinks')
    old_truth = json.loads((previous/'truth.json').read_text())
    if 'metadataReplay' in old_truth:
        raise ValueError('Metadata replay chains are not independent blind evidence')
    if (bytes_digest(previous/'truth.json') != replay['originalTruthSha256']
            or bytes_digest(previous/'corpus.json') != replay['originalCorpusSha256']):
        raise ValueError('Metadata replay original evidence changed')
    old_data, _ = load_truth(previous)
    ignored = {'corpusVersion', 'revisionPurpose'}
    if ({k: v for k, v in old_data.items() if k not in ignored}
            != {k: v for k, v in data.items() if k not in ignored}
            or bytes_digest(previous/'PROTOCOL.md') != bytes_digest(directory/'PROTOCOL.md')
            or truth['labels'] != old_truth['labels']
            or truth['adjudications'] != old_truth['adjudications']):
        raise ValueError('Metadata replay changed inputs, questions, definitions or truth')
    originals = {}
    for name in old_truth['labelSources']:
        record = json.loads((previous/safe_relative(name)).read_text())
        originals[record['reviewer']] = (record, bytes_digest(previous/name))
    for record in reviewers:
        original, sha = originals.get(record['reviewer'], ({}, None))
        if (not record.get('metadataOnlyReplay') or not isinstance(record.get('scannerResultsSeen'), bool)
                or record.get('originalLabelRecord', {}).get('sha256') != sha
                or record['labels'] != original.get('labels')):
            raise ValueError('Metadata replay lacks unchanged original blind labels')
    if utc_time(truth['freeze']['timestampUtc']) < utc_time(old_truth['freeze']['timestampUtc']):
        raise ValueError('Metadata replay precedes original freeze')


def validate_freeze(directory, data, truth, reviewers):
    """Bind label provenance and instructions as well as final truth."""
    import json
    validate_annotations(data, truth, reviewers)
    if data.get('annotationAuditRequired') is True and truth.get('annotationAuditRequired') is not True:
        raise ValueError('Corpus requires full annotation audit')
    annotation_audit_status(directory, truth)
    freeze = truth['freeze']
    for name, field in [('PROTOCOL.md', 'protocolSha256'), ('provenance.json', 'provenanceSha256')]:
        path = directory/name
        if path.is_symlink() or bytes_digest(path) != freeze[field]:
            raise ValueError('Source-only protocol/provenance changed after freeze')
    for name in truth['labelSources']:
        path = directory/safe_relative(name)
        if path.resolve() != path:
            raise ValueError('Independent label evidence cannot follow symlinks')
    if freeze.get('beforeFirstMeasurement') is not True or not freeze.get('timestampUtc'):
        raise ValueError('Missing premeasurement freeze')
    frozen_at = utc_time(freeze['timestampUtc'])
    replay = truth.get('metadataReplay') is not None
    if replay:
        validate_metadata_replay(directory, data, truth, reviewers)
    for reviewer in reviewers:
        if (reviewer.get('sourceOnly') is not True or reviewer.get('otherLabelsSeen') is not False
                or (not replay and reviewer.get('scannerResultsSeen') is not False)
                or reviewer.get('protocolSha256') != freeze['protocolSha256']):
            raise ValueError('Missing blind-label attestation')
        if utc_time(reviewer.get('timestampUtc')) > frozen_at:
            raise ValueError('Label timestamp is after freeze')
    supplements = truth.get('sourceFactReviewBindings', {})
    reviewer_originals = {json.loads((directory/safe_relative(name)).read_text())['reviewer']: sha
                          for name, sha in truth['labelSources'].items()}
    supplement_reviewers = set()
    for name, sha in supplements.items():
        path = directory/safe_relative(name)
        if path.resolve() != path or bytes_digest(path) != sha:
            raise ValueError('Source-fact review evidence changed')
        record = json.loads(path.read_text())
        original_sha = record.get('originalLabelsSha256', record.get('originalLabelRecord', {}).get('sha256'))
        if (record.get('sourceOnly') is not True
                or record.get('newCorpusScannerResultsSeen', record.get('scannerResultsSeen')) is not False
                or record.get('otherLabelsSeen') is not False
                or original_sha != reviewer_originals.get(record.get('reviewer'))
                or record.get('reviewer') not in {r['reviewer'] for r in reviewers}
                or utc_time(record.get('timestampUtc')) > frozen_at):
            raise ValueError('Invalid source-fact review attestation/binding')
        supplement_reviewers.add(record['reviewer'])
    if supplements and supplement_reviewers != {r['reviewer'] for r in reviewers}:
        raise ValueError('Source-fact corrections need both reviewer identities')
    for decision in truth['adjudications']:
        if not set(decision.get('sourceFactReviews', [])).issubset(supplements):
            raise ValueError('Unbound source-fact adjudication')
    provenance = json.loads((directory/'provenance.json').read_text())
    bindings = provenance['files']
    if set(bindings) != {f['path'] for f in data['files']}:
        raise ValueError('Incomplete input provenance')
    for file in data['files']:
        item = bindings[file['path']]
        if item['sha256'] != file['sha256'] or item['lineage'] != file['lineage']:
            raise ValueError('Input provenance differs from manifest')
        if file['split'] == 'holdout' and not (item.get('upstreamCommit') and item.get('upstreamUrl')
                and item.get('upstreamSha256') and item.get('publishedBeforeScanner') is True):
            raise ValueError('Holdout lacks independently authored provenance')
    by_hash = collections.defaultdict(set)
    for file in data['files']:
        by_hash[file['sha256']].add(file['split'])
    if any(len(splits) > 1 for splits in by_hash.values()):
        raise ValueError('Duplicate input bytes cross tuning/holdout')
    for name, sha in provenance['licenseBindings'].items():
        if bytes_digest(directory/safe_relative(name)) != sha:
            raise ValueError('License evidence changed')


def utc_time(value):
    try:
        parsed = datetime.datetime.fromisoformat(value.replace('Z', '+00:00'))
        if parsed.utcoffset() != datetime.timedelta(0):
            raise ValueError('Timestamp must be UTC')
        return parsed
    except (TypeError, AttributeError, ValueError) as error:
        raise ValueError('Missing/invalid UTC evidence timestamp') from error


def aggregate(data, truth, scored, field, measure):
    annotations = {r['id']: r for r in truth['labels']}
    units = {u['id']: u for u in data['units']}
    files = {f['path']: f for f in data['files']}
    rows = {}
    def group(split, value):
        if isinstance(value, list):
            value = '+'.join(sorted(value))
        return rows.setdefault(f'{split}/{value}', dict.fromkeys(COUNTS, 0))
    vocabulary = {'algorithms': ALGORITHMS, 'usage': USAGES, 'artifact': ARTIFACTS, 'caseKind': CASES}
    for split in sorted({f['split'] for f in data['files']}):
        for value in (*vocabulary.get(field, ()), 'unattributed'):
            group(split, value)
    def value_of(row):
        if field == 'algorithmParameters':
            return '+'.join(sorted(row['algorithms'])) + ':' + ','.join(sorted(row['parameters']))
        return row[field]
    for unit in data['units']:
        row = annotations[unit['id']]
        group(files[unit['path']]['split'], value_of(row))[row['label']] += 1
    for detail in scored['details']:
        unit_id = detail['unit']
        path = units[unit_id]['path'] if unit_id else detail['location']['file']
        value = value_of(annotations[unit_id]) if unit_id else 'unattributed'
        if unit_id is None and field == 'artifact':
            value = files[path]['artifact']
        disposition = detail['disposition']
        key = 'excluded_predictions' if disposition == 'excluded_prediction' else disposition
        group(files[path]['split'], value)[key] += 1
    return {k: measure(v) for k, v in sorted(rows.items())}


def rule_coverage(data, truth, catalog):
    if len(catalog) != len(set(catalog)):
        raise ValueError('Duplicate catalog rule')
    counts = {rule: collections.Counter() for rule in catalog}
    labels = label_map(truth['labels'])
    for unit in data['units']:
        if unit['ruleId'] not in counts:
            raise ValueError('Nominated rule missing from catalog')
        counts[unit['ruleId']][labels[unit['id']]] += 1
    result = {}
    for rule, n in sorted(counts.items()):
        eligible = n['positive'] + n['negative']
        status = ('Verified' if n['positive'] and n['negative'] else 'Partial' if eligible
                  else 'Unknown' if n['ambiguous'] else 'Unsupported' if n['unsupported'] else 'Not Tested')
        result[rule] = dict(status=status, positive=n['positive'], negative=n['negative'],
                            ambiguous=n['ambiguous'], unsupported=n['unsupported'], eligible=eligible,
                            reason=('Positive and negative nominal units verified as present in truth; no detection acceptance/completeness claim' if status == 'Verified'
                                    else 'One-sided eligible evidence' if status == 'Partial'
                                    else 'No eligible unit; not a 0% or 100% result'))
    return result
