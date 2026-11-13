"""Account LLVM exports without mixing dependencies, test files or absent branches.

Production source locations only; inline #[cfg(test)] blocks remain in the source
file denominator and are explicitly disclosed in the study.
"""
from pathlib import PurePosixPath

KINDS = ('lines', 'regions', 'functions', 'branches')


def summarize(document, *, branches=False):
    data = document.get('data', [])
    if len(data) != 1:
        raise ValueError('one LLVM export dataset is required')
    files = {}
    totals = {kind: {'count': 0, 'covered': 0} for kind in KINDS}
    for item in data[0].get('files', []):
        path = PurePosixPath(item['filename'])
        try:
            name = path.relative_to('/source')
        except ValueError:
            continue
        if '..' in name.parts or len(name.parts) < 4 or name.parts[0] != 'crates' or name.parts[2] != 'src':
            continue
        key = str(name)
        if key in files:
            raise ValueError('duplicate production file')
        values = {}
        for kind in KINDS:
            count, covered = (item['summary'][kind][k] for k in ('count', 'covered'))
            if type(count) is not int or type(covered) is not int or not 0 <= covered <= count:
                raise ValueError('invalid coverage denominator')
            values[kind] = {'count': count, 'covered': covered}
            for k in ('count', 'covered'):
                totals[kind][k] += values[kind][k]
        files[key] = values
    if not files:
        raise ValueError('no production source coverage')
    for values in [totals, *files.values()]:
        for kind, metric in values.items():
            measured = kind != 'branches' or branches
            metric['percent'] = 100 * metric['covered'] / metric['count'] if measured and metric['count'] else None
            if kind == 'branches':
                metric['status'] = 'Measured' if measured else 'Not Measured'
    return {'totals': totals, 'files': files}
