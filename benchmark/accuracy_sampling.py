"""Freeze source-owned-line and native-claim review packets, without labels/scoring.

Upstream source is untrusted data. This module never imports or executes it.
Only the operator-selected repository's existing quality helpers are imported.
"""
import argparse
import copy
import datetime
import hashlib
import importlib.util
import json
import os
import pathlib
import random
import re
import stat
import subprocess
import sys
from collections import Counter

SOURCE_LIMIT = 2 * 1024 * 1024
CONTEXT_LIMIT = 256 * 1024
JSON_LIMIT = 128 * 1024 * 1024
SOURCE_SEED = 'pqc-accuracy-b-20261126-source-v1'
OWNED_SEED = 'pqc-accuracy-20261126-owned-lines-v1'
SUPPLEMENT_SEED = 'pqc-accuracy-20261126-supplement-v1'
CLAIM_SEED = 'pqc-accuracy-b-20261126-claims-v1'
SUFFIXES = {'.c': 'C', '.cc': 'C++', '.cpp': 'C++', '.cxx': 'C++',
            '.java': 'Java', '.go': 'Go', '.rb': 'Ruby', '.py': 'Python',
            '.js': 'JavaScript', '.ts': 'TypeScript', '.tsx': 'TypeScript',
            '.rs': 'Rust', '.php': 'PHP', '.cs': 'C#'}
LEXICON = re.compile(r'(?i)(crypto|rsa|ecdsa|ecdh|ed25519|x25519|dilithium|kyber|ml.?kem|ml.?dsa|slh.?dsa|cipher|encrypt|decrypt|signature|certificate|tls|x509|hmac|aes|key.exchange|key.pair|public.key)')


def quality_helpers(repo_root):
    path = pathlib.Path(repo_root).resolve() / 'tests/quality/evaluate.py'
    spec = importlib.util.spec_from_file_location('accuracy_canonical_evaluate', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def read_bound(path, expected=None, limit=JSON_LIMIT):
    path = pathlib.Path(path).absolute()
    if path.resolve() != path or not stat.S_ISREG(path.lstat().st_mode):
        raise ValueError('unsafe_or_nonregular_binding')
    with path.open('rb') as stream:
        raw = stream.read(limit + 1)
    if len(raw) > limit:
        raise ValueError('resource_limit')
    if expected is not None and sha(raw) != expected:
        raise ValueError('sha256_drift')
    return raw


def _object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError('duplicate_json_key')
        result[key] = value
    return result


def load_json(path, expected=None):
    return json.loads(read_bound(path, expected), object_pairs_hook=_object)


def write_new(path, data):
    with os.fdopen(os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), 'w', encoding='utf-8') as stream:
        json.dump(data, stream, ensure_ascii=False, indent=2, allow_nan=False)
        stream.write('\n')


def rng(seed, tag):
    return random.Random(int(sha((seed + '\0' + tag).encode()), 16))


_DISPLAY_SPEC = importlib.util.spec_from_file_location('accuracy_display', pathlib.Path(__file__).with_name('accuracy_display.py'))
_DISPLAY = importlib.util.module_from_spec(_DISPLAY_SPEC)
_DISPLAY_SPEC.loader.exec_module(_DISPLAY)


def mask_secrets(text):
    return _DISPLAY.mask_text(text)


def safe_value(value):
    return _DISPLAY.safe_content(value)


def safe_finding(finding):
    return _DISPLAY.safe_native(finding)


def assertion_checklist(finding):
    """Freeze all fact-bearing field paths without weakening the native claim."""
    result = []
    factual_fields = ('rule_id', 'description', 'category', 'location', 'evidence',
                      'risk', 'source_snippet', 'migration_hint', 'recommended_actions')
    advice_requirement = ('Review every factual premise and qualifier recursively in all nested strings, including title, '
                          'rationale, steps, code_examples, algorithm, API, version and platform prerequisites. '
                          'Assess conditional examples under their stated conditions; preserve nominal reference versus '
                          'implementation, call and deployment strength. Exclude normative imperatives, recommended future '
                          'choices, priority, severity and confidence calibration, and unasserted runtime execution. '
                          'A factual premise inside advice is included. For solely normative advice, explicitly document '
                          'that no factual assertion was found; never relabel an unverified premise as normative.')
    requirements = {
        'risk': 'Review risk as a factual taxonomy claim, independently of priority or severity calibration.',
        'source_snippet': ('Review quoted-source identity, line numbers, coordinates and the declared source binding; '
                           'redacted or unavailable material remains unknown where it prevents verification.'),
        'migration_hint': advice_requirement,
        'recommended_actions': advice_requirement,
    }
    for key in factual_fields:
        if key in finding:
            entry = dict(id='native-' + key, fieldPath='/' + key,
                         assertedValue=finding[key], decomposition='review_every_factual_assertion_and_qualifier')
            if key in requirements:
                entry['requirement'] = requirements[key]
            result.append(entry)
    undisposed = sorted(set(finding) - set(factual_fields) - {'finding_id', 'priority', 'severity', 'confidence'})
    if undisposed:
        result.append(dict(id='unclassified-native-fields', fieldPath='/',
                           undisposedFieldPaths=['/' + key.replace('~', '~0').replace('/', '~1') for key in undisposed],
                           requirement='Keep unknown until every unclassified field has an explicit frozen factual or excluded disposition.'))
    result.append(dict(id='taxonomy-contract', fieldPath='/category',
                       requirement='Use explicitly documented category taxonomy; unresolved category remains unknown.'))
    if finding.get('evidence', {}).get('type') == 'certificate':
        result.append(dict(id='certificate-time-object', fieldPath='/evidence/metadata',
                           requirement='Bind parsed object identity. Actual assessment timestamp is required only for temporal predicates; report generated_at is insufficient. Static algorithm, object and explicit-date facts remain independently evaluable.'))
    return result


def source_bytes(repo, root_map, item, helpers):
    relative = helpers.safe_relative(item['path'])
    root = pathlib.Path(root_map.get(repo['id'], repo['root'])).resolve(strict=True)
    path = root / relative
    if item.get('mode') not in {'100644', '100755'}:
        raise ValueError('manifest_not_regular')
    if not 0 <= item['bytes'] <= SOURCE_LIMIT:
        raise ValueError('resource_limit')
    raw = read_bound(path, item['sha256'], SOURCE_LIMIT)
    if len(raw) != item['bytes']:
        raise ValueError('size_drift')
    return raw, str(root)


def source_status(error):
    if isinstance(error, FileNotFoundError):
        return 'missing_source'
    if isinstance(error, UnicodeError):
        return 'undecodable_source'
    if isinstance(error, OSError):
        return 'source_io_error'
    return str(error) if str(error) in {'unsafe_or_nonregular_binding', 'resource_limit', 'sha256_drift', 'size_drift', 'manifest_not_regular'} else 'invalid_source_binding'


def selected_window(repo_id, path, lines, stratum):
    windows = list(range(0, len(lines), 32))
    hits = [start for start in windows if any(LEXICON.search(line) for line in lines[start:start + 32])]
    choices = hits if stratum == 'lexical_path' and hits else windows
    if not choices:
        return None, 0, 'empty_source'
    start = rng(SOURCE_SEED, repo_id + '\0' + path).choice(choices)
    return start, len(choices), 'lexicon_in_selected_file' if stratum == 'lexical_path' and hits else 'uniform_window'


def context_at(text, owned_line):
    lines = text.splitlines(keepends=True)
    if not isinstance(owned_line, int) or not 1 <= owned_line <= len(lines):
        return dict(contextStart=None, contextEnd=None, contextSafeText=None, contextStatus='invalid_anchor')
    start, end = max(0, owned_line - 21), min(len(lines), owned_line + 20)
    safe, changed = mask_secrets(text)
    context = ''.join(safe.splitlines(keepends=True)[start:end])
    if len(context.encode()) > CONTEXT_LIMIT:
        return dict(contextStart=start + 1, contextEnd=end, contextSafeText=None,
                    contextStatus='context_resource_limit', redacted=changed)
    return dict(contextStart=start + 1, contextEnd=end, contextSafeText=context,
                contextStatus='ready', redacted=changed,
                redactionPreventsVerification='review_required' if changed else False)


def source_record(selection, ordinal, repo, manifest, roots, helpers):
    record = {key: selection[key] for key in ('repositoryId', 'population', 'group', 'language', 'split', 'sourcePath', 'sourceSha256')}
    record.update(id=selection.get('id', 'source-b-' + str(ordinal).zfill(4)), commit=repo['commit'],
                  sourceRoot=str(roots.get(repo['id'], repo['root'])), ownedLine=None,
                  contextStart=None, contextEnd=None, contextSafeText=None,
                  sourceSelectionOrdinal=ordinal, sourceStatus='pending',
                  scopeMetadata=dict(unit='one_owned_source_line', contextIsCensus=False,
                                     stratum=selection['stratum'], historicalSelection=ordinal < 374,
                                     manifestSha256=repo['inputManifestSha256']))
    scope = record['scopeMetadata']
    for name in ('lineStart', 'lineEnd', 'windowFrame', 'conditionalWindowProbability', 'windowSelection', 'fileFrame', 'selectedFiles', 'fileInclusionProbability'):
        if name in selection:
            scope[name] = selection[name]
    item = manifest.get(selection['sourcePath'])
    if item is None or item['sha256'] != selection['sourceSha256']:
        record['sourceStatus'] = 'manifest_selection_mismatch'
        return record
    try:
        raw, record['sourceRoot'] = source_bytes(repo, roots, item, helpers)
        text = raw.decode('utf-8')
        lines = text.splitlines()
        start, frame, mode = selected_window(repo['id'], item['path'], lines, selection['stratum'])
        if start is None:
            record['sourceStatus'] = mode
            return record
        if 'lineStart' in selection and (selection['lineStart'] != start + 1 or selection['lineEnd'] != min(start + 32, len(lines))):
            raise ValueError('historical_window_drift')
        if selection.get('supplementWindow') is not None:
            start = selection['supplementWindow']
            frame, mode = len(range(0, len(lines), 32)), 'unused_block_fallback'
        scope.update(lineStart=start + 1, lineEnd=min(start + 32, len(lines)),
                     windowFrame=frame, conditionalWindowProbability=1 / frame, windowSelection=mode)
        nonempty = [i + 1 for i in range(start, min(start + 32, len(lines))) if lines[i].strip()]
        lexical = [i for i in nonempty if LEXICON.search(lines[i - 1])]
        choices = lexical if mode == 'lexicon_in_selected_file' and lexical else nonempty
        if not choices:
            record['sourceStatus'] = 'empty_selected_block'
            return record
        owned = selection.get('supplementOwnedLine')
        if owned is None:
            owned = rng(OWNED_SEED, repo['id'] + '\0' + item['path'] + '\0' + str(start)).choice(choices)
        record['ownedLine'] = owned
        scope.update(ownedLineFrame=len(choices), ownedLineSelection='uniform_lexicon_hit_line' if choices is lexical else 'uniform_nonempty_line',
                     conditionalOwnedLineProbability=1 / len(choices))
        record.update(context_at(text, owned))
        record['sourceStatus'] = 'ready' if record['contextStatus'] == 'ready' else record['contextStatus']
    except (ValueError, OSError, UnicodeError) as error:
        record['sourceStatus'] = 'historical_window_drift' if str(error) == 'historical_window_drift' else source_status(error)
    return record


def supplement_selections(selected, repositories, manifests, roots, helpers):
    """Fill each primary repository's four slots before any source labels exist."""
    result = []
    for repo in repositories:
        if repo['population'] != 'primary':
            continue
        prior = [row for row in selected if row['repositoryId'] == repo['id']]
        deficit = max(0, 4 - len(prior))
        used = {row['sourcePath'] for row in prior}
        eligible = [x for x in manifests[repo['id']].values() if x.get('mode') in {'100644', '100755'} and 0 < x['bytes'] <= SOURCE_LIMIT and pathlib.PurePosixPath(x['path']).suffix.lower() in SUFFIXES]
        remaining = sorted((x for x in eligible if x['path'] not in used), key=lambda x: x['path'])
        rng(SUPPLEMENT_SEED, repo['id']).shuffle(remaining)
        allocations = [(item, {}) for item in remaining[:deficit]]
        if len(allocations) < deficit:
            fallback = []
            for item in sorted(eligible, key=lambda x: x['path']):
                try:
                    raw, _ = source_bytes(repo, roots, item, helpers)
                    lines = raw.decode().splitlines()
                    old_starts = {row['lineStart'] - 1 if 'lineStart' in row else selected_window(repo['id'], item['path'], lines, row['stratum'])[0]
                                  for row in prior if row['sourcePath'] == item['path']}
                    for start in range(0, len(lines), 32):
                        if start not in old_starts:
                            fallback.append((item, dict(supplementWindow=start)))
                except (ValueError, OSError, UnicodeError):
                    continue
            rng(SUPPLEMENT_SEED, repo['id'] + '\0unused-blocks').shuffle(fallback)
            allocations.extend(fallback[:deficit - len(allocations)])
        if len(allocations) < deficit:
            used_lines, line_fallback = set(), []
            for index, row in enumerate(prior):
                record = source_record(row, index, repo, manifests[repo['id']], roots, helpers)
                used_lines.add((row['sourcePath'], record['ownedLine']))
            for item, extra in allocations:
                row = dict(repositoryId=repo['id'], population='primary', group=repo['group'], split=repo['split'],
                           sourcePath=item['path'], sourceSha256=item['sha256'], language=SUFFIXES[pathlib.PurePosixPath(item['path']).suffix.lower()],
                           stratum='predeclared_supplement', **extra)
                record = source_record(row, 374, repo, manifests[repo['id']], roots, helpers)
                used_lines.add((item['path'], record['ownedLine']))
            for item in sorted(eligible, key=lambda x: x['path']):
                try:
                    raw, _ = source_bytes(repo, roots, item, helpers)
                    for i, line in enumerate(raw.decode().splitlines(), 1):
                        if line.strip() and (item['path'], i) not in used_lines:
                            line_fallback.append((item, dict(supplementWindow=((i - 1) // 32) * 32, supplementOwnedLine=i)))
                except (ValueError, OSError, UnicodeError):
                    continue
            rng(SUPPLEMENT_SEED, repo['id'] + '\0unused-lines').shuffle(line_fallback)
            allocations.extend(line_fallback[:deficit - len(allocations)])
        for index in range(deficit):
            if index < len(allocations):
                item, extra = allocations[index]
                result.append(dict(repositoryId=repo['id'], population='primary', group=repo['group'], split=repo['split'],
                                   sourcePath=item['path'], sourceSha256=item['sha256'], language=SUFFIXES[pathlib.PurePosixPath(item['path']).suffix.lower()],
                                   stratum='predeclared_supplement', **extra))
            else:
                result.append(dict(repositoryId=repo['id'], population='primary', group=repo['group'], split=repo['split'],
                                   sourcePath='', sourceSha256=None, language=repo['languages'][0], stratum='frame_exhausted'))
    return result


def certificate_metadata(raw, as_of, openssl='/usr/bin/openssl'):
    """Parse public certificate metadata only; never persist OpenSSL text output."""
    if len(raw) > SOURCE_LIMIT:
        return dict(status='resource_limit')
    pem_count = raw.count(b'-----BEGIN CERTIFICATE-----')
    if pem_count > 1 or b'PRIVATE KEY-----' in raw or (pem_count == 1 and raw.count(b'-----END CERTIFICATE-----') != 1):
        return dict(status='complex_or_private_bundle_unknown')
    form = 'PEM' if pem_count else 'DER'
    if form == 'DER':
        if len(raw) < 2 or raw[0] != 0x30:
            return dict(status='unsupported_certificate_encoding')
        n = raw[1]
        header = 2
        if n & 128:
            count = n & 127
            if not 1 <= count <= 4 or len(raw) < 2 + count:
                return dict(status='invalid_der_length')
            n = int.from_bytes(raw[2:2 + count], 'big')
            header += count
        if header + n != len(raw):
            return dict(status='complex_or_trailing_der_unknown')
    try:
        parsed = subprocess.run([openssl, 'x509', '-inform', form, '-noout', '-text', '-dates'], input=raw,
                                stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, timeout=5, check=False,
                                env={'PATH': '/usr/bin:/bin', 'LANG': 'C', 'LC_ALL': 'C'})
    except (OSError, subprocess.TimeoutExpired):
        return dict(status='certificate_parser_unavailable')
    if parsed.returncode or len(parsed.stdout) > 8 * SOURCE_LIMIT:
        return dict(status='certificate_parse_unknown')
    text = parsed.stdout.decode('ascii', errors='replace')
    patterns = {'signatureAlgorithm': r'Signature Algorithm:\s*([^\r\n]+)',
                'publicKeyAlgorithm': r'Public Key Algorithm:\s*([^\r\n]+)',
                'keyBits': r'Public-Key:\s*\((\d+) bit\)',
                'notBefore': r'^notBefore=(.+)$', 'notAfter': r'^notAfter=(.+)$'}
    data = {key: match.group(1).strip() for key, pattern in patterns.items() if (match := re.search(pattern, text, re.M))}
    data.update(status='parsed', parser=openssl, encoding=form, asOf=as_of, certificateInputSha256=sha(raw))
    try:
        der = subprocess.run([openssl, 'x509', '-inform', form, '-outform', 'DER'], input=raw,
                             stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, timeout=5, check=False,
                             env={'PATH': '/usr/bin:/bin', 'LANG': 'C', 'LC_ALL': 'C'})
        if der.returncode or not der.stdout or len(der.stdout) > SOURCE_LIMIT:
            data['status'] = 'certificate_object_identity_unknown'
        else:
            data.update(certificateDerSha256=sha(der.stdout), objectIndex=0)
    except (OSError, subprocess.TimeoutExpired):
        data['status'] = 'certificate_object_identity_unknown'
    if 'keyBits' in data:
        data['keyBits'] = int(data['keyBits'])
    try:
        date = datetime.datetime.fromisoformat(as_of.replace('Z', '+00:00'))
        before = datetime.datetime.strptime(data['notBefore'], '%b %d %H:%M:%S %Y GMT').replace(tzinfo=datetime.timezone.utc)
        after = datetime.datetime.strptime(data['notAfter'], '%b %d %H:%M:%S %Y GMT').replace(tzinfo=datetime.timezone.utc)
        data['validAtAsOf'] = before <= date <= after
    except (ValueError, KeyError, AttributeError):
        data['validAtAsOf'] = None
    return data


def validate_claim_frame(probe, helpers, comparison=False):
    bindings = probe['bindings']
    if len({x['repositoryId'] for x in bindings}) != len(bindings):
        raise ValueError('duplicate_claim_repository')
    invocation = load_json(probe['invocation']['path'], probe['invocation']['sha256'])
    if invocation['scanner']['binary'] != probe['scannerBinary']:
        raise ValueError('scanner_binding_drift')
    rows = {row['repositoryId']: row for row in invocation['rows']}
    population = sum(x['findingCount'] for x in bindings)
    if population != probe['populationCount']:
        raise ValueError('claim_population_drift')
    if comparison:
        expected, count = [], 0
        for binding in sorted(bindings, key=lambda x: x['repositoryId']):
            expected.extend(count + i for i in sorted(rng(probe['sampleSeed'], binding['repositoryId']).sample(range(binding['findingCount']), min(3, binding['findingCount']))))
            count += binding['findingCount']
    else:
        expected = sorted(random.Random(probe['sampleSeed']).sample(range(population), probe['sampleCount']))
    if [x['populationOrdinal'] for x in probe['selected']] != expected:
        raise ValueError('claim_sample_drift')
    if probe['sampleSeed'] != (CLAIM_SEED + '-comparison' if comparison else CLAIM_SEED):
        raise ValueError('unexpected_claim_seed')
    reports, offset = {}, 0
    selected_by_repo = {}
    for selected in probe['selected']:
        selected_by_repo.setdefault(selected['repositoryId'], []).append(selected)
    for binding in sorted(bindings, key=lambda x: x['repositoryId']):
        repo_id = binding['repositoryId']
        row = rows[repo_id]
        if row['execution'] != 'Succeeded' or row['receipt'] != binding['terminal'] or row['assessment']['findingCount'] != binding['findingCount']:
            raise ValueError('claim_terminal_binding_drift')
        terminal = load_json(binding['terminal']['path'], binding['terminal']['sha256'])
        if terminal['artifacts']['report.json']['sha256'] != binding['reportSha256']:
            raise ValueError('claim_report_binding_drift')
        report = load_json(binding['reportPath'], binding['reportSha256'])
        if len(report['findings']) != binding['findingCount'] or report['generated_at'] != binding['generatedAt']:
            raise ValueError('claim_report_population_drift')
        for selected in selected_by_repo.get(repo_id, []):
            index = selected['nativeOrdinal']
            finding = report['findings'][index]
            if (offset + index != selected['populationOrdinal'] or helpers.digest(finding) != selected['findingSha256']
                    or selected['reportSha256'] != binding['reportSha256'] or selected['reportPath'] != binding['reportPath']
                    or selected['ruleId'] != finding['rule_id'] or selected['location'] != finding['location']):
                raise ValueError('native_finding_drift')
        if repo_id in selected_by_repo:
            reports[repo_id] = report
        offset += binding['findingCount']
    return reports


def freeze_comparison_claims(invocation_path, helpers):
    invocation = load_json(invocation_path)
    bindings, selected, offset = [], [], 0
    for row in sorted(invocation['rows'], key=lambda x: x['repositoryId']):
        terminal = load_json(row['receipt']['path'], row['receipt']['sha256'])
        report_path = pathlib.Path(row['receipt']['path']).parent / 'output/report.json'
        report_sha = terminal['artifacts']['report.json']['sha256']
        report = load_json(report_path, report_sha)
        findings = report['findings']
        bindings.append(dict(repositoryId=row['repositoryId'], reportPath=str(report_path), reportSha256=report_sha,
                             findingCount=len(findings), generatedAt=report['generated_at'], terminal=row['receipt']))
        indices = rng(CLAIM_SEED + '-comparison', row['repositoryId']).sample(range(len(findings)), min(3, len(findings)))
        for index in sorted(indices):
            finding = findings[index]
            selected.append(dict(sampleId='claim-comparison-' + str(offset + index).zfill(6),
                                 populationOrdinal=offset + index, repositoryId=row['repositoryId'], nativeOrdinal=index,
                                 findingSha256=helpers.digest(finding), ruleId=finding['rule_id'], location=finding['location'],
                                 reportPath=str(report_path), reportSha256=report_sha))
        offset += len(findings)
    return dict(version=1, kind='separate_comparison_claim_controls',
                invocation=dict(path=str(invocation_path), sha256=helpers.bytes_digest(invocation_path)),
                scannerBinary=invocation['scanner']['binary'], bindings=bindings, selected=selected,
                populationCount=offset, sampleCount=len(selected), sampleSeed=CLAIM_SEED + '-comparison')


def historical_assessment_at(value):
    """Parse only the native historical Unix-time boundary, without clock fallback."""
    if isinstance(value, str):
        if len(value) > 20 or re.fullmatch(r'(?:0|-?[1-9][0-9]*)', value, flags=re.ASCII) is None:
            return None
        value = int(value)
    if type(value) is not int or not -62135596800 <= value <= 253402300799:
        return None
    try:
        return datetime.datetime.fromtimestamp(value, datetime.timezone.utc).isoformat()
    except (ValueError, OverflowError, OSError):
        return None


def claim_record(selected, finding, report, repo, manifest, roots, helpers, private_dir):
    safe = safe_finding(finding)
    original_safe = safe == finding
    name = selected['sampleId'] + '.json'
    write_new(private_dir / name, safe)
    record = {key: selected[key] for key in ('sampleId', 'repositoryId', 'nativeOrdinal', 'populationOrdinal', 'reportPath', 'reportSha256', 'findingSha256')}
    record.update(id=selected['sampleId'], population=repo['population'], group=repo['group'], language=repo['languages'][0],
                  split=repo['split'], commit=repo['commit'], asOf=report['generated_at'],
                  nativeFinding=safe, nativeFindingPrivatePath='private/native-findings/' + name,
                  originalPreservation='private_exact_copy_and_bound_report' if original_safe else 'bound_report_only_private_copy_redacted',
                  nativeFindingRedacted=not original_safe,
                  assertionChecklist=assertion_checklist(safe),
                  claimSummary={key: safe[key] for key in ('rule_id', 'category', 'description', 'location', 'evidence') if key in safe},
                  sourceReferences=[], sourceStatus='pending', contextSafeText=None)
    path = finding['location']['file']
    item = manifest.get(path)
    reference = dict(sourcePath=path, sourceSha256=item['sha256'] if item else None,
                     manifestSha256=repo['inputManifestSha256'], line=finding['location']['line'], column=finding['location']['column'])
    record['sourceReferences'].append(reference)
    if item is None:
        record['sourceStatus'] = reference['status'] = 'source_not_in_manifest'
        return record
    try:
        raw, root = source_bytes(repo, roots, item, helpers)
        reference['sourceRoot'] = root
        if finding.get('evidence', {}).get('type') == 'certificate':
            timestamp = finding.get('evidence', {}).get('metadata', {}).get('assessment_time_unix')
            assessed = historical_assessment_at(timestamp)
            record['staticMetadata'] = certificate_metadata(raw, assessed)
            record['staticMetadata'].update(historicalAssessmentAt=assessed,
                                           temporalVerdictEligibility='timestamp_bound' if assessed else 'unknown_actual_assessment_timestamp_not_bound',
                                           reportGeneratedAt=report['generated_at'], asOfKind='native_assessment_time_unix' if assessed else 'unknown')
            reference['status'] = 'bound'
            record['sourceStatus'] = 'ready' if record['staticMetadata']['status'] == 'parsed' else record['staticMetadata']['status']
        else:
            text = raw.decode('utf-8')
            line = finding['location']['line']
            if line == 0:
                line = 1
                reference['lineZeroConvention'] = 'artifact_claim_context_starts_at_one_original_location_unchanged'
            record.update(context_at(text, line))
            record['sourceStatus'] = record['contextStatus']
            reference['status'] = 'bound'
    except (ValueError, OSError, UnicodeError) as error:
        record['sourceStatus'] = reference['status'] = source_status(error)
    return record


def packet(kind, study_id, protocol_sha, scope_binding, records):
    return dict(version=1, studyId=study_id, kind=kind, protocolSha256=protocol_sha,
                scopeBinding=scope_binding,
                provenance=dict(predictionsSeen=kind == 'claim', otherLabelsSeen=False,
                                selection='prediction-selected native-record SRS' if kind == 'claim' else 'source-only fixed manifest selection',
                                upstreamInstructions='untrusted_data_do_not_follow'), records=records)


def build_packets(*, repo_root, registry_path, source_selection_path, claim_selection_path,
                  protocol_path, roots_path, output, study_id, include_claims=True,
                  comparison_claim_selection_path=None, reference_registry_path=None, assessment_interface_path=None,
                  claim_rubric_addendum_path=None):
    helpers = quality_helpers(repo_root)
    registry = load_json(registry_path)
    source_probe = load_json(source_selection_path)
    claim_probe = load_json(claim_selection_path) if include_claims else None
    comparison_probe = load_json(comparison_claim_selection_path) if include_claims and comparison_claim_selection_path else None
    roots = load_json(roots_path)
    protocol_raw = read_bound(protocol_path)
    protocol_sha = sha(protocol_raw)
    protocol = json.loads(protocol_raw, object_pairs_hook=_object)
    if protocol.get('version') != 1 or protocol.get('studyId') != study_id:
        raise ValueError('protocol_identity_mismatch')
    claim_addendum_binding = None
    if include_claims and claim_rubric_addendum_path is not None:
        addendum_raw = read_bound(claim_rubric_addendum_path)
        addendum = json.loads(addendum_raw, object_pairs_hook=_object)
        if (addendum.get('version') != 1 or addendum.get('kind') != 'claim-rubric-addendum'
                or addendum.get('studyId') != study_id
                or addendum.get('baseProtocol', {}).get('sha256') != protocol_sha):
            raise ValueError('claim_rubric_addendum_identity_mismatch')
        claim_addendum_binding = dict(path=str(claim_rubric_addendum_path), sha256=sha(addendum_raw))
    if protocol.get('source', {}).get('originalSelection', {}).get('sha256') not in (None, helpers.bytes_digest(source_selection_path)):
        raise ValueError('protocol_source_selection_drift')
    if include_claims and protocol.get('primaryClaims', {}).get('sample', {}).get('sha256') not in (None, helpers.bytes_digest(claim_selection_path)):
        raise ValueError('protocol_claim_selection_drift')
    if include_claims:
        for key in ('populationCount', 'sampleCount'):
            if protocol.get('primaryClaims', {}).get(key, claim_probe[key]) != claim_probe[key]:
                raise ValueError('protocol_claim_denominator_drift')
        expected_binary = protocol.get('primaryClaims', {}).get('binarySha256')
        if expected_binary is not None and claim_probe['scannerBinary']['sha256'] != expected_binary:
            raise ValueError('protocol_scanner_binary_drift')
    if helpers.bytes_digest(registry_path) != source_probe['registry']['sha256']:
        raise ValueError('registry_selection_drift')
    repositories = registry['repositories']
    by_id = {repo['id']: repo for repo in repositories}
    if len(by_id) != len(repositories):
        raise ValueError('duplicate_registry_repository')
    manifests = {}
    for repo in repositories:
        rows = load_json(repo['inputManifest'], repo['inputManifestSha256'])
        index = {}
        for row in rows:
            helpers.safe_relative(row['path'])
            if row['path'] in index:
                raise ValueError('duplicate_manifest_path')
            index[row['path']] = row
        manifests[repo['id']] = index
    selected = copy.deepcopy(source_probe['selected'])
    for row in selected:
        repo = by_id[row['repositoryId']]
        if any(row[key] != repo[key] for key in ('population', 'group', 'split')):
            raise ValueError('source_selection_metadata_drift')
        helpers.safe_relative(row['sourcePath'])
    supplemental = supplement_selections(selected, repositories, manifests, roots, helpers)
    selected.extend(supplemental)
    source_records = [source_record(row, i, by_id[row['repositoryId']], manifests[row['repositoryId']], roots, helpers) for i, row in enumerate(selected)]
    reports = validate_claim_frame(claim_probe, helpers) if include_claims else {}
    comparison_reports = validate_claim_frame(comparison_probe, helpers, comparison=True) if comparison_probe else {}
    output = pathlib.Path(output).absolute()
    output.mkdir(parents=True, exist_ok=False)
    os.chmod(output, 0o700)
    (output / 'source').mkdir()
    (output / 'claims').mkdir()
    private = output / 'private/native-findings'
    private.mkdir(parents=True)
    os.chmod(output / 'private', 0o700)
    binding = dict(registrySha256=helpers.bytes_digest(registry_path),
                   sourceSelectionSha256=helpers.bytes_digest(source_selection_path), rootsSha256=helpers.bytes_digest(roots_path),
                   manifestBindings={repo['id']: repo['inputManifestSha256'] for repo in repositories},
                   generatorBinding=dict(path=str(pathlib.Path(__file__).resolve()), sha256=sha(read_bound(pathlib.Path(__file__).resolve()))),
                   canonicalHelpersBinding=dict(path=str(pathlib.Path(helpers.__file__).resolve()), sha256=sha(read_bound(pathlib.Path(helpers.__file__).resolve()))),
                   physicalRootsAreIdentity=False)
    if reference_registry_path is not None:
        binding['primaryReferenceRegistry'] = dict(path=str(reference_registry_path), sha256=sha(read_bound(reference_registry_path)))
    if assessment_interface_path is not None:
        binding['assessmentInterface'] = dict(path=str(assessment_interface_path), sha256=sha(read_bound(assessment_interface_path)))
    source_binding = dict(binding, sourceSeed=SOURCE_SEED, ownedLineSeed=OWNED_SEED, supplementSeed=SUPPLEMENT_SEED)
    write_new(output / 'source/selection.json', dict(version=1, selected=selected,
              sourceSelectionSha256=binding['sourceSelectionSha256'], supplementalCount=len(supplemental)))
    primary = [r for r in source_records if r['population'] == 'primary']
    comparison = [r for r in source_records if r['population'] != 'primary']
    pilot = []
    for group in sorted({r['group'] for r in primary}):
        pilot.append(next(r for r in primary if r['group'] == group))
    write_new(output / 'source/pilot.json', packet('source', study_id, protocol_sha, source_binding, pilot))
    for population, records in [('primary', primary), ('comparison', comparison)]:
        for i in range(0, len(records), 40):
            write_new(output / ('source/' + population + '-' + str(i // 40 + 1).zfill(3) + '.json'),
                      packet('source', study_id, protocol_sha, source_binding, records[i:i + 40]))
    claim_records = []
    if include_claims:
        claim_binding = dict(binding, claimSelectionSha256=helpers.bytes_digest(claim_selection_path),
                             invocation=claim_probe['invocation'], scannerBinary=claim_probe['scannerBinary'],
                             sampleSeed=claim_probe['sampleSeed'], populationCount=claim_probe['populationCount'],
                             sampleCount=claim_probe['sampleCount'], inclusionProbability=claim_probe['inclusionProbability'])
        if claim_addendum_binding is not None:
            claim_binding['claimRubricAddendum'] = claim_addendum_binding
        for selected in claim_probe['selected']:
            repo = by_id[selected['repositoryId']]
            report = reports[repo['id']]
            claim_records.append(claim_record(selected, report['findings'][selected['nativeOrdinal']], report,
                                             repo, manifests[repo['id']], roots, helpers, private))
        for i in range(0, len(claim_records), 50):
            write_new(output / ('claims/primary-' + str(i // 50 + 1).zfill(3) + '.json'),
                      packet('claim', study_id, protocol_sha, claim_binding, claim_records[i:i + 50]))
        write_new(output / 'claims/primary-all.json', packet('claim', study_id, protocol_sha, claim_binding, claim_records))
        if comparison_probe:
            comparison_records = []
            for selected in comparison_probe['selected']:
                repo = by_id[selected['repositoryId']]
                report = comparison_reports[repo['id']]
                comparison_records.append(claim_record(selected, report['findings'][selected['nativeOrdinal']], report,
                                                      repo, manifests[repo['id']], roots, helpers, private))
            comparison_binding = dict(binding, claimSelectionSha256=helpers.bytes_digest(comparison_claim_selection_path),
                                      invocation=comparison_probe['invocation'], scannerBinary=comparison_probe['scannerBinary'],
                                      selection='independent per-repository comparison controls; excluded from primary estimator')
            if claim_addendum_binding is not None:
                comparison_binding['claimRubricAddendum'] = claim_addendum_binding
            write_new(output / 'claims/comparison-all.json', packet('claim', study_id, protocol_sha, comparison_binding, comparison_records))
            claim_records.extend(comparison_records)
    summary = dict(version=1, studyId=study_id, protocolSha256=protocol_sha,
                   disposition='packet_preparation_only_no_labels_no_accuracy_result',
                   primarySourceSlots=len(primary), comparisonSourceSlots=len(comparison),
                   primaryRepositories=len({r['repositoryId'] for r in primary}),
                   comparisonRepositories=len({r['repositoryId'] for r in comparison}),
                   primaryGroups=dict(Counter(r['group'] for r in primary)),
                   primaryLanguages=dict(Counter(r['language'] for r in primary)),
                   sourceStatuses=dict(Counter(r['sourceStatus'] for r in source_records)),
                   claimCount=len(claim_records), claimStatuses=dict(Counter(r['sourceStatus'] for r in claim_records)),
                   comparisonClaimState='prepared_separate_controls' if comparison_probe else 'pending_separate_frozen_claim_frame_not_supplied',
                   requiredReview='independent_implementation_and_protocol_review_before_semantic_labeling',
                   fileBindings={str(p.relative_to(output)): helpers.bytes_digest(p) for p in sorted(output.rglob('*.json'))})
    write_new(output / 'index.json', summary)
    return summary


def display_view(packet_path, packet_sha, output):
    """Create a display projection without changing the original packet identity."""
    original = load_json(packet_path, packet_sha)
    if original.get('kind') not in {'source', 'claim'} or original.get('version') != 1:
        raise ValueError('invalid_display_packet')
    view = copy.deepcopy(original)
    root = pathlib.Path(output).absolute()
    if root.parent.resolve() != root.parent:
        raise ValueError('unsafe_display_output')
    root.mkdir(mode=0o700)
    binding = dict(version=1, originalPacket=dict(path=str(pathlib.Path(packet_path).absolute()), sha256=packet_sha),
                   tools={name: sha(read_bound(pathlib.Path(__file__).with_name(name)))
                          for name in ('accuracy_sampling.py','accuracy_display.py')},
                   policy='finite_known_private_syntax_and_opaque_content_v1',
                   nativeMaskPolicy='known_private_fields_and_syntax_not_encoded_length',
                   sourceMaskPolicy='known_private_syntax_and_ambiguous_opaque_literals_with_proven_public_exemptions',
                   reviewStatus='hold_pending_independent_privacy_and_correctness_review',
                   labelBinding='original_packet_sha256_plus_display_view_path_and_sha256')
    source_cache = {}
    for row, old in zip(view['records'], original['records']):
        native_changed = False
        context_changed = False
        if original['kind'] == 'claim':
            safe = safe_finding(old['nativeFinding'])
            native_changed = safe != old['nativeFinding']
            row['nativeFinding'] = safe
            row['nativeFindingRedacted'] = bool(old.get('nativeFindingRedacted') or native_changed)
            row['assertionChecklist'] = assertion_checklist(safe)
            row['claimSummary'] = {key: safe[key] for key in ('rule_id','category','description','location','evidence') if key in safe}
            if native_changed:
                row['originalPreservation'] = 'private_original_packet_and_bound_report_only_display_view_redacted'
        if old.get('contextSafeText') is not None:
            ref = old if original['kind'] == 'source' else old['sourceReferences'][0]
            try:
                source_root = pathlib.Path(ref['sourceRoot'])
                relative = pathlib.PurePosixPath(ref['sourcePath'])
                if (not source_root.is_absolute() or source_root.resolve() != source_root
                        or relative.is_absolute() or any(part in {'','.', '..'} for part in ref['sourcePath'].split('/'))
                        or any(c in ref['sourcePath'] for c in ('\\', ':', '\x00'))):
                    raise ValueError('unsafe_or_nonregular_binding')
                cache_key = (str(source_root), str(relative), ref['sourceSha256'])
                if cache_key not in source_cache:
                    raw = read_bound(source_root / relative, ref['sourceSha256'], SOURCE_LIMIT)
                    if len(source_cache) >= 4:
                        source_cache.pop(next(iter(source_cache)))
                    source_cache[cache_key] = mask_secrets(raw.decode('utf-8'))
                safe_text, full_changed = source_cache[cache_key]
                lines = safe_text.splitlines(keepends=True)
                start, end = old['contextStart'], old['contextEnd']
                if type(start) is not int or type(end) is not int or not 1 <= start <= end <= len(lines):
                    raise ValueError('invalid_display_context')
                context = ''.join(lines[start-1:end])
                if len(context) != len(old['contextSafeText']):
                    raise ValueError('invalid_display_context')
                context = ''.join('█' if before == '█' else after for before,after in zip(old['contextSafeText'],context))
                if len(context.encode()) > CONTEXT_LIMIT:
                    raise ValueError('resource_limit')
                row['contextSafeText'] = context
                context_changed = context != old['contextSafeText']
                row['redacted'] = bool(old.get('redacted') or full_changed)
                if full_changed:
                    row['redactionPreventsVerification'] = 'review_required'
            except (ValueError, OSError, UnicodeError):
                row['contextSafeText'] = None
                row['contextStatus'] = 'display_withheld_unverified_source_binding'
                context_changed = True
        row['displayPrivacy'] = dict(addedNativeMask=native_changed, addedContextMask=context_changed,
                                     originalNativeFindingRedacted=bool(old.get('nativeFindingRedacted')),
                                     contextMaskDoesNotInvalidateUnassertedContext=True,
                                     nativeMaskReason='recognized_private_syntax_or_unresolved_encoded_structure' if native_changed else None)
    view['displayViewBinding'] = binding
    destination = root / 'view.json'
    write_new(destination, view)
    result = dict(path=str(destination), sha256=sha(read_bound(destination)),
                  originalPacket=binding['originalPacket'], records=len(view['records']),
                  addedNativeMasks=sum(r['displayPrivacy']['addedNativeMask'] for r in view['records']),
                  changedContexts=sum(r['displayPrivacy']['addedContextMask'] for r in view['records']),
                  reviewStatus=binding['reviewStatus'])
    write_new(root / 'receipt.json', result)
    return result


def display_excerpt(path, expected_sha, start=1, end=None):
    """Read a bound whole file, mask it, then select physical source lines."""
    raw = read_bound(path, expected_sha, SOURCE_LIMIT)
    safe, changed = mask_secrets(raw.decode('utf-8'))
    lines = safe.splitlines(keepends=True)
    end = len(lines) if end is None else end
    if type(start) is not int or type(end) is not int or not 1 <= start <= end <= len(lines):
        raise ValueError('invalid_display_context')
    excerpt = ''.join(lines[start-1:end])
    if len(excerpt.encode()) > CONTEXT_LIMIT:
        raise ValueError('display_resource_limit')
    return dict(binding=dict(path=str(pathlib.Path(path).absolute()), sha256=expected_sha),
                lineStart=start, lineEnd=end, safeText=excerpt, redacted=changed)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    for option in ('registry', 'source-selection', 'claim-selection', 'protocol', 'roots', 'output', 'study-id'):
        parser.add_argument('--' + option, required=True)
    parser.add_argument('--repo-root', default=str(pathlib.Path(__file__).resolve().parents[1]))
    parser.add_argument('--source-only', action='store_true')
    parser.add_argument('--comparison-claim-selection')
    parser.add_argument('--reference-registry')
    parser.add_argument('--assessment-interface')
    parser.add_argument('--claim-rubric-addendum')
    args = parser.parse_args(argv)
    result = build_packets(repo_root=args.repo_root, registry_path=args.registry,
                           source_selection_path=args.source_selection, claim_selection_path=args.claim_selection,
                           protocol_path=args.protocol, roots_path=args.roots, output=args.output,
                           study_id=args.study_id, include_claims=not args.source_only,
                           comparison_claim_selection_path=args.comparison_claim_selection,
                           reference_registry_path=args.reference_registry,
                           assessment_interface_path=args.assessment_interface,
                           claim_rubric_addendum_path=args.claim_rubric_addendum)
    print(json.dumps({k: v for k, v in result.items() if k != 'fileBindings'}, sort_keys=True))
    return 0


if __name__ == '__main__':
    sys.exit(main())
