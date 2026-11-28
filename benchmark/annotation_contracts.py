"""Check supplied source-reviewed annotation contracts before interval scoring.

This checker does not discover anchors, infer labels, parse languages, run a
scanner, or establish independent truth. Contracts must cover every originally
positive/negative unit, including controls. The bound audit's ``rows`` preserve
each unit as ``frozenUnit``, ``originalLabel`` and ``sourceFactPreserved``.
Each contract supplies a supportedInput boolean, a 1-based Unicode codepoint
expectedAnchorColumn, and references to bound routing/anchor source documents.
Only known source lines (> 0) are supported. Existing line-zero dependency
matching and the scorers are outside this tool's scope.

CLI exits 0 for compatible contracts, 1 for not-evaluable units, and 2 for invalid
evidence or an existing output. It exclusively creates a new JSON receipt and
never edits source, intervals, labels, original admission records or scores.
"""

import argparse
import hashlib
import json
import pathlib
import re
import sys


LABELS = {'positive', 'negative', 'unsupported', 'ambiguous'}
ELIGIBLE = {'positive', 'negative'}
MAX_BYTES = 16 * 1024 * 1024


def _require(condition, message):
    if not condition:
        raise ValueError(message)


def _text(value):
    return isinstance(value, str) and bool(value.strip())


def _object(pairs):
    result = {}
    for key, value in pairs:
        _require(key not in result, 'Duplicate JSON key')
        result[key] = value
    return result


def _load(path):
    with pathlib.Path(path).open('rb') as stream:
        data = stream.read(MAX_BYTES + 1)
    _require(len(data) <= MAX_BYTES, 'JSON exceeds size limit')
    result = json.loads(data, object_pairs_hook=_object)
    _require(isinstance(result, dict), 'Expected JSON object')
    return result


def _sha(path):
    _require(path.stat().st_size <= MAX_BYTES, 'Binding exceeds size limit')
    result = hashlib.sha256()
    with path.open('rb') as stream:
        size = 0
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            size += len(block)
            _require(size <= MAX_BYTES, 'Binding exceeds size limit')
            result.update(block)
    return result.hexdigest()


def _binding(value, expected=None):
    _require(isinstance(value, dict) and set(value) == {'path', 'sha256'}, 'Invalid binding schema')
    _require(_text(value['path']) and isinstance(value['sha256'], str)
             and re.fullmatch('[0-9a-f]{64}', value['sha256']), 'Invalid binding values')
    path = pathlib.Path(value['path'])
    _require(path.is_absolute() and path.resolve() == path and path.is_file(),
             'Binding is unavailable or traverses a symlink')
    _require(expected is None or path == pathlib.Path(expected).resolve(), 'Binding path mismatch')
    _require(_sha(path) == value['sha256'], 'Binding hash drift')
    return path


def _index(rows, key):
    _require(isinstance(rows, list), 'Expected record list')
    result = {}
    for row in rows:
        _require(isinstance(row, dict) and _text(row.get(key)), 'Invalid record identity')
        _require(row[key] not in result, 'Duplicate record identity')
        result[row[key]] = row
    return result


def _relative(value):
    _require(_text(value) and '\\' not in value and ':' not in value, 'Invalid relative path')
    path = pathlib.PurePosixPath(value)
    _require(not path.is_absolute() and all(p not in {'', '.', '..'} for p in value.split('/')),
             'Unsafe relative path')
    return pathlib.Path(*path.parts)


def _source_lines(file):
    path = _binding({'path': file['source'], 'sha256': file['sha256']})
    if 'root' in file or 'sourceRelativePath' in file:
        _require('root' in file and 'sourceRelativePath' in file, 'Incomplete source root binding')
        root = pathlib.Path(file['root'])
        _require(root.is_absolute() and root.resolve() == root
                 and root / _relative(file['sourceRelativePath']) == path,
                 'Source escapes bound root')
    with path.open('rb') as stream:
        data = stream.read(MAX_BYTES + 1)
    _require(len(data) <= MAX_BYTES, 'Source exceeds size limit')
    _require(hashlib.sha256(data).hexdigest() == file['sha256'], 'Source changed while reading')
    return [line.removesuffix('\r') for line in data.decode('utf-8').split('\n')]


def check(corpus_path, truth_path, contracts_path):
    sidecar = _load(contracts_path)
    _require(set(sidecar) == {'version', 'corpus', 'truth', 'checker', 'audit',
                             'sourceContractBindings', 'exposure', 'units'}, 'Invalid contract schema')
    _require(type(sidecar['version']) is int and sidecar['version'] == 1, 'Unsupported contracts version')
    _binding(sidecar['corpus'], corpus_path)
    _binding(sidecar['truth'], truth_path)
    _binding(sidecar['checker'], pathlib.Path(__file__))
    audit_path = _binding(sidecar['audit'])
    exposure = sidecar['exposure']
    _require(isinstance(exposure, dict) and set(exposure) ==
             {'predictionsSeen', 'sameModelFamily', 'limitations'}
             and type(exposure['predictionsSeen']) is bool
             and type(exposure['sameModelFamily']) is bool and _text(exposure['limitations']),
             'Missing exposure limitations')
    bindings = sidecar['sourceContractBindings']
    _require(isinstance(bindings, list) and bool(bindings), 'Missing source contract bindings')
    bound_paths = [str(_binding(value)) for value in bindings]
    _require(len(bound_paths) == len(set(bound_paths)), 'Duplicate source contract binding')

    corpus, truth, audit = _load(corpus_path), _load(truth_path), _load(audit_path)
    _require(type(corpus.get('version')) is int and corpus['version'] == 1
             and type(truth.get('version')) is int and truth['version'] == 1, 'Unsupported input version')
    _require(truth.get('corpusSha256') == sidecar['corpus']['sha256'], 'Truth corpus binding mismatch')
    units = _index(corpus['units'], 'id')
    labels = _index(truth['labels'], 'id')
    audits = _index(audit['rows'], 'id')
    _require(bool(units) and units.keys() == labels.keys() == audits.keys(), 'Incomplete source/label audit coverage')
    _require(all(row.get('label') in LABELS for row in labels.values()), 'Invalid label')
    files = _index(corpus['files'], 'path')
    lines = {}
    for name, file in files.items():
        _relative(name)
        lines[name] = _source_lines(file)
    families = corpus['families']
    _require(isinstance(families, dict) and bool(families), 'Missing rule families')
    rule_ids = []
    for family, ids in families.items():
        _require(_text(family) and isinstance(ids, list) and bool(ids)
                 and all(_text(rule) for rule in ids), 'Invalid rule family')
        rule_ids.extend(ids)
    _require(len(rule_ids) == len(set(rule_ids)), 'Duplicate rule family membership')
    intervals = {}
    for identity, unit in units.items():
        _require(unit.get('path') in files and unit.get('ruleId') in rule_ids, 'Invalid unit source/rule')
        _require(all(type(unit.get(k)) is int for k in ('line', 'startColumn', 'endColumn')),
                 'Invalid unit coordinates')
        line, start, end = unit['line'], unit['startColumn'], unit['endColumn']
        source = lines[unit['path']]
        _require(0 < line <= len(source) and 0 < start < end <= len(source[line - 1]) + 1,
                 'Unit outside source line')
        key = (unit['path'], unit['ruleId'], line)
        previous = intervals.setdefault(key, [])
        _require(all(max(start, lo) >= min(end, hi) for lo, hi in previous), 'Overlapping unit intervals')
        previous.append((start, end))
        row = audits[identity]
        _require(row.get('frozenUnit') == unit and row.get('originalLabel') == labels[identity]['label']
                 and _text(row.get('sourceFactPreserved')), 'Source audit provenance mismatch')

    contracts = _index(sidecar['units'], 'id')
    eligible = {identity for identity, row in labels.items() if row['label'] in ELIGIBLE}
    _require(contracts.keys() == eligible, 'Missing or extra eligible source contracts')
    rows, rejected = [], []
    for identity, unit in units.items():
        label, audited = labels[identity]['label'], audits[identity]
        row = {'id': identity, 'originalUnit': unit, 'originalLabel': label,
               'sourceFact': audited['sourceFactPreserved'], 'reasons': []}
        if identity not in eligible:
            row['evaluability'] = 'unchanged-ineligible'
        else:
            contract = contracts[identity]
            _require(set(contract) == {'id', 'originalLabel', 'sourceFact', 'supportedInput',
                                       'expectedAnchorColumn', 'sourceContractPaths'}, 'Invalid unit contract schema')
            _require(contract['originalLabel'] == label and contract['sourceFact'] == row['sourceFact'],
                     'Contract label/fact provenance mismatch')
            _require(type(contract['supportedInput']) is bool, 'Invalid supportedInput')
            column = contract['expectedAnchorColumn']
            _require(type(column) is int and 0 < column <= len(lines[unit['path']][unit['line'] - 1]),
                     'Invalid expected anchor column')
            paths = contract['sourceContractPaths']
            _require(isinstance(paths, list) and bool(paths) and all(_text(path) for path in paths)
                     and len(paths) == len(set(paths)) and set(paths) <= set(bound_paths),
                     'Unbound source contract references')
            row.update(supportedInput=contract['supportedInput'], expectedAnchorColumn=column)
            if not contract['supportedInput']:
                row['reasons'].append('unsupported_input')
            if not unit['startColumn'] <= column < unit['endColumn']:
                row['reasons'].append('anchor_outside_interval')
            row['evaluability'] = 'not-evaluable' if row['reasons'] else 'evaluable'
            if row['reasons']:
                rejected.append(identity)
        rows.append(row)
    return {'version': 1, 'kind': 'source-contract-admission-check',
            'scope': 'Supplied source-reviewed input and anchor contracts only; no label reapproval or metrics.',
            'exposure': exposure, 'corpus': sidecar['corpus'], 'truth': sidecar['truth'],
            'checker': sidecar['checker'], 'audit': sidecar['audit'],
            'contracts': {'path': str(pathlib.Path(contracts_path).resolve()), 'sha256': _sha(pathlib.Path(contracts_path))},
            'sourceContractBindings': bindings, 'rejectedUnitIds': rejected, 'rows': rows}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('corpus', 'truth', 'contracts', 'out'):
        parser.add_argument('--' + name, required=True, type=pathlib.Path)
    args = parser.parse_args()
    try:
        result = check(args.corpus, args.truth, args.contracts)
        with args.out.open('x', encoding='utf-8') as stream:
            json.dump(result, stream, ensure_ascii=False, indent=2, allow_nan=False)
            stream.write('\n')
        return 1 if result['rejectedUnitIds'] else 0
    except (ValueError, OSError, KeyError, TypeError) as error:
        print(f'Annotation contract check failed: {error}', file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
