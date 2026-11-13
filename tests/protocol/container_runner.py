"""Host-side launcher. Target code runs only in a finite, offline Linux container."""
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import re
import selectors
import shutil
import stat
import subprocess
import tarfile
import tempfile
import time
import uuid

ROOT = Path(__file__).resolve().parents[2]
ALLOWED_DIRS = {'crates', 'rules', 'schemas', 'tests', 'tasks', 'shell', 'examples'}
ALLOWED_FILES = {'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', '.gitignore', 'README.md', 'README.ja.md', 'LICENSE', 'docs/quality/RULE-QUALITY-METADATA.json'}
ENV = {'PATH':'/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin',
       'HOME':'/scratch/home', 'TMPDIR':'/scratch/tmp', 'CARGO_HOME':'/scratch/cargo',
       'RUSTUP_HOME':'/usr/local/rustup', 'CARGO_NET_OFFLINE':'true', 'CARGO_BUILD_JOBS':'2',
       'RUST_TEST_THREADS':'2', 'CARGO_INCREMENTAL':'0', 'CARGO_PROFILE_DEV_DEBUG':'0',
       'CARGO_PROFILE_TEST_DEBUG':'0', 'CARGO_TARGET_DIR':'/scratch/target', 'LC_ALL':'C',
       'PYTHONDONTWRITEBYTECODE':'1'}


def digest(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for part in iter(lambda: stream.read(65536), b''):
            h.update(part)
    return h.hexdigest()


def source_bytes(root, name, limit=32*1024**2):
    """Resolve each repository component through held, no-follow directory FDs."""
    path = PurePosixPath(name); parts = path.parts
    if not parts or path.is_absolute() or '..' in parts or '\\' in name or ':' in name or any(ord(c)<32 for c in name):
        raise ValueError('unsafe source path')
    directory = os.open(root, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        for part in parts[:-1]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=directory)
            os.close(directory); directory = child
        fd = os.open(parts[-1], os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=directory)
        with os.fdopen(fd, 'rb') as stream:
            info = os.fstat(stream.fileno())
            if not stat.S_ISREG(info.st_mode) or info.st_size > limit:
                raise ValueError('source must be a bounded regular file')
            data = stream.read(limit+1)
            if len(data) > limit:
                raise ValueError('source grew beyond the size limit')
            return data
    finally:
        os.close(directory)


def snapshot(root, destination):
    root = Path(root).resolve(); destination = Path(destination)
    destination.mkdir(parents=True, exist_ok=False)
    names = subprocess.check_output(['git', 'ls-files', '-z', '--cached', '--others', '--exclude-standard'], cwd=root).decode().split('\0')
    bindings = {}; size = 0
    for name in sorted(set(filter(None, names))):
        path = PurePosixPath(name)
        if path.is_absolute() or '..' in path.parts or any(ord(c) < 32 for c in name) or '\\' in name or ':' in name:
            raise ValueError('unsafe source path')
        if name not in ALLOWED_FILES and path.parts[0] not in ALLOWED_DIRS:
            continue
        if any(p == '.git' or p.startswith('.env') or p == '__pycache__' for p in path.parts):
            continue
        try:
            data = source_bytes(root, name)
        except OSError as error:
            raise ValueError('source changed or contains a link/special entry') from error
        size += len(data)
        if size > 128*1024**2:
            raise ValueError('source snapshot exceeds total size limit')
        target = destination / name; target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data); target.chmod(0o444)
        bindings[name] = hashlib.sha256(data).hexdigest()
        if bindings[name] != hashlib.sha256(source_bytes(root,name)).hexdigest():
            raise ValueError('source changed while copying')
    return bindings


def image_id(path):
    value = json.loads(Path(path).read_text())['image']
    if not re.fullmatch(r'sha256:[0-9a-f]{64}', value):
        raise ValueError('a locally prepared immutable image ID is required')
    return value


def extract_artifacts(data, destination, max_bytes=64*1024**2):
    destination = Path(destination).resolve(); destination.mkdir(parents=True, exist_ok=True)
    total = 0; names = set()
    with tarfile.open(fileobj=io.BytesIO(data)) as archive:
        members = []
        for member in archive:
            if len(members) >= 4096:
                raise ValueError('too many artifact entries')
            members.append(member)
        for member in members:
            name = PurePosixPath(member.name)
            if name.is_absolute() or '..' in name.parts or any(ord(c) < 32 for c in member.name) or '\\' in member.name or ':' in member.name:
                raise ValueError('unsafe artifact path')
            if member.isdir():
                continue
            if member.type not in {tarfile.REGTYPE,tarfile.AREGTYPE} or not name.parts or member.size < 0 or name in names:
                raise ValueError('artifact must be a unique regular file')
            names.add(name); total += member.size
            if total > max_bytes:
                raise ValueError('artifact size limit exceeded')
        for member in members:
            if not member.isfile():
                continue
            target = destination / member.name
            if any(parent.is_symlink() for parent in [target, *target.parents]):
                raise ValueError('artifact destination cannot contain links')
            target.parent.mkdir(parents=True, exist_ok=True)
            with target.open('xb') as stream, archive.extractfile(member) as source:
                shutil.copyfileobj(source, stream, 65536)


def bounded_output(command, limit=64*1024**2, timeout=30):
    process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
    chunks = []; size = 0; deadline = time.monotonic() + timeout
    try:
        with selectors.DefaultSelector() as selector:
            selector.register(process.stdout, selectors.EVENT_READ)
            while True:
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    raise subprocess.TimeoutExpired(command, timeout)
                if not selector.select(remaining):
                    raise subprocess.TimeoutExpired(command, timeout)
                part = os.read(process.stdout.fileno(), min(65536, limit-size+1))
                if not part:
                    break
                chunks.append(part); size += len(part)
                if size > limit:
                    raise ValueError('container artifact export exceeds limit')
        if process.wait(timeout=max(.1, deadline-time.monotonic())):
            raise RuntimeError('container artifact export failed')
        return b''.join(chunks)
    finally:
        if process.poll() is None:
            process.kill()
        process.wait(); process.stdout.close()


def execute(image, output, *, case='suite', previews=False, ssh_kex=False,
            memory=6*1024**3, scratch=4*1024**3, pids=128, timeout=1200):
    if not re.fullmatch(r'sha256:[0-9a-f]{64}', image):
        raise ValueError('immutable local image ID required')
    if case not in {'suite', 'boundary', 'memory', 'disk', 'pids', 'timeout', 'coverage044', 'coverage044-branch', 'mutation045', 'quality055', 'security057', 'fuzz047', 'fuzz047-profile'}:
        raise ValueError('unknown isolation case')
    output = Path(output).resolve(); output.mkdir(parents=True, exist_ok=False)
    record = {'image':image, 'environment':ENV, 'case':case, 'exit':None, 'containerRemoved':False,
              'limits':{'memoryBytes':memory, 'swapBytes':0, 'scratchBytes':scratch,
                        'sharedMemoryBytes':1024**2, 'pids':pids, 'cpus':2, 'wallSeconds':timeout,
                        'memoryEnforced':False, 'aggregateDiskEnforced':False}}
    container = None
    try:
        subprocess.run(['docker', 'image', 'inspect', image], check=True, stdout=subprocess.DEVNULL, timeout=30)
        with tempfile.TemporaryDirectory(prefix='pqc-confined-source-') as temporary:
            source = Path(temporary)/'source'; bindings = snapshot(ROOT, source)
            if case in {'coverage044', 'coverage044-branch', 'mutation045', 'quality055', 'security057', 'fuzz047', 'fuzz047-profile'}:
                # Legacy tests write beneath the repository target/ instead of
                # CARGO_TARGET_DIR. Route only that untracked output to tmpfs.
                (source/'target').symlink_to('/scratch/test-target', target_is_directory=True)
            record['sourceBindings'] = bindings
            for folder in [source, *[p for p in source.rglob('*') if p.is_dir()]]:
                folder.chmod(0o755)
            container = 'pqc-confined-'+uuid.uuid4().hex
            record['containerId'] = container
            command = ['docker', 'create', '--name', container, '--network', 'none', '--read-only', '--user', '65534:65534',
                       '--log-driver', 'none', '--cap-drop', 'ALL', '--security-opt', 'no-new-privileges',
                       '--memory', str(memory), '--memory-swap', str(memory), '--cpus', '2',
                       '--pids-limit', str(pids), '--shm-size', '1m', '--ulimit', 'core=0:0',
                       '--ulimit', 'fsize=1073741824:1073741824', '--ulimit', 'nofile=1024:1024',
                       '--ulimit', 'cpu=900:900', '--tmpfs', f'/scratch:rw,exec,nosuid,nodev,size={scratch},mode=1777',
                       '--mount', f'type=bind,src={source},dst=/source,readonly', '--workdir', '/source',
                       '--entrypoint', '/usr/bin/env', image, '-i', *[f'{k}={v}' for k,v in ENV.items()],
                       '/usr/bin/python3', '/source/tests/protocol/container_entry.py', '--case', case]
            if previews: command.append('--previews')
            if ssh_kex: command.append('--ssh-kex')
            record['containerId'] = subprocess.check_output(command,text=True,timeout=30).strip()
            inspect = json.loads(subprocess.check_output(['docker', 'inspect', container],timeout=10))[0]
            config = inspect['HostConfig']
            assert (config['NetworkMode']=='none' and config['ReadonlyRootfs'] and not config['Privileged']
                    and config['Memory']==memory and config['MemorySwap']==memory and config['PidsLimit']==pids)
            assert inspect['Config']['User']=='65534:65534'
            assert len(inspect['Mounts'])==1 and not inspect['Mounts'][0]['RW'] and inspect['Mounts'][0]['Destination']=='/source'
            record['container'] = {'HostConfig':{k:config[k] for k in ['NetworkMode','ReadonlyRootfs','Memory','MemorySwap','NanoCpus','PidsLimit','SecurityOpt','CapDrop','ShmSize','Tmpfs']},
                                   'User':inspect['Config']['User'], 'mounts':[{'destination':'/source','readonly':True}]}
            subprocess.run(['docker','start',container],check=True,stdout=subprocess.DEVNULL,timeout=30)
            deadline = time.monotonic()+timeout
            while True:
                if time.monotonic() >= deadline:
                    raise subprocess.TimeoutExpired(['docker','exec',container],timeout)
                state = json.loads(subprocess.check_output(['docker','inspect',container],timeout=10))[0]['State']
                if not state['Running']:
                    record['exit'] = state['ExitCode']; record['unexpectedContainerState'] = state
                    raise RuntimeError('container exited before evidence collection')
                try:
                    status = json.loads(bounded_output(['docker','exec',container,'/bin/cat','/scratch/status.json'],limit=4096,timeout=3))
                    break
                except RuntimeError:
                    time.sleep(.05)
            record['exit'] = status['exit']
            if case in {'coverage044', 'coverage044-branch', 'mutation045', 'quality055', 'security057', 'fuzz047', 'fuzz047-profile'} and record['exit']:
                diagnostic = bounded_output(['docker','exec',container,'/usr/bin/tail','-c','32768','/scratch/evidence/entry.log'], limit=32768)
                (output/'failure-diagnostic.log').write_bytes(diagnostic)
            data = bounded_output(['docker','exec',container,'/bin/tar','-C','/scratch/evidence','-cf','-','.'], limit=64*1024**2-32768)
            extract_artifacts(data, output)
            checks = json.loads((output/'checks.json').read_text())
            assert checks['memoryMax'] <= memory and checks['swapMax']==0 and checks['pidsMax']<=pids
            assert checks['scratchCapacity']<=scratch
            record['limits'].update(memoryEnforced=True, aggregateDiskEnforced=True)
            record['sourceBeforeAfterMatch'] = all(hashlib.sha256(source_bytes(ROOT,name)).hexdigest()==h for name,h in bindings.items())
            if record['exit'] or not record['sourceBeforeAfterMatch']:
                raise RuntimeError('confined execution failed; inspect entry.log and evidence')
    except subprocess.TimeoutExpired:
        record['exit'] = 'deadline_exceeded'
        raise
    except BaseException as error:
        if record['exit'] in {None,0}:
            record['exit'] = 'host_verification_failed'
        record['errorType'] = type(error).__name__
        raise
    finally:
        try:
            if container:
                try:
                    removed = subprocess.run(['docker','rm','-f',container],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,timeout=30)
                    if removed.returncode:
                        remaining = subprocess.run(['docker','container','ls','--all','--quiet','--filter','name=^/'+container+'$'],
                                                   capture_output=True,text=True,timeout=10)
                        if remaining.returncode or remaining.stdout.strip():
                            raise RuntimeError('container cleanup could not be confirmed')
                except BaseException:
                    record['exit'] = 'cleanup_failed'
                    raise
                record['containerRemoved'] = True
        finally:
            (output/'HOST-RESULTS.json').write_text(json.dumps(record,indent=2)+'\n')
    return record
