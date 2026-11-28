"""Bounded exploratory OSS scoring; existing corpus/license contracts stay intact.

Only annotated intervals are scored. Other locations are unattributed, so this
adapter cannot establish full-file or repository precision. No independent-label
claim is made here. Source acquisition, licenses and annotation completeness must
be recorded separately by the study owner.
"""
import gzip
import hashlib
import json
import os
import pathlib
import re
import stat

from evaluate import bytes_digest, safe_relative, score, validate_labels


def source_language(name):
    """Inventory-based grouping; headers and untagged INC remain unresolved."""
    suffix = pathlib.PurePosixPath(name).suffix
    if suffix == '.C':
        return 'C++'
    languages = {'.java': 'Java', '.go': 'Go', '.js': 'JavaScript', '.ts': 'TypeScript',
                 '.tsx': 'TSX', '.py': 'Python', '.rs': 'Rust', '.rb': 'Ruby',
                 '.swift': 'Unsupported Swift', '.php': 'PHP', '.phtml': 'PHP',
                 '.inc': 'Unresolved INC', '.cs': 'C#', '.c': 'C', '.cc': 'C++',
                 '.cpp': 'C++', '.cxx': 'C++', '.hpp': 'C++', '.hxx': 'C++',
                 '.hh': 'C++', '.h': 'Ambiguous C/C++ header'}
    return languages.get(suffix.lower(), 'Not Applicable')


def bound_source(root, name, expected_sha256):
    # The operator-selected root may use a system alias such as /tmp on macOS.
    # Only descendants must remain exact, non-symlinked source paths.
    root = pathlib.Path(root).resolve(strict=True)
    path = root / safe_relative(name)
    if path.resolve() != path or not path.is_file() or bytes_digest(path) != expected_sha256:
        raise ValueError('External source changed, unavailable or symlinked')
    return path


def validate_tree(root, files):
    """Bind every non-Git-metadata file, retaining but never following symlinks."""
    root = pathlib.Path(root).resolve(strict=True)
    expected = {}
    for item in files:
        name = item['path']
        safe_relative(name)
        if name in expected or '.git' in pathlib.PurePosixPath(name).parts:
            raise ValueError('Duplicate input or Git metadata in source population')
        expected[name] = item
    actual = set()
    for directory, dirs, names in os.walk(root, followlinks=False):
        if pathlib.Path(directory) != root and ('.git' in dirs or '.git' in names):
            raise ValueError('Unexpected nested Git metadata could change file selection')
        dirs[:] = [d for d in dirs if d != '.git']
        for name in list(dirs):
            path = pathlib.Path(directory)/name
            if path.is_symlink():
                actual.add(path.relative_to(root).as_posix())
                dirs.remove(name)
        actual.update((pathlib.Path(directory)/n).relative_to(root).as_posix() for n in names if n != '.git')
    if actual != expected.keys():
        raise ValueError('External input population changed')
    for name, item in expected.items():
        path = root / name
        mode = path.lstat().st_mode
        if item.get('mode') == '120000':
            if not stat.S_ISLNK(mode) or os.readlink(path) != item['symlink']:
                raise ValueError('Pinned source symlink changed')
        elif not stat.S_ISREG(mode) or path.is_symlink() or path.stat().st_size != item['bytes']:
            raise ValueError('Pinned source mode/size changed')
        else:
            bound_source(root, name, item['sha256'])


def load_bound_report(receipt, truth_paths):
    """Reject changed truth or compressed/plain output before parsing a report."""
    if not receipt['truthBindings'] or receipt['truthBindings'].keys() != truth_paths.keys():
        raise ValueError('Missing frozen truth binding')
    for name, path in truth_paths.items():
        if bytes_digest(path) != receipt['truthBindings'][name]:
            raise ValueError('Frozen truth changed')
    item = receipt['artifacts']['report.json']
    compressed = pathlib.Path(item['compressedPath']).read_bytes()
    if hashlib.sha256(compressed).hexdigest() != item['compressedSha256']:
        raise ValueError('Compressed output changed')
    with gzip.open(item['compressedPath'], 'rb') as stream:
        raw = stream.read(item['bytes'] + 1)
    if len(raw) != item['bytes'] or hashlib.sha256(raw).hexdigest() != item['sha256']:
        raise ValueError('Plain output changed')
    return json.loads(raw)


def selection_bindings(root, *, home=None, xdg=None, cwd=None):
    """Bind ignore-0.4.25's external selection inputs without copying secrets.

    The scanner reads ~/.gitconfig itself; GIT_CONFIG_GLOBAL does not isolate
    this dependency. Preserve its actual precedence, including absent files.
    Config contents stay private; only paths and digests enter study evidence.
    """
    root = pathlib.Path(root).resolve(strict=True)
    cwd = (pathlib.Path.cwd() if cwd is None else pathlib.Path(cwd)).resolve()
    home = pathlib.Path.home() if home is None else pathlib.Path(home)
    home = (home if home.is_absolute() else cwd/home).resolve()
    xdg = pathlib.Path(os.environ.get('XDG_CONFIG_HOME') or home/'.config') if xdg is None else pathlib.Path(xdg)
    xdg = (xdg if xdg.is_absolute() else cwd/xdg).resolve()
    paths = {home/'.gitconfig', xdg/'git/config', xdg/'git/ignore',
             root/'.git/config', root/'.git/info/exclude'}
    effective = xdg/'git/ignore'
    pattern = re.compile(rb'^\s*excludesfile\s*=\s*"?\s*(\S+?)\s*"?\s*$', re.I | re.M | re.ASCII)
    for config in (home/'.gitconfig', xdg/'git/config'):
        if config.is_file():
            match = pattern.search(config.read_bytes())
            if match:
                try:
                    effective = pathlib.Path(match[1].decode('utf-8').replace('~', str(home)))
                except UnicodeDecodeError:
                    continue
                break
    if not effective.is_absolute():
        effective = cwd/effective
    paths.add(effective)
    for directory in (root, *root.parents):
        paths.update(directory/name for name in ('.ignore', '.gitignore', '.git/info/exclude'))
    bindings = {}
    for path in sorted(paths):
        if not path.exists() and not path.is_symlink():
            bindings[str(path)] = dict(exists=False)
        elif not path.is_file():
            raise ValueError('Selection input is not a regular file')
        else:
            bindings[str(path)] = dict(exists=True, resolved=str(path.resolve()), sha256=bytes_digest(path))
    return dict(effectiveGlobalIgnore=str(effective), files=bindings)


def score_annotated(data, labels, findings):
    """Use unchanged one-to-one scorer inside frozen annotated intervals only."""
    validate_labels(data, labels)
    files = {f['path'] for f in data['files']}
    nominated = {r for rules in data['families'].values() for r in rules}
    if len(files) != len(data['files']) or len(nominated) != sum(map(len, data['families'].values())):
        raise ValueError('Duplicate files or rule families')
    intervals = {}
    ids = set()
    for unit in data['units']:
        safe_relative(unit['path'])
        if unit['id'] in ids or unit['path'] not in files or unit['ruleId'] not in nominated:
            raise ValueError('Invalid unit identity')
        ids.add(unit['id'])
        if any(type(unit[k]) is not int for k in ('line', 'startColumn', 'endColumn')) or not (
                unit['line'] > 0 and 0 < unit['startColumn'] < unit['endColumn']):
            raise ValueError('Invalid truth interval')
        key = (unit['path'], unit['ruleId'], unit['line'])
        for previous in intervals.setdefault(key, []):
            if max(previous['startColumn'], unit['startColumn']) < min(previous['endColumn'], unit['endColumn']):
                raise ValueError('Overlapping truth intervals')
        intervals[key].append(unit)
    selected, unattributed, other_rules = [], [], []
    for finding in findings:
        loc = finding['location']
        if finding['rule_id'] not in nominated:
            other_rules.append(finding)
            continue
        candidates = intervals.get((loc['file'], finding['rule_id'], loc['line']), [])
        if loc['line'] == 0:
            name = finding.get('evidence', {}).get('metadata', {}).get('dep_name')
            candidates = [u for u in data['units'] if u['path'] == loc['file'] and
                          u['ruleId'] == finding['rule_id'] and name is not None and u.get('dependencyName') == name]
        if any(loc['line'] == 0 or u['startColumn'] <= loc['column'] < u['endColumn'] for u in candidates):
            selected.append(finding)
        else:
            unattributed.append(finding)
    return dict(accuracy=score(data, labels, selected), unattributed=unattributed,
                other_rules=other_rules,
                scope='Annotated intervals only; not full-file or repository accuracy')
