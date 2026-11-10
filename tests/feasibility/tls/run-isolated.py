#!/usr/bin/env python3
"""Compile offline outside the sandbox, then run only local TLS peers inside it."""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument('--without-tls12', action='store_true')
args = parser.parse_args()
if platform.system() != 'Darwin':
    raise SystemExit('This runner requires the reviewed macOS sandbox-exec policy.')
root = Path(__file__).resolve().parents[3]
manifest = Path(__file__).with_name('Cargo.toml')
label = 'aws-lc-tls13' if args.without_tls12 else 'aws-lc-tls12-tls13'
out = root / 'target/task-031'
out.mkdir(parents=True, exist_ok=True)
command = ['cargo', 'test', '--manifest-path', str(manifest), '--target-dir', str(root / 'target/task-031-build'), '--locked', '--offline', '--no-run', '--message-format=json']
if args.without_tls12:
    command.append('--no-default-features')
start = time.monotonic()
build = subprocess.run(command, capture_output=True, text=True, timeout=300)
(out / f'{label}-build.jsonl').write_text(build.stdout)
(out / f'{label}-build.log').write_text(build.stderr)
if build.returncode:
    raise SystemExit(f'Build failed; inspect {out / (label + "-build.log")}')
artifacts = [json.loads(line) for line in build.stdout.splitlines()]
executables = [row['executable'] for row in artifacts if row.get('reason') == 'compiler-artifact' and row.get('profile', {}).get('test') and row.get('executable')]
if len(executables) != 1:
    raise SystemExit('Expected exactly one test executable.')
binary = Path(executables[0])
profile = '(version 1) (allow default) (deny network-outbound) (allow network-outbound (remote ip "localhost:*"))'
test_command = ['sandbox-exec', '-p', profile, str(binary), '--nocapture', '--test-threads=1']
result = subprocess.run(test_command, capture_output=True, text=True, timeout=60)
log = result.stdout + result.stderr
(out / f'{label}-isolated.log').write_text(log)
record = {'variant': label, 'buildCommand': command, 'buildExit': build.returncode, 'testCommand': test_command, 'testExit': result.returncode, 'totalSeconds': time.monotonic() - start, 'binarySha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'isolationProfile': profile}
(out / f'{label}-result.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps(record, indent=2))
print(log)
raise SystemExit(result.returncode)
