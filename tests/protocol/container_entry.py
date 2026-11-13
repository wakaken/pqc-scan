"""Executed inside the immutable-tool container, never directly on the host."""
import argparse
import contextlib
import errno
import hashlib
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import sys
import time

ROOT = Path('/source')
SCRATCH = Path('/scratch')
EVIDENCE = SCRATCH/'evidence'


def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for part in iter(lambda: stream.read(65536), b''):
            h.update(part)
    return h.hexdigest()


def cgroup(name):
    value = (Path('/sys/fs/cgroup')/name).read_text().strip()
    if value == 'max':
        raise RuntimeError('finite cgroup v2 limits are required')
    return int(value)


def write(name, record):
    (EVIDENCE/name).write_text(json.dumps(record,indent=2)+'\n')


def facts():
    if sys.platform != 'linux' or os.getuid() != 65534 or not ROOT.is_dir():
        raise RuntimeError('requires the confined nonroot Linux launcher')
    memory = cgroup('memory.max'); swap = cgroup('memory.swap.max'); pids = cgroup('pids.max')
    cpu = (Path('/sys/fs/cgroup')/'cpu.max').read_text().split()
    if cpu[0] == 'max' or swap != 0:
        raise RuntimeError('CPU quota and zero additional swap are required')
    active = {p.name for p in Path('/sys/class/net').iterdir()
              if (p/'flags').is_file() and int((p/'flags').read_text(),16) & 1}
    if active != {'lo'}:
        raise RuntimeError('only loopback may be active in the isolated network namespace')
    return {'memoryMax':memory, 'swapMax':swap, 'pidsMax':pids, 'cpuMax':cpu,
            'scratchCapacity':os.statvfs(SCRATCH).f_blocks*os.statvfs(SCRATCH).f_frsize}


def boundary():
    result = facts()
    for key, path in [('sourceReadonly', ROOT/'forbidden-write'), ('rootReadonly', Path('/forbidden-write'))]:
        try:
            path.write_text('unexpected')
        except OSError as error:
            if error.errno not in {errno.EACCES, errno.EROFS}:
                raise
            result[key] = True
        else:
            raise RuntimeError('unexpected writable source/root')
    result['emptyEnvironment'] = 'PQC_SYNTHETIC_SECRET' not in os.environ and os.environ['HOME']=='/scratch/home'
    assert result['emptyEnvironment']
    with socket.socket() as listener:
        listener.bind(('127.0.0.1',0)); listener.listen()
        with socket.create_connection(listener.getsockname(), timeout=1):
            connection, _ = listener.accept(); connection.close()
    result['loopbackWorks'] = True
    with socket.socket() as peer:
        peer.settimeout(1)
        try:
            peer.connect(('192.0.2.1',9))
        except OSError as error:
            if error.errno not in {errno.ENETUNREACH, errno.EHOSTUNREACH, errno.EPERM, errno.EACCES}:
                raise
            result['externalNetworkDenied'] = True
        else:
            raise RuntimeError('external connection unexpectedly allowed')
    return result


def oom_kills():
    events = (Path('/sys/fs/cgroup')/'memory.events').read_text().splitlines()
    return int(dict(line.split() for line in events)['oom_kill'])


def exercise(case):
    result = boundary()
    if case == 'memory':
        before = oom_kills(); children = []
        try:
            code = "from pathlib import Path; import time; Path('/proc/self/oom_score_adj').write_text('1000'); data=bytearray(40*1024**2); data[::4096]=b'x'*len(data[::4096]); time.sleep(30)"
            children = [subprocess.Popen([sys.executable,'-c',code]) for _ in range(2)]
            deadline = time.monotonic()+15
            while oom_kills() == before and time.monotonic() < deadline:
                time.sleep(.05)
            result['oomKills'] = oom_kills()-before
            assert result['oomKills'] > 0, 'memory pressure did not trigger the cgroup limit'
        finally:
            for child in children:
                if child.poll() is None: child.kill()
                child.wait()
    elif case == 'disk':
        folder = SCRATCH/'disk-probe'; folder.mkdir(); count = 0
        try:
            try:
                for i in range(64):
                    with (folder/str(i)).open('wb') as stream:
                        for _ in range(4): stream.write(b'x'*1024**2)
                    count += 1
            except OSError as error:
                if error.errno != errno.ENOSPC: raise
                result.update(aggregateDiskDenied=True, filesWritten=count)
            assert result.get('aggregateDiskDenied') and count > 1
        finally:
            for path in folder.iterdir(): path.unlink()
            folder.rmdir()
    elif case == 'pids':
        children = []
        try:
            for _ in range(64):
                try:
                    pid = os.fork()
                except OSError as error:
                    if error.errno != errno.EAGAIN: raise
                    result.update(forkDenied=True, children=len(children)); break
                if pid == 0:
                    signal.pause(); os._exit(0)
                children.append(pid)
            assert result.get('forkDenied')
        finally:
            for pid in children: os.kill(pid, signal.SIGKILL)
            for pid in children: os.waitpid(pid, 0)
    elif case == 'timeout':
        time.sleep(60)
    write('checks.json',result)


def suite(previews, ssh_kex):
    write('checks.json',boundary())
    cargo = SCRATCH/'cargo'; cargo.mkdir()
    for name in ['registry', 'git']:
        cache = Path('/usr/local/cargo')/name
        if cache.exists(): (cargo/name).symlink_to(cache, target_is_directory=True)
    command = ['cargo','test','--workspace','--all-features','--locked','--offline','--no-run','--message-format=json']
    with (EVIDENCE/'build.jsonl').open('w') as stdout, (EVIDENCE/'build.log').open('w') as stderr:
        subprocess.run(command,cwd=ROOT,stdout=stdout,stderr=stderr,check=True,timeout=900)
    binaries = {}; tests = []
    for line in (EVIDENCE/'build.jsonl').read_text().splitlines():
        item = json.loads(line)
        if item.get('reason')=='compiler-artifact' and item.get('executable'):
            path = item['executable']; binaries[path] = sha(path)
            if item.get('profile',{}).get('test'):
                tests.append((item['target']['name'], path))
    assert tests
    records = []
    for index, (name,path) in enumerate(tests):
        assert all(sha(p)==h for p,h in binaries.items())
        log = f'{index:02d}-{name}.log'
        with (EVIDENCE/log).open('w') as stream:
            completed = subprocess.run([path],cwd=ROOT,stdout=stream,stderr=subprocess.STDOUT,timeout=180)
        records.append({'test':name,'executable':path,'sha256':binaries[path],'exit':completed.returncode,'log':log})
        print(f'{name}: exit {completed.returncode}',flush=True)
        if completed.returncode:
            write('RESULTS.json',{'exit':completed.returncode,'binaries':binaries,'tests':records})
            raise RuntimeError('a confined test executable failed')
    with (EVIDENCE/'clippy.log').open('w') as log:
        subprocess.run(['cargo','clippy','--workspace','--all-targets','--all-features','--locked','--offline','--','-D','warnings'],
                       cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=600)
    binary = SCRATCH/'target/debug/pqc-scan'
    for enabled, script, directory in [(previews,'report-previews.py','previews'), (ssh_kex,'ssh-kex-product.py','ssh-kex')]:
        if enabled:
            args = [sys.executable,str(ROOT/'tests/protocol'/script),'--binary',str(binary),'--out-dir',str(EVIDENCE/directory)]
            if script == 'ssh-kex-product.py': args.append('--inside')
            with (EVIDENCE/(directory+'.log')).open('w') as log:
                subprocess.run(args,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=120)
    assert all(sha(p)==h for p,h in binaries.items())
    write('RESULTS.json',{'command':command,'exit':0,'binaries':binaries,'tests':records,'binariesBeforeAfterMatch':True,
                          'previews':previews,'sshKex':ssh_kex,'limits':facts()})


def main():
    parser = argparse.ArgumentParser(); parser.add_argument('--case', required=True)
    parser.add_argument('--previews', action='store_true'); parser.add_argument('--ssh-kex',action='store_true')
    args = parser.parse_args()
    for folder in [EVIDENCE, SCRATCH/'home', SCRATCH/'tmp']:
        folder.mkdir(exist_ok=True)
    # Keep the container running until the host copies bounded evidence. tmpfs
    # contents disappear on exit, so completion is an atomic status file.
    status = {'exit':0}
    with (EVIDENCE/'entry.log').open('w') as log, contextlib.redirect_stdout(log), contextlib.redirect_stderr(log):
        try:
            if args.case in {'coverage044', 'coverage044-branch'}:
                import runpy
                runpy.run_path(str(ROOT/'tests/quality/task044/entry.py'), run_name='__main__', init_globals={'STUDY_MODE': args.case})
            elif args.case == 'suite': suite(args.previews,args.ssh_kex)
            else: exercise(args.case)
        except BaseException as error:
            status = {'exit':1, 'errorType':type(error).__name__}
            print('confined entry failed:',type(error).__name__,flush=True)
    temporary = SCRATCH/'status.pending'
    temporary.write_text(json.dumps(status)+'\n'); temporary.replace(SCRATCH/'status.json')
    while True:
        time.sleep(60)


if __name__ == '__main__':
    main()
