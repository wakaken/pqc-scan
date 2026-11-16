#!/usr/bin/env python3
"""Check the workspace lockfile without building code or loading checkout config."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
import re
from pathlib import Path
import subprocess
import tempfile
import tomllib

def classify(report, exit_code):
    """Reject failed, filtered and internally inconsistent cargo-audit reports."""
    vulnerabilities = report['vulnerabilities']
    entries, warnings = vulnerabilities['list'], report['warnings']
    if (type(entries) is not list or type(warnings) is not dict or
            type(vulnerabilities['count']) is not int or
            vulnerabilities['count'] != len(entries) or
            type(vulnerabilities['found']) is not bool or
            vulnerabilities['found'] != bool(entries) or
            exit_code != (1 if entries else 0) or report['settings']['ignore'] != [] or
            report['settings']['target_arch'] != [] or report['settings']['target_os'] != [] or
            report['settings']['severity'] is not None):
        raise ValueError('failed, filtered or inconsistent audit')
    findings = []
    for kind, items in [('vulnerability', entries), *sorted(warnings.items())]:
        if type(items) is not list:
            raise ValueError('invalid warnings')
        for item in items:
            advisory, package = item['advisory'], item['package']
            if (not re.fullmatch(r'RUSTSEC-\d{4}-\d{4}', advisory['id']) or
                    advisory['package'] != package['name'] or not package['version'] or
                    package.get('source') != 'registry+https://github.com/rust-lang/crates.io-index'):
                raise ValueError('invalid finding identity')
            findings.append({'kind': kind, 'advisory': advisory, 'package': package,
                             'versions': item.get('versions'), 'affected': item.get('affected')})
    if entries:
        return 'vulnerabilities', findings
    return ('warnings' if findings else 'clean'), findings


def run(binary, database, lockfile, output):
    binary, database, lockfile, output = [Path(p).resolve() for p in (binary, database, lockfile, output)]
    output.mkdir(parents=True, exist_ok=True)
    result = {'status': 'error', 'observedAtUtc': datetime.now(timezone.utc).isoformat(),
              'sourceCommit': os.environ.get('GITHUB_SHA'), 'findings': []}
    raw, errors = b'', b''
    code = 2
    try:
        lock_bytes = lockfile.read_bytes()  # Never generate a missing lockfile.
        packages = tomllib.loads(lock_bytes.decode())['package']
        if not isinstance(packages, list) or not packages:
            raise ValueError('empty lockfile population')
        if not database.is_dir():
            raise ValueError('missing advisory database')
        result.update(lockfileSha256=hashlib.sha256(lock_bytes).hexdigest(),
                      toolSha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                      lockfilePackages=len(packages))
        with tempfile.TemporaryDirectory(prefix='pqc-dependency-audit-', dir=os.environ.get('RUNNER_TEMP')) as neutral:
            home = Path(neutral)/'home'; home.mkdir()
            cargo = Path(neutral)/'cargo'; cargo.mkdir()
            env = {'PATH': '/usr/local/bin:/usr/bin:/bin', 'HOME': str(home),
                   'CARGO_HOME': str(cargo), 'TMPDIR': neutral}
            argv = [str(binary), 'audit', '--db', str(database), '--no-fetch', '--file', str(lockfile), '--json']
            result['command'] = argv
            process = subprocess.run(argv, cwd=Path(neutral), env=env, timeout=180, capture_output=True)
        raw, errors = process.stdout, process.stderr
        result['toolExitCode'] = process.returncode
        report = json.loads(raw)
        status, findings = classify(report, process.returncode)
        if (type(report['lockfile']['dependency-count']) is not int or
                report['lockfile']['dependency-count'] != len(packages)):
            raise ValueError('partial lockfile population')
        result.update(status=status, findings=findings)
        code = 1 if status == 'vulnerabilities' else 0
    except (OSError, ValueError, KeyError, TypeError, subprocess.TimeoutExpired) as error:
        result['errorKind'] = type(error).__name__
        # A timeout/unavailable tool is an error, never a clean empty report.
        if isinstance(error, subprocess.TimeoutExpired):
            raw, errors = error.stdout or b'', error.stderr or b''
    (output/'audit.json').write_bytes(raw)
    (output/'audit.stderr').write_bytes(errors)
    result['exitCode'] = code
    (output/'result.json').write_text(json.dumps(result, indent=2)+'\n')
    print(f"Dependency audit: {result['status']}; exit {code}; see retained evidence.")
    return code


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['binary', 'database', 'lockfile', 'out-dir']:
        parser.add_argument('--'+name, type=Path, required=True)
    args = parser.parse_args()
    return run(args.binary, args.database, args.lockfile, args.out_dir)


if __name__ == '__main__':
    raise SystemExit(main())
