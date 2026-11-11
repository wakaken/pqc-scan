#!/usr/bin/env python3
"""Prepare official dependencies, then execute the study with Docker network none."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import urllib.request

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
OPENSSH_URL = 'https://cdn.openbsd.org/pub/OpenBSD/OpenSSH/portable/openssh-10.2p1.tar.gz'
OPENSSH_SHA = 'ccc42c0419937959263fa1dbd16dafc18c56b984c03562d2937ce56a60f798b2'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--out-dir', type=Path, required=True)
    args = parser.parse_args()
    out = args.out_dir.resolve()
    out.mkdir(parents=True, exist_ok=False)
    allowed = ['Cargo.toml', 'Cargo.lock', 'README.md', 'Dockerfile',
               'run-docker.py', 'run-isolated.py', 'src/bin/russh-client.rs',
               'src/bin/ssh2-client.rs']
    sources = {str((HERE/name).relative_to(ROOT)):sha(HERE/name) for name in allowed}
    with tempfile.TemporaryDirectory(prefix='pqc039-context-') as temp:
        context = Path(temp)
        for name in sources:
            src = ROOT/name
            if src.is_symlink():
                raise RuntimeError('Symlink study source not permitted')
            dest = context/'study'/src.relative_to(HERE)
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(src, dest)
        shutil.copyfile(HERE/'Dockerfile', context/'Dockerfile')
        # Build preparation may download public dependencies; test execution cannot.
        with urllib.request.urlopen(OPENSSH_URL, timeout=30) as response:
            archive = response.read(4*1024*1024+1)
        if len(archive) > 4*1024*1024 or hashlib.sha256(archive).hexdigest() != OPENSSH_SHA:
            raise RuntimeError('Unexpected OpenSSH release archive')
        (context/'openssh-10.2p1.tar.gz').write_bytes(archive)
        command = ['docker','build','--iidfile',str(out/'image-id'),str(context)]
        with (out/'build.log').open('w') as log:
            subprocess.run(command,check=True,stdout=log,stderr=subprocess.STDOUT,timeout=900)
    image = (out/'image-id').read_text().strip()
    create = ['docker','create','--network','none','--cap-drop','NET_RAW','--security-opt','no-new-privileges','--memory','1g','--cpus','2','--pids-limit','128',image]
    container = subprocess.check_output(create,text=True).strip()
    record = {'source_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'source_bindings':sources,'image':image,'openssh_url':OPENSSH_URL,'openssh_archive_sha256':OPENSSH_SHA,'create_command':create}
    try:
        inspect = json.loads(subprocess.check_output(['docker','inspect',container]))[0]
        host = inspect['HostConfig']
        assert host['NetworkMode'] == 'none' and not host['Privileged'] and not host['Binds']
        assert 'no-new-privileges' in host['SecurityOpt']
        record['isolation'] = {k:host[k] for k in ['NetworkMode','Privileged','Binds','SecurityOpt','CapDrop','Memory','NanoCpus','PidsLimit']}
        with (out/'run.log').open('w') as log:
            result = subprocess.run(['docker','start','-a',container],stdout=log,stderr=subprocess.STDOUT,timeout=90)
        record['exit'] = result.returncode
        subprocess.run(['docker','cp',container+':/work/target/task-039-results',str(out/'results')],check=True,stdout=subprocess.DEVNULL)
        metadata = subprocess.check_output(['docker','run','--rm','--network','none','--entrypoint','sh',image,'-c','rustc --version; openssl version; /opt/openssh/sbin/sshd -V 2>&1; /work/target/task-039-build/debug/ssh2-client; sha256sum /work/target/task-039-build/debug/russh-client /work/target/task-039-build/debug/ssh2-client'],text=True)
        record['versions_and_local_capabilities'] = metadata.splitlines()
        assert all(sha(ROOT/p) == h for p,h in sources.items()), 'Source changed during build/test'
        record['before_after_sources_match'] = True
        if result.returncode:
            raise RuntimeError('Isolated study failed; inspect run.log')
    finally:
        subprocess.run(['docker','rm','-f',container],check=True,stdout=subprocess.DEVNULL)
        record['container_removed'] = True
        (out/'DOCKER-RESULTS.json').write_text(json.dumps(record,indent=2)+'\n')
    print('Isolated SSH KEX study passed; container and fixture keys removed.')


if __name__ == '__main__':
    main()
