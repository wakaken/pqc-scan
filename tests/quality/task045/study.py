"""Small, explicit mutation sample; not an automatic Rust mutation generator."""
import hashlib
from pathlib import Path, PurePosixPath
import re


def apply(root, row):
    name = PurePosixPath(row['path'])
    if name.is_absolute() or '..' in name.parts:
        raise ValueError('mutation path must stay in the scratch copy')
    path = Path(root)/name
    before = path.read_bytes()
    old, new = row['old'].encode(), row['new'].encode()
    if not old or old == new or before.count(old) != 1:
        raise ValueError('mutation requires one exact, changed source site')
    after = before.replace(old, new, 1)
    path.write_bytes(after)
    return hashlib.sha256(before).hexdigest(), hashlib.sha256(after).hexdigest()


def classify(build_exit, test_exit, log, build_log=""):
    if 'timeout' in (build_exit, test_exit):
        return 'timeout'
    if build_exit != 0:
        # Be conservative: only an ordinary rustc diagnostic with an error code
        # establishes source invalidity. Unknown build failures stop the study.
        infrastructure = ('no space', 'out of memory', 'disk quota', 'killed',
                          'sigkill', 'failed to download', 'could not execute',
                          'linking with', 'permission denied', 'read-only file',
                          'could not write', 'failed to run custom build command',
                          'internal compiler error')
        if (build_exit not in (1, 101) or
                any(value in build_log.lower() for value in infrastructure) or
                not re.search(r'error\[E\d{4}\]', build_log)):
            raise ValueError('build infrastructure failure or unclassified diagnostic')
        return 'unviable'
    results = re.findall(r'test result: (ok|FAILED)\. (\d+) passed; (\d+) failed;', log)
    if not results or sum(int(p)+int(f) for _,p,f in results) == 0:
        raise ValueError('no executed test results; cannot assess mutation')
    failed = sum(int(f) for _,_,f in results)
    if test_exit == 101 and failed > 0:
        return 'caught'
    if test_exit == 0 and failed == 0:
        return 'missed'
    raise ValueError('execution error is not an assertion-based mutation outcome')


def summarize(rows):
    counts = dict.fromkeys(('caught', 'missed', 'timeout', 'unviable'), 0)
    seen = set()
    if not rows:
        raise ValueError('empty study')
    for row in rows:
        if row['id'] in seen or row['outcome'] not in counts:
            raise ValueError('duplicate or invalid mutation outcome')
        seen.add(row['id'])
        counts[row['outcome']] += 1
    denominator = counts['caught'] + counts['missed']
    return {'counts':counts, 'scoreDenominator':denominator,
            'scorePercent':100*counts['caught']/denominator if denominator else None}
