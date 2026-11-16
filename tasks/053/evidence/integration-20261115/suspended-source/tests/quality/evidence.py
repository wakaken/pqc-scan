"""Offline, source-bound developer evidence aggregation; commands are data only."""
import collections
import hashlib
import html
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import subprocess

STATES = {'Verified', 'Passed', 'Failed', 'Partial', 'Unsupported', 'Unknown',
          'Not Tested', 'Not Applicable'}
MAX_FILE = 64 * 1024**2


def binding_digest(bindings):
    return hashlib.sha256(json.dumps(bindings, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def relative(name):
    if not isinstance(name, str) or not name or name != PurePosixPath(name).as_posix():
        raise ValueError('Noncanonical evidence path')
    p = PurePosixPath(name)
    if p.is_absolute() or any(x in {'.', '..'} for x in p.parts) or '\\' in name or ':' in name or any(ord(x) < 32 for x in name):
        raise ValueError('Unsafe evidence path')
    return p


def read_bytes(repo, name):
    """Hold no-follow directory descriptors; never read links or special files."""
    parts = relative(name).parts
    fd = os.open(repo, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        for part in parts[:-1]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd)
            fd = child
        file_fd = os.open(parts[-1], os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=fd)
        with os.fdopen(file_fd, 'rb') as stream:
            info = os.fstat(stream.fileno())
            if not stat.S_ISREG(info.st_mode) or info.st_size > MAX_FILE:
                raise ValueError('Evidence must be a bounded regular file')
            data = stream.read(MAX_FILE + 1)
            if len(data) > MAX_FILE:
                raise ValueError('Evidence grew beyond limit')
            return data
    finally:
        os.close(fd)


def parse_json(data):
    def unique(pairs):
        value = {}
        for key, item in pairs:
            if key in value:
                raise ValueError('Duplicate JSON key')
            value[key] = item
        return value
    def invalid(value):
        raise ValueError('Nonfinite JSON number')
    return json.loads(data, object_pairs_hook=unique, parse_constant=invalid)


def checked(repo, reference):
    data = read_bytes(repo, reference['path'])
    if not re.fullmatch('[0-9a-f]{64}', reference.get('sha256', '')) or hashlib.sha256(data).hexdigest() != reference['sha256']:
        raise ValueError('Evidence hash mismatch')
    return data


def requirements(data):
    rows = {}
    for line in data.decode().splitlines():
        match = re.match(r'^\| (Q[1-6]-\d{2})\b', line)
        if not match:
            continue
        owner = re.search(r'\[(\d{3})\]\([^)]*tasks/\1/task\.yaml\)', line)
        if not owner or match[1] in rows:
            raise ValueError('Missing/duplicate requirement owner')
        rows[match[1]] = owner[1]
    if not rows or {key[:2] for key in rows} != {f'Q{i}' for i in range(1, 7)}:
        raise ValueError('Requirements must explicitly cover Q1-Q6')
    return rows


def pointer(data, path):
    if not isinstance(path, str) or (path and not path.startswith('/')):
        raise ValueError('Invalid JSON pointer')
    value = data
    try:
        for part in path.split('/')[1:] if path else []:
            if re.search(r'~(?![01])', part):
                raise ValueError('Invalid JSON pointer escape')
            key = part.replace('~1', '/').replace('~0', '~')
            if isinstance(value, list):
                if not re.fullmatch(r'0|[1-9]\d*', key):
                    raise ValueError('Invalid JSON array index')
                value = value[int(key)]
            else:
                value = value[key]
    except (KeyError, TypeError, IndexError) as error:
        raise ValueError('JSON observation pointer not found') from error
    if len(json.dumps(value, allow_nan=False)) > 65536:
        raise ValueError('Observation exceeds output limit')
    return value


def applicability(repo, bindings):
    if not isinstance(bindings, dict) or len(bindings) > 10000:
        raise ValueError('Invalid source bindings')
    matched, mismatched = [], []
    for name, expected in bindings.items():
        relative(name)
        if not isinstance(expected, str) or not re.fullmatch('[0-9a-f]{64}', expected):
            raise ValueError('Invalid measured source hash')
        try:
            same = hashlib.sha256(read_bytes(repo, name)).hexdigest() == expected
        except (FileNotFoundError, NotADirectoryError):
            same = False
        (matched if same else mismatched).append(name)
    return dict(status=('Unknown' if not bindings else 'Partial' if mismatched else 'Verified'),
                matched=matched, mismatched=mismatched,
                reason=('No measured source bindings supplied' if not bindings else
                        'Measured source differs from current checkout' if mismatched else
                        'All declared measured source bytes match; scope is limited to that declared population'))


def test_evidence(record, artifacts, source):
    reviews = record['reviews']
    if not isinstance(reviews, list) or len(reviews) > 2:
        raise ValueError('Invalid review stages')
    stages = set()
    for review in reviews:
        if review.get('stage') not in {'implementation', 'test'} or review['stage'] in stages or review.get('artifact') not in artifacts:
            raise ValueError('Invalid or unbound review reference')
        stages.add(review['stage'])
    complete = (stages == {'implementation', 'test'}
                and len({r.get('reviewer') for r in reviews}) == 2
                and all(isinstance(r.get('reviewer'), str) and r['reviewer'].strip()
                        and r.get('status') == 'accepted'
                        and r.get('sourceDigest') == binding_digest(record['sourceBindings']) for r in reviews))
    current = source['status'] == 'Verified'
    adequate = 'Verified' if complete and current else 'Partial' if reviews else 'Not Tested'
    return dict(record=record['id'], executionStatus=record['measurement']['executionStatus'],
                commands=record['measurement']['commands'], reviews=reviews,
                sourceApplicability=source['status'], adequacyStatus=adequate,
                reason=('Two source-bound stages accepted for the declared matching source population' if adequate == 'Verified'
                        else 'Incomplete, stale or source-mismatched review evidence; command success alone is insufficient'))


def build_index(catalog_path, repo):
    repo_argument = Path(repo).absolute()
    repo = repo_argument.resolve()
    catalog_path = Path(catalog_path).absolute()
    try:
        name = catalog_path.relative_to(repo_argument).as_posix()
    except ValueError as error:
        raise ValueError('Catalog must be inside the checkout') from error
    catalog_bytes = read_bytes(repo, name)
    catalog = parse_json(catalog_bytes)
    if catalog.get('version') != 1 or not isinstance(catalog.get('records'), list) or len(catalog['records']) > 256:
        raise ValueError('Invalid developer evidence catalog')
    owners = requirements(checked(repo, catalog['requirements']))
    records, tests, ids = [], [], set()
    for entry in catalog['records']:
        record = parse_json(json.dumps(entry, allow_nan=False))
        identifier = record['id']
        if not isinstance(identifier, str) or not re.fullmatch('[a-zA-Z0-9_-]{1,100}', identifier) or identifier in ids:
            raise ValueError('Invalid/duplicate record id')
        ids.add(identifier)
        selected = record['requirements']
        if not selected or len(set(selected)) != len(selected) or any(owners.get(key) != record['task'] for key in selected):
            raise ValueError('Unknown or reassigned requirement')
        if record['result'].get('status') not in STATES or not record['result'].get('summary'):
            raise ValueError('Missing result status/scope')
        measurement = record['measurement']
        for key in ['head', 'version', 'features', 'standards', 'method', 'environment', 'date', 'comparison']:
            if not measurement.get(key):
                raise ValueError('Missing measurement context: ' + key)
        if measurement.get('executionStatus') not in STATES or not isinstance(measurement.get('commands'), list) or not measurement['commands']:
            raise ValueError('Missing execution evidence')
        for command in measurement['commands']:
            if not isinstance(command.get('argv'), list) or not command['argv'] or any(not isinstance(arg, str) for arg in command['argv']):
                raise ValueError('Invalid recorded command')
            exit_code = command.get('exitCode')
            if (exit_code is not None and type(exit_code) is not int) or (exit_code is None and not command.get('reason')):
                raise ValueError('Unknown execution needs a reason')
        if not isinstance(record.get('artifacts'), list) or not record['artifacts'] or len(record['artifacts']) > 128:
            raise ValueError('Missing/bounded artifacts required')
        artifacts = {}
        for artifact in record['artifacts']:
            if artifact['path'] in artifacts or not artifact.get('purpose'):
                raise ValueError('Duplicate/missing artifact purpose')
            artifacts[artifact['path']] = checked(repo, artifact)
        observations = []
        for observation in record.get('observations', []):
            if observation.get('artifact') not in artifacts or not observation.get('label'):
                raise ValueError('Observation must name a bound artifact')
            value = pointer(parse_json(artifacts[observation['artifact']]), observation['pointer'])
            observations.append(dict(observation, value=value))
        if not isinstance(record['limitations'], list) or not record['limitations'] or not isinstance(record['concerns'], list):
            raise ValueError('Explicit limitations/concerns required')
        source = applicability(repo, record['sourceBindings'])
        test = test_evidence(record, artifacts, source)
        records.append(dict(record, sourceApplicability=source, observations=observations))
        tests.append(test)
    rows = []
    for key, owner in owners.items():
        found = [r for r in records if key in r['requirements']]
        current = [r for r in found if r['sourceApplicability']['status'] == 'Verified']
        if not found:
            status, reason = 'Not Tested', 'No registered evidence for this requirement; upstream work remains unfinished/unavailable'
        elif not current:
            status, reason = 'Partial', 'Only historical or unbound-source evidence is available; original outcomes are retained'
        else:
            states = {r['result']['status'] for r in current}
            status = ('Failed' if 'Failed' in states else 'Unknown' if states == {'Unknown'}
                      else next(iter(states)) if len(states) == 1 else 'Partial')
            if status in {'Passed', 'Verified'} and any(t['adequacyStatus'] != 'Verified' for t in tests if t['record'] in {r['id'] for r in current}):
                status = 'Partial'
            reason = 'Source-scoped observations; command execution, result and review adequacy remain separate'
        rows.append(dict(id=key, ownerTask=owner, status=status, reason=reason, records=[r['id'] for r in found]))
    axes = {f'Q{i}': dict(status='Partial' if any(r['records'] for r in rows if r['id'].startswith(f'Q{i}-')) else 'Not Tested',
                          requirements=[r['id'] for r in rows if r['id'].startswith(f'Q{i}-')]) for i in range(1, 7)}
    try:
        head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=repo, stderr=subprocess.DEVNULL, timeout=10, text=True).strip()
    except (OSError, subprocess.SubprocessError):
        head = 'Unknown'
    return dict(model='PQS-QAM-evidence/1', infrastructureStatus='Verified',
                target=dict(head=head, scope='Current checkout plus separately identified historical/remote measurements'),
                catalog=dict(path=name, sha256=hashlib.sha256(catalog_bytes).hexdigest()),
                requirementsSource=catalog['requirements'], releaseDecision='Unknown', axes=axes,
                requirements=rows, records=records, testEvidence=tests,
                limitations=['This integration does not complete pending upstream tasks or grant release acceptance.',
                             'Matching declared source bindings does not expand the original measured population.',
                             'Historical measurements, tool failures and product findings are distinct.'])


def text(value):
    return html.escape(str(value), quote=False).replace('|', '\\|').replace('`', '&#96;').replace('\n', ' ')


def write_index(result, output, *, existing=False):
    output = Path(output)
    if existing:
        if any((output/name).exists() for name in ['quality-evidence-index.json', 'test-evidence-index.json', 'quality-assessment.md']):
            raise ValueError('Integrated evidence output already exists')
    else:
        output.mkdir(parents=True, exist_ok=False)
    (output/'quality-evidence-index.json').write_text(json.dumps(result, indent=2, ensure_ascii=False, allow_nan=False)+'\n')
    (output/'test-evidence-index.json').write_text(json.dumps(dict(model=result['model'], catalog=result['catalog'],
        records=result['testEvidence']), indent=2, ensure_ascii=False, allow_nan=False)+'\n')
    rows = ['# Integrated quality assessment', '',
            f"{len(result['requirements'])} requirements accounted for; {len(result['records'])} evidence records. Release decision: **{result['releaseDecision']}**.", '',
            '| Requirement | Status | Owner task | Evidence records | Reason |', '|---|---|---|---|---|']
    rows += ['| '+' | '.join([text(r['id']), text(r['status']), text(r['ownerTask']), text(', '.join(r['records'])), text(r['reason'])])+' |' for r in result['requirements']]
    rows += ['', '## Measurement details and concerns', '']
    for r in result['records']:
        m = r['measurement']
        rows += [f"### {text(r['id'])}", '', f"**{text(r['result']['status'])}**: {text(r['result']['summary'])}",
                 f"Source applicability: **{r['sourceApplicability']['status']}**; head `{text(m['head'])}`; version `{text(m['version'])}`; date {text(m['date'])}.",
                 f"Features: {text(m['features'])}. Standards: {text(m['standards'])}.",
                 f"Method: {text(m['method'])}. Environment: {text(m['environment'])}.",
                 f"Comparison: {text(m['comparison'])}.", '']
        rows += [f"- {text(o['label'])}: `{text(json.dumps(o['value'], ensure_ascii=False, allow_nan=False))}`" for o in r['observations']]
        rows += [f"- Concern ({text(c['priority'])}): {text(c['text'])}" for c in r['concerns']]
        rows += [f"- Limitation: {text(item)}" for item in r['limitations']]
        rows += [f"- Artifact: `{text(a['path'])}`; SHA-256 `{a['sha256']}`; {text(a['purpose'])}" for a in r['artifacts']]
        rows += [f"- Command: `{text(json.dumps(c['argv'], ensure_ascii=False))}`; exit `{text(c['exitCode'])}`" + (f"; {text(c['reason'])}" if c.get('reason') else '') for c in m['commands']]
        test = next(t for t in result['testEvidence'] if t['record'] == r['id'])
        rows += [f"- Execution: {test['executionStatus']}; review adequacy: {test['adequacyStatus']}; {text(test['reason'])}", '']
    rows += ['## Scope and acceptance', '', f"Checkout head: `{result['target']['head']}`. Catalog SHA-256: `{result['catalog']['sha256']}`.", '', *['- '+x for x in result['limitations']], '']
    (output/'quality-assessment.md').write_text('\n'.join(rows))
