"""Admit frozen invocation metadata and artifact bytes before legacy parsing."""
import hashlib
import json
import os
import pathlib
import re
import stat

MAX_METADATA = 64 * 1024 * 1024
MAX_ARTIFACT = 128 * 1024 * 1024
MAX_TOTAL = 2 * MAX_ARTIFACT
ARTIFACTS = frozenset(('report.json', 'report.html', 'report.rules.html', 'report.md', 'report.sarif',
                      'cbom.json', 'dependency-sbom.json', 'diagnostics.json', 'scan.inventory.v1.json',
                      'cbom.cdx-1.7.json', 'dependency-sbom.cdx-1.7.json', 'stdout.log', 'stderr.log'))


class Rejected(ValueError):
    pass


def require(ok, code):
    if not ok:
        raise Rejected(code)


def canonical_digest(value):
    raw = json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(',', ':'), allow_nan=False).encode()
    return hashlib.sha256(raw).hexdigest()


def _unique(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, 'E_PREFLIGHT_DUPLICATE_KEY')
        result[key] = value
    return result


def decode(raw):
    try:
        value = json.loads(raw, object_pairs_hook=_unique,
                           parse_constant=lambda _: (_ for _ in ()).throw(Rejected('E_PREFLIGHT_JSON')))
    except (ValueError, UnicodeError, RecursionError):
        raise Rejected('E_PREFLIGHT_JSON') from None
    require(isinstance(value, dict), 'E_PREFLIGHT_OBJECT')
    return value


def _open(path, directory=False):
    original = os.fspath(path)
    path = pathlib.Path(path)
    require(original == str(path) and path.parts[0] == '/', 'E_PREFLIGHT_PATH')
    require(path.is_absolute() and all(part not in ('.', '..') for part in path.parts), 'E_PREFLIGHT_PATH')
    require('\\' not in str(path), 'E_PREFLIGHT_PATH')
    descriptor = os.open('/', os.O_RDONLY | os.O_DIRECTORY)
    try:
        for index, part in enumerate(path.parts[1:]):
            is_directory = index < len(path.parts) - 2 or directory
            flags = os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK
            if is_directory:
                flags |= os.O_DIRECTORY
            child = os.open(part, flags, dir_fd=descriptor)
            os.close(descriptor)
            descriptor = child
        info = os.fstat(descriptor)
        require(stat.S_ISDIR(info.st_mode) if directory else stat.S_ISREG(info.st_mode), 'E_PREFLIGHT_TYPE')
        return descriptor
    except (OSError, Rejected):
        os.close(descriptor)
        raise Rejected('E_PREFLIGHT_PATH_TYPE') from None


def checked_file(path, expected=None, limit=MAX_METADATA, collect=False):
    if expected is not None:
        require(isinstance(expected, dict) and {'sha256'} <= expected.keys() <= {'path', 'sha256', 'bytes'},
                'E_PREFLIGHT_BINDING')
        require(isinstance(expected['sha256'], str) and re.fullmatch('[0-9a-f]{64}', expected['sha256']),
                'E_PREFLIGHT_DIGEST')
        require('bytes' not in expected or type(expected['bytes']) is int and 0 <= expected['bytes'] <= limit,
                'E_PREFLIGHT_SIZE')
    descriptor = _open(path)
    try:
        before = os.fstat(descriptor)
        require(before.st_size <= limit and (expected is None or 'bytes' not in expected or
                expected['bytes'] == before.st_size), 'E_PREFLIGHT_SIZE')
        digest = hashlib.sha256()
        chunks = []
        total = 0
        while True:
            block = os.read(descriptor, min(1024 * 1024, limit - total + 1))
            if not block:
                break
            total += len(block)
            require(total <= limit, 'E_PREFLIGHT_SIZE')
            digest.update(block)
            if collect:
                chunks.append(block)
        after = os.fstat(descriptor)
        require(total == before.st_size == after.st_size and before.st_mtime_ns == after.st_mtime_ns and
                before.st_ctime_ns == after.st_ctime_ns, 'E_PREFLIGHT_CHANGED')
        actual = dict(path=str(path), bytes=total, sha256=digest.hexdigest())
        require(expected is None or actual['sha256'] == expected['sha256'], 'E_PREFLIGHT_HASH')
        return b''.join(chunks) if collect else actual
    finally:
        os.close(descriptor)


def bound_json(binding):
    require(isinstance(binding, dict) and 'path' in binding, 'E_PREFLIGHT_BINDING')
    return decode(checked_file(binding['path'], binding, collect=True))


def _metadata(path, expected_digest=None):
    value = decode(checked_file(path, collect=True))
    require(expected_digest is None or canonical_digest(value) == expected_digest, 'E_PREFLIGHT_METADATA_DIGEST')
    return value


def _relocated(ref, aliases):
    matches = [item for item in aliases if item.get('original') == ref['path'] and item.get('sha256') == ref['sha256']]
    require(len(matches) <= 1, 'E_PREFLIGHT_ALIAS')
    return dict(ref, path=matches[0]['preserved']) if matches else ref


def preflight_invocation(binding, required_artifacts=None, binary_aliases=()):
    """Return admitted metadata without parsing any output artifact body."""
    wrapper = bound_json(binding)
    require(isinstance(wrapper['receiptPath'], str) and wrapper['receiptPath'] == str(pathlib.Path(wrapper['receiptPath'])), 'E_PREFLIGHT_PATH')
    actual_path = pathlib.Path(wrapper['receiptPath'])
    actual = _metadata(actual_path)
    require(wrapper == dict(actual, receiptPath=str(actual_path)), 'E_PREFLIGHT_WRAPPER')
    started = _metadata(actual_path.parent / 'started.json')
    require(all(actual.get(key) == value for key, value in started.items()), 'E_PREFLIGHT_REQUEST')
    ids = started['selectedIds']
    entries = started['selectedEntries']
    samples = started['expectedSamples']
    require(isinstance(ids, list) and all(isinstance(item, str) for item in ids) and len(ids) == len(set(ids)) and
            isinstance(entries, dict) and set(entries) == set(ids) and
            type(samples) is int and 0 < samples <= 1000 and samples == started['configuration']['repeats'],
            'E_PREFLIGHT_MATRIX')
    environment = {key: started['coordinatorEnvironment'][key] for key in ('python', 'platform', 'machine', 'cpuCount', 'hardware', 'rust', 'cargo')}
    rows = actual['rows']
    require(isinstance(rows, list) and len(rows) == len(ids) * samples, 'E_PREFLIGHT_MATRIX')
    expected = {(identity, sample) for identity in ids for sample in range(samples)}
    seen = set()
    terminals = {}
    for row in rows:
        key = row['repositoryId'], row['sample']
        require(type(row['sample']) is int and key in expected and key not in seen and row['execution'] == 'Succeeded',
                'E_PREFLIGHT_MATRIX')
        seen.add(key)
        terminal_ref = row['receipt']
        terminal = bound_json(terminal_ref)
        attempt = pathlib.Path(terminal_ref['path']).parent
        require(pathlib.Path(terminal_ref['path']).name == 'terminal.json' and str(attempt) not in terminals,
                'E_PREFLIGHT_ATTEMPT')
        attempt_started = _metadata(attempt / 'started.json', terminal['startedDigest'])
        bindings = attempt_started['bindings']
        require(canonical_digest(bindings) == terminal['executionKey'] and
                terminal['execution'] == row['execution'] and attempt_started['repositoryId'] == key[0] and
                bindings['registryEntry'] == entries[key[0]] and entries[key[0]]['id'] == key[0] and
                bindings['sample'] == key[1] and attempt_started['phase'] == started['phase'] == bindings['phase'] and
                all(bindings[item] == started[item] for item in ('configuration', 'scanner', 'protocol', 'harness')) and
                bindings['environment'] == environment and row['assessment'] == terminal['assessment'],
                'E_PREFLIGHT_ATTEMPT_BINDING')
        entry = entries[key[0]]
        require(all(row[item] == entry[item] for item in ('population', 'split')) and
                bindings['input']['commit'] == entry['commit'] and
                bindings['input']['manifest']['sha256'] == entry['inputManifestSha256'], 'E_PREFLIGHT_SOURCE')
        manifest = dict(path=entry['inputManifest'], sha256=entry['inputManifestSha256'])
        checked_file(manifest['path'], manifest)
        checked_file(bindings['input']['manifest']['path'], bindings['input']['manifest'])
        terminals[str(attempt)] = (row, terminal, bindings)
    require(seen == expected, 'E_PREFLIGHT_MATRIX')
    scanner = actual['scanner']
    require(canonical_digest(scanner['rules']) == scanner['rulesSha256'], 'E_PREFLIGHT_RULES')
    binary = _relocated(scanner['binary'], binary_aliases)
    checked_file(binary['path'], binary, limit=MAX_ARTIFACT)
    if 'buildReceipt' in scanner:
        build = bound_json(_relocated(scanner['buildReceipt'], binary_aliases))
        require(build['binarySha256'] == scanner['binary']['sha256'] and build['rulesSha256'] == scanner['rulesSha256'] and
                build['source'] == scanner['productSource'], 'E_PREFLIGHT_BUILD')
    for attempt_name, (_, terminal, bindings) in terminals.items():
        artifacts = terminal['artifacts']
        required = bindings['configuration']['requiredArtifacts']
        require(isinstance(required, list) and len(required) == len(set(required)) and set(required) <= ARTIFACTS and
                isinstance(artifacts, dict) and set(required) <= set(artifacts) <= ARTIFACTS, 'E_PREFLIGHT_ARTIFACT_NAMES')
        if required_artifacts is not None:
            require(set(required_artifacts) | {'stdout.log', 'stderr.log'} <= set(artifacts), 'E_PREFLIGHT_REQUIRED_ARTIFACTS')
        output = pathlib.Path(attempt_name) / 'output'
        descriptor = _open(output, directory=True)
        try:
            require(set(os.listdir(descriptor)) == set(artifacts), 'E_PREFLIGHT_ARTIFACT_NAMES')
        finally:
            os.close(descriptor)
        total = 0
        for name, ref in artifacts.items():
            require(set(ref) == {'bytes', 'sha256'}, 'E_PREFLIGHT_ARTIFACT_BINDING')
            total += checked_file(output / name, ref, limit=MAX_ARTIFACT)['bytes']
            require(total <= MAX_TOTAL, 'E_PREFLIGHT_TOTAL_SIZE')
    return dict(wrapper=wrapper, path=actual_path, invocation=actual, started=started, attempts=terminals)
