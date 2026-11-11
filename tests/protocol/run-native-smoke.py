#!/usr/bin/env python3
"""Offline build and exact-binary execution with explicit macOS boundaries.

No hostile-code memory/aggregate disk containment is claimed; see isolation.py.
"""
import argparse
import os
import tempfile
import isolation
import hashlib
import json
from pathlib import Path
import platform
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def sources():
    names = subprocess.check_output(['git','ls-files','--cached','--others','--exclude-standard','--','crates','schemas','Cargo.toml','Cargo.lock','rust-toolchain.toml','tests/fixtures/ssh-kex-host.pub','tests/protocol'],cwd=ROOT,text=True).splitlines()
    return {name:digest(ROOT/name) for name in sorted(set(names)) if (ROOT/name).is_file()}

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--out-dir', default='target/protocol-matrix')
    parser.add_argument('--target-dir', default='target/probe-runtime-build')
    args = parser.parse_args()
    if platform.system() != 'Darwin':
        raise RuntimeError('This isolation runner requires macOS sandbox-exec; another OS needs its own reviewed runner.')
    output = (ROOT / args.out_dir).resolve()
    target = (ROOT / args.target_dir).resolve()
    for path in [output, target]:
        if not path.is_relative_to((ROOT / 'target').resolve()) or path == (ROOT / 'target').resolve():
            raise RuntimeError('Output/build directories must be dedicated subdirectories of target')
    if output == target or output.is_relative_to(target) or target.is_relative_to(output):
        raise RuntimeError('Evidence and build directories must be separate')
    output.mkdir(parents=True, exist_ok=True)
    target.mkdir(parents=True, exist_ok=True)
    cargo = Path(subprocess.check_output(['rustup', 'which', 'cargo'], text=True).strip()).resolve()
    cache = Path(os.environ.get('CARGO_HOME', str(Path.home() / '.cargo'))).resolve()
    scratch_context = tempfile.TemporaryDirectory(prefix='pqc-protocol-scratch-')
    # Keep scratch alive through exact-binary execution; removed on function exit.
    with scratch_context as temp:
        return execute(output, target, cargo, cache, Path(temp).resolve())


def execute(output, target, cargo, cache, scratch):
    env = isolation.environment(scratch, cargo)
    # Resolve the selected system SDK before isolation; bypass xcrun's ambient
    # per-user cache and license lookup inside children. No SDK setup is changed.
    clang = Path(subprocess.check_output(['xcrun', '--sdk', 'macosx', '--find', 'clang'], env={'PATH':'/usr/bin:/bin:/usr/sbin:/sbin'}, text=True).strip()).resolve()
    sdk = Path(subprocess.check_output(['xcrun', '--sdk', 'macosx', '--show-sdk-path'], env={'PATH':'/usr/bin:/bin:/usr/sbin:/sbin'}, text=True).strip()).resolve()
    env['PATH'] = str(clang.parent) + os.pathsep + env['PATH']
    env['CC'] = str(clang)
    env['SDKROOT'] = str(sdk)

    for name in ['registry', 'git']:
        if (cache / name).exists():
            (scratch / 'cargo' / name).symlink_to(cache / name, target_is_directory=True)
    build_profile = isolation.profile(ROOT, scratch, [cargo.parent.parent, cache / 'registry', cache / 'git'], [target])
    test_profile = isolation.profile(ROOT, scratch)
    results_path = output / 'RESULTS.json'
    results_path.write_text(json.dumps({'status':'building', 'exit':None}) + '\n')
    source_bindings = sources()
    command = [str(cargo), 'test', '--workspace', '--all-features', '--locked', '--offline', '--no-run', '--message-format=json', '--target-dir', str(target)]
    with (output / 'build.jsonl').open('w') as stdout, (output / 'build.log').open('w') as stderr:
        build = isolation.run(command, ROOT, env, build_profile, timeout=1200, stdout=stdout, stderr=stderr)
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
            result = isolation.run([path], ROOT, env, test_profile, timeout=180, stdout=stream, stderr=subprocess.STDOUT)
        records.append({'test': name, 'executable': path, 'sha256': binaries[path], 'exit': result.returncode, 'log': str(log.relative_to(ROOT))})
        print(f'{name}: exit {result.returncode}', flush=True)
        code = code or result.returncode
    if any(digest(binary) != sha for binary, sha in binaries.items()):
        raise RuntimeError('A compiled executable changed during execution')
    if source_bindings != sources():
        raise RuntimeError('Source content changed during compilation/tests; rerun on a frozen snapshot')
    record = {'command': command, 'buildExit': build.returncode, 'platform': platform.platform(), 'machine': platform.machine(), 'buildProfile': build_profile, 'testProfile': test_profile, 'environmentKeys': sorted(env), 'compiler':str(clang), 'sdk':str(sdk), 'limits': {'cpuSeconds':900, 'fileBytes':1073741824, 'openFiles':1024, 'processesPerUser':1024, 'memoryEnforced':False, 'aggregateDiskEnforced':False}, 'sourceBindings':source_bindings, 'binaries': binaries, 'tests': records, 'exit': code, 'beforeAfterHashesMatch': True}
    results_path.write_text(json.dumps(record, indent=2) + '\n')
    return code

if __name__ == '__main__':
    sys.exit(main())
