#!/usr/bin/env python3
"""Compile outside isolation, then run exact workspace test binaries with no egress."""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
PROFILE = '(version 1) (allow default) (deny network-outbound) (allow network-outbound (remote ip "localhost:*"))'

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def sources():
    names = subprocess.check_output(['git','ls-files','--cached','--others','--exclude-standard','--','crates','schemas','Cargo.toml','Cargo.lock','rust-toolchain.toml','tests/fixtures/ssh-kex-host.pub'],cwd=ROOT,text=True).splitlines()
    return {name:digest(ROOT/name) for name in sorted(set(names)) if (ROOT/name).is_file()}

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--out-dir', default='target/protocol-matrix')
    parser.add_argument('--target-dir', default='target/probe-runtime-build')
    args = parser.parse_args()
    if platform.system() != 'Darwin':
        raise RuntimeError('This isolation runner requires macOS sandbox-exec; another OS needs its own reviewed runner.')
    output = ROOT / args.out_dir
    output.mkdir(parents=True, exist_ok=True)
    results_path = output / 'RESULTS.json'
    results_path.write_text(json.dumps({'status':'building', 'exit':None}) + '\n')
    source_bindings = sources()
    command = ['cargo', 'test', '--workspace', '--all-features', '--locked', '--no-run', '--message-format=json', '--target-dir', args.target_dir]
    with (output / 'build.jsonl').open('w') as stdout, (output / 'build.log').open('w') as stderr:
        build = subprocess.run(command, cwd=ROOT, stdout=stdout, stderr=stderr)
    if build.returncode:
        results_path.write_text(json.dumps({'status':'build_failed','buildExit':build.returncode,'exit':build.returncode}) + '\n')
        return build.returncode
    tests = []
    binaries = {}
    for line in (output / 'build.jsonl').read_text().splitlines():
        item = json.loads(line)
        if item.get('reason') != 'compiler-artifact' or not item.get('executable'):
            continue
        path = item['executable']
        binaries[path] = digest(path)
        if item.get('profile', {}).get('test'):
            tests.append((item['target']['name'], path))
    if not tests:
        raise RuntimeError('No test executables were emitted')
    records = []
    code = 0
    for index, (name, path) in enumerate(tests):
        if any(digest(binary) != sha for binary, sha in binaries.items()):
            raise RuntimeError('A compiled executable changed before execution')
        log = output / f'{index:02d}-{name}.log'
        with log.open('w') as stream:
            result = subprocess.run(['sandbox-exec', '-p', PROFILE, path], cwd=ROOT, stdout=stream, stderr=subprocess.STDOUT, timeout=180)
        records.append({'test': name, 'executable': path, 'sha256': binaries[path], 'exit': result.returncode, 'log': str(log.relative_to(ROOT))})
        print(f'{name}: exit {result.returncode}', flush=True)
        code = code or result.returncode
    if any(digest(binary) != sha for binary, sha in binaries.items()):
        raise RuntimeError('A compiled executable changed during execution')
    if source_bindings != sources():
        raise RuntimeError('Source content changed during compilation/tests; rerun on a frozen snapshot')
    record = {'command': command, 'buildExit': build.returncode, 'platform': platform.platform(), 'machine': platform.machine(), 'profile': PROFILE, 'sourceBindings':source_bindings, 'binaries': binaries, 'tests': records, 'exit': code, 'beforeAfterHashesMatch': True}
    results_path.write_text(json.dumps(record, indent=2) + '\n')
    return code

if __name__ == '__main__':
    sys.exit(main())
