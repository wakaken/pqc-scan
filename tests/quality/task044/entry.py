"""TASK-044 study entry; run only via the existing strict container launcher."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
import gzip
import traceback

sys.path.insert(0, '/source/tests/protocol')
sys.path.insert(0, '/source/tests/quality')
from container_entry import boundary, EVIDENCE, SCRATCH, write
from task044.coverage import summarize

NIGHTLY = 'nightly-2026-02-12'  # Explicit branch-coverage toolchain only.
COMMANDS = []


def run(args, name, *, env=None, timeout=600, required=True, separate_stderr=False):
    start = time.monotonic()
    status = None
    try:
        with (EVIDENCE/name).open('w') as stream, (EVIDENCE/(name+'.stderr')).open('w') as errors:
            result = subprocess.run(args, cwd='/source', env=env, stdout=stream,
                                    stderr=errors if separate_stderr else subprocess.STDOUT, timeout=timeout)
        status = result.returncode
    except subprocess.TimeoutExpired:
        status = 'timeout'
    finally:
        COMMANDS.append({'command': args, 'environment': env, 'log': name,
                         'exit': status, 'wallSeconds': time.monotonic()-start,
                         'timeoutSeconds': timeout})
        write('commands.json', COMMANDS)
    if required and status != 0:
        raise RuntimeError(f'{name}: {status}')
    return status


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def measure(label, *, all_features=False, nightly=False):
    folder = EVIDENCE/label
    folder.mkdir()
    target = SCRATCH/('nightly-target' if nightly else 'target')
    raw = SCRATCH/('raw-'+label)
    raw.mkdir()
    toolchain = NIGHTLY if nightly else '1.93.1'
    env = dict(os.environ, CARGO_TARGET_DIR=str(target),
               RUSTFLAGS='-C instrument-coverage' + (' -Z coverage-options=branch' if nightly else ''),
               LLVM_PROFILE_FILE=str(raw/'%p-%m.profraw'))
    cargo = ['cargo', '+'+toolchain]
    args = cargo + ['test', '--workspace', '--locked', '--offline', '--no-run', '--message-format=json']
    if all_features:
        args.append('--all-features')
    # Keep JSON separate from compiler diagnostics for exact artifact discovery.
    with (folder/'build.jsonl').open('w') as stdout, (folder/'build.log').open('w') as stderr:
        start = time.monotonic()
        result = subprocess.run(args, cwd='/source', env=env, stdout=stdout, stderr=stderr, timeout=700)
    COMMANDS.append({'command': args, 'environment': env, 'exit': result.returncode,
                     'wallSeconds': time.monotonic()-start, 'timeoutSeconds': 700,
                     'log': label+'/build.log', 'artifacts': label+'/build.jsonl'})
    write('commands.json', COMMANDS)
    if result.returncode:
        raise RuntimeError('instrumented build failed')
    binaries = {}
    tests = []
    for line in (folder/'build.jsonl').read_text().splitlines():
        item = json.loads(line)
        if item.get('reason') == 'compiler-artifact' and item.get('executable'):
            path = item['executable']
            binaries[path] = sha(path)
            if item.get('profile', {}).get('test'):
                tests.append(path)
    if not tests:
        raise RuntimeError('no test executable')
    sysroot = subprocess.check_output(['rustc', '+'+toolchain, '--print', 'sysroot'], text=True).strip()
    llvm = Path(sysroot)/'lib/rustlib/aarch64-unknown-linux-gnu/bin'
    profile = SCRATCH/(label+'.profdata')
    raw_hashes = {}
    write(label+'/binaries-before.json', {'binaries': binaries, 'testExecutables': tests})
    if all_features:
        previous = EVIDENCE/('branch-default' if nightly else 'default')/'binaries-before.json'
        old = json.loads(previous.read_text())['binaries']
        removed = []
        for name in old.keys()-binaries.keys():
            path = Path(name)
            if path.parent != target/'debug/deps' or path.is_symlink():
                raise RuntimeError('unexpected retired executable path')
            path.unlink()
            removed.append(name)
        shutil.rmtree(target/'debug/build')
        write(label+'/retired-cache.json', {'executables': removed,
              'reason': 'previous variant measured and hashed; no current executable removed'})

    for i, binary in enumerate(tests):
        run([binary],
            f'{label}/test-{i:02}.log', env=env, timeout=180)
        profiles = sorted(raw.glob('*.profraw'))
        if not profiles:
            raise RuntimeError('no runtime counters')
        raw_hashes.update({p.name: sha(p) for p in profiles})
        pending = SCRATCH/(label+'.pending.profdata')
        # Merge after each test binary, including its CLI children, so raw
        # profiles cannot accumulate beyond the finite scratch filesystem.
        inputs = ([str(profile)] if profile.exists() else []) + list(map(str, profiles))
        run([str(llvm/'llvm-profdata'), 'merge', '-sparse', *inputs, '-o', str(pending)],
            f'{label}/merge-{i:02}.log')
        pending.replace(profile)
        write(label+'/raw-profile-hashes.json', raw_hashes)
        for path in profiles:
            path.unlink()
    for path, expected in binaries.items():
        if sha(path) != expected:
            raise RuntimeError('executable changed during measurement')
    objects = [piece for path in sorted(binaries) for piece in ['-object', path]]
    base = [str(llvm/'llvm-cov'), 'export', '-instr-profile='+str(profile),
            '-ignore-filename-regex=^(/scratch|/rustc|/usr)|/tests/', *objects]
    # llvm-cov is the authority for executable regions/lines; do not reconstruct
    # its line semantics from source text or treat regions as branches.
    run(base + ['-skip-functions', '-skip-expansions'], label+'/coverage.json', timeout=120, separate_stderr=True)
    run(base + ['-summary-only', '-dump'], label+'/diagnostic.txt', timeout=120, separate_stderr=True)
    with profile.open('rb') as src, gzip.open(folder/'coverage.profdata.gz', 'wb') as dst:
        shutil.copyfileobj(src, dst)
    export = json.loads((folder/'coverage.json').read_text())
    summary = summarize(export, branches=nightly)
    mapped = set(summary['files'])
    inventory = sorted(str(p.relative_to('/source')) for p in Path('/source/crates').glob('*/src/**/*.rs'))
    summary['unmappedSourceFiles'] = sorted(set(inventory)-mapped)
    write(label+'/summary.json', summary)
    show = [str(llvm/'llvm-cov'), 'show', '-instr-profile='+str(profile),
            '-ignore-filename-regex=^(/scratch|/rustc|/usr)|/tests/',
            '-show-line-counts-or-regions', '-show-instantiations=false', *objects]
    run(show, label+'/source-coverage.txt', timeout=120)
    write(label+'/binaries.json', {'binaries': binaries, 'testExecutables': tests,
          'profraw': raw_hashes, 'profdataSha256': sha(profile)})
    # Bound memory/disk; source-level JSON + native profiles remain as evidence.
    shutil.rmtree(raw)


def smoke():
    # A cheap real LLVM round trip catches stdout/stderr or flag errors before
    # spending a workspace build. This fixture is not product assurance.
    root = SCRATCH/'smoke'
    root.mkdir()
    (root/'main.rs').write_text('fn main() { let x = std::env::args().count(); if x > 1 { println!("yes"); } }\n')
    run(['rustc', '-C', 'instrument-coverage', str(root/'main.rs'), '-o', str(root/'app')], 'smoke-build.log')
    env = dict(os.environ, LLVM_PROFILE_FILE=str(root/'run.profraw'))
    run([str(root/'app'), 'positive'], 'smoke-run.log', env=env)
    sysroot = subprocess.check_output(['rustc', '--print', 'sysroot'], text=True).strip()
    llvm = Path(sysroot)/'lib/rustlib/aarch64-unknown-linux-gnu/bin'
    run([str(llvm/'llvm-profdata'), 'merge', '-sparse', str(root/'run.profraw'), '-o', str(root/'run.profdata')], 'smoke-merge.log')
    run([str(llvm/'llvm-cov'), 'export', '-object', str(root/'app'),
         '-instr-profile='+str(root/'run.profdata')], 'smoke.json', separate_stderr=True)
    data = json.loads((EVIDENCE/'smoke.json').read_text())
    assert data['data'][0]['totals']['lines']['covered'] > 0


def main():
    write('checks.json', boundary())
    (SCRATCH/'test-target').mkdir()
    cargo = SCRATCH/'cargo'
    cargo.mkdir()
    for name in ('registry', 'git'):
        cache = Path('/usr/local/cargo')/name
        if cache.exists():
            (cargo/name).symlink_to(cache, target_is_directory=True)
    smoke()
    if globals().get('STUDY_MODE') == 'coverage044-branch':
        run(['rustc', '+'+NIGHTLY, '-Vv'], 'nightly.log')
        measure('branch-default', nightly=True)
        measure('branch-all-features', nightly=True, all_features=True)
    else:
        run(['rustc', '-Vv'], 'rustc.log')
        measure('default')
        measure('all-features', all_features=True)


if __name__ == '__main__':
    try:
        main()
    except BaseException:
        traceback.print_exc()
        raise
    finally:
        # Preserve failed runs too, without relaxing the launcher's 64-MiB cap.
        for path in EVIDENCE.rglob('*'):
            if path.is_file() and path.stat().st_size > 2*1024**2 and path.suffix != '.gz':
                with path.open('rb') as src, gzip.open(str(path)+'.gz', 'wb') as dst:
                    shutil.copyfileobj(src, dst)
                path.unlink()
