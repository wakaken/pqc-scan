#!/usr/bin/env python3
"""Build allowlisted source, then execute only SSH interop with Docker network none."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'target/task-033/linux-interop'

def run(args, **kwargs):
    return subprocess.run(args, check=True, **kwargs)

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    selected = []
    names = subprocess.check_output(['git', 'ls-files', '--cached', '--others', '--exclude-standard', '-z'], cwd=ROOT).decode().split('\0')
    with tempfile.TemporaryDirectory(prefix='pqc-ssh-interop-') as tmp:
        context = Path(tmp)
        for name in sorted(set(filter(None, names))):
            if name not in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml') and not name.startswith('crates/'):
                continue
            src = ROOT / name
            if src.is_symlink() or not src.is_file() or any(part in ('.git', 'target') or part.startswith('.env') for part in Path(name).parts):
                raise RuntimeError(f'Unsafe build source: {name}')
            dst = context / name
            dst.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(src, dst)
            selected.append({'path':name, 'sha256':sha(dst)})
        shutil.copyfile(ROOT / 'tests/protocol/ssh-interop.Dockerfile', context / 'Dockerfile')
        tag = 'pqc-scan-task033-interop:local'
        with (OUT / 'pull.log').open('w') as log:
            run(['docker', 'pull', 'rust:1.93.1-bookworm'], stdout=log, stderr=subprocess.STDOUT)
        with (OUT / 'build.log').open('w') as log:
            run(['docker','build','--pull','--iidfile',str(OUT / 'image-id'),'-t',tag,str(context)], stdout=log, stderr=subprocess.STDOUT)
    image = (OUT / 'image-id').read_text().strip()
    inspect_image = json.loads(subprocess.check_output(['docker','image','inspect',image]))[0]
    base = json.loads(subprocess.check_output(['docker','image','inspect','rust:1.93.1-bookworm']))[0]
    container = subprocess.check_output(['docker','create','--network','none', image]).decode().strip()
    record = {'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT).decode().strip(), 'sources':selected, 'dockerfile_sha256':sha(ROOT / 'tests/protocol/ssh-interop.Dockerfile'), 'runner_sha256':sha(Path(__file__)), 'image':image, 'base_id':base['Id'], 'base_repo_digests':base['RepoDigests'], 'platform':f"{inspect_image['Os']}/{inspect_image['Architecture']}", 'container':container}
    try:
        state = json.loads(subprocess.check_output(['docker','inspect',container]))[0]
        assert state['HostConfig']['NetworkMode'] == 'none'
        assert not state['HostConfig']['Privileged']
        record['network_mode'] = state['HostConfig']['NetworkMode']
        record['privileged'] = state['HostConfig']['Privileged']
        record['security_opt'] = state['HostConfig']['SecurityOpt']
        with (OUT / 'test.log').open('w') as log:
            result = subprocess.run(['docker','start','-a',container],stdout=log,stderr=subprocess.STDOUT)
        state = json.loads(subprocess.check_output(['docker','inspect',container]))[0]
        record['start_exit'] = result.returncode
        record['exit'] = state['State']['ExitCode']
        run(['docker','cp',container+':/test-binary',str(OUT / 'test-binary')])
        record['binary_sha256'] = sha(OUT / 'test-binary')
        run(['docker','cp',container+':/work/target/task-033/.',str(OUT / 'reports')])
        # Metadata runs separately with the same nonegress/no-privilege constraints.
        record['versions'] = subprocess.check_output(['docker','run','--rm','--network','none','--entrypoint','sh',image,'-c','rustc --version; /usr/sbin/sshd -V 2>&1; sha256sum /test-binary']).decode()
    finally:
        run(['docker','rm','-f',container],stdout=subprocess.DEVNULL)
        (OUT / 'RESULTS.json').write_text(json.dumps(record,indent=2)+'\n')
    if record.get('exit') != 0:
        raise SystemExit(record.get('exit',1))

if __name__ == '__main__':
    main()
