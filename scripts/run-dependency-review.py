#!/usr/bin/env python3
"""Run task-051 checks in the existing immutable offline tool/cache image."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('container_runner', ROOT/'tests/protocol/container_runner.py')
confined = importlib.util.module_from_spec(spec); spec.loader.exec_module(confined)

ENTRY = '''
import json, os, pathlib, subprocess, sys, time, traceback
scratch = pathlib.Path('/scratch')
for name in ['home', 'tmp', 'cargo', 'evidence']:
    (scratch/name).mkdir()
for name in ['registry', 'git']:
    cache = pathlib.Path('/usr/local/cargo')/name
    if cache.exists(): (scratch/'cargo'/name).symlink_to(cache)
sys.path.insert(0, '/source/tests/protocol')
import container_entry
boundary = container_entry.boundary()
(scratch/'evidence'/'boundary.json').write_text(json.dumps(boundary, indent=2)+'\\n')
commands = [
    ['python3', '-m', 'unittest', 'discover', '-s', 'tests/distribution', '-v'],
    ['python3', 'scripts/review-dependencies.py', '--source-root', '/source',
     '--source-head', SOURCE_HEAD, '--preparation', '/preparation',
     '--out-dir', '/scratch/evidence/review'],
]
results = []
for index, command in enumerate(commands):
    with (scratch/'evidence'/f'check-{index}.log').open('wb') as stream:
        try:
            result = subprocess.run(command, stdout=stream, stderr=subprocess.STDOUT, timeout=180)
            code = result.returncode
        except subprocess.TimeoutExpired:
            code = 'timeout'
    results.append({'command': command, 'exit': code})
(scratch/'evidence'/'checks.json').write_text(json.dumps(results, indent=2)+'\\n')
(scratch/'status.json').write_text(json.dumps({'exit': int(any(r['exit'] != 0 for r in results))}))
while True: time.sleep(60)
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--image', required=True, help='Existing immutable Docker image SHA-256')
    parser.add_argument('--preparation', type=Path, required=True)
    parser.add_argument('--out-dir', type=Path, required=True)
    args = parser.parse_args()
    if not re.fullmatch(r'sha256:[0-9a-f]{64}', args.image):
        parser.error('immutable image ID required')
    output = args.out_dir.resolve(); output.mkdir(parents=True, exist_ok=False)
    preparation = args.preparation.resolve()
    source_head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    names = ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'LICENSE',
             '.github/workflows/release.yml', 'scripts/package-release.py',
             'scripts/review-dependencies.py', 'scripts/run-dependency-review.py',
             'tests/protocol/container_runner.py', 'tests/protocol/container_entry.py']
    for folder in ['crates', 'third_party/notices', 'tests/distribution']:
        names += [p.relative_to(ROOT).as_posix() for p in sorted((ROOT/folder).rglob('*'))
                  if p.is_file() and '__pycache__' not in p.parts]
    bindings = {}
    record = {'source_head': source_head, 'image': args.image, 'source_inputs': bindings,
              'exit': None, 'container_removed': False,
              'limits': {'network': 'none', 'source': 'readonly', 'root': 'readonly',
                         'memory_bytes': 1024**3, 'swap_bytes': 0, 'scratch_bytes': 512*1024**2,
                         'cpus': 1, 'pids': 64, 'wall_seconds': 420}}
    container = 'pqc-dependency051-'+uuid.uuid4().hex
    try:
        with tempfile.TemporaryDirectory(prefix='pqc-dependency051-') as temp:
            source = Path(temp)/'source'; source.mkdir()
            for name in names:
                data = confined.source_bytes(ROOT, name)
                bindings[name] = hashlib.sha256(data).hexdigest()
                destination = source/name; destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes(data); destination.chmod(0o444)
            entry = ('SOURCE_HEAD = '+repr(source_head)+'\n'+ENTRY).encode()
            (source/'entry.py').write_bytes(entry)
            record['entry_sha256'] = hashlib.sha256(entry).hexdigest()
            record['preparation_inputs'] = {n: confined.digest(preparation/n) for n in ['tool.json', 'database.json', 'cargo-audit']}
            env = {'PATH': '/usr/local/cargo/bin:/usr/bin:/bin', 'HOME': '/scratch/home',
                   'TMPDIR': '/scratch/tmp', 'CARGO_HOME': '/scratch/cargo',
                   'RUSTUP_HOME': '/usr/local/rustup', 'CARGO_NET_OFFLINE': 'true',
                   'PYTHONDONTWRITEBYTECODE': '1', 'LC_ALL': 'C'}
            record['environment'] = env
            command = ['docker', 'create', '--name', container, '--network', 'none', '--read-only',
                       '--user', '65534:65534', '--log-driver', 'none', '--cap-drop', 'ALL',
                       '--security-opt', 'no-new-privileges', '--memory', '1g', '--memory-swap', '1g',
                       '--cpus', '1', '--pids-limit', '64', '--shm-size', '1m',
                       '--ulimit', 'core=0:0', '--ulimit', 'fsize=16777216:16777216',
                       '--ulimit', 'nofile=256:256', '--ulimit', 'cpu=180:180',
                       '--tmpfs', '/scratch:rw,exec,nosuid,nodev,size=512m,mode=1777',
                       '--mount', f'type=bind,src={source},dst=/source,readonly',
                       '--mount', f'type=bind,src={preparation},dst=/preparation,readonly',
                       '--workdir', '/source', '--entrypoint', '/usr/bin/env', args.image,
                       '-i', *[f'{k}={v}' for k, v in env.items()], 'python3', '/source/entry.py']
            record['command'] = command
            subprocess.run(command, check=True, capture_output=True, timeout=30)
            subprocess.run(['docker', 'start', container], check=True, capture_output=True, timeout=30)
            deadline = time.monotonic()+420
            while True:
                if time.monotonic() >= deadline:
                    raise subprocess.TimeoutExpired(['docker', 'exec', container], 420)
                state = json.loads(subprocess.check_output(['docker', 'inspect', container], timeout=10))[0]['State']
                if not state['Running']:
                    raise RuntimeError('container exited before evidence collection')
                try:
                    status = json.loads(confined.bounded_output(['docker', 'exec', container, '/bin/cat', '/scratch/status.json'], limit=4096, timeout=3))
                    break
                except RuntimeError:
                    time.sleep(.1)
            record['exit'] = status['exit']
            data = confined.bounded_output(['docker', 'exec', container, '/bin/tar', '-C', '/scratch/evidence', '-cf', '-', '.'],
                                           limit=16*1024**2, timeout=30)
            confined.extract_artifacts(data, output/'artifacts', max_bytes=16*1024**2)
            boundary = json.loads((output/'artifacts/boundary.json').read_text())
            if not (boundary['memoryMax'] <= 1024**3 and boundary['swapMax'] == 0 and
                    boundary['pidsMax'] <= 64 and boundary['scratchCapacity'] <= 512*1024**2):
                raise ValueError('container limits not enforced')
            record['boundary'] = boundary
            record['source_unchanged'] = all(confined.digest(ROOT/n) == h for n, h in bindings.items())
            if not record['source_unchanged']:
                raise ValueError('source changed during assessment')
    except BaseException as error:
        record['error'] = type(error).__name__
        if record['exit'] in (None, 0):
            record['exit'] = 'host_validation_failed'
        raise
    finally:
        removed = subprocess.run(['docker', 'rm', '--force', container], capture_output=True, timeout=30)
        record['container_removed'] = removed.returncode == 0
        (output/'RUN.json').write_text(json.dumps(record, sort_keys=True, indent=2)+'\n')
    print(json.dumps({'exit': record['exit'], 'container_removed': record['container_removed']}))
    return record['exit'] if record['container_removed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
