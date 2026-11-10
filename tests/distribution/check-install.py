#!/usr/bin/env python3
"""Clean-prefix offline acceptance for locally prepared macOS archives."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('archive', type=Path)
parser.add_argument('--out-dir', type=Path, required=True)
args = parser.parse_args()
script = Path(__file__).resolve().parents[2]/'scripts/package-release.py'
spec = importlib.util.spec_from_file_location('package_release', script)
package = importlib.util.module_from_spec(spec)
spec.loader.exec_module(package)
package.verify_archive(args.archive)
args.out_dir.mkdir(parents=True, exist_ok=True)
with tempfile.TemporaryDirectory(prefix='pqc-clean-install-') as directory:
    base = Path(directory).resolve()
    install, fixture = base/'install', base/'fixture'
    install.mkdir()
    fixture.mkdir()
    (fixture/'example.py').write_text('cipher_suite = "TLS_RSA_WITH_AES_128_CBC_SHA"\n')
    with tarfile.open(args.archive, 'r:gz') as archive:
        # Verification has rejected symlinks, duplicate names and escaping paths.
        archive.extractall(install, filter='data')
    bundle, = install.iterdir()
    binary, rules = bundle/'bin/pqc-scan', bundle/'rules/default'
    sandbox = ['/usr/bin/sandbox-exec', '-p', '(version 1)(allow default)(deny network*)']
    results = {}
    for name, command in [('rules', ['rules', 'list', '--rules-dir', str(rules)]), ('scan', ['scan', str(fixture), '--rules-dir', str(rules), '--format', 'json', '--out-dir', str(base/'reports')])]:
        result = subprocess.run(sandbox+[str(binary)]+command, cwd=base, capture_output=True, text=True)
        results[name] = {'exit': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr}
        if result.returncode:
            raise RuntimeError(f'{name} failed: {result.stderr}')
    report = json.loads((base/'reports/report.json').read_text())
    if not report['findings'] or not results['rules']['stdout'].strip():
        raise RuntimeError('expected synthetic finding and installed rules')
    provenance = json.loads((bundle/'PROVENANCE.json').read_text())
    (args.out_dir/'INSTALL-RESULTS.json').write_text(json.dumps({'archive_sha256': hashlib.sha256(args.archive.read_bytes()).hexdigest(), 'fixture_sha256': hashlib.sha256((fixture/'example.py').read_bytes()).hexdigest(), 'network_policy': 'macOS sandbox-exec deny network*', 'provenance': provenance, 'results': results, 'findings': len(report['findings']), 'clean_prefix_removed': True}, sort_keys=True, indent=2)+'\n')
print('Clean-prefix installation, installed rules and external synthetic scan passed offline.')
