"""Recheck this evidence bundle without executing specimens or scanning networks."""
import ast
import hashlib
import json
import pathlib
import subprocess
import sys
import tarfile

repo = pathlib.Path(__file__).resolve().parents[3]
bundle = pathlib.Path(__file__).resolve().parent
sha = lambda value: hashlib.sha256(value).hexdigest()
index = json.loads((bundle/'EVIDENCE-INDEX.json').read_text())
for name, expected in index.items():
    assert sha((bundle/name).read_bytes()) == expected, name
entry = json.loads((bundle/'ENTRY.json').read_text())
for name, expected in entry['protectedBindings'].items():
    assert sha((repo/name).read_bytes()) == expected, name
with tarfile.open(bundle/'RAW-MEASUREMENTS.tar.gz') as archive:
    for name in ['legacy', 'v1', 'v2', 'lock-controls']:
        report = json.loads((bundle/(name+'-report')/'quality.json').read_text())
        assert report['reporterSha256'] == sha((repo/'tests/quality/report.py').read_bytes())
        assert report['sourceSummarySha256'] == sha(archive.extractfile(name+'/summary.json').read())
        for path, expected in report['rawBindings'].items():
            assert sha(archive.extractfile(name+'/'+path).read()) == expected, (name, path)
    audit = json.loads((bundle/'code-scanning-audit.json').read_text())
    raw = archive.extractfile('self-scan/report.sarif').read()
    filtered = json.load(archive.extractfile('self-scan/code-scanning.sarif'))
    assert sha(raw) == audit['rawSarifSha256']
    assert sum(len(run['results']) for run in json.loads(raw)['runs']) == audit['rawResults'] == 1357
    assert sum(len(run['results']) for run in filtered['runs']) == audit['retainedResults'] == 28
sys.path.insert(0, str(repo/'tests/quality'))
from evaluate import load_truth, validate_corpus
for name in ['', 'expanded', 'expanded-v2', 'expanded-v2-lock-controls']:
    data, labels = load_truth(repo/'tests/quality'/name)
    validate_corpus(data, repo)
original = subprocess.check_output(['git', 'show', entry['head']+':tests/quality/evaluate.py'], cwd=repo).decode()
current = (repo/'tests/quality/evaluate.py').read_text()
def scorer(text):
    return ast.dump(next(node for node in ast.parse(text).body
                         if isinstance(node, ast.FunctionDef) and node.name == 'score'))
assert scorer(original) == scorer(current)
print(f'Verified {len(index)} artifact hashes, {len(entry["protectedBindings"])} protected files, raw repeated measurements and unchanged scorer')
