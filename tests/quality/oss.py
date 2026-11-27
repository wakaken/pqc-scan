import datetime
import gzip
import json
import math
import os
import pathlib
import re
import shutil
import uuid
from dataclasses import dataclass

import external
import run
from evaluate import bytes_digest, digest, order_digest, safe_relative, semantic_digest
from task050.study import fingerprints

REPO = pathlib.Path(__file__).resolve().parents[2]
REQUIRED = ('report.json', 'report.html', 'report.md', 'report.sarif', 'cbom.json',
            'dependency-sbom.json', 'diagnostics.json', 'scan.inventory.v1.json',
            'cbom.cdx-1.7.json', 'dependency-sbom.cdx-1.7.json')
MAX_JSON = 128 * 1024 * 1024
MAX_FILES = 500000
MAX_INPUT_BYTES = 16 * 1024**3
LEGACY_ENUMERATION_ASSESSOR = 'b56b829c06208106d3bc0250d6a17ed13be13cd31dc1ce2d93b100026bc83900'


def _input_partial(limitations):
    return any(limitations.get(key) for key in ('unavailable', 'missing', 'unreadablePresent',
               'uninitializedSubmodules', 'gitlinks'))


def inspection_state(coverage, limitations=None):
    enumeration_failed = coverage.get('enumeration_failed_entries')
    if type(enumeration_failed) is not int or enumeration_failed < 0:
        raise ValueError('Invalid enumeration failure count')
    partial = _input_partial(limitations or {}) or enumeration_failed or any(
        coverage[k] for k in ('partial_files', 'analysis_failed_files', 'unknown_files',
                             'unsupported_files', 'read_failed_files'))
    return 'Partial' if partial else 'Complete'


SUBSETS = {'Sustainsys__Saml2', 'bitwarden__server'}


@dataclass(frozen=True)
class Selector:
    repositories: tuple = ()
    groups: tuple = ()
    population: str = None
    priority: bool = False
    all: bool = False


@dataclass(frozen=True)
class RunRequest:
    registry: pathlib.Path
    selector: Selector
    protocol: pathlib.Path
    phase: str
    binary: pathlib.Path
    rules: pathlib.Path
    build_receipt: pathlib.Path
    output: pathlib.Path
    resume: bool = False
    timeout: float = None
    min_free_gib: float = None
    profile: str = 'default'


def _json(path, limit=MAX_JSON):
    path = pathlib.Path(path)
    if path.is_symlink() or not path.is_file() or path.stat().st_size > limit:
        raise ValueError('JSON input is unavailable, symlinked or exceeds parsing limit')
    opener = gzip.open if path.suffix == '.gz' else open
    with opener(path, 'rb') as stream:
        raw = stream.read(limit + 1)
    if len(raw) > limit:
        raise ValueError('Expanded JSON exceeds parsing limit')
    try:
        return json.loads(raw, parse_constant=lambda _: (_ for _ in ()).throw(ValueError('Non-finite JSON')))
    except (RecursionError, UnicodeError) as error:
        raise ValueError('JSON nesting or encoding exceeds parser capability') from error


def _publish(path, value):
    path = pathlib.Path(path)
    temporary = path.with_name('.' + path.name + '.' + uuid.uuid4().hex)
    with temporary.open('x') as stream:
        json.dump(value, stream, indent=2, ensure_ascii=False, allow_nan=False)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())
    try:
        os.link(temporary, path)
    finally:
        temporary.unlink()


def _reference(path):
    path = pathlib.Path(path).resolve(strict=True)
    return dict(path=str(path), sha256=bytes_digest(path))


def _resolve(name):
    path = pathlib.Path(name)
    return path if path.is_absolute() else REPO / safe_relative(name)


def select_repositories(registry, selector):
    """Resolve explicit selectors, retaining the registry's frozen order."""
    data = _json(registry)
    if not isinstance(data, dict) or not isinstance(data.get('repositories'), list):
        raise ValueError('Invalid registry shape')
    rows = data['repositories']
    for row in rows:
        if (not isinstance(row, dict) or row.get('population') not in {'primary', 'comparison'}
                or not isinstance(row.get('group'), str) or not isinstance(row.get('scope'), str)
                or not re.fullmatch(r'[0-9a-f]{40}', row.get('commit', ''))):
            raise ValueError('Invalid registry identity, pin or scope')
    ids = [r['id'] for r in rows]
    if len(ids) != len(set(ids)) or any(not re.fullmatch(r'[A-Za-z0-9_.-]+', x) for x in ids):
        raise ValueError('Invalid or duplicate repository identity')
    if not any((selector.repositories, selector.groups, selector.population, selector.priority, selector.all)):
        raise ValueError('An explicit selector is required')
    if selector.all and any((selector.repositories, selector.groups, selector.population, selector.priority)):
        raise ValueError('--all cannot be combined with narrowing selectors')
    if set(selector.repositories) - set(ids):
        raise ValueError('Unknown repository selector')
    if set(selector.groups) - {r['group'] for r in rows}:
        raise ValueError('Unknown group selector')
    if selector.population not in {None, 'primary', 'comparison'}:
        raise ValueError('Unknown population selector')
    result = [r for r in rows if (not selector.repositories or r['id'] in selector.repositories)
              and (not selector.groups or r['group'] in selector.groups)
              and (not selector.population or r['population'] == selector.population)
              and (not selector.priority or r['priority'])]
    if not result:
        raise ValueError('Selection is empty')
    return result


class _OutputOverlap(ValueError):
    pass


def _check_output_roots(output, *records):
    if output is None:
        return
    for record in records:
        for key in ('root', 'inputRoot'):
            if record.get(key):
                root = pathlib.Path(record[key]).resolve()
                if output.is_relative_to(root) or root.is_relative_to(output):
                    raise _OutputOverlap('Evidence and input roots must be disjoint')


def _acquisition(row, output=None):
    candidates = row.get('historicalEvidence', [])
    record = dict(row.get('acquisition', {}))
    _check_output_roots(output, row, record, *candidates)
    if row.get('inputManifest') and row['id'] not in SUBSETS:
        manifest = row['inputManifest']
        record.update(manifest if isinstance(manifest, dict) else dict(path=manifest))
        record.setdefault('root', row.get('root'))
        record.setdefault('sha256', row.get('inputManifestSha256'))
    if not record and candidates:
        latest = candidates[-1]
        records = _json(_resolve(latest['record']))
        if isinstance(records, dict) and 'repositories' in records:
            records = records['repositories']
        if isinstance(records, list):
            matches = [r for r in records if r.get('id') == latest.get('id', row['id'])
                       or r.get('url', r.get('repository')) == row['repository']
                       or r.get('repository') == row['id'].replace('__', '/')]
            if len(matches) != 1:
                raise ValueError('Historical acquisition identity is unavailable or ambiguous')
            record = matches[0].copy()
        else:
            record = records.copy()
        record.update(latest)
        record['recordBinding'] = _reference(_resolve(latest['record']))
    if row.get('inputManifest') and row['id'] in SUBSETS:
        record.update(path=row['inputManifest'], sha256=row.get('inputManifestSha256'))
    _check_output_roots(output, record)
    if row.get('inputLimitations', {}).get('unreadablePresent'):
        raise ValueError('Explicit unreadable input prevents full population verification')
    if not record:
        raise ValueError('Missing acquisition manifest')
    manifest = record.get('path') or record.get('inventory') or record.get('inputBindings')
    expected = record.get('sha256') or record.get('inventorySha256') or record.get('inputBindingsSha256')
    if not manifest or not expected:
        raise ValueError('Acquisition manifest lacks its frozen digest')
    path = _resolve(manifest)
    if bytes_digest(path) != expected:
        raise ValueError('Acquisition manifest changed')
    content = _json(path)
    if isinstance(content, dict):
        if content.get('commit', row['commit']) != row['commit']:
            raise ValueError('Acquisition commit disagrees with registry')
        files = content['files']
        if isinstance(files, dict):
            files = [dict(value, path=name) for name, value in files.items()]
    else:
        files = content
    if not isinstance(files, list) or len(files) > MAX_FILES:
        raise ValueError('Input population exceeds manifest limit')
    if sum(f.get('bytes', 0) for f in files) > MAX_INPUT_BYTES:
        raise ValueError('Input bytes exceed hashing budget')
    if record.get('commit', row['commit']) != row['commit']:
        raise ValueError('Acquisition pin disagrees with registry')
    root = pathlib.Path(record.get('root') or record.get('inputRoot') or row['root']).resolve(strict=True)
    if row['id'] in SUBSETS:
        if not record.get('scopeSelection') or not record.get('originalInventorySha256'):
            raise ValueError('Licensed subset receipt is incomplete; full-root fallback is forbidden')
        if root != pathlib.Path(row['root']).resolve(strict=True):
            raise ValueError('Licensed subset root disagrees with registry')
    _validate_input(root, files)
    binding = dict(root=str(root), manifest=_reference(path), files=len(files),
                   bytes=sum(f.get('bytes', 0) for f in files), scope=row['scope'],
                   acquisitionDigest=digest(record), commit=row['commit'], limitations=row.get('inputLimitations', {}))
    return root, files, binding


def _validate_input(root, files):
    external.validate_tree(root, files)
    for item in files:
        if item.get('mode') in {'100644', '100755'}:
            executable = bool((root / item['path']).stat().st_mode & 0o111)
            if executable != (item['mode'] == '100755'):
                raise ValueError('Pinned executable mode changed')


def _rules(root):
    root = pathlib.Path(root).resolve(strict=True)
    result = {}
    for path in sorted(root.rglob('*')):
        if path.is_symlink():
            raise ValueError('Rules must not contain symlinks')
        if path.is_file():
            if path.stat().st_size > MAX_JSON:
                raise ValueError('Rules exceed hashing budget')
            result[path.relative_to(root).as_posix()] = bytes_digest(path)
    if not result:
        raise ValueError('Rules directory is empty')
    return result


def _configuration(request):
    protocol = _json(request.protocol)
    if not isinstance(protocol, dict):
        raise ValueError('Invalid protocol shape')
    if protocol.get('document'):
        if bytes_digest(_resolve(protocol['document'])) != protocol.get('documentSha256'):
            raise ValueError('Protocol document changed')
    collect_performance = protocol.get('collectPerformance', True)
    if type(collect_performance) is not bool:
        raise ValueError('collectPerformance must be a boolean')
    profile = protocol.get('profiles', {}).get(request.profile, {})
    if not isinstance(profile, dict) or set(profile) - {'threads', 'repeats'}:
        raise ValueError('Unsupported profile configuration')
    if protocol.get('profiles') and request.profile not in protocol['profiles']:
        raise ValueError('Unknown scanner profile')
    config = dict(threads=profile.get('threads', protocol.get('threads', 1)),
                  repeats=profile.get('repeats', protocol.get('repeats', 1)),
                  timeoutSeconds=request.timeout if request.timeout is not None else protocol.get('timeoutSeconds', 300),
                  minFreeGiB=request.min_free_gib if request.min_free_gib is not None else protocol.get('minFreeGiB', 2),
                  profile=request.profile, format='all', diagnostics=True, inventory=True, cyclonedx='1.7',
                  maxFileSizeMiB=protocol.get('limits', {}).get('maxFileSizeMiB', 2),
                  environment={key: os.environ.get(key) for key in ('LANG', 'LC_ALL', 'LC_CTYPE', 'TZ')},
                  verificationLimits=dict(maxJsonBytes=MAX_JSON, maxArtifactBytes=MAX_JSON,
                      maxAttemptArtifactBytes=2*MAX_JSON, maxInputFiles=MAX_FILES, maxInputBytes=MAX_INPUT_BYTES))
    if type(config['maxFileSizeMiB']) is not int or not 1 <= config['maxFileSizeMiB'] <= 64:
        raise ValueError('Invalid scanner file-size limit')
    for key in ('threads', 'repeats'):
        if type(config[key]) is not int or not 1 <= config[key] <= 256:
            raise ValueError('Threads/repeats must be bounded positive integers')
    for key in ('timeoutSeconds', 'minFreeGiB'):
        if not isinstance(config[key], (int, float)) or not math.isfinite(config[key]) or config[key] < 0:
            raise ValueError('Invalid resource configuration')
    if not 0 < config['timeoutSeconds'] <= 86400:
        raise ValueError('Wall timeout must be finite and at most one day')
    if protocol.get('limits', {}).get('maxArtifactBytes', MAX_JSON) != MAX_JSON:
        raise ValueError('Protocol artifact limit disagrees with this coordinator verification bound')
    if protocol.get('requireKernelConfinement'):
        raise ValueError('Strict process-tree confinement is unavailable')
    required = protocol.get('requiredArtifacts', list(REQUIRED))
    if not set(REQUIRED).issubset(required):
        raise ValueError('Protocol omits required output formats')
    for name in required:
        safe_relative(name)
    config['requiredArtifacts'] = required
    config['resourceCapabilities'] = dict(wall='Enforced by runner timeout', freeDisk='Observed before every attempt',
        cpu='Observed wait4 child plus waited descendants', rss='Observed child high-water; not process-tree aggregate',
        filesystem='Unavailable', network='Unavailable', diskQuota='Unavailable')
    if not collect_performance:
        config['collectPerformance'] = False
        config['resourceCapabilities'].update(cpu=run.PERFORMANCE_DISABLED, rss=run.PERFORMANCE_DISABLED)
    return protocol, config


def _scanner_binding(request):
    binary = pathlib.Path(request.binary).resolve(strict=True)
    rules = _rules(request.rules)
    receipt = _json(request.build_receipt)
    if not receipt.get('source') or not isinstance(receipt['source'], dict):
        raise ValueError('Build receipt must bind the scanner product source')
    if receipt.get('binarySha256') != bytes_digest(binary) or receipt.get('rulesSha256') != digest(rules):
        raise ValueError('Build receipt disagrees with binary or frozen rules')
    return dict(binary=_reference(binary), rules=rules, rulesSha256=digest(rules),
                buildReceipt=_reference(request.build_receipt), productSource=receipt['source'],
                features=receipt.get('features'), buildProfile=receipt.get('buildProfile'))


def _harness():
    paths = [pathlib.Path(__file__), REPO/'benchmark/entry.py']
    paths += [REPO/'tests/quality'/name for name in ('run.py', 'evaluate.py', 'external.py', 'task050/study.py')]
    return {str(p.relative_to(REPO)): bytes_digest(p) for p in paths}


def _file_bindings(directory):
    artifacts = {}
    total = 0
    for path in sorted(directory.rglob('*')):
        if path.is_symlink():
            raise ValueError('Output contains symlink')
        if path.is_file():
            size = path.stat().st_size
            total += size
            if size > MAX_JSON or total > 2 * MAX_JSON:
                raise ValueError('Output exceeds verification budget')
            artifacts[path.relative_to(directory).as_posix()] = dict(bytes=size, sha256=bytes_digest(path))
    return artifacts


def _artifacts(directory, required, limitations=None, historical_enumeration=False):
    artifacts = _file_bindings(directory)
    if not set(required).issubset(artifacts):
        raise ValueError('Required output is missing')
    report = _json(directory/'report.json')
    if not isinstance(report, dict) or not isinstance(report.get('findings'), list):
        raise ValueError('Invalid native report structure')
    diagnostics = _json(directory/'diagnostics.json')
    _json(directory/'scan.inventory.v1.json')
    outputs = {}
    for name in required:
        value = _json(directory/name) if name.endswith(('.json', '.sarif')) else (directory/name).read_text()
        outputs[name] = fingerprints(name, value)
    coverage = diagnostics.get('coverage', {})
    terminal = ('analyzed_files', 'partial_files', 'analysis_failed_files', 'unsupported_files',
                'not_applicable_files', 'unknown_files')
    if not coverage or any(type(coverage.get(k)) is not int or coverage[k] < 0
                           for k in (*terminal, 'selected_files', 'read_ok_files', 'read_failed_files')):
        raise ValueError('Missing or invalid inspection coverage')
    if coverage['selected_files'] != coverage['read_ok_files'] + coverage['read_failed_files'] or coverage['read_ok_files'] != sum(coverage[k] for k in terminal):
        raise ValueError('Inspection coverage does not reconcile')
    state_coverage = dict(coverage)
    if historical_enumeration:
        state_coverage.setdefault('enumeration_failed_entries', 0)
    state = inspection_state(state_coverage, limitations)
    if historical_enumeration:
        state = inspection_state(dict(state_coverage, enumeration_failed_entries=0), limitations)
    return artifacts, dict(semanticDigest=semantic_digest(report), orderDigest=order_digest(report),
        diagnosticDigest=digest({k: v for k, v in diagnostics.items() if k != 'generated_at'}),
        inspection=state, coverage=coverage, outputs=outputs, inputLimitations=limitations or {},
        findingCount=len(report['findings']), accuracy=None,
        accuracyReason='No independently frozen qualified annotation scope is admitted by this descriptive harness')


def _verify_performance(terminal, configuration):
    if configuration.get('collectPerformance', True):
        return
    measurement = terminal.get('measurement')
    if measurement is None and terminal.get('execution') != 'Succeeded':
        return
    if (not isinstance(measurement, dict)
            or any(measurement.get(key) is not None for key in
                   ('wallSeconds', 'cpuSeconds', 'peakRssBytes', 'rssNativeUnit', 'resourceScope'))
            or measurement.get('resourceMethod') != run.PERFORMANCE_DISABLED):
        raise ValueError('Performance data is forbidden when collection is disabled')


def _verify_terminal(path, expected_key=None):
    terminal = _json(path)
    if terminal.get('execution') != 'Succeeded' or expected_key is not None and terminal.get('executionKey') != expected_key:
        raise ValueError('Attempt is not reusable')
    started = _json(path.parent/'started.json')
    if (digest(started) != terminal['startedDigest'] or digest(started['bindings']) != terminal['executionKey']
            or started['repositoryId'] != started['bindings']['registryEntry']['id']
            or started['phase'] != started['bindings']['phase']):
        raise ValueError('Attempt start binding changed')
    _verify_performance(terminal, started['bindings']['configuration'])
    historical_enumeration = started['bindings']['harness'].get('tests/quality/oss.py') == LEGACY_ENUMERATION_ASSESSOR
    artifacts, assessment = _artifacts(path.parent/'output', started['bindings']['configuration']['requiredArtifacts'],
        started['bindings']['input'].get('limitations'), historical_enumeration=historical_enumeration)
    if artifacts != terminal['artifacts'] or assessment != terminal['assessment']:
        raise ValueError('Attempt artifacts or assessment changed')
    return terminal, started


def _attempt(request, row, root, files, bindings, base):
    key = digest(bindings)
    if request.resume:
        for candidate in sorted(base.glob('*/terminal.json')):
            try:
                terminal, _ = _verify_terminal(candidate, key)
                return dict(repositoryId=row['id'], sample=bindings['sample'], reused=True,
                            receipt=_reference(candidate), execution=terminal['execution'], assessment=terminal['assessment'])
            except (OSError, ValueError, KeyError, TypeError):
                continue
    directory = base/uuid.uuid4().hex
    directory.mkdir(parents=True, exist_ok=False)
    started = dict(schemaVersion=1, attemptId=directory.name, repositoryId=row['id'],
                   phase=request.phase, bindings=bindings,
                   startedUtc=datetime.datetime.now(datetime.timezone.utc).isoformat())
    _publish(directory/'started.json', started)
    terminal = dict(schemaVersion=1, executionKey=key, startedDigest=digest(started), execution='Failed',
                    assessment=None, artifacts={}, measurement=None)
    try:
        if shutil.disk_usage(directory).free < bindings['configuration']['minFreeGiB'] * 1024**3:
            raise ValueError('Insufficient free disk before scan')
        output = directory/'output'
        command = [str(request.binary.resolve()), 'scan', str(root), '--format', 'all', '--diagnostics',
                   '--inventory-json', '--cyclonedx-17', '--threads', str(bindings['configuration']['threads']),
                   '--max-file-size-mib', str(bindings['configuration']['maxFileSizeMiB']), '--lang', 'en',
                   '--rules-dir', str(request.rules.resolve()), '--out-dir', str(output)]
        if bindings['configuration'].get('collectPerformance', True):
            measurement = run.measure(command, output, bindings['configuration']['timeoutSeconds'])
        else:
            measurement = run.measure(command, output, bindings['configuration']['timeoutSeconds'], collect_performance=False)
        terminal['measurement'] = measurement
        terminal['artifacts'] = _file_bindings(output)
        if measurement['timedOut']:
            terminal['execution'] = 'TimedOut'
        elif measurement['exitCode'] == 0:
            terminal['artifacts'], terminal['assessment'] = _artifacts(output, bindings['configuration']['requiredArtifacts'], bindings['input'].get('limitations'))
            terminal['execution'] = 'Succeeded'
        _validate_input(root, files)
        if _reference(_resolve(bindings['input']['manifest']['path'])) != bindings['input']['manifest']:
            raise ValueError('Input manifest changed during scan')
        if _reference(request.protocol) != bindings['protocol'] or _configuration(request)[1] != bindings['configuration']:
            raise ValueError('Protocol or effective configuration changed during scan')
        if external.selection_bindings(root, cwd=run.REPO) != bindings['selection']:
            raise ValueError('Selection inputs changed during scan')
        if _scanner_binding(request) != bindings['scanner'] or _harness() != bindings['harness']:
            raise ValueError('Scanner or harness changed during scan')
    except (OSError, ValueError, KeyError, TypeError, RuntimeError) as error:
        terminal['execution'] = 'InvalidEvidence' if terminal['measurement'] else 'NotRun'
        terminal['assessment'] = None
        terminal['failure'] = dict(kind=type(error).__name__, reason='Admission, execution or artifact validation failed')
        if (directory/'output').exists() and not terminal['artifacts']:
            try:
                terminal['artifacts'] = _file_bindings(directory/'output')
            except (OSError, ValueError):
                terminal['artifactsUnverified'] = True
    _publish(directory/'terminal.json', terminal)
    return dict(repositoryId=row['id'], sample=bindings['sample'], reused=False, receipt=_reference(directory/'terminal.json'),
                execution=terminal['execution'], assessment=terminal['assessment'])


def run_selected(request):
    """Run a frozen expected matrix serially, or reuse fully verified observations."""
    if not re.fullmatch(r'[A-Za-z0-9_.-]+', request.phase):
        raise ValueError('Invalid phase')
    rows = select_repositories(request.registry, request.selector)
    protocol, config = _configuration(request)
    scanner = _scanner_binding(request)
    harness = _harness()
    if config.get('collectPerformance', True):
        coordinator = run.environment(pathlib.Path(request.binary).resolve())
    else:
        coordinator = run.environment(pathlib.Path(request.binary).resolve(), collect_performance=False)
    coordinator['provenanceRole'] = 'Coordinator checkout environment; product source is exclusively the build receipt'
    environment = {k: coordinator[k] for k in ('python', 'platform', 'machine', 'cpuCount', 'hardware', 'rust', 'cargo')}
    output = pathlib.Path(request.output).resolve()
    admissions = {}
    overlap = False
    for row in rows:
        try:
            admissions[row['id']] = _acquisition(row, output)
        except (OSError, ValueError, KeyError, TypeError) as error:
            admissions[row['id']] = error
            overlap = overlap or isinstance(error, _OutputOverlap)
    receipt = dict(schemaVersion=1, phase=request.phase, registry=_reference(request.registry),
        protocol=_reference(request.protocol), configuration=config, scanner=scanner, harness=harness, coordinatorEnvironment=coordinator,
        selectedIds=[r['id'] for r in rows], selectedEntries={r['id']: r for r in rows}, expectedSamples=config['repeats'], rows=[],
        accuracy=None, accuracyReason='Descriptive study; independent truth is not admitted',
        exposure='Every launched descriptive scan exposes scanner output; historical holdout labels do not establish independence')
    if not overlap:
        output.mkdir(parents=True, exist_ok=True)
        invocation_dir = output/'invocations'/uuid.uuid4().hex
        invocation_dir.mkdir(parents=True, exist_ok=False)
        _publish(invocation_dir/'started.json', {k: v for k, v in receipt.items() if k != 'rows'})
    for row in rows:
        try:
            if overlap:
                raise _OutputOverlap('Evidence and input roots must be disjoint')
            admission = admissions[row['id']]
            if isinstance(admission, Exception):
                raise admission
            root, files, acquisition = admission
            for sample in range(config['repeats']):
                _validate_input(root, files)
                bindings = dict(registryEntry=row, protocol=receipt['protocol'], harness=harness,
                    scanner=scanner, input=acquisition, selection=external.selection_bindings(root, cwd=run.REPO),
                    configuration=config, environment=environment, sample=sample, phase=request.phase, purpose='descriptive')
                result = _attempt(request, row, root, files, bindings, output/'runs'/row['id']/request.phase)
                receipt['rows'].append(dict(result, population=row['population'], split=row['split']))
        except (OSError, ValueError, KeyError, TypeError) as error:
            for sample in range(config['repeats']):
                if any(r['repositoryId'] == row['id'] and r['sample'] == sample for r in receipt['rows']):
                    continue
                receipt['rows'].append(dict(repositoryId=row['id'], population=row['population'], split=row['split'],
                    sample=sample, execution='NotRun', assessment=None,
                    reason=('Explicit unreadable input prevents population verification' if row.get('inputLimitations', {}).get('unreadablePresent') else 'Acquisition or input admission unavailable'),
                    inputLimitations=row.get('inputLimitations', {}), errorKind=type(error).__name__))
    receipt['executionComplete'] = (len(receipt['rows']) == len(rows)*config['repeats']
                                    and all(r['execution'] == 'Succeeded' for r in receipt['rows']))
    receipt['inspectionComplete'] = receipt['executionComplete'] and all(r['assessment']['inspection'] == 'Complete' for r in receipt['rows'])
    receipt['evaluationComplete'] = False
    receipt['unmetPredicates'] = ['Independent qualified truth, before/after comparison, rendered report acceptance and final source review remain external requirements']
    if overlap:
        receipt['rejectionReason'] = 'Evidence and all selected input roots must be disjoint; no evidence was written'
        return receipt
    destination = invocation_dir/'invocation.json'
    _publish(destination, receipt)
    return dict(receipt, receiptPath=str(destination))


def _verified_invocation(path):
    invocation = _json(path)
    started = _json(pathlib.Path(path).parent/'started.json')
    if any(invocation.get(key) != value for key, value in started.items()):
        raise ValueError('Invocation diverges from its frozen request')
    if (len(started['selectedIds']) != len(set(started['selectedIds']))
            or set(started['selectedEntries']) != set(started['selectedIds'])
            or any(entry['id'] != name for name, entry in started['selectedEntries'].items())
            or started['expectedSamples'] != started['configuration']['repeats']):
        raise ValueError('Invalid frozen invocation matrix')
    expected = {(name, sample) for name in started['selectedIds'] for sample in range(started['expectedSamples'])}
    actual = {(r['repositoryId'], r['sample']) for r in invocation['rows']}
    if actual != expected or len(invocation['rows']) != len(expected):
        raise ValueError('Invocation matrix is incomplete or duplicated')
    results = {}
    references = set()
    environment = {k: started['coordinatorEnvironment'][k]
                   for k in ('python', 'platform', 'machine', 'cpuCount', 'hardware', 'rust', 'cargo')}
    for row in invocation['rows']:
        key = row['repositoryId'], row['sample']
        expected_entry = started['selectedEntries'][row['repositoryId']]
        if any(row[k] != expected_entry[k] for k in ('population', 'split')):
            raise ValueError('Invocation population diverges from its frozen registry entry')
        reference = row.get('receipt')
        if not reference:
            if row['execution'] == 'Succeeded':
                raise ValueError('Successful invocation row lacks its attempt receipt')
            results[key] = (row, None, None)
            continue
        attempt_path = pathlib.Path(reference['path']).resolve(strict=True)
        if attempt_path in references:
            raise ValueError('Invocation reuses an attempt receipt within its expected matrix')
        references.add(attempt_path)
        if bytes_digest(attempt_path) != reference['sha256']:
            raise ValueError('Referenced attempt receipt changed')
        terminal = _json(attempt_path)
        attempt_started = _json(attempt_path.parent/'started.json')
        bindings = attempt_started['bindings']
        entry = bindings['registryEntry']
        if (digest(attempt_started) != terminal['startedDigest']
                or digest(bindings) != terminal['executionKey']
                or terminal['execution'] != row['execution']
                or attempt_started['repositoryId'] != row['repositoryId']
                or entry != expected_entry
                or entry['id'] != row['repositoryId'] or bindings['sample'] != row['sample']
                or attempt_started['phase'] != started['phase'] or bindings['phase'] != started['phase']
                or any(entry[k] != row[k] for k in ('population', 'split'))
                or any(bindings[k] != started[k] for k in ('configuration', 'scanner', 'protocol', 'harness'))
                or bindings['environment'] != environment):
            raise ValueError('Invocation row disagrees with its frozen attempt request')
        _verify_performance(terminal, bindings['configuration'])
        if row['execution'] != 'Succeeded':
            if terminal['artifacts'] != _file_bindings(attempt_path.parent/'output'):
                raise ValueError('Failed attempt artifacts changed')
            results[key] = (row, None, None)
            continue
        terminal, attempt_started = _verify_terminal(attempt_path)
        if row['assessment'] != terminal['assessment']:
            raise ValueError('Invocation projection disagrees with attempt')
        results[key] = row, terminal, attempt_started['bindings']
    return invocation, results


def compare_runs(before, after, out):
    """Compare verified matrices and compatible descriptive observations."""
    left, old = _verified_invocation(before)
    right, new = _verified_invocation(after)
    collect_performance = all(receipt['configuration'].get('collectPerformance', True) for receipt in (left, right))
    rows = []
    for key in sorted(old.keys() | new.keys()):
        previous, current = old.get(key), new.get(key)
        result = dict(repositoryId=key[0], sample=key[1], accuracy=None,
                      population=(current or previous)[0]['population'])
        if not previous or not current or previous[1] is None or current[1] is None:
            result.update(status='Unavailable', reason='Missing or unsuccessful paired sample')
        else:
            a, b = previous[2], current[2]
            compatible = all(a[k] == b[k] for k in ('registryEntry', 'input', 'selection', 'configuration', 'protocol', 'purpose', 'environment', 'harness'))
            compatible = compatible and all(a['scanner'].get(k) == b['scanner'].get(k) for k in ('features', 'buildProfile'))
            if not compatible:
                result.update(status='Incompatible', reason='Registry entry, input, scope, selection, protocol, configuration, environment, harness or build flags differ')
            else:
                x, y = previous[1]['assessment'], current[1]['assessment']
                result.update(status='Comparable', population=current[0]['population'],
                    semanticEqual=x['semanticDigest'] == y['semanticDigest'],
                    diagnosticEqual=x['diagnosticDigest'] == y['diagnosticDigest'],
                    orderEqual=x['orderDigest'] == y['orderDigest'],
                    beforeInspection=inspection_state(x['coverage'], x['inputLimitations']),
                    afterInspection=inspection_state(y['coverage'], y['inputLimitations']),
                    findingDelta=y['findingCount']-x['findingCount'],
                    outputComparisons={name: dict(semanticEqual=x['outputs'][name]['semantic'] == y['outputs'][name]['semantic'],
                        orderEqual=x['outputs'][name]['order'] == y['outputs'][name]['order']) for name in x['outputs']})
                result['beforeMeasurement'] = previous[1]['measurement']
                result['afterMeasurement'] = current[1]['measurement']
        rows.append(result)
    summary = dict(schemaVersion=1, before=_reference(before), after=_reference(after), rows=rows,
                   accuracy=None, evaluationComplete=False,
                   unmetPredicates=['Independent accuracy and browser acceptance are not established by descriptive comparison'])
    summary['populations'] = {}
    for population in ('primary', 'comparison'):
        paired = [r for r in rows if r.get('population') == population and r['status'] == 'Comparable']
        summary['populations'][population] = dict(comparableSamples=len(paired),
            unavailableSamples=sum(r['status'] != 'Comparable' and r['population'] == population for r in rows),
            accuracy=None)
    summary['repositories'] = {}
    for name in sorted({r['repositoryId'] for r in rows}):
        paired = [r for r in rows if r['repositoryId'] == name and r['status'] == 'Comparable']
        summary['repositories'][name] = dict(comparableSamples=len(paired),
            beforePerformance=run.distributions([r['beforeMeasurement'] for r in paired]) if paired and collect_performance else None,
            afterPerformance=run.distributions([r['afterMeasurement'] for r in paired]) if paired and collect_performance else None,
            performanceInterpretation=('Descriptive distributions only; protocol speed comparison sample requirements are not established'
                if collect_performance else 'Performance collection disabled; performance evaluation is deferred'), accuracy=None)
    _publish(pathlib.Path(out), summary)
    return summary
